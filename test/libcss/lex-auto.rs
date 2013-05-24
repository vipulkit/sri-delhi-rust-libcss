extern mod css;
extern mod parserutils ; 
extern mod css;
extern mod testutils;

use css::lex::lexer::*;
use css::charset::csdetect::*;

use parserutils::input::inputstream::*;

use core::str::*;
use core::float::*;
use core::io::*;
use testutils::*;

static EXP_ENTRY_TEXT_LEN:int = (128);

pub type  line_func =  
    ~extern fn(data:~str , pw:&mut line_ctx_lex) -> bool;

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

fn token_to_string(token:css_token_type)-> ~str {
    let mut returnString =~"";
    match token {
        CSS_TOKEN_IDENT(x)=>{
            returnString += ~"IDENT:" + x;
        },
        CSS_TOKEN_ATKEYWORD(x)=>{
            returnString += ~"ATKEYWORD:" + x;
        },
        CSS_TOKEN_HASH(x)=>{
            returnString += ~"HASH:" + x;
        },
        CSS_TOKEN_FUNCTION(x)=>{
            returnString += ~"FUNCTION:" + x;
        }, 
        CSS_TOKEN_STRING(x)=>{
            returnString += ~"STRING:" + x;
        }, 
        CSS_TOKEN_INVALID_STRING=>{
            returnString += ~"INVALID_STRING";
        }, 
        CSS_TOKEN_URI(x)=>{
            returnString += ~"URI:" + x;
        }, 
        CSS_TOKEN_UNICODE_RANGE(ch1 , ch2)=>{
            returnString += ~"UNICODE_RANGE: " + from_char(ch1)+~" "+from_char(ch2);
        }, 
        CSS_TOKEN_CHAR(ch)=>{
            returnString += ~"CHAR:" + from_char(ch);
        },
        CSS_TOKEN_NUMBER(NumVal , x)=>{
            returnString += ~"NUMBER:" + NumValToString(NumVal)+~" "+x;
        }, 
        CSS_TOKEN_PERCENTAGE(NumVal , x)=>{
            returnString += ~"PERCENTAGE:"+NumValToString(NumVal)+~" "+x;
        }, 
        CSS_TOKEN_DIMENSION(NumVal , x1, x2)=>{
            returnString += ~"DIMENSION:"+NumValToString(NumVal)+~" "+x1+~" "+x2;
        },
        CSS_TOKEN_CDO=>{
            returnString += ~"CDO";
        }, 
        CSS_TOKEN_CDC=>{
            returnString += ~"CDC";
        }, 
        CSS_TOKEN_S=>{
            returnString += ~"S";
        },
        /*Delim(ch)=>{
            returnString += ~"Delim " + from_char(ch);
        },*/
        // CSS_TOKEN_COMMENT, 
        // CSS_TOKEN_INCLUDES, CSS_TOKEN_DASHMATCH, CSS_TOKEN_PREFIXMATCH, 
        // CSS_TOKEN_SUFFIXMATCH, CSS_TOKEN_SUBSTRINGMATCH, 
        CSS_TOKEN_EOF =>{
            returnString += ~"EOF";
        }
    }
    return returnString;   
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
            ctx.exp.push(data);
			//css__parse_expected(ctx, &data);
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


pub fn run_test(data:~[u8], exp:~[~str]) {
	// io::println("run test");
	// io::println(~"run test data="+ from_bytes(*data));
	let (inputStreamOption, _)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT as int), Some(~css__charset_extract));

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
    lexer.data_done();
    // io::println(~"after append data="+ from_bytes(*data));
    let mut index = 0;
    loop {
    	// io::println("inside loop");
        let (token_option,error)= lexer.get_token();
       
        match(error)	{
            LEXER_OK => {
            	let token_string = token_to_string(token_option.unwrap());
                io::println(fmt!("Expected token == %?", exp[index]));
                io::println(fmt!("Found token == %?", token_string));
                if token_string != exp[index] {
                    //fail!(~"Expected and Found tokens do not match.");
                }
                index += 1;
            },
            _=>{
                //io::println(fmt!("error == %?", error));
            	if token_option.is_some() {
                    let token_string = token_to_string(token_option.unwrap());
                    io::println(fmt!("Expected token == %?", exp[index]));
                    io::println(fmt!("Found token == %?", token_string));
                    if token_string != exp[index] {
                        //fail!(~"Expected and Found tokens do not match.");
                    }
                    index += 1;
                }
                break;
            }
        } // match
        
    }
    
    io::println(fmt!("Expected token == %?", exp[index]));
    let (token_option,error)= lexer.get_token();
    let token_string = token_to_string(token_option.unwrap());
    io::println(fmt!("Found token == %?", token_string));
    if token_string != exp[index] {
        //fail!(~"Expected and Found tokens do not match.");
    }
    index += 1;

    assert!(index == exp.len());
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