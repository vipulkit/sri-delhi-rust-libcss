extern mod std;

extern mod parserutils ; 
use parserutils::* ;
 fn main()
{
let args : ~[~str] = os::args();
    io::println(args[1]);
    let r:@Reader = io::file_reader(&Path(args[1])).get(); // r is result<reader, err_str>
    
    // r.read_line(); // skip line


    let mut parser : @mut lpu = lpu();
	//let mut asc: parserutils_inputstream_private_ptr;
	let pRslt= parser.parserutils_inputstream_create(~"ASCII",1);
	let mut stream:parserutils_inputstream;
	match(pRslt)
	{
		PARSERUTILS_INPUTSTREAM_CREATE_OK(temp)=>
		{ 
			// let mut data:@[u8]= @[10,10,10,10,10,10,10,10,10,10,10];
			// let mut dta2:@[u8]= @[10,10,10,10,10,10,10];
			stream = temp;
			let reader = io::stdin();
			let mut (ptr,length):(~[u8],uint)= (~[],0) ;
			while !r.eof() {
     			let line:&str=r.read_line();
     			let data:~[u8]= line.to_bytes();
         		io::println(line);
         		io::println(fmt!("%?",data));
         		reader.read_byte();
         		lpu::parserutils_inputstream_append(&mut stream,data);
         		//parser.print_inputstream(&mut stream);
         		io::println("Pass");
         		loop{
         			match(parser.parserutils_inputstream_peek(&mut stream,0))
					{
						PARSERUTILS_PEEK_OK(x,y)=>{
							ptr=x;
							length=y;
							parser.parserutils_inputstream_advance(& mut stream, length);
							io::println(fmt!("peek data->%?,%?",ptr,length));
							io::println("sandeep");
						},
						PARSERUTILS_NEEDDATA =>{break;}
						PARSERUTILS_EOF=>{break;}
						_=>{io::println("invalid");}
					}
					parser.print_inputstream(&mut stream);
					
					
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