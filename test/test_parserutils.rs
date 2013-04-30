
extern mod std;
extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;
extern mod csdetect;
use csdetect::*;
use test::*;
use core::str::*;

use parserutils::* ;
use parserutils_inputstream::*;

fn main() {
	let mut test1 = result::unwrap(test_report(&"temp_log.csv"));
	let module_name: ~str=~"test _parserutils";
	let  file_name : ~str=~"parserutils_input stream";
	let mut function_name : ~str;
	let mut test_name : ~str=~"";
	let mut comment: ~str=~"";
	let (inputStreamOption, ParserUtilsError)= lpu_inputstream(~"ASCII" , None);
	match(ParserUtilsError)
	{
		PARSERUTILS_OK=>{
			function_name=~"lpu_inputstream";
			comment = ~"input stream created successfully";
			test1.pass( copy module_name, copy file_name  , function_name, test_name , comment) ;

			let mut stream:~lpu_inputstream = inputStreamOption.unwrap();
			let mut data:~[u8]= ~[10,10,10,10,10,10/*,10,10,10,10,10*/];
	    	let mut data2:~[u8]= ~[10,10,10,10,10,10,10];
	    	stream.parserutils_inputstream_insert(data2);
            stream.parserutils_inputstream_append(data);

            let (charsetStringOption,charsetSource)= stream.parserutils_inputstream_read_charset();
            test_name =fmt!("(%?,%?)",charsetStringOption.get(),charsetSource);
            function_name=~"parserutils_inputstream_read_charset";
			comment = ~"charset read successfully if valid value in tuple in this line";			
			test1.fail( copy module_name, copy file_name  , function_name, test_name , comment) ;

			test_name =~"";
            function_name=~"parserutils_inputstream_change_charset";
			let toCharset= ~"UTF-16";

            match(stream.parserutils_inputstream_change_charset(copy toCharset,CSS_CHARSET_DEFAULT))
			{
				PARSERUTILS_OK=>{
					comment = ~"charset changed successfully ";
					test1.pass( copy module_name, copy file_name  , function_name, test_name , comment) ;
						},
				_=>{
					comment = ~"charset not changed successfully ";
					test1.fail( copy module_name, copy file_name  , function_name, test_name , comment) ;
				}
			}
			 let (charsetStringOption,charsetSource)= stream.parserutils_inputstream_read_charset();
            //io::println(fmt!("%?,%?",charsetStringOption.get(),charsetSource));
            test_name =~"";
            function_name=~"parserutils_inputstream_change_charset";
			let toCharset= ~"UTF-16";
            if eq(&toCharset,&charsetStringOption.get()) {
            	comment = ~"charset changed successfully ";
				test1.pass( copy module_name, copy file_name  , function_name, test_name , comment) ;
			}
            else{
            	comment = ~"charset not changed successfully ";
				test1.fail( copy module_name, copy file_name  , function_name, test_name , comment)
			}
            let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(0);
            let (array,length)= tuple.get();
            io::println(fmt!("%?,%?",array,length));
            function_name=~"parserutils_inputstream_peek";
			comment = ~"peek read successfully if valid value in tuple in this line";
            test_name=fmt!("%?,%?",array,length);
            test1.fail( copy module_name, copy file_name  , function_name, test_name , comment) ;
				
		}
		_=>{
			function_name=~"lpu_inputstream";
			comment = ~"input stream creation failed";
            test_name=~"";
			test1.fail( copy module_name, copy file_name  , function_name, test_name , comment) ;
		}
	}
	
	
}
