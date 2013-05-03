#[link(name = "css_language", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_stylesheet;
extern mod std;
extern mod wapcaplet;
extern mod css_propstrings;
extern mod css_properties;

use css_enum::* ;
use css_stylesheet::*;
use std::arc;
use wapcaplet::*;
use css_propstrings::*;
use css_properties::*;


pub struct context_entry {
	event_type:css_parser_event,        /* < Type of entry */
	data:Option<CSS_RULE_DATA_TYPE>     //< Data for context */
} 

pub struct css_namespace {
	prefix:Option<arc::RWARC<~lwc_string>> ,		/**< Namespace prefix */
	uri:Option<arc::RWARC<~lwc_string>>		//< Namespace URI */
}

pub struct css_language {
	sheet:@mut css_stylesheet,
	lwc_instance:arc::RWARC<~lwc>,		
	context:~[context_entry], 
	state:language_state,	
	strings: ~css_propstrings,  
	properties: ~css_properties,
	default_namespace:Option<~str>, 
	namespaces:~[~css_namespace],
}

	fn  css_language(sheet:@mut css_stylesheet, lwc_inst:arc::RWARC<~lwc>) -> ~css_language {
		
	~css_language {
		sheet:sheet,
		lwc_instance: lwc_inst.clone(),
		strings: css_propstrings::css_propstrings(lwc_inst.clone()),
		properties: css_properties::css_properties(),
		context:~[], 
		state:CHARSET_PERMITTED,
		default_namespace:None,   
		namespaces:~[]
	}
}


pub impl css_language {
	
	pub fn language_handle_event(&mut self, event_type:css_parser_event, tokens:~[~css_token])-> css_result {
			match event_type {
				
				CSS_PARSER_START_STYLESHEET => {
					self.handleStartStylesheet()
				}
				
				CSS_PARSER_END_STYLESHEET=>{
					self.handleEndStylesheet()
				}
				
				CSS_PARSER_START_RULESET=>{
					self.handleStartRuleset(tokens)
				}
				
				CSS_PARSER_END_RULESET=>{
					self.handleEndRuleset()
				}
				
				CSS_PARSER_START_ATRULE=>{
					self.handleStartAtRule(tokens)
				}
				
				CSS_PARSER_END_ATRULE=>{
					self.handleEndAtRule()
				}
				
				CSS_PARSER_START_BLOCK=>{
					self.handleStartBlock()
				}
				
				CSS_PARSER_END_BLOCK=>{
					self.handleEndBlock()
				}
				
				CSS_PARSER_BLOCK_CONTENT=>{
					self.handleBlockContent(tokens)
				}
				
				CSS_PARSER_DECLARATION=>{
					self.handleDeclaration(tokens)
				}
			}
		}


	pub fn handleStartStylesheet(&mut self ) -> css_result {
			let entry:context_entry = context_entry {
				event_type: CSS_PARSER_START_STYLESHEET, 
				data:None                                       
			};
					
			self.context.push(entry);
			CSS_OK
		}

		pub fn handleEndStylesheet(&mut self)->css_result {
			if vec::is_empty(self.context) {
				return CSS_INVALID
			}
			match self.context.last().event_type {
				CSS_PARSER_START_STYLESHEET => {},
				_   =>return CSS_INVALID
			}

			self.context.pop();
			CSS_OK
		}

		pub fn handleStartRuleset(&mut self, tokens:~[~css_token]) ->css_result	{
			
			let mut cur:context_entry ;
			let mut parent_rule :Option<CSS_RULE_DATA_TYPE> = None ;

			/* Retrieve parent rule from stack, if any */
			if !vec::is_empty(self.context)	{
				cur=self.context[self.context.len()-1];
				match cur.event_type {
					CSS_PARSER_START_STYLESHEET =>{},
					_=>{parent_rule = cur.data;}
				}
			}
			
			let mut curRule = self.sheet.css_stylesheet_rule_create(CSS_RULE_SELECTOR);
			
			if !vec::is_empty(tokens) {
				match self.parseSelectorList(&tokens, curRule) {
					CSS_OK => {},
					x      =>   return x  
				}
			}

			let mut entry:context_entry = context_entry {
				event_type: CSS_PARSER_START_STYLESHEET, 
				data:Some(curRule)
			};
			self.context.push(entry);

		
			match css_stylesheet::css__stylesheet_add_rule(self.sheet, curRule, parent_rule) {
				CSS_OK => 	{},
				x      => 	{
					self.context.pop();
					return x  
				}   
			 } 
			
			// /* Flag that we've had a valid rule, so @import/@namespace/@charset 
			//  * have no effect. */
			  self.state = HAD_RULE;

			/* Rule is now owned by the sheet, so no need to destroy it */

			  CSS_OK
		}

	pub fn handleEndRuleset(&mut self)->css_result {
				
		let mut cur:context_entry;
		
		/* Retrieve parent rule from stack, if any */
			if !vec::is_empty(self.context) {
				cur=self.context[self.context.len()-1];
				match cur.event_type {
					CSS_PARSER_START_RULESET => {
						self.context.pop();
						CSS_OK
					},
					_ =>	CSS_INVALID
				}
			}
			else {
				CSS_INVALID
			}
	}

	pub fn handleStartAtRule(&self, vector:~[~css_token])->css_result {
		CSS_OK  
	}

	pub fn handleEndAtRule(&mut self)->css_result {
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context)	{
			cur=self.context[self.context.len()-1];
			match cur.event_type {
				CSS_PARSER_START_ATRULE => {
					self.context.pop();
					CSS_OK
				},
				_ =>	CSS_INVALID
			}
		}
		else {
			CSS_INVALID
		}
	}
	

	pub fn handleStartBlock(&mut self)->css_result {
		
		let mut cur:context_entry;
		let mut entry:context_entry = context_entry{ event_type:CSS_PARSER_START_BLOCK, data:None };
		

		/* If the current item on the stack isn't a block, 
		 * then clone its data field. This ensures that the relevant rule
		 * is available when parsing the block contents. */
		if !vec::is_empty(self.context) {
			cur=self.context[self.context.len()-1];
			match cur.event_type {
				CSS_PARSER_START_BLOCK =>	{},
				_ =>	entry.data = cur.data
			}
		}

		self.context.push(entry);
		CSS_OK  
	}

	pub fn handleEndBlock(&mut self)->css_result {
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context)	{
			cur=self.context[self.context.len()-1];
			match cur.event_type {
				CSS_PARSER_START_BLOCK => {
					let mut curRule = cur.data;
					self.context.pop();
					/* If the block we just popped off the stack was associated with a 
					* non-block stack entry, and that entry is not a top-level statement,
					* then report the end of that entry, too. */
					match curRule {
						None => CSS_OK,
						Some(x) => { 
							match x {
								RULE_SELECTOR(_) =>
									match css_stylesheet::css__stylesheet_get_base_rule(x).parent_rule {
										Some(pRule) => 
											match css_stylesheet::css__stylesheet_get_parent_type(pRule) {
												CSS_RULE_PARENT_STYLESHEET  => self.handleEndRuleset(),
												_ => CSS_OK
											},
										None => CSS_OK                      
									},
								_ =>    CSS_OK
							}
						}
					}
				},
				_ 	=>	return CSS_INVALID
			} // end of match
		}
		else {
			return CSS_INVALID  
		}       
	}


	pub fn handleBlockContent(&mut self, tokens:~[~css_token])-> css_result {
		// * Block content comprises either declarations (if the current block is
		// * associated with @page, @font-face or a selector), or rulesets (if the
		// * current block is associated with @media). 
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context) {
			cur=self.context[self.context.len()-1];
			match cur.data {
				None => CSS_INVALID,
				Some(curRule) => {
					match curRule {
						RULE_SELECTOR(_) | RULE_PAGE (_) | RULE_FONT_FACE(_) => {                                   
							//Expect declarations 
							return self.handleDeclaration(tokens);
						},
						RULE_MEDIA(_) => {
							// Expect rulesets 
							return self.handleStartRuleset(tokens);
						},  
						_ =>    return CSS_INVALID
					}
				}
			} // end of match
		}
		else {
			return CSS_INVALID  
		}       
	}

	pub fn handleDeclaration(&mut self, tokens:~[~css_token])->css_result {
		let ctx: @mut uint = @mut 0u;   
		 // Locations where declarations are permitted:
		 // *
		 // * + In @page
		 // * + In @font-face
		 // * + In ruleset
		 
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context) {
			cur=self.context[self.context.len()-1];
			match cur.data {
				None => CSS_INVALID,
				Some(curRule) => {
					match curRule {
						RULE_SELECTOR(_) | RULE_PAGE (_) | RULE_FONT_FACE(_) => {                                   
							// Strip any leading whitespace (can happen if in nested block) 
							css_properties::consumeWhitespace(&tokens, ctx);

							// IDENT ws ':' ws value 
							// * 
							// * In CSS 2.1, value is any1, so '{' or ATKEYWORD => parse error
							 
							
							if tokens.len() > *ctx {   
								let ident =&tokens[*ctx];
								*ctx = *ctx + 1;
								match ident.token_type { 
									CSS_TOKEN_IDENT(_) => {
										css_properties::consumeWhitespace(&tokens, ctx);
										if tokens.len() <= *ctx || !css_language::tokenIsChar(&tokens[*ctx],':') {
											return CSS_INVALID
										}
										else {
											*ctx += 1;
											css_properties::consumeWhitespace(&tokens, ctx);
											match curRule {
												RULE_FONT_FACE(font_face_rule) =>	
													return css_language::css__parse_font_descriptor(ident, &tokens, ctx, font_face_rule),
												_ =>	
													return self.parseProperty(ident, &tokens, ctx, curRule)	
											}
										}				
									} 
									_ => return CSS_INVALID
								} 
							}
							else {
								return CSS_INVALID
							}       
						},
						_ =>    return CSS_INVALID
					}
				}
			} // end of match
		}
		else {
			return CSS_INVALID  
		}       
	}

	pub fn parseSelectorList(&mut self, tokens:&~[~css_token], curRule: CSS_RULE_DATA_TYPE) -> css_result {
		let ctx: @mut uint = @mut 0u;
		
		loop {
			/* Strip any leading whitespace (can happen if in nested block) */
			css_properties::consumeWhitespace(tokens, ctx);

			/* selector_list   -> selector [ ',' ws selector ]* */
			match self.parseSelector(tokens, ctx) {
				(CSS_OK, Some(selector)) => {
					match css_stylesheet::css__stylesheet_rule_add_selector(curRule, selector) {
						CSS_OK => {
							if *ctx < tokens.len() {
								//Iterate over vector to check for invalid character
								if !css_language::tokenIsChar(&tokens[*ctx],',') {
									*ctx = *ctx+1;   //For iteration to the next position
									return CSS_INVALID
								}
								*ctx = *ctx+1 //For iteration to the next position
							}
							else {
								break
							} 
						},
						x => return x
					}//End of match rule_add_selector
				},
				(x, _) => return x              
			} //End of match parseSelector
		}// End of Loop
		CSS_OK
	}


	/**
	 * Determine if a token is a character
	 *
	 * \param token  The token to consider
	 * \param c      The character to match (lowercase ASCII only)
	 * \return True if the token matches, false otherwise
	 */
	pub fn tokenIsChar(token:&~css_token, c:char) -> bool {
		let result = false;

		match token.token_type {
			CSS_TOKEN_CHAR(c) => {   
					if lwc::lwc_string_length(token.idata.clone()) == 1 {
						let mut token_char = lwc::lwc_string_data(token.idata.clone()).char_at(0);

						// Ensure lowercase comparison 
						if 'A' <= token_char && token_char <= 'Z' {
							token_char += 'a' - 'A'
						}
							
						if token_char == c {
							return true
						}
					}                       
				},
			_ => return result
		}           
		
		return result
	}

	/******************************************************************************
	 * Property parsing functions                             *
	 ******************************************************************************/

	pub fn parseProperty(&mut self , property: &~css_token , vector: &~[~css_token], ctx:@mut uint, curRule: CSS_RULE_DATA_TYPE) -> css_result {
		
		let mut style: @mut css_style;
		let mut index = AZIMUTH as uint;

		while (index < Z_INDEX as uint) {
			if self.strings.lwc_string_caseless_isequal(property.idata.clone() , index) {
				break
			}
			index +=1;
		}

		if index == Z_INDEX as uint + 1 {
			return CSS_INVALID;
		}

		style = self.sheet.css__stylesheet_style_create();

		(*self.properties.property_handlers[index - AZIMUTH as uint])(&mut self.strings , vector , ctx , style);
		let flags = self.css__parse_important(vector , ctx);

		css_properties::consumeWhitespace(vector , ctx);

		// if tokens.len() > *ctx { 	
		//    	let ident =&tokens[*ctx];
		// 	*ctx = *ctx + 1;

		if (flags != 0) {
			self.css__make_style_important(style);
		}

		/* Append style to rule */
		match self.sheet.css__stylesheet_rule_append_style(curRule, style) {
			CSS_OK => CSS_OK,
			x => x
		}
	}

	pub fn parseSelector(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result, Option<@mut css_selector>) {
		
		/* selector -> simple_selector [ combinator simple_selector ]* ws
		 * 
		 * Note, however, that, as combinator can be wholly whitespace,
		 * there's an ambiguity as to whether "ws" has been reached. We 
		 * resolve this by attempting to extract a combinator, then 
		 * recovering when we detect that we've reached the end of the
		 * selector.
		 */

		match self.parseSimpleSelector(vector, ctx) {
			(CSS_OK, Some(selector)) => {
				let mut result = selector;
				loop {
					if *ctx >= vector.len() || css_language::tokenIsChar(&vector[*ctx],',') {
						return (CSS_OK, Some(result))
					}
					else {
						let comb = @mut CSS_COMBINATOR_NONE;        
						match self.parseCombinator(vector, ctx, comb) {
							CSS_OK => {
								/* In the case of "html , body { ... }", the whitespace after
								 * "html" and "body" will be considered an ancestor combinator.
								 * This clearly is not the case, however. Therefore, as a 
								 * special case, if we've got an ancestor combinator and there 
								 * are no further tokens, or if the next token is a comma,
								 * we ignore the supposed combinator and continue. */
								match *comb {
									CSS_COMBINATOR_ANCESTOR => {
										if *ctx >= vector.len() || css_language::tokenIsChar(&vector[*ctx],',') {
											loop
										}
									},
									_ => {
										match self.parseSimpleSelector(vector, ctx) {
											(CSS_OK, Some(other_selector)) => {   
												match css_stylesheet::css__stylesheet_selector_combine(*comb, selector, other_selector) {
													CSS_OK => { result = other_selector}
													x => return (x,None)
												}
											},
											(x,y) => return(x,y)
										} // End of match parseSimpleSelector
									}       
								} // End of match comb
							},  
							x => return (x, Some(selector))
						}// End of outer match parseCombinator
					} // End of If Else
				} //End of loop
			},
			(x,y) => return (x,y)
		} // End of outer match parseSimpleSelector
	}

	pub fn parseSimpleSelector(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result, Option<@mut css_selector>) {
		let orig_ctx = *ctx;
		/* simple_selector  -> type_selector specifics
		 *          -> specific specifics
		 */
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}        
		
		let mut selector : @mut css_selector;
		let qname: @mut css_qname = @mut css_qname{ name:~"", ns:~""};

		//match ( vector[*ctx].token_type as uint ==    CSS_TOKEN_IDENT as uint )
		if css_language::tokenIsChar(&vector[*ctx], '*') || css_language::tokenIsChar(&vector[*ctx], '|') {
			
			/* Have type selector */
			match self.parseTypeSelector(vector, ctx, qname) {
				CSS_OK => {
					selector = self.sheet.css__stylesheet_selector_create(copy *qname);
				},
				x => {
					*ctx = orig_ctx;
					return (x, None)
				}                       
			} 
		}   
		else {
			/* Universal selector */
			match self.default_namespace {
				Some (copy ns) => qname.ns = ns,
				None => qname.ns = self.strings.lwc_string_data(UNIVERSAL as uint)
			}   
			
			qname.name = self.strings.lwc_string_data(UNIVERSAL as uint);

			selector =  self.sheet.css__stylesheet_selector_create(copy *qname);
			/* Ensure we have at least one specific selector */
			match self.parseAppendSpecific(vector, ctx, selector) {
				CSS_OK => {},
				error  => return (error,None)
			}			
		}   
		
		
		match self.parseSelectorSpecifics(vector, ctx, selector) {
			CSS_OK => return (CSS_OK, Some(selector)),
			error => return (error, None)
		}   
		
	}

	pub fn parseCombinator(&mut self, vector:&~[~css_token], ctx:@mut uint, comb:@mut css_combinator) -> css_result {
		
		let mut token:&~css_token;
		/* combinator      -> ws '+' ws | ws '>' ws | ws '~' ws | ws1 */
		*comb = CSS_COMBINATOR_NONE;

		loop {
			if *ctx >= vector.len() {
				break
			} 

			token = &vector[*ctx];
			if css_language::tokenIsChar(token, '+') {
				*comb = CSS_COMBINATOR_SIBLING
			}   
			else if css_language::tokenIsChar(token,  '>') {
				*comb = CSS_COMBINATOR_PARENT   
			}
			else if css_language::tokenIsChar(token, '~') {
				*comb = CSS_COMBINATOR_GENERIC_SIBLING
			}   
			else {
				match token.token_type {
					CSS_TOKEN_S =>  *comb = CSS_COMBINATOR_ANCESTOR,
					_           =>  break
				}
			}

			*ctx += 1;  

			/* If we've seen a '+', '>', or '~', we're done. */
			if *comb as uint != CSS_COMBINATOR_ANCESTOR as uint {
				break
			}   

		}
		
		/* No valid combinator found */
		match *comb {
			CSS_COMBINATOR_NONE => return CSS_INVALID,  
			_                   => {
				/* Consume any trailing whitespace */
				css_properties::consumeWhitespace(vector, ctx);
				return CSS_OK
			}
		} 
	}   

	pub fn parseTypeSelector(&mut self, vector:&~[~css_token], ctx:@mut uint, qname:@mut css_qname) -> css_result {
		let mut token:&~css_token;
		let mut prefix:Option<arc::RWARC<~lwc_string>> =None;

		/* type_selector    -> namespace_prefix? element_name
		 * namespace_prefix -> [ IDENT | '*' ]? '|'
		 * element_name     -> IDENT | '*'
		 */
		 if *ctx >= vector.len() {
			return CSS_INVALID
		} 
		
		token = &vector[*ctx];
		
		if !css_language::tokenIsChar(token, '|') {
			 prefix = Some(token.idata.clone());
			*ctx += 1; //Iterate
		}

		if ( *ctx < vector.len() && css_language::tokenIsChar(&vector[*ctx], '|')) {
			
			/* Have namespace prefix */
			*ctx += 1; //Iterate

			/* Expect element_name */
			if *ctx >= vector.len() || ( match vector[*ctx].token_type { CSS_TOKEN_IDENT(_) => false, _ => true} && !css_language::tokenIsChar(&vector[*ctx], '*') ) {
				return CSS_INVALID
			}
			*ctx += 1; //Iterate

			match self.lookupNamespace(prefix, qname) {
				CSS_OK  => qname.name = lwc::lwc_string_data(vector[*ctx].idata.clone()),
				error   => return error
			}   
		} 
		else {
			/* No namespace prefix */
			match self.default_namespace {
				Some (copy ns) => qname.ns = ns,
				None => qname.ns = self.strings.lwc_string_data(UNIVERSAL as uint)
			}


			qname.name = match prefix {
							Some (x) => lwc::lwc_string_data(x),
							None => ~""
						}
			
		}
		
		return CSS_OK
	}   

	pub fn parseSelectorSpecifics(&mut self, vector:&~[~css_token], ctx:@mut uint, parent:@mut css_selector ) -> css_result {
		let mut token:&~css_token;

		/* specifics -> specific* */
		loop {
			if *ctx >= vector.len() {
				break;
			}	
			else {
				token = &vector[*ctx];
				if (match token.token_type { CSS_TOKEN_S => false, _ => true }) && 
					!css_language::tokenIsChar(token, '+')  &&
					!css_language::tokenIsChar(token, '>')  &&
					!css_language::tokenIsChar(token, '~')  &&
					!css_language::tokenIsChar(token, ',') {
					match self.parseAppendSpecific(vector,ctx,parent) {
						CSS_OK 	=> loop,
						error	=>	return error
					}
				}
				else {
					break;
				}
			}	
		}
		CSS_OK		
	}  


	pub fn parseAppendSpecific(&mut self, vector:&~[~css_token], ctx:@mut uint, parent:@mut css_selector ) -> css_result{
		
		match self.parseSpecific(vector, ctx, false) {
			(CSS_OK,Some(specific)) => return css_stylesheet::css__stylesheet_selector_append_specific(parent,specific),
			(error,_) => return error
		}   	
	}   


	pub fn parseSpecific(&mut self, vector:&~[~css_token], ctx:@mut uint, in_not:bool) -> (css_result,Option<@mut css_selector_detail>) {
		
		/* specific  -> [ HASH | class | attrib | pseudo ] */

		let mut token:&~css_token;
		
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}	
		
		token = &vector[*ctx];

		match token.token_type {
			CSS_TOKEN_HASH(_)	=> {
				let qname:css_qname=css_qname{ns:~"", name:lwc::lwc_string_data(token.idata.clone())};
				match css_stylesheet::css__stylesheet_selector_detail_init (CSS_SELECTOR_ID, qname, 
											CSS_SELECTOR_DETAIL_VALUE_STRING,None, None, false) {
					(CSS_OK, res) => {
						*ctx +=1;
						(CSS_OK, res) 
					} 
					(error, y) =>  (error,y)	
				}
	
			} 
			_ 	=> {
				if (css_language::tokenIsChar(token, '.')) {
					self.parseClass(vector, ctx)
				} 
				else if (css_language::tokenIsChar(token, '[')) {
					self.parseAttrib(vector, ctx)
				}
				else if (css_language::tokenIsChar(token, ':')) {
					self.parsePseudo(vector, ctx, in_not)
				} 
				else {
					(CSS_INVALID,None)
				}
			}
		}		
	}

	/**
	 * Look up a namespace prefix
	 *
	 * \param c       Language parser context
	 * \param prefix  Namespace prefix to find, or NULL for none
	 * \param uri     Pointer to location to receive namespace URI
	 * \return CSS_OK on success, CSS_INVALID if prefix is not found
	 */
	pub fn lookupNamespace(&mut self, prefix:Option<arc::RWARC<~lwc_string>>, qname:@mut css_qname) -> css_result {
		let mut idx:uint=0;
		
		match prefix {
			None =>	qname.ns = ~"",
			Some(value) => {
				for self.namespaces.each |ns| {
					match ns.prefix {
						Some(_) => {
							let ns_prefix = ns.prefix.get_ref().clone();
							if lwc::lwc_string_isequal(ns_prefix,value.clone()) {
								break
							}	
						},	
						None => {}
					}
					idx += 1;	
				}

				if (idx == self.namespaces.len()) {
					return CSS_INVALID
				}	

				match self.namespaces[idx].uri {
					Some(_) => qname.ns = lwc::lwc_string_data(self.namespaces[idx].uri.get_ref().clone()),
					None => qname.ns = ~""
				}
			}
		}	
		CSS_OK
	}

	/******************************************************************************
	* Selector list parsing functions					      *
	******************************************************************************/
	pub fn  parseClass(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result,Option<@mut css_selector_detail>) {
		
		let mut token:&~css_token;
		
		/* class     -> '.' IDENT */
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}	
		
		token = &vector[*ctx];
		*ctx +=1; //Iterate				
		
		if !css_language::tokenIsChar(token, '.') {
			return (CSS_INVALID,None)
		}	

		token = &vector[*ctx];
		*ctx +=1; //Iterate	

		match token.token_type {
			CSS_TOKEN_IDENT(_) => {
				let qname:css_qname=css_qname{ns:~"", name:lwc::lwc_string_data(token.idata.clone())};
				return css_stylesheet::css__stylesheet_selector_detail_init (CSS_SELECTOR_CLASS, qname, 
													CSS_SELECTOR_DETAIL_VALUE_STRING,None, None, false)
			}
			_ => return (CSS_INVALID,None)
		}
	}

	pub fn  parseAttrib(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result,Option<@mut css_selector_detail>) {
		let mut token:&~css_token;
		
		/* attrib    -> '[' ws namespace_prefix? IDENT ws [
		 *		       [ '=' | 
		 *		         INCLUDES | 
		 *		         DASHMATCH | 
		 *		         PREFIXMATCH |
		 *		         SUFFIXMATCH | 
		 *		         SUBSTRINGMATCH 
		 *		       ] ws
		 *		       [ IDENT | STRING ] ws ]? ']'
		 * namespace_prefix -> [ IDENT | '*' ]? '|'
		 */
		
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}	
		
		token = &vector[*ctx];
		*ctx +=1; //Iterate				
		
		if !css_language::tokenIsChar(token, '[') {
			return (CSS_INVALID,None)
		}	

		css_properties::consumeWhitespace(vector, ctx);

		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}

		token = &vector[*ctx];
		*ctx +=1; //Iterate				

		if (match token.token_type { CSS_TOKEN_IDENT(_) => false, _ => true}) && !css_language::tokenIsChar(token, '*') &&
				!css_language::tokenIsChar(token, '|') {
			return (CSS_INVALID, None)
		}	
		
		let mut prefix: Option<arc::RWARC<~lwc_string>> = None;

		if css_language::tokenIsChar(token, '|') {
			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
		} 
		else if (*ctx < vector.len() && css_language::tokenIsChar(&vector[*ctx], '|')) {
			prefix = Some(token.idata.clone());
			*ctx += 1;
			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
		}

		if match token.token_type { CSS_TOKEN_IDENT(_) => false, _ => true} {
			return (CSS_INVALID, None)
		}	

		let qname:@mut css_qname=@mut css_qname{ns:~"", name:~""};
		match self.lookupNamespace(prefix, qname) {	CSS_OK  => {}, error   => return (error,None)}   

		qname.name = lwc::lwc_string_data(vector[*ctx].idata.clone());

		css_properties::consumeWhitespace(vector, ctx);

		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}

		token = &vector[*ctx];
		*ctx +=1; //Iterate

		let mut tkn_type = CSS_SELECTOR_ATTRIBUTE;
		let mut value:Option<&~css_token> = None;

		if !css_language::tokenIsChar(token, ']') {
			if css_language::tokenIsChar(token, '=') {
				tkn_type = CSS_SELECTOR_ATTRIBUTE_EQUAL;
			}
			// else {
			// 	match token.token_type {
			// 		CSS_TOKEN_INCLUDES 		 => tkn_type = CSS_SELECTOR_ATTRIBUTE_INCLUDES, 
			// 		CSS_TOKEN_DASHMATCH 	 => tkn_type = CSS_SELECTOR_ATTRIBUTE_DASHMATCH,
			// 		CSS_TOKEN_PREFIXMATCH 	 => tkn_type = CSS_SELECTOR_ATTRIBUTE_PREFIX,
			// 		CSS_TOKEN_SUFFIXMATCH 	 => tkn_type = CSS_SELECTOR_ATTRIBUTE_SUFFIX,
			// 		CSS_TOKEN_SUBSTRINGMATCH => tkn_type = CSS_SELECTOR_ATTRIBUTE_SUBSTRING,
			// 		_ 						 => return (CSS_INVALID,None)
			// 	}
			// }
			css_properties::consumeWhitespace(vector, ctx);

			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
			
			match token.token_type{ CSS_TOKEN_IDENT(_) => {}, CSS_TOKEN_STRING(_) => {}, _ => return (CSS_INVALID,None) }

			value = Some(token);

			css_properties::consumeWhitespace(vector, ctx);

			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
			
			if !css_language::tokenIsChar(token, ']') {
				return (CSS_INVALID, None)
			}	
		}
		
		 
		return css_stylesheet::css__stylesheet_selector_detail_init (tkn_type,copy *qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
							match value {Some(tkn)=>Some(lwc::lwc_string_data(tkn.idata.clone())), None => None }, None, false)
	}


	pub fn  parsePseudo(&mut self, vector:&~[~css_token], ctx:@mut uint, in_not:bool) -> (css_result,Option<@mut css_selector_detail>) {
		let mut token:&~css_token;
		let mut tkn_type = CSS_SELECTOR_PSEUDO_CLASS;
		let mut value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;
		let mut require_element:bool = false;
		let mut negate:bool = false;
		let mut lut_idx:uint;
		let mut selector_type:css_selector_type = CSS_SELECTOR_PSEUDO_CLASS;
		let qname:@mut css_qname=@mut css_qname{ns:~"", name:~""};
		/* pseudo    -> ':' ':'? [ IDENT | FUNCTION ws any1 ws ')' ] */

		let mut detail_value_string = ~"";

		if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

		token = &vector[*ctx];
		*ctx +=1; //Iterate
		
		if  !css_language::tokenIsChar(token, ':') {
			return (CSS_INVALID,None)
		}

		/* Optional second colon before pseudo element names */
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}

		token = &vector[*ctx];
		*ctx +=1; //Iterate

		if css_language::tokenIsChar(token, ':') {
			/* If present, we require a pseudo element */
			require_element = true;

			/* Consume subsequent token */
			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
		}

		/* Expect IDENT or FUNCTION */
		match token.token_type {
			CSS_TOKEN_IDENT(_) => {},
			CSS_TOKEN_FUNCTION (_) => {},
			_ => return (CSS_INVALID, None)
		} 
			
		qname.name=lwc::lwc_string_data(token.idata.clone());
		
		/* Search lut for selector type */
		match self.strings.is_selector_pseudo(copy qname.name) {
			Some((sel_type,idx)) => {
				lut_idx = idx as uint;
				tkn_type = selector_type
			},	
			None => return (CSS_INVALID, None) // Not found: invalid */
		}
		
		/* Required a pseudo element, but didn't find one: invalid */
		if require_element && match tkn_type {CSS_SELECTOR_PSEUDO_ELEMENT => false, _ => true} {
			return (CSS_INVALID, None)	
		}	

		/* :not() and pseudo elements are not permitted in :not() */
		if in_not && (match tkn_type {CSS_SELECTOR_PSEUDO_ELEMENT => true, _ => false} || match self.strings.pseudo_class_list[lut_idx] {NOT => true, _  => false} ) {
			return (CSS_INVALID, None)	
		}	

		if match token.token_type { CSS_TOKEN_FUNCTION(_) => true, _ => false} {
			
			let mut fun_type = match tkn_type{ CSS_SELECTOR_PSEUDO_ELEMENT => self.strings.pseudo_element_list[lut_idx],_ => self.strings.pseudo_class_list[lut_idx]} ;

			css_properties::consumeWhitespace(vector, ctx);

			match fun_type {
				LANG => {
					/* IDENT */
					if *ctx >= vector.len() {
						return (CSS_INVALID, None)
					}

					token = &vector[*ctx];
					*ctx +=1; //Iterate
					
					match token.token_type {
						CSS_TOKEN_IDENT(_) => {},
						_ => return (CSS_INVALID, None)
					 } 
						
					detail_value_string = lwc::lwc_string_data(token.idata.clone());
					value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;

					css_properties::consumeWhitespace(vector, ctx);
				}, 
				NTH_CHILD | NTH_LAST_CHILD | NTH_OF_TYPE | NTH_LAST_OF_TYPE => {
					/* an + b */
					match self.parseNth(vector, ctx) {
						(CSS_OK, Some(specific)) => {
							specific.value_type = CSS_SELECTOR_DETAIL_VALUE_NTH;
							// Iterate to the next location
							if *ctx >= vector.len() {
								return (CSS_INVALID, None)
							}
										
							token = &vector[*ctx];
							*ctx += 1 ; 
							
							if !css_language::tokenIsChar(token, ')') {
								return (CSS_INVALID, None)
							}

							return (CSS_OK,Some(specific))
						},	
						(error,_) => return (error,None)
					}
					
				},	
				NOT => {
					// type_selector | specific */
					
					if *ctx >= vector.len() {
						return (CSS_INVALID, None)
					}
						
					token = &vector[*ctx];
						
					if (match token.token_type {	CSS_TOKEN_IDENT(_) => true, _  => false }) || 
							css_language::tokenIsChar(token, '*') || css_language::tokenIsChar(token, '|') {
						/* Have type selector */
						match self.parseTypeSelector(vector, ctx, qname) {
							CSS_OK => {
								tkn_type = CSS_SELECTOR_ELEMENT;

								detail_value_string = ~"";
								value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;
							},
							x => return (x, None)
						}						
					} 
					else {
						/* specific */
						let mut det:css_selector_detail;

						match self.parseSpecific(vector, ctx, true) {
							(CSS_OK,Some(specific)) => {
								specific.negate = true;
								css_properties::consumeWhitespace(vector, ctx);
								if *ctx >= vector.len() {
									return (CSS_INVALID, None)
								}
											
								token = &vector[*ctx];
								*ctx += 1 ;
								
								if !css_language::tokenIsChar(token, ')') {
									return (CSS_INVALID, None)
								}

								return (CSS_OK,Some(specific))
							}	
							(error,_) => return (error,None)
						}  
					}

					negate = true;
					css_properties::consumeWhitespace(vector, ctx)
				},
				_ => {}
			}

			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}
						
			token = &vector[*ctx];
			*ctx += 1 ;
			
			if !css_language::tokenIsChar(token, ')') {
				return (CSS_INVALID, None)
			} 
		
		}

		return css_stylesheet::css__stylesheet_selector_detail_init( tkn_type,copy *qname, value_type, Some(detail_value_string), None, negate);
	}

	pub fn  parseNth(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result,Option<@mut css_selector_detail>) {
		

		return (CSS_OK,None)
	}
	// ===========================================================================================================
	// CSS-LANGUAGE implementation/data-structs ends here 
	// ===========================================================================================================

	

	// ===========================================================================================================
	// CSS-FONT-FACE implementation/data-structs starts here 
	// ===========================================================================================================

	/**
	 * Parse a descriptor in an @font-face rule
	 *
	 * \param descriptor  Token for this descriptor
	 * \param vector      Vector of tokens to process
	 * \param ctx         Pointer to vector iteration context
	 * \param rule        Rule to process descriptor into
	 * \return CSS_OK on success,
	 *         CSS_BADPARM on bad parameters,
	 *         CSS_INVALID on invalid syntax,
	 *        
	 */
	pub fn css__parse_font_descriptor( descriptor:&~css_token, vector:&~[~css_token], ctx:@mut uint, curRule:@mut css_rule_font_face) -> css_result {
						
		CSS_INVALID
	}   
	
	// ===========================================================================================================
	// PARSE-IMPORTANT implementation/data-structs starts here 
	// ===========================================================================================================

	/**
	* Parse !important
	*
	* \param c       Parsing context
	* \param vector  Vector of tokens to process
	* \param ctx     Pointer to vector iteration context
	* \param result  Pointer to location to receive result
	* \return CSS_OK on success,
	*         CSS_INVALID if "S* ! S* important" is not at the start of the vector
	*
	* Post condition: \a *ctx is updated with the next token to process
	*                 If the input is invalid, then \a *ctx remains unchanged.
	*/
	pub fn css__parse_important(&mut self, vector:&~[~css_token], ctx:@mut uint) -> u8{
		0
	}

	pub fn css__make_style_important(&mut self, style: @mut css_style) {
		
	}

}