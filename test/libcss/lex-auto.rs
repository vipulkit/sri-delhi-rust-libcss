extern mod std;
extern mod css;
extern mod parserutils ; 
extern mod testutils;
extern mod wapcaplet;

use css::utils::errors::*;
use css::lex::lexer::*;
use css::charset::csdetect::*;

use parserutils::input::inputstream::*;

// use core::str::*;
use core::float::*;
use testutils::*;

static EXP_ENTRY_TEXT_LEN:int = (128);

pub type  line_func =  
    ~extern fn(data:~str , pw:&mut line_ctx_lex) -> bool;

fn token_to_string(token:css_token_type)-> ~str {
    let mut returnString =~"";
    match token {
        CSS_TOKEN_IDENT=>{
            returnString += ~"IDENT:";
        },
        CSS_TOKEN_ATKEYWORD=>{
            returnString += ~"ATKEYWORD:";
        },
        CSS_TOKEN_HASH=>{
            returnString += ~"HASH:";
        },
        CSS_TOKEN_FUNCTION=>{
            returnString += ~"FUNCTION:";
        }, 
        CSS_TOKEN_STRING=>{
            returnString += ~"STRING:";
        }, 
        CSS_TOKEN_INVALID_STRING=>{
            returnString += ~"INVALID:";
        }, 
        CSS_TOKEN_URI=>{
            returnString += ~"URI:";
        }, 
        CSS_TOKEN_UNICODE_RANGE=>{
            returnString += ~"UNICODE-RANGE:";
        }, 
        CSS_TOKEN_CHAR=>{
            returnString += ~"CHAR:";
        },
        CSS_TOKEN_NUMBER=>{
            returnString += ~"NUMBER:";
        }, 
        CSS_TOKEN_PERCENTAGE=>{
            returnString += ~"PERCENTAGE:";
        }, 
        CSS_TOKEN_DIMENSION=>{
            returnString += ~"DIMENSION:";
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
        CSS_TOKEN_COMMENT => {
            returnString += ~"COMMENT";
        },
        CSS_TOKEN_INCLUDES => {
            returnString += ~"INCLUDES";
        },
        CSS_TOKEN_DASHMATCH => {
            returnString += ~"DASHMATCH";
        },
        CSS_TOKEN_PREFIXMATCH => {
            returnString += ~"PREFIXMATCH";
        },
        CSS_TOKEN_SUFFIXMATCH => {
            returnString += ~"SUFFIXMATCH";
        },
        CSS_TOKEN_SUBSTRINGMATCH => {
            returnString += ~"SUBSTRINGMATCH";
        }
        CSS_TOKEN_EOF =>{
            returnString += ~"EOF";
        }
    }
    return returnString;   
}

pub fn handle_line(mut data:~str,  pw:LINE_CTX_DATA_TYPE)->bool
{
    // io::println("Entering: handle_line");

    let mut ctx = match pw { 
        CSDETECT(_) => fail!(~"In File lex-auto.rs, Function handle_line, argument LINE_CTX_DATA_TYPE contains incorrect struct line_ctx_csdetect"), 
        LEX(x) => x 
    };

    // unsafe{io::println(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    io::println(~"STRING="+data);

    if data.len() <= 0 {
		io::println("error");
		return true;
	}
   
	if (data[0] == '#' as u8) {
        // io::println("Entering: data[0] == '#' as u8");
		if (ctx.inexp) {
			/* This marks end of testcase, so run it */

			run_test(copy ctx.buf , copy ctx.exp);
			ctx.exp= ~[];
			ctx.buf=~[];
		}

		if (ctx.indata && str::ends_with(data, &"expected")) {
			ctx.indata = false;
			ctx.inexp = true;
            //unsafe{io::println(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
		} else if (!ctx.indata) {
			ctx.indata = str::ends_with(data,&"data");
			ctx.inexp  = str::ends_with(data,&"expected");
            //unsafe{io::println(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
		} else {
			ctx.buf += data.to_bytes();
            ctx.buf.push('\n' as u8);
            unsafe {io::println(fmt!("buf == %?\n\n" , ctx.buf));}
		}
	}
	else {
		if ctx.indata {
			ctx.buf += data.to_bytes();
            ctx.buf.push('\n' as u8);
			
            unsafe {io::println(fmt!("buf == %?\n\n" , ctx.buf));}
		}
		if (ctx.inexp) {
            // Convert /n to '/n'
            let mut new_data = str::replace(data, "\\n", "\n");
            new_data = str::replace(new_data, "\\t", "\t");

            let mut unescaped_data = ~"";
            let mut counter = 0;

            while (counter < new_data.len()) {
                if (new_data[counter] as char == '\\') {
                    counter +=1;
                    loop;
                }

                unescaped_data.push_char(new_data[counter] as char);
                counter +=1;
            }
            io::print("unescaped_data == ");
            io::println(unescaped_data);

            ctx.exp.push(unescaped_data);
            unsafe{io::println(fmt!("exp == %?", ctx.exp));}
		}
	}

    //unsafe{io::println(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
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
		mut buf:~[],
		mut exp : ~[],

		mut indata:false,
		mut inexp:false
	};
	//ctx.buf.push(0);//why?
	assert!(css__parse_testfile(copy fileName, ~handle_line, LEX(ctx)) == true);
	
    if unsafe {copy ctx.buf.len()} > 0 {
        // io::println("testMain : inside if ctx.buf.len() > 0");
		run_test(copy ctx.buf,copy ctx.exp);
	}
    io::println("testMain : outside if ctx.buf.len() > 0");
}


pub fn run_test(data:~[u8], exp:~[~str]) {
	// io::println("run test");
	// io::println(~"run test data="+ from_bytes(*data));
	let (inputStreamOption, _)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT as int), Some(~css__charset_extract));

    let inputstream = 
        match(inputStreamOption) {
            Some(x)   => x,
            None        => {
                // io::println("InputStream is not created, hence lexer can't be initialised");                        
                fail!(~"InputStream is not created, hence lexer can't be initialised");
            }
        };

    // io::println("Creating lexer");
    let mut lexer = css_lexer::css__lexer_create(inputstream);

    lexer.css__lexer_append_data(data);
    lexer.css__lexer_append_data(~[]);
    // lexer.data_done();
    // io::println(~"after append data="+ from_bytes(*data));
    let mut index = 0;
    loop {
    	// io::println("inside loop");
        let (error,token_option)= lexer.css__lexer_get_token();
       
        match(error)	{
            CSS_OK => {
                let token = token_option.unwrap();
                // io::println(foundmt!("token == %?", token));

                let token_type_string = token_to_string(token.token_type);
                // unsafe{io::println(fmt!("token bytes == %?", token.data.data));}
                let token_data = str::from_bytes(copy token.data.data);
                let mut found = token_type_string;
                
                if ((token.token_type as int) < (CSS_TOKEN_LAST_INTERN as int)) {
                    found += token_data;
                }

                //let found = {copy str::trim(found_str)};
                io::println(fmt!("found == %?", found));
                io::println(fmt!("Expected token == %?", (exp[index])));
                // io::println(fmt!("Expected token bytes == %?", str::to_bytes(exp[index])));
                if  !(found == exp[index]) {
                    // io::println(fmt!("Expected token == %?", (exp[index])));
                    // io::println(fmt!("Found token == %?", (found)));
                    fail!(~"Expected and Found tokens do not match.");

                }

                index += 1;
            },
            _=>{
                io::println(fmt!("error = %?", error));
            	if token_option.is_some() {
                    
                    let token = token_option.unwrap();
                    // io::println(fmt!("token == %?", token));

                    let token_type_string = token_to_string(token.token_type);
                    let token_data = str::from_bytes(copy token.data.data);

                    let found = fmt!("%s%s" , token_type_string , token_data);

                    io::println(fmt!("found == %?", found));
                    io::println(fmt!("Expected token == %?", (exp[index])));
                    if  found.ne(&exp[index]) {
                        // io::println(fmt!("Expected token == %?", (exp[index])));
                        // io::println(fmt!("Found token == %?", (found)));
                        fail!(~"Expected and Found tokens do not match.");

                    }
                    index += 1;
                }
                break;
            }
        } // match

        if (index == exp.len()) {break;}
        
    }

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

fn main() {
    testMain(~"data/lex/tests1.dat");
}