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
 
<<<<<<< HEAD
fn main() {
	let CHUNKSIZE:int =10;
	let args : ~[~str] = os::args();
    // io::println(args[1]);
=======
fn main()
{
	let CHUNKSIZE:int =10;
	let args : ~[~str] = os::args();
    let external_argument : ~str = copy args[1];
    io::println(args[1]);
>>>>>>> 39712dfcfc408342c7dc7a3037377a26268dc3cf
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let mut fileLen:int;
    let mut data:~str;
    let mut dataBytes:~[u8];
    let reader = io::stdin();
    let mut exit:bool=false;

    let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
    let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"UTF-8",Some(~css__charset_extract));
    let mut lexer = lcss_lexer((inputStreamOption, ParserUtilsError)).unwrap();
	r.seek(0,SeekEnd);
	fileLen = r.tell() as int;
	
	r.seek(0,SeekSet);

<<<<<<< HEAD
	while(fileLen > 0 ) {
=======
	while(fileLen > CHUNKSIZE) {
>>>>>>> 39712dfcfc408342c7dc7a3037377a26268dc3cf
		dataBytes = r.read_bytes(CHUNKSIZE as uint);
                
        fileLen -= dataBytes.len() as int ;
		let str1= str::from_bytes(dataBytes);
				
<<<<<<< HEAD
=======
        test1.info(~"test_lexer_chunks.rs", copy external_argument, ~"lexer",~"css_lexer.rs"  , ~"file reading", ~"test_lexer" ,~"contents of file", fmt!("read data is %?", str1),~"") ;  
>>>>>>> 39712dfcfc408342c7dc7a3037377a26268dc3cf
				
		lexer.lexer_append_data(dataBytes);
        let mut tok:css_token_type;
		while(true) {
            let (tokOpt,Errr)= lexer.css__lexer_get_token();
            match(Errr)	{
                LEXER_NEEDDATA => {
                    if tokOpt.is_some() {
                        tok= tokOpt.unwrap();
<<<<<<< HEAD
=======
                        test1.info( ~"test_lexer_chunks.rs", copy external_argument, ~"file reading", ~"test_lexer" ,~"token read is---NEED DATA----", fmt!(" %?",tok )) ;
>>>>>>> 39712dfcfc408342c7dc7a3037377a26268dc3cf
                               
                    }
                    break
                },
                _=>{}
            }

            tok= match(tokOpt)	{
               	Some(tok)=>tok,
               	None=> {
                    test1.info( ~"lexer",~"css_lexer.rs"  , ~"css__lexer_get_token", ~"test_lexer" , ~"token read is None") ;
                    break
                }
            };

<<<<<<< HEAD
            test1.info( ~"lexer",~"css_lexer.rs"  , ~"css__lexer_get_token", ~"test_lexer" , fmt!("token read is %?",tok )) ;
=======
           test1.info( ~"test_lexer_chunks.rs", copy external_argument, ~"file reading", ~"test_lexer" ,~"token read is", fmt!(" %?",tok )) ;
                	              	

					
			match(tok) {
				CSS_TOKEN_EOF  => { 
                    exit=true ;
                    break
                },
				_=>{}
			}					
>>>>>>> 39712dfcfc408342c7dc7a3037377a26268dc3cf
        }
	}


	dataBytes = r.read_bytes(fileLen as uint);
                
                
    let str1= str::from_bytes(dataBytes);
              
<<<<<<< HEAD
    lexer.lexer_append_data(dataBytes);
    lexer.data_done();
=======
    test1.info( ~"test_lexer_chunks.rs", copy external_argument, ~"file reading", ~"test_lexer" ,~"contents of file", fmt!("read data is %?", str1),~"") ;   
                
    lexer.lexer_append_data(dataBytes);
    let mut tok:css_token_type;
    while(true) {
        let (tokOpt,Errr)= lexer.css__lexer_get_token();
        match(Errr) {
            LEXER_NEEDDATA => {
                if tokOpt.is_some() {
                    tok= tokOpt.unwrap();
                    test1.info( ~"test_lexer_chunks.rs", copy external_argument, ~"file reading", ~"test_lexer" ,~"token read is---NEED DATA----", fmt!(" %?",tok )) ;
                               
                }
                break
            },
            _=>{}
        }
        
        tok= match(tokOpt) {
            Some(tok)=>tok,
            None=> break
        };
        test1.info( ~"test_lexer_chunks.rs", copy external_argument, ~"file reading", ~"test_lexer" ,~"token read is", fmt!(" %?",tok )) ;
                                    
>>>>>>> 39712dfcfc408342c7dc7a3037377a26268dc3cf


    let (tokOpt,Errr)= lexer.css__lexer_get_token();

    match(tokOpt)  {
        Some(tok)=>{
            test1.info( ~"lexer",~"css_lexer.rs"  , ~"css__lexer_get_token", ~"test_lexer" , fmt!("token read is %?",tok ));
            match tok {
                CSS_TOKEN_EOF => test1.pass( ~"lexer",~"css_lexer.rs"  , ~"css__lexer_get_token", ~"test_lexer" , ~"CSS_LEXER_PASSED"),
                _=> test1.fail( ~"lexer",~"css_lexer.rs"  , ~"css__lexer_get_token", ~"test_lexer" , ~"CSS_LEXER_FAILED")
            }
        },
        None=> test1.info( ~"lexer",~"css_lexer.rs"  , ~"css__lexer_get_token", ~"test_lexer" , ~"token read is None")
    };
} 
