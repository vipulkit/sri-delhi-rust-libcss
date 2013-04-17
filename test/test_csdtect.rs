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
	let mut num_skip_char : uint = 0;
	match args[1] {
		~"utf8.txt"  => { encoding = ~"UTF-8" ;
						   encsrcVal = CSS_CHARSET_DEFAULT;	
						   num_skip_char = 0;
							},
		~"utf16.txt"  => { encoding = ~"UTF-16LE";
							encsrcVal = CSS_CHARSET_DOCUMENT;
							num_skip_char = 2;
						 },
		~"utf32.txt"  => {	encoding = ~"UTF-32LE";
							encsrcVal = CSS_CHARSET_DOCUMENT;
							num_skip_char = 4;
							},
		_           =>  {	encoding = ~"" ; //Unknown File Format"
							encsrcVal = CSS_CHARSET_DICTATED;   // means ERROR
							}
	}


// Header of input file is  being skipped, intentionally
	let (inputStreamOption, ParserUtilsError) = lpu_inputstream(copy encoding, Some(~css__charset_extract));
    let r2 : @Reader = io::file_reader(&Path(copy args[1])).get();	    
	let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
	test1.info( ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"") ;

	io::println(" Test 1");

	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{			
			
			test1.info( ~"csdetect",~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"input stream created successfully") ;			
			let mut stream2 : ~lpu_inputstream = inputStreamOption.unwrap();


// // mibenum test
			


			let mut flagValue : int = 0;
			while !r2.eof() {				
     			let mut data : ~[u8]= r2.read_bytes(100);
     			let mut buffData : ~[u8];     	

			if flagValue == 0
				{
				if (data[0] == 0x00 && data[1] == 0x00 && 
						data[2] == 0xFE && data[3] == 0xFF) {
					data.tailn(4);
				} else if (data[0] == 0xFF && data[1] == 0xFE &&
						data[2] == 0x00 && data[3] == 0x00) {
					data.tailn(4);
				} else if (data[0] == 0xFE && data[1] == 0xFF) {
					data.tailn(2);
				} else if (data[0] == 0xFF && data[1] == 0xFE) {
					data.tailn(2);
				} else if (data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF) {
					data.tailn(3);
				}
				flagValue += 1;
			}

         		test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"file read successfully"+fmt!("%?",data)) ;
         		stream2.parserutils_inputstream_append(data);         		         		
         	
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

		},
		_=>{test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"input stream not created") ;}
	}

	let (inputStreamOption, ParserUtilsError) = lpu_inputstream(copy encoding, Some(~css__charset_extract));	

	// Header of input file is not being skipped
    let r : @Reader = io::file_reader(&Path(copy args[1])).get();	    
	let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
	test1.info( ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"") ;

	io::println(" Test 2");

	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{			
			
			test1.info( ~"csdetect",~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"input stream created successfully") ;			
			let mut stream : ~lpu_inputstream = inputStreamOption.unwrap();
			
		// mibenum test
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


			while !r.eof() {				
     			let mut data : ~[u8]= r.read_bytes(100);
     			let mut buffData : ~[u8];
     			
         		test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"file read successfully"+fmt!("%?",data)) ;
         		stream.parserutils_inputstream_append(data);         		         		

         	
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
	
	
}