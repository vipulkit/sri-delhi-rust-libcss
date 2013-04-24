#[link(name = "css_stylesheet", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod std ;

use css_enum::* ;

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
	combinator:Option<@mut css_selector>,
	rule:Option<CSS_RULE_DATA_TYPE>,
	specificity:uint,

	data:~[@mut css_selector_detail]

}



pub struct css_style {
	bytecode:~[u32]
	//sheet:Option<@css_stylesheet>
}


pub struct css_selector_hash {
	elements:~[@mut css_selector],
	classes:~[@mut css_selector],
	ids:~[@mut css_selector],
	universal:~[@mut css_selector]
}

pub struct css_stylesheet {
	selectors:css_selector_hash,
	rule_count:uint,
	rule_list:Option<CSS_RULE_DATA_TYPE>,
	last_rule:Option<CSS_RULE_DATA_TYPE>,
	disabled:bool,
	url:~str,
	title:~str,
	level:css_language_level,
	quirks_allowed:bool,
	quirks_used:bool,
	inline_style:bool,
	cached_style:Option<@mut css_style>
}

pub struct css_rule {
	parent_rule:Option<CSS_RULE_DATA_TYPE> ,
	parent_stylesheet:Option<@mut css_stylesheet>,
	prev:Option<CSS_RULE_DATA_TYPE>,
	next:Option<CSS_RULE_DATA_TYPE>,
	//rule_type:css_rule_type,
	index:uint//,items:uint
}

pub struct css_rule_selector {
	base:@mut css_rule,
	selectors:~[@mut css_selector],
	style:Option<@mut css_style>
} 

pub struct css_rule_media {
	base:@mut css_rule,
	media:u64,
	first_child:Option<CSS_RULE_DATA_TYPE>,
	last_child:Option<CSS_RULE_DATA_TYPE>
} 

pub struct css_rule_font_face {
	base:@mut css_rule,
	//font_face:@css_font_face ;
} 

pub struct css_rule_page {
	base:@mut css_rule,
	selector:Option<@mut css_selector>,
	style:Option<@mut css_style>
} 

pub struct css_rule_import {
	base:@mut css_rule,
	url:~str,
	media:u64,
	sheet:Option<@css_stylesheet>
} 
pub struct css_rule_charset {
	base:@mut css_rule,
	encoding:~str	
} 


enum CSS_RULE_DATA_TYPE {
	RULE_UNKNOWN(@mut css_rule),
	RULE_SELECTOR(@mut css_rule_selector),
	RULE_CHARSET(@mut css_rule_charset),
	RULE_IMPORT(@mut css_rule_import),
	RULE_MEDIA(@mut css_rule_media),
	RULE_FONT_FACE(@mut css_rule_font_face),
	RULE_PAGE(@mut css_rule_page)
}

impl css_stylesheet {
	pub fn css__stylesheet_style_create(&mut self ) -> @mut css_style {

		if self.cached_style.is_none() {
			@mut css_style{bytecode:~[]} 
		}
		else {
			self.cached_style.swap_unwrap()
		}
	}

	pub fn css__stylesheet_merge_style(target : @mut css_style, style: @mut css_style) {
		target.bytecode += copy style.bytecode;
	}

	pub fn css__stylesheet_style_append(target : @mut css_style, bytecode: u32) {
		target.bytecode.push(bytecode);
	}
	
	pub fn css__stylesheet_style_vappend(target : @mut css_style, bytecodes: &[u32] ) {
		target.bytecode += bytecodes;
	}

	pub fn css__stylesheet_selector_create(&mut self, qname : css_qname ) -> @mut css_selector {
		let mut sel = @mut css_selector{  
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

	pub fn css__stylesheet_selector_detail_init (
		detail : @mut css_selector_detail, 
		sel_type: css_selector_type,
		qname : css_qname, 
		value_type : css_selector_detail_value_type,
		string_value : Option<~str> , 
		ab_value : Option<(int,int)>,
		negate:bool
	)  -> css_result {

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
	
	pub fn css__stylesheet_selector_append_specific(selector : @mut css_selector, selector_type: css_selector_type,
												name : css_qname , val_type : css_selector_detail_value_type,
												string_value : Option<~str> , ab_value : Option<(int,int)>,
												negate:bool, comb_type : css_combinator)  -> css_result  {
		let mut detail = @mut css_selector_detail{
			// combinator:None,
			// rule:None,
			// specificity:0,

			qname:name,
			selector_type:selector_type,
			combinator_type:comb_type,
			value_type:val_type,
			negate:negate,

			string:None,
			a:0,
			b:0
		};

		match selector_type {
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

	pub fn css__stylesheet_selector_combine(combinator_type : css_combinator, a : @mut css_selector , 
											b : @mut css_selector) -> css_result {
		match b.combinator {
			Some(_)=> return CSS_INVALID,
			None=> {}
		};

		for a.data.each_mut |&detail| {
			match detail.selector_type {
				CSS_SELECTOR_PSEUDO_ELEMENT => return CSS_INVALID ,
				_=> loop
			};
		}

		b.combinator=Some(a);
		b.data[0].combinator_type=combinator_type;
		b.specificity += a.specificity;
		CSS_OK
	}

	pub fn css_stylesheet_rule_create(&mut self, rule_type : css_rule_type ) -> CSS_RULE_DATA_TYPE  {
		let mut base_rule = @mut css_rule{ 
			parent_rule:None,
			parent_stylesheet:None,
			next:None,
			prev:None,
			index:0
		};

		match rule_type {
			CSS_RULE_UNKNOWN=>  { 	
				let mut ret_rule = @mut css_rule{ 
					parent_rule:None,
					parent_stylesheet:None,
					next:None,
					prev:None,
					index:0
				};
				RULE_UNKNOWN(ret_rule) 
			},

			CSS_RULE_SELECTOR=> {	
				let mut ret_rule = @mut css_rule_selector{
					base:base_rule,
					selectors:~[],
					style:None
				};  
				RULE_SELECTOR(ret_rule)
			} ,


			CSS_RULE_CHARSET=>  {	
				let mut ret_rule = @mut css_rule_charset{
					base:base_rule,
					encoding:~""
				};  
				RULE_CHARSET(ret_rule) 
			},

			CSS_RULE_IMPORT=>   {	
				let mut ret_rule = @mut css_rule_import{
					base:base_rule,
					url:~"",
					media:0,
					sheet:None
				};  
				RULE_IMPORT(ret_rule) 
			},

			CSS_RULE_MEDIA=> 	{	
				let mut ret_rule = @mut css_rule_media{ 
					base:base_rule,
					media:0,
					first_child:None,
					last_child:None
				};  
				RULE_MEDIA(ret_rule) 
			},

			CSS_RULE_FONT_FACE=>{	
				let mut ret_rule = @mut css_rule_font_face{
					base:base_rule
				};  
				RULE_FONT_FACE(ret_rule) 
			},

			CSS_RULE_PAGE=>		{ 	
				let mut ret_rule = @mut css_rule_page{
					base:base_rule,
					selector:None,
					style:None
				};  
				RULE_PAGE(ret_rule) 
			}

		}
	}

	pub fn css__stylesheet_rule_add_selector(css_rule : CSS_RULE_DATA_TYPE , selector : @mut css_selector) -> css_result {

		match css_rule {
			RULE_SELECTOR(x)=> {
				selector.rule = Some(css_rule);
				x.selectors.push(selector);
				CSS_OK
			},
			_=> CSS_BADPARM 
		}
	}
	
	pub fn css__stylesheet_rule_append_style(&mut self, css_rule : CSS_RULE_DATA_TYPE , style : @mut css_style) -> css_result {
		match css_rule {
			RULE_PAGE(page)=> {
				if page.style.is_none() {
					page.style = Some(style);
				}
				else {
					let mut page_style = page.style.get();
					css_stylesheet::css__stylesheet_merge_style(page_style,style);
					page.style = Some(page_style);
				}
			},
			RULE_SELECTOR(selector)=> {
				if selector.style.is_none() {
					selector.style = Some(style);
				}
				else {
					let mut selector_style = selector.style.get();
					css_stylesheet::css__stylesheet_merge_style(selector_style,style);
					selector.style = Some(selector_style);
				}
			},
			_=> return CSS_BADPARM 
		};
		CSS_OK
	}

	pub fn css__stylesheet_rule_set_charset(css_rule : CSS_RULE_DATA_TYPE, charset: ~str) -> css_result {
		if charset.len() <= 0 {
			return CSS_BADPARM;
		}

		match css_rule {
			RULE_CHARSET(x) => {
				x.encoding = charset;
				CSS_OK
			}
			_ => {
				CSS_BADPARM
			}
		}
	}

	pub fn css__stylesheet_rule_set_nascent_import(css_rule : CSS_RULE_DATA_TYPE, url_str:~str, 
													media:u64) -> css_result 	{

		match css_rule {
			RULE_IMPORT(x) => {
				x.url = url_str;
				x.media=media;
				CSS_OK
			}
			_ => {
				CSS_BADPARM
			}
		}
	}

	pub fn css__stylesheet_rule_set_media(css_rule : CSS_RULE_DATA_TYPE,
										 media:u64) -> css_result {

		match css_rule {
			RULE_MEDIA(x) => {
				x.media=media;
				CSS_OK
			}
			_ => {
				CSS_BADPARM
			}
		}
	}

	pub fn css__stylesheet_rule_set_page_selector(css_rule : CSS_RULE_DATA_TYPE,
													selector:@mut css_selector) -> css_result {

		match css_rule {
			RULE_PAGE(x) => {
				x.selector= Some(selector);
				CSS_OK
			}
			_ => {
				CSS_BADPARM
			}
		}
	}
	
	fn css__stylesheet_get_base_rule(css_rule : CSS_RULE_DATA_TYPE) -> @mut css_rule {
		match css_rule {
			RULE_UNKNOWN(r) => {
				r
			},
			RULE_SELECTOR(r)=>{
				r.base
			},
			RULE_CHARSET(r)=>{
				r.base
			},
			RULE_IMPORT(r)=>{
				r.base
			},
			RULE_MEDIA(r)=>{
				r.base
			},
			RULE_FONT_FACE(r)=>{
				r.base
			},
			RULE_PAGE(r)=>{
				r.base
			},
		}
	}

	pub fn css__stylesheet_add_rule(sheet : @mut css_stylesheet, css_rule : CSS_RULE_DATA_TYPE,
									parent_rule : Option<CSS_RULE_DATA_TYPE> ) -> css_result {
		
		let mut base_rule = css_stylesheet::css__stylesheet_get_base_rule(css_rule);

		base_rule.index = sheet.rule_count;

		match sheet._add_selectors(css_rule) {
			CSS_OK => {},
			_=> return CSS_INVALID
		}

		match parent_rule {
			Some(prule)=> {
				match prule {
					RULE_MEDIA(media_prule)=>{
						base_rule.parent_rule = parent_rule;

						match media_prule.last_child {
							None=>{
								base_rule.next = None;
								base_rule.prev = None;
								media_prule.first_child = Some(css_rule);
								media_prule.last_child = Some(css_rule);
							},
							Some(last_child)=>{
								let mut last_child_base_rule = css_stylesheet::css__stylesheet_get_base_rule(last_child);
								last_child_base_rule.next = Some(css_rule);
								base_rule.prev = media_prule.last_child ;
								base_rule.next = None;
								media_prule.last_child = Some(css_rule);
							}
						}
					},
					_=> return CSS_INVALID
				}
			},
			None=>{
				base_rule.parent_stylesheet = Some(sheet);

				match sheet.last_rule {
					None=>{
						base_rule.prev = None;
						base_rule.next = None;
						sheet.rule_list = Some(css_rule);
						sheet.last_rule = Some(css_rule);
					},
					Some(last_rule)=>{
						let mut last_rule_base_rule = css_stylesheet::css__stylesheet_get_base_rule(last_rule);
						last_rule_base_rule.next = Some(css_rule);
						base_rule.prev = sheet.last_rule;
						base_rule.next = None;
						sheet.last_rule = Some(css_rule);
					}
				}
			}
		}
		CSS_OK
	}
	
	pub fn css__stylesheet_remove_rule(sheet : @mut css_stylesheet, css_rule : CSS_RULE_DATA_TYPE) 
										-> css_result {
		match sheet._remove_selectors(css_rule) {
			CSS_OK=>{},
			_=>return CSS_INVALID
		}

		let mut base_rule = css_stylesheet::css__stylesheet_get_base_rule(css_rule);
		match base_rule.next {
			None=> {
				sheet.last_rule = base_rule.prev;
			},
			Some(base_rule_next)=>{
				let mut next_rule = css_stylesheet::css__stylesheet_get_base_rule(base_rule_next);
				next_rule.prev = base_rule.prev;
			}
		}

		match base_rule.prev {
			None=>{
				sheet.rule_list = base_rule.next ;
			},
			Some(base_rule_prev)=>{
				let mut prev_rule = css_stylesheet::css__stylesheet_get_base_rule(base_rule_prev);
				prev_rule.next = base_rule.next ;
			}
		}
		CSS_OK
	}

	pub fn _add_selectors(&mut self, css_rule : CSS_RULE_DATA_TYPE) -> css_result {
		match css_rule {
			RULE_SELECTOR(x) => {
				if x.base.parent_rule.is_some() {
					return CSS_INVALID;
				}

				for x.selectors.each_mut |_| {
					// do hash insert , for each selector
					//
				}
				CSS_OK
			},
			RULE_MEDIA(x) => {
				if x.base.parent_rule.is_some() {
					return CSS_INVALID;
				}

				let mut ptr = x.first_child;
				loop {
					match ptr {
						None=> return CSS_OK,
						Some(current_rule) => {
							match(self._add_selectors(current_rule))
							{
								CSS_OK => {
									ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
									loop;
								}

								_ => {

									let mut rptr = current_rule ;
									loop {
										match css_stylesheet::css__stylesheet_get_base_rule(rptr).prev {
											Some(y)=> { 
												self._remove_selectors(y);
												rptr = y;
												loop;
											},
											None=> { return CSS_INVALID },
										}
									}
								}
							}
						}
					}
				}
			},
			_ => {
				CSS_OK
			}
		}
	}

	pub fn _remove_selectors(&mut self, css_rule : CSS_RULE_DATA_TYPE) -> css_result {

		match css_rule {
			RULE_SELECTOR(x) => {
				for x.selectors.each_mut |_| {
					// do hash remove , for each selector
					// check for error result , if error - return error
				}
				CSS_OK
			},

			RULE_MEDIA(x)=> {

				let mut ptr = x.first_child;
				loop {
					match ptr {
						None=> return CSS_OK,
						Some(current_rule) => {
							match(self._remove_selectors(current_rule))
							{
								CSS_OK => {
									ptr = css_stylesheet::css__stylesheet_get_base_rule(current_rule).next;
									loop;
								}

								_ => return CSS_INVALID
							}
						}
					}
				}
			},
			_=>{CSS_OK}
		}
	}
}
