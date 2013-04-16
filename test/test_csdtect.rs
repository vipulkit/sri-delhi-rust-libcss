extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;
extern mod csdetect;
use csdetect::*;
use test::*;
use parserutils::* ;
use parserutils_inputstream::*;

fn main()
{
	let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"UTF-16",Some(~css__charset_extract));
	let args : ~[~str] = os::args();
    io::println(copy args[1]);    

    let r:@Reader = io::file_reader(&Path(copy args[1])).get();
	
	let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
	test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"") ;

	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{
			
			test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"input stream created successfully") ;
			let mut stream:~lpu_inputstream = inputStreamOption.unwrap();
			
			
			while !r.eof() {
				io::println("line");
     			let data:~[u8]= r.read_bytes(100);
         		test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"file read successfully"+fmt!("%?",data)) ;
         		stream.parserutils_inputstream_append(data);
         		loop{
         			let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(2);
         			match(parserutilsError)
					{
						PARSERUTILS_OK=>{
							let mut(ptr,length)= tuple.get();
							stream.parserutils_inputstream_advance( length);
							test1.info( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , fmt!("peek data->%?,%?",ptr,length)) ;
						},
						PARSERUTILS_NEEDDATA =>{break;}
						PARSERUTILS_EOF=>{break;}
						_=>{
							test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"invalid ");
							break;
						}
					}
         		}
	         }
		},
		_=>{test1.fail( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"input stream not created successfully") ;}
	}
	test1.pass( ~"csdetect",~"csdetect.rs"  , ~"css__charset_extract", copy args[1] , ~"") ; 
     
}