extern mod std;
extern mod parserutils;
extern mod css;

use parserutils::charset::csdetect::*;
use parserutils::charset::aliases::*;
use parserutils::input::inputstream::*;
use css::lex::lexer::*;
use core::str::*;
use core::float::*;

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
        let (inputStreamOption, ParserUtilsError)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT), Some(~css__charset_extract));
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
            io::println(fmt!("final_buf is %s",str::from_bytes(final_buf)));
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
                let ExpectedToken = stringToToken(copy expectedstr);
                let result = matchtokens(copy tok, copy ExpectedToken);
                assert!(result);
                match tok {
                    CSS_TOKEN_EOF => break,
                    _ => {}
                }
            }
        }
    }
}

fn matchNumericValue(NumVal1:NumericValue,NumVal2:NumericValue)-> bool {
    match NumVal1 {
        Integer(x1)=>{
           match NumVal2 {
                Integer(x2)=>{
                    if x1==x2 {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
           }
        },
        Float(x1)=>{
            match NumVal2 {
                Float(x2)=>{
                    if (x1==x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
           }
        }
    }
}
fn matchtokens(token1:css_token_type,token2:css_token_type)->bool {
     match token1 {
        CSS_TOKEN_IDENT(x1)=>{
            match token2 {
                CSS_TOKEN_IDENT(x2)=>{
                    if str::eq(&x1,&x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        },
        CSS_TOKEN_ATKEYWORD(x1)=>{
            match token2 {
                CSS_TOKEN_ATKEYWORD(x2)=>{
                    if str::eq(&x1,&x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        },
        CSS_TOKEN_HASH(x1)=>{
            match token2 {
                CSS_TOKEN_HASH(x2)=>{
                    if str::eq(&x1,&x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        },
        CSS_TOKEN_FUNCTION(x1)=>{
            match token2 {
                CSS_TOKEN_FUNCTION(x2)=>{
                    if str::eq(&x1,&x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_STRING(x1)=>{
            match token2 {
                CSS_TOKEN_STRING(x2)=>{
                    if str::eq(&x1,&x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_INVALID_STRING=>{
            match token2 {
                CSS_TOKEN_INVALID_STRING=>{
                    return true;
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_URI(x1)=>{
            match token2 {
                CSS_TOKEN_URI(x2)=>{
                    if str::eq(&x1,&x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_UNICODE_RANGE(x1 , y1)=>{
            match token2 {
                CSS_TOKEN_UNICODE_RANGE(x2,y2)=>{
                    if x1==x2 && y1==y2{
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_CHAR(x1)=>{
            match token2 {
                CSS_TOKEN_CHAR(x2)=>{
                    if x1==x2 {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        },
        CSS_TOKEN_NUMBER(x1 , y1)=>{
            match token2 {
                CSS_TOKEN_NUMBER(x2,y2)=>{
                    if str::eq(&y1,&y2) && matchNumericValue(x1,x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_PERCENTAGE(x1 , y1)=>{
            match token2 {
                CSS_TOKEN_PERCENTAGE(x2 , y2)=>{
                    if str::eq(&y1,&y2) && matchNumericValue(x1,x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_DIMENSION(x1 , y1, z1)=>{
            match token2 {
                CSS_TOKEN_DIMENSION(x2 , y2, z2)=>{
                    if str::eq(&y1,&y2) &&  str::eq(&z1,&z2) && matchNumericValue(x1,x2) {
                        return true;
                    }
                    else {
                        return false;
                    }
                },
                _=> return false
            }
        },
        CSS_TOKEN_CDO=>{
            match token2 {
                CSS_TOKEN_CDO=>{
                    return true;
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_CDC=>{
            match token2 {
                CSS_TOKEN_CDC=>{
                        return true;
                },
                _=> return false
            }
        }, 
        CSS_TOKEN_S=>{
            match token2 {
                CSS_TOKEN_S=>{
                        return true;
                },
                _=> return false
            }
        },
        CSS_TOKEN_EOF=>{
            match token2 {
                CSS_TOKEN_EOF=>{
                        return true;
                },
                _=> return false
            }
        } 
    }
}
fn NumericValue(string:~str) -> NumericValue {

    let NumVal:NumericValue;
    let mut isFloat= false;
    for string.each() |i| {
        if i=='.' as u8 {
            isFloat= true;
        }
    }
    if !isFloat {
         NumVal=Integer(int::from_str(string).unwrap());
    }
    else {
        NumVal=Float(float::from_str(string).unwrap());
    }
       
    return NumVal;
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
fn stringToToken(string:~str)->(css_token_type) {
    //io::println(~"string = "+string);
    let mut isToken:bool =true;
    let mut token:~str=~"";
    let mut data:~[~str]=~[];
    for each_word(string) |ww| {
        if isToken {
            token = ww.to_owned();
            //io::println(~"ww = "+ww);
            isToken =false;
        }
        else {
            //io::println(~"ww = "+ww);
            data.push(ww.to_owned());
        }
    }

    match token {
        ~"IDENT"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_IDENT( copy data[0]);
        },
        ~"ATKEYWORD"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_ATKEYWORD(copy data[0]);
        },
        ~"HASH"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_HASH(copy data[0]);
        },
        ~"FUNCTION"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_FUNCTION(copy data[0]);
        },
        ~"STRING"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_STRING(copy data[0]);
        },
        ~"INVALID_STRING"=> {
            /*if data.len() ==0 {
                fail!();
            }*/
            return CSS_TOKEN_INVALID_STRING;
        },
        ~"URI"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_URI(copy data[0]);
        },
        ~"UNICODE_RANGE"=> {
            if data.len() < 2 {
                fail!();
            }
            return CSS_TOKEN_UNICODE_RANGE(copy data[0].char_at(0),copy data[1].char_at(0));//char,char error
        },
        ~"CHAR"=> {
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_CHAR(data[0].char_at(0));//char  error
        },
        ~"NUMBER"=> {
            if data.len() < 2 {
                fail!();
            }
            return CSS_TOKEN_NUMBER(NumericValue(copy data[0]),copy data[1]);
        },
        ~"PERCENTAGE"=> {
            if data.len() < 2 {
                fail!();
            }
            return CSS_TOKEN_PERCENTAGE(NumericValue(copy data[0]),copy data[1]);
        },
        ~"DIMENSION"=> {
            if data.len() < 3 {
                fail!();
            }
            return CSS_TOKEN_DIMENSION(NumericValue(copy data[0]),copy data[1],copy data[2]);
        },
        ~"CDO"=> {
            
            return CSS_TOKEN_CDO;
        },
        ~"CDC"=> {
            
            return CSS_TOKEN_CDC;
        },
        ~"S"=> {
            
            return CSS_TOKEN_S;
        },
        ~"EOF"=> {
            
            return CSS_TOKEN_EOF;
        },
        _=>{
            //io::println(token);
            return CSS_TOKEN_INVALID_STRING;
        }
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