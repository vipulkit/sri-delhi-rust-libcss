extern mod std;
extern mod wapcaplet;
extern mod test;


use std::arc;
use wapcaplet::*;
use test::*;


fn main() {
	let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));
	let module_name: ~str=~"wapcaplet";
	let  file_name : ~str=~"wapcaplet.rs";
	let mut function_name : ~str = ~"";
	let mut test_name : ~str=~"";
	let mut comment: ~str=~"";

	// test 1: Creating a lwc instance
	let mut lwc_instance = lwc();
	function_name = ~"lwc()";
	test_name = ~"Creating a lwc instance";
	comment = ~"lwc instance created";
	test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);



	do lwc_instance.write |l| {
		
		// test 2: interning a null string
		function_name = ~"lwc_intern_string";
		test_name = ~"interning a null string";
		comment = ~"null string interned";
		test_logger.info(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let  p = l.lwc_intern_string(~"");
		
		// test 3: interning a normal string
		function_name = ~"lwc_intern_string";
		test_name = ~"interning a normal string";
		comment = ~"string interned successfull";
		test_logger.info(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let q = l.lwc_intern_string(~"hellowapcaplet");
		
		// test 4: interning a sub string with correct offset and length
		function_name = ~"lwc_intern_substring";
		test_name = ~"interning a sub string of a lwc_string";
		comment = ~"internment of a sub string of lenght 5 from offset 2 in hellowapcaplet";
		test_logger.info(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let r = l.lwc_intern_substring(q ,2 , 5);
		
		// test 5: lwc_string_caseless_isequal of two same string in different case
		function_name = ~"lwc_string_caseless_isequal";
		test_name = ~"lwc_string_caseless_isequal of two same string in different case";
		comment = ~"returns true";
		test_logger.pass(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let s = l.lwc_intern_string(~"abc");
		let q = l.lwc_intern_string(~"aBc");
		let r = l.lwc_string_caseless_isequal(s , q);

		// test 6: lwc_string_caseless_isequal of two same string in same case
		function_name = ~"lwc_string_caseless_isequal";
		test_name = ~"lwc_string_caseless_isequal of two same string in same case";
		comment = ~"true";
		test_logger.pass(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let s = l.lwc_intern_string(~"abc");
		let q = l.lwc_intern_string(~"abc");
		let r = l.lwc_string_caseless_isequal(s , q);

		// test 7: ref count increase of a interned string
		function_name = ~"lwc_string_ref";
		test_name = ~"ref count increase of a already interned string";
		comment = ~"ref count increases";
		test_logger.pass(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let t = l.lwc_intern_string(~"abcdef");
		let r = l.lwc_string_ref(t);

		// test 8: ref count decrease of a interned string
		function_name = ~"lwc_string_unref";
		test_name = ~"ref count decrease of a interned string";
		comment = ~"ref count decreases";
		test_logger.pass(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let t = l.lwc_intern_string(~"abcdef");
		let r = l.lwc_string_unref(t);

		// test 9: ref count decrease of a interned string with ref count already 0
		function_name = ~"lwc_string_unref";
		test_name = ~"ref count decrease of a interned string with ref count already 0";
		comment = ~"this case handled : but should return some error or warning if tried";
		test_logger.info(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let t = l.lwc_intern_string(~"abcdef");
		l.lwc_string_unref(t.clone());
		l.lwc_string_unref(t.clone());

		// test 10: ref count decrease of a interned string with ref count already 0
		// function_name = ~"lwc_string_unref";
		// test_name = ~"ref count decrease of a interned string with ref count already 0";
		// comment = ~"this case handled : but should return some error or warning if tried";
		// test_logger.info(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		// let t = l.lwc_intern_string(~"abcdef");
		// l.lwc_string_unref(t.clone());
		// l.lwc_string_unref(t.clone());

		

		// test 10: internment of a sub string of lenght 5 from offset 2 in null string(slice with lenght or offset greater than actual length of string)
		function_name = ~"lwc_intern_substring";
		test_name = ~"internment of a sub string of lenght 5 from offset 2 in null string(slice with lenght or offset greater than actual length of string)";
		comment = ~"task fails: index out of bound";
		test_logger.fail(copy module_name , copy file_name , copy test_name , copy function_name , copy comment);
		let r = l.lwc_intern_substring(p ,2 , 5);

	}
}