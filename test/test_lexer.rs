
extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;
extern mod css_lexer;
extern mod csdetect;
extern mod css_enum;

use css_enum::*;
use core::io::*;
use csdetect::*;
use css_lexer::*;
use test::*;
use parserutils::* ;
use parserutils_inputstream::*;
 
 fn main() {
	let CHUNKSIZE:uint =10;
	let args : ~[~str] = os::args();
    let external_argument : ~str = copy args[1];
    io::println(args[1]);
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let mut fileLen:int;
    let mut data:~str;
    let mut dataBytes:~[u8];
    let reader = io::stdin();
    let mut exit:bool=false;

    let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
    let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"UTF-8",Some(~css__charset_extract));
    let mut lexer = lcss_lexer((inputStreamOption, ParserUtilsError)).unwrap();
	
    while(!r.eof()) {
				
        data= r.read_line();
        dataBytes= str::to_bytes(data);
                
		let str1= str::from_bytes(dataBytes);
		
        test1.info( ~"test_lexer.rs", copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"file reading", ~"test_lexer" ,~"contents of file", fmt!("read data is %?", str1),~"") ;   
				
		lexer.lexer_append_data(dataBytes);
        let mut tok:css_token_type;
		while(true) {
            let (tokOpt,Errr)= lexer.css__lexer_get_token();
            match(Errr)	{
                LEXER_NEEDDATA => {
                    if tokOpt.is_some() {
                        tok= tokOpt.unwrap();
                        test1.info( ~"test_lexer.rs", copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"file reading", ~"test_lexer" ,~"token read is---NEED DATA----", fmt!(" %?",tok )) ;
                    }
                    break
                },
                _=>{}
        	}
            tok= match(tokOpt) {
                Some(tok)=>tok,
                None=> break
    	    };
            test1.info( ~"test_lexer.rs", copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"file reading", ~"test_lexer" ,~"token read is", fmt!(" %?",tok )) ;                	              	
					
		    match(tok)	{
                CSS_TOKEN_EOF  => { 
                    exit=true ;
                    break
                },
			    _=>{}
		    }					
        }
    }

} 
