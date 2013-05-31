extern mod std;
extern mod css; // To be removed
extern mod wapcaplet;
extern mod parserutils;

use std::arc;
use wapcaplet::*;
use parserutils::input::inputstream::*;
use parserutils::charset::encodings::utf8impl::*;
use parserutils::utils::errors::*;

use css::utils::errors::*;
use css::utils::parserutilserror::*;


pub enum css_token_type {
    CSS_TOKEN_IDENT,
    CSS_TOKEN_ATKEYWORD,
    CSS_TOKEN_HASH,
    CSS_TOKEN_FUNCTION,
    CSS_TOKEN_STRING,
    CSS_TOKEN_INVALID_STRING,
    CSS_TOKEN_URI,
    CSS_TOKEN_UNICODE_RANGE,
    CSS_TOKEN_CHAR,
    CSS_TOKEN_NUMBER,
    CSS_TOKEN_PERCENTAGE,
    CSS_TOKEN_DIMENSION,

    /* Those tokens that want strings interned appear above */
    
    CSS_TOKEN_CDO,
    CSS_TOKEN_CDC,
    CSS_TOKEN_S,
    CSS_TOKEN_COMMENT,
    CSS_TOKEN_INCLUDES,
    CSS_TOKEN_DASHMATCH,
    CSS_TOKEN_PREFIXMATCH, 
    CSS_TOKEN_SUFFIXMATCH,
    CSS_TOKEN_SUBSTRINGMATCH,
    CSS_TOKEN_EOF 
}

pub static CSS_TOKEN_LAST_INTERN:css_token_type = CSS_TOKEN_CDO;

pub struct css_token_data {
    data: ~[u8],
    len: uint
}

pub struct css_token {
    
    data: css_token_data,

    token_type: css_token_type,
    idata: Option<arc::RWARC<~lwc_string>>,

    col: uint,
    line: uint
}

pub enum states {
    sSTART      =  0,
    sATKEYWORD  =  1,
    sSTRING     =  2,
    sHASH       =  3,
    sNUMBER     =  4, 
    sCDO        =  5,
    sCDC        =  6,
    sS      =  7,
    sCOMMENT    =  8,
    sMATCH      =  9, 
    sURI        = 10,
    sIDENT      = 11,
    sESCAPEDIDENT   = 12,
    sURL        = 13,
    sUCR        = 14 
}

struct _context {
    first: u8,      // first character read from token
    orig_bytes: uint,       // storage of cuurent number of bytes read for rewinding
    last_was_star: bool,
    last_was_cr: bool,
    bytes_for_url: uint,
    data_len_for_url: uint,
    hex_count: int
}

static _state: uint = 4;
static _sub_state: uint = 4;

pub struct css_lexer {
    input: ~inputstream,
    bytes_read_for_token: uint,
    token: Option<~css_token>,
    escape_seen: bool,
    unescaped_token_data: Option<~[u8]>,  // used if eascapeSeen  = true
    state: states,
    substate: uint,
    context: _context,
    emit_comments: bool,
    current_col: uint,
    current_line: uint
}

// pub fn preprocess(input: &str) -> ~str {
//  str::replace(str::replace(str::replace(input,
//  "\r\n", "\n"),
//  "\r", "\n"),
//  "\x00", "\uFFFD")
// }

impl css_lexer {
    pub fn css__lexer_create(inputstream: ~inputstream) -> ~css_lexer {
        let _data = css_token_data {
            data: ~[],
            len: 0
        };
        let _token = ~css_token {
            data: _data,
            token_type: CSS_TOKEN_EOF,
            idata: None,
            col: 0,
            line: 0
        };
        let context_inst = _context {
            first: 0,
            orig_bytes: 0,
            last_was_star: false,
            last_was_cr: false,
            bytes_for_url: 0,
            data_len_for_url: 0,
            hex_count: 0
        };
        ~css_lexer{ 
            input: inputstream,
            bytes_read_for_token: 0,
            token: Some(_token),
            escape_seen: false,
            unescaped_token_data: Some(~[]),
            state: sSTART,
            substate: 0,
            emit_comments: false,
            context: context_inst,      
            current_col: 1,
            current_line: 1,
        }
    }

    pub fn css__lexer_get_token(&mut self) -> (css_error , Option<~css_token>){
        let mut start_again = false;

        let ret_val =
            match self.state {
                sSTART => {
                    self.start()
                },
                sATKEYWORD => {
                    self.at_keyword()
                },
                sSTRING => {
                    self.string()
                },
                sHASH => {
                    self.hash()
                },
                sNUMBER => {
                    self.number_or_percentage_or_dimension()
                },
                sCDO => {
                    self.cdo()
                },
                sCDC => {
                    self.cdc_or_ident_or_function_or_npd()
                },
                sS => {
                    self.s()
                },
                sCOMMENT => {
                    let (error, token_option) = self.comment();
                    if (!self.emit_comments && error as int == CSS_OK as int) {
                        let token = token_option.unwrap();

                        if (token.token_type as int == CSS_TOKEN_COMMENT as int) {
                            self.state = sSTART;
                            start_again = true;

                            (CSS_OK, None)
                        }
                        else {
                            (error, Some(token))
                        }
                    }
                    else {
                        (error, token_option)
                    }
                },
                sMATCH => {
                    self.match_prefix()
                },
                sURI => {
                    self.uri()
                },
                sIDENT => {
                    self.ident_or_function()
                },
                sESCAPEDIDENT => {
                    self.escaped_ident_or_function()
                },
                sURL => {
                    self.uri()
                },
                sUCR => {
                    self.unicode_range()
                }
            }; // match

        if (!start_again) { 
            return ret_val;
        }
        else { // goto start;
            return self.css__lexer_get_token();
        }
    }

    /******************************************************************************
     * Utility routines                                                           *
     ******************************************************************************/

    pub fn APPEND(&mut self, data: &[u8], len: uint) {
        self.append_to_token_data(data, len);

        self.bytes_read_for_token += len;
        self.current_col += len;
    }

    pub fn append_to_token_data(&mut self , data: &[u8], len: uint) {
        
        if self.escape_seen {
            self.unescaped_token_data.get_mut_ref().push_all(data.slice(0,len));
        }

        self.token.get_mut_ref().data.len += len;
    }

    pub fn emit_token(&mut self , input_token_type: Option<css_token_type>) -> (css_error, Option<~css_token>) {

        let mut t = self.token.swap_unwrap();
        let token_type = match (input_token_type) {
            Some(tt) => {
                t.token_type = tt;
                tt
            },
            None => {
                t.token_type
            }
        };
        

        if (self.escape_seen) {
            t.data.data = self.unescaped_token_data.swap_unwrap();
        }
        else {
            let (pu_peek_result, pu_peek_error) = self.input.parserutils_inputstream_peek(0);

            assert!((token_type as int == CSS_TOKEN_EOF as int) || 
                (pu_peek_error as int == PARSERUTILS_OK as int));

            match token_type {
                CSS_TOKEN_EOF => {
                    t.data.data = ~[];
                }
                _ => {
                    let (cptr, _) = pu_peek_result.unwrap();
                    t.data.data = cptr.slice(0, t.data.len).to_owned();
                }
            }
        }

        match token_type {
            CSS_TOKEN_ATKEYWORD => {
                /* Strip the '@' from the front */
                vec::shift(&mut t.data.data);
                t.data.len -=1;
            },
            CSS_TOKEN_STRING => {
                /* Strip the leading quote */
                vec::shift(&mut t.data.data);
                t.data.len -=1;

                /* Strip the trailing quote, iff it exists (may have hit EOF) */
                if (t.data.len > 0) {
                    let last = t.data.data.pop();

                    if (last == '"' as u8 || last =='\'' as u8) {
                        t.data.len -=1;
                    }
                    else {
                        t.data.data.push(last);
                    }
                }
            },
            CSS_TOKEN_INVALID_STRING => {
                /* Strip the leading quote */
                vec::shift(&mut t.data.data);
                t.data.len -=1;
            },
            CSS_TOKEN_HASH => {
                /* Strip the '#' from the front */
                vec::shift(&mut t.data.data);
                t.data.len -=1;
            },
            CSS_TOKEN_PERCENTAGE => {
                /* Strip the '%' from the end */
                t.data.data.pop();
                t.data.len -=1;
            },
            CSS_TOKEN_DIMENSION => {},
            CSS_TOKEN_URI => {
                /* Strip the "url(" from the start */
                t.data.data = vec::tailn(t.data.data, 4).to_owned();
                t.data.len -= 4;

                /* Strip any leading whitespace */
                /* Strip any leading quote */
                /* Strip the trailing ')' */
                /* Strip any trailing whitespace */
                /* Strip any trailing quote */
                do vec::retain(&mut t.data.data) |&c| {
                    if (c == ' ' as u8 || c == ')' as u8 || c =='"' as u8 || c == '\'' as u8) {
                        false
                    }
                    else {
                        true
                    }
                }

                t.data.len = t.data.data.len();

            },
            CSS_TOKEN_UNICODE_RANGE => {
                /* Remove "U+" from the start */
                t.data.data = vec::tailn(t.data.data, 2).to_owned();
                t.data.len -= 2;
            },
            CSS_TOKEN_COMMENT => {
                /* Strip the leading '/' and '*' */
                /* Strip the trailing '*' and '/' */

                t.data.data = t.data.data.slice(2, t.data.data.len()-2).to_owned();
                t.data.len-=4;
            },
            CSS_TOKEN_FUNCTION => {
                /* Strip the trailing '(' */
                t.data.data.pop();
                t.data.len -= 1;
            },
            _=> {

            }
        }
        self.state = sSTART;
        self.substate = 0;

        return (CSS_OK,Some(t));
    }

    /******************************************************************************
     * State machine components                                                   *
     ******************************************************************************/

    pub fn at_keyword(&mut self) -> (css_error, Option<~css_token>) {
        enum at_keyword_substates {
            Initial = 0, 
            Escape = 1, 
            NMChar = 2
        }

        /* ATKEYWORD = '@' ident 
         * 
         * The '@' has been consumed.
         */

        if (self.substate == Initial as uint) {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (!start_nm_char(c)) {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            if (c != '\\') {
                self.APPEND(cptr, clen);
                self.substate = NMChar as uint; // fall through
            } else {
                self.bytes_read_for_token += clen;
                self.substate = Escape as uint;
            }
        }

        if (self.substate == Escape as uint) {
            let error = self.consume_escape(false);
            if (error as int != CSS_OK as int) {
                if (error as int == CSS_EOF as int || error as int == CSS_INVALID as int) {
                    /* Rewind the '\\' */
                    self.bytes_read_for_token -= 1;

                    return self.emit_token(Some(CSS_TOKEN_CHAR));
                }

                return (error, None);
            }

            // goto nmchar;
            self.substate = NMChar as uint;
        }

        // goto nmchar;
        if (self.substate == NMChar as uint) {
            let error = self.consume_nm_chars();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }
        }

        self.emit_token(Some(CSS_TOKEN_ATKEYWORD))
    }


    pub fn cdc_or_ident_or_function_or_npd(&mut self) -> (css_error, Option<~css_token>) {
        
        enum CDC_or_Ident_or_function_or_NPD_substates { 
            Initial = 0, 
            Escape = 1, 
            Gt = 2 
        }

        /* CDC = "-->"
         * IDENT = [-]? nmstart nmchar*
         * FUNCTION = [-]? nmstart nmchar* '('
         * NUMBER = num = [-+]? ([0-9]+ | [0-9]* '.' [0-9]+)
         * PERCENTAGE = num '%'
         * DIMENSION = num ident
         *
         * The first dash has been consumed. Thus, we must consume the next 
         * character in the stream. If it's a dash, then we're dealing with 
         * CDC. If it's a digit or dot, then we're dealing with NPD. 
         * Otherwise, we're dealing with IDENT/FUNCTION.
         */


        if (self.substate == Initial as uint) {
            
            /* Fall through */
            self.substate = Gt as uint;

            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* We can only match char with what we've read so far */
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (!start_nm_char(c)) {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            if (char::is_digit(c) || c == '.') {
                /* NPD */
                self.APPEND(cptr, clen);
                self.state = sNUMBER;
                self.substate = 0;
                /* Abuse "first" to store first non-sign character */
                self.context.first = c as u8;
                //return NumberOrPercentageOrDimension(lexer, token);
                return self.number_or_percentage_or_dimension();
            }

            if (c != '-' && !start_nm_start(c)) {
                /* Can only be CHAR */
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }


            if (c != '\\') {
                self.APPEND(cptr, clen);
            }

            if (c != '-') {
                if (c == '\\') {
                    self.bytes_read_for_token += clen;
                    self.substate = Escape as uint; // goto Escape
                }
                else {
                    self.state = sIDENT;
                    self.substate = 0;
                    return self.ident_or_function();
                }
            }

            
        }
        
        if (self.substate == Gt as uint) {
        

            /* Ok, so we're dealing with CDC. Expect a '>' */
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                
                /* CHAR is the only match here */
                /* Remove the '-' we read above */
                self.bytes_read_for_token -= 1;
                self.token.get_mut_ref().data.len -= 1;
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == '>') {
                self.APPEND(cptr, clen);

                self.token.get_mut_ref().token_type = CSS_TOKEN_CDC;
            } else {
                /* Remove the '-' we read above */
                self.bytes_read_for_token -= 1;
                self.token.get_mut_ref().data.len -= 1;
                self.token.get_mut_ref().token_type = CSS_TOKEN_CHAR;
            }
        }

        if (self.substate == Escape as uint) {
            let error = self.consume_escape(false);
            if (error as int != CSS_OK as int) {
                if (error as int == CSS_EOF as int || error as int == CSS_INVALID as int) {
                    /* Rewind the '\\' */
                    self.bytes_read_for_token -= 1;

                    return self.emit_token(Some(CSS_TOKEN_CHAR));
                }

                return (error, None);
            }

            self.state = sIDENT;
            self.substate = 0;
            return self.ident_or_function();
        }
        
        self.emit_token(None) // == token.token_type
    }
    
    pub fn cdo(&mut self) -> (css_error, Option<~css_token>) {
        enum cdo_substates { Initial = 0, Dash1 = 1, Dash2 = 2 };

        /* CDO = "<!--"
         * 
         * The '<' has been consumed
         */

        if (self.substate == Initial as uint) {
            /* Expect '!' */
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* We can only match char with what we've read so far */
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == '!') {
                self.APPEND(cptr, clen);
            } else {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            /* Fall Through */
            self.substate = Dash1 as uint;
        }

        if (self.substate == Dash1 as uint) {
            /* Expect '-' */
            let (pu_peek_result , perror) = 
                            self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* CHAR is the only match here */
                /* Remove the '!' we read above */
                self.bytes_read_for_token -= 1;
                self.token.get_mut_ref().data.len -= 1;
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == '-') {
                self.APPEND(cptr, clen);
            } else {
                /* Remove the '!' we read above */
                self.bytes_read_for_token -= 1;
                self.token.get_mut_ref().data.len -= 1;
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            /* Fall through */
            self.substate = Dash2 as uint;           
        }

        if (self.substate == Dash2 as uint) {
            /* Expect '-' */
            let (pu_peek_result , perror) = 
                    self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* CHAR is the only match here */
                /* Remove the '-' and the '!' we read above */
                self.bytes_read_for_token -= 2;
                self.token.get_mut_ref().data.len -= 2;
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;


            if (c == '-') {
                self.APPEND(cptr, clen);
            } else {
                /* Remove the '-' and the '!' we read above */
                self.bytes_read_for_token -= 2;
                self.token.get_mut_ref().data.len -= 2;
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }
        }

        self.emit_token(Some(CSS_TOKEN_CDO))
    }

    pub fn comment(&mut self) -> (css_error, Option<~css_token>) {
        enum comment_substates { Initial = 0, InComment = 1 };

        /* COMMENT = '/' '*' [^*]* '*'+ ([^/] [^*]* '*'+)* '/'
         *
         * The '/' has been consumed.
         */

        if (self.substate == Initial as uint) {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            let (cptr , _) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c != '*') {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            /* Fall through */
            self.substate = InComment as uint;
        }

        if (self.substate == InComment as uint) {
            loop {
                let (pu_peek_result , perror) = 
                    self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

                if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                    return (css_error_from_parserutils_error(perror), None);
                }

                if (perror as int == PARSERUTILS_EOF as int) {
                    /* As per unterminated strings, 
                     * we ignore unterminated comments. */
                    return self.emit_token(Some(CSS_TOKEN_EOF));
                }

                let (cptr , clen) = pu_peek_result.unwrap();
                let c = cptr[0] as char;

                self.APPEND(cptr, clen);
                
                if (self.context.last_was_star && c == '/') {
                    break;
                }

                self.context.last_was_star = (c == '*');

                if (c == '\n' /*|| c == '\f'*/) {
                    self.current_col = 1;
                    self.current_line+=1;
                }

                if (self.context.last_was_cr && c != '\n') {
                    self.current_col = 1;
                    self.current_line+=1;
                }
                self.context.last_was_cr = (c == '\r');
            }
        }
        self.emit_token(Some(CSS_TOKEN_COMMENT))
    }

    pub fn escaped_ident_or_function(&mut self) -> (css_error, Option<~css_token>) {

    /* IDENT = ident = [-]? nmstart nmchar*
     * FUNCTION = ident '(' = [-]? nmstart nmchar* '('
     *
     * In this case, nmstart is an escape sequence and no '-' is present.
     *
     * The '\\' has been consumed.
     */

        let error = self.consume_escape(false);
        if (error as int != CSS_OK as int) {
            if (error as int == CSS_EOF as int || error as int == CSS_INVALID as int) {
                /* The '\\' is a token of its own */
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            return (error, None);
        }

        self.state = sIDENT;
        self.substate = 0;
        self.ident_or_function()
    }

    pub fn hash(&mut self) -> (css_error, Option<~css_token>) {

        /* HASH = '#' name  = '#' nmchar+ 
         *
         * The '#' has been consumed.
         */

        let error = self.consume_nm_chars();
        if (error as int != CSS_OK as int) {
            return (error, None);
        }

        /* Require at least one NMChar otherwise, we're just a raw '#' */
        if (self.bytes_read_for_token - self.context.orig_bytes > 0) {
            return self.emit_token(Some(CSS_TOKEN_HASH));
        }

        self.emit_token(Some(CSS_TOKEN_CHAR))
    }

    pub fn ident_or_function(&mut self) -> (css_error, Option<~css_token>) {

        enum ident_or_function_substates { Initial = 0, Bracket = 1 };

        /* IDENT = ident = [-]? nmstart nmchar*
         * FUNCTION = ident '(' = [-]? nmstart nmchar* '('
         *
         * The optional dash and nmstart are already consumed
         */

        if (self.substate == Initial as uint) {
            /* Consume all subsequent nmchars (if any exist) */
            let error = self.consume_nm_chars();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }

            /* Fall through */
            self.substate = Bracket as uint;
        }

        if (self.substate == Bracket as uint) {
            let (pu_peek_result , perror) = 
                    self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* IDENT, rather than CHAR */
                return self.emit_token(Some(CSS_TOKEN_IDENT));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == '(') {
                self.APPEND(cptr, clen);

                self.token.get_mut_ref().token_type = CSS_TOKEN_FUNCTION;
            } else {
                self.token.get_mut_ref().token_type = CSS_TOKEN_IDENT;
            }
        }

        self.emit_token(None)
    }

    pub fn match_prefix(&mut self) -> (css_error, Option<~css_token>) {

        /* INCLUDES       = "~="
         * DASHMATCH      = "|="
         * PREFIXMATCH    = "^="
         * SUFFIXMATCH    = "$="
         * SUBSTRINGMATCH = "*="
         *
         * The first character has been consumed.
         */

        let (pu_peek_result , perror) = 
            self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

        if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
            return (css_error_from_parserutils_error(perror), None);
        }

        if (perror as int == PARSERUTILS_EOF as int) {
            return self.emit_token(Some(CSS_TOKEN_CHAR));
        }

        let (cptr , clen) = pu_peek_result.unwrap();
        let c = cptr[0] as char;

        if (c != '=') {
            return self.emit_token(Some(CSS_TOKEN_CHAR));
        }

        self.APPEND(cptr, clen);

        let token_type = 
            match (self.context.first as char) {
                '~' => CSS_TOKEN_INCLUDES, 
                '|' => CSS_TOKEN_DASHMATCH,    
                '^' => CSS_TOKEN_PREFIXMATCH,  
                '$' => CSS_TOKEN_SUFFIXMATCH,  
                '*' => CSS_TOKEN_SUBSTRINGMATCH,   
                _ => fail!()
            };
        
        self.emit_token(Some(token_type))
    }

    pub fn number_or_percentage_or_dimension(&mut self) -> (css_error, Option<~css_token>) {

        enum number_or_percentage_or_dimension_substates { 
            Initial = 0, Dot = 1, MoreDigits = 2, Suffix = 3, NMChars = 4, Escape = 5, NMChars2 = 6 };

        /* NUMBER = num = [-+]? ([0-9]+ | [0-9]* '.' [0-9]+)
         * PERCENTAGE = num '%'
         * DIMENSION = num ident
         *
         * The sign, or sign and first digit or dot, 
         * or first digit, or '.' has been consumed.
         */

        if (self.substate == Initial as uint) {
            let error = self.consume_digits();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }

            /* Fall through */
            self.substate == Dot as uint;
        }

        if (self.substate == Dot as uint) {
            
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                let mut token_type = CSS_TOKEN_NUMBER;

                if ( self.token.get_mut_ref().data.len == 1 && 
                    (self.context.first == '.' as u8 || self.context.first == '+' as u8)
                   ) {
                    token_type = CSS_TOKEN_CHAR;
                }
                
                self.emit_token(Some(token_type));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            /* Bail if we've not got a '.' or we've seen one already */
            if c != '.' || self.context.first == '.' as u8 {
                // goto suffix
                self.substate = Suffix as uint;
            }
            else {
                /* Save the token length up to the end of the digits */
                self.context.orig_bytes = self.bytes_read_for_token;
                
                /* Append the '.' to the token */
                self.APPEND(cptr, clen);

                /* Fall through */
                self.substate = MoreDigits as uint;
            }
        }

        if (self.substate == MoreDigits as uint) {

            let error = self.consume_digits();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }

            if (self.bytes_read_for_token - self.context.orig_bytes == 1) {
                /* No digits after dot => dot isn't part of number */
                self.bytes_read_for_token -= 1;
                self.token.get_mut_ref().data.len -= 1;
            }

            /* Fall through */
            self.substate = Suffix as uint;
        }
        
        if (self.substate == Suffix as uint) {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                let mut token_type = CSS_TOKEN_NUMBER;

                if ( self.token.get_ref().data.len == 1 && 
                    (self.context.first == '.' as u8 || self.context.first == '+' as u8)
                   ) {
                    token_type = CSS_TOKEN_CHAR;
                }
                
                return self.emit_token(Some(token_type));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;
            
            /* A solitary '.' or '+' is a CHAR, not numeric */
            let mut emit_char = false;
            if ( self.token.get_ref().data.len == 1 && 
                    (self.context.first == '.' as u8 || self.context.first == '+' as u8)) {
                emit_char = true;
            }
            if (emit_char) {
                return self.emit_token(Some(CSS_TOKEN_CHAR));
            }

            if (c == '%') {
                self.APPEND(cptr, clen);
                return self.emit_token(Some(CSS_TOKEN_PERCENTAGE));
            } else if (!start_nm_start(c)) {
                return self.emit_token(Some(CSS_TOKEN_NUMBER));
            }

            if (c != '\\') {
                self.APPEND(cptr, clen);

                /* Fall through */
                self.substate = NMChars as uint;
            } else {
                self.bytes_read_for_token += clen;
                //goto escape;
                self.substate = Escape as uint;
            }

            
        }

        if (self.substate == NMChars as uint) {
            let error = self.consume_nm_chars();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }
        }
            
        if (self.substate == Escape as uint) {
            let error = self.consume_escape(false);
            if (error as int != CSS_OK as int) {
                if (error as int == CSS_EOF as int || error as int == CSS_INVALID as int) {
                    /* Rewind the '\\' */
                    self.bytes_read_for_token -= 1;

                    /* This can only be a number */
                    return self.emit_token(Some(CSS_TOKEN_NUMBER));
                }

                return (error, None);
            }
            // goto nmchars;
            self.substate = NMChars2 as uint;
        }

        if (self.substate == NMChars2 as uint) {
            let error = self.consume_nm_chars();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }
        }

        self.emit_token(Some(CSS_TOKEN_DIMENSION))
    }

    pub fn s(&mut self) -> (css_error, Option<~css_token>) {

        /* S = wc*
     * 
     * The first whitespace character has been consumed.
     */

        let error = self.consume_w_chars();
        if (error as int != CSS_OK as int) {
            return (error, None);
        }

        self.emit_token(Some(CSS_TOKEN_S))
    }

    pub fn start(&mut self) -> (css_error, Option<~css_token>) {

        loop {
            /* Advance past the input read for the previous token */
            if (self.bytes_read_for_token > 0) {
                self.input.parserutils_inputstream_advance(self.bytes_read_for_token);
                self.bytes_read_for_token = 0;
            }

            /* Reset in preparation for the next token */
            for self.token.each_mut |t| {
                t.token_type = CSS_TOKEN_EOF;
                t.data.data = ~[];
                t.data.len = 0;
                t.idata = None;
                t.col = self.current_col;
                t.line = self.current_line;
            }

            self.escape_seen = false;
            if (self.unescaped_token_data.is_some()) {
                self.unescaped_token_data = None;
            }


            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return self.emit_token(Some(CSS_TOKEN_EOF));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            self.APPEND(cptr, clen);

            if (clen > 1 || c >= 0x80 as char) {
                self.state = sIDENT;
                self.substate = 0;

                return self.ident_or_function();
            }

            match (c) {
                '@'=> {
                    self.state = sATKEYWORD;
                    self.substate = 0;
                    return self.at_keyword();
                }

                '"'|'\''=> {
                    self.state = sSTRING;
                    self.substate = 0;
                    self.context.first = c as u8;
                    return self.string();
                }
                '#' => {
                    self.state = sHASH;
                    self.substate = 0;
                    self.context.orig_bytes = self.bytes_read_for_token;
                    return self.hash();
                }
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'.'|'+' => {
                    self.state = sNUMBER;
                    self.substate = 0;
                    self.context.first = c as u8;
                    return self.number_or_percentage_or_dimension();
                }
                '<'=> {
                    self.state = sCDO;
                    self.substate = 0;
                    return self.cdo();
                }
                '-'=> {
                    self.state = sCDC;
                    self.substate = 0;
                    return self.cdc_or_ident_or_function_or_npd();
                }
                ' '|'\t'| '\r'| '\n' => {
                    self.state = sS;
                    self.substate = 0;
                    if (c=='\n') {
                        self.current_col = 1;
                        self.current_line += 1;
                    }
                    self.context.last_was_cr = (c == '\r');
                    return self.s();
                }
                '/' => {
                    self.state = sCOMMENT;
                    self.substate = 0;
                    self.context.last_was_star = false;
                    self.context.last_was_cr = false;
                    let (error, token_option) = self.comment();
                    if (!self.emit_comments && error as int == CSS_OK as int) {
                        let token = token_option.unwrap();

                        if (token.token_type as int == CSS_TOKEN_COMMENT as int) {
                            //goto start;
                            loop;
                        }
                        else {
                            return (error, Some(token))
                        }
                    }
                    else {
                        return (error, token_option);
                    }
                }
                '~'|'|'|'^'|'$'|'*' => {
                    self.state = sMATCH;
                    self.substate = 0;
                    self.context.first = c as u8;
                    return self.match_prefix();
                }
                'u'|'U' => {
                    self.state = sURI;
                    self.substate = 0;
                    return self.uri_or_unicode_range_or_ident_or_function();
                }
                'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|
                'n'|'o'|'p'|'q'|'r'|'s'|'t'/*'u'*/ |'v'|'w'|'x'|'y'|
                'z'|'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|
                'M'|'N'|'O'|'P'|'Q'|'R'|'S'|'T'|/*'U'*/ 'V'|'W'|'X'|
                'Y'|'Z'|'_' => {
                    self.state = sIDENT;
                    self.substate = 0;
                    return self.ident_or_function();
                }
                '\\'=> {
                    self.state = sESCAPEDIDENT;
                    self.substate = 0;
                    return self.escaped_ident_or_function();
                }
                _=> {
                    return self.emit_token(Some(CSS_TOKEN_CHAR));
                }
            } // match (c)
        } // loop
    }

    pub fn string(&mut self) -> (css_error, Option<~css_token>) {

        /* STRING = string
         *
         * The open quote has been consumed.
         */

        let error = self.consume_string();
        if (error as int != CSS_OK as int && error as int != CSS_EOF as int && error as int != CSS_INVALID as int) {
            return (error, None);
        }

        /* EOF will be reprocessed in Start() */
        match error {
            CSS_OK | CSS_EOF => {
                self.emit_token(Some(CSS_TOKEN_STRING))
            }
            CSS_INVALID => {
                self.emit_token(Some(CSS_TOKEN_INVALID_STRING))
            }
            _ => {fail!()}
        }
    }

    pub fn uri_or_unicode_range_or_ident_or_function(&mut self) -> (css_error, Option<~css_token>) {
        /* URI = "url(" w (string | urlchar*) w ')' 
         * UNICODE-RANGE = [Uu] '+' [0-9a-fA-F?]{1,6}(-[0-9a-fA-F]{1,6})?
         * IDENT = ident = [-]? nmstart nmchar*
         * FUNCTION = ident '(' = [-]? nmstart nmchar* '('
         *
         * The 'u' (or 'U') has been consumed.
         */

        let (pu_peek_result , perror) = 
                        self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

        if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
            return (css_error_from_parserutils_error(perror), None);
        }

        if (perror as int == PARSERUTILS_EOF as int) {
            /* IDENT, rather than CHAR */
            return self.emit_token(Some(CSS_TOKEN_IDENT));
        }

        let (cptr , clen) = pu_peek_result.unwrap();
        let c = cptr[0] as char;

        if (c == 'r' || c == 'R') {
            self.APPEND(cptr, clen);

            self.state = sURL;
            self.substate = 0;
            return self.uri();
        } else if (c == '+') {
            self.APPEND(cptr, clen);

            self.state = sUCR;
            self.substate = 0;
            self.context.hex_count = 0;
            return self.unicode_range();
        }

        /* Can only be IDENT or FUNCTION. Reprocess current character */
        self.state = sIDENT;
        self.substate = 0;
        
        self.ident_or_function()
    }
    
    pub fn uri(&mut self) -> (css_error, Option<~css_token>) {
        enum uri_substates { Initial = 0, LParen = 1, W1 = 2, Quote = 3, 
        URL = 4, W2 = 5, RParen = 6, String = 7 };

        /* URI = "url(" w (string | urlchar*) w ')' 
         *
         * 'u' and 'r' have been consumed.
         */

        if (self.substate == Initial as uint) {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* IDENT */
                return self.emit_token(Some(CSS_TOKEN_IDENT));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == 'l' || c == 'L') {
                self.APPEND(cptr, clen);
            }
            else {
                self.state = sIDENT;
                self.substate = 0;

                return self.ident_or_function();
            }

            /* Fall through */
            self.substate = LParen as uint;
        }

        if (self.substate == LParen as uint) {

            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return self.emit_token(Some(CSS_TOKEN_IDENT));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == '(') {
                self.APPEND(cptr, clen);
            }
            else {
                self.state = sIDENT;
                self.substate = 0;

                return self.ident_or_function();
            }

            /* Save the number of input bytes read for "url(" */
            self.context.bytes_for_url = self.bytes_read_for_token;
            /* And the length of the token data at the same point */
            self.context.data_len_for_url = { self.token.get_ref().data.len };

            self.context.last_was_cr = false;

            /* Fall through */
            self.substate = W1 as uint;
        }

        if (self.substate == W1 as uint) {
            let error = self.consume_w_chars();

            if (error as int != CSS_OK as int) {
                return (error, None);
            }

            /* Fall through */
            self.substate = Quote as uint;
        }

        if (self.substate == Quote as uint) {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* Rewind to "url(" */
                self.bytes_read_for_token = self.context.bytes_for_url;
                {self.token.get_mut_ref().data.len = self.context.data_len_for_url;}
                return self.emit_token(Some(CSS_TOKEN_FUNCTION));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == '"' || c == '\'') {
                self.APPEND(cptr, clen);
                self.context.first = c as u8;

                // goto string;
                self.substate == String as uint;
            }

            /* Potential minor optimisation: If string is more common, 
             * then fall through to that state and branch for the URL 
             * state. Need to investigate a reasonably large corpus of 
             * real-world data to determine if this is worthwhile. */

            /* Fall through */
            self.substate = URL as uint;
        }

        /* re-ordered states to avoid goto */
        if (self.substate == String as uint) {
            let error = self.consume_string();
            if (error as int == CSS_INVALID as int) {
                /* Rewind to "url(" */
                self.bytes_read_for_token = self.context.bytes_for_url;
                {self.token.get_mut_ref().data.len = self.context.data_len_for_url;}
                return self.emit_token(Some(CSS_TOKEN_FUNCTION));
            } 
            else if (error as int != CSS_OK as int && error as int != CSS_EOF as int) {
                return (error, None);
            }
        
            /* EOF gets handled in RParen */
            self.context.last_was_cr = false;

            //goto w2;
            self.substate = W2 as uint;
        }

        if (self.substate == URL as uint) {
            let error = self.consume_url_chars();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }

            self.context.last_was_cr = false;
            
            /* Fall through */
            self.substate = W2 as uint;
        }

        if (self.substate == W2 as uint) {
            let error = self.consume_w_chars();
            if (error as int != CSS_OK as int) {
                return (error, None);
            }

            /* Fall through */
            self.substate = RParen as uint;
        }

        if (self.substate == RParen as uint) {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return (css_error_from_parserutils_error(perror), None);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                /* Rewind to "url(" */
                self.bytes_read_for_token = self.context.bytes_for_url;
                {self.token.get_mut_ref().data.len = self.context.data_len_for_url;}
                return self.emit_token(Some(CSS_TOKEN_FUNCTION));
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c != ')') {
                /* Rewind to "url(" */
                self.bytes_read_for_token = self.context.bytes_for_url;
                {self.token.get_mut_ref().data.len = self.context.data_len_for_url;}
                return self.emit_token(Some(CSS_TOKEN_FUNCTION));
            }

            self.APPEND(cptr, clen);
            // break;
        }

        self.emit_token(Some(CSS_TOKEN_URI))
    }

    pub fn unicode_range(&mut self) -> (css_error, Option<~css_token>) {
        // TODO: Abhijeet
        (CSS_OK, None)
    }

    /******************************************************************************
     * Input consumers                                                            *
     ******************************************************************************/ 

    pub fn consume_digits(&mut self) -> css_error {
        /* digit = [0-9] */

        /* Consume all digits */
        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (char::is_digit(c)) {
                self.APPEND(cptr, clen);
            }
            else {
                break;
            }
        }

        return CSS_OK;
    }

    fn consume_escape(&mut self, nl : bool) -> css_error {

        /* escape = unicode | '\' [^\n\r\f0-9a-fA-F] 
             * 
             * The '\' has been consumed.
             */
        
        let (pu_peek_result , perror) = 
            self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
        
        match perror {
        
            PARSERUTILS_NOMEM | 
            PARSERUTILS_BADPARM | 
            PARSERUTILS_INVALID | 
            PARSERUTILS_FILENOTFOUND | 
            PARSERUTILS_NEEDDATA | 
            PARSERUTILS_BADENCODING => {
                return css_error_from_parserutils_error(perror);
            }
            PARSERUTILS_EOF => {
               return CSS_EOF;
            }
            _ => {
                /* continue */
            }
        }

        let (cptr , clen) = pu_peek_result.unwrap();
        let mut c = cptr[0] as char;

        if (!nl && (c=='\n' || c=='\r' /* || c=='\f'*/)) {
            /* These are not permitted */
            return CSS_INVALID;
        }

        /* Create unescaped buffer, if it doesn't already exist */
        if (self.unescaped_token_data.is_none()) {
            self.unescaped_token_data = Some(~[]);
        }

        /* If this is the first escaped character we've seen for this token,
         * we must copy the characters we've read to the unescaped buffer */
        if (!self.escape_seen) {
            if (self.bytes_read_for_token > 1) {
                let (pu_peek_result , perror) = 
                    self.input.parserutils_inputstream_peek(0);

                assert!(perror as int == PARSERUTILS_OK as int);

                let (sdata , _) = pu_peek_result.unwrap();
                /* -1 to skip '\\' */
                self.unescaped_token_data.get_mut_ref().push_all(sdata.slice(0, self.bytes_read_for_token-1));
            }

            self.token.get_mut_ref().data.len = self.bytes_read_for_token-1;
            self.escape_seen = true;
        }

        if (char::is_digit_radix(c,16)) {
            self.bytes_read_for_token += clen;

            match (self.consume_unicode(char::to_digit(c, 16).unwrap() as u32)) {
                CSS_OK => {
                    /* continue */
                }
                x => {
                    self.bytes_read_for_token -= clen;
                    return x;
                }
            }
        }

        /* If we're handling escaped newlines, convert CR(LF)? to LF */
        if (nl && c=='\r') {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token+clen);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                c='\n';
                self.APPEND(&[c as u8], 1);

                self.current_col = 1;
                self.current_line += 1;

                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();

            c=cptr[0] as char;

            if (c=='\n') {
                self.APPEND(cptr, 1);
                /* And skip the '\r' in the input */
                self.bytes_read_for_token += clen;

                self.current_col = 1;
                self.current_line += 1;

                return CSS_OK;
            }
            
            self.APPEND(cptr, clen); // cptr has been redefined above
            return CSS_OK;
        }
        else if (nl && (c == '\n'/* || c == '\f'*/)) {
            /* APPEND will increment this appropriately */
            self.current_col = 0;
            self.current_line+=1;
        }
        else if (c != '\n' && c != '\r' /*&& c != '\f'*/) {
            self.current_col+=1;
        }

        /* Append the unescaped character */
        self.APPEND(cptr, clen);

        CSS_OK
    }

    pub fn consume_nm_chars(&mut self) -> css_error
    {
        /* nmchar = [a-zA-Z] | '-' | '_' | nonascii | escape */

        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (start_nm_char(c) && c != '\\') {
                self.APPEND(cptr, clen);
            }

            if (c == '\\') {
                self.bytes_read_for_token += clen;

                let error = self.consume_escape(false);
                if (error as int != CSS_OK as int) {
                    /* Rewind '\\', so we do the 
                     * right thing next time */
                    self.bytes_read_for_token -= clen;

                    /* Convert either EOF or INVALID into OK.
                     * This will cause the caller to believe that
                     * all NMChars in the sequence have been 
                     * processed (and thus proceed to the next
                     * state). Eventually, the '\\' will be output
                     * as a CHAR. */
                    if (error as int == CSS_EOF as int || error as int == CSS_INVALID as int) {
                        return CSS_OK;
                    }

                    return error;
                }
            }

            if (!start_nm_char(c)) {
                break;
            }
        }

        return CSS_OK;
    }

    pub fn consume_string(&mut self) -> css_error
    {
        
        let quote = self.context.first as char;
        let permittedquote = 
            match(quote) {
                '"' => '\'',
                _ => '"'
            };

        /* string = '"' (stringchar | "'")* '"' | "'" (stringchar | '"')* "'"
         *
         * The open quote has been consumed.
         */

        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (c == permittedquote) {
                self.APPEND(cptr, clen);
            } else if (start_string_char(c)) {
                let error = self.consume_string_chars();
                if (error as int!= CSS_OK as int) {
                    return error;
                }
            } else if (c != quote) {
                /* Invalid character in string */
                return CSS_INVALID;
            }

            if (c == quote) {
                /* Append closing quote to token data */
                self.APPEND(cptr, clen);
                break;
            }
        }

        return CSS_OK;
    }

    pub fn consume_string_chars(&mut self) -> css_error
    {
        /* stringchar = urlchar | ' ' | ')' | '\' nl */

        loop {
            let (pu_peek_result , perror) = 
                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);

            if (perror as int != PARSERUTILS_OK as int && perror as int != PARSERUTILS_EOF as int) {
                return css_error_from_parserutils_error(perror);
            }

            if (perror as int == PARSERUTILS_EOF as int) {
                return CSS_OK;
            }

            let (cptr , clen) = pu_peek_result.unwrap();
            let c = cptr[0] as char;

            if (start_string_char(c) && c != '\\') {
                self.APPEND(cptr, clen);
            }

            if (c == '\\') {
                self.bytes_read_for_token += clen;

                let error = self.consume_escape(true);
                if (error as int != CSS_OK as int) {
                    /* Rewind '\\', so we do the 
                     * right thing next time. */
                    self.bytes_read_for_token -= clen;

                    /* Convert EOF to OK. This causes the caller
                     * to believe that all StringChars have been
                     * processed. Eventually, the '\\' will be
                     * output as a CHAR. */
                    if (error as int == CSS_EOF as int) {
                        return CSS_OK;
                    }

                    return error;
                }
            }

            if (!start_string_char(c)) {
                break;
            }
        }

        return CSS_OK;
    }

    fn consume_unicode(&mut self, mut ucs : u32) -> css_error {
        let cptr : @mut u8;
        let mut count : int = 0;
        let mut bytes_read_init : uint = self.bytes_read_for_token;

        while (count < 5) {
           let (pu_peek_result , error) = 
            self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
        
                PARSERUTILS_NOMEM | 
                PARSERUTILS_BADPARM | 
                PARSERUTILS_INVALID | 
                PARSERUTILS_FILENOTFOUND | 
                PARSERUTILS_NEEDDATA | 
                PARSERUTILS_BADENCODING => {
                    self.bytes_read_for_token = bytes_read_init;
                    return css_error_from_parserutils_error(error);
                }
                PARSERUTILS_EOF => {
                   break;
                }
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    if char::is_digit_radix(_cptr[0] as char, 16){
                        self.bytes_read_for_token += clen;
                        ucs = (ucs << 4) | u32::from_str_radix(str::from_char(_cptr[0] as char), 16).unwrap();
                    }
                    else{
                        break;
                    }
                }

            }

            count += 1;
        }

        if (ucs > 0x10FFFF || ucs <= 0x0008 || ucs == 0x000B ||
                (0x000E <= ucs && ucs <= 0x001F) ||
                (0x007F <= ucs && ucs <= 0x009F) ||
                (0xD800 <= ucs && ucs <= 0xDFFF) ||
                (0xFDD0 <= ucs && ucs <= 0xFDEF) ||
                (ucs & 0xFFFE) == 0xFFFE) {
            ucs = 0xFFFD;
        } else if (ucs == 0x000D) {
            ucs = 0x000A;
        }

        let (utf8sequence_option, pu_charset_error) = parserutils_charset_utf8_from_ucs4(ucs);
        match (pu_charset_error) {
            PARSERUTILS_OK => {
                let (pu_peek_result , error) = 
                 self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
                match error {
                
                    PARSERUTILS_NOMEM | 
                    PARSERUTILS_BADPARM | 
                    PARSERUTILS_INVALID | 
                    PARSERUTILS_FILENOTFOUND | 
                    PARSERUTILS_NEEDDATA | 
                    PARSERUTILS_BADENCODING => {
                        self.bytes_read_for_token = bytes_read_init;
                        return css_error_from_parserutils_error(error);
                    }
                    PARSERUTILS_EOF => {
                       return CSS_OK;
                    }
                    PARSERUTILS_OK => {
                        let mut (_cptr , clen) = pu_peek_result.unwrap();
                        if (_cptr[0] as char == '\r') { // Potential CRLF 
                            //let mut p_cr : u8 = _cptr[0];
                            let (pu_peek_result2 , error2) = 
                                self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
                            self.bytes_read_for_token = bytes_read_init;

                            match error2 {
                           
                                PARSERUTILS_NOMEM | 
                                PARSERUTILS_BADPARM | 
                                PARSERUTILS_INVALID | 
                                PARSERUTILS_FILENOTFOUND | 
                                PARSERUTILS_NEEDDATA | 
                                PARSERUTILS_BADENCODING => {
                                    self.bytes_read_for_token = bytes_read_init;
                                    return css_error_from_parserutils_error(error2);
                                }
                                PARSERUTILS_EOF => {
                                   return CSS_OK;
                                }
                                PARSERUTILS_OK => {
                                    let (_cptr2 , clen2) = pu_peek_result2.unwrap();
                                    if (_cptr2[0] as char == '\n') { // Potential CRLF 
                                        self.bytes_read_for_token += 1;
                                        _cptr = _cptr2;
                                    }
                                }
                            }
                        }
                        let mut utf8sequence = utf8sequence_option.unwrap();
                        self.append_to_token_data(utf8sequence, utf8sequence.len());
                        if (is_space(_cptr[0] as char)) {
                           self.bytes_read_for_token += clen;
                        }

                        if _cptr[0] as char=='\r' || _cptr[0] as char == '\n' /*|| _cptr[0] == '\f'*/ {
                           self.current_col = 1;
                           self.current_line += 1;
                        }
                        else {
                           self.current_col += self.bytes_read_for_token - bytes_read_init + 2;
                        }
                    }

                } 
            }
            _ => {
                return css_error_from_parserutils_error(pu_charset_error);
            }
        }

        CSS_OK
    }

    pub fn consume_url_chars(&mut self) -> css_error {
        loop {
            let (pu_peek_result , error) = self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    let c = _cptr[0] as char;
                    
                    if start_url_char(c) && c != '\\' {
                        self.append_to_token_data(_cptr, clen);
                        self.bytes_read_for_token += clen;
                        self.current_col += clen;
                    }

                    if c == ('\\') {
                        self.bytes_read_for_token += clen;
                        let lex_error = self.consume_escape(false);

                        match lex_error {
                            CSS_OK => {},
                            _ => {
                                self.bytes_read_for_token -= clen;
                                return CSS_OK;
                            }
                        }
                    }

                    if !start_url_char(c) {
                        break;
                    }
                }

                _ => {
                    return CSS_INVALID;
                },

            }

            
        }

        return CSS_OK
    } 

    pub fn consume_w_chars(&mut self) -> css_error {
        loop {
            let (pu_peek_result , error) = self.input.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    let c = _cptr[0] as char;
                    
                    if is_space(c) {
                        self.append_to_token_data(_cptr, clen);
                        self.bytes_read_for_token += clen;
                        self.current_col += clen;
                    }

                    if c == ('\n'){
                        self.current_col = 1;
                        self.current_line += 1;
                    }

                    if (self.context.last_was_cr && c != ('\n')) {
                        self.current_col = 1;
                        self.current_line += 1;
                    }

                    self.context.last_was_cr = (c == '\r');

                    if !is_space(c) {
                        break;
                    }
                },

                _ => {
                    return CSS_INVALID;
                }

            }

        }

        if self.context.last_was_cr {
            self.current_col = 1;
            self.current_line += 1;
        }

        CSS_OK
    }

} // impl css_lexer




fn start_nm_char(c: char) -> bool{
    return c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || 
        ('0' <= c && c <= '9') || c == '-' || c >= 0x80 as char || c == '\\';
}

fn start_nm_start(c: char) -> bool{
    return c == '_' || ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') ||
        c >= 0x80 as char || c == '\\';
}

fn start_string_char(c: char) -> bool{
    return start_url_char(c) || c == ' ' || c == ')';
}

fn start_url_char(c: char) -> bool{
    return c == '\t' || c == '!' || ('#' <= c && c <= '&') || c == '(' ||
        ('*' <= c && c <= '~') || c >= 0x80 as char || c == '\\';
}

fn is_space(c: char) -> bool{
    return c == ' ' || c == '\r' || c == '\n' || c == '\t';
}


fn main() {
    io::println("lexer");
}