use wapcaplet::*;

use bytecode::opcodes::*;
use bytecode::bytecode::*;

use include::properties::*;
use include::types::*;

use stylesheet::*;

use lex::lexer::*;

use parse::common::*;
use parse::font_face::*;
use parse::propstrings::*;
use parse::properties::common::*;
use parse::properties::properties::*;

use utils::errors::*;
use std::cast::*;


enum language_state {

    CHARSET_PERMITTED,
    IMPORT_PERMITTED,
    NAMESPACE_PERMITTED,
    HAD_RULE
    
}

pub struct context_entry {
    event_type:css_parser_event,        /* < Type of entry */
    data:Option<css_rule_data_index>     //< Data for context */
} 

pub struct css_namespace {
    prefix:Option<uint> ,        /**< Namespace prefix */
    uri:Option<uint>     //< Namespace URI */
}


pub struct css_language {
    sheet:uint,
    context:~[context_entry], 
    state:language_state,   
    properties: ~css_properties,
    default_namespace:Option<uint>, 
    namespaces:~[~css_namespace]
}

pub fn css_language(sheet:uint) -> ~css_language {
    //debug!("Entering: css_language");
   
    let cr_properties = css_properties::css_properties();

    ~css_language {
        sheet:sheet,
        properties: cr_properties,
        context:~[], 
        state:CHARSET_PERMITTED,
        default_namespace:None,   
        namespaces:~[]
	}
}


impl css_language {
    
	/**
	* #Description:
	*   Handler for core parser events.

	* #Arguments:
	*  'event_type' - The event type.

	*  'tokens' - Vector of tokens read since last event.

	* #Return Value:
	*   'css_error' - CSS_OK on success, CSS_INVALID to indicate parse error.
	*/
    pub fn language_handle_event(&mut self,  stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, event_type:css_parser_event, tokens:&~[~css_token])
        -> css_error {
	let mut css_er:css_error;
        //debug!("Entering: language_handle_event");
        match event_type {
            
            CSS_PARSER_START_STYLESHEET => {
                css_er = self.handleStartStylesheet();
            }
            
            CSS_PARSER_END_STYLESHEET=>{
                css_er = self.handleEndStylesheet();
            }
            
            CSS_PARSER_START_RULESET=>{
                css_er = self.handleStartRuleset(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens);
            }
            
            CSS_PARSER_END_RULESET=>{
                css_er = self.handleEndRuleset();
            }
            
            CSS_PARSER_START_ATRULE=>{
                css_er = self.handleStartAtRule(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens);
            }
            
            CSS_PARSER_END_ATRULE=>{
                css_er = self.handleEndAtRule();
            }
            
            CSS_PARSER_START_BLOCK=>{
                css_er = self.handleStartBlock();

            }
            
            CSS_PARSER_END_BLOCK=>{
                css_er = self.handleEndBlock(stylesheet_vector, css_rule_data_list);
            }
            
            CSS_PARSER_BLOCK_CONTENT=>{
                css_er = self.handleBlockContent(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens);
            }
            
            CSS_PARSER_DECLARATION=>{
                css_er = self.handleDeclaration(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens);
            }
        }
	css_er
    }


    pub fn handleStartStylesheet(&mut self ) -> css_error {
        //debug!("Entering: handleStartStylesheet");
        let entry:context_entry = context_entry {
            event_type: CSS_PARSER_START_STYLESHEET, 
            data:None                                       
        };
        
        self.context.push(entry);
        //debug!("Exiting: handleStartStylesheet");
        CSS_OK
    }

    pub fn handleEndStylesheet(&mut self)->css_error {
        //debug!("Entering: handleEndStylesheet");
        if self.context.is_empty() {
            return CSS_INVALID
        }

        match self.context.last().event_type {
            CSS_PARSER_START_STYLESHEET => {},
            _   =>return CSS_INVALID
	}

        self.context.pop();
        CSS_OK
    }

    pub fn handleStartRuleset(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, tokens:&~[~css_token]) ->css_error    {
        //debug!("Entering: handleStartRuleset");

        let mut parent_rule :Option<uint> = None ;

        /* Retrieve parent rule from stack, if any */
        if !self.context.is_empty() {
            let cur:&context_entry = &self.context[self.context.len()-1];
            match cur.event_type {
                CSS_PARSER_START_STYLESHEET =>{},
                _=>{parent_rule = cur.data;}
            }
        }
        
        let curRule = stylesheet_vector[self.sheet].css_stylesheet_rule_create(css_rule_data_list, CSS_RULE_SELECTOR);
        
        //if !vec::is_empty(*tokens) {
        match self.parseSelectorList(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens, curRule) {
            CSS_OK => {},
            x =>   return x  
		  
        }
        //}

        let entry:context_entry = context_entry {
            event_type: CSS_PARSER_START_RULESET, 
            data:Some(curRule)
        };
        self.context.push(entry);

    
        match css_stylesheet::css__stylesheet_add_rule(stylesheet_vector, css_rule_data_list, self.sheet, lwc_ref, curRule, parent_rule) {
            CSS_OK =>   {},
            x      =>   {
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

    pub fn handleEndRuleset(&mut self)->css_error {
        //debug!("Entering: handleEndRuleset");
        let mut cur:context_entry;
        
        /* Retrieve parent rule from stack, if any */
        if !self.context.is_empty() {
            cur=self.context[self.context.len()-1];
            match cur.event_type {
                CSS_PARSER_START_RULESET => {
                    self.context.pop();
                    CSS_OK
                },
                _ =>   CSS_INVALID
            }
        }
        else {
            CSS_INVALID
        }
    }

    pub fn handleStartAtRule(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token])->css_error {
        //debug!("Entering: handleStartAtRule");
       // context_entry entry = { CSS_PARSER_START_ATRULE, NULL };
            
        let ctx: &mut uint =&mut 0;
        let mut curRule:Option<uint> = None;
        let mut token: &~css_token; 

        /* vector contains: ATKEYWORD ws any0 */

        if *ctx >= vector.len() {
            return CSS_INVALID
        }
        
        let atkeyword = &vector[*ctx];
        *ctx += 1; //Iterate

        consumeWhitespace(vector, ctx);

        /* We now have an ATKEYWORD and the context for the start of any0, if 
         * there is one */
        match atkeyword.token_type { CSS_TOKEN_ATKEYWORD => {}, _ => return CSS_INVALID };

        if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, atkeyword.idata.get_ref().clone(), CHARSET as uint) {
            match self.state {
                CHARSET_PERMITTED => {
                    /* any0 = STRING */
                    if (*ctx == 0) {
                        return CSS_INVALID;
                    }
                        
                    if *ctx >= vector.len() {
                        return CSS_INVALID
                    }

                    let charset = &vector[*ctx];
                    *ctx += 1; //Iterate
                    
                    if match charset.token_type {CSS_TOKEN_STRING => false, _ => true} {
                        return CSS_INVALID;
                    }
                    
                    if !(*ctx >= vector.len()) {
                        return CSS_INVALID
                    }

                    //token = &vector[*ctx]; Not used
                    *ctx += 1; //Iterate
                    
                    let temp_rule = stylesheet_vector[self.sheet].css_stylesheet_rule_create(css_rule_data_list, CSS_RULE_CHARSET);
                    
                    match css_stylesheet::css__stylesheet_rule_set_charset(&mut css_rule_data_list[temp_rule], lwc_ref.lwc_string_data(charset.idata.get_ref().clone())) {
                        CSS_OK => {},
                        error => return error
                    }
                    
                    match css_stylesheet::css__stylesheet_add_rule(stylesheet_vector, css_rule_data_list, self.sheet, lwc_ref, temp_rule, None){
                        CSS_OK => {},
                        error => return error

                    }

                    curRule = Some(temp_rule);
                    /* Rule is now owned by the sheet, 
                     * so no need to destroy it */

                    self.state = IMPORT_PERMITTED
                },
                _ => return CSS_INVALID
            }
        } 
        else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, atkeyword.idata.get_ref().clone(), LIBCSS_IMPORT as uint) {
            if self.state as uint <= IMPORT_PERMITTED as uint {
                let mut url:~str;
                let media:&mut u64 =&mut  0;

                /* any0 = (STRING | URI) ws 
                 *    (IDENT ws (',' ws IDENT ws)* )? */
                

                if *ctx >= vector.len() {
                    return CSS_INVALID
                }

                let uri = &vector[*ctx];
                *ctx += 1; //Iterate
                    
                if match uri.token_type { CSS_TOKEN_STRING | CSS_TOKEN_URI => false, _ => true} {
                    return CSS_INVALID
                }
                    

                consumeWhitespace(vector, ctx);

                /* Parse media list */
                match self.parseMediaList(lwc_ref, propstrings_ref, vector, ctx, media) {
                    CSS_OK => {},
                    error => return error
                }
                
                /* Create rule */
                let temp_rule = stylesheet_vector[self.sheet].css_stylesheet_rule_create(css_rule_data_list, CSS_RULE_IMPORT);

                /* Resolve import URI */
                match (*stylesheet_vector[self.sheet].resolve)(stylesheet_vector[self.sheet].url, uri.idata.get_ref().clone())
                { 
                    (CSS_OK,Some(ret_url)) => url = lwc_ref.lwc_string_data(ret_url).to_owned(),
                    (error,_) => return error
                }   

               
                /* Inform rule of it */
                match css_stylesheet::css__stylesheet_rule_set_nascent_import(&mut css_rule_data_list[temp_rule], url, *media){
                    CSS_OK => {},
                    error => return error 
                }
                

                /* Inform client of need for import */
                match stylesheet_vector[self.sheet].import {
                    Some(import_fn) => 
                        match (*import_fn)(url, media){
                            CSS_OK => {},
                            error => return error
                    },
                    None => {}
                }

                /* Add rule to sheet */
                match css_stylesheet::css__stylesheet_add_rule(stylesheet_vector, css_rule_data_list, self.sheet, lwc_ref, temp_rule, None){
                        CSS_OK => {},
                        error => return error
				 
                }

                curRule = Some(temp_rule);

                /* Rule is now owned by the sheet, 
                 * so no need to destroy it */

                self.state = IMPORT_PERMITTED
            } 
            else {
                return CSS_INVALID
            }
        } 
        else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, atkeyword.idata.get_ref().clone(), NAMESPACE as uint) {
            if self.state as uint <= NAMESPACE_PERMITTED as uint {
                let mut prefix:Option<uint> = None;

                /* any0 = (IDENT ws)? (STRING | URI) ws */

                if *ctx >= vector.len() {
                    return CSS_INVALID
                }	           

                token = &vector[*ctx];
                *ctx += 1; //Iterate
                
                if match token.token_type { CSS_TOKEN_IDENT => true, _ => false} {
                    prefix = Some(token.idata.get_ref().clone());

                    consumeWhitespace(vector, ctx);

                    if *ctx >= vector.len() {
                        return CSS_INVALID
                    }

                    token = &vector[*ctx];
                    *ctx += 1; //Iterate
                }

                if match token.token_type { CSS_TOKEN_STRING | CSS_TOKEN_URI => false, _ => true} {
                    return CSS_INVALID
                }

                consumeWhitespace(vector, ctx);

                match self.addNamespace(lwc_ref, prefix, token.idata.get_ref().clone()){
                    CSS_OK => {},
                    error => return error
		             
                }

                self.state = NAMESPACE_PERMITTED;
            } 
            else {
                return CSS_INVALID;
            }
        } 
        else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, atkeyword.idata.get_ref().clone(), MEDIA as uint) {
            let media :&mut u64 =&mut 0;

            /* any0 = IDENT ws (',' ws IDENT ws)* */

            match self.parseMediaList(lwc_ref, propstrings_ref, vector, ctx, media){
                CSS_OK => {},
                error => return error
		}
                
            let temp_rule = stylesheet_vector[self.sheet].css_stylesheet_rule_create(css_rule_data_list, CSS_RULE_MEDIA);
            
            match css_stylesheet::css__stylesheet_rule_set_media(&mut css_rule_data_list[temp_rule], *media){
                CSS_OK => {},
                error => return error
			 
            }

            
            match css_stylesheet::css__stylesheet_add_rule(stylesheet_vector, css_rule_data_list, self.sheet, lwc_ref, temp_rule, None){
                CSS_OK => {},
                error => return error   
			 
            }
            
            curRule = Some(temp_rule);

            /* Rule is now owned by the sheet, 
             * so no need to destroy it */

            self.state = HAD_RULE;
        }
        else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, atkeyword.idata.get_ref().clone(), FONT_FACE as uint) {
            let temp_rule = stylesheet_vector[self.sheet].css_stylesheet_rule_create(css_rule_data_list, CSS_RULE_FONT_FACE);
            
            consumeWhitespace(vector, ctx);

            match css_stylesheet::css__stylesheet_add_rule(stylesheet_vector, css_rule_data_list, self.sheet, lwc_ref, temp_rule, None){
                CSS_OK => {},
                error => return error   
			 
            }
            
            /* Rule is now owned by the sheet, 
             * so no need to destroy it */

            curRule = Some(temp_rule);

            self.state = HAD_RULE;
        }
        else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, atkeyword.idata.get_ref().clone(), PAGE as uint) {
            
            /* any0 = (':' IDENT)? ws */
            let temp_rule = stylesheet_vector[self.sheet].css_stylesheet_rule_create(css_rule_data_list, CSS_RULE_PAGE);
           
            consumeWhitespace(vector, ctx);

            if *ctx < vector.len() {
                //token = &vector[*ctx]; Value not used later on 
                
                match self.parseSelector(stylesheet_vector, lwc_ref, propstrings_ref, vector, ctx) {
                    (CSS_OK, Some(selector)) => {
                        match css_stylesheet::css__stylesheet_rule_set_page_selector(&mut css_rule_data_list[temp_rule], selector) {
                            CSS_OK => {},
                            x => return x
                        }//End of match rule_set_page_selector
                    },
                    (x, _) => return x              
                } //End of match parseSelector
               
            }

            match css_stylesheet::css__stylesheet_add_rule(stylesheet_vector, css_rule_data_list, self.sheet, lwc_ref, temp_rule, None){
                CSS_OK => {},
                error => return error   
			 
            }

            curRule = Some(temp_rule);

            /* Rule is now owned by the sheet, 
             * so no need to destroy it */

            self.state = HAD_RULE;
        } 
        else {
            return CSS_INVALID;
        }

        let entry:context_entry = context_entry {
                event_type: CSS_PARSER_START_ATRULE, 
                data:curRule };
        
        self.context.push(entry);
        
        return CSS_OK
    }

    pub fn handleEndAtRule(&mut self)->css_error {
        //debug!("Entering: handleEndAtRule");
        let mut cur:context_entry;
        
        if !self.context.is_empty() {
            cur=self.context[self.context.len()-1];
            match cur.event_type {
                CSS_PARSER_START_ATRULE => {
                    self.context.pop();
                    CSS_OK
                },
                _ => CSS_INVALID
		   
            }
        }
        else {
            CSS_INVALID
        }
    }
    

    pub fn handleStartBlock(&mut self)->css_error {
        //debug!("Entering: handleStartBlock");
        let mut cur:context_entry;
        let mut entry:context_entry = context_entry{ event_type:CSS_PARSER_START_BLOCK, data:None };
        

        /* If the current item on the stack isn't a block, 
         * then clone its data field. This ensures that the relevant rule
         * is available when parsing the block contents. */
        if !self.context.is_empty() {
            cur=self.context[self.context.len()-1];
            match cur.event_type {
                CSS_PARSER_START_BLOCK =>   {},
                _ =>    entry.data = cur.data
            }
        }

        self.context.push(entry);
        CSS_OK  
    }

    pub fn handleEndBlock(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type])->css_error {
        //debug!("Entering: handleEndBlock");
        let mut cur:context_entry;
        
        if !self.context.is_empty() {
            cur=self.context[self.context.len()-1];
            match cur.event_type {
                CSS_PARSER_START_BLOCK => {
                    let curRule = cur.data;
                    self.context.pop();
                    /* If the block we just popped off the stack was associated with a 
                    * non-block stack entry, and that entry is not a top-level statement,
                    * then report the end of that entry, too. */
                    if curRule.is_some()  {
                        match css_rule_data_list[curRule.unwrap()].rule_type {
                            CSS_RULE_SELECTOR => {
                                let base_rule = css_rule_data_list[curRule.unwrap()].rule_selector.get_ref().base;
                                match stylesheet_vector[self.sheet].css_rule_list[base_rule].parent_rule {
                                    Some(pRule) => 
                                        match stylesheet_vector[self.sheet].css__stylesheet_get_parent_type(css_rule_data_list, pRule) {
                                            CSS_RULE_PARENT_STYLESHEET  => self.handleEndRuleset(),
                                            _ => CSS_OK
                                        },
                                    None => CSS_OK                      
                                }
                            },
                            _ =>    CSS_OK
                        
                        }
                    } 
                    else {
                        return CSS_OK;
                    }
                },
                _   =>  return CSS_INVALID
		
            } // end of match
        }
        else {
            return CSS_INVALID  
        }       
    }


    pub fn handleBlockContent(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, tokens:&~[~css_token])-> css_error {
        //debug!("Entering: handleBlockContent");
        // * Block content comprises either declarations (if the current block is
        // * associated with @page, @font-face or a selector), or rulesets (if the
        // * current block is associated with @media). 
        let mut cur:context_entry;
        
        if !self.context.is_empty() {
            cur=self.context[self.context.len()-1];
            match cur.data {
                None => CSS_INVALID,
                Some(curRule) => {
                    match css_rule_data_list[curRule].rule_type {
                        CSS_RULE_SELECTOR | CSS_RULE_PAGE  | CSS_RULE_FONT_FACE => {                                   
                            //Expect declarations 
                            return self.handleDeclaration(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens);
                        },
                        CSS_RULE_MEDIA => {
                            // Expect rulesets 
                            return self.handleStartRuleset(stylesheet_vector, css_rule_data_list, lwc_ref, propstrings_ref, tokens);
                        },  
                        _ => return CSS_INVALID
			    
                    }
                }
            } // end of match
        }
        else {
		return CSS_INVALID  
	     }
        
    }

    pub fn handleDeclaration(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, tokens:&~[~css_token])->css_error {
        //debug!("Entering: handleDeclaration");
        let ctx: &mut uint = &mut 0u;   
         // Locations where declarations are permitted:
         // *
         // * + In @page
         // * + In @font-face
         // * + In ruleset
         
        let mut cur:context_entry;
        
        if !self.context.is_empty() {
            cur=self.context[self.context.len()-1];
            // //debug!(fmt!("Entering: cur.data == %? , cur.event_type == %?" , cur.data.unwrap() , cur.event_type));
            match cur.data {
                None => CSS_INVALID,
                Some(curRule) => {
                    match css_rule_data_list[curRule].rule_type {
                        CSS_RULE_SELECTOR | CSS_RULE_PAGE  | CSS_RULE_FONT_FACE => {                                   
                            // Strip any leading whitespace (can happen if in nested block) 
                            consumeWhitespace(tokens, ctx);

                            // IDENT ws ':' ws value 
                            // * 
                            // * In CSS 2.1, value is any1, so '{' or ATKEYWORD => parse error
                             
                            
                            if tokens.len() > *ctx {   
                                let ident =&tokens[*ctx];
                                *ctx = *ctx + 1;
                                match ident.token_type { 
                                    CSS_TOKEN_IDENT => {
                                        consumeWhitespace(tokens, ctx);
                                        if tokens.len() <= *ctx || !tokenIsChar(&tokens[*ctx], lwc_ref, ':') {
                                            return CSS_INVALID
                                        }
                                        else {
                                            *ctx += 1;
                                            consumeWhitespace(tokens, ctx);
                                            match css_rule_data_list[curRule].rule_type {
                                                CSS_RULE_FONT_FACE =>  
							{
								let css_er:css_error = css__parse_font_descriptor(stylesheet_vector, self.sheet, lwc_ref,  ident, propstrings_ref, tokens, ctx, css_rule_data_list[curRule].rule_font_face.get_mut_ref());
		        	                                return css_er;
								 
							},
                                                _ =>   {
							let css_er:css_error = self.parseProperty(stylesheet_vector, lwc_ref, propstrings_ref, ident, tokens, ctx, &mut css_rule_data_list[curRule]) ;  
		                                        return css_er;
						       }
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

    pub fn parseSelectorList(&mut self, stylesheet_vector:&mut ~[css_stylesheet], css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, tokens:&~[~css_token], curRule: css_rule_data_index) -> css_error {
        //debug!("Entering: parseSelectorList");
        let ctx: &mut uint = &mut 0u;
        //debug!(fmt!("parseSelectorList:: tokens == %?", tokens));
        loop {
            /* Strip any leading whitespace (can happen if in nested block) */
            consumeWhitespace(tokens, ctx);

            /* selector_list   -> selector [ ',' ws selector ]* */
            match self.parseSelector(stylesheet_vector, lwc_ref, propstrings_ref, tokens, ctx) {
                (CSS_OK, Some(selector)) => {
                    match stylesheet_vector[self.sheet].css__stylesheet_rule_add_selector(css_rule_data_list, curRule, selector) {
                        CSS_OK => {
                            if *ctx < tokens.len() {
                                //Iterate over vector to check for invalid character
                                if !tokenIsChar(&tokens[*ctx], lwc_ref, ',') {
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
     * At-rule parsing functions                              *
     ******************************************************************************/

    pub fn parseMediaList(&mut self, lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, media:&mut u64) -> css_error
    {
        //debug!("Entering: parseMediaList");
        let mut ret:u64 = 0;

        if *ctx < vector.len() {
                
			let mut token = &vector[*ctx];
			*ctx += 1; //Iterate
					
			loop {
				if match token.token_type { CSS_TOKEN_IDENT => false, _ => true} {
					return CSS_INVALID
				}

				if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), AURAL as uint) {
					ret |= CSS_MEDIA_AURAL as u64;
				} 
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), BRAILLE as uint) {
					ret |= CSS_MEDIA_BRAILLE as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), EMBOSSED as uint) {
					ret |= CSS_MEDIA_EMBOSSED as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), HANDHELD as uint) {
					ret |= CSS_MEDIA_HANDHELD as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), PRINT as uint) {
					ret |= CSS_MEDIA_PRINT as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), PROJECTION as uint) {
					ret |= CSS_MEDIA_PROJECTION as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), SCREEN as uint) {
				   ret |= CSS_MEDIA_SCREEN as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), SPEECH as uint) {
					ret |= CSS_MEDIA_SPEECH as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), TTY as uint) {
					ret |= CSS_MEDIA_TTY as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), TV as uint) {
					ret |= CSS_MEDIA_TV as u64;
				}
				else if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), ALL as uint) {
					ret |= CSS_MEDIA_ALL as u64;
				}
				else {
					return CSS_INVALID;   
				}
					
				consumeWhitespace(vector, ctx);

				if *ctx >= vector.len() {
				   break;
				}

				token = &vector[*ctx];
				*ctx += 1; //Iterate
				

				if !tokenIsChar(token, lwc_ref,  ',') {
					return CSS_INVALID;
				}
					

				consumeWhitespace(vector, ctx);
			}
		}
		
        /* If, after parsing the media list, we still have no media, 
         * then it must be ALL. */
        if ret == 0 {
            ret = CSS_MEDIA_ALL as u64;
        }
            

        *media = ret;

        return CSS_OK;
    }

    /**
    * #Arguments:
    *  'c'  - Parsing context to add to. 
    *  'prefix' - Namespace prefix, or NULL for default namespace.
    *  'uri'    - Namespace URI.
    * #Return Value:
    * 'css_error' - CSS_OK on success,  
                    CSS_INVALID if the input is not valid.
    */
    pub fn addNamespace(&mut self, lwc_ref:&mut ~lwc, _prefix:Option<uint>, uri:uint) -> css_error {
        //debug!("Entering: addNamespace");
        match _prefix {
            Some(prefix) => {
                /* Replace, or add mapping */
                let mut prefix_match = false;
                let mut idx = 0;

                for ns in self.namespaces.iter() {
                    
                    if lwc_ref.lwc_string_isequal(ns.prefix.get_ref().clone(), prefix) {
                        prefix_match = true;
                    }
                    if prefix_match {
                        break
                    }
                    idx += 1;    
                }

                if (idx == self.namespaces.len()) {
                    /* Not found, create a new mapping */
                    let ns = ~css_namespace{prefix:Some(prefix),uri:None};

                    self.namespaces.push(ns)
                   
                }
                    
                /* Special case: if new namespace uri is "", use NULL */
                if (lwc_ref.lwc_string_length(uri) == 0) {
                    self.namespaces[idx].uri = None
                }    
                else {
                    self.namespaces[idx].uri = Some(uri)
                }    
            },
            None => {

                /* Special case: if new namespace uri is "", use NULL */
                if (lwc_ref.lwc_string_length(uri) == 0){
                    self.default_namespace = None
                }
                else {
                    self.default_namespace = Some(uri)
                }
            } 
        }    
        
        return CSS_OK
    }

    /******************************************************************************
     * Property parsing functions                             *
     ******************************************************************************/

    pub fn parseProperty(&mut self , stylesheet_vector:&mut ~[css_stylesheet], lwc_ref:&mut ~lwc, 
                            propstrings_ref:& css_propstrings, property: &~css_token ,
                            vector: &~[~css_token], ctx:&mut uint, curRule: &mut ~css_rule_data_type) -> css_error {
        //debug!("Entering: parseProperty");
        let mut style: ~css_style;
        let mut index = AZIMUTH as uint;

        while (index <= Z_INDEX as uint) {
            if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, property.idata.get_ref().clone() , index) {
                break
            }
            index +=1;
        }

        if index == Z_INDEX as uint + 1 {
            return CSS_INVALID;
        }

        style = css_stylesheet::css__stylesheet_style_create(self.sheet) ;
        //debug!(fmt!("parseProperty:: style.bytecode (1) == %?" , style.bytecode));
        let error = (*self.properties.property_handlers[index - AZIMUTH as uint])(stylesheet_vector, self.sheet , lwc_ref, propstrings_ref , vector , ctx , &mut style);

        //debug!(fmt!("parseProperty:: style.bytecode (2)== %?" , style.bytecode));

        if error as int != CSS_OK as int {
            return error;
        }
        
        let (status,flags) = self.css__parse_important(lwc_ref, propstrings_ref, vector , ctx);
        if status as int != CSS_OK as int {
            //debug!("Exiting: parseProperty (1)");
            return status;
        }
        consumeWhitespace(vector , ctx);

        if *ctx < vector.len() {
            return CSS_INVALID;
        }

        *ctx += 1;

        if (flags != 0) {
            self.css__make_style_important(&mut style);
        }

        //debug!("Exiting: parseProperty (2)");
        /* Append style to rule */
        match stylesheet_vector[self.sheet].css__stylesheet_rule_append_style(curRule, style) {
            CSS_OK => {
                //debug!("Exiting: parseProperty (3)");
                CSS_OK
            },
            x => {
                //debug!("Exiting: parseProperty (4)");
                x
            }
        }
    }

    pub fn parseSelector(&mut self, stylesheet_vector:&mut ~[css_stylesheet], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint) -> (css_error, Option<uint>) {
        //debug!("Entering: parseSelector");
        /* selector -> simple_selector [ combinator simple_selector ]* ws
         * 
         * Note, however, that, as combinator can be wholly whitespace,
         * there's an ambiguity as to whether "ws" has been reached. We 
         * resolve this by attempting to extract a combinator, then 
         * recovering when we detect that we've reached the end of the
         * selector.
         */

        match self.parseSimpleSelector(stylesheet_vector, lwc_ref, propstrings_ref, vector, ctx) {
            (CSS_OK, Some(selector_)) => {
                let mut result = selector_;
                let mut _selector = selector_;
                loop {
                    if *ctx >= vector.len() || tokenIsChar(&vector[*ctx], lwc_ref, ',') {
                        return (CSS_OK, Some(result))
                    }
                    else {
                        let mut comb = CSS_COMBINATOR_NONE;        
                        match self.parseCombinator(lwc_ref, vector, ctx, &mut comb) {
                            CSS_OK => {
                                /* In the of "html , body { ... }", the whitespace after
                                 * "html" and "body" will be considered an ancestor combinator.
                                 * This clearly is not the case, however. Therefore, as a 
                                 * special case, if we've got an ancestor combinator and there 
                                 * are no further tokens, or if the next token is a comma,
                                 * we ignore the supposed combinator and continue. */
                                if (comb as uint == CSS_COMBINATOR_ANCESTOR as uint &&
										(*ctx >= vector.len() || tokenIsChar(&vector[*ctx],lwc_ref, ',') )) {
                                    loop;
                                }
                                
                                match self.parseSimpleSelector(stylesheet_vector, lwc_ref, propstrings_ref, vector, ctx) {
                                    (CSS_OK, Some(other_selector)) => {   
										result = other_selector;
                                        match stylesheet_vector[self.sheet].css__stylesheet_selector_combine(comb, _selector, other_selector) {
                                            CSS_OK => { _selector = other_selector}
                                            x => return (x,None)
                                        }
                                    },
                                    (x,y) => return(x,y)
                                } // End of match parseSimpleSelector
                            },  
                            x => return (x, Some(_selector))
                        }// End of outer match parseCombinator
                    } // End of If Else
                } //End of loop
            },
            (x,y) => return (x,y)
        } // End of outer match parseSimpleSelector
    }

    pub fn parseSimpleSelector(&mut self, stylesheet_vector:&mut ~[css_stylesheet], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint) -> (css_error, Option<uint>) {
        //debug!("Entering: parseSimpleSelector");
        let reason = "Funtion parseSimpleSelector";
        let orig_ctx = *ctx;
        /* simple_selector  -> type_selector specifics
         *          -> specific specifics
         */
        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }        
        
        let token = &vector[*ctx];

        let mut selector : uint;
        let mut qname: ~css_qname = ~css_qname{ name:lwc_ref.lwc_intern_string(&""), ns:lwc_ref.lwc_intern_string(&"") };

        //match ( vector[*ctx].token_type as uint ==    CSS_TOKEN_IDENT as uint )
        if token.token_type as int == CSS_TOKEN_IDENT as int ||  tokenIsChar(token, lwc_ref,  '*') || tokenIsChar(token, lwc_ref,  '|') {
            
            /* Have type selector */
            match self.parseTypeSelector(lwc_ref, propstrings_ref, vector, ctx, &mut qname) {
                CSS_OK => {
                    selector = stylesheet_vector[self.sheet].css__stylesheet_selector_create(lwc_ref, qname);
                },
                x => {
                    *ctx = orig_ctx;
                    return (x, None)
                }                       
            } 
        }   
        else {
            /* Universal selector */
            if self.default_namespace.is_some() {
                qname.ns = self.default_namespace.expect(reason);
            }
            else {
                qname.ns = propstrings_ref.get_lwc_string(UNIVERSAL as uint)
            }
            
            qname.name = propstrings_ref.get_lwc_string(UNIVERSAL as uint);

            selector =  stylesheet_vector[self.sheet].css__stylesheet_selector_create(lwc_ref, qname);
            /* Ensure we have at least one specific selector */
            match self.parseAppendSpecific(stylesheet_vector, lwc_ref, propstrings_ref, vector, ctx, selector) {
                CSS_OK => {},
                error  => return (error,None)
            }           
        }   
        
        
        match self.parseSelectorSpecifics(stylesheet_vector, lwc_ref, propstrings_ref, vector, ctx, selector) {
            CSS_OK => return (CSS_OK, Some(selector)),
            error => return (error, None)
        }   
        
    }

    pub fn parseCombinator(&mut self, lwc_ref:&mut ~lwc, vector:&~[~css_token], ctx:&mut uint, comb:&mut css_combinator) -> css_error {
        //debug!("Entering: parseCombinator");
        let mut token: &~css_token;
        /* combinator      -> ws '+' ws | ws '>' ws | ws '~' ws | ws1 */
        *comb = CSS_COMBINATOR_NONE;

        loop {
            if *ctx >= vector.len() {
                break
            } 

            token = &vector[*ctx];
            //debug!(fmt!("parseCombinator :: token == %?", token));
			if tokenIsChar(token, lwc_ref,  '+') {
                *comb = CSS_COMBINATOR_SIBLING
            }   
            else if tokenIsChar(token, lwc_ref,   '>') {
                *comb = CSS_COMBINATOR_PARENT   
            }
            else if tokenIsChar(token, lwc_ref,  '~') {
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

    pub fn parseTypeSelector(&mut self, lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, qname:&mut ~css_qname) -> css_error {
        //debug!("Entering: parseTypeSelector");
        let mut token: &~css_token;
        let mut prefix:Option<uint> =None;
        let reason = "Function parseTypeSelector";

        /* type_selector    -> namespace_prefix? element_name
         * namespace_prefix -> [ IDENT | '*' ]? '|'
         * element_name     -> IDENT | '*'
         */
         if *ctx >= vector.len() {
            return CSS_INVALID
        } 
        
        token = &vector[*ctx];
        let mut token_null = false;
		
        if !tokenIsChar(token, lwc_ref,  '|') {
            prefix = Some(token.idata.get_ref().clone());
            		
			*ctx += 1; //Iterate
            
			if (*ctx < vector.len()){
				token = &vector[*ctx]; // peek
			}
			else {
				token_null = true;
			}
        }

        if ( !token_null && tokenIsChar(token, lwc_ref,  '|')) {
            
            /* Have namespace prefix */
            *ctx += 1; //Iterate

            /* Expect element_name */
            if *ctx >= vector.len() || ( match vector[*ctx].token_type { CSS_TOKEN_IDENT => false, _ => true} && !tokenIsChar(&vector[*ctx], lwc_ref,'*') ) {
                return CSS_INVALID
            }

            token = &vector[*ctx]; 
            *ctx += 1; //Iterate

            match self.lookupNamespace(lwc_ref, prefix, qname) {
                CSS_OK  => qname.name = token.idata.get_ref().clone(),
                error   => return error
            }   
        } 
        else {
            /* No namespace prefix */
            if self.default_namespace.is_some() {
                qname.ns = self.default_namespace.expect(reason);
            }
            else {
                qname.ns = propstrings_ref.get_lwc_string(UNIVERSAL as uint)
            }

			//debug!(fmt!("prefix=%?",prefix));
            qname.name = match prefix {
                Some(x) => x,
                None => lwc_ref.lwc_intern_string(&"") 
            };
			//debug!(fmt!("qname=%?",qname));
        }
        
        return CSS_OK
    }   

    pub fn parseSelectorSpecifics(&mut self,  stylesheet_vector:&mut ~[css_stylesheet], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, parent:uint ) -> css_error {
        //debug!("Entering: parseSelectorSpecifics");
        let mut token: &~css_token;

        /* specifics -> specific* */
        loop {
            if *ctx >= vector.len() {
                break;
            }   
            else {
                token = &vector[*ctx];
                if (match token.token_type { CSS_TOKEN_S => false, _ => true }) && 
                    !tokenIsChar(token, lwc_ref,  '+')  &&
                    !tokenIsChar(token, lwc_ref,  '>')  &&
                    !tokenIsChar(token, lwc_ref,  '~')  &&
                    !tokenIsChar(token, lwc_ref,  ',') {
                    match self.parseAppendSpecific(stylesheet_vector, lwc_ref, propstrings_ref, vector,ctx,parent) {
                        CSS_OK  => loop,
                        error   =>  return error
                    }
                }
                else {
                    break;
                }
            }   
        }
        CSS_OK      
    }  


    pub fn parseAppendSpecific(&mut self, stylesheet_vector:&mut ~[css_stylesheet], lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, parent:uint ) -> css_error{
        //debug!("Entering: parseAppendSpecific");
        match self.parseSpecific(lwc_ref, propstrings_ref, vector, ctx, false) {
            (CSS_OK,Some(specific)) => return stylesheet_vector[self.sheet].css__stylesheet_selector_append_specific(parent,specific),
            (error,_) => return error
        }       
    }   


    pub fn parseSpecific(&mut self, lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, in_not:bool) -> (css_error,Option<~css_selector_detail>) {
        //debug!("Entering: parseSpecific");
        /* specific  -> [ HASH | class | attrib | pseudo ] */

        let mut token: &~css_token;
        
        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }   
        
        token = &vector[*ctx];

        match token.token_type {
            CSS_TOKEN_HASH   => {
                let qname=~css_qname{ns: lwc_ref.lwc_intern_string(&""), name:token.idata.get_ref().clone()};
                match css_stylesheet::css__stylesheet_selector_detail_init (CSS_SELECTOR_ID, qname, 
                                            CSS_SELECTOR_DETAIL_VALUE_STRING,None, None, false) {
                    (CSS_OK, res) => {
                        //debug!(fmt!("parseSpecific:: css__stylesheet_selector_detail_init result == %?", res));
                        *ctx +=1;
                        (CSS_OK, res) 
                    } 
                    (error, y) =>  (error,y)    
                }
    
            } 
            _   => {
                if (tokenIsChar(token, lwc_ref,  '.')) {
                    self.parseClass(lwc_ref, vector, ctx)
                } 
                else if (tokenIsChar(token, lwc_ref,  '[')) {
                    self.parseAttrib(lwc_ref, vector, ctx)
                }
                else if (tokenIsChar(token, lwc_ref,  ':')) {
                    self.parsePseudo(lwc_ref, propstrings_ref, vector, ctx, in_not)
                } 
                else {
                    (CSS_INVALID,None)
                }
            }
        }       
    }

    /**
    * #Arguments:
    *  'c'  - Parsing context to add to. 
    *  'prefix' - Namespace prefix, or NULL for default namespace.
    *  'qname'    - 
    * #Return Value:
    * 'css_error' - CSS_OK on success,  
                    CSS_INVALID if the input is not valid.
    */
    pub fn lookupNamespace(&mut self, lwc_ref:&mut ~lwc, prefix:Option<uint>, qname:&mut ~css_qname) -> css_error {
        //debug!("Entering: lookupNamespace");
        let mut idx:uint=0;
        let reason = "Function lookupNamespace";
        match prefix {
            None => {
                //debug!("Entering: lookupNamespace (1)");
                qname.ns = lwc_ref.lwc_intern_string(&"")
            },
            Some(value) => {
                //debug!("Entering: lookupNamespace (2)");
                for ns in self.namespaces.iter() {
                    match ns.prefix {
                        Some(_) => {
                            //debug!("Entering: lookupNamespace (3)");
                            let ns_prefix = ns.prefix.expect(reason);
                            if ( lwc_ref.lwc_string_isequal(ns_prefix , value)) {
                                break;
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
                    Some(_)=> {
                        //debug!("Entering: lookupNamespace (4)");
                        qname.ns = self.namespaces[idx].uri.unwrap()
                    },
                    None => {
                        //debug!("Entering: lookupNamespace (5)");
                        qname.ns = lwc_ref.lwc_intern_string(&"")
                    }
                }
            }
        }   
        CSS_OK
    }

    /******************************************************************************
    * Selector list parsing functions                         *
    ******************************************************************************/
    pub fn  parseClass(&mut self, lwc_ref:&mut ~lwc, vector:&~[~css_token], ctx:&mut uint) -> (css_error,Option<~css_selector_detail>) {
        //debug!("Entering: parseClass");
        let mut token: &~css_token;
        
        /* class     -> '.' IDENT */
        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }   
        
        token = &vector[*ctx];
        *ctx +=1; //Iterate             
        
        if !tokenIsChar(token, lwc_ref,  '.') {
            return (CSS_INVALID,None)
        }   

        token = &vector[*ctx];
        *ctx +=1; //Iterate 

        match token.token_type {
            CSS_TOKEN_IDENT => {
                let qname=~css_qname{ns:lwc_ref.lwc_intern_string(&""), name:token.idata.get_ref().clone()};
                return css_stylesheet::css__stylesheet_selector_detail_init (CSS_SELECTOR_CLASS, qname, 
                                                    CSS_SELECTOR_DETAIL_VALUE_STRING,None, None, false)
            }
            _ => return (CSS_INVALID,None)
        }
    }

    pub fn  parseAttrib(&mut self, lwc_ref:&mut ~lwc, vector:&~[~css_token], ctx:&mut uint) -> (css_error,Option<~css_selector_detail>) {
        //debug!("Entering: parseAttrib");
        let mut token: &~css_token;
        
        /* attrib    -> '[' ws namespace_prefix? IDENT ws [
         *             [ '=' | 
         *               INCLUDES | 
         *               DASHMATCH | 
         *               PREFIXMATCH |
         *               SUFFIXMATCH | 
         *               SUBSTRINGMATCH 
         *             ] ws
         *             [ IDENT | STRING ] ws ]? ']'
         * namespace_prefix -> [ IDENT | '*' ]? '|'
         */
        
        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }   
        
        token = &vector[*ctx];
        *ctx +=1; //Iterate             
        
        if !tokenIsChar(token, lwc_ref,  '[') {
            return (CSS_INVALID,None)
        }   

        consumeWhitespace(vector, ctx);

        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }

        token = &vector[*ctx];
        *ctx +=1; //Iterate             

        if (match token.token_type { CSS_TOKEN_IDENT => false, _ => true}) && !tokenIsChar(token, lwc_ref,  '*') &&
                !tokenIsChar(token, lwc_ref,  '|') {
            return (CSS_INVALID, None)
        }   
        
        let mut prefix:Option<uint> = None;

        if tokenIsChar(token, lwc_ref,  '|') {
            if *ctx >= vector.len() {
                return (CSS_INVALID, None)
            }

            token = &vector[*ctx];
            *ctx +=1; //Iterate
        } 
        else if (*ctx < vector.len() && tokenIsChar(&vector[*ctx], lwc_ref,'|')) {
            prefix = Some(token.idata.get_ref().clone());
            *ctx += 1;
            if *ctx >= vector.len() {
                return (CSS_INVALID, None)
            }

            token = &vector[*ctx];
            *ctx +=1; //Iterate
        }

        if match token.token_type { CSS_TOKEN_IDENT => false, _ => true} {
            return (CSS_INVALID, None)
        }   

        let mut qname:~css_qname=~css_qname{ns:lwc_ref.lwc_intern_string(&""), name:lwc_ref.lwc_intern_string(&"") };
        match self.lookupNamespace(lwc_ref, prefix, &mut qname) { CSS_OK  => {}, error   => return (error,None)}   

        qname.name = token.idata.get_ref().clone();
		//debug!(fmt!("Qname=%?",copy qname.name));
		
        consumeWhitespace(vector, ctx);

        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }

        token = &vector[*ctx];
        *ctx +=1; //Iterate

        let mut tkn_type = CSS_SELECTOR_ATTRIBUTE;
        let mut value:Option<&~css_token> = None;

        if !tokenIsChar(token, lwc_ref,  ']') {
            if tokenIsChar(token, lwc_ref,  '=') {
                tkn_type = CSS_SELECTOR_ATTRIBUTE_EQUAL;
            }
            else {
                match token.token_type {
                    CSS_TOKEN_INCLUDES       => tkn_type = CSS_SELECTOR_ATTRIBUTE_INCLUDES, 
                    CSS_TOKEN_DASHMATCH      => tkn_type = CSS_SELECTOR_ATTRIBUTE_DASHMATCH,
                    CSS_TOKEN_PREFIXMATCH    => tkn_type = CSS_SELECTOR_ATTRIBUTE_PREFIX,
                    CSS_TOKEN_SUFFIXMATCH    => tkn_type = CSS_SELECTOR_ATTRIBUTE_SUFFIX,
                    CSS_TOKEN_SUBSTRINGMATCH => tkn_type = CSS_SELECTOR_ATTRIBUTE_SUBSTRING,
                    _                        => return (CSS_INVALID,None)
                }
            }
            consumeWhitespace(vector, ctx);

            if *ctx >= vector.len() {
                return (CSS_INVALID, None)
            }

            token = &vector[*ctx];
            *ctx +=1; //Iterate
            
            match token.token_type{ CSS_TOKEN_IDENT => {}, CSS_TOKEN_STRING => {}, _ => return (CSS_INVALID,None) }

            value = Some(token);

            consumeWhitespace(vector, ctx);

            if *ctx >= vector.len() {
                return (CSS_INVALID, None)
            }

            token = &vector[*ctx];
            *ctx +=1; //Iterate
            
            if !tokenIsChar(token, lwc_ref,  ']') {
                return (CSS_INVALID, None)
            }   
        }
        
         
        return css_stylesheet::css__stylesheet_selector_detail_init (tkn_type,qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
                            match value {Some(tkn)=>Some(tkn.idata.unwrap()), None => None }, None, false)
    }


    pub fn parsePseudo(&mut self, lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, in_not:bool) -> (css_error,Option<~css_selector_detail>) {
        //debug!("Entering: parsePseudo");
        let mut token: &~css_token;
        //let mut tkn_type = CSS_SELECTOR_PSEUDO_CLASS;
        let mut value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;
        let mut require_element:bool = false;
        let mut negate:bool = false;
        let mut lut_idx:uint;
        let mut selector_type:css_selector_type;
        let mut qname:~css_qname=~css_qname{ns:lwc_ref.lwc_intern_string(&""), name:lwc_ref.lwc_intern_string(&"")};
        /* pseudo    -> ':' ':'? [ IDENT | FUNCTION ws any1 ws ')' ] */

        let mut detail_value_string:Option<uint> = None;

        if *ctx >= vector.len() {
                return (CSS_INVALID, None)
            }

        token = &vector[*ctx];
        *ctx +=1; //Iterate
        
        if  !tokenIsChar(token, lwc_ref,  ':') {
            return (CSS_INVALID,None)
        }

        /* Optional second colon before pseudo element names */
        if *ctx >= vector.len() {
            return (CSS_INVALID, None)
        }

        token = &vector[*ctx];
        *ctx +=1; //Iterate

        if tokenIsChar(token, lwc_ref,  ':') {
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
            CSS_TOKEN_IDENT => {},
            CSS_TOKEN_FUNCTION  => {},
            _ => return (CSS_INVALID, None)
        } 
            
        qname.name=token.idata.get_ref().clone();
        
        /* Search lut for selector type */
        match propstrings_ref.is_selector_pseudo(lwc_ref, qname.name) {
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
        if in_not && (match selector_type {CSS_SELECTOR_PSEUDO_ELEMENT => true, _ => false} || lut_idx == NOT as uint) {
            return (CSS_INVALID, None)  
        }   
		
		
        if match token.token_type { CSS_TOKEN_FUNCTION => true, _ => false} {
            
            let mut fun_type:index_property;
            unsafe {
                fun_type = transmute(lut_idx);
                forget(fun_type);
            }

            consumeWhitespace(vector, ctx);
			
            match fun_type {
                LANG  => {
                    /* IDENT */
                    if *ctx >= vector.len() {
                        return (CSS_INVALID, None)
                    }

                    token = &vector[*ctx];
                    *ctx +=1; //Iterate
                    
                    match token.token_type {
                        CSS_TOKEN_IDENT => {},
                        _ => return (CSS_INVALID, None)
                     } 
                        
                    detail_value_string = Some(token.idata.get_ref().clone());
                    value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;

                    consumeWhitespace(vector, ctx);
                }, 
                NTH_CHILD | NTH_LAST_CHILD  | NTH_OF_TYPE  | NTH_LAST_OF_TYPE  => {
                    /* an + b */
                    let mut specific:~css_selector_detail;
                    match self.parseNth(lwc_ref, propstrings_ref, vector, ctx, qname) {
                        (CSS_OK, Some(specific_ret)) => {
                            specific = specific_ret;
                            specific.selector_type = selector_type;
                            // Iterate to the next location
                            if *ctx >= vector.len() {
                                //debug!("Exiting: parsePseudo (Nth 1)");
                                return (CSS_INVALID, None)
                            }
                                        
                            token = &vector[*ctx];
                            *ctx += 1 ; 
                            //debug!(fmt!("parsePseudo (Nth) :: token == %?", token));
                            if !tokenIsChar(token, lwc_ref,  ')') {
                                //debug!("Exiting: parsePseudo (Nth 2)");
                                return (CSS_INVALID, None)
                            }

                            //debug!("Exiting: parsePseudo (Nth 3)");
                            return (CSS_OK,Some(specific))
                        },  
                        (error,_) => { 
                            //debug!("Exiting: parsePseudo (Nth 4)");
                            return (error,None); 
                        }
                    }
                    
                },  
                NOT => {
                    // type_selector | specific */
                    
                    if *ctx >= vector.len() {
                        return (CSS_INVALID, None)
                    }
                        
                    token = &vector[*ctx];
                        
                    if (match token.token_type {    CSS_TOKEN_IDENT => true, _  => false }) || 
                            tokenIsChar(token, lwc_ref,  '*') || tokenIsChar(token, lwc_ref,  '|') {
                        /* Have type selector */
                        match self.parseTypeSelector(lwc_ref, propstrings_ref, vector, ctx, &mut qname) {
                            CSS_OK => {
                                selector_type = CSS_SELECTOR_ELEMENT;

                                detail_value_string = None;
                                value_type = CSS_SELECTOR_DETAIL_VALUE_STRING;
                            },
                            x => return (x, None)
                        }                       
                    } 
                    else {
                        /* specific */
                        let mut specific:~css_selector_detail;
                        match self.parseSpecific(lwc_ref, propstrings_ref, vector, ctx, true) {
                            (CSS_OK,Some(specific_ret)) => {
                                specific = specific_ret;
                                specific.negate = true;
                                consumeWhitespace(vector, ctx);
                                if *ctx >= vector.len() {
                                    return (CSS_INVALID, None)
                                }
                                            
                                token = &vector[*ctx];
                                *ctx += 1 ;
                                
                                if !tokenIsChar(token, lwc_ref,  ')') {
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
            
            if !tokenIsChar(token, lwc_ref,  ')') {
                return (CSS_INVALID, None)
            } 
        
        }

        return css_stylesheet::css__stylesheet_selector_detail_init(selector_type, qname, value_type, detail_value_string, None, negate);
    }

    pub fn parseNth(&mut self, lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint, qname:~css_qname) -> (css_error,Option<~css_selector_detail>) {
        //debug!("Entering: parseNth");
        let mut token: &~css_token;

        let mut value: ~css_selector_detail = ~css_selector_detail{
            qname:qname,
            selector_type:CSS_SELECTOR_PSEUDO_CLASS,
            combinator_type:CSS_COMBINATOR_NONE,  
            value_type:CSS_SELECTOR_DETAIL_VALUE_NTH,
            negate:false,
    
            string:None,
            a:0,
            b:0
        }; 
        /* nth -> [ DIMENSION | IDENT ] ws [ [ CHAR ws ]? NUMBER ws ]?
         *        (e.g. DIMENSION: 2n-1, 2n- 1, 2n -1, 2n - 1)
         *        (e.g. IDENT: -n-1, -n- 1, -n -1, -n - 1)
         *     -> NUMBER ws
         *     -> IDENT(odd) ws
         *     -> IDENT(even) ws
         */

        // Vector Iterate
        if *ctx >= vector.len() {
            //debug!("Exiting: parseNth (1)");
            return (CSS_INVALID, None)
        }
            
        token = &vector[*ctx];
        *ctx += 1;
                        
        match token.token_type { 
            CSS_TOKEN_IDENT | CSS_TOKEN_DIMENSION => {
                if (match token.token_type { CSS_TOKEN_IDENT => true, _ => false}) &&
                        propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), ODD as uint) {
                    /* Odd */
                    value.a = 2;
                    value.b = 1;
                }
                else if (match token.token_type { CSS_TOKEN_IDENT => true, _ => false}) &&
                            propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), EVEN as uint)
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
                                        
                    let mut a:i32;
                    let mut b:i32 = 0;
                    let mut sign:i32 = 1;
                    let mut had_sign = false;
                    let mut had_b = false;

                    let mut len = lwc_ref.lwc_string_length(token.idata.get_ref().clone());
                    let mut data = lwc_ref.lwc_string_data(token.idata.get_ref().clone());
                    let mut data_index = 0;
                    /* Compute a */
                    if (match token.token_type {
                        CSS_TOKEN_IDENT => true, 
                        _ => false   
                    }) {
                        if len < 2 {
                            if (data[data_index + 0] != 'n' as u8) && (data[data_index + 0] != 'N' as u8)   {
                                //debug!("Exiting: parseNth (2)");
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
                                //debug!("Exiting: parseNth (3)");
                                return (CSS_INVALID, None)  
                            }
                                
                            /* -n */
                            a = -1 << 10;

                            data_index += 2;
                            len -= 2;
                        }

                        if len > 0 {
                            //debug!("Entering: parseNth:: len > 0");
                            //debug!(fmt!("parseNth:: len == %?", len));
                            //debug!(fmt!("parseNth:: data == %?", data));

                            if (data[data_index + 0] != '-' as u8)
                            {
                                //debug!("Exiting: parseNth (4)");
                                return (CSS_INVALID, None)
                            }
                                

                            /* -n- */
                            //debug!("parseNth:: -n-");
                            sign = -1;
                            had_sign = true;

                            if len > 1 {
                                /* Reject additional sign */
                                if (data[data_index + 1] == '-' as u8) || (data[data_index + 1] == '+' as u8)
                                {
                                    //debug!("Exiting: parseNth (5)");
                                    return (CSS_INVALID, None)
                                }
                                    

                                /* -n-b */
                                //debug!("parseNth:: -n-b");
                                let (ret_b,consumed) = css__number_from_string( data, &mut (data_index + 1), true);
                                b = ret_b;
                                if consumed != len - 1
                                {   
                                    //debug!("Exiting: parseNth (6)");
                                    return (CSS_INVALID, None)
                                }

                                had_b = true;
                            }
                        }
                    } 
                    else {
                        /* 2n */
                        
                        let (ret_a, consumed_) = css__number_from_lwc_string(lwc_ref,token.idata.get_ref().clone(), true);
                        let mut consumed = consumed_;
                        a = ret_a;
                        if consumed == 0 || ((data[data_index + consumed] != 'n' as u8) && (data[data_index + consumed] != 'N' as u8)) {
                            //debug!("Exiting: parseNth (7)");
                            return (CSS_INVALID, None)
                        }

                        consumed += 1;
                        if len - consumed > 0 {
                            if (data[data_index + consumed] != '-' as u8) {
                                //debug!("Exiting: parseNth (8)");
                                return (CSS_INVALID, None)
                            }

                            /* 2n- */
                            sign = -1;
                            had_sign = true;

                            consumed += 1;
                            if len - consumed > 0 {
                                let bstart:uint;

                                /* Reject additional sign */
                                if (data[data_index + consumed] == '-' as u8) ||    (data[data_index + consumed] == '+' as u8) {
                                    //debug!("Exiting: parseNth (9)");
                                    return (CSS_INVALID, None)
                                }

                                /* 2n-b */
                                bstart = consumed;

                                let (ret_b,consumed) = css__number_from_string( data, &mut (data_index + bstart), true);
                                b= ret_b;
                                if consumed != len - bstart {
                                    //debug!("Exiting: parseNth (10)");
                                    return (CSS_INVALID, None)
                                }

                                had_b = true;
                            }
                        }
                    }

                    if had_b == false {
                        //debug!("Entering: parseNth:: if had_b == false");
                        consumeWhitespace(vector, ctx);

                        /* Look for optional b : [ [ CHAR ws ]? NUMBER ws ]? */
                        if *ctx < vector.len() {
                            token = &vector[*ctx];  
                        }
                        
                        if (had_sign == false &&  *ctx < vector.len() &&
                             (tokenIsChar(token, lwc_ref,  '-') || tokenIsChar(token, lwc_ref,  '+'))) {
                            
                            *ctx += 1; //iterate

                            had_sign = true;

                            if tokenIsChar(token, lwc_ref,  '-'){
                                sign = -1
                            }   

                            consumeWhitespace(vector, ctx);

                            if *ctx < vector.len() {
                                token = &vector[*ctx];
                            }
                            
                        }

                        /* Expect NUMBER */
                        if *ctx < vector.len() && (match token.token_type 
                            { CSS_TOKEN_NUMBER => true, _ => false }) {
                            //debug!("Entering: parseNth:: /* Expect NUMBER */");

                            *ctx += 1;

                            /* If we've already seen a sign, ensure one
                             * does not occur at the start of this token
                             */
                            if had_sign && lwc_ref.lwc_string_length(token.idata.get_ref().clone()) > 0 {
                                data = lwc_ref.lwc_string_data(token.idata.get_ref().clone());
                                data_index = 0;

                                if data.char_at(data_index + 0) == '-' || data.char_at(data_index + 0) == '+'
                                {
                                    //debug!("Exiting: parseNth (11)");
                                    return (CSS_INVALID,None)   
                                }                                   
                            }

                            let (ret_b,consumed) = css__number_from_lwc_string(lwc_ref,token.idata.get_ref().clone(), true);
                            b = ret_b;
                            //debug!(fmt!("parseNth:: b == %?", b));
                            if consumed != lwc_ref.lwc_string_length(token.idata.get_ref().clone())
                            {
                                //debug!("Exiting: parseNth (12)");
                                return (CSS_INVALID, None)
                            }
                        }
                    }

                    value.a = a >> 10;
                    value.b = (b >> 10) * sign;

                    //debug!(fmt!("parseNth:: value == %?", value));
                }
            },
            CSS_TOKEN_NUMBER  => {
                //debug!("Entering: parseNth:: CSS_TOKEN_NUMBER");
                let (ret_val,consumed) = css__number_from_lwc_string(lwc_ref,token.idata.get_ref().clone(), true);
                if consumed != lwc_ref.lwc_string_length(token.idata.get_ref().clone())
                {
                    //debug!("Exiting: parseNth (13)");
                    return (CSS_INVALID, None)
                }   

                value.a = 0;
                value.b = ret_val >> 10;
            } ,
            _  =>  {
                //debug!("Exiting: parseNth (14)");
                return (CSS_INVALID, None);
            }
        }
    

        consumeWhitespace(vector, ctx);
        //debug!("Exiting: parseNth (15)");
        return (CSS_OK,Some(value))
    }
    // ===========================================================================================================
    // CSS-LANGUAGE implementation/data-structs ends here 
    // ===========================================================================================================

        
    // ===========================================================================================================
    // PARSE-IMPORTANT implementation/data-structs starts here 
    // ===========================================================================================================

    /**
    * #Arguments:
    *  'vector' - Vector of tokens to process.
    *  'ctx'    - Pointer to vector iteration context.
    * #Return Value:
    * '(css_error, u8)' - (CSS_OK, result) on success along with result,  
                    (CSS_INVALID, 0) if "S* ! S* important" is not at the start of the vector.
    * #Post condition:
    *   ctx is updated with the next token to process.
    *   If the input is invalid, then ctx remains unchanged.
    */
    pub fn css__parse_important(&mut self, lwc_ref:&mut ~lwc, propstrings_ref:& css_propstrings, vector:&~[~css_token], ctx:&mut uint) -> (css_error,u8){
        //debug!("Entering: css__parse_important");
        let orig_ctx = *ctx;
        let mut flags :u8 =0;
        
        consumeWhitespace(vector, ctx);

        if *ctx >= vector.len() {
            //debug!("Exiting: css__parse_important (1)");
            return (CSS_OK,flags)
        }
        
        let mut token = &vector[*ctx];
        //debug!(fmt!("css__parse_important:: token == %?", token));
        *ctx += 1; //Iterate
        
        if  tokenIsChar(token, lwc_ref,  '!') {
            
            consumeWhitespace(vector, ctx);

            if *ctx >= vector.len() || match vector[*ctx].token_type { CSS_TOKEN_IDENT => false, _ => true} {
                *ctx = orig_ctx;
                //debug!("Exiting: css__parse_important (2)");
                return (CSS_INVALID,flags)
            }
                        
            token = &vector[*ctx];
            *ctx += 1; //Iterate        

            if propstrings_ref.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), IMPORTANT as uint) {
                flags |= FLAG_IMPORTANT as u8;
            } else {
                *ctx = orig_ctx;
                //debug!("Exiting: css__parse_important (3)");
                return (CSS_INVALID, flags);
            }
        } else {
            *ctx = orig_ctx;
            //debug!("Exiting: css__parse_important (4)");
            return (CSS_INVALID,flags);
        }
        //debug!("Exiting: css__parse_important (5)");
        return (CSS_OK,flags);
    }

    pub fn css__make_style_important(&mut self, style: &mut ~css_style) {
        //debug!("Entering: css__make_style_important");
        let bytecode:&mut ~[u32] = &mut style.bytecode;
        let mut offset = 0;
		let bytecode_len : uint = bytecode.len();
        while offset < bytecode_len {
            
            let opv = bytecode[offset].clone();

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

                            if  value & 0xff == CONTENT_COUNTERS as u16 {
                                    offset+=2; /* two string entries */
                            }

                            //NOT NEEDED 
                            //if value & 0xff ==    CONTENT_OPEN_QUOTE as u16 ||
                            //  value & 0xff == CONTENT_CLOSE_QUOTE as u16 ||
                            //  value & 0xff == CONTENT_NO_OPEN_QUOTE as u16 ||
                            //  value & 0xff == CONTENT_NO_CLOSE_QUOTE as u16 {
                            //  //Do Nothing    
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
                        if value == VOLUME_NUMBER as u16 {
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
}
