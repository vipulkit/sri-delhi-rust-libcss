
extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;

use test::*;
use parserutils::* ;
use parserutils_inputstream::*;

 fn main() {
	let args : ~[~str] = os::args();
    io::println(args[1]);
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let reader = io::stdin();
    let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
    let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"UTF-16",None);
	match(ParserUtilsError) {
		PARSERUTILS_OK=>{
			
			test1.pass( ~"parserutils",~"parserutils_inputStream.rs"  , ~"lpu_inputstream", ~"test_parserutils_inputstream.rs" , ~"input stream created successfully") ;
			let mut stream:~lpu_inputstream = inputStreamOption.unwrap();
			
			while !r.eof() {
				io::println("line");
     			
     			let data:~[u8]= r.read_bytes(100);
         		
         		test1.pass( ~"test_parserutils_inputstream.rs",~"test_parserutils_inputstream.rs"  , ~"file reader", ~"test_parserutils_inputstream.rs" , ~"file read successfully"+fmt!("%?",data)) ;
         		
         		stream.parserutils_inputstream_append(data);
         		
         		loop {
         			let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(2);
         			match(parserutilsError) {
						PARSERUTILS_OK=>{
							let mut(ptr,length)= tuple.get();
							stream.parserutils_inputstream_advance( length);
							test1.pass( ~"parserutils",~"parserutils_inputStream.rs"  , ~"parserutils_inputstream_peek", ~"test_parserutils_inputstream.rs" , fmt!("peek data->%?,%?",ptr,length)) ;
							
						},
						PARSERUTILS_NEEDDATA =>{break;}
						PARSERUTILS_EOF=>{break;}
						_=>{test1.pass( ~"parserutils",~"parserutils_inputStream.rs"  , ~"parserutils_inputstream_peek", ~"test_parserutils_inputstream.rs" , ~"invalid") ;break;}
					}					
         		}//end of loop				
	         }	        
		},
		_=>{test1.fail( ~"parserutils",~"parserutils_inputStream.rs"  , ~"lpu_inputstream", ~"test_parserutils_inputstream.rs" , ~"input stream not created successfully") ;}
	}
	test1.pass( ~"parserutils",~"parserutils_inputStream.rs"  , ~"whole functionality", ~"test_parserutils_inputstream.rs" , ~"PASS") ;      
}