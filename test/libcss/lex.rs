
extern mod std;
extern mod parserutils;
extern mod css;
extern mod extra;


use std::{io,str};

use parserutils::input::inputstream::*;
use css::utils::errors::*;

use css::charset::csdetect::*;
use css::lex::lexer::*;

fn main() {
    debug!("lex");
    lex(~"data/lex/tests1.dat");
}

fn lex(fileName: ~str) {

    let r: @Reader = io::file_reader(&Path(fileName)).unwrap();
    let mut dataFlag = false;
    let mut expectedFlag = false;
    let mut resetFlag = false;
    let mut finalstr: ~str = ~"";
    let mut expectedstr: ~str = ~"";
    let mut final_buf: ~[u8] = ~[];

    while !r.eof() {
        let (inputStreamOption, _)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT as int), Some(@css__charset_extract));
        let inputstream =
            match(inputStreamOption) {
                Some(x) => x,
                None => {
                    debug!("InputStream is not created, hence lexer can't be initialised");
                    fail!(~"inputstream is None");
                }
            };
        let mut lexer = css_lexer::css__lexer_create(inputstream);    
        let buf = r.read_line();

        if buf == ~"#data" {
            // debug!(buf);
            dataFlag = true;
            expectedFlag = false;
            resetFlag = false;
        }
        else if buf == ~"#errors" || buf == ~"" {
            dataFlag = false;
            expectedFlag = false;
            resetFlag = false;
        }
        else if buf == ~"#expected" {
            expectedFlag = true;
            dataFlag = false;
            resetFlag = false;

        }
        else if buf == ~"#reset" {
            dataFlag = false;
            expectedFlag = false;
            resetFlag = true;
        }
        else if dataFlag {
            // debug!(buf);
            finalstr.push_str(buf);
            finalstr.push_char('\n');
            // debug!(finalstr);
        }
        else if expectedFlag {
            expectedstr.push_str(buf);
        }

        if resetFlag && !dataFlag && !expectedFlag {
            let mut z = 0 ;
            let z_len = finalstr.len() ;
            while z<z_len {
                final_buf.push(finalstr[z]);
                z += 1;
            }
            finalstr = ~"";
            
            lexer.css__lexer_append_data(final_buf);
            final_buf = ~[];
            // debug!(fmt!("final_buf is %s",str::from_bytes(final_buf)));
            loop {
                let (error, token_option) = lexer.css__lexer_get_token();
                match error {
                    CSS_OK => {
                        let token = token_option.unwrap();
                        // debug!(fmt!("token == %?", token));

                        let token_type_string = token_to_string(token.token_type);
                        let token_data = str::from_bytes(token.data.data.clone());

                        let found = fmt!("%s%s" , token_type_string , token_data);
                        debug!(fmt!("found == %?", found));
                        // debug!(fmt!("Expected token == %?", (exp[index])));
                        match token_type_string {
                            ~"EOF" => break,
                            _=>{}
                        }

                    },
                    _=>{
                        debug!(fmt!("error = %?", error));
                        if token_option.is_some() {
                            
                            let token = token_option.unwrap();
                            // debug!(fmt!("token == %?", token));

                            let token_type_string = token_to_string(token.token_type);
                            let token_data = str::from_bytes(token.data.data.clone());

                            let found = fmt!("%s%s" , token_type_string , token_data);

                            debug!(fmt!("found == %?", found));
                            match token_type_string {
                                ~"EOF" => break,
                                _=>{}
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

fn token_to_string(token:css_token_type)-> ~str {
    let mut returnString =~"";
    match token {
        CSS_TOKEN_IDENT=>{
            returnString = returnString + "IDENT:";
        },
        CSS_TOKEN_ATKEYWORD=>{
            returnString = returnString + "ATKEYWORD:";
        },
        CSS_TOKEN_HASH=>{
            returnString = returnString + "HASH:";
        },
        CSS_TOKEN_FUNCTION=>{
            returnString = returnString + "FUNCTION:";
        }, 
        CSS_TOKEN_STRING=>{
            returnString = returnString + "STRING:";
        }, 
        CSS_TOKEN_INVALID_STRING=>{
            returnString = returnString + "INVALID_STRING";
        }, 
        CSS_TOKEN_URI=>{
            returnString = returnString + "URI:";
        }, 
        CSS_TOKEN_UNICODE_RANGE=>{
            returnString = returnString + "UNICODE_RANGE: ";
        }, 
        CSS_TOKEN_CHAR=>{
            returnString = returnString + "CHAR:";
        },
        CSS_TOKEN_NUMBER=>{
            returnString = returnString + "NUMBER:";
        }, 
        CSS_TOKEN_PERCENTAGE=>{
            returnString = returnString + "PERCENTAGE:";
        }, 
        CSS_TOKEN_DIMENSION=>{
            returnString = returnString + "DIMENSION:";
        },
        CSS_TOKEN_CDO=>{
            returnString = returnString + "CDO";
        }, 
        CSS_TOKEN_CDC=>{
            returnString = returnString + "CDC";
        }, 
        CSS_TOKEN_S=>{
            returnString = returnString + "S";
        },
        CSS_TOKEN_COMMENT => {
            returnString = returnString + "COMMENT";
        },
        CSS_TOKEN_INCLUDES => {
            returnString = returnString + "INCLUDES";
        },
        CSS_TOKEN_DASHMATCH => {
            returnString = returnString + "DASHMATCH";
        },
        CSS_TOKEN_PREFIXMATCH => {
            returnString = returnString + "PREFIXMATCH";
        },
        CSS_TOKEN_SUFFIXMATCH => {
            returnString = returnString + "SUFFIXMATCH";
        },
        CSS_TOKEN_SUBSTRINGMATCH => {
            returnString = returnString + "SUBSTRINGMATCH";
        }
        CSS_TOKEN_EOF =>{
            returnString = returnString + "EOF";
        }
    }
    return returnString;   
}

#[test]
fn tests1() {
    lex(~"data/lex/tests1.dat");
}

#[test]
fn tests2() {
    lex(~"data/lex/tests2.dat");
}

#[test]
fn regression() {
    lex(~"data/lex/regression.dat");
}