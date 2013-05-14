
extern mod std;
//extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;

use test::*;
use parserutils::* ;
use parserutils::input::inputstream::*;
use parserutils::charset::alias::*;

 fn main() {
	let args : ~[~str] = os::args();
	let mut external_argument : ~str = copy args[1];
    io::println(fmt!("value of external_argument is %?", external_argument));
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let reader = io::stdin();
    let mut test1 = result::unwrap(test_report(&"Unit_test_report.csv"));
    let (inputStreamOption, ParserUtilsError)= inputstream(~"UTF-16",None);

	match(ParserUtilsError) {
		PARSERUTILS_OK=>{
			
			test1.pass( ~"test_parserutils_inputstream.rs", copy external_argument, ~"parserutils",~"parserutils_inputStream.rs"  , ~"lpu_inputstream", ~"input stream creation with UTF-16,None" , ~"input stream should be created",~"input stream created successfully",~"") ;
			let mut stream:~inputstream = inputStreamOption.unwrap();
			
			while !r.eof() {
				io::println("line");
     			
     			let data:~[u8]= r.read_bytes(100);
         		
         		test1.pass( ~"test_parserutils_inputstream.rs", copy external_argument, ~"test_parserutils_inputstream.rs",~"test_parserutils_inputstream.rs"  , ~"file reader", ~"file reads 100 bytes",~"file reads 100 bytes" , ~"file read successfully"+fmt!("%?",data),~"") ;
         		
         		stream.parserutils_inputstream_append(data);
         		
         		loop {
         			let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(2);
         			match(parserutilsError) {
						PARSERUTILS_OK=>{
							let mut(ptr,length)= tuple.get();
							stream.parserutils_inputstream_advance( length);
							test1.pass( ~"test_parserutils_inputstream.rs", copy external_argument, ~"parserutils",~"parserutils_inputStream.rs"  , ~"parserutils_inputstream_peek", ~"input stream reading with offset 2" ,~"input stream should be read" ,fmt!("peek data->%?,%?",ptr,length),~"") ;
							
						},
						PARSERUTILS_NEEDDATA =>{break;}
						PARSERUTILS_EOF=>{break;}
						_=>{test1.pass( ~"test_parserutils_inputstream.rs", copy external_argument, ~"parserutils",~"parserutils_inputStream.rs"  , ~"parserutils_inputstream_peek", ~"input stream reading with offset 2",~"end of file should be encountered"  , ~"end of file encountered",~"") ;break;}
					}					
         		}//end of loop				
	        }	        
		},
		_=>{test1.fail( ~"test_parserutils_inputstream.rs", copy external_argument, ~"parserutils",~"parserutils_inputStream.rs"  , ~"lpu_inputstream", ~"input stream creation with UTF-16,None" , ~"input stream should be created", ~"input stream not created successfully",~"") ;}
	}
	test1.pass( ~"test_parserutils_inputstream.rs", copy external_argument, ~"parserutils",~"parserutils_inputStream.rs"  , ~"whole functionality", ~"test_parserutils_inputstream.rs" ,~"",~"", ~"PASS") ;      
}