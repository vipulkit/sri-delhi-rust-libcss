
extern mod std;
//extern mod parserutils_inputstream;
extern mod parserutils ; 
extern mod test;
//extern mod csdetect;
//use csdetect::*;
use test::*;
use core::str::*;

use parserutils::* ;
//use parserutils_inputstream::*;
use parserutils::input::inputstream::*;
use parserutils::charset::csdetect::*;
use parserutils::charset::aliases::*;

fn main() {
	let mut test1 = result::unwrap(test_report(&"Unit_test_report.csv"));
	let module_name: ~str=~"test _parserutils";
	let  file_name : ~str=~"parserutils_input stream";
	let mut function_name : ~str;
	let mut test_name : ~str=~"";
	let mut comment: ~str=~"";
	let mut external_argument : ~str = ~"";

	let (inputStreamOption, ParserUtilsError)= inputstream(~"US-ASCII" , None);

	match(ParserUtilsError)	{
		PARSERUTILS_OK=>{
			function_name=~"lpu_inputstream";
			test1.pass( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, ~"inputstream with parameters US-ASCII and NONE",~"input stream should be created",~"input stream created successfully", ~"") ;

			let mut stream:~inputstream = inputStreamOption.unwrap();
			let mut data:~[u8]= ~[10,10,10,10,10,10];
	    	let mut data2:~[u8]= ~[10,10,10,10,10,10,10];
	    	stream.parserutils_inputstream_insert(data2);
            stream.parserutils_inputstream_append(data);

            let (charsetStringOption,charsetSource)= stream.parserutils_inputstream_read_charset();
            test_name =fmt!("(%?,%?)",copy charsetStringOption.get(),charsetSource);
            function_name=~"parserutils_inputstream_read_charset";
			comment = ~"charset read successfully if valid value in tuple in this line";			
			//test1.info( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, ~"US-ASCII",test_name , comment) ;

			test_name =~"";
            function_name=~"parserutils_inputstream_change_charset";
			let toCharset= ~"UTF-16";

            match(stream.parserutils_inputstream_change_charset(copy toCharset,CSS_CHARSET_DEFAULT)) {
				PARSERUTILS_OK=>{
					comment = ~"charset changed successfully ";
					test1.pass( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, copy toCharset+~":checking return value" , ~"PARSERUTILS_OK",~"PARSERUTILS_OK",comment) ;
						},
				  _  => {
					comment = ~"charset not changed successfully ";
					test1.fail( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, copy toCharset +~":checking return value" , ~"PARSERUTILS_OK",~"none PARSERUTILS_OK", comment) ;
				}
			}
			let (charsetStringOption,charsetSource)= stream.parserutils_inputstream_read_charset();
            
            test_name =~"";
            function_name=~"parserutils_inputstream_change_charset";
			let toCharset= ~"UTF-16";
            if eq(&toCharset,& (copy charsetStringOption).get()) {
            	comment = ~"charset changed successfully ";
				test1.pass( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, test_name ,copy toCharset, charsetStringOption.get(),comment) ;
			}
            else{
            	comment = ~"charset not changed successfully ";
				test1.fail( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, test_name , copy toCharset, charsetStringOption.get(),comment)
			}
            let (tuple,parserutilsError)=stream.parserutils_inputstream_peek(0);
           
           let (array, length):(~[u8],uint)= (~[], 0);
            function_name=~"parserutils_inputstream_peek";
			comment = ~"peek read successfully if valid value in tuple in this line";
            test_name = ~"testing peek ";//fmt!("%?,%?",array,length);
            match(parserutilsError) {
            	PARSERUTILS_OK	=>	{
            		test1.pass( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, test_name, ~"PARSERUTILS_OK",~"PARSERUTILS_OK", comment);
             	},
             	_ => test1.fail( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, test_name ,~"PARSERUTILS_OK",~"None PARSERUTILS_OK", comment) 
            }
           
				
		}
		_=>{
			function_name=~"lpu_inputstream";
			comment = ~"input stream creation failed";
            test_name=~"";
			test1.fail( ~"test_parserutils.rs",copy external_argument, copy module_name, copy file_name  , function_name, ~"inputstream with parameters US-ASCII and NONE",~"input stream should be created",~"input stream not created successfully", ~"") ;
		}
	}
	
	
}
