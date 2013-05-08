#[link(name = "css_language", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_stylesheet;
extern mod std;
extern mod wapcaplet;
extern mod css_propstrings;
extern mod css_properties;
extern mod css_bytecode;

use css_enum::* ;
use css_stylesheet::*;
use std::arc;
use wapcaplet::*;
use css_propstrings::*;
use css_properties::*;
use css_bytecode::*;

/**
 * Callback to resolve an URL
 *
 * \param dict  String internment context
 * \param base  Base URI (absolute)
 * \param rel   URL to resolve, either absolute or relative to base
 * \param abs   location to receive result
 * \return CSS_OK on success, appropriate error otherwise.
 */
type css_url_resolution_fn = @extern fn (base:~str, rel:arc::RWARC<~lwc_string>) -> (css_result,Option<arc::RWARC<~lwc_string>>);
type reserved_fn = @extern fn (strings:&mut ~css_propstrings, ident:&~css_token) -> bool;

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
	resolve : css_url_resolution_fn // URL resolution function */
}

	fn  css_language(sheet:@mut css_stylesheet, lwc_inst:arc::RWARC<~lwc>, resolve:css_url_resolution_fn ) -> ~css_language {
		
	~css_language {
		sheet:sheet,
		lwc_instance: lwc_inst.clone(),
		strings: css_propstrings::css_propstrings(lwc_inst.clone()),
		properties: css_properties::css_properties(sheet),
		context:~[], 
		state:CHARSET_PERMITTED,
		default_namespace:None,   
		namespaces:~[],
		resolve : resolve
	}
}


pub impl css_language {
	
	pub fn language_handle_event(&mut self, event_type:css_parser_event, tokens:&~[~css_token])-> css_result {
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

		pub fn handleStartRuleset(&mut self, tokens:&~[~css_token]) ->css_result	{
			
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
			
			if !vec::is_empty(*tokens) {
				match self.parseSelectorList(tokens, curRule) {
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

	pub fn handleStartAtRule(&self, vector:&~[~css_token])->css_result {
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


	pub fn handleBlockContent(&mut self, tokens:&~[~css_token])-> css_result {
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

	pub fn handleDeclaration(&mut self, tokens:&~[~css_token])->css_result {
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
							consumeWhitespace(tokens, ctx);

							// IDENT ws ':' ws value 
							// * 
							// * In CSS 2.1, value is any1, so '{' or ATKEYWORD => parse error
							 
							
							if tokens.len() > *ctx {   
								let ident =&tokens[*ctx];
								*ctx = *ctx + 1;
								match ident.token_type { 
									CSS_TOKEN_IDENT(_) => {
										consumeWhitespace(tokens, ctx);
										if tokens.len() <= *ctx || !tokenIsChar(&tokens[*ctx],':') {
											return CSS_INVALID
										}
										else {
											*ctx += 1;
											consumeWhitespace(tokens, ctx);
											match curRule {
												RULE_FONT_FACE(font_face_rule) =>	
													return self.css__parse_font_descriptor(ident, tokens, ctx, font_face_rule),
												_ =>	
													return self.parseProperty(ident, tokens, ctx, curRule)	
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
			consumeWhitespace(tokens, ctx);

			/* selector_list   -> selector [ ',' ws selector ]* */
			match self.parseSelector(tokens, ctx) {
				(CSS_OK, Some(selector)) => {
					match css_stylesheet::css__stylesheet_rule_add_selector(curRule, selector) {
						CSS_OK => {
							if *ctx < tokens.len() {
								//Iterate over vector to check for invalid character
								if !tokenIsChar(&tokens[*ctx],',') {
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

	/******************************************************************************
	 * Property parsing functions                             *
	 ******************************************************************************/

	pub fn parseProperty(&mut self , property: &~css_token , vector: &~[~css_token], ctx:@mut uint, curRule: CSS_RULE_DATA_TYPE) -> css_result {
		
		let mut style: @mut css_style;
		let mut index = AZIMUTH as uint;

		while (index < Z_INDEX as uint) {
			if self.strings.lwc_string_caseless_isequal(property.idata.get_ref().clone() , index) {
				break
			}
			index +=1;
		}

		if index == Z_INDEX as uint + 1 {
			return CSS_INVALID;
		}

		style = self.sheet.css__stylesheet_style_create();

		(*self.properties.property_handlers[index - AZIMUTH as uint])(self.sheet , &mut self.strings , vector , ctx , style);

		let (status,flags) = self.css__parse_important(vector , ctx);
		if match status { CSS_OK => false, _ => true} {
			return CSS_INVALID;
		}
		consumeWhitespace(vector , ctx);

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
					if *ctx >= vector.len() || tokenIsChar(&vector[*ctx],',') {
						return (CSS_OK, Some(result))
					}
					else {
						let comb = @mut CSS_COMBINATOR_NONE;        
						match self.parseCombinator(vector, ctx, comb) {
							CSS_OK => {
								/* In the of "html , body { ... }", the whitespace after
								 * "html" and "body" will be considered an ancestor combinator.
								 * This clearly is not the case, however. Therefore, as a 
								 * special case, if we've got an ancestor combinator and there 
								 * are no further tokens, or if the next token is a comma,
								 * we ignore the supposed combinator and continue. */
								match *comb {
									CSS_COMBINATOR_ANCESTOR => {
										if *ctx >= vector.len() || tokenIsChar(&vector[*ctx],',') {
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
		if tokenIsChar(&vector[*ctx], '*') || tokenIsChar(&vector[*ctx], '|') {
			
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
			if tokenIsChar(token, '+') {
				*comb = CSS_COMBINATOR_SIBLING
			}   
			else if tokenIsChar(token,  '>') {
				*comb = CSS_COMBINATOR_PARENT   
			}
			else if tokenIsChar(token, '~') {
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
				consumeWhitespace(vector, ctx);
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
		
		if !tokenIsChar(token, '|') {
			 prefix = Some(token.idata.get_ref().clone());
			*ctx += 1; //Iterate
		}

		if ( *ctx < vector.len() && tokenIsChar(&vector[*ctx], '|')) {
			
			/* Have namespace prefix */
			*ctx += 1; //Iterate

			/* Expect element_name */
			if *ctx >= vector.len() || ( match vector[*ctx].token_type { CSS_TOKEN_IDENT(_) => false, _ => true} && !tokenIsChar(&vector[*ctx], '*') ) {
				return CSS_INVALID
			}
			*ctx += 1; //Iterate

			match self.lookupNamespace(prefix, qname) {
				CSS_OK  => qname.name = lwc::lwc_string_data(vector[*ctx].idata.get_ref().clone()),
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
					!tokenIsChar(token, '+')  &&
					!tokenIsChar(token, '>')  &&
					!tokenIsChar(token, '~')  &&
					!tokenIsChar(token, ',') {
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
				let qname:css_qname=css_qname{ns:~"", name:lwc::lwc_string_data(token.idata.get_ref().clone())};
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
				if (tokenIsChar(token, '.')) {
					self.parseClass(vector, ctx)
				} 
				else if (tokenIsChar(token, '[')) {
					self.parseAttrib(vector, ctx)
				}
				else if (tokenIsChar(token, ':')) {
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
		
		if !tokenIsChar(token, '.') {
			return (CSS_INVALID,None)
		}	

		token = &vector[*ctx];
		*ctx +=1; //Iterate	

		match token.token_type {
			CSS_TOKEN_IDENT(_) => {
				let qname:css_qname=css_qname{ns:~"", name:lwc::lwc_string_data(token.idata.get_ref().clone())};
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
		
		if !tokenIsChar(token, '[') {
			return (CSS_INVALID,None)
		}	

		consumeWhitespace(vector, ctx);

		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}

		token = &vector[*ctx];
		*ctx +=1; //Iterate				

		if (match token.token_type { CSS_TOKEN_IDENT(_) => false, _ => true}) && !tokenIsChar(token, '*') &&
				!tokenIsChar(token, '|') {
			return (CSS_INVALID, None)
		}	
		
		let mut prefix: Option<arc::RWARC<~lwc_string>> = None;

		if tokenIsChar(token, '|') {
			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
		} 
		else if (*ctx < vector.len() && tokenIsChar(&vector[*ctx], '|')) {
			prefix = Some(token.idata.get_ref().clone());
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

		qname.name = lwc::lwc_string_data(vector[*ctx].idata.get_ref().clone());

		consumeWhitespace(vector, ctx);

		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}

		token = &vector[*ctx];
		*ctx +=1; //Iterate

		let mut tkn_type = CSS_SELECTOR_ATTRIBUTE;
		let mut value:Option<&~css_token> = None;

		if !tokenIsChar(token, ']') {
			if tokenIsChar(token, '=') {
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
			consumeWhitespace(vector, ctx);

			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
			
			match token.token_type{ CSS_TOKEN_IDENT(_) => {}, CSS_TOKEN_STRING(_) => {}, _ => return (CSS_INVALID,None) }

			value = Some(token);

			consumeWhitespace(vector, ctx);

			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

			token = &vector[*ctx];
			*ctx +=1; //Iterate
			
			if !tokenIsChar(token, ']') {
				return (CSS_INVALID, None)
			}	
		}
		
		 
		return css_stylesheet::css__stylesheet_selector_detail_init (tkn_type,copy *qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
							match value {Some(tkn)=>Some(lwc::lwc_string_data(tkn.idata.get_ref().clone())), None => None }, None, false)
	}


	pub fn  parsePseudo(&mut self, vector:&~[~css_token], ctx:@mut uint, in_not:bool) -> (css_result,Option<@mut css_selector_detail>) {
		let mut token:&~css_token;
		//let mut tkn_type = CSS_SELECTOR_PSEUDO_CLASS;
		let mut value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;
		let mut require_element:bool = false;
		let mut negate:bool = false;
		let mut lut_idx:uint;
		let mut selector_type:css_selector_type;
		let qname:@mut css_qname=@mut css_qname{ns:~"", name:~""};
		/* pseudo    -> ':' ':'? [ IDENT | FUNCTION ws any1 ws ')' ] */

		let mut detail_value_string = ~"";

		if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}

		token = &vector[*ctx];
		*ctx +=1; //Iterate
		
		if  !tokenIsChar(token, ':') {
			return (CSS_INVALID,None)
		}

		/* Optional second colon before pseudo element names */
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}

		token = &vector[*ctx];
		*ctx +=1; //Iterate

		if tokenIsChar(token, ':') {
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
			
		qname.name=lwc::lwc_string_data(token.idata.get_ref().clone());
		
		/* Search lut for selector type */
		match self.strings.is_selector_pseudo(copy qname.name) {
			Some((sel_type,idx)) => {
				lut_idx = idx as uint;
				selector_type = sel_type
			},	
			None => return (CSS_INVALID, None) // Not found: invalid */
		}
		
		/* Required a pseudo element, but didn't find one: invalid */
		if require_element && match selector_type {CSS_SELECTOR_PSEUDO_ELEMENT => false, _ => true} {
			return (CSS_INVALID, None)	
		}	

		/* :not() and pseudo elements are not permitted in :not() */
		if in_not && (match selector_type {CSS_SELECTOR_PSEUDO_ELEMENT => true, _ => false} || match self.strings.pseudo_class_list[lut_idx] {NOT => true, _  => false} ) {
			return (CSS_INVALID, None)	
		}	

		if match token.token_type { CSS_TOKEN_FUNCTION(_) => true, _ => false} {
			
			let mut fun_type = match selector_type{ CSS_SELECTOR_PSEUDO_ELEMENT => self.strings.pseudo_element_list[lut_idx],_ => self.strings.pseudo_class_list[lut_idx]} ;

			consumeWhitespace(vector, ctx);

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
						
					detail_value_string = lwc::lwc_string_data(token.idata.get_ref().clone());
					value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;

					consumeWhitespace(vector, ctx);
				}, 
				NTH_CHILD | NTH_LAST_CHILD | NTH_OF_TYPE | NTH_LAST_OF_TYPE => {
					/* an + b */
					match self.parseNth(vector, ctx) {
						(CSS_OK, Some(specific)) => {
							
							specific.selector_type = selector_type;
							// Iterate to the next location
							if *ctx >= vector.len() {
								return (CSS_INVALID, None)
							}
										
							token = &vector[*ctx];
							*ctx += 1 ; 
							
							if !tokenIsChar(token, ')') {
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
							tokenIsChar(token, '*') || tokenIsChar(token, '|') {
						/* Have type selector */
						match self.parseTypeSelector(vector, ctx, qname) {
							CSS_OK => {
								selector_type = CSS_SELECTOR_ELEMENT;

								detail_value_string = ~"";
								value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;
							},
							x => return (x, None)
						}						
					} 
					else {
						/* specific */
						
						match self.parseSpecific(vector, ctx, true) {
							(CSS_OK,Some(specific)) => {
								specific.negate = true;
								consumeWhitespace(vector, ctx);
								if *ctx >= vector.len() {
									return (CSS_INVALID, None)
								}
											
								token = &vector[*ctx];
								*ctx += 1 ;
								
								if !tokenIsChar(token, ')') {
									return (CSS_INVALID, None)
								}

								return (CSS_OK,Some(specific))
							}	
							(error,_) => return (error,None)
						}  
					}

					negate = true;
					consumeWhitespace(vector, ctx)
				},
				_ => {}
			}

			if *ctx >= vector.len() {
				return (CSS_INVALID, None)
			}
						
			token = &vector[*ctx];
			*ctx += 1 ;
			
			if !tokenIsChar(token, ')') {
				return (CSS_INVALID, None)
			} 
		
		}

		return css_stylesheet::css__stylesheet_selector_detail_init(selector_type,copy *qname, value_type, Some(detail_value_string), None, negate);
	}

	pub fn  parseNth(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result,Option<@mut css_selector_detail>) {
	
		let mut token:&~css_token;
		let mut negate:bool = false;
		let qname:css_qname = css_qname{ name:~"", ns:~""};
		let mut value: @mut css_selector_detail = @mut css_selector_detail{
			qname:qname,
			selector_type:CSS_SELECTOR_PSEUDO_CLASS,
			combinator_type:CSS_COMBINATOR_NONE,  
			value_type:CSS_SELECTOR_DETAIL_VALUE_NTH,
			negate:negate,
	
			string:None,
			a:0,
			b:0
		};;  
		/* nth -> [ DIMENSION | IDENT ] ws [ [ CHAR ws ]? NUMBER ws ]?
		 *        (e.g. DIMENSION: 2n-1, 2n- 1, 2n -1, 2n - 1)
		 *        (e.g. IDENT: -n-1, -n- 1, -n -1, -n - 1)
		 *     -> NUMBER ws
		 *     -> IDENT(odd) ws
		 *     -> IDENT(even) ws
		 */

		// Vector Iterate
		if *ctx >= vector.len() {
			return (CSS_INVALID, None)
		}
			
		token = &vector[*ctx];
						
		match token.token_type { 
			CSS_TOKEN_IDENT(_) | CSS_TOKEN_DIMENSION(_,_,_) => {
				if (match token.token_type { CSS_TOKEN_IDENT(_) => true, _ => false}) &&
						self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), ODD as uint) {
					/* Odd */
					value.a = 2;
					value.b = 1;
				}
				else if (match token.token_type { CSS_TOKEN_IDENT(_) => true, _ => false}) &&
							self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), EVEN as uint)
				{
					/* Even */
					value.a = 2;
					value.b = 0;
				}
				else {
					/* [ DIMENSION | IDENT ] ws [ [ CHAR ws ]? NUMBER ws ]?
					 *
					 * (e.g. DIMENSION: 2n-1, 2n- 1, 2n -1, 2n - 1)
					 * (e.g. IDENT: n, -n-1, -n- 1, -n -1, -n - 1)
					 */
										
					let mut a:int;
					let mut b:int = 0;
					let mut sign:int = 1;
					let mut had_sign = false;
					let mut had_b = false;

					let mut len = lwc::lwc_string_length(token.idata.get_ref().clone());
					let mut data = lwc::lwc_string_data(token.idata.get_ref().clone());
					let mut data_index = 0;
					/* Compute a */
					if (match token.token_type {
						CSS_TOKEN_IDENT(_) => true, 
						_ => false    //TODO check this condition
					}) {
						if len < 2 {
							if (data[data_index + 0] != 'n' as u8) && (data[data_index + 0] != 'N' as u8)	{
								return (CSS_INVALID, None)
							}
								
							/* n */
							a = 1 << 10;

							data_index += 1;
							len -= 1
						} 
						else {
							if (data[data_index + 0] != '-' as u8) || ((data[data_index + 1] != 'n' as u8) && (data[data_index + 1] != 'N' as u8))
							{
								return (CSS_INVALID, None)	
							}
								
							/* -n */
							a = -1 << 10;

							data_index += 2;
							len -= 2;
						}

						if len > 0 {
							if (data[data_index + 0] != '-' as u8)
							{
								return (CSS_INVALID, None)
							}
								

							/* -n- */
							sign = -1;
							had_sign = true;

							if len > 1 {
								/* Reject additional sign */
								if (data[data_index + 1] == '-' as u8) || (data[data_index + 1] == '+' as u8)
								{
									return (CSS_INVALID, None)
								}
									

								/* -n-b */
								let (ret_b,consumed) = css__number_from_string( data, data_index + 1, true);
								b = ret_b;
								if consumed != len - 1
								{
									return (CSS_INVALID, None)
								}

								had_b = true;
							}
						}
					} 
					else {
						/* 2n */
						let mut (ret_a, consumed) = css__number_from_lwc_string(token.idata.get_ref().clone(), true);
						a = ret_a;
						if consumed == 0 || ((data[data_index + consumed] != 'n' as u8) && (data[data_index + consumed] != 'N' as u8)) {
							return (CSS_INVALID, None)
						}

						consumed += 1;
						if len - consumed > 0 {
							if (data[data_index + consumed] != '-' as u8) {
								return (CSS_INVALID, None)
							}

							/* 2n- */
							sign = -1;
							had_sign = true;

							consumed += 1;
							if len - consumed > 0 {
								let bstart:uint;

								/* Reject additional sign */
								if (data[data_index + consumed] == '-' as u8) ||	(data[data_index + consumed] == '+' as u8) {
									return (CSS_INVALID, None)
								}

								/* 2n-b */
								bstart = consumed;

								let (ret_b,consumed) = css__number_from_string( data, data_index + bstart, true);
								b= ret_b;
								if consumed != len - bstart {
									return (CSS_INVALID, None)
								}

								had_b = true;
							}
						}
					}

					if had_b == false {
						consumeWhitespace(vector, ctx);

						/* Look for optional b : [ [ CHAR ws ]? NUMBER ws ]? */
						if *ctx < vector.len() {
							token = &vector[*ctx];	
						}
						
						if (had_sign == false &&  *ctx < vector.len() &&
							 (tokenIsChar(token, '-') || tokenIsChar(token, '+'))) {
							
							*ctx += 1; //iterate

							had_sign = true;

							if tokenIsChar(token, '-'){
								sign = -1
							}	

							consumeWhitespace(vector, ctx);

							if *ctx < vector.len() {
								token = &vector[*ctx];
							}
							
						}

						/* Expect NUMBER */
						if *ctx < vector.len() && (match token.token_type 
							{ CSS_TOKEN_NUMBER(_,_) => true, _ => false }) {

							*ctx += 1;

							/* If we've already seen a sign, ensure one
							 * does not occur at the start of this token
							 */
							if had_sign && lwc::lwc_string_length(token.idata.get_ref().clone()) > 0 {
								data = lwc::lwc_string_data(token.idata.get_ref().clone());
								data_index = 0;

								if data.char_at(data_index + 0) == '-' || data.char_at(data_index + 0) == '+'
								{
									return (CSS_INVALID,None)	
								}									
							}

							let (ret_b,consumed) = css__number_from_lwc_string(token.idata.get_ref().clone(), true);
							b = ret_b;
							if consumed != lwc::lwc_string_length(token.idata.get_ref().clone())
							{
								return (CSS_INVALID, None)
							}
						}
					}

					value.a = a << 10;
					value.b = b << 10 * sign;
				}
			},
			CSS_TOKEN_NUMBER(_,_)  => {
				
				let (ret_val,consumed) = css__number_from_lwc_string(token.idata.get_ref().clone(), true);
				if consumed != lwc::lwc_string_length(token.idata.get_ref().clone())
				{
					return (CSS_INVALID, None)
				}	

				value.a = 0;
				value.b = ret_val << 10;
			} ,
			_  => return (CSS_INVALID, None)
		}
	

		consumeWhitespace(vector, ctx);
		return (CSS_OK,Some(value))
	}
	// ===========================================================================================================
	// CSS-LANGUAGE implementation/data-structs ends here 
	// ===========================================================================================================

		
	// ===========================================================================================================
	// PARSE-IMPORTANT implementation/data-structs starts here 
	// ===========================================================================================================

	/**
	* Parse !important
	*
	* \param vector  Vector of tokens to process
	* \param ctx     Pointer to vector iteration context
	* \return (CSS_OK, result) on success along with result,
	*         (CSS_INVALID, 0) if "S* ! S* important" is not at the start of the vector
	*
	* Post condition: \a *ctx is updated with the next token to process
	*                 If the input is invalid, then \a *ctx remains unchanged.
	*/
	pub fn css__parse_important(&mut self, vector:&~[~css_token], ctx:@mut uint) -> (css_result,u8){
		
		let orig_ctx = *ctx;
		let mut flags :u8 =0;
		
		consumeWhitespace(vector, ctx);

		if *ctx >= vector.len() {
			return (CSS_OK,flags)
		}
		
		let mut token = &vector[*ctx];
		*ctx += 1; //Iterate
		
		if  tokenIsChar(token, '!') {
			
			consumeWhitespace(vector, ctx);

			if *ctx >= vector.len() || match vector[*ctx].token_type { CSS_TOKEN_IDENT(_) => false, _ => true} {
				*ctx = orig_ctx;
				return (CSS_INVALID,flags)
			}
						
			token = &vector[*ctx];
			*ctx += 1; //Iterate		

			if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), IMPORTANT as uint) {
				flags |= FLAG_IMPORTANT as u8;
			} else {
				*ctx = orig_ctx;
				return (CSS_INVALID, flags);
			}
		} else {
			*ctx = orig_ctx;
			return (CSS_INVALID,flags);
		}

		return (CSS_OK,flags);
	}

	pub fn css__make_style_important(&mut self, style: @mut css_style) {
	
		let bytecode:&mut ~[u32] = &mut style.bytecode;
		let mut offset = 0;

		while offset < bytecode.len() {
			
			let opv = copy bytecode[offset];

			/* Extract opv components, setting important flag */
			let op: css_properties_e = getOpcode(opv);
			let flags = getFlags(opv) | FLAG_IMPORTANT as u8;
			let mut value = getValue(opv);

			/* Write OPV back to bytecode */
			bytecode[offset] = buildOPV(op, flags, value);

			offset += 1;

			/* Advance past any property-specific data */
			if !isInherit(opv) {
				match op {
					CSS_PROP_AZIMUTH => {
						if (value & !(AZIMUTH_BEHIND as u16)) == AZIMUTH_ANGLE as u16 {
							offset += 2; /* length + units */
						}	
					},	
					
					CSS_PROP_BORDER_TOP_COLOR |
					CSS_PROP_BORDER_RIGHT_COLOR |
					CSS_PROP_BORDER_BOTTOM_COLOR |
					CSS_PROP_BORDER_LEFT_COLOR |
					CSS_PROP_BACKGROUND_COLOR |
					CSS_PROP_COLUMN_RULE_COLOR => {
						//assert(BACKGROUND_COLOR_SET == //(enum op_background_color)BORDER_COLOR_SET);
						//assert(BACKGROUND_COLOR_SET == //(enum op_background_color)COLUMN_RULE_COLOR_SET);

						if value == (BACKGROUND_COLOR_SET as u16) {
							offset += 1; /* colour */	
						}				
					},	
					
					CSS_PROP_BACKGROUND_IMAGE |
					CSS_PROP_CUE_AFTER |
					CSS_PROP_CUE_BEFORE |
					CSS_PROP_LIST_STYLE_IMAGE => {
						//assert(BACKGROUND_IMAGE_URI == //(enum op_background_image)CUE_AFTER_URI);
						//assert(BACKGROUND_IMAGE_URI == //(enum op_background_image)CUE_BEFORE_URI);
						//assert(BACKGROUND_IMAGE_URI == //(enum op_background_image)LIST_STYLE_IMAGE_URI);

						if (value == BACKGROUND_IMAGE_URI as u16) {
							offset += 1; /* string table entry */
						}	
					},	
					
					CSS_PROP_BACKGROUND_POSITION => {
						if ((value & 0xf0) == BACKGROUND_POSITION_HORZ_SET as u16){
							offset += 2; /* length + units */
						}
							
						if ((value & 0x0f) == BACKGROUND_POSITION_VERT_SET as u16){
							offset += 2; /* length + units */
						}
					},	
					CSS_PROP_BORDER_SPACING => {
						if (value == BORDER_SPACING_SET as u16){
							offset += 4; /* two length + units */
						}	
					},

					CSS_PROP_BORDER_TOP_WIDTH |
					CSS_PROP_BORDER_RIGHT_WIDTH |
					CSS_PROP_BORDER_BOTTOM_WIDTH |
					CSS_PROP_BORDER_LEFT_WIDTH |
					CSS_PROP_OUTLINE_WIDTH |
					CSS_PROP_COLUMN_RULE_WIDTH => {
						//assert(BORDER_WIDTH_SET == //(enum op_border_width)OUTLINE_WIDTH_SET);
						//assert(BORDER_WIDTH_SET == //(enum op_border_width)COLUMN_RULE_WIDTH_SET);

						if (value == BORDER_WIDTH_SET as u16){
							offset += 2; /* length + units */
						}
					},
						
					CSS_PROP_MARGIN_TOP |
					CSS_PROP_MARGIN_RIGHT |
					CSS_PROP_MARGIN_BOTTOM |
					CSS_PROP_MARGIN_LEFT |
					CSS_PROP_BOTTOM |
					CSS_PROP_LEFT |
					CSS_PROP_RIGHT |
					CSS_PROP_TOP |
					CSS_PROP_HEIGHT |
					CSS_PROP_WIDTH |
					CSS_PROP_COLUMN_WIDTH |
					CSS_PROP_COLUMN_GAP => {
						//assert(BOTTOM_SET == //(enum op_bottom)LEFT_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)RIGHT_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)TOP_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)HEIGHT_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)MARGIN_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)WIDTH_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)COLUMN_WIDTH_SET);
						//assert(BOTTOM_SET == //(enum op_bottom)COLUMN_GAP_SET);

						if (value == BOTTOM_SET as u16) {
							offset += 2; /* length + units */
						}
					},
						
					CSS_PROP_CLIP => {
						if ((value & CLIP_SHAPE_MASK as u16) == CLIP_SHAPE_RECT as u16) {
							if ((value & CLIP_RECT_TOP_AUTO as u16) == 0) {
								offset += 2; /* length + units */
							}
								
							if ((value & CLIP_RECT_RIGHT_AUTO as u16) == 0) {
								offset += 2; /* length + units */
							}
								
							if ((value & CLIP_RECT_BOTTOM_AUTO as u16) == 0) {
								offset += 2; /* length + units */
							}
								
							if ((value & CLIP_RECT_LEFT_AUTO as u16) == 0) {
								offset += 2; /* length + units */
							}	
						}
					},

					CSS_PROP_COLOR => {
						if (value == COLOR_SET as u16) {
							offset+=1; /* colour */
						}	
					},

					CSS_PROP_COLUMN_COUNT => {
						if (value == COLUMN_COUNT_SET as u16) {
							offset+=1; /* colour */
						}	
					},

					CSS_PROP_CONTENT => {
						while  (value != CONTENT_NORMAL as u16) && (value != CONTENT_NONE as u16) {
							if value & 0xff == CONTENT_COUNTER as u16 || 
								value & 0xff == CONTENT_URI as u16 ||
								value & 0xff == CONTENT_ATTR as u16 || 
								value & 0xff == CONTENT_STRING as u16 {
									offset+=1; /* string table entry */
							}

							if	value & 0xff == CONTENT_COUNTERS as u16 {
									offset+=2; /* two string entries */
							}

							//NOT NEEDED 
							//if value & 0xff ==	CONTENT_OPEN_QUOTE as u16 ||
							// 	value & 0xff == CONTENT_CLOSE_QUOTE as u16 ||
							// 	value & 0xff == CONTENT_NO_OPEN_QUOTE as u16 ||
							// 	value & 0xff == CONTENT_NO_CLOSE_QUOTE as u16 {
							// 	//Do Nothing	
							// }

							value = bytecode[offset] as u16;
						    offset += 1;
						}
					},

					CSS_PROP_COUNTER_INCREMENT |
					CSS_PROP_COUNTER_RESET => {
						//assert(COUNTER_INCREMENT_NONE == //(enum op_counter_increment)COUNTER_RESET_NONE);

						while value != COUNTER_INCREMENT_NONE as u16 {
							offset+=2; /* string + integer */

							value = bytecode[offset] as u16;
						        offset+=1;
						}
					}

					CSS_PROP_CURSOR => {
						while value == CURSOR_URI as u16 {
							offset += 1; /* string table entry */

							value = bytecode[offset] as u16;
						        offset += 1;
						}
					},

					CSS_PROP_ELEVATION => {
						if (value == ELEVATION_ANGLE as u16) {
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_FONT_FAMILY => {
						while (value != FONT_FAMILY_END as u16) {
							if value == FONT_FAMILY_STRING as u16 ||
								value == FONT_FAMILY_IDENT_LIST as u16 {
									offset += 1; /* string table entry */
							}

							value = bytecode[offset] as u16;
					        offset += 1;
						}
					}

					CSS_PROP_FONT_SIZE => {
						if (value == FONT_SIZE_DIMENSION as u16) {
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_LETTER_SPACING |
					CSS_PROP_WORD_SPACING => {
						//assert(LETTER_SPACING_SET == //(enum op_letter_spacing)WORD_SPACING_SET);

						if (value == LETTER_SPACING_SET as u16) {
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_LINE_HEIGHT => {
						if value == LINE_HEIGHT_NUMBER as u16 {
							offset += 1; /* value */
						}
						else if value == LINE_HEIGHT_DIMENSION as u16 {
							offset += 2; /* length + units */
						}
					},

					CSS_PROP_MAX_HEIGHT|
					CSS_PROP_MAX_WIDTH => {
						//assert(MAX_HEIGHT_SET == (enum op_max_height)MAX_WIDTH_SET);

						if (value == MAX_HEIGHT_SET as u16){
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_PADDING_TOP|
					CSS_PROP_PADDING_RIGHT|
					CSS_PROP_PADDING_BOTTOM|
					CSS_PROP_PADDING_LEFT|
					CSS_PROP_MIN_HEIGHT|
					CSS_PROP_MIN_WIDTH|
					CSS_PROP_PAUSE_AFTER|
					CSS_PROP_PAUSE_BEFORE|
					CSS_PROP_TEXT_INDENT => {
						//assert(MIN_HEIGHT_SET == (enum op_min_height)MIN_WIDTH_SET);
						//assert(MIN_HEIGHT_SET == (enum op_min_height)PADDING_SET);
						//assert(MIN_HEIGHT_SET == (enum op_min_height)PAUSE_AFTER_SET);
						//assert(MIN_HEIGHT_SET == (enum op_min_height)PAUSE_BEFORE_SET);
						//assert(MIN_HEIGHT_SET == (enum op_min_height)TEXT_INDENT_SET);

						if (value == MIN_HEIGHT_SET as u16) {
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_OPACITY => {
						if (value == OPACITY_SET as u16) {
							offset += 1; /* value */
						}	
					},

					CSS_PROP_ORPHANS|
					CSS_PROP_PITCH_RANGE|
					CSS_PROP_RICHNESS|
					CSS_PROP_STRESS|
					CSS_PROP_WIDOWS => {
						//assert(ORPHANS_SET == //(enum op_orphans)PITCH_RANGE_SET);
						//assert(ORPHANS_SET == //(enum op_orphans)RICHNESS_SET);
						//assert(ORPHANS_SET == //(enum op_orphans)STRESS_SET);
						//assert(ORPHANS_SET == //(enum op_orphans)WIDOWS_SET);

						if (value == ORPHANS_SET as u16) {
							offset += 1; /* value */
						}	
					},

					CSS_PROP_OUTLINE_COLOR => {
						if (value == OUTLINE_COLOR_SET as u16) {
							offset += 1; /* color */
						}	
					},

					CSS_PROP_PITCH => {
						if (value == PITCH_FREQUENCY as u16) {
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_PLAY_DURING => {
						if (value == PLAY_DURING_URI as u16) {
							offset += 1; /* string table entry */
						}	
					},

					CSS_PROP_QUOTES => {
						while (value != QUOTES_NONE as u16) {
							offset += 2; /* two string table entries */

							value = bytecode[offset] as u16;
						        offset += 1;
						}
					},

					CSS_PROP_SPEECH_RATE => {
						if (value == SPEECH_RATE_SET as u16) {
							offset += 1; /* rate */
						}	
					},

					CSS_PROP_VERTICAL_ALIGN => {
						if (value == VERTICAL_ALIGN_SET as u16) {
							offset += 2; /* length + units */
						}	
					},

					CSS_PROP_VOICE_FAMILY => {
						while (value != VOICE_FAMILY_END as u16) {
							if value == VOICE_FAMILY_STRING as u16 ||
								value == VOICE_FAMILY_IDENT_LIST as u16 {
									offset += 1; /* string table entry */
							}

							value = bytecode[offset] as u16;
							offset += 1;
						}
					},

					CSS_PROP_VOLUME => {
						if value ==	VOLUME_NUMBER as u16 {
								offset += 1; /* value */
						}
						else if value == VOLUME_DIMENSION as u16 {
								offset += 2; /* value + units */
						}
					},

					CSS_PROP_Z_INDEX => {
						if (value == Z_INDEX_SET as u16){
							offset += 1; /* z index */
						}	
					},

					_ =>  {}
				}
			}
		}

	}

	// ===========================================================================================================
	// PARSE-IMPORTANT implementation/data-structs ends here 
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
	pub fn css__parse_font_descriptor(&mut self, descriptor:&~css_token, vector:&~[~css_token], ctx:@mut uint, curRule:@mut css_rule_font_face) -> css_result {
		
		let font_face:Option<@mut css_font_face> = curRule.font_face;
		
		if self.strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(), FONT_FAMILY as uint) {
			return self.font_face_parse_font_family(vector, ctx, *font_face.get_ref())
		}
		else if self.strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(),SRC as uint) {
			return self.font_face_parse_src(vector, ctx, *font_face.get_ref())
		}
		else if self.strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(),FONT_STYLE as uint) {
			return self.font_face_parse_font_style(vector, ctx, *font_face.get_ref())
		}
		else if self.strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(),FONT_WEIGHT as uint) {
			return self.font_face_parse_font_weight(vector, ctx, *font_face.get_ref())
		}
		
		CSS_INVALID
	}   

	pub fn font_face_parse_font_family(&mut self, vector:&~[~css_token], ctx:@mut uint,	
		font_face:@mut css_font_face) -> css_result	{
		
		match self.css__ident_list_or_string_to_string(vector, ctx,Some(@css_language::font_rule_font_family_reserved))
		{
			(CSS_OK,Some(string)) => { 
				self.css__font_face_set_font_family(font_face, string);
				return CSS_OK
			},
			(error,_) => return error
		}
		
	}

	pub fn font_face_parse_src(&mut self, vector:&~[~css_token], ctx:@mut uint,
		font_face:@mut css_font_face) -> css_result	{
	
		let orig_ctx = *ctx;
		let mut srcs:~[~css_font_face_src]=~[];
				
		/* src             ::= spec-or-name [ ',' spec-or-name ]*
	 	 * spec-or-name    ::= font-face-spec | font-face-name
	 	 * font-face-spec  ::= URI [ 'format(' STRING [ ',' STRING ]* ')' ]?
	 	 * font-face-name  ::= 'local(' ident-list-or-string ')'
	 	 * ident-list-or-string ::= IDENT IDENT* | STRING
	 	 */

		/* Create one css_font_face_src for each consecutive location and
		 * [potentially] type pair in the comma-separated list
		 */
		loop {	
			
			let location_type:@mut css_font_face_location_type =@mut CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED;
			let format:@mut css_font_face_format =@mut CSS_FONT_FACE_FORMAT_UNSPECIFIED;
			let mut new_src:~css_font_face_src =~css_font_face_src{location:None, bits:~[]};			

			match self.font_face_src_parse_spec_or_name(vector, ctx, location_type, format) {
				(CSS_OK,location) => {
					new_src.location = location;
					new_src.bits.push(*format as u8 << 2 | *location_type as u8 );
					srcs.push(new_src);
				},
				(error,_) => {
					*ctx = orig_ctx;
					return error
				}	
			}
			
								
			consumeWhitespace(vector, ctx);
			if *ctx < vector.len() && tokenIsChar(&vector[*ctx], ',')
			{
				*ctx += 1 //Iterate
			} 
			else
			{
				*ctx += 1; //Iterate
				break
			}
		
		}

		match self.css__font_face_set_srcs(font_face, srcs) {
			CSS_OK => return CSS_OK, 
			error => {
				*ctx = orig_ctx;
				return error
			} 
		}
		
	}

	pub fn font_face_parse_font_style(&mut self, vector:&~[~css_token], ctx:@mut uint,
		font_face:@mut css_font_face) -> css_result	{
	
		let orig_ctx = *ctx;
		let style:css_font_style_e;
		

		/* IDENT(normal, italic, oblique) */

		if *ctx > vector.len() || match vector[*ctx].token_type {CSS_TOKEN_IDENT(_) => false, _ => true} {
			*ctx = orig_ctx;
			return CSS_INVALID
		}	
		
		let token = &vector[*ctx];
		*ctx += 1;

		if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), NORMAL as uint) {
			style = CSS_FONT_STYLE_NORMAL;
		} 
		else if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), ITALIC as uint) {
			style = CSS_FONT_STYLE_ITALIC
		}
		else if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), OBLIQUE as uint) {
			style = CSS_FONT_STYLE_OBLIQUE
		} else {
			*ctx = orig_ctx;
			 return CSS_INVALID;
		}

		
		font_face.bits[0] = (font_face.bits[0] & 0xfc) | style as u8;
		return CSS_OK;
	}

	pub fn font_face_parse_font_weight(&mut self, vector:&~[~css_token], ctx:@mut uint,
		font_face:@mut css_font_face) -> css_result	{
	
		let orig_ctx = *ctx;
		let token:&~css_token;
		let weight:css_font_weight_e;
		
		/* NUMBER (100, 200, 300, 400, 500, 600, 700, 800, 900) | 
		 * IDENT (normal, bold) */
		
		if *ctx >= vector.len() || match vector[*ctx].token_type { CSS_TOKEN_IDENT(_) | CSS_TOKEN_NUMBER(_,_) => false, _ => true } {
			return CSS_INVALID;
		}

		token = &vector[*ctx];
		*ctx += 1;


		if match token.token_type { CSS_TOKEN_NUMBER(_,_) => true, _ => false }  {
			let (num, consumed) = css__number_from_lwc_string(token.idata.get_ref().clone(), true);
			/* Invalid if there are trailing characters */
			if consumed != lwc::lwc_string_length(token.idata.get_ref().clone()) {
				*ctx = orig_ctx;
				return CSS_INVALID;
			}

			match num >> 10 {
				100 => weight = CSS_FONT_WEIGHT_100,
				200 => weight = CSS_FONT_WEIGHT_200,
				300 => weight = CSS_FONT_WEIGHT_300,
				400 => weight = CSS_FONT_WEIGHT_400,
				500 => weight = CSS_FONT_WEIGHT_500,
				600 => weight = CSS_FONT_WEIGHT_600,
				700 => weight = CSS_FONT_WEIGHT_700,
				800 => weight = CSS_FONT_WEIGHT_800,
				900 => weight = CSS_FONT_WEIGHT_900,
				_ =>  {
					*ctx = orig_ctx;
					return CSS_INVALID
				}
			}	
		} 
		else if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),NORMAL as uint) {
			weight = CSS_FONT_WEIGHT_NORMAL
		} 
		else if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),BOLD as uint) {
			weight = CSS_FONT_WEIGHT_BOLD
		} else {
			*ctx = orig_ctx;
			return CSS_INVALID
		}

		font_face.bits[0] = (font_face.bits[0] & 0xc3) | (weight as u8 << 2);
		
		return CSS_OK;
	}

	pub fn font_face_src_parse_spec_or_name(&mut self, vector:&~[~css_token], ctx:@mut uint, 
		location_type:@mut css_font_face_location_type, format:@mut	css_font_face_format) -> (css_result, Option<arc::RWARC<~lwc_string>>)
	{
		let mut token: &~css_token;
		let location:arc::RWARC<~lwc_string>;
	 	/* spec-or-name    ::= font-face-spec | font-face-name
	 	 * font-face-spec  ::= URI [ 'format(' STRING [ ',' STRING ]* ')' ]?
	 	 * font-face-name  ::= 'local(' ident-list-or-string ')'
	 	 * ident-list-or-string ::= IDENT IDENT* | STRING
	 	 */

		consumeWhitespace(vector, ctx);

		if *ctx >= vector.len() {
			return (CSS_INVALID,None);	
		} 
		token = &vector[*ctx];
		*ctx += 1;	//Iterate

		match token.token_type {
			CSS_TOKEN_URI(_) => {
				match (*self.resolve)(copy self.sheet.url, token.idata.get_ref().clone())
				{ 
					(CSS_OK,Some(loc)) => location =loc,
					(error,_) => return (error,None)
				}	

				*location_type = CSS_FONT_FACE_LOCATION_TYPE_URI;

				consumeWhitespace(vector, ctx);

				if *ctx < vector.len() {
					
					token = &vector[*ctx];
					if match token.token_type { CSS_TOKEN_FUNCTION(_) => true, _ => false}  &&
						self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),FORMAT as uint) {
					
						*ctx += 1;

						match self.font_face_src_parse_format(vector, ctx, format) {
							CSS_OK => {},
							error => {
								return (error,None);
							}	
						}
					}
				}		
			},
			CSS_TOKEN_FUNCTION(_) => {
				if self.strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LOCAL as uint) {
					consumeWhitespace(vector, ctx);

					match self.css__ident_list_or_string_to_string(vector, ctx, None) {
						(CSS_OK,Some(loc)) => location = loc,
						(error,x) => return (error,x)
					}
					consumeWhitespace(vector, ctx);

					if *ctx >= vector.len() || !tokenIsChar(&vector[*ctx], ')') {
						*ctx +=1; //Iterate
						return (CSS_INVALID, None)
					}

					*ctx +=1; //Iterate
					*location_type = CSS_FONT_FACE_LOCATION_TYPE_LOCAL
				} 
				else {
					return (CSS_INVALID, None)
				}
			},
			_ => return (CSS_INVALID, None)		
		}

		return (CSS_OK, Some(location))
	}

	/**
	 * Set a font-faces array of srcs.
	 *
	 * \param font_face  The font-face 
	 * \param srcs	     The array of css_font_face_srcs
	 * \return CSS_OK
	 */
	pub fn css__font_face_set_srcs(&mut self, font_face:@mut css_font_face, srcs:~[~css_font_face_src]) -> css_result
	{
		font_face.srcs = srcs;
				
		CSS_OK
	}

	/**
	 * Set a font-face's font-family name
	 *
	 * \param font_face    The font-face
	 * \param font_family  Font-family name
	 * \return CSS_OK
	 *         
	 */
	pub fn css__font_face_set_font_family(&mut self, font_face: @mut css_font_face, font_family:arc::RWARC<~lwc_string>) -> css_result
	{
		// if (font_face.font_family != NULL)
		// do self.lwc_instance.write |l|{
		// 					l.lwc_string_unref(font_face.font_family)
		// }	
		font_face.font_family = Some(font_family.clone());

		return CSS_OK
	}

	pub fn font_rule_font_family_reserved(strings:&mut ~css_propstrings, ident:&~css_token) -> bool
	{
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), SERIF as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(),SANS_SERIF as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), CURSIVE as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), FANTASY as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), MONOSPACE as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), INHERIT as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), INITIAL as uint) ||
		strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), DEFAULT as uint)
	}

	pub fn font_face_src_parse_format(&mut self, vector:&~[~css_token], ctx:@mut uint, format:@mut	css_font_face_format) -> css_result	{
		
		let mut token:&~css_token;

		*format = CSS_FONT_FACE_FORMAT_UNSPECIFIED;

		/* 'format(' STRING [ ',' STRING ]* ')' 
		 * 
		 * 'format(' already consumed
		 */

		loop {
			consumeWhitespace(vector, ctx);

			if  *ctx > vector.len() || match vector[*ctx].token_type { CSS_TOKEN_STRING(_) => false, _ => true} {
				return CSS_INVALID
			} 
				
			token =&vector[*ctx];
			*ctx +=1;	//Iterate

			if self.strings.lwc_string_isequal(token.idata.get_ref().clone(), WOFF as uint) {
			    *format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_WOFF as uint) }
			} 
			else if self.strings.lwc_string_isequal(token.idata.get_ref().clone(),TRUETYPE as uint) ||
				self.strings.lwc_string_isequal(token.idata.get_ref().clone(),OPENTYPE as uint) {
				*format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_OPENTYPE as uint) }
			} 
			else if self.strings.lwc_string_isequal(token.idata.get_ref().clone(), EMBEDDED_OPENTYPE as uint) {
				*format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE as uint) }
			}
			else if self.strings.lwc_string_isequal(token.idata.get_ref().clone(),SVG as uint) {
				*format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_SVG as uint) }	
			} 
			else {
				/* The spec gives a list of possible strings, which 
				 * hints that unknown strings should be parse errors,
				 * but it also talks about "unknown font formats",
				 * so we treat any string we don't know not as a parse
				 * error, but as indicating an "unknown font format".
				 */
				*format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_UNKNOWN as uint) }		
				
			}

			consumeWhitespace(vector, ctx);
			if *ctx >= vector.len() {
				*ctx += 1;
				return CSS_INVALID
			}
			else{
				token = &vector[*ctx];
				*ctx += 1;
				if !tokenIsChar(token, ',') {
					break;
				}
			} 		
		} 
		
		if !tokenIsChar(token, ')') {
			return CSS_INVALID
		}	

		return CSS_OK
	}

	/**
	 * Create a string from a list of IDENT/S tokens if the next token is IDENT
	 * or references the next token's string if it is a STRING
	 *
	 * \param self       Parsing context
	 * \param vector     Vector containing tokens
	 * \param ctx        Vector iteration context
	 * \param reserved   Callback to determine if an identifier is reserved
	 * \param result     Location to receive resulting string
	 * \return CSS_OK on success, appropriate error otherwise.
	 *
	 * Post condition: \a *ctx is updated with the next token to process
	 *                 If the input is invalid, then \a *ctx remains unchanged.
	 *
	 *                 The resulting string's reference is passed to the caller
	 */
	pub fn css__ident_list_or_string_to_string(&mut self, vector:&~[~css_token], ctx:@mut uint,
			reserved:Option<reserved_fn>) -> (css_result, Option<arc::RWARC<~lwc_string>>)
	{
		
		//TO DO
		if *ctx >= vector.len()	{
			return (CSS_INVALID,None)
		}
		
		let mut token = &vector[*ctx];	
		
		match token.token_type {
			CSS_TOKEN_STRING(_) => {
				*ctx += 1; //Iterate
				return (CSS_OK,Some(token.idata.get_ref().clone()))
			},	
			//TO DO CSS_TOKEN_IDENT => 	return css__ident_list_to_string(c, vector, ctx, reserved, result),
			_ => return (CSS_INVALID,None)
		}	
	}


}