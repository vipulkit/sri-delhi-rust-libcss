
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
 
fn main() {
	let CHUNKSIZE:int =10;
	let args : ~[~str] = os::args();
    // io::println(args[1]);
    let mut external_argument : ~str = copy args[1];    
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let mut fileLen:int;
    let mut data:~str;
    let mut dataBytes:~[u8];
    let reader = io::stdin();


    let mut exit:bool=false;
    let mut test_logger = result::unwrap(test_report(&"Unit_test_report_lexer_chunk.csv"));   

    let (inputStreamOption, ParserUtilsError)= inputstream(~"UTF-8",Some(~css__charset_extract));

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

	while(fileLen > 0 ) {
		dataBytes = r.read_bytes(CHUNKSIZE as uint);
                
        fileLen -= dataBytes.len() as int ;
		let str1= str::from_bytes(dataBytes);
				
				
		lexer.lexer_append_data(dataBytes);
        let mut tok:css_token_type;
		while(true) {
            let (tokOpt,Errr)= lexer.get_token();
            match(Errr)	{
                LEXER_NEEDDATA => {
                    if tokOpt.is_some() {
                        tok= tokOpt.unwrap();
                          test_logger.info( ~"test_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" ,~"token should be read", ~"NEED DATA"+fmt!("token read is %?",tok )) ;      
                    }
                    break
                },
                _=>{}
            }

            tok= match(tokOpt)	{
               	Some(tok)=>tok,
               	None=> {
                    test_logger.info( ~"test_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" , ~"token should be read",~"token read is None") ;
                    break
                }
            };

            test_logger.info( ~"test_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" ,~"token should be read", fmt!("token read is %?",tok )) ;
        }
	}


	dataBytes = r.read_bytes(fileLen as uint);
                
                
    let str1= str::from_bytes(dataBytes);
              
    lexer.lexer_append_data(dataBytes);
    lexer.data_done();


    let (tokOpt,Errr)= lexer.get_token();

    match(tokOpt)  {
        Some(tok)=>{
            test_logger.info(~"test_lexer_chunks.rs" ,  copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"file read in chunks" , ~"token should be read",fmt!("token read is %?",tok ));
            match tok {
                CSS_TOKEN_EOF => test_logger.pass( ~"test_lexer_chunks.rs" ,copy external_argument  ,~"lexer",~"css_lexer.rs"  , ~"get_token", ~"end of file reached" ,~"CSS_TOKEN_EOF",~"end of file reached", ~"CSS_LEXER_PASSED"),
                _=> test_logger.fail(~"test_lexer_chunks.rs" , copy external_argument,  ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"end of file reached" ,~"non CSS_TOKEN_EOF",~"End of file not reached", ~"CSS_LEXER_FAILED")
            }
        },
        None=> test_logger.info(~"test_lexer_chunks.rs" , copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"get_token", ~"test_lexer" ,~"token should be read", ~"token read is None")
    };
} 
