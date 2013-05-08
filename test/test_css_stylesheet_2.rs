
extern mod std;
extern mod css_stylesheet;
extern mod css_enum;
extern mod test;

use css_stylesheet::*;
use css_enum::*;
use test::*;


fn main(){

	let mut test_logger : ~test_report = result::unwrap(test_report(&"Unit_test_report.csv"));

	// 1. css__selector_hash is created and instantiated.
	let css_selector_hash_instance = css__selector_hash_functionalities_test();	

	// showing content of created hash_instance
	/*
	let mut l_data : ~str = ~" Current status of css_selector_hash_instance is:  ";
	let mut r_data : &str = fmt!("%?", css_selector_hash_instance);
	str::append(l_data, r_data);	
	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , ~"", l_data);
	*/

	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , ~"", ~"Current state of hash_instance is shown below");
	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , ~"", fmt!("%?", css_selector_hash_instance));

	// Instatitiated, as it is required for css_stylesheet
	let css_style_instance : @mut css_style = @mut css_style {					
							bytecode:~[1234]  // random value for testing
	}; 

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
									cached_style:Some(css_style_instance),  // An instance is created for verification
									string_vector:~[]
	};				

	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"", ~"" , ~"", ~"Current state of css_stylesheet_instance is shown below");
	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"", ~"" , ~"", fmt!("%?", css_stylesheet_instance));

	// 2.
	let css_stylesheet_instance_1 = css__stylesheet_style_functionalities_test(css_stylesheet_instance);

	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"", ~"" , ~"", ~"Updated state of css_stylesheet_instance is shown below");
	test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet", ~"css_stylesheet.rs", ~"", ~"" , ~"", fmt!("%?", css_stylesheet_instance_1));	

	// test for Remaining Hash functions :
	css__selector_hash_functionalities_test_3(css_selector_hash_instance);
}

// 1.
fn css__selector_hash_functionalities_test() -> @mut css_selector_hash {

	let mut test_logger : ~test_report = result::unwrap(test_report(&"Unit_test_report.csv"));

	let css_selector_hash_instance = css_selector_hash::css__selector_hash_create();	

	// verify by comparing one specific component of the structure, namely default_slots
	if css_selector_hash_instance.default_slots == (1<<6){
		test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , int::to_str(1<<6), int::to_str(css_selector_hash_instance.default_slots as int), ~"default_slots value is correct");
	}
	else{
		test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"" , int::to_str(1<<6), int::to_str(css_selector_hash_instance.default_slots as int), ~"default_slots value is wrong");
	}

	return css_selector_hash_instance;
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
	  	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"css_style_instance.bytecode", string_a, string_b,  ~"") ;
	}
	else{
	  	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"css_style_instance.bytecode", string_a, string_b,  ~"") ;
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
		  	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_merge_style", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	  }
    else{
	  	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_merge_style", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
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
	  	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_append", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	 }
	 else{
	  	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_append", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
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
		  	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_vappend", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	}
	else{
		  	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_vappend", ~"css_style_instance.bytecode", string_a, string_b, ~"") ;
	}

	// Updating the values
	css_stylesheet_instance.cached_style = Some(css_style_value);
	return css_stylesheet_instance;

} // test_css__stylesheet_style_functionalities ends here


fn css__selector_hash_functionalities_test_3(css_selector_hash_instance : @mut css_selector_hash){

	let mut test_logger : ~test_report = result::unwrap(test_report(&"Unit_test_report.csv"));	
	
	// A random instance of css_selector is created	
	let css_qname_instance_1 = css_qname{
		name: ~"name_1",
		ns: ~"ns_1"
	};

	// create a new selector altogether
	let css_selector_detail_instance_1 : @mut css_selector_detail = @mut css_selector_detail {
										qname: copy css_qname_instance_1,      	
										selector_type:CSS_SELECTOR_ID,    
										combinator_type:CSS_COMBINATOR_SIBLING,    
										value_type:CSS_SELECTOR_DETAIL_VALUE_NTH, 
										negate: true,			

										//css_selector_detail_value - union merged
										string:None,
										a: 20,
										b: 300
									};

    let css_selector_instance_1 : @mut css_selector = @mut css_selector {
								combinator:None,  
								rule:None,		
								specificity:5000,
								data:~[css_selector_detail_instance_1]
							};

	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance_1)){
		CSS_OK	=>	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"Non CSS_OK", ~"CSS_OK", ~"Without inserting a selector trying to remove "),
		   _    =>  test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"Non CSS_OK", ~"Non CSS_OK",  ~"Without inserting a selector trying to remove")
	}

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_1)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"CSS_OK", ~"Inserting a selector "),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Inserting a selector")
	}

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_1)){
		CSS_OK	=>	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"non CSS_OK", ~"CSS_OK",  ~"Inserting duplicate selector"),
		   _    =>  test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"non CSS_OK", ~"non CSS_OK", ~"Inserting duplicate selector "),
	}

	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance_1)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"CSS_OK", ~"Removing an inserted selector"),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Removing an inserted selector")
	}

	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance_1)){
		CSS_OK	=>	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"Non CSS_OK", ~"CSS_OK", ~"Removing an inserted selector twice"),
		   _    =>  test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"Non CSS_OK", ~"Non CSS_OK",  ~"Removing an inserted selector twice")
	}

	// Inserting this value once again and then going to verify "find" function
	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_1)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"CSS_OK", ~"Inserting a selector "),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Inserting a selector")
	}

	// Verification of find function
	let (hashEntryOption, resultData) = css_selector_hash_instance.css__selector_hash_find(~"New_name");
	match(resultData){
		CSS_OK	=>	{						
						match (hashEntryOption) {
							Some(val) => {
											test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"New_name" , ~"Non CSS_OK", ~"CSS_OK", ~"New_name: a value which doesn't exist");
											test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name_1" , ~"", fmt!("name_1 is found at %?", val));
										},		
							None      => test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"New_name" , ~"Non CSS_OK", ~"Non CSS_OK", ~"New_name: a value which doesn't exist")
						}
					},
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"New_name" , ~"CSS_OK", ~"None CSS_OK", ~"New_name: a value which doesn't exist")
	}	

	// Verification of find function
	let (hashEntryOption, resultData) = css_selector_hash_instance.css__selector_hash_find(~"name_1");
	match(resultData){
		CSS_OK	=>	{						
						match (hashEntryOption) {
							Some(val) =>{
											test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name_1" , ~"CSS_OK", ~"CSS_OK", ~"name_1: a value which exists in the hash"); 
											test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name_1" , ~"", fmt!("name_1 is found at %?", val));
										},	
							None      => test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
						}
					},
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
	}

	// Verification of css__selector_hash_find_by_id function
	let (hashEntryOption, resultData) = css_selector_hash_instance.css__selector_hash_find_by_id(~"name_1");
	match(resultData){
		CSS_OK	=>	{						
						match (hashEntryOption) {
							Some(val) =>{
											test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_id", ~"name_1" , ~"CSS_OK", ~"CSS_OK", ~"name_1: a value which exists in the hash"); 
											test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_id", ~"name_1" , ~"", fmt!("name_1 is found at %?", val));
										},	
							None      => test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_id", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
						}
					},
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_id", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
	}	

	// Verification of css__selector_hash_find_by_id function
	let (hashEntryOption, resultData) = css_selector_hash_instance.css__selector_hash_find_by_class(~"name_1");
	match(resultData){
		CSS_OK	=>	{						
						match (hashEntryOption) {
							Some(val) =>{
											test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_class", ~"name_1" , ~"CSS_OK", ~"CSS_OK", ~"name_1: a value which exists in the hash"); 
											test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_class", ~"name_1" , ~"", fmt!("name_1 is found at %?", val));
										},	
							None      => test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_class", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
						}
					},
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_by_class", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
	}	

	// Verification of css__selector_hash_find_by_id function
	let (hashEntryOption, resultData) = css_selector_hash_instance.css__selector_hash_find_universal();
	match(resultData){
		CSS_OK	=>	{						
						match (hashEntryOption) {
							Some(val) =>{
											test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_universal", ~"name_1" , ~"CSS_OK", ~"CSS_OK", ~"name_1: a value which exists in the hash"); 
											test_logger.info( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_universal", ~"name_1" , ~"", fmt!("name_1 is found at %?", val));
										},	
							None      => test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_universal", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
						}
					},
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find_universal", ~"name_1" , ~"CSS_OK", ~"None CSS_OK", ~"name_1: a value which exists in the hash")
	}

	// Another instance of css_selector	
	let css_qname_instance_2 = css_qname{
		name: ~"name_2",
		ns: ~"ns_2"
	};

	// create a new selector altogether
	let css_selector_detail_instance_2 : @mut css_selector_detail = @mut css_selector_detail {
										qname: copy css_qname_instance_2,      	
										selector_type:CSS_SELECTOR_ID,    
										combinator_type:CSS_COMBINATOR_SIBLING,    
										value_type:CSS_SELECTOR_DETAIL_VALUE_NTH, 
										negate: true,			

										//css_selector_detail_value - union merged
										string:None,
										a: 20,
										b: 300
									};

    let css_selector_instance_2 : @mut css_selector = @mut css_selector {
								combinator:None,  
								rule:None,		
								specificity:5000,
								data:~[css_selector_detail_instance_2]
							};

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_2)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"CSS_OK", ~"Inserting a selector "),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Inserting a selector")
	}

	// Another instance of css_selector	
	let css_qname_instance_3 = css_qname{
		name: ~"*",
		ns: ~"ns_3"
	};

	// create a new selector altogether
	// should get inserted into Named Class
	let css_selector_detail_instance_3 : @mut css_selector_detail = @mut css_selector_detail {
										qname: copy css_qname_instance_3,      	
										selector_type:CSS_SELECTOR_ID,    
										combinator_type:CSS_COMBINATOR_SIBLING,    
										value_type:CSS_SELECTOR_DETAIL_VALUE_NTH, 
										negate: true,			

										//css_selector_detail_value - union merged
										string:None,
										a: 20,
										b: 300
									};

    let css_selector_instance_3 : @mut css_selector = @mut css_selector {
								combinator:None,  
								rule:None,		
								specificity:5000,
								data:~[css_selector_detail_instance_3]
							};

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_3)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"CSS_OK", ~"Inserting a selector "),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Inserting a selector")
	}
						
	// create a new selector altogether
	// should get inserted into Named Id

	// Another instance of css_selector	
	let css_qname_instance_4 = css_qname{
		name: ~"*",
		ns: ~"ns_4"
	};

	let css_selector_detail_instance_4 : @mut css_selector_detail = @mut css_selector_detail {
										qname: copy css_qname_instance_4,      	
										selector_type:CSS_SELECTOR_ATTRIBUTE,    
										combinator_type:CSS_COMBINATOR_SIBLING,    
										value_type:CSS_SELECTOR_DETAIL_VALUE_NTH, 
										negate: false,			

										//css_selector_detail_value - union merged
										string:None,
										a: 20,
										b: 300
									};

    let css_selector_instance_4 : @mut css_selector = @mut css_selector {
								combinator:None,  
								rule:None,		
								specificity:5000,
								data:~[css_selector_detail_instance_4]
							};

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_4)){
		CSS_OK	=>	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"Non CSS_OK", ~"CSS_OK",  ~"Inserting a selector"),
		   _    =>  test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"Non CSS_OK", ~"Non CSS_OK", ~"Inserting a selector ")
	}
	

	// create a new selector altogether
	// should get inserted into Named Id
	// Another instance of css_selector	
	let css_qname_instance_5 = css_qname{
		name: ~"*abcd",
		ns: ~"ns_5"
	};


	let css_selector_detail_instance_5 : @mut css_selector_detail = @mut css_selector_detail {
										qname: copy css_qname_instance_5,      	
										selector_type:CSS_SELECTOR_ID,    
										combinator_type:CSS_COMBINATOR_SIBLING,    
										value_type:CSS_SELECTOR_DETAIL_VALUE_NTH, 
										negate: false,			

										//css_selector_detail_value - union merged
										string:None,
										a: 20,
										b: 300
									};

    let css_selector_instance_5 : @mut css_selector = @mut css_selector {
								combinator:None,  
								rule:None,		
								specificity:5000,
								data:~[css_selector_detail_instance_5]
							};

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_5)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"CSS_OK", ~"Inserting a selector "),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Inserting a selector")
	}

	match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance_5)){
		CSS_OK	=>	test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"CSS_OK", ~"Inserting same selector twice "),
		   _    =>  test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"", ~"CSS_OK", ~"Non CSS_OK",  ~"Inserting same selector twice")
	}

	match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance_5)){
		CSS_OK	=>	test_logger.fail( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"Non CSS_OK", ~"CSS_OK", ~"Removing an inserted selector "),
		   _    =>  test_logger.pass( ~"test_css_stylesheet_2.rs", ~"", ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"", ~"Non CSS_OK", ~"Non CSS_OK",  ~"Removing an inserted selector ")
	}
							
}