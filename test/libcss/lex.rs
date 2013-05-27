extern mod std;
extern mod parserutils;
extern mod css;

use parserutils::charset::aliases::*;
use parserutils::input::inputstream::*;
use parserutils::utils::error::*;

use css::charset::csdetect::*;
use css::lex::lexer::*;
use core::str::*;

fn main() {
    io::println("lex");
    lex(~"data/lex/tests1.dat");
}

fn lex(fileName: ~str) {

    let r: @Reader = io::file_reader(&Path(fileName)).get();
    let mut dataFlag = false;
    let mut expectedFlag = false;
    let mut resetFlag = false;
    let mut finalstr: ~str = ~"";
    let mut expectedstr: ~str = ~"";
    let mut final_buf: ~[u8] = ~[];

    while !r.eof() {
        let (inputStreamOption, ParserUtilsError)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT as int), Some(~css__charset_extract));
        match(ParserUtilsError) {
            PARSERUTILS_OK=>{}
            _ => {assert!(false);} // when inputstream is not created
        }
        let inputstream =
            match(inputStreamOption) {
                Some(x) => x,
                None => {
                    // io::println("InputStream is not created, hence lexer can't be initialised");
                    fail!(~"inputstream is None");
                }
            };

        let mut lexer = css_lexer::css__lexer_create(inputstream);
        let buf = r.read_line();

        if buf == ~"#data" {
            // io::println(buf);
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
            // io::println(buf);
            finalstr.push_str(buf);
            // finalstr.push_char('\n');
            // io::println(finalstr);
        }
        else if expectedFlag {
            expectedstr.push_str(buf);


        }

        if resetFlag && !dataFlag && !expectedFlag {
            for str::each_char(finalstr) |i| {
                final_buf.push(i as u8);
            }
            finalstr = ~"";
            lexer.lexer_append_data(final_buf);
            lexer.data_done();
            final_buf = ~[];
            // io::println(fmt!("final_buf is %s",str::from_bytes(final_buf)));
            let mut tok:css_token_type;
            loop {
                let mut (tokOption, STATUS) = lexer.get_token();
                match STATUS {
                    LEXER_NEEDDATA => {
                        if tokOption.is_some() {
                            tok = tokOption.unwrap();
                        }
                        break
                    },
                    _ => {}
                }
                //failing at .5%
                tok = match tokOption {
                    Some(token) => token,
                    None => {
                        break
                    }
                };
                io::println(fmt!("token is %?" , tokenToString(copy tok)));
                match tok {
                    CSS_TOKEN_EOF => break,
                    _ => {}
                }
            }
        }
    }
}

fn tokenToString(token:css_token_type)-> ~str {
    let mut returnString =~"";
    match token {
        CSS_TOKEN_IDENT(x)=>{
            returnString += ~"IDENT " + x;
        },
        CSS_TOKEN_ATKEYWORD(x)=>{
            returnString += ~"ATKEYWORD " + x;
        },
        CSS_TOKEN_HASH(x)=>{
            returnString += ~"HASH " + x;
        },
        CSS_TOKEN_FUNCTION(x)=>{
            returnString += ~"FUNCTION " + x;
        }, 
        CSS_TOKEN_STRING(x)=>{
            returnString += ~"STRING " + x;
        }, 
        CSS_TOKEN_INVALID_STRING=>{
            returnString += ~"INVALID_STRING ";
        }, 
        CSS_TOKEN_URI(x)=>{
            returnString += ~"URI " + x;
        }, 
        CSS_TOKEN_UNICODE_RANGE(ch1 , ch2)=>{
            returnString += ~"UNICODE_RANGE " + from_char(ch1)+~" "+from_char(ch2);
        }, 
        CSS_TOKEN_CHAR(ch)=>{
            returnString += ~"CHAR " + from_char(ch);
        },
        CSS_TOKEN_NUMBER(NumVal , x)=>{
            returnString += ~"NUMBER " + NumValToString(NumVal)+~" "+x;
        }, 
        CSS_TOKEN_PERCENTAGE(NumVal , x)=>{
            returnString += ~"PERCENTAGE "+NumValToString(NumVal)+~" "+x;
        }, 
        CSS_TOKEN_DIMENSION(NumVal , x1, x2)=>{
            returnString += ~"DIMENSION "+NumValToString(NumVal)+~" "+x1+~" "+x2;
        },
        CSS_TOKEN_CDO=>{
            returnString += ~"CDO ";
        }, 
        CSS_TOKEN_CDC=>{
            returnString += ~"CDC ";
        }, 
        CSS_TOKEN_S=>{
            returnString += ~"S ";
        },
        // CSS_TOKEN_COMMENT, 
        // CSS_TOKEN_INCLUDES, CSS_TOKEN_DASHMATCH, CSS_TOKEN_PREFIXMATCH, 
        // CSS_TOKEN_SUFFIXMATCH, CSS_TOKEN_SUBSTRINGMATCH, 
        CSS_TOKEN_EOF =>{
            returnString += ~"EOF ";
        }
    }
    return returnString;   
}

fn NumValToString(NumVal:NumericValue)-> ~str {
    match NumVal {
        Integer(x)=>{
            return fmt!("%?",x);
        },
        Float(x)=>{
            return fmt!("%?",x);
        },
    }
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