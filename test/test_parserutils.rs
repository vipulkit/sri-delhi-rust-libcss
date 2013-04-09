//sandy
extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
use parserutils::* ;
use parserutils_inputstream::*;
// use libc::c_uint;
//use libc::size_t  ;
fn sss(data: ~[u8], smibenum:~u16, source:~u32) -> parserutils_error
{
	return PARSERUTILS_OK;
}

fn main() {
	
	let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"ASCII");
	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{
			io::println("test name:test_parserUtils>>file name:parserutils_input stream>> functn name>> lpu_inputstream");
			io::println("Pass");
			let mut stream:~lpu_inputstream = inputStreamOption.get();
			let mut data:~[u8]= ~[10,10,10,10,10,10/*,10,10,10,10,10*/];
	    	let mut data2:~[u8]= ~[10,10,10,10,10,10,10];
	    	stream.parserutils_inputstream_insert(data2);
            stream.parserutils_inputstream_append(data);
            let (charsetStringOption,charsetSource)= stream.parserutils_inputstream_read_charset();
            io::println(fmt!("%?,%?",charsetStringOption.get(),charsetSource));
            io::println("test name:test_parserUtils>>file name:parserutils_input stream>> functn name>> parserutils_inputstream_read_charset");
			io::println("Pass if valid values in above line");
			match(stream.parserutils_inputstream_change_charset(~"ISO-10646-UTF-1", 0))
			{
				PARSERUTILS_OK=>{
					io::println("test name:test_parserUtils>>file name:parserutils_input stream>> functn name>> parserutils_inputstream_change_charset");
					io::println("Pass");
						},
				_=>{
					io::println("test name:test_parserUtils>>file name:parserutils_input stream>> functn name>> parserutils_inputstream_change_charset");
					io::println("Fail");
				}
			}
			 let (charsetStringOption,charsetSource)= stream.parserutils_inputstream_read_charset();
            io::println(fmt!("%?,%?",charsetStringOption.get(),charsetSource));

          let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(0);
          let (array,length)= tuple.get();
          io::println(fmt!("%?,%?",array,length));
		}
		_=>{io::println("test name:test_parserUtils>>file name:parserutils_input stream>> functn name>> lpu_inputstream");
			io::println("failed");}
	}
	
	
}
