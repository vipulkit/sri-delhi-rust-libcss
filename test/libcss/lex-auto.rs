extern mod css;
extern mod parserutils ; 
extern mod css;
extern mod testutils;

use css::lex::lexer::*;
use parserutils::input::inputstream::*;
use parserutils::charset::csdetect::*;
use core::str::*;
use core::float::*;
use core::io::*;
use testutils::*;

static EXP_ENTRY_TEXT_LEN:int = (128);

pub type  line_func =  
    ~extern fn(data:~str , pw:&mut line_ctx) -> bool;

struct line_ctx {
	buflen:uint,
	bufused:uint,
	buf:~[u8],

	explen:uint,
	expused:uint,
	exp:~[css_token_type],

	indata:bool,
	inexp:bool
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
        /*Delim(x1)=>{
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
        },*/
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
        /*Delim(ch)=>{
            returnString += ~"Delim " + from_char(ch);
        },*/
        // CSS_TOKEN_COMMENT, 
        // CSS_TOKEN_INCLUDES, CSS_TOKEN_DASHMATCH, CSS_TOKEN_PREFIXMATCH, 
        // CSS_TOKEN_SUFFIXMATCH, CSS_TOKEN_SUBSTRINGMATCH, 
        CSS_TOKEN_EOF =>{
            returnString += ~"EOF ";
        }
    }
    return returnString;   
}
fn stringToToken(string:&~str)->(css_token_type) {
    //io::println(~"string = "+string);
    let mut isToken:bool =true;
    let mut token:~str=~"";
    let mut data:~[~str]=~[];
    for each_split_char(*string, ':') |ww|{
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
            let mut len:int  = data.len() as int;
            let mut iter:int = 0;
            let mut stringData =~"";
            while iter < len -1 {
                stringData += data[iter];
                stringData += ~" ";
                iter+=1;
            }
            stringData += data[iter];
            return CSS_TOKEN_STRING(stringData);
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
            if data.len() < 1 {
                fail!();
            }
            return CSS_TOKEN_NUMBER(NumericValue(copy data[0]),copy data[0]);
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
        /*~"Delim"=> {
            if data.len() < 1 {
                fail!();
            }
            return Delim(copy data[0].char_at(0));//char error
        },*/
        ~"EOF"=> {
            
            return CSS_TOKEN_EOF;
        },
        _=>{
            //io::println(token);
            return CSS_TOKEN_INVALID_STRING;
        }
    }
} 

pub fn handle_line(mut data:~str,  pw:LINE_CTX_DATA_TYPE)->bool
{
	//io::println("handle_line");
	//io::println(~"STRING="+data);
	
    let ctx :@mut line_ctx_lex;

    match pw { CSDETECT(_) => fail!(~"In File lex-auto.rs, Function handle_line, argument LINE_CTX_DATA_TYPE contains incorrect struct line_ctx_csdetect"), LEX(x) => ctx = x }

    if data.len() <= 0 {
		io::println("error");
		return true;
	}
   
	if (data[0] == '#' as u8) {
		if (ctx.inexp) {
			/* This marks end of testcase, so run it */

			run_test(copy ctx.buf , copy ctx.exp);

			//ctx.buf[0] = 0;//
			ctx.exp= ~[];
			ctx.buf=~[];
			ctx.bufused = 0;

			ctx.expused = 0;
		}

		if (ctx.indata && str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"expected")) {
			ctx.indata = false;
			ctx.inexp = true;
		} else if (!ctx.indata) {
			ctx.indata = str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"data");
			ctx.inexp  = str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"expected")
		} else {
			ctx.buf = unsafe { ctx.buf.slice(0,ctx.bufused).to_owned() };
			ctx.buf += data.to_bytes();
			ctx.bufused += data.len();
		}
	}
	else {
		if ctx.indata {
			ctx.buf = unsafe { ctx.buf.slice(0,ctx.bufused).to_owned() };
			ctx.buf += data.to_bytes();
			ctx.bufused += data.len();
		}
		if (ctx.inexp) {
			if (data[data.len() - 1] == '\n' as u8) {
				data= data.slice(0,data.len()-1).to_owned();
			}
				//datalen -= 1;

			css__parse_expected(ctx, &data);
		}
	}

	return true;
}

fn testMain(fileName: ~str) {
	// io::println(~"testMain : "+ fileName);
	let len = css__parse_filesize(copy fileName);
	if len ==0 {
		return;
	}
	let ctx: @mut line_ctx_lex = @mut line_ctx_lex
    {
		mut buflen:len,
		mut bufused:0,
		mut buf:~[],
		mut explen : 0,
		mut expused :0,
		mut exp : ~[],
		mut indata:false,
		mut inexp:false
	};
	//ctx.buf.push(0);//why?
	assert!(css__parse_testfile(copy fileName, ~handle_line, LEX(ctx)) == true);
	if ctx.bufused > 0 {
		run_test(copy ctx.buf,copy ctx.exp);
	}
}


pub fn css__parse_expected(ctx:@mut line_ctx_lex, mut data:&~str) {
	
	let mut token_type:css_token_type ;
	token_type = stringToToken(data);
	// io::println(fmt!(" token= %?",token_type));
	ctx.exp.push( token_type );
	
	// io::println(fmt!("buflen=%?",ctx.buflen));
	// io::println(fmt!("bufused=%?",ctx.bufused));
	// io::println(fmt!("buf=%?",from_bytes(ctx.buf)));
	// io::println(fmt!("explen=%?",ctx.explen));
	// io::println(fmt!("expused=%?",ctx.expused));
	// io::println(fmt!("exp=%?",ctx.exp));
	// io::println(fmt!("indata=%?",ctx.indata));
	// io::println(fmt!("inexp=%?",ctx.inexp));
	// io::println(~"data = "+*data);
	
}

pub fn run_test(data:~[u8], exp:~[css_token_type]) {
	// io::println("run test");
	// io::println(~"run test data="+ from_bytes(*data));
	let (inputStreamOption, _)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT), Some(~css__charset_extract));

    let inputstream = 
        match(inputStreamOption) {
            Some(x)   => x,
            None        => {
                            io::println("InputStream is not created, hence lexer can't be initialised");                        
                            fail!();
            }
        };

    // io::println("Creating lexer");
    let mut lexer = css_lexer::css__lexer_create(inputstream);

    lexer.lexer_append_data(data);
    //lexer.data_done();
    // io::println(~"after append data="+ from_bytes(*data));
    let mut tok:css_token_type;
    let mut count=0;
    loop {
    	// io::println("inside loop");
        let (tokOpt,Errr)= lexer.get_token();
        match(Errr)	{
            LEXER_NEEDDATA => {
            	// io::println("inside LEXER_NEEDDATA");
                if tokOpt.is_some() {
                    tok= tokOpt.unwrap();
                    count +=1;
                    let result = matchtokens(copy tok, copy exp[count]);
                    if !result {
                        io::println(fmt!("fail::Expected Token = %? , Found = %?",exp[count], tok));
                       
                    }
                    // io::println(fmt!("token=%?",tok));
                }
                break
            },
            _=>{
            	// io::println("inside _")
            }
        }
        tok= match(tokOpt)	{
            Some(tok)=> {
            	// io::println("hi");
            	tok
            },
            None=> {
                // io::println("error");
                break
            }
        };
        count +=1;
        let result = matchtokens(copy tok, copy exp[count]);
        if !result {
            io::println(fmt!("fail::Expected Token = %? , Found = %?",exp[count], tok));
        }
		//io::println(fmt!("token=%?",tok));
		match tok {
			CSS_TOKEN_EOF => break,
			_ => {}
		}
    }
    assert!(count == exp.len());
}


#[test]
fn tests1() {
    testMain(~"data/lex/tests1.dat");
}

#[test]
fn tests2() {
    testMain(~"data/lex/tests2.dat");
}

#[test]
fn regression() {
    testMain(~"data/lex/regression.dat");
}