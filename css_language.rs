#[link(name = "css_language", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_stylesheet;
extern mod std;
extern mod wapcaplet;
extern mod css_propstrings_parallel;


use css_enum::* ;
use css_stylesheet::*;
use std::arc;
use wapcaplet::*;
use css_propstrings_parallel::*;

pub struct context_entry {
	event_type:css_parser_event,        /* < Type of entry */
	data:Option<CSS_RULE_DATA_TYPE>     /*< Data for context */
} 

pub struct css_token {
	token_type: css_token_type,
	data: ~[u8],
	idata: arc::RWARC<~lwc_string>,
	// col: u32,
	// line: u32
}

pub struct css_language {
		sheet:@mut css_stylesheet,
		//lwc_instance:arc::RWARC<~lwc>,        
		context:~[context_entry], 
		state:language_state,   
		strings:~[~str],
		default_namespace:~str, 
		//namespaces:~[~css_namespace],   
}

//fn  css_language(sheet:@mut css_stylesheet, lwc_inst:arc::RWARC<~lwc>) -> ~css_language {
	fn  css_language(sheet:@mut css_stylesheet) -> ~css_language {
		//let empty_lwc_string = sheet.lwc_instance.lwc_intern_string(@"");
		//lwc_instance:sheet.lwc_instance,      
		// strings:copy sheet.propstrings,
		// default_namespace:empty_lwc_string,  
		// namespaces:@css_namespace
		// {
		//  prefix:empty_lwc_string,    
		//  uri:empty_lwc_string    
		// },   
		// let lwc_inst=lwc();
		// let empty_lwc_string = lwc_inst.lwc_intern_string(@"");
		

	~css_language {
		sheet:sheet,
		//lwc_instance:lwc_inst.clone(),
		context:~[], 
		state:CHARSET_PERMITTED,
		strings:~[],
		default_namespace:~"",   
		//namespaces:~[~css_namespce{prefix:lwc_inst.lwc_intern_string(@""), uri:lwc_inst.lwc_intern_string(@"")}]
	}
}


pub impl css_language
{
	
	pub fn language_handle_event(&mut self, event_type:css_parser_event, tokens:~[~css_token])-> css_result
		{
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


	pub fn handleStartStylesheet(&mut self ) -> css_result
		{
			let entry:context_entry = context_entry 
										{
											event_type: CSS_PARSER_START_STYLESHEET, 
											data:None                                       
										 };
					
			self.context.push(entry);
			CSS_OK
		}

		pub fn handleEndStylesheet(&mut self)->css_result
		{
			if vec::is_empty(self.context)
			{
				return CSS_INVALID
			}
			match self.context.last().event_type 
			{
				CSS_PARSER_START_STYLESHEET => {},
										_   =>return CSS_INVALID
			}

			self.context.pop();
			CSS_OK
		}

		pub fn handleStartRuleset(&mut self, tokens:~[~css_token]) ->css_result 
		{
			
			let mut cur:context_entry ;
			let mut parent_rule :Option<CSS_RULE_DATA_TYPE> = None ;

			/* Retrieve parent rule from stack, if any */
			if !vec::is_empty(self.context)
			{
				cur=self.context[self.context.len()-1];
				match cur.event_type
				{
					CSS_PARSER_START_STYLESHEET =>{},
					_=>{parent_rule = cur.data;}
				}
			}
			
			let mut curRule = self.sheet.css_stylesheet_rule_create(CSS_RULE_SELECTOR);
			
			if !vec::is_empty(tokens)
			{
				match self.parseSelectorList(&tokens, curRule)
				{
					CSS_OK => {},
					x      =>   return x  
				}
			}

			let mut entry:context_entry = context_entry 
										{
											event_type: CSS_PARSER_START_STYLESHEET, 
											data:Some(curRule)
										 };
			self.context.push(entry);

		
			match css_stylesheet::css__stylesheet_add_rule(self.sheet, curRule, parent_rule)
			{
				CSS_OK => {},
				x      => {
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

	pub fn handleEndRuleset(&mut self)->css_result
	{
				
		let mut cur:context_entry;
		
		/* Retrieve parent rule from stack, if any */
			if !vec::is_empty(self.context)
			{
				cur=self.context[self.context.len()-1];
				match cur.event_type
				{
					CSS_PARSER_START_RULESET => {
													self.context.pop();
													CSS_OK
												},
					_                        =>     CSS_INVALID
				}
			}
			else 
			{
				CSS_INVALID
			}
	}

	pub fn handleStartAtRule(&self, vector:~[~css_token])->css_result
	{
		CSS_OK  
	}

	pub fn handleEndAtRule(&mut self)->css_result
	{
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context)
		{
			cur=self.context[self.context.len()-1];
			match cur.event_type
			{
				CSS_PARSER_START_ATRULE => {
												self.context.pop();
												CSS_OK
											},
				_                        =>     CSS_INVALID
			}
		}
		else 
		{
			CSS_INVALID
		}
	}
	

	pub fn handleStartBlock(&mut self)->css_result
	{
		
		let mut cur:context_entry;
		let mut entry:context_entry = context_entry{ event_type:CSS_PARSER_START_BLOCK, data:None };
		

		/* If the current item on the stack isn't a block, 
		 * then clone its data field. This ensures that the relevant rule
		 * is available when parsing the block contents. */
		if !vec::is_empty(self.context)
		{
			cur=self.context[self.context.len()-1];
			match cur.event_type
			{
				CSS_PARSER_START_BLOCK => {},
				_                        =>     entry.data = cur.data
			}
		}

		self.context.push(entry);
		CSS_OK  
	}

	pub fn handleEndBlock(&mut self)->css_result
	{
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context)
		{
			cur=self.context[self.context.len()-1];
			match cur.event_type
			{
				CSS_PARSER_START_BLOCK =>   
				{
					let mut curRule = cur.data;
					self.context.pop();
					/* If the block we just popped off the stack was associated with a 
* non-block stack entry, and that entry is not a top-level statement,
* then report the end of that entry, too. */
					match curRule
					{
						None => CSS_OK,
						Some(x) => 
						{   match x
							{
								RULE_SELECTOR(_) =>
									match css_stylesheet::css__stylesheet_get_base_rule(x).parent_rule
									{
										Some(pRule) => 
											match css_stylesheet::css__stylesheet_get_parent_type(pRule)
											{
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
				_    => 
					return CSS_INVALID
			} // end of match
		}
		else
		{
			return CSS_INVALID  
		}       
	}


	pub fn handleBlockContent(&mut self, tokens:~[~css_token])-> css_result
	{
		// * Block content comprises either declarations (if the current block is
		// * associated with @page, @font-face or a selector), or rulesets (if the
		// * current block is associated with @media). 
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context)
		{
			cur=self.context[self.context.len()-1];
			match cur.data
			{
				None => CSS_INVALID,
				Some(curRule) => 
				{   match curRule
					{
						RULE_SELECTOR(_) | RULE_PAGE (_) | RULE_FONT_FACE(_) =>
						{                                   
							//Expect declarations 
							return self.handleDeclaration(tokens);
						},
						RULE_MEDIA(_) =>  
						{
							// Expect rulesets 
							return self.handleStartRuleset(tokens);
						},  
						_ =>    return CSS_INVALID
					}
				}
			} // end of match
		}
		else
		{
			return CSS_INVALID  
		}       
	}

	pub fn handleDeclaration(&mut self, tokens:~[~css_token])->css_result
	{
		let ctx: @mut uint = @mut 0u;   
		 // Locations where declarations are permitted:
		 // *
		 // * + In @page
		 // * + In @font-face
		 // * + In ruleset
		 
		let mut cur:context_entry;
		
		if !vec::is_empty(self.context)
		{
			cur=self.context[self.context.len()-1];
			match cur.data
			{
				None => CSS_INVALID,
				Some(curRule) => 
				{   match curRule
					{
						RULE_SELECTOR(_) | RULE_PAGE (_) | RULE_FONT_FACE(_) =>
						{                                   
							// Strip any leading whitespace (can happen if in nested block) 
							css_language::consumeWhitespace(&tokens, ctx);

							// IDENT ws ':' ws value 
							// * 
							// * In CSS 2.1, value is any1, so '{' or ATKEYWORD => parse error
							 
							
							if tokens.len() > *ctx
							{   
								let ident =&tokens[*ctx];
								*ctx = *ctx + 1;
								match ident.token_type
								{ 
									CSS_TOKEN_IDENT(_) => 
									{
										css_language::consumeWhitespace(&tokens, ctx);
										if tokens.len() <= *ctx || !css_language::tokenIsChar(&tokens[*ctx],':')
										{
											return CSS_INVALID
										}
										else 
										{
											*ctx += 1;
											css_language::consumeWhitespace(&tokens, ctx);
											match curRule
											{
												RULE_FONT_FACE(font_face_rule) =>   
													return css_language::css__parse_font_descriptor(ident, &tokens, ctx, font_face_rule),
												_ =>    
													return css_language::parseProperty(ident, &tokens, ctx, curRule)    
											}
										 }              
									} 
									_ => return CSS_INVALID
								} 
							}
							else
							{
								return CSS_INVALID
							}       
						},
						_ =>    return CSS_INVALID
					}
				}
			} // end of match
		}
		else
		{
			return CSS_INVALID  
		}       
	}

	pub fn parseSelectorList(&self, tokens:&~[~css_token], curRule: CSS_RULE_DATA_TYPE) -> css_result
	{
		let ctx: @mut uint = @mut 0u;
		
		loop 
		{
			/* Strip any leading whitespace (can happen if in nested block) */
			css_language::consumeWhitespace(tokens, ctx);

			/* selector_list   -> selector [ ',' ws selector ]* */
			match self.parseSelector(tokens, ctx)
			{
				(CSS_OK, Some(selector)) => 
				{
					match css_stylesheet::css__stylesheet_rule_add_selector(curRule, selector)
					{
						CSS_OK =>
						{
							if *ctx < tokens.len() 
							{
								//Iterate over vector to check for invalid character
								if !css_language::tokenIsChar(&tokens[*ctx],',') 
								{
									*ctx = *ctx+1;   //For iteration to the next position
									return CSS_INVALID
								}
								
								*ctx = *ctx+1 //For iteration to the next position
								
							}
							else
							{
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
	 * Helper functions                                                           *
	 ******************************************************************************/

	/**
	 * Consume all leading whitespace tokens
	 *
	 * \param vector  The vector to consume from
	 * \param ctx     The vector's context
	 */
	pub fn consumeWhitespace(vector:&~[~css_token], ctx:@mut uint) 
	{
		loop
		{
			if *ctx < vector.len() 
			{
				match vector[*ctx].token_type
				{
					CSS_TOKEN_S =>
					{
						*ctx = *ctx+1
					},
					_  => return    
				} 
			}
			else 
			{
				break
			}
		} 
	}   

	/**
	 * Determine if a token is a character
	 *
	 * \param token  The token to consider
	 * \param c      The character to match (lowercase ASCII only)
	 * \return True if the token matches, false otherwise
	 */
	pub fn tokenIsChar(token:&~css_token, c:char) -> bool
	{
		let result = false;

		match token.token_type
		{
			CSS_TOKEN_CHAR(c) =>
				{   
					if lwc::lwc_string_length(token.idata.clone()) == 1
					{
						let mut token_char = lwc::lwc_string_data(token.idata.clone()).char_at(0);

						// Ensure lowercase comparison 
						if 'A' <= token_char && token_char <= 'Z'
						{
							token_char += 'a' - 'A'
						}
							
						if token_char == c
						{
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

	pub fn parseProperty(property:&~css_token,vector:&~[~css_token], ctx:@mut uint, curRule:CSS_RULE_DATA_TYPE) -> css_result
	{
		CSS_OK
	}

	pub fn parseSelector(&self, vector:&~[~css_token], ctx:@mut uint) -> (css_result, Option<@mut css_selector>)
	{
		
		/* selector -> simple_selector [ combinator simple_selector ]* ws
		 * 
		 * Note, however, that, as combinator can be wholly whitespace,
		 * there's an ambiguity as to whether "ws" has been reached. We 
		 * resolve this by attempting to extract a combinator, then 
		 * recovering when we detect that we've reached the end of the
		 * selector.
		 */

		match self.parseSimpleSelector(vector, ctx)
		{
			(CSS_OK, Some(selector)) =>
			{
				let mut result = selector;
				loop
				{
					if *ctx >= vector.len() || css_language::tokenIsChar(&vector[*ctx],',')
					{
						return (CSS_OK, Some(result))
					}
					else
					{
						let comb = @mut CSS_COMBINATOR_NONE;        
						match self.parseCombinator(vector, ctx, comb)
						{
							CSS_OK =>
							{
							 /* In the case of "html , body { ... }", the whitespace after
							  * "html" and "body" will be considered an ancestor combinator.
							  * This clearly is not the case, however. Therefore, as a 
							  * special case, if we've got an ancestor combinator and there 
							  * are no further tokens, or if the next token is a comma,
							  * we ignore the supposed combinator and continue. */
								match *comb
								{
									CSS_COMBINATOR_ANCESTOR => 
									{
										if *ctx >= vector.len() || css_language::tokenIsChar(&vector[*ctx],',')
										{
											loop
										}
									},
									_ =>  
									{
										match self.parseSimpleSelector(vector, ctx)
										{
											(CSS_OK, Some(other_selector)) =>
											{   
												match css_stylesheet::css__stylesheet_selector_combine(*comb, selector, other_selector)
												{
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

	pub fn parseSimpleSelector(&self, vector:&~[~css_token], ctx:@mut uint) -> (css_result, Option<@mut css_selector>)
	{
		let orig_ctx = *ctx;
		/* simple_selector  -> type_selector specifics
		 *          -> specific specifics
		 */
		if *ctx >= vector.len()
		{
			return (CSS_INVALID, None)
		}        
		
		let mut selector : @mut css_selector;
		let qname: @mut css_qname = @mut css_qname{ name:~"", ns:~""};

		//match ( vector[*ctx].token_type as uint ==    CSS_TOKEN_IDENT as uint )
		if css_language::tokenIsChar(&vector[*ctx], '*') || css_language::tokenIsChar(&vector[*ctx], '|')
		{
			
			/* Have type selector */
			match self.parseTypeSelector(vector, ctx, qname)
			{
				CSS_OK => 
				{
					selector = self.sheet.css__stylesheet_selector_create(copy *qname);
					// {
					//  CSS_OK => {},   
					//  x => 
					//  {
					//      *ctx = orig_ctx;
					//      return (x, None)
					//  }
					// }    
				},
				x => 
				{
					*ctx = orig_ctx;
					return (x, None)
				}                       
			} 
		}   
		else
		{
			/* Universal selector */
			if (self.default_namespace == ~"")
			{
				qname.ns = copy self.strings[UNIVERSAL as uint]
			}   
			else
			{
				qname.ns = copy self.default_namespace;
			}   

			qname.name = copy self.strings[UNIVERSAL as uint];

			selector =  self.sheet.css__stylesheet_selector_create(copy *qname);
			// {
			//  CSS_OK => 
			//  {
					/* Ensure we have at least one specific selector */
			match self.parseAppendSpecific(vector, ctx, selector)
			{
				CSS_OK => {},
				error  => return (error,None)
			}
			//  },
			//  (error,y) => return (error,None)
			// }
		}   
		
		
		match self.parseSelectorSpecifics(vector, ctx, selector)
		{
			CSS_OK => return (CSS_OK, Some(selector)),
			error => return (error, None)
		}   
		
	}

	pub fn parseCombinator(&self, vector:&~[~css_token], ctx:@mut uint, comb:@mut css_combinator) -> css_result
	{
		
		let mut token:&~css_token;
		/* combinator      -> ws '+' ws | ws '>' ws | ws '~' ws | ws1 */
		*comb = CSS_COMBINATOR_NONE;

		loop 
		{
			if *ctx >= vector.len()
			{
				break
			} 

			token = &vector[*ctx];
			if css_language::tokenIsChar(token, '+')
			{
				*comb = CSS_COMBINATOR_SIBLING
			}   
			else if css_language::tokenIsChar(token,  '>')
			{
				*comb = CSS_COMBINATOR_PARENT   
			}
			else if css_language::tokenIsChar(token, '~')
			{
				*comb = CSS_COMBINATOR_GENERIC_SIBLING
			}   
			else 
			{
				match token.token_type
				{
					CSS_TOKEN_S =>  *comb = CSS_COMBINATOR_ANCESTOR,
					_           =>  break
				}
			}

			*ctx += 1;  

			/* If we've seen a '+', '>', or '~', we're done. */
			if *comb as uint != CSS_COMBINATOR_ANCESTOR as uint
			{
				break
			}   

		}
		
		/* No valid combinator found */
		match *comb 
		{
			CSS_COMBINATOR_NONE => return CSS_INVALID,  
			_                   => 
			{
				/* Consume any trailing whitespace */
				css_language::consumeWhitespace(vector, ctx);
				return CSS_OK
			}
		} 
				
	}   

	pub fn parseTypeSelector(&self, vector:&~[~css_token], ctx:@mut uint, qname:@mut css_qname) -> css_result
	{
		let mut token:&~css_token;
		//let prefix:lwc_string;

		/* type_selector    -> namespace_prefix? element_name
		 * namespace_prefix -> [ IDENT | '*' ]? '|'
		 * element_name     -> IDENT | '*'
		 */
		 if *ctx >= vector.len()
		{
			return CSS_INVALID
		} 
		
		token = &vector[*ctx];
		
		if !css_language::tokenIsChar(token, '|')  
		{
			//TO DO prefix = token.idata;
			*ctx += 1; //Iterate
		}

		if ( *ctx < vector.len() && css_language::tokenIsChar(&vector[*ctx], '|')) 
		{
			
			/* Have namespace prefix */
			*ctx += 1; //Iterate

			/* Expect element_name */
			if *ctx >= vector.len() || match token.token_type { CSS_TOKEN_IDENT(_) => false, _ => true} && !css_language::tokenIsChar(&vector[*ctx], '*') 
			{
				return CSS_INVALID
			}

			//TO DO match self.lookupNamespace(prefix, qname)
			// {
			// 	CSS_OK  => qname.name = token.idata,
			// 	error   => return error
			// }   
		} 
		else 
		{
			/* No namespace prefix */
			if self.default_namespace == ~""
			{
				qname.ns = copy self.strings[UNIVERSAL as uint];
			} 
			else 
			{
				qname.ns = copy self.default_namespace
			}

			//TO DO qname.name = prefix;
		}
		
		return CSS_OK
	}   

	pub fn parseAppendSpecific(&self, vector:&~[~css_token], ctx:@mut uint, parent:@mut css_selector ) -> css_result
	{
		

		match self.parseSpecific(vector, ctx, false)
		{
			(CSS_OK,Some(specific)) => CSS_OK, //TO DO return css_stylesheet::css__stylesheet_selector_append_specific(parent,specific),
			(error,_) => return error
		}   
		
	}   

	pub fn parseSelectorSpecifics(&self, vector:&~[~css_token], ctx:@mut uint, parent:@mut css_selector ) -> css_result
	{
		return CSS_OK
	}   

	pub fn parseSpecific(&self, vector:&~[~css_token], ctx:@mut uint, in_not:bool) -> (css_result,Option<@mut css_selector_detail>)
	{
		return (CSS_OK,None)
	}

	/**
	 * Look up a namespace prefix
	 *
	 * \param c       Language parser context
	 * \param prefix  Namespace prefix to find, or NULL for none
	 * \param uri     Pointer to location to receive namespace URI
	 * \return CSS_OK on success, CSS_INVALID if prefix is not found
	 */
	//pub fn lookupNamespace(&self, prefix:@lwc_string, uri:@mut lwc_string) -> css_result
	pub fn lookupNamespace(&self, prefix:@lwc_string, qname:@mut css_qname) -> css_result
	{
		
		return CSS_OK
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
	pub fn css__parse_font_descriptor( descriptor:&~css_token, vector:&~[~css_token], ctx:@mut uint, curRule:@mut css_rule_font_face) -> css_result
	{
						
		CSS_INVALID
	}   

	
}