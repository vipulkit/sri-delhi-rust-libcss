
extern mod parserutils ; 
extern mod test;
extern mod css;

//use css_enum::*;
use core::io::*;

use css::lex::lexer::*;
use test::*;
use parserutils::* ;
use parserutils::input::inputstream::*;
use parserutils::charset::csdetect::*;
use core::str::*;
use core::int::*;
use core::float::*;
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
                Delim(x1)=>{
                    match token2 {
                        Delim(x2)=>{
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
                // CSS_TOKEN_COMMENT, 
                // CSS_TOKEN_INCLUDES, CSS_TOKEN_DASHMATCH, CSS_TOKEN_PREFIXMATCH, 
                // CSS_TOKEN_SUFFIXMATCH, CSS_TOKEN_SUBSTRINGMATCH, 
                CSS_TOKEN_EOF=>{
                    match token2 {
                        CSS_TOKEN_EOF=>{
                                return true;
                        },
                        _=> return false
                    }
                } 
                

            }
            return true;
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
        Delim(ch)=>{
            returnString += ~"Delim " + from_char(ch);
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
        ~"Delim"=> {
            if data.len() < 1 {
                fail!();
            }
            return Delim(copy data[0].char_at(0));//char error
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
fn main() {
	let CHUNKSIZE:int =10;
	let args : ~[~str] = os::args();
    // io::println(args[1]);
    let mut external_argument : ~str = copy args[1];
    let mut verification_file:~str =   copy args[1]; 
    let mut read_write_mode = copy args[2];
    let mut isReadMode = true;
      
    match read_write_mode {
        ~"read"=>{
            isReadMode = true;
        },
        ~"write"=>{
            isReadMode = false;
        },
        _=>{
            io::println ("argument can be a> read b> write ");
            return;
        }
    }
    
    verification_file= verification_file.slice(0,verification_file.len()-4).to_owned();
    verification_file += ~"_token.css";
    let writer_tokens:@core::io::Writer = io::file_writer(&Path(verification_file), ~[io::Create/*, io::NoFlag*/]).get();  ;
    let r_tokens:@Reader =  io::file_reader(&Path(verification_file)).get();
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let mut expectedTokenString:~str;
    
    //io::println(data);
    let mut fileLen:int;
    let mut data:~str;
    let mut dataBytes:~[u8];
    let reader = io::stdin();


    let mut exit:bool=false;
    let mut test_logger = result::unwrap(test_report(&"Unit_test_report_lexer_chunk.csv"));   

    let (inputStreamOption, ParserUtilsError)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT), Some(~css__charset_extract));

    let inputstream = 
        match(inputStreamOption) {
            Some(x)   => x,
            None        => {
                            io::println("InputStream is not created, hence lexer can't be initialised");                        
                            fail!();
            }
        };

    io::println("Creating lexer");
    let mut lexer = css_lexer::css__lexer_create(inputstream);
    /*let mut lexer : ~css_lexer;
    if lexerOption.is_some(){
            lexer = lexerOption.unwrap();
    }
    else{
            io::println("Lexer is not created.");                        
                        fail!();           
    }*/

	r.seek(0,SeekEnd);
	fileLen = r.tell() as int;
	
	r.seek(0,SeekSet);

	while(fileLen > CHUNKSIZE ) {
		dataBytes = r.read_bytes(CHUNKSIZE as uint);
                
        fileLen -= dataBytes.len() as int ;
		let str1= str::from_bytes(dataBytes);
				
		//io::println(~"data =  "+str1);		
		lexer.lexer_append_data(dataBytes);
        let mut tok:css_token_type;
		while(true) {
            let (tokOpt,Errr)= lexer.get_token();
            match(Errr)	{
                LEXER_NEEDDATA => {
                    if tokOpt.is_some() {
                        tok= tokOpt.unwrap();
                        if isReadMode {
                               let ExpectedTokenString = (r_tokens.read_line());
                               let ExpectedToken = stringToToken(copy ExpectedTokenString);
                                let result = matchtokens(copy tok, copy ExpectedToken);
                                if !result {
                                    io::println(fmt!("fail::Expected Token String = %?(%?) , Found = %?",ExpectedTokenString,ExpectedToken, tok));
                                    //reader.read_byte();
                                    //reader.read_byte();
                                }
                                //assert!(result == true);
                        }
                        else {
                                io::println(fmt!("found Token is = %?", copy tok));
                                writer_tokens.write_line(tokenToString(copy tok));
                        }
                        
                        
                        //test_logger.info( ~"test_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" ,~"token should be read", ~"NEED DATA"+fmt!("token read is %?",tok )) ;      
                    }
                    break
                },
                _=>{}
            }

            tok= match(tokOpt)	{
               	Some(tok)=>tok,
               	None=> {
                    test_logger.info( ~"ctest_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" , ~"token should be read",~"token read is None") ;
                    io::println("");
                    break
                }
            };
            
           
            if isReadMode {
                let ExpectedTokenString = (r_tokens.read_line());
                let ExpectedToken = stringToToken(copy ExpectedTokenString);
                let result = matchtokens(copy tok, copy ExpectedToken);
                if !result {
                    io::println(fmt!("fail::Expected Token String = %?(%?) , Found = %?",ExpectedTokenString,ExpectedToken, tok));
                    //reader.read_byte();
                    //reader.read_byte();
                }
                //assert!(result == true);
            }
            else {
                io::println(fmt!("found Token is = %?", copy tok));
                writer_tokens.write_line(tokenToString(copy tok));
            }
            //test_logger.info( ~"test_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" ,~"token should be read", fmt!("token read is %?",tok )) ;
        }
	}


	dataBytes = r.read_bytes(fileLen as uint);
                
                
    let str1= str::from_bytes(dataBytes);
    //::println(~"data =  "+str1);  
    lexer.lexer_append_data(dataBytes);
    lexer.data_done();

    while(true) {
        let (tokOpt,Errr)= lexer.get_token();

        match(tokOpt)  {
            Some(tok)=>{
                //test_logger.info(~"test_lexer_chunks.rs" ,  copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" , ~"token should be read",fmt!("token read is %?",tok ));
                if isReadMode {
                    let ExpectedTokenString = (r_tokens.read_line());
                    let ExpectedToken = stringToToken(copy ExpectedTokenString);
                    let result = matchtokens(copy tok, copy ExpectedToken);
                    if !result {
                        io::println(fmt!("fail::Expected Token String = %?(%?) , Found = %?",ExpectedTokenString,ExpectedToken, tok));
                        //reader.read_byte();
                        //reader.read_byte();
                    }
                    //assert!(result == true);
                }
                else {
                    io::println(fmt!("found Token is = %?", copy tok));
                    writer_tokens.write_line(tokenToString(copy tok));
                }
                match tok {
                    CSS_TOKEN_EOF => {
                        test_logger.pass( ~"test_lexer_chunks.rs" ,copy external_argument  ,~"lexer",~"css_lexer.rs"  , ~"get_token", ~"end of file reached" ,~"CSS_TOKEN_EOF",~"end of file reached", ~"CSS_LEXER_PASSED");
                        break;
                    },
                    _=>{ 

                    //test_logger.info(~"test_lexer_chunks.rs" ,  copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" , ~"token should be read",fmt!("token read is %?",tok )); 
                    }   
                }
            },
            None=> {
                test_logger.info(~"btest_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"test_lexer" ,~"token should be read", ~"token read is None");
                test_logger.fail(~"atest_lexer_chunks.rs" , copy external_argument,  ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"end of file reached" ,~"non CSS_TOKEN_EOF",~"End of file not reached", ~"CSS_LEXER_FAILED");
            }
        };
    }
} 
