use parse::common::*;
use parse::language::*;
use lex::lexer::*;
use utils::errors::*;

use wapcaplet::*;
use std::arc;

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
    priv language: ~css_language,
    priv lexer: ~css_lexer,
    priv lwc: arc::RWARC<~lwc>,

    priv last_was_ws : bool,
    priv match_char : char,
    priv open_items_stack : ~[char],
    priv parse_error : bool,
    priv pushback: Option<@css_token>,
    priv state_stack: ~[(uint,uint)], /*Parser state stack*/
    priv states: ~[state],
    priv tokens: ~[@css_token],
}

pub impl css_parser {

    /* constructor */
    fn css__parser_create_internal(language: ~css_language, lexer: ~css_lexer, lwc: arc::RWARC<~lwc>, initial:(uint, uint) ) 
        -> Option<~css_parser> {
        
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
            
        };

        parser.state_stack.push(initial);

        Some (parser)
    }

    /* public constructors */
    pub fn css__parser_create(language: ~css_language, lexer: ~css_lexer, lwc: arc::RWARC<~lwc>) 
        -> Option<~css_parser> {
        let initial = ( sStart as uint, 0u );

        css_parser::css__parser_create_internal(language, lexer, lwc, initial)
    }

    pub fn css__parser_create_for_inline_style(language: ~css_language, lexer: ~css_lexer, lwc: arc::RWARC<~lwc>) 
        -> Option<~css_parser> {
        let initial = (sInlineStyle as uint, 0);

        css_parser::css__parser_create_internal(language, lexer, lwc, initial)
    }


    pub fn css__parser_parse_chunk(&mut self, data: ~[u8]) -> css_error {
        self.lexer.css__lexer_append_data(data);

        loop {
            if self.state_stack.is_empty() {
                break;
            }

            // io::println(fmt!("css__parser_parse_chunk:: state_stack (1) == %?", self.state_stack));
            let (current_state, _) = self.state_stack[self.state_stack.len()-1];

            unsafe {
                let current_state_enum : parse_states = cast::transmute(current_state);
                // io::println(fmt!("css__parser_parse_chunk:: current state == %?", current_state_enum));
            }
            let result = (*self.states[current_state])(self);

            match(result) {
                CSS_OK => loop,
                _ => return result
            }
        }

        CSS_OK
    }

    pub fn css__parser_completed(&mut self) -> css_error {

        loop {
            if self.state_stack.is_empty() {
                break;
            }
            // io::println(fmt!("css__parser_completed, state_stack (1) == %?", self.state_stack));
            let (current_state, _) = self.state_stack[self.state_stack.len()-1];
            // io::println(fmt!("css__parser_completed, state_stack (2) == %?", self.state_stack));
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

    fn transition(&mut self, to:(uint,uint), subsequent:(uint,uint))
    {
        
        /* Replace current state on the stack with the subsequent one */
        if (!self.state_stack.is_empty()) {
            self.state_stack.pop();
        }
        self.state_stack.push(subsequent);

        /* Push next state on the stack */
        self.state_stack.push(to);

        self.parse_error = false;
    }

    fn transition_no_ret(&mut self, _:(uint,uint))
    {
        /* Replace current state on the stack with destination */
        if (!self.state_stack.is_empty()) {
            self.state_stack.pop();
        }

        self.parse_error = false;
    }

    fn done(&mut self)
    {
        // io::println("Entering: done");
        /* Pop current state from stack */
        self.state_stack.pop();
        // io::println("Leaving: done");
    }

    fn eat_ws(&mut self) -> css_error
    {
        let (parser_error, token_option) = self.get_token();
        if (token_option.is_none()) {
            return parser_error;
        }
        let token = token_option.unwrap();

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

    fn push_back(&mut self, token: @css_token) {
        // io::println("Entering: push_back");
        /*// io::println(fmt!("token == %?", token));
        // io::println(fmt!("self.tokens == %?", self.tokens));*/

        assert!(self.pushback.is_none());

        self.pushback = Some(token);
        self.tokens.pop();
        // io::println("Exiting: push_back");
    }


    fn intern_string (&mut self, string: ~str) -> arc::RWARC<~lwc_string> {
        let mut interned_string: Option<arc::RWARC<~lwc_string>> = None;

        do self.lwc.write |lwc| {
            interned_string = Some(lwc.lwc_intern_string(copy string));
        }

        interned_string.unwrap()
    }

    fn get_token(&mut self) -> (css_error, Option<@css_token>) {

        let mut token_option: Option<@css_token>;

        /* Use pushback, if it exists */
        if self.pushback.is_some() {
            token_option = Some(self.pushback.swap_unwrap());
        }
        else {
            /* Otherwise, ask the lexer */
            let (lexer_error, lexer_token_option) = self.lexer.css__lexer_get_token();

            if (lexer_error as int != CSS_OK as int) {
                return (lexer_error, None);
            }

            let mut t = lexer_token_option.unwrap();
            /* If the last token read was whitespace, keep reading
             * tokens until we encounter one that isn't whitespace */
            while (self.last_was_ws && t.token_type as int == CSS_TOKEN_S as int) {
                let (lexer_error, lexer_token_option) = self.lexer.css__lexer_get_token();
                if (lexer_error as int != CSS_OK as int) {
                    return (lexer_error, None);
                }

                t = lexer_token_option.unwrap();
            }

            if ((t.token_type as int) < (CSS_TOKEN_LAST_INTERN as int)) {
                let idata = Some(self.intern_string(str::from_bytes(copy t.data.data)));

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

        (CSS_OK, token_option)
    }

    /* parser states */
    fn parse_start(parser:&mut css_parser) -> css_error {
        enum parse_start_sub_states { 
            Initial = 0, 
            AfterWS = 1, 
            AfterStylesheet = 2 
        };

        // io::println(fmt!("parse_start: state_stack (1) == %?", parser.state_stack));
        let mut (current_state,current_substate) = parser.state_stack.pop();
        assert!(current_state == sStart as uint);
        // io::println(fmt!("parse_start: state_stack (2) == %?", parser.state_stack));

        while (true) {
            match (current_substate) {
                0 /*Initial*/ => {
                    parser.language.language_handle_event(CSS_PARSER_START_STYLESHEET, &parser.tokens);
                    current_substate = AfterWS as uint;
                },
                1 /*AfterWS*/ => {
                    let to = (sStylesheet as uint, Initial as uint);
                    let subsequent = (sStart as uint, AfterWS as uint);
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

        return CSS_OK;
    } /* parse_start */


    fn parse_stylesheet(parser:&mut css_parser) -> css_error {
        enum parse_stylesheet_sub_states { 
            Initial = 0, 
            WS = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
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
                    } /* Initial */

                    1 /* WS */=> {
                        let eat_ws_result = parser.eat_ws();
                        match (eat_ws_result) {
                            CSS_OK => {
                                current_substate = Initial as uint;
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
        enum parser_statement_sub_states { 
            Initial = 0 
        };

        let mut (current_state, _) = parser.state_stack.pop();
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
        enum parse_ruleset_sub_states { 
            Initial = 0, 
            Brace = 1, 
            WS = 2 
        };
        
        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sRuleset as uint);

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
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c=='{') {
                                match (
                                    parser.language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens)
                                ) {
                                    CSS_INVALID => {
                                        let to = (sMalformedSelector as uint, Initial as uint);
                                        parser.transition_no_ret(to);

                                        return CSS_OK;
                                    } /* CSS_INVALID */
                                    _ => {
                                        current_substate = WS as uint;
                                    }
                                }
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
                    if (!parser.parse_error) {
                        match (
                            parser.language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens)
                        ) {
                            CSS_INVALID => {
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
                        }

                        _ => {
                            fail!(); // Should not happen
                        }
                    } /* match token_type */
                }

                2 /* WS */ => {
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
        enum parse_ruleset_end_substates { 
            Initial = 0, 
            DeclList = 1, 
            Brace = 2, 
            WS = 3 
        };
        
        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sRulesetEnd as uint);

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

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '}' && c != ';') {
                                /* If this can't possibly be the start of a decl-list, then
                                 * attempt to parse a declaration. This will catch any invalid
                                 * input at this point and read to the start of the next
                                 * declaration. FIRST(decl-list) = (';', '}') */
                                parser.push_back(token);
                                
                                let to = (sDeclaration as uint, Initial as uint);
                                let subsequent = (sRulesetEnd as uint, DeclList as uint);

                                parser.transition(to, subsequent);
                                return CSS_OK;
                            } /* if */
                            current_substate = DeclList as uint;
                        } /* CSS_TOKEN_CHAR */

                        _ => {
                            parser.push_back(token);
                                
                            let to = (sDeclaration as uint, Initial as uint);
                            let subsequent = (sRulesetEnd as uint, DeclList as uint);

                            parser.transition(to, subsequent);
                            return CSS_OK;
                        } /* _ */
                    } /* match token_type */
                } /* Initial */

                1 /* DeclList */ =>  {
                    let to = (sDeclList as uint, Initial as uint);
                    let subsequent = (sRuleset as uint, Brace as uint);

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
                        } /* CSS_TOKEN_EOF */

                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            if (c != '}') {
                                /* This should never happen, as FOLLOW(decl-list)
                                 * contains only '}' */
                                fail!();
                            }
                            current_substate = WS as uint;
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

        parser.done();
        CSS_OK
    } /* parse_ruleset_end */

    fn parse_at_rule(parser: &mut css_parser) -> css_error {
        
        enum parse_at_rule_substates { 
            Initial = 0, 
            WS = 1, 
            Any = 2, 
            AfterAny = 3 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
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
        
        enum parser_at_rule_end_substates { 
            Initial = 0, 
            WS = 1, 
            AfterBlock = 2 
        };
        
        let mut (current_state, current_substate) = parser.state_stack.pop();
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
                            else if (c==';') {
                                /* continue */
                            }
                            else {
                                /* should never happen */
                                fail!();
                            }
                        }

                        _ => {
                            /* should never happen */
                                fail!();
                        }
                    } /* match token_type */
                    current_substate = WS as uint;
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
        enum parse_block_substates { 
            Initial = 0, 
            WS = 1, 
            Content = 2, 
            Brace = 3, 
            WS2 = 4 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
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
                                fail!();
                            }
                        }
                        _ => {
                            /* This should never happen, as FIRST(block) == '{' */
                            fail!();
                        }
                    } /* match token_type */

                    parser.tokens.clear();
                    current_substate = WS as uint;
                } /* Initial */
                
                1 /* WS */ => {
                    let eat_ws_result = parser.eat_ws();
                    match (eat_ws_result) {
                        CSS_OK => {
                            current_substate = Content as uint;
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
                                fail!();
                            }
                        } /* CSS_TOKEN_CHAR */

                        _ => {
                            fail!();
                        }
                    } /* match token_type */

                    current_substate = WS2 as uint;
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
        
            enum parse_block_content_substates { 
                Initial = 0, 
                WS = 1 
            };
            
        let mut (current_state, current_substate) = parser.state_stack.pop();
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
        enum parse_selector_substates { 
            Initial = 0, 
            AfterAny1 = 1 
        };
        
        let mut (current_state, current_substate) = parser.state_stack.pop();
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
        enum parser_declaration_substates { 
            Initial = 0, 
            Colon = 1, 
            WS = 2, 
            AfterValue1 = 3 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sDeclaration as uint);

        while (true) {
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
        enum parse_decl_list_substates { 
            Initial = 0, 
            WS = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sDeclList as uint);

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
        enum parse_decl_list_end_substates { 
            Initial = 0, 
            AfterDeclaration = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sDeclListEnd as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match (token.token_type) {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            parser.push_back(token);
                            
                            if (c!=';' && c != '}') {
                                let to = (sDeclaration as uint, Initial as uint);
                                let subsequent = (sDeclListEnd as uint, AfterDeclaration as uint);

                                parser.transition(to, subsequent);
                                return CSS_OK;
                            }
                        } /* CSS_TOKEN_CHAR */
                        _ => {
                            parser.push_back(token);                            
                        }
                    } /* match token_type */
                    
                    current_substate = AfterDeclaration as uint; /* fall through */
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
        enum parse_property_substates { 
            Initial = 0, 
            WS = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
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
        enum parse_value_0_substates { 
            Initial = 0, 
            AfterValue = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
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
        enum parse_value_1_substates { 
            Initial = 0, 
            AfterValue = 1 
        };
        
        let mut (current_state, current_substate) = parser.state_stack.pop();
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

                match (token.token_type) {
                    CSS_TOKEN_CHAR => {
                        let c = token.data.data[0] as char;
                        parser.push_back(token);

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
                        parser.push_back(token);

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
        enum parse_value_substates { 
            Initial = 0, 
            WS = 1 
        };
        
        let mut (current_state, current_substate) = parser.state_stack.pop();
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
        enum parse_any_0_substates { 
            Initial = 0, 
            AfterAny = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sAny0 as uint);

        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
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
                            parser.push_back(token);

                            let to =  (sAny as uint, Initial as uint);
                            let subsequent =  (sAny0 as uint, AfterAny as uint);

                            parser.transition(to,subsequent);
                            return CSS_OK;
                        }/* _ */
                    } /* match token_type */
                } /* Initial */

                1 /* AfterAny */ => {
                    if (parser.parse_error) {
                        parser.done();
                        return CSS_OK;
                    }

                    current_substate = Initial as uint;
                } /* AfterAny */

                _ => {
                    fail!();
                }
            } /* match current_substate */
        }/* while */

        CSS_OK
    } /* parse_any_0 */

    fn parse_any_1(parser: &mut css_parser) -> css_error {
        enum parse_any_1_substates { 
            Initial = 0, 
            AfterAny = 1,
            AfterAny0 = 2
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
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
                        else if (c=='{') {
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
        enum parse_any_substates { 
            Initial = 0, 
            WS = 1,
            AfterAny0 = 2,
            WS2 = 3
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sAny as uint);
        
        while (true) {
            match (current_substate) {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match token.token_type {
                        CSS_TOKEN_IDENT |
                        CSS_TOKEN_NUMBER |
                        CSS_TOKEN_PERCENTAGE |
                        CSS_TOKEN_DIMENSION |
                        CSS_TOKEN_STRING |
                        CSS_TOKEN_URI |
                        CSS_TOKEN_HASH |
                        CSS_TOKEN_UNICODE_RANGE => {

                        }
                        
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            match(c) {
                                '(' => { 
                                    parser.match_char=')';
                                    current_substate = WS as uint;
                                },
                                '[' => {
                                    parser.match_char=']';
                                    current_substate = WS as uint;
                                },
                                _ => {
                                    current_substate = WS2 as uint;
                                }
                            }
                        }
                        CSS_TOKEN_FUNCTION => {
                            parser.match_char = ')';
                            current_substate = WS as uint;
                        }
                        _ => {
                            parser.parse_error = true;
                            parser.done();
                            return CSS_OK;
                        }
                    } /* match token_type */
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

    fn parse_malformed_declaration(parser: &mut css_parser) -> css_error {
        enum parse_malformed_declaration_substates{ 
            Initial = 0, 
            Go = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sMalformedDecl as uint);

        if (current_substate == Initial as uint) {
            parser.open_items_stack.clear();

            current_substate = Go as uint;
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
                    parser.push_back(token);
                    break;
                }/* CSS_TOKEN_EOF */

                CSS_TOKEN_CHAR => {
                    let c = token.data.data[0] as char;
                    match (c) {
                        '{' | '}' | '[' | ']' | '(' | ')' | ';' => {
                            if (parser.open_items_stack.is_empty()
                                && (c==';' || c=='}')) {
                                parser.push_back(token);
                                break;
                            }

                            if (c==';') {
                                loop;
                            }

                            let match_char = parser.open_items_stack[parser.open_items_stack.len()-1];

                            let want_char = match c {
                                '}' => '{',
                                ']' => '[',
                                ')' => '(',
                                _ => fail!()
                            };

                            if (match_char == want_char) {
                                parser.open_items_stack.pop();
                            }
                            else {
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

        CSS_OK
    } /* parse_malformed_declaration */

    fn parse_malformed_selector(parser: &mut css_parser) -> css_error {
        enum parse_malformed_selector_substates{ 
            Initial = 0, 
            Go = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sMalformedSelector as uint);

        if (current_substate == Initial as uint) {
            parser.open_items_stack.clear();

            current_substate = Go as uint;
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
        enum parse_malformed_at_rule_substates{ 
            Initial = 0, 
            Go = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sMalformedAtRule as uint);

        if (current_substate == Initial as uint) {
            parser.open_items_stack.clear();

            current_substate = Go as uint;
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
        enum parse_inline_style_substates { 
            Initial = 0, 
            WS = 1, 
            AfterISBody0 = 2 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sInlineStyle as uint);

        while (true) {
            match current_substate {
                0 /* Initial */ => {
                    parser.language.language_handle_event(
                        CSS_PARSER_START_STYLESHEET, &parser.tokens);
                    parser.language.language_handle_event(
                        CSS_PARSER_START_RULESET, &parser.tokens);

                    current_substate = WS as uint;
                } /* Initial */

                1 /* WS */ => {
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
        enum parse_IS_body_0_substates { 
            Initial = 0, 
            AfterISBody = 1 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sISBody0 as uint);

        while (true) {
            match current_substate {
                0 /* Initial */ => {
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

                        _=> {
                            parser.push_back(token);

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
                } /* AfterISBody */

                _ /* _ */ => {
                    fail!();
                } /* _ */

            } /* match current_substate */
        } /* while */

        parser.done();
        CSS_OK
    } /* parse_IS_body_0 */

    fn parse_IS_body(parser: &mut css_parser) -> css_error {
        enum parse_IS_body_substates { 
            Initial = 0, 
            DeclList = 1, 
            Brace = 2, 
            WS = 3 
        };

        let mut (current_state, current_substate) = parser.state_stack.pop();
        assert!(current_state == sISBody as uint);

        while(true) {
            match current_substate {
                0 /* Initial */ => {
                    let (parser_error, token_option) = parser.get_token();
                    if (token_option.is_none()) {
                        return parser_error;
                    }
                    let token = token_option.unwrap();

                    match token.token_type {
                        CSS_TOKEN_CHAR => {
                            let c = token.data.data[0] as char;
                            parser.push_back(token);

                            if (c != '}' && c !=';') {
                                let to = ( sDeclaration as uint, Initial as uint );
                                let subsequent = ( sISBody as uint, DeclList as uint );

                                parser.transition(to, subsequent);
                                return CSS_OK;
                            }

                            current_substate = DeclList as uint; /* fall through */
                        }

                        _ => {
                            parser.push_back(token);
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
                                fail!();
                            }
                            current_substate = WS as uint; /* fall through */
                        }

                        _ => {
                            current_substate = WS as uint; /* fall through */
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

        CSS_OK
    } /* parse_IS_body */

}