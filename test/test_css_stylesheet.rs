
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
							bytecode:~[9898]};  // random value

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
									cached_style:Some(css_style_instance)  	// An instance is created for verification
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

		let mut test_logger : ~test_report = result::unwrap(test_report(&"temp_log.csv"));

		let css_selector_hash_instance = css_selector_hash::css__selector_hash_create();	

		if css_selector_hash_instance.default_slots == (1<<6){
			test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"default_slots" , ~"default_slots value is correct");
		}
		else{
			test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"default_slots" , ~"default_slots value is wrong");
		}

	 	return css_selector_hash_instance;
}

fn css__selector_hash_functionalities_test_2(css_selector_instance : @mut css_selector, css_selector_hash_instance : @mut css_selector_hash){

		let mut test_logger : ~test_report = result::unwrap(test_report(&"temp_log.csv"));

		// Without inserting anything trying to remove
		match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance)){
			CSS_OK	=>	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~""),
			   _    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~"")
		}

		match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance)){
			CSS_OK	=>	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~""),
			   _    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~"")
		}
        
        match(css_selector_hash_instance.css__selector_hash_find(~"name")){
            (None,y)     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name" , ~""),
            (Some(x), y)  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name" , ~"")
        }

        match(css_selector_hash_instance.css__selector_hash_find(~"Sushanta")){
            (None, y)     =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"Sushanta" , ~""),
            (Some(x), y)  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"Sushanta" , ~"")
        }

        match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance)){
            CSS_OK  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~""),
               _    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~"")
        }

        // Since entry has been removed hence, entry shouldn't be found.
        match(css_selector_hash_instance.css__selector_hash_find(~"name")){
            (None,y)     =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name" , ~""),
            (Some(x), y)  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_find", ~"name" , ~"")
        }
	
		css_selector_instance.data[1].qname.name = ~"*";        
		match(css_selector_hash_instance.css__selector_hash_insert(css_selector_instance)){
			CSS_OK	=>	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~""),
			   _    =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~"")
		}

		match(css_selector_hash_instance.css__selector_hash_remove(css_selector_instance)){
			CSS_OK	=>	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~""),
			   _    =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~"")
		}
}


// 2.
fn css__stylesheet_style_functionalities_test(css_stylesheet_instance : @mut css_stylesheet) -> @mut css_stylesheet {

			let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));
			let mut css_style_instance = @mut css_style {bytecode : ~[]};  // value is initialised					

			if (!css_stylesheet_instance.cached_style.is_none()){			
					css_style_instance = css_stylesheet_instance.cached_style.unwrap();			
			}
			else{
				  ()
			}
					 
			// A.			
			let css_style_value = css_stylesheet_instance.css__stylesheet_style_create();

			if css_style_value.bytecode == css_style_instance.bytecode{  		 
			  	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"" , ~"") ;
			}
			else{
			  	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"" , ~"") ;
			}

			// B.
			let css_style_source = @mut css_style {
											bytecode:~[11]};
			
            let b4Value : ~[u32] = copy css_style_value.bytecode + copy css_style_source.bytecode;

			css_stylesheet::css__stylesheet_merge_style(css_style_value, css_style_source); 

			let mut afterValue = copy css_style_value.bytecode;
						
			if b4Value.eq(&afterValue){
				  	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_merge_style", ~"", ~"") ;
			  }
		    else{
			  	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_merge_style", ~"" , ~"") ;
			  }


			// C.
			let value : u32 = 1000; // random value for testing

			// b4Value is updated
			let mut b4Value = copy css_style_value.bytecode;
			b4Value.push(value);

			css_stylesheet::css__stylesheet_style_append(css_style_value, value);

			// afterValue is updated
			afterValue = copy css_style_value.bytecode;

					 
			if afterValue.eq(&b4Value){
			  	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_append", ~"" , ~"") ;
			 }
			 else{
			  	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_append", ~"" , ~"") ;
			 }

			// D.
			let value : &[u32] = &[2000, 90, 100]; // random value for testing

			// buffValue is updated
			let mut b4Value = copy css_style_value.bytecode;
			b4Value += value;
			css_stylesheet::css__stylesheet_style_vappend(css_style_value, value);

			// afterValue is updated
			afterValue = copy css_style_value.bytecode;

			if afterValue.eq(&b4Value){
				  	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_vappend", ~"" , ~"") ;
			}
			else{
				  	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_vappend", ~"" , ~"") ;
			}

			// Updating the values
			css_stylesheet_instance.cached_style = Some(css_style_value);
			return css_stylesheet_instance;

} // test_css__stylesheet_style_functionalities ends here


// 3.
fn css__stylesheet_selector_functionalities_test(css_stylesheet_instance : @mut css_stylesheet) -> @mut css_selector {

			let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));

			let css_qname_instance = css_qname{
				 name: ~"name",
				 ns: ~"ns"};
					
			// A. 
			 let mut buff_qname = copy css_qname_instance;
			 let css_selector_instance = css_stylesheet_instance.css__stylesheet_selector_create(css_qname_instance);			 			

			 match(css_selector_instance.combinator)
			 {
			 	Some (x) =>   test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"combinator value is wrong"),
			 	None     =>   test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"combinator value is correct")
			 }

			 match(css_selector_instance.rule)
			 {
			 	Some (x) =>   test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"rule value is wrong"),
			 	None     =>   test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"rule value is correct")
			 }

			 if css_selector_instance.specificity != CSS_SPECIFICITY_D{
			 	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"specificity value is wrong");
			 }
			 else {
			    test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"specificity value is correct");
			 }

			let css_selector_data = copy css_selector_instance.data;
				 
			if (css_selector_data[0].qname.name == buff_qname.name) && (css_selector_data[0].qname.ns == buff_qname.ns){
				test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.qname is correct");	
			}
			else {
				test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.qname is wrong");		
			}
					 
			 match(css_selector_data[0].selector_type)
			 {
				 	CSS_SELECTOR_ELEMENT  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.selector_type value is correct"),
				 	   _                  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.selector_type value is wrong")
			 }


			match(css_selector_data[0].combinator_type)
			 {
			 	CSS_COMBINATOR_NONE  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.combinator_type value is correct"),
			 	   _                  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.combinator_type value is wrong")
			 }

			match(css_selector_data[0].value_type)
			 {
			 	CSS_SELECTOR_DETAIL_VALUE_STRING  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.value_type value is correct"),
					   	   _                  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.value_type value is wrong")
			 }

			if (css_selector_data[0].negate == false) {
				 test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.negate value is correct");
			}
			else{
					test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.negate value is wrong");	
				}

			if (css_selector_data[0].a == 0) {
					 test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.a value is correct");
			}
			else{
					test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.a value is wrong");	
			}

			if (css_selector_data[0].b == 0) {
					 test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.b value is correct");
			}
			else{
					test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.b value is wrong");	
			}


			match(copy css_selector_data[0].string){
				 	Some (x) =>   test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.string value is wrong"),
				 	None     =>   test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"css_selector_data.string value is correct")	
			}			

		    // B.

			let mut css_result_value = css_stylesheet::css__stylesheet_selector_detail_init(css_selector_data[0], CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
			None, None, false);

			match(css_result_value){
				CSS_OK => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~""),
				 _  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"")	
			}

			let mut css_result_value = css_stylesheet::css__stylesheet_selector_detail_init(css_selector_data[0], CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
			Some(~"RandomValue"), None, false);

			match(css_result_value){
				CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~""),
				 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"")	
			}

			let mut css_result_value = css_stylesheet::css__stylesheet_selector_detail_init(css_selector_data[0], CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_NTH,
			Some(~"RandomValue"), None, false);

			match(css_result_value){
				CSS_OK => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~""),
				 _  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"")	
			}

			let mut css_result_value = css_stylesheet::css__stylesheet_selector_detail_init(css_selector_data[0], CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_NTH,
			None, Some((11, 22)), false);

			match(css_result_value){
				CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~""),
				 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"")	
			}

		      // C.	
		      // Note: Herein, css_selector_instance is getting one new value pushed.

			match(css_stylesheet::css__stylesheet_selector_append_specific(css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
				None, None, false, CSS_COMBINATOR_NONE)){
				CSS_OK => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~""),
				 _  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~"")	
				}

			match(css_stylesheet::css__stylesheet_selector_append_specific(css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
				Some(~"RandomValue"), None, false, CSS_COMBINATOR_NONE)){
				CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~""),
				 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~"")	
				}

			match(css_stylesheet::css__stylesheet_selector_append_specific(css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_NTH,
				None, Some((1,2)), false, CSS_COMBINATOR_NONE)){
				CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~""),
				 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~"")	
				}			
			
		  	// D.						
		  	// For testing below mentioned function            
		  	let css_qname_instance_New = css_qname{
												 name: ~"New_name",
				 								  ns: ~"New_ns"	};	

		  	let  mut css_selector_detail_instance_New = @mut css_selector_detail{ 
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
				CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_combine", ~"" , ~""),
				 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_combine", ~"" , ~"")	
			}
			
		return css_selector_instance;
}


fn css_stylesheet_rule_functionalities_test(css_stylesheet_instance : @mut css_stylesheet, css_selector_instance : @mut css_selector) -> @mut css_stylesheet {

			let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));
			let css_style_instance = css_stylesheet_instance.cached_style.unwrap();

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_UNKNOWN);
			match(retVal){
				RULE_UNKNOWN(x)    => {																		
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_BADPARM   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_UNKNOWN" , ~""),							
										     _        =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_UNKNOWN" , ~"")							
										}
									},	
				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_UNKNOWN" , ~"")

			}

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_SELECTOR);
			match(retVal){
			RULE_SELECTOR(x)    => {
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_SELECTOR" , ~""),							
										     _        =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_SELECTOR" , ~"")							
									}
									
									match(css_stylesheet_instance.css__stylesheet_rule_append_style(retVal, css_style_instance)){
										CSS_BADPARM   =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_SELECTOR" , ~""),							
										     _        =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_SELECTOR" , ~"")							
									}
									},	
				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_SELECTOR" , ~"")
				
			}

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_CHARSET);
			match(retVal){
			RULE_CHARSET(x)    => {
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_BADPARM   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_CHARSET" , ~""),							
										     _        =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_CHARSET" , ~"")							
									}

									match(css_stylesheet::css__stylesheet_rule_set_charset(retVal, ~"testValue")){
										CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_charset", ~"RULE_CHARSET" , ~""),							
										     _        =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_charset", ~"RULE_CHARSET" , ~"")
									}												
								},
				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_CHARSET" , ~"")
				
			}

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_IMPORT);
			match(retVal){
			RULE_IMPORT(x)    => {
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_BADPARM   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_IMPORT" , ~""),							
										     _        =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_IMPORT" , ~"")							
									}
																							// roughURL and 1234 are random value for testing			
									match(css_stylesheet::css__stylesheet_rule_set_nascent_import(retVal, ~"roughURL", 1234)){
										CSS_OK	   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_nascent_import", ~"CSS_RULE_IMPORT" , ~""),							
										     _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_nascent_import", ~"CSS_RULE_IMPORT" , ~"")							
									}						
								},	
				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_IMPORT" , ~"")
				
			}

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_MEDIA);
			match(retVal){
			RULE_MEDIA(x)    => {
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_MEDIA" , ~""),							
										     _    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_MEDIA" , ~"")							
									}	
																				// 4321 is a random value for testing			
									match(css_stylesheet::css__stylesheet_rule_set_media(retVal, 4321)){
										CSS_OK   	=>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_media", ~"CSS_RULE_MEDIA" , ~""),							
										     _      =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_media", ~"CSS_RULE_MEDIA" , ~"")							
									}											
								},	

				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_MEDIA" , ~"")
				
			}

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_FONT_FACE);
			match(retVal){
			RULE_FONT_FACE(x)    => {
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_BADPARM   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_FONT_FACE" , ~""),							
										     _        =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_FONT_FACE" , ~"")							
									}
								},	
				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_FONT_FACE" , ~"")
				
			}

			let retVal = css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_PAGE);
			match(retVal){
			RULE_PAGE(x)    => {
									match(css_stylesheet::css__stylesheet_rule_add_selector(retVal, css_selector_instance)){
										CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_PAGE" , ~""),
										CSS_BADPARM  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_PAGE" , ~""),
										     _   =>  test_logger.info( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"CSS_RULE_PAGE" , ~"new return value")
									}
									
									match(css_stylesheet::css__stylesheet_rule_set_page_selector(retVal, css_selector_instance)){
										CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_page_selector", ~"CSS_RULE_PAGE" , ~""),							
										     _   =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_page_selector", ~"CSS_RULE_PAGE" , ~"")							
									}
									
									let css_style_instance = css_stylesheet_instance.cached_style.unwrap();
									match(css_stylesheet_instance.css__stylesheet_rule_append_style(retVal, css_style_instance)){
										CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_PAGE" , ~""),							
										     _    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"CSS_RULE_PAGE" , ~"")							
									}						

								},	
				  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_PAGE" , ~"")
				
			}

			css_stylesheet_instance.cached_style = Some(css_style_instance);

		return css_stylesheet_instance;
}