extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
use parserutils::* ;
use parserutils_inputstream::*;
 fn main()
{
let args : ~[~str] = os::args();
    io::println(args[1]);
    let r:@Reader = io::file_reader(&Path(args[1])).get(); 
    let reader = io::stdin();

    let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"UTF-16",None);
	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{
			io::println("test name:test_parserUtils_inputStream>>file name:parserutils_input stream>> functn name>> lpu_inputstream");
			io::println("Pass");
			let mut stream:~lpu_inputstream = inputStreamOption.get();
			
			
			//let mut (ptr,length):(~[u8],uint)= (~[],0) ;
			while !r.eof() {
				io::println("line");
     			//let line:&str=r.read_bytes();
     			let data:~[u8]= r.read_bytes(100);
         		//io::println(line);
         		io::println("line");
         		io::println(fmt!("%?",data));
         		reader.read_byte();
         		stream.parserutils_inputstream_append(data);
         		//parser.print_inputstream(&mut stream);
         		io::println("Pass");
         		loop{
         			let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(2);
         			match(parserutilsError)
					{
						PARSERUTILS_OK=>{
							let mut(ptr,length)= tuple.get();
							stream.parserutils_inputstream_advance( length);
							io::println(fmt!("peek data->%?,%?",ptr,length));
							io::println("sandeep");
						},
						PARSERUTILS_NEEDDATA =>{break;}
						PARSERUTILS_EOF=>{break;}
						_=>{io::println("invalid");break;}
					}
					//parser.print_inputstream(&mut stream);
					
					
					reader.read_byte();
					reader.read_byte();

         		}//end of loop
				
	         }
	         /*
	         lpu::parserutils_inputstream_insert(&mut stream,"helloo".to_bytes());
	         io::println("Pass");
	         loop{
         			match(parser.parserutils_inputstream_peek(&mut stream,0))
					{
						PARSERUTILS_PEEK_OK(x,y)=>{
							ptr=x;
							length=y;
							parser.parserutils_inputstream_advance(& mut stream, length);
						},
						PARSERUTILS_NEEDDATA =>{break;}
						_=>{}
					}
					io::println(fmt!("%?,%?",ptr,length));
         		}//end of loop*/
		},
		_=>{}
	}
	io::println("Pass"); 
     
}