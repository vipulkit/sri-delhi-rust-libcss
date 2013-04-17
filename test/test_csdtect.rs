extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;
extern mod csdetect;
use csdetect::*;
use test::*;
use parserutils::* ;
use parserutils_inputstream::*;
use core::str::*;
use std::arc;

fn main()
{
	let args : ~[~str] = os::args(); 

    // value initialization
	let mut encoding : ~str;	
	let mut encsrcVal: css_charset_source;
	match args[1] {
		~"utf8.txt"  => { encoding = ~"UTF-8" ;
						   encsrcVal = CSS_CHARSET_DEFAULT;	
							},
		~"utf16.txt"  => { encoding = ~"UTF-16";
							encsrcVal = CSS_CHARSET_DOCUMENT;
						 },
		~"utf32.txt"  => {	encoding = ~"UTF-32";
							encsrcVal = CSS_CHARSET_DOCUMENT;
							},
		_           =>  {	encoding = ~"" ; //Unknown File Format"
							encsrcVal = CSS_CHARSET_DICTATED;   // means ERROR
							}
	}

	let (inputStreamOption, ParserUtilsError) = lpu_inputstream(copy encoding, Some(~css__charset_extract));	

    let r : @Reader = io::file_reader(&Path(copy args[1])).get();	
    
	let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
	test1.info( ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"") ;

	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{			
			
			test1.info( ~"csdetect",~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"input stream created successfully") ;			
			let mut stream : ~lpu_inputstream = inputStreamOption.unwrap();
			
			while !r.eof() {				
     			let mut data : ~[u8]= r.read_bytes(100);
     			let mut buffData : ~[u8];
     			
         		test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"file read successfully"+fmt!("%?",data)) ;
         		stream.parserutils_inputstream_append(data);         		         		


         	// // mibenum test
			match(arc::get(&stream.input.lpu_instance).parserutils_charset_mibenum_to_name(stream.mibenum))
			{
				Some(x)  => {
								if eq(&x, &encoding){
								test1.pass( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"mibenum value") ;								
								}
								else{
								test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"mibenum value") ;								
								}
							},
				None     => test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"mibenum value") 
			}			

			// encsrc test
			match stream.encsrc
			{
				encsrcVal  => {
									test1.pass( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"encsrc value");									
							}
			}

         		loop{
         			let (tuple, parserutilsError) = stream.parserutils_inputstream_peek(2);

         			match(parserutilsError)
					{
						PARSERUTILS_OK=>{

							let mut(ptr,length)= tuple.get();
							stream.parserutils_inputstream_advance(length);
							test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , fmt!("peek data->%?,%?",ptr,length)) ;
						},

						PARSERUTILS_NEEDDATA => {break;}
						PARSERUTILS_EOF => {break;}
						_=>{
							test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"invalid ");
							break;
						}
					}
         		}
	         }
		},
		_=>{test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"input stream not created") ;}
	}
	

	let (inputStreamOption, ParserUtilsError) = lpu_inputstream(copy encoding, Some(~css__charset_extract));
    let r2 : @Reader = io::file_reader(&Path(copy args[1])).get();	    
	let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
	test1.info( ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"") ;

	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{			
			
			test1.info( ~"csdetect",~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"input stream created successfully") ;			
			let mut stream2 : ~lpu_inputstream = inputStreamOption.unwrap();

			let mut flagValue : int = 0;
			while !r2.eof() {				
     			let mut data : ~[u8]= r.read_bytes(100);
     			let mut buffData : ~[u8];     			

     			// skipping first line intentionally
     			if flagValue == 0
     			{     				     			     			
     			buffData = vec::from_slice(data.tailn(20));       			
     			data = buffData;
     			flagValue += 1;
     			}

         		test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"file read successfully"+fmt!("%?",data)) ;
         		stream2.parserutils_inputstream_append(data);         		         		


         	// // mibenum test
			match(arc::get(&stream2.input.lpu_instance).parserutils_charset_mibenum_to_name(stream2.mibenum))
			{
				Some(x)  => {
								if eq(&x, &encoding){
								test1.pass( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"mibenum value") ;								
								}
								else{
								test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"mibenum value") ;								
								}
							},
				None     => test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"mibenum value") 
			}			

			// encsrc test
			match stream2.encsrc
			{
				encsrcVal  => {
									test1.pass( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"encsrc value");									
							}
			}

         		loop{
         			let (tuple, parserutilsError) = stream2.parserutils_inputstream_peek(2);

         			match(parserutilsError)
					{
						PARSERUTILS_OK=>{

							let mut(ptr,length)= tuple.get();
							stream2.parserutils_inputstream_advance(length);
							test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , fmt!("peek data->%?,%?",ptr,length)) ;
						},

						PARSERUTILS_NEEDDATA => {break;}
						PARSERUTILS_EOF => {break;}
						_=>{
							test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"invalid ");
							break;
						}
					}
         		}
	         }
		},
		_=>{test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"input stream not created") ;}
	}

}