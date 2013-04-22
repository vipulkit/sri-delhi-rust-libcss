#[link(name = "css_stylesheet", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod parserutils_inputstream;
extern mod wapcaplet;
extern mod std ;


use std::arc;
use css_enum::* ;
use wapcaplet::*;
use parserutils_inputstream::*;

static CSS_STYLE_DEFAULT_SIZE : uint = 16 ;

pub struct css_qname {
	name:~str,
	ns:~str
}

pub struct css_selector_detail {
	qname:css_qname,
	selector_type:css_selector_type,
	combinator_type:css_combinator,
	value_type:css_selector_detail_value_type,
	negate:bool,

	//css_selector_detail_value - union merged
	string:Option<~str>,
	a:int,
	b:int
}

pub struct css_selector {
	combinator:Option<@css_selector>,
	rule:Option<@css_rule>,
	specificity:uint,

	data:~[@mut css_selector_detail]

}

pub struct css_selector_hash {
	elements:@[@css_selector],
	classes:@[css_selector],
	ids:@[@css_selector],
	universal:@[@css_selector]
}

pub struct css_style {
	bytecode:~[u32]
	//sheet:Option<@css_stylesheet>
}

pub struct css_stylesheet {
	selectors:css_selector_hash,
	rule_count:uint,
	rule_list:Option<@css_rule>,
	last_rule:Option<@css_rule>,
	disabled:bool,
	url:~str,
	title:~str,
	level:css_language_level,
	quirks_allowed:bool,
	quirks_used:bool,
	inline_style:bool,
	size:uint,
	cached_style:Option<~css_style>
}

pub struct css_rule {
	parent_rule:Option<@css_rule> ,
	parent_stylesheet:Option<@css_stylesheet>,
	prev:Option<@css_rule>,
	next:Option<@css_rule>,
	rule_type:css_rule_type,
	index:uint,
	items:uint
}

pub struct css_rule_selector {
	base:@css_rule,
	selectors:~[~css_selector],
	style:Option<@css_style>
} 

pub struct css_rule_media {
	base:@css_rule,
	media:u64,
	first_child:Option<@css_rule>,
	last_child:Option<@css_rule>
} 

pub struct css_rule_font_face {
	base:@css_rule,
	//font_face:@css_font_face ;
} 

pub struct css_rule_page {
	base:@css_rule,
	selector:Option<~css_selector>,
	style:Option<@css_style>
} 

pub struct css_rule_import {
	base:@css_rule,
	url:~str,
	media:u64,
	sheet:Option<@css_stylesheet>
} 
pub struct css_rule_charset {
	base:@css_rule,
	encoding:~str	
} 


enum CSS_RULE_DATA_TYPE {
	RULE_UNKNOWN(~css_rule),
	RULE_SELECTOR(~css_rule_selector),
	RULE_CHARSET(~css_rule_charset),
	RULE_IMPORT(~css_rule_import),
	RULE_MEDIA(~css_rule_media),
	RULE_FONT_FACE(~css_rule_font_face),
	RULE_PAGE(~css_rule_page)
}

impl css_stylesheet {
	pub fn css__stylesheet_style_create(&mut self ) -> ~css_style {

		if self.cached_style.is_none() {
			~css_style{bytecode:~[]} 
		}
		else {
			self.cached_style.swap_unwrap()
		}
	}

	pub fn css__stylesheet_merge_style(target : &mut ~css_style, style: &css_style) {
		target.bytecode += style.bytecode;
	}

	pub fn css__stylesheet_style_append(target : &mut ~css_style, bytecode: u32) {
		target.bytecode.push(bytecode);
	}
	
	pub fn css__stylesheet_style_vappend(target : &mut ~css_style, bytecodes: &[u32] ) {
		target.bytecode += bytecodes;
	}

	pub fn css__stylesheet_selector_create(&mut self, qname : css_qname ) -> ~css_selector {
		let mut sel = ~css_selector{  
			combinator:None, 
			rule:None, 
			specificity:{
				if self.inline_style {
					CSS_SPECIFICITY_A
				}
				else if (qname.name.len() != 1 || str::char_at(qname.name,0) != '*') {
					CSS_SPECIFICITY_D
				}
				else {
					0u
				}
			},
			data:~[]
		};

		let mut sel_data = @mut css_selector_detail{
			qname:qname,
			selector_type: CSS_SELECTOR_ELEMENT,
			combinator_type: CSS_COMBINATOR_NONE,
			value_type:CSS_SELECTOR_DETAIL_VALUE_STRING,
			negate:false,
			string: None,
			a:0,
			b:0
		};
		sel.data.push(sel_data);
		sel
	}

	pub fn css__stylesheet_selector_detail_init(detail : &mut ~css_selector_detail, sel_type: css_selector_type,
												qname : css_qname , value_type : css_selector_detail_value_type,
												string_value : Option<~str> , ab_value : Option<(int,int)>,
												negate:bool)  -> css_result {

		detail.selector_type = sel_type;
		detail.qname= qname;
		detail.value_type=value_type;
		detail.negate=negate;
		match value_type {
			CSS_SELECTOR_DETAIL_VALUE_STRING=>  {
				if string_value.is_none() {
					CSS_BADPARM
				}
				else { 
					detail.string=string_value ;
					CSS_OK 
				}
			}
			CSS_SELECTOR_DETAIL_VALUE_NTH => 
				match ab_value { 
					None=> CSS_BADPARM,
					Some((x,y))=> { 
									detail.a=x ; 
									detail.b=y; 
									CSS_OK
								  }
				}
		}
	}
	
	pub fn css__stylesheet_selector_append_specific(selector : &mut ~css_selector, sel_type: css_selector_type,
												name : css_qname , val_type : css_selector_detail_value_type,
												string_value : Option<~str> , ab_value : Option<(int,int)>,
												negate:bool, comb_type : css_combinator)  -> css_result  {
		let mut detail = @mut css_selector_detail{
			// combinator:None,
			// rule:None,
			// specificity:0,

			qname:name,
			selector_type:sel_type,
			combinator_type:comb_type,
			value_type:val_type,
			negate:negate,

			string:None,
			a:0,
			b:0
		};

		match sel_type {
			CSS_SELECTOR_CLASS=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_PSEUDO_CLASS=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE_EQUAL=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE_DASHMATCH=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE_INCLUDES=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE_PREFIX=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE_SUFFIX=> selector.specificity += CSS_SPECIFICITY_C, 
			CSS_SELECTOR_ATTRIBUTE_SUBSTRING=> selector.specificity += CSS_SPECIFICITY_C, 

			CSS_SELECTOR_ID=> selector.specificity += CSS_SPECIFICITY_B ,
			
			CSS_SELECTOR_PSEUDO_ELEMENT=> selector.specificity += CSS_SPECIFICITY_D ,
			CSS_SELECTOR_ELEMENT=> selector.specificity += CSS_SPECIFICITY_D 
		};

		match val_type {
			CSS_SELECTOR_DETAIL_VALUE_STRING =>	match string_value {
													None=> return CSS_BADPARM,
													Some(copy x)=>	{ 
														detail.string=Some(x);
													}
												},
			CSS_SELECTOR_DETAIL_VALUE_NTH=> match ab_value { 
												None=> return CSS_BADPARM,
												Some((x,y))=> { 
													detail.a=x ; 
													detail.b=y;
									  			}
									  		}
		};
		selector.data.push(detail);
		CSS_OK
	}
	pub fn css__stylesheet_selector_combine(typ : css_combinator, a : @css_selector , 
											b : &mut ~css_selector) -> css_result {
		match b.combinator {
			Some(x)=> return CSS_INVALID,
			None=> {}
		};

		for a.data.each |&detail| {
			match detail.selector_type {
				CSS_SELECTOR_PSEUDO_ELEMENT => return CSS_INVALID ,
				_=> loop
			};
		}

		b.combinator=Some(a);
		b.data[0].combinator_type=typ;
		b.specificity += a.specificity;
		CSS_OK
	}
	pub fn css_stylesheet_rule_create(&mut self, typ : css_rule_type ) -> CSS_RULE_DATA_TYPE  {
		let mut base_rule = @css_rule{ 
			parent_rule:None,
			parent_stylesheet:None,
			next:None,
			prev:None,
			rule_type:typ,
			index:0,
			items:0
		};

		match typ {
			CSS_RULE_UNKNOWN=>  { 	
				let mut ret_rule = ~css_rule{ 
					parent_rule:None,
					parent_stylesheet:None,
					next:None,
					prev:None,
					rule_type:typ,
					index:0,
					items:0
				};
				RULE_UNKNOWN(ret_rule) 
			},

			CSS_RULE_SELECTOR=> {	
				let mut ret_rule = ~css_rule_selector{
					base:base_rule,
					selectors:~[],
					style:None
				};  
				RULE_SELECTOR(ret_rule)
			} ,


			CSS_RULE_CHARSET=>  {	
				let mut ret_rule = ~css_rule_charset{
					base:base_rule,
					encoding:~""
				};  
				RULE_CHARSET(ret_rule) 
			},

			CSS_RULE_IMPORT=>   {	
				let mut ret_rule = ~css_rule_import{
					base:base_rule,
					url:~"",
					media:0,
					sheet:None
				};  
				RULE_IMPORT(ret_rule) 
			},

			CSS_RULE_MEDIA=> 	{	
				let mut ret_rule = ~css_rule_media{ 
					base:base_rule,
					media:0,
					first_child:None,
					last_child:None
				};  
				RULE_MEDIA(ret_rule) 
			},

			CSS_RULE_FONT_FACE=>{	
				let mut ret_rule = ~css_rule_font_face{
					base:base_rule
				};  
				RULE_FONT_FACE(ret_rule) 
			},

			CSS_RULE_PAGE=>		{ 	
				let mut ret_rule = ~css_rule_page{
					base:base_rule,
					selector:None,
					style:None
				};  
				RULE_PAGE(ret_rule) 
			}

		}
	}

	pub fn css__stylesheet_rule_add_selector() {
		
	}
}

fn main () {
	io::println(fmt!("\n hellow world"));
}