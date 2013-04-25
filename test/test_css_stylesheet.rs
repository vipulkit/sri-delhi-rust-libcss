
extern mod std;
extern mod css_stylesheet;
extern mod css_enum;
extern mod test;

use css_stylesheet::*;
use css_enum::*;
use test::*;


fn main() {

	let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));

	let css_qname_instance = css_qname{
		 name: ~"name",
		 ns: ~"ns"};
	
	let  mut css_selector_detail_instance = @mut css_selector_detail{ 
									qname:copy css_qname_instance, 
									selector_type:CSS_SELECTOR_ELEMENT, 
									combinator_type:CSS_COMBINATOR_NONE, 
									value_type:CSS_SELECTOR_DETAIL_VALUE_STRING, 
									negate: false,	
									string: Some(~"RoughData"),	
									a: 0,
									b: 0 
								};	

	let mut css_selector_instance =  @mut css_selector {
											combinator:None,	
											rule: None,		
											specificity:0,
											data:~[css_selector_detail_instance] };	

	let hash_entry_instance =  @mut hash_entry {
							selector:copy css_selector_instance,
							next:None	};


	let css_selector_hash_instance = @mut css_selector_hash {
				default_slots: 0,
				elements:~[Some(hash_entry_instance)],
				classes: ~[Some(hash_entry_instance)],
				ids:~[Some(hash_entry_instance)],
				universal:~[Some(hash_entry_instance)]
				};
				

	let css_rule_instance = @mut css_rule {
				parent_rule:None ,
				parent_stylesheet:None,
				prev:None,
				next:None,				
				index: 0 };

				
	let css_style_instance = @mut css_style {
					//bytecode:~[]	};
					bytecode:~[9898]};  // random value for testing purpose

	
	let css_stylesheet_instance = @mut css_stylesheet {
				selectors:css_selector_hash_instance,
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
				//cached_style:None 
				cached_style:Some(copy css_style_instance)
			};

 // 1.			
  let css_style_value = css_stylesheet_instance.css__stylesheet_style_create();
 
  if css_style_value.bytecode == css_style_instance.bytecode{  	
  	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"" , ~"") ;
  }
  else{
  	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_style_create", ~"" , ~"") ;
  }

// 2.
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


// 3.
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

// 4.

let value : &[u32] = &[2000]; // random value for testing

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

// ===================================================================================================================


 // 1.

 // buffer value of css_qname_instance for future usage
 let mut buff_qname = copy css_qname_instance;
 let css_selector_instance_2 = css_stylesheet_instance.css__stylesheet_selector_create(css_qname_instance);

 match(css_selector_instance_2.combinator)
 {
 	Some (x) =>   test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"combinator value is wrong"),
 	None     =>   test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"combinator value is correct")
 }

 match(css_selector_instance_2.rule)
 {
 	Some (x) =>   test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"rule value is wrong"),
 	None     =>   test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"rule value is correct")
 }

 if css_selector_instance_2.specificity != CSS_SPECIFICITY_D{
 	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"specificity value is wrong");
 }
 else {
    test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"specificity value is correct");
 }

 // Verify data value as well
/*
 if (css_selector_instance_2.data == ~[]){
 	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"data value is correct");	
 }
 else{
 	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_create", ~"" , ~"data value is wrong");
 }
*/

let css_selector_data = copy css_selector_instance_2.data;
 
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

// =====================================================================================================================================================================


// 2.

let mut css_result_value = css_stylesheet::css__stylesheet_selector_detail_init(css_selector_data[0], CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
	None, None, false);

	match(css_result_value){
		CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~""),
		 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_detail_init", ~"" , ~"")	
	}

// 3.

// Need to correct: Remove "COPY"
match(css_stylesheet::css__stylesheet_selector_append_specific(copy css_selector_instance, CSS_SELECTOR_ELEMENT, copy buff_qname, CSS_SELECTOR_DETAIL_VALUE_STRING,
		None, None, false, CSS_COMBINATOR_NONE)){
		CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~""),
		 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_append_specific", ~"" , ~"")	
}


// Need to correct: Remove "COPY"	
match(css_stylesheet::css__stylesheet_selector_combine(CSS_COMBINATOR_NONE,copy css_selector_instance, copy css_selector_instance)){
		CSS_OK => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_combine", ~"" , ~""),
		 _  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_selector_combine", ~"" , ~"")	
}


match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_UNKNOWN)){
	RULE_UNKNOWN(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_UNKNOWN" , ~"");							
						},	
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_UNKNOWN" , ~"")

}

match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_SELECTOR)){
RULE_SELECTOR(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_SELECTOR" , ~"");							
						},	
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_SELECTOR" , ~"")
	
}

match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_CHARSET)){
RULE_CHARSET(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_CHARSET" , ~"");							
						},
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_CHARSET" , ~"")
	
}

match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_IMPORT)){
RULE_IMPORT(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_IMPORT" , ~"");							
						},	
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_IMPORT" , ~"")
	
}

match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_MEDIA)){
RULE_MEDIA(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_MEDIA" , ~"");							
						},	
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_MEDIA" , ~"")
	
}

match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_FONT_FACE)){
RULE_FONT_FACE(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_FONT_FACE" , ~"");							
						},	
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_FONT_FACE" , ~"")
	
}

match(css_stylesheet_instance.css_stylesheet_rule_create(CSS_RULE_PAGE)){
RULE_PAGE(x)    => {
							test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_PAGE" , ~"");							
					},	
	  _             =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css_stylesheet_rule_create", ~"CSS_RULE_PAGE" , ~"")
	
}


let css_rule_selector_instance = @mut css_rule_selector{
									 base: css_rule_instance,	
									 selectors: ~[copy css_selector_instance],
									 style: None
};

	match(css_stylesheet::css__stylesheet_rule_add_selector(RULE_SELECTOR(copy css_rule_selector_instance), copy css_selector_instance)){
		CSS_OK  =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"RULE_SELECTOR" , ~""),
		  _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"RULE_SELECTOR" , ~"")
	}


	match(css_stylesheet::css__stylesheet_rule_add_selector(RULE_UNKNOWN(copy css_rule_instance), copy css_selector_instance)){
		CSS_OK  =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"RULE_UNKNOWN" , ~""),
		  _     =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_add_selector", ~"RULE_UNKNOWN" , ~"")
	}



	match(css_stylesheet_instance.css__stylesheet_rule_append_style(RULE_SELECTOR(copy css_rule_selector_instance), copy css_style_instance))
	{
		CSS_OK   => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"RULE_SELECTOR" , ~""),
		  _      => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"RULE_SELECTOR" , ~"")
	}

	match(css_stylesheet_instance.css__stylesheet_rule_append_style(RULE_UNKNOWN(copy css_rule_instance), css_style_instance))
	{
		CSS_OK   => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"RULE_UNKNOWN" , ~""),
		  _      => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_append_style", ~"RULE_UNKNOWN" , ~"")
	}    

	let css_rule_charset_instance = @mut css_rule_charset {
		base: css_rule_instance,
		encoding: ~"str"	
	};
    
    match(css_stylesheet::css__stylesheet_rule_set_charset((RULE_CHARSET(css_rule_charset_instance)), ~"RoughData")){
    	CSS_OK	=> test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_charset", ~"RULE_CHARSET" , ~""),
    	   _    => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_charset", ~"RULE_CHARSET" , ~"")
    }


    let css_rule_import_instance = @mut css_rule_import {
	base : css_rule_instance,
	url:~"URL",
	media:676,
	sheet:None } ;


	match(css_stylesheet::css__stylesheet_rule_set_nascent_import((RULE_IMPORT(css_rule_import_instance)), ~"google", 980)){
		CSS_OK	=> test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_nascent_import", ~"RULE_IMPORT" , ~""),
    	   _    => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_nascent_import", ~"RULE_IMPORT" , ~"")	
	}


	let  css_rule_media_instance = @mut css_rule_media {
	base:css_rule_instance,
	media: 786,
	first_child:None,
	last_child:None } ;

	match(css_stylesheet::css__stylesheet_rule_set_media((RULE_MEDIA(css_rule_media_instance)), 980)) {
		CSS_OK	=> test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_media", ~"RULE_MEDIA" , ~""),
    	   _    => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_media", ~"RULE_MEDIA" , ~"")	
	}


	let  css_rule_page_instance = @mut css_rule_page {
	base:css_rule_instance,
	selector:None,
	style:None } ;

	match(css_stylesheet::css__stylesheet_rule_set_page_selector((RULE_PAGE(css_rule_page_instance)), css_selector_instance)) {
		CSS_OK	=> test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_page_selector", ~"RULE_PAGE" , ~""),
    	   _    => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_rule_set_page_selector", ~"RULE_PAGE" , ~"")	
	}

	let base_rule = css_stylesheet::css__stylesheet_get_base_rule(RULE_PAGE(css_rule_page_instance));

	if (base_rule.parent_rule.is_some() && base_rule.parent_stylesheet.is_some()) {
			test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_get_base_rule", ~"RULE_PAGE" , ~"both values are having Some");
		}

	else if (base_rule.parent_rule.is_none() && base_rule.parent_stylesheet.is_none()) {
		test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_get_base_rule", ~"RULE_PAGE" , ~"both values are having None");
	}
	else {
		test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_get_base_rule", ~"RULE_PAGE" , ~"");
	}


match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, RULE_PAGE(css_rule_page_instance), None)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"RULE_PAGE" , ~""),
	   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"RULE_PAGE" , ~"")
}

match(css_stylesheet::css__stylesheet_add_rule(css_stylesheet_instance, RULE_MEDIA(css_rule_media_instance), None)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"RULE_MEDIA" , ~""),
	   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_add_rule", ~"RULE_MEDIA" , ~"")
}
	

match(css_stylesheet::css__stylesheet_remove_rule(css_stylesheet_instance, RULE_MEDIA(css_rule_media_instance))) {
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"RULE_MEDIA" , ~""),
	   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__stylesheet_remove_rule", ~"RULE_MEDIA" , ~"")
}


match(css_stylesheet_instance._add_selectors(RULE_MEDIA(css_rule_media_instance))){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"RULE_MEDIA" , ~""),
	   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"RULE_MEDIA" , ~"")	
}

match(css_stylesheet_instance._add_selectors(RULE_PAGE(css_rule_page_instance))){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"RULE_PAGE" , ~""),
	   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_add_selectors", ~"RULE_PAGE" , ~"")	
}


match(css_stylesheet_instance._remove_selectors(RULE_PAGE(css_rule_page_instance))){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"RULE_PAGE" , ~""),
	   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_selectors", ~"RULE_PAGE" , ~"")	
}

// =========================================================================================================================


let css_selector_hash_instance_2 = css_selector_hash::css__selector_hash_create();	

if css_selector_hash_instance_2.default_slots == (1<<6){
	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"default_slots" , ~"default_slots value is correct");
}
else{
	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"default_slots" , ~"default_slots value is wrong");
}

match(css_selector_hash_instance_2.elements[0]){
	Some(x)  => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"elements" , ~"elements value is wrong"),
	None     => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"elements" , ~"elements value is correct")
}

match(css_selector_hash_instance_2.classes[0]){
	Some(x)  => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"classes" , ~"classes value is wrong"),
	None     => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"classes" , ~"classes value is correct")
}

match(css_selector_hash_instance_2.ids[0]){
	Some(x)  => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"ids" , ~"ids value is wrong"),
	None     => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"ids" , ~"ids value is correct")
}

match(css_selector_hash_instance_2.universal[0]){
	Some(x)  => test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"universal" , ~"universal value is wrong"),
	None     => test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_create", ~"universal" , ~"universal value is correct")
}


// ====================================================================================================


let css_selector_detail_instance_100 = @mut css_selector_detail {
	qname: copy buff_qname,
	selector_type: CSS_SELECTOR_CLASS,
	combinator_type:CSS_COMBINATOR_NONE,
	value_type:CSS_SELECTOR_DETAIL_VALUE_STRING,
	negate:false,
	string:None,
	a:100,
	b:200
};


let css_selector_instance_100 =  @mut css_selector {
	combinator:None,
	rule:None,
	specificity:0,

	data:~[css_selector_detail_instance_100] };


let _class_name_str = css_selector_hash::_class_name(css_selector_instance_100);	
if _class_name_str.eq(&buff_qname.name){
	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_class_name", ~"" , ~"_class_name value is correct");
}
else{
	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_class_name", ~"" , ~"_class_name value is wrong");
}

// ================================================================================================================


let css_selector_detail_instance_100 = @mut css_selector_detail {
	qname: copy buff_qname,
	selector_type: CSS_SELECTOR_ID,
	combinator_type:CSS_COMBINATOR_NONE,
	value_type:CSS_SELECTOR_DETAIL_VALUE_STRING,
	negate:false,
	string:None,
	a:100,
	b:200
};


let css_selector_instance_100 =  @mut css_selector {
	combinator:None,
	rule:None,
	specificity:0,
	data:~[css_selector_detail_instance_100] };

let _id_name_str = css_selector_hash::_id_name(css_selector_instance_100);	
if _id_name_str.eq(&buff_qname.name){
	test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_id_name", ~"" , ~"_id_name value is correct");
}
else{
	test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_id_name", ~"" , ~"_id_name value is wrong");
}


match(css_selector_hash_instance_2.css__selector_hash_insert(css_selector_instance_100)){
		CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~""),
		   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_insert", ~"" , ~"")
	}


match(css_selector_hash_instance_2._insert_into_chain(Element, 0, css_selector_instance_100)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_insert_into_chain", ~"" , ~"Element"),
		   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_insert_into_chain", ~"" , ~"Element")
}

match(css_selector_hash_instance_2._insert_into_chain(Universal, 0, css_selector_instance_100)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_insert_into_chain", ~"" , ~"Universal"),
		   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_insert_into_chain", ~"" , ~"Universal")
}


match(css_selector_hash_instance_2.css__selector_hash_remove(css_selector_instance_100)){
CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~""),
		   _     =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"css__selector_hash_remove", ~"" , ~"")	
}


match(css_selector_hash_instance_2._remove_from_chain(Universal, 0, css_selector_instance_100)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_from_chain", ~"Universal_index_0" , ~""),
		_    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_from_chain", ~"Universal_index_0" , ~"")	
}

match(css_selector_hash_instance_2._remove_from_chain(Universal, 3, css_selector_instance_100)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_from_chain", ~"Universal_index_3" , ~""),
		_    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_from_chain", ~"Universal_index_3" , ~"")	
}

match(css_selector_hash_instance_2._remove_from_chain(Element, 3, css_selector_instance_100)){
	CSS_OK   =>  test_logger.pass( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_from_chain", ~"Element_index_3" , ~""),
		_    =>  test_logger.fail( ~"stylesheet",~"css_stylesheet.rs", ~"_remove_from_chain", ~"Element_index_3" , ~"")	
}


}// main function ends here
