#[link(name = "css_language", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_stylesheet;
extern mod std;


use css_enum::* ;
use css_stylesheet::*;

pub struct context_entry {
	event_type:css_parser_event,		/* < Type of entry */
	data:Option<CSS_RULE_DATA_TYPE>		/*< Data for context */
} 

pub struct css_language {
		sheet:@mut css_stylesheet,
		//lwc_instance:sheet.lwc_instance,		
		context:~[context_entry], 
		state:language_state,	
		// strings:copy sheet.propstrings,
		// default_namespace:empty_lwc_string,	
		// namespaces:@css_namespace {	prefix:empty_lwc_string, uri:empty_lwc_string},	
		// num_namespaces:uint	
}

fn  css_language(sheet:@mut css_stylesheet) -> ~css_language {
		//let empty_lwc_string = sheet.lwc_instance.lwc_intern_string(@"");
		//lwc_instance:sheet.lwc_instance,		
		// strings:copy sheet.propstrings,
		// default_namespace:empty_lwc_string,	
		// namespaces:@css_namespace
		// {
		// 	prefix:empty_lwc_string,	
		// 	uri:empty_lwc_string	
		// },	
		// num_namespaces:0	
		// let lwc_inst=lwc();
		// let empty_lwc_string = lwc_inst.lwc_intern_string(@"");
		

	~css_language {
		sheet:sheet,
		context:~[], 
		state:CHARSET_PERMITTED,	
	}
}


impl css_language
{
	
	pub fn  language_handle_event(&mut self, event_type:css_parser_event, tokens:~[~str])-> css_result
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
					self.handleBlockContent()
				}
				
				CSS_PARSER_DECLARATION=>{
					self.handleDeclaration()
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

		pub fn handleStartRuleset(&mut self, tokens:~[~str]) ->css_result 
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
				match self.parseSelectorList(&tokens, Some(curRule))
				{
					CSS_OK => {},
					x      =>	return x  
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
					_ 						 =>		CSS_INVALID
				}
			}
			else 
			{
				CSS_INVALID
			}
	}

	pub fn handleStartAtRule(&self, vector:~[~str])->css_result
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
				_ 						 =>		CSS_INVALID
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
				_ 						 =>		entry.data = cur.data
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
						{	match x
							{
								RULE_SELECTOR(_) =>
									match css_stylesheet::css__stylesheet_get_base_rule(x).parent_rule
									{
										Some(pRule) => 
											match css_stylesheet::css__stylesheet_get_parent_type(pRule)
								   			{
								   				CSS_RULE_PARENT_STYLESHEET	=> self.handleEndRuleset(),
								   				_ => CSS_OK
								   			},
										None => CSS_OK		 				
									},
								_ => 	CSS_OK
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

	pub fn handleBlockContent(&self)->css_result
	{
		/* Block content comprises either declarations (if the current block is
	 * associated with @page, @font-face or a selector), or rulesets (if the
	 * current block is associated with @media). */

		CSS_OK	
	}

	pub fn handleDeclaration(&self)->css_result
	{
		CSS_OK	
	}

	pub fn parseSelectorList(&self, tokens:&~[~str], curRule: Option<CSS_RULE_DATA_TYPE>) -> css_result
	{
		CSS_OK
	}


}


// ===========================================================================================================
// CSS-LANGUAGE implementation/data-structs ends here 
// ===========================================================================================================

