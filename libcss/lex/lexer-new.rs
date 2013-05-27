extern mod std;
extern mod wapcaplet;
extern mod parserutils;

use std::arc;
use wapcaplet::*;
use parserutils::input::inputstream::*;
use parserutils::utils::error::*;


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

pub struct css_token {
    data: ~[u8],
    token_type: css_token_type,
    idata: Option<arc::RWARC<~lwc_string>>,
}

pub enum lexer_error {
    LEXER_OK = 0,
    LEXER_NEEDDATA = 1,
    LEXER_INVALID = 2
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

pub struct _context {
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
    inputstream_instance: ~inputstream,
    bytes_read_for_token: uint,
    token: @mut css_token,
    escape_seen: bool,
    unescaped_token_data: ~[u8],  // used if eascapeSeen  = true
    state: states,
    substate: uint,
    context: _context,
    emit_comments: bool,
    current_col: u32,
    current_line: u32
}

// pub fn preprocess(input: &str) -> ~str {
//  str::replace(str::replace(str::replace(input,
//  "\r\n", "\n"),
//  "\r", "\n"),
//  "\x00", "\uFFFD")
// }

impl css_lexer {
    pub fn css__lexer_create(inputstream: ~inputstream) -> ~css_lexer {
        let _token = @mut css_token {
            data: ~[],
            token_type: CSS_TOKEN_EOF,
            idata: None
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
            inputstream_instance: inputstream,
            bytes_read_for_token: 0,
            token: _token,
            escape_seen: false,
            unescaped_token_data: ~[],
            state: sSTART,
            substate: 0,
            emit_comments: false,
            context: context_inst,      
            current_col: 1,
            current_line: 1,
        }
    }

    pub fn css__lexer_get_token(&mut self) -> (lexer_error , Option<css_token>){

        match self.state {
            sSTART => {},
            sATKEYWORD => {},
            sSTRING => {},
            sHASH => {},
            sNUMBER => {},
            sCDO => {},
            sCDC => {},
            sS => {},
            sCOMMENT => {},
            sMATCH => {},
            sURI => {},
            sIDENT => {},
            sESCAPEDIDENT => {},
            sURL => {},
            sUCR => {}
        }
        return (LEXER_INVALID , None);
    }

    pub fn append_to_token_data(&mut self , data: ~[u8]) {
        let mut token = self.token;
        if self.escape_seen {
            self.unescaped_token_data.push_all(data);
        }
    }

    pub fn emit_token(&mut self , token_type: css_token_type) -> lexer_error {

        match token_type {
            CSS_TOKEN_ATKEYWORD => {},
            CSS_TOKEN_STRING => {},
            CSS_TOKEN_INVALID_STRING => {},
            CSS_TOKEN_HASH => {},
            CSS_TOKEN_PERCENTAGE => {},
            CSS_TOKEN_DIMENSION => {},
            CSS_TOKEN_URI => {},
            CSS_TOKEN_UNICODE_RANGE => {},
            CSS_TOKEN_COMMENT => {},
            CSS_TOKEN_FUNCTION => {},
            _ => {}
        }
        self.state = sSTART;
        self.substate = 0;

        return LEXER_OK;
    }

    pub fn consume_escape(&mut self , nl: bool) -> lexer_error {
        LEXER_OK
    }

    pub fn consume_url_chars(&mut self) -> lexer_error {
        let mut cptr: ~[u8];
        let mut c: u8 = 0;

        loop {
            let (pu_peek_result , error) = self.inputstream_instance.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    c = _cptr[0];
                    
                    if start_url_char(c) && c != '\\' as u8 {
                        self.append_to_token_data(_cptr);
                        self.bytes_read_for_token += clen;
                        self.current_col += clen as u32;
                    }

                    if c == ('\\' as u8) {
                        self.bytes_read_for_token += clen;
                        let lex_error = self.consume_escape(false);

                        match lex_error {
                            LEXER_OK => {},
                            _ => {
                                self.bytes_read_for_token -= clen;
                                return LEXER_OK;
                            }
                        }
                    }
                }

                _ => {
                    return LEXER_INVALID;
                },

            }

            if !start_url_char(c) {
                break;
            }
        }

        return LEXER_OK
    } 

    pub fn consume_w_chars(&mut self) -> lexer_error {
        let mut cptr: ~[u8];
        let mut c: u8 = 0;

        loop {
            let (pu_peek_result , error) = self.inputstream_instance.parserutils_inputstream_peek(self.bytes_read_for_token);
            match error {
                

                PARSERUTILS_OK => {
                    let (_cptr , clen) = pu_peek_result.unwrap();
                    c = _cptr[0];
                    
                    if is_space(c) {
                        self.append_to_token_data(_cptr);
                        self.bytes_read_for_token += clen;
                        self.current_col += clen as u32;
                    }

                    if c == ('\n' as u8){
                        self.current_col = 1;
                        self.current_line += 1;
                    }

                    if (self.context.last_was_cr && c != ('\n' as u8)) {
                        self.current_col = 1;
                        self.current_line += 1;
                    }

                    self.context.last_was_cr = (c == '\r' as u8);
                },

                _ => {
                    return LEXER_INVALID;
                }

            }

            if !is_space(c) {
                break;
            }
        }

        if self.context.last_was_cr {
            self.current_col = 1;
            self.current_line += 1;
        }

        LEXER_OK
    }

}



fn start_nm_char(c: u8) -> bool{
    return c == '_' as u8 || ('a' as u8 <= c && c <= 'z' as u8) || ('A' as u8 <= c && c <= 'Z' as u8) || 
        ('0' as u8 <= c && c <= '9' as u8) || c == '-' as u8 || c >= 0x80 as u8 || c == '\\' as u8;
}

fn start_nm_start(c: u8) -> bool{
    return c == '_' as u8 || ('a' as u8 <= c && c <= 'z' as u8) || ('A' as u8 <= c && c <= 'Z' as u8) ||
        c >= 0x80 || c == '\\' as u8;
}

fn start_string_char(c: u8) -> bool{
    return start_url_char(c) || c == ' ' as u8 || c == ')' as u8;
}

fn start_url_char(c: u8) -> bool{
    return c == '\t' as u8 || c == '!' as u8 || ('#' as u8 <= c && c <= '&' as u8) || c == '(' as u8 ||
        ('*' as u8 <= c && c <= '~' as u8) || c >= 0x80 || c == '\\' as u8;
}

fn is_space(c: u8) -> bool{
    return c == ' ' as u8 || c == '\r' as u8 || c == '\n' as u8 || c == '\t' as u8;
}


fn main() {
    io::println("lexer");
}