use parse::common::*;
use parse::language::*;
use lex::lexer::*;
use utils::errors::*;

use wapcaplet::*;
use extra::time::*;
use std::cast::*;
use std::str::*;

/**
 * Major state numbers
 */
pub enum parse_states {
    sStart = 0,
    sStylesheet = 1,
    sStatement = 2,
    sRuleset = 3,
    sRulesetEnd = 4,
    sAtRule = 5,
    sAtRuleEnd = 6,
    sBlock = 7,
    sBlockContent = 8,
    sSelector = 9,
    sDeclaration = 10,
    sDeclList = 11,
    sDeclListEnd = 12,
    sProperty = 13,
    sValue0 = 14,
    sValue1 = 15,
    sValue = 16,
    sAny0 = 17,
    sAny1 = 18,
    sAny = 19,
    sMalformedDecl = 20,
    sMalformedSelector = 21,
    sMalformedAtRule = 22,
    sInlineStyle = 23,
    sISBody0 = 24,
    sISBody = 25
}

type state =  ~extern fn(parser: &mut css_parser) ->css_error;

pub struct css_parser {
    language: ~css_language,
    lexer: ~css_lexer,
    priv lwc: @mut lwc,

    priv last_was_ws : bool,
    priv match_char : char,
    priv open_items_stack : ~[char],
    priv parse_error : bool,
    priv pushback: Option<@css_token>,
    priv state_stack: ~[(uint,uint)], /*Parser state stack*/
    priv states: ~[state],
    priv tokens: ~[@css_token],
    css_lexer_get_token_time: float
}

impl css_parser {

    /**
    * #Description:
    *   Create a CSS parser (internal).

    * #Arguments:

    *  'language' - 

    *  'lexer' - 

    *  'lwc' - 

    *  'initial' - 

    * #Return Value:
    *   'Option<~css_parser>' - location to receive parser instance.
    */
    fn css__parser_create_internal(language: ~css_language, lexer: ~css_lexer, lwc: @mut lwc, initial:(uint, uint) ) 
        -> Option<~css_parser> {

        debug!("Entering: css__parser_create_internal");
        
        let mut states = ~[
            ~css_parser::parse_start,
            ~css_parser::parse_stylesheet,
            ~css_parser::parse_statement,
            ~css_parser::parse_ruleset,
            ~css_parser::parse_ruleset_end,
            ~css_parser::parse_at_rule,
            ~css_parser::parse_at_rule_end,
            ~css_parser::parse_block,
            ~css_parser::parse_block_content,
            ~css_parser::parse_selector,
            ~css_parser::parse_declaration,
            ~css_parser::parse_decl_list,
            ~css_parser::parse_decl_list_end,
            ~css_parser::parse_property,
            ~css_parser::parse_value_0,
            ~css_parser::parse_value_1,
            ~css_parser::parse_value,
            ~css_parser::parse_any_0,
            ~css_parser::parse_any_1,
            ~css_parser::parse_any,
            ~css_parser::parse_malformed_declaration,
            ~css_parser::parse_malformed_selector,
            ~css_parser::parse_malformed_at_rule,
            ~css_parser::parse_inline_style,
            ~css_parser::parse_IS_body_0,
            ~css_parser::parse_IS_body
        ];

        let mut parser = ~css_parser {
            language: language,
            lexer: lexer,
            lwc: lwc.clone(),

            last_was_ws: false,
            match_char: 0 as char,
            open_items_stack : ~[],
            parse_error: false,
            pushback: None,
            state_stack: ~[],
            states: states,
            tokens: ~[],
	    css_lexer_get_token_time:0f
        };

        parser.state_stack.push(initial);

        Some (parser)
    }

    /**
    * #Description:

    *   Create a CSS parser.

    * #Arguments:

    *  'language' - 

    *  'lexer' - 

    *  'lwc' - 

    * #Return Value:
    *   'Option<~css_parser>' - location to receive parser instance.
    */
    pub fn css__parser_create(language: ~css_language, lexer: ~css_lexer, lwc: @mut lwc) 
        -> Option<~css_parser> {
        debug!("Entering: css__parser_create");
        let initial = ( sStart as uint, 0u );

        css_parser::css__parser_create_internal(language, lexer, lwc, initial)
    }

    /**
    * #Description:
    *   Create a CSS parser for an inline style.

    * #Arguments:

    *  'language' - 

    *  'lexer' - 

    *  'lwc' - 

    * #Return Value:

    *   'Option<~css_parser>' - location to receive parser instance.
    */
    pub fn css__parser_create_for_inline_style(language: ~css_language, lexer: ~css_lexer, lwc: @mut lwc) 
        -> Option<~css_parser> {
        debug!("Entering: css__parser_create_for_inline_style");
        let initial = (sInlineStyle as uint, 0);

        css_parser::css__parser_create_internal(language, lexer, lwc, initial)
    }


    /**
    * #Description:
    *   Parse a chunk of data using a CSS parser.

    * #Arguments:
    *  'data' -  Pointer to the chunk to parse. 

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__parser_parse_chunk(&mut self, data: ~[u8]) -> css_error {
        debug!("Entering: css__parser_parse_chunk");
        self.lexer.css__lexer_append_data(data);

        loop {
            if self.state_stack.is_empty() {
                debug!("Entering: css__parser_parse_chunk:: self.state_stack.is_empty()");
                break;
            }

            debug!(fmt!("css__parser_parse_chunk:: state_stack (1) == %?", self.state_stack));
            let (current_state, _) = self.state_stack[self.state_stack.len()-1];
            debug!(fmt!("css__parser_parse_chunk:: state_stack (2) == %?", self.state_stack));

            unsafe {
                let current_state_enum : parse_states = transmute(current_state);
                debug!(fmt!("css__parser_parse_chunk:: current state == %?", current_state_enum));
            }
            let result = (*self.states[current_state])(self);

            match(result) {
                CSS_OK => loop,
                _ => return result
            }
        }

        CSS_OK
    }

    /**
    * #Description:
    *   Inform a CSS parser that all data has been received.

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    pub fn css__parser_completed(&mut self) -> css_error {
        debug!("Entering: css__parser_completed ");
        self.lexer.css__lexer_append_data(~[]);
        loop {
            if self.state_stack.is_empty() {
                break;
            }
            debug!(fmt!("css__parser_completed, state_stack (1) == %?", self.state_stack));
            let (current_state, _) = self.state_stack[self.state_stack.len()-1];
            debug!(fmt!("css__parser_completed, state_stack (2) == %?", self.state_stack));
            let result = (*self.states[current_state])(self);

            match(result) {
                CSS_OK => loop,
                _ => return result
            }
        }
        CSS_OK
    }

    /* Utility functions */

    /* writing this function in our current architecture is too cumbersome */
    /* the functionality should be implemented by each calling function */
    /* pub fn expect(&mut self, css_token_type token_type) -> css_error */

    /**
    * #Description:
    *   Transition to a new state, ensuring return to the one specified.

    * #Arguments:
    *  'to' -  Destination state. 

    *  'subsequent' -  The state to return to. 
    */
    fn transition(&mut self, to:(uint,uint), subsequent:(uint,uint))
    {
        debug!(fmt!("Entering: transition : to == %? , subsequent == %?",to,subsequent));
        debug!(fmt!("transition:: state_stack 1 == %?", self.state_stack));
        /* Replace current state on the stack with the subsequent one */
        if (!self.state_stack.is_empty()) {
            self.state_stack.pop();
        }
        self.state_stack.push(subsequent);

        /* Push next state on the stack */
        self.state_stack.push(to);

        debug!(fmt!("transition:: state_stack 2 == %?", self.state_stack));

        self.parse_error = false;
    }

    /**
    * #Description:
    *   Transition to a new state, returning to previous state on stack.

    * #Arguments:
    *  'to' -  Destination state. 

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    fn transition_no_ret(&mut self, to:(uint,uint))
    {
        debug!(fmt!("Entering: transition_no_ret : to == %?",to));
        debug!(fmt!("transition_no_ret:: state_stack 1 == %?", self.state_stack));
        /* Replace current state on the stack with destination */
        if (!self.state_stack.is_empty()) {
            self.state_stack.pop();
        }
        self.state_stack.push(to) ;

        debug!(fmt!("transition_no_ret:: state_stack 2 == %?", self.state_stack));

        self.parse_error = false;
    }

    /**
    * #Description:
    *   Return to previous state on the stack.
    */
    fn done(&mut self)
    {
        debug!("Entering: done");
        /* Pop current state from stack */
        debug!(fmt!("done::state_stack 1 == %?", self.state_stack));
        self.state_stack.pop();
        debug!(fmt!("done::state_stack 2 == %?", self.state_stack));
    }

    /**
    * #Description:
    *   Eat whitespace tokens.

    * #Return Value:
    *   'css_error' - CSS_OK on success, appropriate error otherwise.
    */
    fn eat_ws(&mut self) -> css_error
    {
        debug!("Entering: eat_ws");
        let (parser_error, token_option) = self.get_token();
        if (token_option.is_none()) {
            return parser_error;
        }
        let token = token_option.unwrap();
        debug!(fmt!("Entering : eat_ws token.token_type == %?" , token.token_type));
        match token.token_type {
            CSS_TOKEN_S => {
                return CSS_OK;
            }
            _=> {
                self.push_back(token);
                return CSS_OK;
            }
        }
    }

    /**
    * #Description:
    *   Push a token back on the input.

    * #Arguments:
    *  'token' -  The token to push back. 
    */
    fn push_back(&mut self, token: @css_token) {
        debug!("Entering: push_back");
        // debug!("Entering: push_back");
        /*// debug!(fmt!("token == %?", token));
        // debug!(fmt!("self.tokens == %?", self.tokens));*/

        assert!(self.pushback.is_none());

        self.pushback = Some(token);
        self.tokens.pop();
        // debug!("Exiting: push_back");
    }


    fn intern_string (&mut self, string: ~str) -> @mut lwc_string {
        debug!("Entering: intern_string");
        let mut interned_string: Option<@mut lwc_string> = None;

        interned_string = Some(self.lwc.lwc_intern_string(copy string));

        interned_string.unwrap()
    }

    /**
    * #Description:
    *   Retrieve the next token in the input.

    * #Return Value:
    *   '(css_error, Option<@css_token>)' - (CSS_OK, location to receive token) on success, (appropriate error, None) otherwise.
    */
    fn get_token(&mut self) -> (css_error, Option<@css_token>) {

        debug!("Entering: get_token");
        let mut token_option: Option<@css_token>;

        /* Use pushback, if it exists */
        if self.pushback.is_some() {
            token_option = Some(self.pushback.swap_unwrap());
        }
        else {
            let mut start_time = precise_time_ns();
            /* Otherwise, ask the lexer */
            let (lexer_error, lexer_token_option) = self.lexer.css__lexer_get_token();
            let mut end_time = precise_time_ns();
            let css_lexer_get_token_time = (end_time as float - start_time as float);
            self.css_lexer_get_token_time += css_lexer_get_token_time ;

            if (lexer_error as int != CSS_OK as int) {
                return (lexer_error, None);
            }

            let mut t = lexer_token_option.unwrap();

            let mut start_time = precise_time_ns();
            /* If the last token read was whitespace, keep reading
             * tokens until we encounter one that isn't whitespace */
            while (self.last_was_ws && t.token_type as int == CSS_TOKEN_S as int) {
                let (lexer_error, lexer_token_option) = self.lexer.css__lexer_get_token();
            let mut end_time = precise_time_ns();
            let css_lexer_get_token_time = (end_time as float - start_time as float);
            self.css_lexer_get_token_time += css_lexer_get_token_time ;
                if (lexer_error as int != CSS_OK as int) {
                    return (lexer_error, None);
                }

                t = lexer_token_option.unwrap();
            }

            if ((t.token_type as int) < (CSS_TOKEN_LAST_INTERN as int)) {
                let idata = Some(self.intern_string(from_bytes(t.data.data)));

                let t1_data = css_token_data {
                    data: copy t.data.data,
                    len: t.data.len
                };

                let t1 = @css_token{
                    data:t1_data,
                    token_type:t.token_type,
                    idata:idata,
                    col:t.col,
                    line:t.line
                };

                token_option = Some(t1);
            }
            else {
                let t1_data = css_token_data {
                    data: copy t.data.data,
                    len: t.data.len
                };

                let t1 = @css_token{
                    data:t1_data,
                    token_type:t.token_type,
                    idata:None,
                    col:t.col,
                    line:t.line
                };

                token_option = Some(t1);
            }
           
        }

        self.tokens.push(token_option.get());
        self.last_was_ws = (token_option.get().token_type as int == CSS_TOKEN_S as int);

        debug!(fmt!("token_option == %?",token_option)) ;
        (CSS_OK, token_option)
    }

    #[inline(always)]
    fn update_current_substate(&mut self, new_substate:uint) {
        debug!("Entering update_current_substate");
        debug!(fmt!("update_current_substate: state stack1 == %?" , self.state_stack));
        let (current_state,_) = self.state_stack.pop();
        self.state_stack.push((current_state, new_substate));
        debug!(fmt!("update_current_substate: state stack2 == %?" , self.state_stack));
    }

    /* parser states */
    fn parse_start(parser:&mut css_parser) -> css_error {
        debug!("Entering: parse_start");
        enum parse_start_sub_states { 
            Initial = 0, 
            AfterWS = 1, 
            AfterStylesheet = 2 
        };

        // debug!(fmt!("parse_start: state_stack (1) == %?", parser.state_stack));
        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state,current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sStart as uint);
        // debug!(fmt!("parse_start: state_stack (2) == %?", parser.state_stack));

        while (true) {
            match (current_substate) {
                0 /*Initial*/ => {
                    parser.language.language_handle_event(CSS_PARSER_START_STYLESHEET, &parser.tokens);

                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }

                    current_substate = AfterWS as uint;
                    parser.update_current_substate(AfterWS as uint);
                },
                1 /*AfterWS*/ => {
                    let to = (sStylesheet as uint, Initial as uint);
                    let subsequent = (sStart as uint, AfterStylesheet as uint);
                    parser.transition(to, subsequent);

                    return CSS_OK;
                },
                2 /*AfterStylesheet*/ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match token.token_type {
                        CSS_TOKEN_EOF => {
                            /* do nothing, as expected*/
                            break;
                        }
                        _=> {
                            parser.push_back(token);
                            return CSS_INVALID;
                        }
                    }
                } /*AfterStylesheet*/,

                _ => {
                    fail!();
                }
            }
        }


        parser.language.language_handle_event(CSS_PARSER_END_STYLESHEET, &parser.tokens);
        parser.tokens.clear();

        parser.done();
        return CSS_OK;

    } /* parse_start */


    fn parse_stylesheet(parser:&mut css_parser) -> css_error {
        debug!("Entering: parse_stylesheet");
        enum parse_stylesheet_sub_states { 
            Initial = 0, 
            WS = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sStylesheet as uint);

            while (true) {
                match (current_substate) {
                    0 /*Initial*/=> {
                        let (parser_error, token_option) = parser.get_token();
                        if (token_option.is_none()) {
                            return parser_error;
                        }
                        let token = token_option.unwrap();

                        match token.token_type {
                            CSS_TOKEN_EOF => {
                                parser.push_back(token);
                                parser.tokens.clear();
                                parser.done();
                                return CSS_OK;
                            } /* CSS_TOKEN_EOF */
                            
                            CSS_TOKEN_CDO | CSS_TOKEN_CDC => {
                                /*do nothing*/
                            }
                            _ => {
                                parser.push_back(token);

                                let to = (sStatement as uint, Initial as uint);
                                let subsequent = (sStylesheet as uint, WS as uint);

                                parser.transition(to, subsequent);

                                return CSS_OK;
                            } /* _ */
                        }
                        current_substate = WS as uint;
                        parser.update_current_substate(WS as uint);
                    } /* Initial */

                    1 /* WS */=> {
                        let eat_ws_result = parser.eat_ws();
                        match (eat_ws_result) {
                            CSS_OK => {
                                current_substate = Initial as uint;
                                parser.update_current_substate(Initial as uint);
                            }
                            _ => {
                                return eat_ws_result;
                            }
                        }
                    } /* WS */

                    _ => {
                        /* error */
                        fail!();
                    }
                }
            } /* while */

        CSS_OK
    } /* parse_stylesheet */

    fn parse_statement(parser: &mut css_parser) -> css_error
    {
        debug!("Entering: parse_statement");
        enum parser_statement_sub_states { 
            Initial = 0 
        };

        let (current_state, _) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sStatement as uint);

        let mut to = (sRuleset as uint, Initial as uint);

        let (parser_error, token_option) = parser.get_token();
        if (token_option.is_none()) {
            return parser_error;
        }
        let token = token_option.unwrap();

        match (token.token_type) {
            CSS_TOKEN_ATKEYWORD => {
                to = (sAtRule as uint, Initial as uint);
            }
            _ => {}
        }

        parser.push_back(token);

        parser.transition_no_ret(to);
        return CSS_OK;
    } /* parse statement */


    fn parse_ruleset(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_ruleset");
        enum parse_ruleset_sub_states { 
            Initial = 0, 
            Brace = 1, 
            WS = 2 
        };
        
        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sRuleset as uint);

        while (true) {
            debug!(fmt!("Entering: parse_ruleset:: current_substate is = %? ",current_substate)); 
            match (current_substate) {
                0 /* Initial */ => { 
                    debug!("Entering: parse_ruleset:: substate-initial");   
                    parser.tokens.clear();

                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_CHAR => {
                            debug!("Entering: parse_ruleset:: substate-initial::CSS_TOKEN_CHAR");   
                            let c = token.data.data[0] as char;
                            if (c=='{') {
                                match (
                                    parser.language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens)
                                ) {
                                    CSS_INVALID => {
                                        debug!("Entering: parse_ruleset:: substate-initial::CSS_INVALID");   
                                        let to = (sMalformedSelector as uint, Initial as uint);
                                        parser.transition_no_ret(to);

                                        return CSS_OK;
                                    } /* CSS_INVALID */
                                    _ => {
                                        debug!("Entering: parse_ruleset:: substate-initial:: WS");   
                                        current_substate = WS as uint;
                                        parser.update_current_substate(WS as uint);
                                    }
                                }
                            }
                            else{
                                let to = (sSelector as uint, Initial as uint);
                                let subsequent = (sRuleset as uint, Brace as uint);

                                parser.push_back(token);
                                
                                parser.transition(to, subsequent);
                                return CSS_OK;
                            }

                        }

                        _ => {

                            let to = (sSelector as uint, Initial as uint);
                            let subsequent = (sRuleset as uint, Brace as uint);

                            parser.push_back(token);
                            
                            parser.transition(to, subsequent);
                            return CSS_OK;
                        }
                    } /* match token.token_type */
                } /* Initial */
            
                1 /* Brace */ => {
                    debug!("Entering: parse_ruleset:: substate-brace");  
                    if (!parser.parse_error) {
                        match (
                            parser.language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens)
                        ) {
                            CSS_INVALID => {
                                debug!("Entering: language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens) => CSS_INVALID");
                                parser.parse_error = true;
                            }
                            _ => {

                            }
                        }
                    } /* if */

                    if (parser.parse_error) {
                        let to = (sMalformedSelector as uint, Initial as uint);

                        parser.transition_no_ret(to);

                        return CSS_OK;
                    }

                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */
                        
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '{') {
                                fail!(); // Should not happen
                            }
                            current_substate = WS as uint;
                            parser.update_current_substate(WS as uint);
                        }

                        _ => {
                            fail!(); // Should not happen
                        }
                    } /* match token_type */
                }

                2 /* WS */ => {
                    debug!("Entering: parse_ruleset:: substate-WS");  
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                }

                _ => {
                    fail!();
                }
            } /* match current_substate */

        } /* while */

        let mut to = (sRulesetEnd as uint, Initial as uint);
        parser.transition_no_ret(to);

        CSS_OK
    } /* parse_ruleset */


    fn parse_ruleset_end(parser:&mut css_parser) -> css_error {
        debug!("Entering: parse_ruleset_end");
        enum parse_ruleset_end_substates { 
            Initial = 0, 
            DeclList = 1, 
            Brace = 2, 
            WS = 3 
        };
        
        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sRulesetEnd as uint);

        while (true) {
            debug!(fmt!("parse_ruleset_end: current_substate == %?" , current_substate));
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();
                    parser.push_back(token);

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '}' && c != ';') {
                                /* If this can't possibly be the start of a decl-list, then
                                 * attempt to parse a declaration. This will catch any invalid
                                 * input at this point and read to the start of the next
                                 * declaration. FIRST(decl-list) = (';', '}') */
                                
                                
                                let to = (sDeclaration as uint, Initial as uint);
                                let subsequent = (sRulesetEnd as uint, DeclList as uint);

                                parser.transition(to, subsequent);
                                return CSS_OK;
                            } /* if */
                            current_substate = DeclList as uint;
                            parser.update_current_substate(DeclList as uint);
                        } /* CSS_TOKEN_CHAR */

                        _ => {
                                
                            let to = (sDeclaration as uint, Initial as uint);
                            let subsequent = (sRulesetEnd as uint, DeclList as uint);

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        } /* _ */
                    } /* match token_type */
                } /* Initial */

                1 /* DeclList */ =>  {
                    let to = (sDeclList as uint, Initial as uint);
                    let subsequent = (sRulesetEnd as uint, Brace as uint);

                    parser.transition(to,subsequent);
                    return CSS_OK;
                } /* DeclList */

                2 /* Brace */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    debug!(fmt!("parse_ruleset_end: token.token_type == %?" , token.token_type));
                    match token.token_type {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '}') {
                                /* This should never happen, as FOLLOW(decl-list)
                                 * contains only '}' */
                                fail!(~"Expected }");
                            }
                            current_substate = WS as uint;
                            parser.update_current_substate(WS as uint);
                        }
                        _ => {
                            fail!();
                        }
                    }
                } /* Brace */

                3 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */
                _ => {
                    fail!();
                }
            } /* match current_substate */
        } /* while */

        parser.language.language_handle_event(CSS_PARSER_END_RULESET, &parser.tokens);

        parser.done();
        CSS_OK
    } /* parse_ruleset_end */

    fn parse_at_rule(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_at_rule");
        enum parse_at_rule_substates { 
            Initial = 0, 
            WS = 1, 
            Any = 2, 
            AfterAny = 3 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sAtRule as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    parser.tokens.clear();

                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_ATKEYWORD => {
                            current_substate = WS as uint;
                            parser.update_current_substate(WS as uint);      
                        }
                        _ => {
                            fail!();
                        }
                    }
                } /* Initial */

                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            current_substate = Any as uint;
                            parser.update_current_substate(Any as uint);
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                2 /* Any */ => {
                    let to = ( sAny0 as uint, Initial as uint);
                    let subsequent = ( sAtRule as uint, AfterAny as uint);

                    parser.transition(to, subsequent);
                    return CSS_OK;
                } /* Any */

                3 /* AfterAny */ => {
                    if (parser.parse_error) {
                        let to = (sMalformedAtRule as uint, Initial as uint);

                        parser.transition_no_ret(to);
                        return CSS_OK;
                    } /* if */
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c==')' || c==']') {
                                let to = (sAny0 as uint, Initial as uint);
                                let subsequent = (sAtRule as uint, AfterAny as uint);

                                parser.transition(to, subsequent);
                                return CSS_OK;
                            }
                            else {
                                parser.push_back(token);
                                break;
                            }
                        } /* CSS_TOKEN_CHAR */
                        _ => {
                            parser.push_back(token);
                            break;
                        } /* _ */
                    }
                } /* AfterAny */

                _ => {
                    fail!();
                } /* _ */
            } /* match current_substate */
        } /* while */

        let to = (sAtRuleEnd as uint, Initial as uint);
        parser.transition_no_ret(to);

        CSS_OK
    } /* parse_at_rule */

    fn parse_at_rule_end(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_at_rule_end");
        enum parser_at_rule_end_substates { 
            Initial = 0, 
            WS = 1, 
            AfterBlock = 2 
        };
        
        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sAtRuleEnd as uint);

        while (true) {
            match (current_substate) {

                0 /* Initial */ => {
                    match(parser.language.language_handle_event(CSS_PARSER_START_ATRULE, & parser.tokens)) {
                        CSS_INVALID => {
                            let to = (sMalformedAtRule as uint, Initial as uint);

                            parser.transition_no_ret(to);
                            return CSS_OK;
                        }
                        _=> {}
                    }

                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c=='{') {
                                parser.push_back(token);

                                let to = (sBlock as uint, Initial as uint);
                                let subsequent = (sAtRuleEnd as uint, AfterBlock as uint);
                                
                                parser.transition(to,subsequent);
                                return CSS_OK;
                            } /* if */
                            else if (c!=';') {
                                fail!(~"Expected ;")
                            }
                        }

                        _ => {
                            /* should never happen */
                                fail!();
                        }
                    } /* match token_type */
                    current_substate = WS as uint;
                    parser.update_current_substate(WS as uint);
                } /* Initial */

                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                2 /* AfterBlock */ => {
                    break;
                } /* AfterBlock */

                _ => {
                    fail!();
                }

            } /* match current_substate */
        } /* while */
    
        parser.language.language_handle_event(CSS_PARSER_END_ATRULE, &parser.tokens);

        parser.done();
        CSS_OK
    } /* parse_at_rule_end */

    fn parse_block(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_block");
        enum parse_block_substates { 
            Initial = 0, 
            WS = 1, 
            Content = 2, 
            Brace = 3, 
            WS2 = 4 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sBlock as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    parser.language.language_handle_event(CSS_PARSER_START_BLOCK, &parser.tokens);

                    match (token.token_type) {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '{') {
                                /* This should never happen, as FIRST(block) == '{' */
                                fail!(~"Expected {");
                            }
                        }
                        _ => {
                            /* This should never happen, as FIRST(block) == '{' */
                            fail!();
                        }
                    } /* match token_type */

                    parser.tokens.clear();
                    current_substate = WS as uint;
                    parser.update_current_substate(WS as uint);
                } /* Initial */
                
                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            current_substate = Content as uint;
                            parser.update_current_substate(Content as uint);
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                2 /* Content */ => {
                    let to = (sBlockContent as uint, Initial as uint);
                    let subsequent = (sBlock as uint, Brace as uint);

                    parser.transition(to, subsequent);
                    return CSS_OK;
                } /* Content */

                3 /* Brace */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '}') {
                                /* This should never happen, as 
                                 * FOLLOW(block-content) == '}' */
                                fail!(~"Expected }");
                            }
                        } /* CSS_TOKEN_CHAR */

                        _ => {
                            fail!();
                        }
                    } /* match token_type */

                    current_substate = WS2 as uint;
                    parser.update_current_substate(WS2 as uint);
                } /* Brace */

                4 /* WS2 */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS2 */

                _ => {
                    fail!();
                }
            } /* match current_substate */
        } /* while */
        
        parser.language.language_handle_event(CSS_PARSER_END_BLOCK, &parser.tokens);
        parser.tokens.clear();
        parser.done();

        CSS_OK
    } /* parse_block */

    fn parse_block_content(parser: &mut css_parser) -> css_error {
            debug!("Entering: parse_block_content");
            enum parse_block_content_substates { 
                Initial = 0, 
                WS = 1 
            };
            
        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sBlockContent as uint);

            while (true) {
                match (current_substate) {
                    0 /* Initial */ => {
                        let (parser_error, token_option) = parser.get_token();
                        if (token_option.is_none()) {
                            return parser_error;
                        }
                        let mut token = token_option.unwrap();

                        match (token.token_type) {
                            CSS_TOKEN_ATKEYWORD => {
                                current_substate = WS as uint;
                            } /* CSS_TOKEN_ATKEYWORD */
                            
                            CSS_TOKEN_CHAR => {
                                let c = token.data.data[0] as char;
                                if (c=='{') { /* Grammar ambiguity. Assume block */
                                    parser.push_back(token);
                                    parser.language.language_handle_event(
                                        CSS_PARSER_BLOCK_CONTENT, &parser.tokens);
                                    parser.tokens.clear();

                                    let to = (sBlock as uint, Initial as uint);
                                    let subsequent = (sBlockContent as uint, Initial as uint);

                                    parser.transition(to, subsequent);
                                    return CSS_OK;
                                } /* if */
                                else if (c==';') { /* Grammar ambiguity. Assume semi */
                                    parser.push_back(token);
                                    parser.language.language_handle_event(
                                        CSS_PARSER_BLOCK_CONTENT, &parser.tokens);

                                    let (parser_error, token_option) = parser.get_token();
                                    if (token_option.is_none()) {
                                        return parser_error;
                                    }
                                    token = token_option.unwrap();
                                    // TODO <Abhijeet> : Doesn't get used anywhere, why?

                                    parser.tokens.clear();

                                    current_substate = WS as uint;
                                    parser.update_current_substate(WS as uint);
                                } /* else if */
                                else if (c=='}') { /* Grammar ambiguity. Assume end */
                                    parser.push_back(token);
                                    
                                    parser.language.language_handle_event(
                                        CSS_PARSER_BLOCK_CONTENT, &parser.tokens);
                                    parser.tokens.clear();

                                    parser.done();
                                    return CSS_OK;
                                } /* else if */
                            } /* CSS_TOKEN_CHAR */

                            CSS_TOKEN_EOF => {
                                parser.push_back(token);
                                
                                parser.language.language_handle_event(
                                    CSS_PARSER_BLOCK_CONTENT, &parser.tokens);
                                parser.tokens.clear();

                                parser.done();
                                return CSS_OK;
                            } /* CSS_TOKEN_EOF */

                            _ => {

                            }
                        } /* match token_type */

                        if (current_substate == Initial as uint) {
                            parser.push_back(token);
                            
                            let to = (sAny as uint, Initial as uint);
                            let subsequent = (sBlockContent as uint, Initial as uint);

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        } /* if */

                    } /* Initial */

                    1 /* WS */ => {
                        let eat_ws_result = parser.eat_ws();
                        match (eat_ws_result) {
                            CSS_OK => {
                                
                            }
                            _ => {
                                return eat_ws_result;
                            }
                        }
                        current_substate = Initial as uint;
                        parser.update_current_substate(Initial as uint);
                    } /* WS */

                    _ => {
                        fail!();
                    } /* _ */
                } /* match current_substate */
            } /* while */
        
        parser.done();
        CSS_OK
    } /* parse_block_content */

    fn parse_selector(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_selector");
        enum parse_selector_substates { 
            Initial = 0,
            AfterAny1 = 1 
        };
        
        let (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sSelector as uint);

        match (current_substate) {
            0 /* Initial */ => {
                parser.tokens.clear();

                let to = (sAny1 as uint, Initial as uint);
                let subsequent = (sSelector as uint, AfterAny1 as uint);

                parser.transition(to, subsequent);
                return CSS_OK;              
            } /* Initial */

            1 /* AfterAny1 */ => {
                /* do nothing */
            } /* AfterAny1 */

            _ => {
                fail!();
            }
        }
        
        parser.done();
        CSS_OK
    } /* parse_selector */

    fn parse_declaration(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_declaration");
        enum parser_declaration_substates { 
            Initial = 0, 
            Colon = 1, 
            WS = 2, 
            AfterValue1 = 3 
        };

        /* declaration -> property ':' ws value1 */

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sDeclaration as uint);

        while (true) {
            debug!(fmt!("parse_declaration:: current_substate == %?", current_substate));
            match (current_substate) {
                0 /* Initial */ => {
                    parser.tokens.clear();

                    let to = ( sProperty as uint, Initial as uint);
                    let subsequent = ( sDeclaration as uint, Colon as uint);
                    parser.transition(to, subsequent);

                    return CSS_OK;
                } /* Initial */
                
                1 /* Colon */ => {
                    if (parser.parse_error) {
                        let to = ( sMalformedDecl as uint, Initial as uint);
                        parser.parse_error = false;

                        parser.transition_no_ret(to);

                        return CSS_OK;
                    }

                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != ':') { /* parse error -- expected : */
                                parser.push_back(token);
                                
                                let to = (sMalformedDecl as uint, Initial as uint);
                                
                                parser.transition_no_ret(to);
                                return CSS_OK;
                            } /* if */
                        } /* CSS_TOKEN_CHAR */

                        _ => { /* parse error -- expected : */
                            parser.push_back(token);

                            let to = (sMalformedDecl as uint, Initial as uint);
                            
                            parser.transition_no_ret(to);
                            return CSS_OK;
                        } /* _ */
                    } /* match token_type */

                    current_substate = WS as uint; /* Fall through */
                    parser.update_current_substate(WS as uint);
                } /* Colon */

                2 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            let to = (sValue1 as uint, Initial as uint);
                            let subsequent = (sDeclaration as uint, AfterValue1 as uint);
                            
                            parser.transition(to, subsequent);
                            return CSS_OK;                          
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                3 /* AfterValue1 */ => {
                    if (parser.parse_error) {
                        parser.parse_error = false;

                        let to = (sMalformedDecl as uint, Initial as uint);
                        parser.transition_no_ret(to);

                        return CSS_OK;
                    }
                    debug!(fmt!("parse_declaration:: AfterValue1:: parser.tokens == %?", parser.tokens));
                    parser.language.language_handle_event(CSS_PARSER_DECLARATION, &parser.tokens);
                    break;
                } /* AfterValue1 */

                _ => {
                    fail!();
                }
            } /* match current_substate */
        } /* while */

        parser.done();
        CSS_OK
    } /* parse_declaration */

    fn parse_decl_list(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_decl_list");
        enum parse_decl_list_substates { 
            Initial = 0, 
            WS = 1 
        };

        /* decl-list -> ';' ws decl-list-end
         *           ->
         */

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sDeclList as uint);
        while (true) {
            debug!(fmt!("Entering: decl-list: current_substate == %?" , current_substate));
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != ';' && c != '}') { /* Should never happen */
                                fail!();
                            } /* if */

                            if (c=='}') {
                                parser.push_back(token);
                                parser.done();
                                return CSS_OK;
                            } /* if */
                            else { /* ; */
                                current_substate = WS as uint; /* Fall through */
                                parser.update_current_substate(WS as uint);
                            } /* else */
                        } /* CSS_TOKEN_CHAR */

                        _ => { /* parse error -- expected : */
                            fail!(); /* Should never happen */
                        }
                    } /* match token_type */
                } /* Initial */

                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                _ => {
                    fail!();
                }
            } /* match current_substate */
        }/* while */

        let to = (sDeclListEnd as uint, Initial as uint);
        parser.transition_no_ret(to);

        CSS_OK
    } /* parse_decl_list */

    fn parse_decl_list_end(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_decl_list_end");
        enum parse_decl_list_end_substates { 
            Initial = 0, 
            AfterDeclaration = 1 
        };

        /* decl-list-end -> declaration decl-list 
         *               -> decl-list
         */
        
        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sDeclListEnd as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();
                    
                    if (token.token_type as int != CSS_TOKEN_CHAR as int) || (token.data.data[0] != ';' as u8 && token.data.data[0] != '}' as u8) {
                        
                        parser.push_back(token);
                        let to = (sDeclaration as uint, Initial as uint);
                        let subsequent = (sDeclListEnd as uint, AfterDeclaration as uint);
                        parser.transition(to, subsequent);
                        return CSS_OK;
                    }
                    else {
                        parser.push_back(token);
                    }
                    
                    current_substate = AfterDeclaration as uint; /* fall through */
                    parser.update_current_substate(AfterDeclaration as uint);
                } /* Initial */

                1 /* AfterDeclaration */ => {
                    break;
                } /* AfterDeclaration */

                _ => {
                    fail!();
                }
            }
        } /* while */

        let to = (sDeclList as uint, Initial as uint);
        
        parser.transition_no_ret(to);
        CSS_OK
    } /* parse_decl_list_end */

    fn parse_property(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_property");
        enum parse_property_substates { 
            Initial = 0, 
            WS = 1 
        };

        /* property -> IDENT ws */

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sProperty as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            
                            parser.done();
                            return CSS_OK;
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_IDENT => {
                            current_substate = WS as uint; /* fall through */
                            parser.update_current_substate(WS as uint);
                        }/* CSS_TOKEN_IDENT */

                        _ => { /* parse error */
                            parser.parse_error = true;

                            parser.done();
                            return CSS_OK;
                        }
                    } /* match token_type */
                } /* Initial */

                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                _ => {
                    fail!();
                } /* _ */
            } /* match current_substate */
        } /* while */

        parser.done();
        CSS_OK
    } /* parse_property */

    fn parse_value_0(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_value_0");
        enum parse_value_0_substates { 
            Initial = 0, 
            AfterValue = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sValue0 as uint);

        while(true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;  
                        }/* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => { 
                            let c = token.data.data[0] as char;
                            if (c==';' || c=='}') { /* Grammar ambiguity -- assume ';' or '}' mark end */
                                parser.push_back(token);
                                parser.done();
                                return CSS_OK;
                            }
							parser.push_back(token);
                            
                            let to = ( sValue as uint, Initial as uint );
                            let subsequent = ( sValue0 as uint, AfterValue as uint );

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        } /* CSS_TOKEN_CHAR */

                        _ => {
                            parser.push_back(token);
                            
                            let to = ( sValue as uint, Initial as uint );
                            let subsequent = ( sValue0 as uint, AfterValue as uint );

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        } /* _ */
                    } /* match token_type */
                } /* Initial */

                1 /* AfterValue */ => {
                    if (parser.parse_error) {
                        parser.done();
                        return CSS_OK;
                    }

                    current_substate = Initial as uint;
                    parser.update_current_substate(Initial as uint);
                } /* AfterValue */ 

                _ => {
                    fail!();
                }
            } /* match current_substate */
        } /* while */

        parser.done();
        CSS_OK
    } /* parse_value_0 */

    fn parse_value_1(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_value_1");
        enum parse_value_1_substates { 
            Initial = 0, 
            AfterValue = 1 
        };
        
        /* value1 -> value value0 */
        
        let (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sValue1 as uint);

        match (current_substate) {
            0 /* Initial */ => {
                let (parser_error, token_option) = parser.get_token();
                if (token_option.is_none()) {
                    return parser_error;
                }
                let token = token_option.unwrap();

                let to = ( sValue as uint, Initial as uint );
                let subsequent = ( sValue1 as uint, AfterValue as uint );
                parser.push_back(token);

                match (token.token_type) {
                    CSS_TOKEN_CHAR => {
                        let c = token.data.data[0] as char;

                        if (c==';' || c=='}') {
                            /* Grammar ambiguity -- assume ';' or '}' mark end */
                            parser.parse_error = true;

                            parser.done();
                            return CSS_OK;
                        } /* if */

                        parser.transition(to, subsequent);
                        return CSS_OK;
                    }

                    _ => {

                        parser.transition(to, subsequent);
                        return CSS_OK;
                    }
                } /* match token_type */
            } /* Initial */

            1 /* AfterValue */ => {
                if (parser.parse_error) {
                    parser.done();
                    return CSS_OK;
                } /* if */
            } /* AfterValue */

            _ => {
                fail!();
            }
        } /* match current_substate */

        let to = (sValue0 as uint, Initial as uint);

        parser.transition_no_ret(to);
        CSS_OK
    } /* parse_value_1 */

    fn parse_value(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_value");
        enum parse_value_substates { 
            Initial = 0, 
            WS = 1 
        };

        /* value  -> any
         *        -> block
         *        -> ATKEYWORD ws
         */

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sValue as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_ATKEYWORD => {
                            current_substate = WS as uint;
                            parser.update_current_substate(WS as uint);
                        }
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            parser.push_back(token);

                            let mut to = (sAny as uint, Initial as uint);

                            if (c=='{') {
                                to = (sBlock as uint, Initial as uint);
                            }

                            parser.transition_no_ret(to);
                            return CSS_OK;
                        }
                        _ => {
                            parser.push_back(token);

                            let mut to = (sAny as uint, Initial as uint);

                            parser.transition_no_ret(to);
                            return CSS_OK;
                        }
                    } /* match token_type */
                } /* Initial */

                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                _ => {
                    fail!();
                } /* _ */
            } /* match current_substate */
        }/* while */

        parser.done();
        CSS_OK
    } /* parse_value */

    fn parse_any_0(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_any_0");
        enum parse_any_0_substates { 
            Initial = 0, 
            AfterAny = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sAny0 as uint);

        while (true) {
            debug!(fmt!("Entering: parse_any_0 :: current_substate=%?",current_substate));
            match (current_substate) {
                0 /* Initial */ => {
                    debug!(fmt!("Entering: parse_any_0 :: case initial "));
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match token.token_type {
                        CSS_TOKEN_EOF => {
                            debug!(fmt!("Entering: parse_any_0 :: case initial : CSS_TOKEN_EOF"));
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        }/* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => { 
                            debug!(fmt!("Entering: parse_any_0 :: case initial : CSS_TOKEN_CHAR"));
                            let c = token.data.data[0] as char;
                            parser.push_back(token);

                            /* Grammar ambiguity: 
                             * assume '{', ';', ')', ']' mark end */
                            if (c == '{' || c == ';' || c == ')' || c == ']') {
                                parser.done();
                                return CSS_OK;
                            }

                            let to =  (sAny as uint, Initial as uint);
                            let subsequent =  (sAny0 as uint, AfterAny as uint);

                            parser.transition(to,subsequent);
                            return CSS_OK;
                        }/* CSS_TOKEN_CHAR */

                        _ => {
                            debug!(fmt!("Entering: parse_any_0 :: case initial : _"));
                            parser.push_back(token);

                            let to =  (sAny as uint, Initial as uint);
                            let subsequent =  (sAny0 as uint, AfterAny as uint);

                            parser.transition(to,subsequent);
                            return CSS_OK;
                        }/* _ */
                    } /* match token_type */
                } /* Initial */

                1 /* AfterAny */ => {
                    debug!(fmt!("Entering: parse_any_0 :: case AfterAny "));
                    if (parser.parse_error) {
                        parser.done();
                        return CSS_OK;
                    }

                    current_substate = Initial as uint;
                    parser.update_current_substate(Initial as uint);
                } /* AfterAny */

                _ => {
                    fail!();
                }
            } /* match current_substate */
        }/* while */
        parser.done();
        CSS_OK
    } /* parse_any_0 */

    fn parse_any_1(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_any_1");
        enum parse_any_1_substates { 
            Initial = 0, 
            AfterAny = 1,
            AfterAny0 = 2
        };

        let (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sAny1 as uint);

        match (current_substate) {
            0 /* Initial */ => {
                let to = ( sAny as uint, Initial as uint );
                let subsequent = ( sAny1 as uint, AfterAny as uint );

                parser.transition(to, subsequent);
                return CSS_OK;
            } /* Initial */

            1 /* AfterAny */ => {
                let to = (sAny0 as uint, Initial as uint);
                let subsequent = (sAny1 as uint, AfterAny0 as uint);

                parser.transition(to, subsequent);
                return CSS_OK;
            } /* AfterAny */

            2 /* AfterAny0 */ => {
                if (parser.parse_error) {
                    parser.done();
                    return CSS_OK;
                }

                let (parser_error, token_option) = parser.get_token();
                if (token_option.is_none()) {
                    return parser_error;
                }
                let token = token_option.unwrap();

                match token.token_type {
                    CSS_TOKEN_CHAR => {
                        let c = token.data.data[0] as char;
                        parser.push_back(token);

                        if (c==';' || c==')' || c==']') {
                            let to = (sAny as uint, Initial as uint);
                            let subsequent = (sAny1 as uint, AfterAny as uint);

                            parser.transition(to,subsequent);
                            return CSS_OK;
                        }
                        else if (c!='{') {
                            parser.parse_error = true;
                        }
                    }

                    _ => {
                        parser.push_back(token);
                        parser.parse_error = true;
                    }
                }
            } /* AfterAny0 */

            _ => {
                fail!();
            }
        }

        parser.done();
        CSS_OK
    } /* parse_any_1 */

    fn parse_any(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_any");
        enum parse_any_substates { 
            Initial = 0, 
            WS = 1,
            AfterAny0 = 2,
            WS2 = 3
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sAny as uint);
        
        while (true) {
            debug!(fmt!("Entering: parse_any:: while(true):: current_substate == %?", current_substate));
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    if token.token_type as int != CSS_TOKEN_IDENT as int &&
                       token.token_type as int != CSS_TOKEN_NUMBER as int &&
                       token.token_type as int != CSS_TOKEN_PERCENTAGE as int &&
                       token.token_type as int != CSS_TOKEN_DIMENSION as int &&
                       token.token_type as int != CSS_TOKEN_STRING as int &&
                       token.token_type as int != CSS_TOKEN_CHAR as int &&
                       token.token_type as int != CSS_TOKEN_URI as int &&
                       token.token_type as int != CSS_TOKEN_HASH as int &&
                       token.token_type as int != CSS_TOKEN_UNICODE_RANGE as int &&
                       token.token_type as int != CSS_TOKEN_INCLUDES as int &&
                       token.token_type as int != CSS_TOKEN_DASHMATCH as int &&
                       token.token_type as int != CSS_TOKEN_PREFIXMATCH as int &&
                       token.token_type as int != CSS_TOKEN_SUFFIXMATCH as int &&
                       token.token_type as int != CSS_TOKEN_SUBSTRINGMATCH as int &&
                       token.token_type as int != CSS_TOKEN_FUNCTION as int {
                            parser.parse_error = true;
                            parser.done();
                            return CSS_OK;
                       }


                    match token.token_type {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            match(c) {
                                '(' => { 
                                    parser.match_char=')';
                                    // current_substate = WS as uint;
                                    parser.update_current_substate(WS as uint);
                                },
                                '[' => {
                                    parser.match_char=']';
                                    // current_substate = WS as uint;
                                    parser.update_current_substate(WS as uint);
                                },
                                _ => {
                                }
                            }
                            
                        }
                        CSS_TOKEN_FUNCTION => {
                            parser.match_char = ')';
                            // current_substate = WS as uint;
                            parser.update_current_substate(WS as uint);
                        }
                        _ => {
                            
                        }
                    } /* match token_type */
                    current_substate = WS2 as uint; /* Fall through */
                    parser.update_current_substate(WS2 as uint);
                } /* Initial */

                1 /* WS */ => {
                    let to =  (sAny0 as uint, Initial as uint) ;
                    let subsequent =  (sAny as uint, AfterAny0 as uint) ;

                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            /* continue */
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }

                    parser.transition(to, subsequent);
                    return CSS_OK;
                } /* WS */

                2 /* AfterAny0 */ => {
                    debug!("Entering: parse_any:: AfterAny0");
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match token.token_type {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            /* Match correct close bracket (grammar ambiguity) */
                            if (c==parser.match_char) { 
                                current_substate = WS2 as uint;
                                parser.update_current_substate(WS2 as uint);
                                loop;
                            }

                            let to = ( sAny0 as uint, Initial as uint );
                            let subsequent = ( sAny as uint, AfterAny0 as uint );

                            parser.transition(to,subsequent);
                            return CSS_OK;
                        }
                        _ => {
                            let to = ( sAny0 as uint, Initial as uint );
                            let subsequent = ( sAny as uint, AfterAny0 as uint );

                            parser.transition(to,subsequent);
                            return CSS_OK;
                        }
                    }
                } /* AfterAny0 */

                3 /* WS2 */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS2 */

                _ => {
                    fail!();
                }

            } /* match current_substate */
        } /* while */
        
        parser.done();
        CSS_OK
    } /* parse_any */

    // TODO review : piyush
    fn parse_malformed_declaration(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_malformed_declaration");
        enum parse_malformed_declaration_substates{ 
            Initial = 0, 
            Go = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sMalformedDecl as uint);

        if (current_substate == Initial as uint) {
            parser.open_items_stack.clear();

            current_substate = Go as uint;
            parser.update_current_substate(Go as uint);
        }

        if (current_substate != Go as uint) {
            fail!();
        }

        /* Go */ /* Fall Through */
        while (true) {
            let (parser_error, token_option) = parser.get_token();
            if (token_option.is_none()) {
                return parser_error;
            }
            let token = token_option.unwrap();

            match token.token_type {
                CSS_TOKEN_EOF => {
                    /* Push the last token (';', '}' or EOF) back */
                    parser.push_back(token);
                    break;
                }/* CSS_TOKEN_EOF */

                CSS_TOKEN_CHAR => {
                    let c = token.data.data[0] as char;
                    match (c) {
                        '{' | '}' | '[' | ']' | '(' | ')' | ';' => {
                            /* If the stack is empty, then we're done if we've got
                             * either a ';' or '}' */
                            if (parser.open_items_stack.is_empty()
                                && (c==';' || c=='}')) {
                                /* Push the last token (';', '}' or EOF) back */
                                parser.push_back(token);
                                break;
                            }

                            /* Regardless, if we've got a semicolon, ignore it */
                            if (c==';') {
                                loop;
                            }

                            /* Get corresponding start tokens for end tokens */
                            let want_char : Option<char> = match c {
                                '}' => Some('{'),
                                ']' => Some('['),
                                ')' => Some('('),
                                _ => None
                            };
                            
                            /* Either pop off the stack, if we've matched the 
                             * current item, or push the start token on */
                            if (!parser.open_items_stack.is_empty()) {
                                let match_char = parser.open_items_stack.pop();

                                if (match_char != want_char.get()) {
                                    parser.open_items_stack.push(match_char);
                                }
                                
                            }
                            else if want_char.is_none() {
                                parser.open_items_stack.push(c);
                            }
                        }

                        _ => loop
                    }
                } /* CSS_TOKEN_CHAR */

                _ => {
                    loop;
                }
            } /* match token_type */
        } /* while */

        /* Discard the tokens we've read */
        parser.tokens.clear();

        parser.done();
        CSS_OK
    } /* parse_malformed_declaration */

    fn parse_malformed_selector(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_malformed_selector");
        enum parse_malformed_selector_substates{ 
            Initial = 0, 
            Go = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sMalformedSelector as uint);

        if (current_substate == Initial as uint) {
            parser.open_items_stack.clear();

            current_substate = Go as uint;
            parser.update_current_substate(Go as uint);
        }

        if (current_substate != Go as uint) {
            fail!();
        }

        /* Go */ /* Fall Through */
        loop {
            let (parser_error, token_option) = parser.get_token();
            if (token_option.is_none()) {
                return parser_error;
            }
            let token = token_option.unwrap();

            match token.token_type {
                CSS_TOKEN_EOF => {
                    break;
                }/* CSS_TOKEN_EOF */

                CSS_TOKEN_CHAR => {
                    let c = token.data.data[0] as char;
                    match (c) {
                        '{' | '}' | '[' | ']' | '(' | ')' => {

                            if (!parser.open_items_stack.is_empty()) {
                                let match_char = parser.open_items_stack[parser.open_items_stack.len()-1];
                                
                                let want_char = match c {
                                    '}' => '{',
                                    ']' => '[',
                                    ')' => '(',
                                    _ => 0 as char
                                };

                                if (match_char == want_char) {
                                    parser.open_items_stack.pop();
                                }
                                else if (want_char == 0 as char) {
                                    parser.open_items_stack.push(c);
                                }

                                if (want_char == '{' && parser.open_items_stack.is_empty()) {
                                    break;
                                }
                            } 
                            else if (c!='}' && c!=')' && c!=']') {
                                parser.open_items_stack.push(c);
                            } 
                        }

                        _ => loop
                    }
                    
                } /* CSS_TOKEN_CHAR */

                _ => {
                    loop;
                }
            } /* match token_type */
        } /* while */
        let eat_ws_result = parser.eat_ws();
        match (eat_ws_result) {
            CSS_OK => {
                parser.tokens.clear();
                parser.done();
                return CSS_OK;
            }
            _ => {
                return eat_ws_result;
            }
        }
    } /* parse_malformed_selector */

    fn parse_malformed_at_rule(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_malformed_at_rule");
        enum parse_malformed_at_rule_substates{ 
            Initial = 0, 
            Go = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sMalformedAtRule as uint);

        if (current_substate == Initial as uint) {
            parser.open_items_stack.clear();

            current_substate = Go as uint;
            parser.update_current_substate(Go as uint);
        }

        if (current_substate != Go as uint) {
            fail!();
        }

        /* Go */ /* Fall Through */
        loop {
            let (parser_error, token_option) = parser.get_token();
            if (token_option.is_none()) {
                return parser_error;
            }
            let token = token_option.unwrap();

            match token.token_type {
                CSS_TOKEN_EOF => {
                    break;
                }/* CSS_TOKEN_EOF */

                CSS_TOKEN_CHAR => {
                    let c = token.data.data[0] as char;
                    match (c) {
                        '{' | '}' | '[' | ']' | '(' | ')' | ';' => {

                            if (c==';' && parser.open_items_stack.is_empty()) {
                                break;
                            }
                            else if (c==';') {
                                loop;
                            }

                            if (!parser.open_items_stack.is_empty()) {
                                let match_char = parser.open_items_stack[parser.open_items_stack.len()-1];
                                
                                let want_char = match c {
                                    '}' => '{',
                                    ']' => '[',
                                    ')' => '(',
                                    _ => 0 as char
                                };

                                if (match_char == want_char) {
                                    parser.open_items_stack.pop();
                                }
                                else if (want_char == 0 as char) {
                                    parser.open_items_stack.push(c);
                                }

                                if (want_char == '{' && parser.open_items_stack.is_empty()) {
                                    break;
                                }
                            }
                            else if (c!='}' && c!=')' && c!=']') {
                                parser.open_items_stack.push(c);
                            }

                        }

                        _ => loop
                    }
                    
                } /* CSS_TOKEN_CHAR */

                _ => {
                    loop;
                }
            } /* match token_type */
        }

        let eat_ws_result = parser.eat_ws();
        match (eat_ws_result) {
            CSS_OK => {
                parser.tokens.clear();
                parser.done();
                return CSS_OK;
            }
            _ => {
                return eat_ws_result;
            }
        }
    } /* parse_malformed_at_rule */

    fn parse_inline_style(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_inline_style");
        enum parse_inline_style_substates { 
            Initial = 0, 
            WS = 1, 
            AfterISBody0 = 2 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sInlineStyle as uint);

        while (true) {
            match current_substate {
                0 /* Initial */ => {
                    parser.language.language_handle_event(
                        CSS_PARSER_START_STYLESHEET, &parser.tokens);
                    parser.language.language_handle_event(
                        CSS_PARSER_START_RULESET, &parser.tokens);

                    current_substate = WS as uint;
                    parser.update_current_substate(WS as uint);
                } /* Initial */

                1 /* WS */ => {
                    // current_substate = WS as uint; // TODO review
                    parser.update_current_substate(WS as uint);
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            /*continue*/
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }

                    let to = (sISBody0 as uint, Initial as uint);
                    let subsequent = (sInlineStyle as uint, AfterISBody0 as uint);

                    parser.transition(to,subsequent);
                    return CSS_OK;
                } /* WS */

                2 /* AfterISBody0 */ => {
                    parser.language.language_handle_event(
                        CSS_PARSER_END_RULESET,&parser.tokens);
                    parser.language.language_handle_event(
                        CSS_PARSER_END_STYLESHEET,&parser.tokens);

                    break;
                } /* AfterISBody0 */

                _ => {
                    fail!();
                }
            } /* match current_substate */
        } /* while */

        parser.done();
        CSS_OK
    } /* parse_inline_style */

    fn parse_IS_body_0(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_IS_body_0");
        enum parse_IS_body_0_substates { 
            Initial = 0, 
            AfterISBody = 1 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sISBody0 as uint);

        loop {
            match current_substate {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();
                    parser.push_back(token);

                    match token.token_type {
                        CSS_TOKEN_EOF => {
                            parser.done();
                            return CSS_OK;
                        }/* CSS_TOKEN_EOF */

                        _=> {
                            let to = ( sISBody as uint, Initial as uint );
                            let subsequent = ( sISBody0 as uint, AfterISBody as uint );

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        } /* _ */
                    } /* match token_type */
                } /* Initial */

                1 /* AfterISBody */ => {
                    if parser.parse_error {
                        parser.done();
                        return CSS_OK;
                    }

                    current_substate = Initial as uint;
                    parser.update_current_substate(Initial as uint);
                } /* AfterISBody */

                _ /* _ */ => {
                    fail!();
                } /* _ */

            } /* match current_substate */
        } /* while */

        // parser.done();
        // CSS_OK
    } /* parse_IS_body_0 */

    fn parse_IS_body(parser: &mut css_parser) -> css_error {
        debug!("Entering: parse_IS_body");
        enum parse_IS_body_substates { 
            Initial = 0, 
            DeclList = 1, 
            Brace = 2, 
            WS = 3 
        };

        let mut current_state:uint = 0;
        let mut current_substate:uint = 0;
        (current_state, current_substate) = parser.state_stack[parser.state_stack.len()-1];
        assert!(current_state == sISBody as uint);

        while(true) {
            match current_substate {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();
                    parser.push_back(token);

                    match token.token_type {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;

                            if (c != '}' && c !=';') {
                                let to = ( sDeclaration as uint, Initial as uint );
                                let subsequent = ( sISBody as uint, DeclList as uint );

                                parser.transition(to, subsequent);
                                return CSS_OK;
                            }

                            current_substate = DeclList as uint; /* fall through */
                            parser.update_current_substate(DeclList as uint);
                        }

                        _ => {
                            
                            let to = ( sDeclaration as uint, Initial as uint );
                            let subsequent = ( sISBody as uint, DeclList as uint );

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        }
                    } /* match token_type */
                } /* Initial */

                1 /* DeclList */ => {
                    let to = (sDeclList as uint, Initial as uint);
                    let subsequent = (sISBody as uint, Brace as uint);

                    parser.transition(to,subsequent);
                    return CSS_OK;
                } /* DeclList */

                2 /* Brace */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match token.token_type {
                        CSS_TOKEN_EOF => {
                            parser.push_back(token);
                            parser.done();
                            return CSS_OK;
                        }/* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '}') {
                                fail!(~"Expected }");
                            }
                            current_substate = WS as uint; /* fall through */
                            parser.update_current_substate(WS as uint);
                        }

                        _ => {
                            current_substate = WS as uint; /* fall through */
                            parser.update_current_substate(WS as uint);
                        }
                    } /* match token_type */
                } /* Brace */

                3 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            break;
                        }
                        _ => {
                            return eat_ws_result;
                        }
                    }
                } /* WS */

                _ /*  */ => {
                    fail!();
                } /*  */
            } /* match current_substate */
        } /* while */

        parser.done();
        CSS_OK
    } /* parse_IS_body */

}
