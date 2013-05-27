
extern mod std;
extern mod css;
// mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;
//extern mod csdetect;

//use csdetect::*;
use test::*;
use parserutils::charset::aliases::*;
use parserutils::input::parserutils_filter::* ;
use parserutils::input::inputstream::*;
use parserutils::utils::errors::*;
use css::charset::csdetect::*;
//use parserutils_inputstream::*;
use core::str::*;
use std::arc;

fn main() {

	let args : ~[~str] = os::args(); 
	
	// value initialization
	let mut encoding : ~str;	
	let mut encsrcVal: int;
	let mut num_skip_char : uint = 0;	
	let mut external_argument : ~str = copy args[1];
	io::println(fmt!("value of external_argument is %?", external_argument));
	match args[1] {
		~"utf8.txt"  => {
						   encoding = ~"UTF-8" ;
						   encsrcVal = 0;	
						   num_skip_char = 0;
						   external_argument = ~"utf8.txt";
							},
		~"utf16.txt"  => {
							encoding = ~"UTF-16LE";
							encsrcVal = 3;
							num_skip_char = 2;
							external_argument = ~"utf16.txt";
						 },
		~"utf32.txt"  => {
							encoding = ~"UTF-32LE";
							encsrcVal = 3;
							num_skip_char = 4;
							external_argument = ~"utf32.txt";
							},
		_           =>  {	
							encoding = ~"UTF-8" ;
							encsrcVal = 0;   
							// any unknow encoding would be considered as UTF-8
						}
	}


	// Test 1: Header of input file is  being skipped	
	let (inputStreamOption, ParserUtilsError) = inputstream(Some(copy encoding),Some(encsrcVal), Some(~css__charset_extract));
	let r2 : @Reader = io::file_reader(&Path(copy args[1])).get();	    
	let mut test1 = result::unwrap(test_report(&"Unit_test_report.csv"));
	//test1.info( ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"") ;	

	match(ParserUtilsError)	{
		PARSERUTILS_OK=>{			
			
			test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"input stream created with default charset and css__charset_extract fn" ,~"input stream creation",~"input stream created successfully",~"") ;			
			let mut stream2 : ~inputstream = inputStreamOption.unwrap();
			let mut flagValue : int = 0;

			while !r2.eof() {				
				let mut data : ~[u8]= r2.read_bytes(100);
				let mut buffData : ~[u8];     	

				if flagValue == 0 {
					if (data[0] == 0x00 && data[1] == 0x00 && 
							data[2] == 0xFE && data[3] == 0xFF) {
						data.tailn(4);
					} 
					else if (data[0] == 0xFF && data[1] == 0xFE &&
							data[2] == 0x00 && data[3] == 0x00) {
						data.tailn(4);
					} 
					else if (data[0] == 0xFE && data[1] == 0xFF) {
						data.tailn(2);
					} 
					else if (data[0] == 0xFF && data[1] == 0xFE) {
						data.tailn(2);
					} 
					else if (data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF) {
						data.tailn(3);
					}
					flagValue += 1;
				}

				test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract",~"file reading->"+ copy args[1] , ~"file reading", fmt!("%?",data),~"") ;
				stream2.parserutils_inputstream_append(data);         		         		
			
				loop {
					let (tuple, parserutilsError) = stream2.parserutils_inputstream_peek(2);

					match(parserutilsError) {
						PARSERUTILS_OK=>{
							let mut(ptr,length)= tuple.get();
							stream2.parserutils_inputstream_advance(length);
							test1.info( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"read input stream chunk data and byte length per character " , fmt!("peek data->%?,%?",ptr,length),~"") ;
						},
						PARSERUTILS_NEEDDATA=> {break;},
						PARSERUTILS_EOF=> {break;},
						_ =>{
							test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"end of file reached" , ~"completes reading of input stream", ~"reading of input stream completed", ~"");
							break;
						}
					}
				}
			}

			 // mibenum test 
			match(arc::get(&stream2.input.instance).parserutils_charset_mibenum_to_name(stream2.mibenum)) {
				Some(x)  => {
								if eq(&x, &encoding.to_lower()){
									test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"check if the mibenum read by charset fn into input stream is ok ",copy encoding, x, ~"mibenum value") ;								
								}
								else{
									test1.fail( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract",~"check if the mibenum read by charset fn into input stream is ok ", copy encoding, x, ~"mibenum value") ;								
								}
							},
							
				 None     => test1.fail(  ~"test_csdtect.rs", copy external_argument, ~"parseUtils Filter",~"parserutils_filter.rs"  , ~"parserutils_charset_mibenum_to_name", copy args[1] , ~"Some", ~"None" ,~"") 
			}			

			// encsrc test
			match stream2.encsrc {
				encsrcVal  => {
									test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"check if the encsrc value read by charset fn into input stream is ok " ,fmt!("%?",encsrcVal),fmt!("%?",stream2.encsrc), ~"encsrc value");									
							}
			}

		},
		_   =>  {
			test1.fail( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"input stream created with default charset and css__charset_extract fn" ,~"input stream creation",~"input stream not created successfully",~"") ;	
		}
	}

	// Test 2: Header of input file is not being skipped
	let (inputStreamOption, ParserUtilsError) = inputstream(Some(copy encoding),Some(encsrcVal), Some(~css__charset_extract));		
	let r : @Reader = io::file_reader(&Path(copy args[1])).get();	    
	let mut test1 = result::unwrap(test_report(&"Unit_test_report.csv"));
	//test1.info( ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", copy args[1] , ~"") ;

	match(ParserUtilsError) {
		PARSERUTILS_OK=>{			
			
			test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract",~"input stream created with default charset and css__charset_extract fn" ,~"input stream creation",~"input stream created successfully",~"") ;						
			let mut stream : ~inputstream = inputStreamOption.unwrap();

			while !r.eof() {				
				let mut data : ~[u8]= r.read_bytes(100);
				let mut buffData : ~[u8];
				
				test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract",~"file reading->"+ copy args[1] , ~"file reading",fmt!("%?",data),~"") ;
				stream.parserutils_inputstream_append(data);         		         		

			
				loop{
					let (tuple, parserutilsError) = stream.parserutils_inputstream_peek(2);

					match(parserutilsError) {
						PARSERUTILS_OK=>{

							let mut(ptr,length)= tuple.get();
							stream.parserutils_inputstream_advance(length);
							test1.info( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", ~"read input stream chunk data and byte length per character " , fmt!("peek data->%?,%?",ptr,length),~"") ;
						},

						PARSERUTILS_NEEDDATA => {break;}
						PARSERUTILS_EOF => {break;}
						_=>{
							test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"end of file reached" , ~"completes reading of input stream", ~"reading of input stream completed", ~"");
							break;
						}
					}
				}
			}

			 // mibenum test
			match(arc::get(&stream.input.instance).parserutils_charset_mibenum_to_name(stream.mibenum)) {
				Some(x)  => {
								if eq(&x, &encoding.to_lower()){
								test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract", ~"check if the mibenum read by charset fn into input stream is ok ",copy encoding, x, ~"mibenum value") ;									
								}
								else{
								test1.fail(~"test_csdtect.rs", copy external_argument, ~"csdetect",~"csdetect.rs", ~"css__charset_extract",~"check if the mibenum read by charset fn into input stream is ok ",copy encoding, x, ~"mibenum value") ;								
								}
							},
				None     => test1.fail( ~"test_csdtect.rs", copy external_argument, ~"parseUtils Filter",~"parserutils_filter.rs"  , ~"parserutils_charset_mibenum_to_name", copy args[1] , ~"Some", ~"None", ~"") 
			}			

			// encsrc test
			match stream.encsrc {
			encsrcVal  => {
								test1.pass( ~"test_csdtect.rs", copy external_argument, ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", ~"check if the encsrc value read by charset fn into input stream is ok " ,fmt!("%?",encsrcVal),fmt!("%?",stream.encsrc), ~"encsrc value");								
							}
			}
		},
		_   =>  {
					test1.fail( ~"test_csdtect.rs", copy external_argument, ~"csdetect", ~"csdetect.rs", ~"css__charset_extract", ~"input stream created with default charset and css__charset_extract fn" ,~"input stream creation",~"input stream not created successfully",~"") ;	
		}
	}	
}