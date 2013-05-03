
extern mod std;
extern mod css_stylesheet;
extern mod css_enum;
extern mod test;

use css_stylesheet::*;
use css_enum::*;
use test::*;


fn main() {		


	// 1. css__selector_hash is created and instantiated.
	let css_selector_hash_instance = css__selector_hash_functionalities_test();	

	// Instatitiated as it is required for css_stylesheet
	let css_style_instance : @mut css_style = @mut css_style {					
							bytecode:~[9898]
	};  // random value

	let css_stylesheet_instance : @mut css_stylesheet = @mut css_stylesheet {
									selectors:css_selector_hash_instance,	// returned from: css__selector_hash_functionalities_test
									rule_count:0,
									rule_list:None,
									last_rule:None,
									disabled:false,
									url:~"URL",
									title:~"title",
									level:CSS_LEVEL_1,
									quirks_allowed:false,
									quirks_used:false,
									inline_style:false,
									cached_style:Some(css_style_instance),  	// An instance is created for verification
									string_vector:~[]
	};				

	// 2.
	let css_stylesheet_instance_1 = css__stylesheet_style_functionalities_test(css_stylesheet_instance);

	//3.
	let css_selector_instance = css__stylesheet_selector_functionalities_test(css_stylesheet_instance_1);

	//4.
	// test for Remaining Hash functions :
	css__selector_hash_functionalities_test_2(css_selector_instance, css_selector_hash_instance);

	//5. 																// stylesheet instance returned from css__stylesheet_selector_functionalities_test 
	let css_stylesheet_instance_2 = css_stylesheet_rule_functionalities_test(css_stylesheet_instance_1, css_selector_instance);

}// main function ends here


// 1.
fn css__selector_hash_functionalities_test() -> @mut css_selector_hash {

	let mut test_logger : ~test_report = result::unwrap(test_report(&"Unit_test_report.csv"));

	let css_selector_hash_instance = css_selector_hash::css__selector_hash_create();	

	if css_selector_hash_instance.default_slots == (1<<6){
		test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , int::to_str(1<<6), int::to_str(css_selector_hash_instance.default_slots as int), ~"default_slots value is correct");
	}
	else{
		test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , int::to_str(1<<6), int::to_str(css_selector_hash_instance.default_slots as int), ~"default_slots value is wrong");
	}

	return css_selector_hash_instance;
}

fn css__selector_hash_functionalities_test_2(css_selector_instance : @mut css_selector, css_selector_hash_instance : @mut css_selector_hash){

	let mut test_logger : ~test_report = result::unwrap(test_report(&"Unit_test_report.csv"));

	// Without inserting a selector trying to remove function
	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"CSS_OK",  ~"Without inserting a selector trying to remove"),
		   _    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"None CSS_OK", ~"Without inserting a selector trying to remove ")
	}

	// inserting a selector
	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~"CSS_OK", ~"CSS_OK", ~"inserting a selector"),
		   _    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~"CSS_OK", ~"None CSS_OK", ~"inserting a selector")
	}
	
	// After inserting data trying to find inserted value
	match(css_selector_hash_instance.css__selector_hash_find(~"name")){
		(None,y)     =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name", ~"Non zero value", ~"No value", ~"After inserting data trying to find inserted value"),
		(Some(x), y)  =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name", ~"Non zero value", ~"None zero value",~"After inserting data trying to find inserted value")
	}

	// Trying to find a value which doesn't exist in the Hash
	match(css_selector_hash_instance.css__selector_hash_find(~"RandomValue")){
		(None, y)     =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"RandomValue", ~"None", ~"None", ~"Trying to find a value which doesn't exist in the Hash"),
		(Some(x), y)  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"RandomValue", ~"None", ~"Some", ~"Trying to find a value which doesn't exist in the Hash")
	}

	// After inserting a selector anything trying to remove
	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance)){
		CSS_OK  =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"CSS_OK", ~"After inserting a selector anything trying to remove"),
		   _    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"Non CSS_OK", ~"After inserting a selector anything trying to remove")
	}

	// Finding an entry which has already beeen removed
	match(css_selector_hash_instance.css__selector_hash_find(~"name")){
		(None,y)     =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name" , ~"CSS_OK", ~"CSS_OK", ~" Finding an entry which has already beeen removed"),
		(Some(x), y)  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name", ~"CSS_OK", ~"CSS_OK", ~" Finding an entry which has already beeen removed")
	}

	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"CSS_OK", ~"Removing previously inserted data"),
		   _    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"Non CSS_OK", ~"Removing previously inserted data")
	}
}


// 2.
fn css__stylesheet_style_functionalities_test(css_stylesheet_instance : @mut css_stylesheet) -> @mut css_stylesheet {

	let mut test_logger = result::unwrap(test_report(&"Unit_test_report.csv"));
	let mut css_style_instance = @mut css_style {bytecode : ~[]};  // value is initialised					

	if (!css_stylesheet_instance.cached_style.is_none()){			
		css_style_instance = css_stylesheet_instance.cached_style.unwrap();			
	}
					 
	// A.			
	let css_style_value = css_stylesheet_instance.css__stylesheet_style_create();

	// Appending string data for proper formatting.
	let mut string_a : ~str = ~"";			
	css_style_value.bytecode.each_mut(|&elt| {				
	string_a.push_str(fmt!("%? ", elt));
	true
	});

	let mut string_b : ~str = ~"";			
	css_style_instance.bytecode.each_mut(|&elt| {				
		string_b.push_str(fmt!("%? ", elt));
		true
	});

						 					
	if string_a.eq(&string_b){	
	  	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"css_style_instance.bytecode", string_a, string_b,  ~"") ;
	}
	else{
	  	test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"css_style_instance.bytecode", string_a, string_b,  ~"") ;
	}
	

	// B.
	let css_style_source = @mut css_style {
									bytecode:~[11]};
	
    let mut b4Value : ~[u32] = copy css_style_value.bytecode + copy css_style_source.bytecode;

	css_stylesheet::css__stylesheet_merge_style(css_style_value, css_style_source); 

	let mut afterValue = copy css_style_value.bytecode;
				
	// Appending string data for proper formatting.
	let mut string_a : ~str = ~"";			
	b4Value.each_mut(|&elt| {				
		string_a.push_str(fmt!("%? ", elt));
		true
	});

	let mut string_b : ~str = ~"";			
	afterValue.each_mut(|&elt| {				
		string_b.push_str(fmt!("%? ", elt));
		true
	});

	
	if string_a.eq(&string_b){	
		  	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_merge_style", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	  }
    else{
	  	test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_merge_style", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	  }


	// C.
	let value : u32 = 1000; // random value for testing

	// b4Value is updated
	let mut b4Value = copy css_style_value.bytecode;
	b4Value.push(value);

	css_stylesheet::css__stylesheet_style_append(css_style_value, value);

	// afterValue is updated
	afterValue = copy css_style_value.bytecode;

	// Appending string data for proper formatting.
	let mut string_a : ~str = ~"";			
	b4Value.each_mut(|&elt| {				
		string_a.push_str(fmt!("%? ", elt));
		true
	});

	let mut string_b : ~str = ~"";			
	afterValue.each_mut(|&elt| {				
		string_b.push_str(fmt!("%? ", elt));
		true
	});

			 
	if string_a.eq(&string_b){
	  	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_append", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	 }
	 else{
	  	test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_append", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	 }

	// D.
	let value : &[u32] = &[2000, 90, 100]; // random value for testing

	// buffValue is updated
	let mut b4Value = copy css_style_value.bytecode;
	b4Value += value;
	css_stylesheet::css__stylesheet_style_vappend(css_style_value, value);

	// afterValue is updated
	afterValue = copy css_style_value.bytecode;

	// Appending string data for proper formatting.
	let mut string_a : ~str = ~"";			
	b4Value.each_mut(|&elt| {				
		string_a.push_str(fmt!("%? ", elt));
		true
	});

	let mut string_b : ~str = ~"";			
	afterValue.each_mut(|&elt| {				
		string_b.push_str(fmt!("%? ", elt));
		true
	});


	if afterValue.eq(&b4Value){
		  	test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_vappend", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	}
	else{
		  	test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_vappend", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	}

	// Updating the values
	css_stylesheet_instance.cached_style = Some(css_style_value);
	return css_stylesheet_instance;

} // test_css__stylesheet_style_functionalities ends here


// 3.
fn css__stylesheet_selector_functionalities_test(css_stylesheet_instance : @mut css_stylesheet) -> @mut css_selector {

	let mut test_logger = result::unwrap(test_report(&"Unit_test_report.csv"));

	let css_qname_instance = css_qname{
		name: ~"name",
		ns: ~"ns"
	};
			
	// A. 
	let mut buff_qname = copy css_qname_instance;
	let css_selector_instance = css_stylesheet_instance.css__stylesheet_selector_create(css_qname_instance);			 			

	match(css_selector_instance.combinator) {
		Some (x) =>   test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"", ~"No value" , ~"Some value", ~"combinator value is wrong"),
		None     =>   test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"", ~"No value" , ~"No value", ~"combinator value is correct")
	}

	match(css_selector_instance.rule) {
		Some (x) =>   test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"No value" , ~"Some value", ~"rule value is wrong"),
		None     =>   test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"No value" , ~"No value", ~"rule value is correct")
	}

	if css_selector_instance.specificity != CSS_SPECIFICITY_D{
		test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"CSS_SPECIFICITY_D", fmt!("%?", copy css_selector_instance.specificity), ~"specificity value is wrong");
	}
	else {
		test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" ,  ~"CSS_SPECIFICITY_D", fmt!("%?", copy css_selector_instance.specificity), ~"specificity value is correct");
	}

	let css_selector_data = copy css_selector_instance.data;
		 
	if (css_selector_data[0].qname.name == buff_qname.name) && (css_selector_data[0].qname.ns == buff_qname.ns){
		test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , fmt!("%?", copy buff_qname.name), fmt!("%?", copy css_selector_data[0].qname.name), ~"css_selector_data.qname is correct");	
	}
	else {
		test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , fmt!("%?", copy buff_qname.name), fmt!("%?", copy css_selector_data[0].qname.name), ~"css_selector_data.qname is wrong");		
	}
			 
	match(css_selector_data[0].selector_type) {
		CSS_SELECTOR_ELEMENT  =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"", ~"CSS_SELECTOR_ELEMENT", fmt!("%?", copy css_selector_data[0].selector_type), ~"css_selector_data.selector_type value is correct"),
		   _                  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"", ~"CSS_SELECTOR_ELEMENT", fmt!("%?", copy css_selector_data[0].selector_type), ~"css_selector_data.selector_type value is wrong")
	}


	match(css_selector_data[0].combinator_type) {
		CSS_COMBINATOR_NONE  =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"CSS_COMBINATOR_NONE", ~"CSS_COMBINATOR_NONE", ~"css_selector_data.combinator_type value is correct"),
		   _                  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" ,~"CSS_COMBINATOR_NONE", ~"Non CSS_COMBINATOR_NONE", ~"css_selector_data.combinator_type value is wrong")
	}

	match(css_selector_data[0].value_type) {
		CSS_SELECTOR_DETAIL_VALUE_STRING  =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"CSS_SELECTOR_DETAIL_VALUE_STRING", ~"CSS_SELECTOR_DETAIL_VALUE_STRING", ~"css_selector_data.value_type value is correct"),
				   _                  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"CSS_SELECTOR_DETAIL_VALUE_STRING", ~"Non CSS_SELECTOR_DETAIL_VALUE_STRING", ~"css_selector_data.value_type value is wrong")
	}

	 
	// B.

	let mut (css_result_value, OptValue) = css_stylesheet::css__stylesheet_selector_detail_init(CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
	None, None, false);

	match(css_result_value){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~"Non CSS_OK", ~"")	
	}

	let mut (css_result_value, OptValue) = css_stylesheet::css__stylesheet_selector_detail_init(CSS_SELECTOR_ID, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
	Some(~"RandomValue"), None, false);

	match(css_result_value){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"", ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~"Non CSS_OK", ~"")	
	}

	let (css_result_value, OptValue) = css_stylesheet::css__stylesheet_selector_detail_init(CSS_SELECTOR_ATTRIBUTE, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_NTH,
	Some(~"RandomValue"), None, false);

	match(css_result_value){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~"Non CSS_OK",~"")	
	}

	let mut (css_result_value, OptValue) = css_stylesheet::css__stylesheet_selector_detail_init(CSS_SELECTOR_ATTRIBUTE_PREFIX, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_NTH,
	None, Some((11, 22)), false);

	match(css_result_value){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"CSS_OK", ~" Non CSS_OK", ~"")	
	}

	  // C.	
	  // Note: Herein, css_selector_instance is getting one new value pushed.

	  // Commenting for quickfix of compilation error.	
	 /*   
  	match(css_stylesheet::css__stylesheet_selector_append_specific(css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
		None, None, false, CSS_COMBINATOR_NONE)){
		CSS_OK => test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~" CSS_SELECTOR_DETAIL_VALUE_STRING" , ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"CSS_SELECTOR_DETAIL_VALUE_STRING" , ~"CSS_OK", ~"Non CSS_OK", ~"")	
		}

	match(css_stylesheet::css__stylesheet_selector_append_specific(css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
		Some(~"RandomValue"), None, false, CSS_COMBINATOR_NONE)){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"CSS_SELECTOR_DETAIL_VALUE_STRING", ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"CSS_SELECTOR_DETAIL_VALUE_STRING", ~"CSS_OK", ~"Non CSS_OK", ~"")	
		}

	match(css_stylesheet::css__stylesheet_selector_append_specific(css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_NTH,
		None, Some((1,2)), false, CSS_COMBINATOR_NONE)){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"CSS_SELECTOR_DETAIL_VALUE_NTH" , ~"CSS_OK", ~"CSS_OK", ~""),
		 _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"CSS_SELECTOR_DETAIL_VALUE_NTH" , ~"CSS_OK", ~"Non CSS_OK", ~"")	
		}					
	*/
	
	// D.						
	// For testing below mentioned function            
	let css_qname_instance_New = css_qname{
										name: ~"New_name",
										ns: ~"New_ns"	
	};	

	let  mut css_selector_detail_instance_New = @mut css_selector_detail {
													qname:css_qname_instance_New, 
													selector_type:CSS_SELECTOR_CLASS, 
													combinator_type:CSS_COMBINATOR_NONE, 
													value_type:CSS_SELECTOR_DETAIL_VALUE_STRING, 
													negate: false,	
													string: Some(~"RoughData"),	
													a: 0,
													b: 0 
	};	

	let css_selector_instance_rough =  @mut css_selector {
											combinator:None,	
											rule: None,		
											specificity:0,
											data:~[css_selector_detail_instance_New]};	

	match(css_stylesheet::css__stylesheet_selector_combine(CSS_COMBINATOR_ANCESTOR, css_selector_instance, css_selector_instance_rough)){
		CSS_OK => test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_combine", ~"", ~"CSS_OK", ~"CSS_OK", ~""),
		_  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_combine", ~"",~"CSS_OK", ~"Non CSS_OK", ~"")	
	}
	
	return css_selector_instance;
}


fn css_stylesheet_rule_functionalities_test(css_stylesheet_instance : @mut css_stylesheet, css_selector_instance : @mut css_selector) -> @mut css_stylesheet {

	let mut test_logger = result::unwrap(test_report(&"Unit_test_report.csv"));
	let css_style_instance = css_stylesheet_instance.cached_style.unwrap();

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_UNKNOWN);
	match(retVal){
		RULE_UNKNOWN(x)    => {																		
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_BADPARM   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_UNKNOWN", ~"CSS_BADPARM", ~"CSS_BADPARM", ~""),							
					 _        =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_UNKNOWN", ~"CSS_BADPARM", ~"non CSS_BADPARM", ~"")
				}
			
			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule without parent rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule without parent rule")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule with parent rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule with parent rule")
			}	

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_UNKNOWN", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

		},	
		  _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_UNKNOWN", ~"RULE_UNKNOWN(x)", ~"Non RULE_UNKNOWN(x)", ~"")

	}

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_SELECTOR);
	match(retVal){
		RULE_SELECTOR(x)    => {
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~""),							
					 _   =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"")							
			}
			
			match(css_stylesheet_instance.css__stylesheet_rule_append_style(retVal, css_style_instance)){
				CSS_BADPARM   =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_SELECTOR" , ~"Non CSS_BADPARM", ~"CSS_BADPARM", ~""),							
					 _        =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_SELECTOR" , ~"Non CSS_BADPARM", ~"Non CSS_BADPARM", ~"")	
			}

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}	

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule")
			}

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_SELECTOR", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}
			
		},	
		  _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_SELECTOR", ~"RULE_SELECTOR(x)", ~"Non RULE_SELECTOR(x)", ~"")
		
	}

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_CHARSET);
	match(retVal){
		RULE_CHARSET(x)    => {
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_BADPARM   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_CHARSET", ~"CSS_BADPARM", ~"CSS_BADPARM", ~""),							
					 _        =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_CHARSET",  ~"CSS_BADPARM", ~"Non CSS_BADPARM", ~"")							
			}

			match(css_stylesheet::css__stylesheet_rule_set_charset(retVal, ~"testValue")){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_charset", ~"RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~""),							
					 _        =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_charset", ~"RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}	

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}

			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}	

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule")
			}

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_CHARSET", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}									

		},
		_  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_CHARSET", ~"RULE_CHARSET(x)", ~"Non RULE_CHARSET(x)", ~"")
		
	}

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_IMPORT);
	match(retVal){
		RULE_IMPORT(x)    => {
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_BADPARM   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_IMPORT",  ~"CSS_BADPARM", ~"CSS_BADPARM", ~""),							
					 _        =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_IMPORT",  ~"CSS_BADPARM", ~"Non CSS_BADPARM", ~"")							
			}
																	// roughURL and 1234 are random value for testing			
			match(css_stylesheet::css__stylesheet_rule_set_nascent_import(retVal, ~"roughURL", 1234)){
				CSS_OK	   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_nascent_import", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~""),							
					 _     =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_nascent_import", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"")							
			}		

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}

			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}		

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule")
			}

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_IMPORT", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

		},	
		  _  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_IMPORT", ~"RULE_IMPORT(x)", ~"Non RULE_IMPORT(x)", ~"")				
	}

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_MEDIA);
	match(retVal){
		RULE_MEDIA(x)    => {
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_MEDIA",  ~"CSS_OK", ~"CSS_OK", ~""),							
					 _    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"")							
			}	
														// 4321 is a random value for testing			
			match(css_stylesheet::css__stylesheet_rule_set_media(retVal, 4321)){
				CSS_OK   	=>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_media", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~""),							
					 _      =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_media", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~"")							
			}	

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}

			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}
			
			// Isn't it crashing
			// Commented, so that, it won't affect the execution flow
			/*									
			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule")
			}*/

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_MEDIA", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

		},	

		_ =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_MEDIA", ~"RULE_MEDIA(x)", ~"Non RULE_MEDIA(x)", ~"")
		
	}

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_FONT_FACE);
	match(retVal){
		RULE_FONT_FACE(x)    => {
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_BADPARM   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_FONT_FACE", ~"CSS_BADPARM", ~"CSS_BADPARM", ~""),							
					 _        =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_FONT_FACE", ~"CSS_BADPARM", ~"Non CSS_BADPARM", ~"")							
			}

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}

			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}
			
			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule")
			}

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_FONT_FACE", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

		},	
		_ =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_FONT_FACE", ~"RULE_FONT_FACE(x)", ~"Non RULE_FONT_FACE(x)", ~"")
		
	}

	let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_PAGE);
	match(retVal) {
		RULE_PAGE(x) => {
			match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),
				_  =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~""),
			}
							
			match(css_stylesheet::css__stylesheet_rule_set_page_selector(retVal, css_selector_instance)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_page_selector", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),							
				_   =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_page_selector", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"")							
			}
			
			let css_style_instance = css_stylesheet_instance.cached_style.unwrap();
			match(css_stylesheet_instance.css__stylesheet_rule_append_style(retVal, css_style_instance)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),							
				_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"")							
			}						
			
			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule before _add_rule"),							
			_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule before _add_rule")
			}

			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, None)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),							
			_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rsCSS_RULE_PAGE", ~"css__stylesheet_add_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}

			match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, retVal, Some(retVal))){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"")
			}		

			match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~"_remove_rule after _add_rule"),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"_remove_rule after _add_rule")
			}

			// _add_selectors and _remove_selectors
			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}

			match(css_stylesheet_instance._add_selectors(retVal)){
				CSS_OK   =>  test_logger.pass( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"CSS_OK", ~""),							
					_    =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"CSS_RULE_PAGE", ~"CSS_OK", ~"Non CSS_OK", ~"")										
			}									

		},	
	 	_ =>  test_logger.fail( ~"test_css_stylesheet.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_PAGE", ~"RULE_PAGE(x)", ~"Non RULE_PAGE(x)", ~"")
			
	}

	css_stylesheet_instance.cached_style = Some(css_style_instance);

	return css_stylesheet_instance;
}