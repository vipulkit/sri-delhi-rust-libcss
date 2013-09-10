extern mod std;
extern mod css;
extern mod parserutils ; 
extern mod wapcaplet;
extern mod extra;


use std::{io,str};

use css::utils::errors::*;
use css::lex::lexer::*;
use css::charset::csdetect::*;

use parserutils::input::inputstream::*;

pub struct line_ctx_lex {
    buf:~[u8],

    indata:bool,
    inexp:bool
}

fn check_newline(x: &u8) -> bool { *x == ('\n' as u8) }

pub type  line_func =  
    ~extern fn(data:~str , pw:&mut line_ctx_lex) -> bool;

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
            returnString = returnString + "INVALID:";
        }, 
        CSS_TOKEN_URI=>{
            returnString = returnString + "URI:";
        }, 
        CSS_TOKEN_UNICODE_RANGE=>{
            returnString = returnString + "UNICODE-RANGE:";
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


fn match_vec_u8(expected_data: &[u8] , found_string: &str) -> bool {

    if found_string.len() != expected_data.len() {
        // debug!("lenghts don't match");
        return false;
    }
    let mut z = 0 ;
    let z_len = found_string.len() ;
    while z<z_len {
      if found_string[z] != expected_data[z] {
            return false;
        }
        z += 1;
    } 
    
    true
}

pub fn handle_line(args: ~[u8],  ctx:@mut line_ctx_lex,w:@Writer )->bool
{
    let data : ~[u8] = args ;
    // unsafe{debug!(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    if  (data.len() == 0) {
        // debug!("error");
        return true;
    }

    if (data[0] == '#' as u8) {
        if (ctx.inexp ) {
            /* This marks end of testcase, so run it */

           if ctx.buf.len() > 0 {
            w.write_line("#expected");

            run_test(ctx.buf.clone(), w);
            w.write_line("#reset");
            ctx.buf=~[];
	  }
        }
        if (ctx.indata && ( match_vec_u8(data, &"#expected") || match_vec_u8(data,&"#reset" ))) {
            ctx.indata = false;
            ctx.inexp  = match_vec_u8(data,&"#expected");
            if (!ctx.inexp)
            {
            	ctx.inexp = match_vec_u8(data,&"#reset"); 
	    }
        } else if (!ctx.indata) {
            ctx.indata = match_vec_u8(data,&"#data");
            ctx.inexp  = match_vec_u8(data,&"#expected");
            if (!ctx.inexp)
            {
            	ctx.inexp = match_vec_u8(data,&"#reset"); 
	    }
        } else {
            ctx.buf = ctx.buf + data.clone();
            ctx.buf.push('\n' as u8);
        }
    }		
    else {
        if ctx.indata {
            ctx.buf = ctx.buf + data.clone();
            ctx.buf.push('\n' as u8);
        }
    }

    return true;
}

fn testMain(fileName: ~str, fileWrite:~str) {
    // debug!(~"testMain : "+ fileName);
    let ctx = @mut line_ctx_lex
    {
        buf:~[],

        indata:false,
        inexp:false
    };

    let file_content_result = io::read_whole_file(&Path(fileName)) ;
    let w:@Writer = io::file_writer(&Path(fileWrite),[io::Create, io::Truncate]).unwrap();
    let mut file_content : ~[u8] ;
    match file_content_result {
        Ok(x) => {
            file_content = x ;
        },
        Err(_) => {
            file_content = ~[] ;
            debug!(fmt!("\n Error opening file"));
            assert!(false) ;
        }
    }        
    let mut vec_lines = file_content.split_iter(check_newline) ;

    for vec_lines.advance |each_line| {
        handle_line(each_line.to_owned(),ctx, w);
    }
    
    if ctx.buf.len() > 0 {
        w.write_line("#expected");
        run_test(ctx.buf.clone(), w);
        w.write_line("#reset");
    }
}


pub fn run_test(data:~[u8], w:@Writer) {
    // debug!("run test");
    let (inputStreamOption, _)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT as int), Some(@css__charset_extract));

    let inputstream = 
        match(inputStreamOption) {
            Some(x)   => x,
            None        => {
                fail!(~"InputStream is not created, hence lexer can't be initialised");
            }
        };

    let mut lexer = css_lexer::css__lexer_create(inputstream);

    lexer.css__lexer_append_data(data);
    lexer.css__lexer_append_data([]);

    // debug!(~"after append data="+ from_bytes(*data));
    let mut found_ch:~str;
    loop {
        let (error,token_option)= lexer.css__lexer_get_token();
        if token_option.is_none()
	{
            break;
	}

        match(error)    {
            CSS_OK => {
                let token = token_option.unwrap();
                // debug!(foundmt!("token == %?", token));

                let token_type_string = token_to_string(token.token_type);
                // unsafe{debug!(fmt!("token bytes == %?", token.data.data));}
                let token_data = str::from_bytes(token.data.data.clone());
                found_ch = token_type_string;
                
                if ((token.token_type as int) < (CSS_TOKEN_LAST_INTERN as int)) {
                    found_ch = found_ch + token_data;
                }
                else if ((token.token_type as int) == (CSS_TOKEN_EOF as int))
                {
                    w.write_line("EOF");
                    break;
                }

                let mut out: ~str = ~"";
                out.reserve_at_least(found_ch.len());

                for found_ch.iter().advance |c| {
                    if (c == '\n' || c == '\\' || c == '\t')   
                    {
                        do c.escape_default |c| {
                            out.push_char(c);
                        }
                    }
                    else
                    {
                        out.push_char(c);
                    }
                }
                w.write_line(out);
            },
            _=>{
                debug!("error = %?", error);
                    if token_option.is_some() {
                        
                        //fail!(~"Expected and Found tokens do not match.");

                }
                break;
            }
        } // match 
    }

}


#[test]
fn tests1() {
    testMain(~"data/lex/tests1.dat",~"data/lex/lex_result1.dat");
}

#[test]
fn tests2() {
    testMain(~"data/lex/tests2.dat",~"data/lex/lex_result2.dat");
}

#[test]
fn regression() {
    testMain(~"data/lex/regression.dat", ~"data/lex/lex_reg_result.dat");
}

fn main() {
    testMain(~"data/lex/tests1.dat",~"data/lex/lex_results.dat");
}
