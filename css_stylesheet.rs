#[link(name = "css_stylesheet", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_bytecode;
extern mod css_enum;
extern mod std ;


use css_enum::* ;
use css_bytecode::*;
use core::managed::*;

static CSS_STYLE_DEFAULT_SIZE : uint = 16 ;

// /**< Qualified name of selector */
pub struct css_qname {  
	name:~str,
	ns:~str
}

pub struct css_selector_detail {
	qname:css_qname,      				 /**< Interned name */
	selector_type:css_selector_type,     /**< The type of selector  */
	combinator_type:css_combinator,      /**< The combinator type */
	value_type:css_selector_detail_value_type,  /**<   Value of selector  */
	negate:bool,						/**< Detail match is inverted */

	//css_selector_detail_value - union merged
	string:Option<~str>,
	a:int,
	b:int
}

/**< css_selector */
pub struct css_selector {
	combinator:Option<@mut css_selector>,   /**< Combining selector */
	rule:Option<CSS_RULE_DATA_TYPE>,		/**< Owning rule */
	specificity:uint,						/**< Specificity of selector */	
	data:~[@mut css_selector_detail]		/* *< Selector data */
}


pub struct css_style {
	bytecode:~[u32]        				
	//sheet:Option<@css_stylesheet>
}
pub struct hash_entry {
	selector:@mut css_selector,
	next:Option<@mut hash_entry>
}

/**< Hashtable of selectors */
pub struct css_selector_hash {
	default_slots:uint,
	elements:~[Option<@mut hash_entry>],
	classes:~[Option<@mut hash_entry>],
	ids:~[Option<@mut hash_entry>],
	universal:~[Option<@mut hash_entry>]
}

pub struct css_stylesheet {
	selectors:@mut css_selector_hash,   	/**< Hashtable of selectors */
	rule_count:uint,						/**< Number of rules in sheet */
	rule_list:Option<CSS_RULE_DATA_TYPE>,	/**< List of rules in sheet */
	last_rule:Option<CSS_RULE_DATA_TYPE>,   /**< Last rule in list */
	disabled:bool,				     		/**< Whether this sheet is  disabled */
	url:~str,								/**< URL of this sheet */
	title:~str,								/**< Title of this sheet */
	level:css_language_level,				/**< Language level of sheet */
	quirks_allowed:bool,					/**< Quirks permitted */
	quirks_used:bool,						/**< Quirks actually used */
	inline_style:bool,						/**< Is an inline style */
	cached_style:Option<@mut css_style>		/* *< Cache for style parsing */	
}

pub struct css_rule {
	parent_rule:Option<CSS_RULE_DATA_TYPE> ,         /**< containing parent rule */ 
	parent_stylesheet:Option<@mut css_stylesheet>,   /**< parent stylesheet */				
	prev:Option<CSS_RULE_DATA_TYPE>,				 /**< prev in list */
	next:Option<CSS_RULE_DATA_TYPE>,				/**< next in list */
	//rule_type:css_rule_type,
	index:uint//,items:uint							/**< index in sheet */
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


pub enum CSS_RULE_DATA_TYPE {
	RULE_UNKNOWN(@mut css_rule),
	RULE_SELECTOR(@mut css_rule_selector),
	RULE_CHARSET(@mut css_rule_charset),
	RULE_IMPORT(@mut css_rule_import),
	RULE_MEDIA(@mut css_rule_media),
	RULE_FONT_FACE(@mut css_rule_font_face),
	RULE_PAGE(@mut css_rule_page)
}

pub enum CSS_RULE_PARENT_TYPE {
	CSS_RULE_PARENT_STYLESHEET,
	CSS_RULE_PARENT_RULE
}

impl css_stylesheet {
	
	pub fn css__stylesheet_style_appendOPV(
										style: @mut css_style,
										opcode:css_properties_e,
										flags:u8,
										value:u16 ) {

		css_stylesheet::css__stylesheet_style_append(
			style,
			buildOPV(opcode,flags,value)
		)
	}

	pub fn css_stylesheet_style_inherit(
										style: @mut css_style,
										opcode:css_properties_e) {

		css_stylesheet::css__stylesheet_style_append(
			style,
			buildOPV_flag(opcode,FLAG_INHERIT,0) 
		)
	}

	/**
	 * Create a style
	 *
	 * \param self css_stylesheet
	 * \return css_style 
	 */
	pub fn css__stylesheet_style_create(&mut self ) -> @mut css_style {
		if self.cached_style.is_none() {
			@mut css_style{bytecode:~[]} 
		}
		else {
			self.cached_style.swap_unwrap()
		}
	}


	/**
	 * Merge a style to a CSS style
	 * 
	 * \param target The style to merge to
	 * \param style	 The style to merge
	 */
	
	pub fn css__stylesheet_merge_style(target : @mut css_style, style: @mut css_style) {
		target.bytecode += copy style.bytecode;
	}

	/**
	 * Append a style to a CSS style
	 * 
	 * \param target The style to add to
	 * \param style	 The style to add
	 */

	pub fn css__stylesheet_style_append(target : @mut css_style, bytecode: u32) {
		target.bytecode.push(bytecode);
	}
	
	/** append one or more css code entries to a style 
	 * 
	 * \param target The style to add to
	 * \param bytecodes	vector of style to add
	 */
	 
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
		sel_type: css_selector_type,
		qname : css_qname, 
		value_type : css_selector_detail_value_type,
		string_value : Option<~str> , 
		ab_value : Option<(int,int)>,
		negate:bool
	)  -> (css_result, Option<@mut css_selector_detail>) 
	{
		let detail : @mut css_selector_detail = @mut css_selector_detail{
			qname:qname,
			selector_type:sel_type,
			combinator_type:CSS_COMBINATOR_NONE,  
			value_type:value_type,
			negate:negate,

			//css_selector_detail_value - union merged
			string:None,
			a:0,
			b:0
		};
		
		match value_type {
			CSS_SELECTOR_DETAIL_VALUE_STRING=>  {
				if string_value.is_some() {
					detail.string=string_value ;
				}
			},
			CSS_SELECTOR_DETAIL_VALUE_NTH => 
				match ab_value { 
					None=> {},
					Some((x,y))=> { 
									detail.a=x ; 
									detail.b=y; 
								  }
				}
		}
		(CSS_OK,Some(detail)) 
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

	pub fn css__stylesheet_rule_set_nascent_import(
		css_rule : CSS_RULE_DATA_TYPE, url_str:~str, media:u64) -> css_result {

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

	pub fn css__stylesheet_rule_set_media(
		css_rule : CSS_RULE_DATA_TYPE, media:u64) -> css_result {

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

	pub fn css__stylesheet_rule_set_page_selector(
		css_rule : CSS_RULE_DATA_TYPE, selector:@mut css_selector) -> css_result {

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
	
	pub fn css__stylesheet_get_parent_type(css_rule :  CSS_RULE_DATA_TYPE) -> CSS_RULE_PARENT_TYPE {
		let base_rule = css_stylesheet::css__stylesheet_get_base_rule(css_rule);

		if (base_rule.parent_rule.is_some() && base_rule.parent_stylesheet.is_none()) {
			return CSS_RULE_PARENT_RULE;
		}

		if (base_rule.parent_rule.is_none() && base_rule.parent_stylesheet.is_some()) {
			return CSS_RULE_PARENT_STYLESHEET;
		}

		fail!(~"Parent type is ambiguous");
	}
	pub fn css__stylesheet_get_base_rule(css_rule : CSS_RULE_DATA_TYPE) -> @mut css_rule {
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

				let mut i : uint = 0 ;
				unsafe {
					while (i<x.selectors.len()) {
						match self.selectors.css__selector_hash_insert(x.selectors[i]) {
							CSS_OK=> { 
								i += 1;
								loop;
							} ,
							_=> {
								while (i>0){
									// Ignore errors 
									self.selectors.css__selector_hash_remove(x.selectors[i]);
									i -= 1;
								}
								// Remove zeroth element
								self.selectors.css__selector_hash_remove(x.selectors[i]);
								return CSS_INVALID;
							}
						}
					}
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

				for x.selectors.each_mut |&selector| {

					match self.selectors.css__selector_hash_remove(selector) {
						CSS_OK=> loop ,
						_=> return CSS_INVALID
					}
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

/////////////////////////////////////////////////////
// 			Implementation of css/src/select/hash.c
/////////////////////////////////////////////////////

pub enum css_hash_type {
	Element = 1,
	Class = 2,
	Ids = 3,
	Universal = 4
}

impl css_selector_hash {

	pub fn css__selector_hash_create() -> @mut css_selector_hash {
		let mut hash = @mut css_selector_hash{ 
						default_slots:(1<<6),
						elements:~[], 
						classes:~[], 
						ids:~[],
						universal:~[] 
		};
		for uint::range(0,hash.default_slots) |_| {
			hash.elements.push(None);
			hash.classes.push(None);
			hash.ids.push(None);
			hash.universal.push(None);
		}
		hash
	}
	
	pub fn _class_name(selector : @mut css_selector) 
						-> ~str {

		for selector.data.each_mut |&element| {
			match element.selector_type {
				CSS_SELECTOR_CLASS=>{
					if (element.negate == false) {
					return copy element.qname.name;
					}
				},
				_=>{}
			}
		}

		~""
	}

	pub fn _id_name(selector : @mut css_selector) 
						-> ~str {

		for selector.data.each_mut |&element| {
			match element.selector_type {
				CSS_SELECTOR_ID=>{
					if (element.negate == false) {
					return copy element.qname.name;
					}
				},
				_=>{}
			}
		}

		~""
	}


	pub fn _hash_name( string: ~str ) -> uint {

		let mut z: uint = 0x811c9dc5;
	    let mut i: uint = 0;
	    let mut string_index = str::char_len(string);
	    while string_index>0 {
	        z = z*0x01000193;
	        z = (z^(string[i]) as uint);
	        string_index = string_index-1;
	        i = i+1; 
	    }
	    z = z%4091;
	    z
	}
	
	pub fn css__selector_hash_insert(&mut self, selector : @mut css_selector) 
									-> css_result {
		unsafe {
			let mut mask :uint ;
			let mut index:uint=0;
			let mut name :~str ;
			if (vec::uniq_len(&selector.data) > 0){

				// Named Element
				if ( selector.data[0].qname.name.len() != 1) || 
					(str::char_at(selector.data[0].qname.name,0) != '*' ) {
						mask = self.default_slots-1 ;
						index = css_selector_hash::_hash_name(copy (selector.data[0].qname.name)) & mask ;
						return self._insert_into_chain(Element,index,selector);
				}

				// Named Class
				else if css_selector_hash::_class_name(selector).len() == 0  {
					name = css_selector_hash::_class_name(selector);
					mask = self.default_slots-1 ;
					index = css_selector_hash::_hash_name(name) & mask ;
					return self._insert_into_chain(Class,index,selector);
				}

				// Named Id
				else if css_selector_hash::_id_name(selector).len() == 0 {
					name = css_selector_hash::_id_name(selector);
					mask = self.default_slots-1 ;
					index = css_selector_hash::_hash_name(name) & mask ;
					return self._insert_into_chain(Ids,index,selector);
				}
				else{
					return self._insert_into_chain(Universal,index,selector);
				}
			}
			// Universal Chain
			return self._insert_into_chain(Universal,index,selector);
		}
	}

	
	pub fn _insert_into_chain(&mut self, 
							hash_type : css_hash_type,
							index:uint,
							selector : @mut css_selector) 
							-> css_result {

		let mut hash_entry_list = 
				match hash_type {
					Element => &mut self.elements ,
					Class => &mut self.classes ,
					Ids =>  &mut self.ids ,
					Universal => &mut self.universal ,
				};
		let mut entry = @mut hash_entry{
				selector:selector,
				next:None
		};
		//&~[Option<@mut hash_entry>] 

		match (*hash_entry_list)[index] {
			None=>{
				(*hash_entry_list)[index] = Some(entry);
			},
			Some(index_element)=>{

				let mut search = index_element;
				let mut prev = index_element ;
				let mut first_pos : bool = true ;
				loop {
					if( search.selector.specificity> selector.specificity ) {
						break ;
					}

					if( search.selector.specificity == selector.specificity){
						if(search.selector.rule.is_none() || selector.rule.is_none() ){
							return CSS_BADPARM ;
						}

						let mut base_search_rule = css_stylesheet::css__stylesheet_get_base_rule(search.selector.rule.get());
						let mut base_selector_rule = css_stylesheet::css__stylesheet_get_base_rule(selector.rule.get());

						if(base_search_rule.index > base_selector_rule.index) {
							break ;
						}
					}

					prev = search ;
					search = 
						match search.next {
							None=>{
								break ;
							},
							Some(next_ptr)=>{
								first_pos = false ;
								next_ptr
							}
					};
				}
				if(first_pos){
					(*hash_entry_list)[index] = Some(entry);
					entry.next = Some(search);
				}
				else {
					prev.next= Some(entry);
					entry.next=Some(search);
				}
			}
		}
		CSS_OK
	}

	pub fn css__selector_hash_remove(&mut self, selector : @mut css_selector) 
									-> css_result {
		unsafe {
			let mut mask :uint ;
			let mut index:uint=0;
			let mut name :~str ;
			if (vec::uniq_len(&selector.data) > 0){

				// Named Element
				if ( selector.data[0].qname.name.len() != 1) || 
					(str::char_at(selector.data[0].qname.name,0) != '*' ) {
						mask = self.default_slots-1 ;
						index = css_selector_hash::_hash_name(copy (selector.data[0].qname.name)) & mask ;
						return self._remove_from_chain(Element,index,selector);
				}

				// Named Class
				else if css_selector_hash::_class_name(selector).len() == 0  {
					name = css_selector_hash::_class_name(selector);
					mask = self.default_slots-1 ;
					index = css_selector_hash::_hash_name(name) & mask ;
					return self._remove_from_chain(Class,index,selector);
				}

				// Named Id
				else if css_selector_hash::_id_name(selector).len() == 0 {
					name = css_selector_hash::_id_name(selector);
					mask = self.default_slots-1 ;
					index = css_selector_hash::_hash_name(name) & mask ;
					return self._remove_from_chain(Ids,index,selector);
				}
				else{
					return self._remove_from_chain(Universal,index,selector);
				}
			}
			// Universal Chain
			return self._remove_from_chain(Universal,index,selector);
		}
	}

	pub fn _remove_from_chain(&mut self, 
							hash_type : css_hash_type,
							index:uint,
							selector : @mut css_selector) 
							-> css_result {

		let mut hash_entry_list = 
			match hash_type {
				Element => &mut self.elements ,
				Class => &mut self.classes ,
				Ids =>  &mut self.ids ,
				Universal => &mut self.universal ,
			};
		//&~[Option<@mut hash_entry>] 

		match (*hash_entry_list)[index] {
			None=>{
				return CSS_INVALID ;
			},
			Some(index_element)=>{

				let mut search = index_element;
				let mut prev = index_element ;
				let mut first_pos : bool = true ;

				loop {

					if (mut_ptr_eq(selector,search.selector) == true ) {
						break;
					}

					prev = search ;
					search = 
						match search.next {
							None=>{
								return CSS_INVALID ;
							},
							Some(next_ptr)=>{
								first_pos = false ;
								next_ptr
							}
					};
				}
				if(first_pos){
					(*hash_entry_list)[index] = search.next;
				}
				else {
					prev.next= search.next;
				}
			}
		}
		CSS_OK
	}

	pub fn is_string_caseless_equal(a : &str , b : &str ) -> bool {

		if ( a.len() != b.len() ) {
			return false ;
		}
		
		let mut i :uint = a.len() ;
		for uint::range(0,i) |e| {
			if a[e] == b[e] {
				loop;
			}

			if (a[e] >= 'A' as u8  && a[e] <= 'Z'  as u8) {
			  	if (a[e]+32) == b[e] {
					loop;
				}
				else {
					return false ;
				}
			}

			if (b[e] >= 'A'  as u8 && b[e] <= 'Z'  as u8) {
			  	if (b[e]+32) == a[e] {
					loop;
				}
				else {
					return false ;
				}
			}
			return false ;
		}
		return true ;
	}

	pub fn css__selector_hash_find(&mut self,
								name : ~str) 
								-> (Option<@mut hash_entry>,css_result) {

		let mut mask  = self.default_slots-1 ;
		let mut index = css_selector_hash::_hash_name(copy name) & mask ; 
		let mut head = self.elements[index];

		loop {
			match head {
				None=>{
					return (None,CSS_OK);
				},
				Some(node_element)=>{

					for node_element.selector.data.each_mut |&detail_element| {
						unsafe {
							if css_selector_hash::is_string_caseless_equal(
								detail_element.qname.name,name) {
								return (head,CSS_OK);
							}
						}
					}

					match node_element.next {
						None=> {
							return (None,CSS_OK);
						},
						Some(_)=>{
							head = node_element.next ;
							loop ;
						}
					}
				}
			}
		}
	}
	

	pub fn css__selector_hash_find_by_class(&mut self,
								name : ~str) 
								-> (Option<@mut hash_entry>,css_result) {

		let mut mask  = self.default_slots-1 ;
		let mut index = css_selector_hash::_hash_name(copy name) & mask ; 
		let mut head = self.classes[index];

		loop {
			match head {
				None=>{
					return (None,CSS_OK);
				},
				Some(node_element)=>{

					let mut n = css_selector_hash::_class_name(node_element.selector);

					unsafe {
						if css_selector_hash::is_string_caseless_equal(n, name) {
							return (head,CSS_OK);
						}
					}

					match node_element.next {
						None=> {
							return (None,CSS_OK);
						},
						Some(_)=>{
							head = node_element.next ;
							loop ;
						}
					}
				}
			}
		}
	}

	pub fn css__selector_hash_find_by_id(&mut self,
								name : ~str) 
								-> (Option<@mut hash_entry>,css_result) {

		let mut mask  = self.default_slots-1 ;
		let mut index = css_selector_hash::_hash_name(copy name) & mask ; 
		let mut head = self.ids[index];

		loop {
			match head {
				None=>{
					return (None,CSS_OK);
				},
				Some(node_element)=>{

					let mut n = css_selector_hash::_id_name(node_element.selector);

					unsafe {
						if css_selector_hash::is_string_caseless_equal(n, name) {
							return (head,CSS_OK);
						}
					}

					match node_element.next {
						None=> {
							return (None,CSS_OK);
						},
						Some(_)=>{
							head = node_element.next ;
							loop ;
						}
					}
				}
			}
		}
	}


	pub fn css__selector_hash_find_universal(&mut self) 
								-> (Option<@mut hash_entry>,css_result) {

		let mut head = self.universal[0] ;
		match head {
			None=>{
				return (None,CSS_OK);
			},
			Some(_)=>{
				return (self.universal[0],CSS_OK);
			}
		}
	}

	pub fn _iterate_elements(current : @mut hash_entry) 
							-> (Option<@mut hash_entry>,css_result) {

		let mut head = current;

		loop {
			match head.next {
				None=>{
					return (None,CSS_OK);
				},
				Some(next_entry)=>{
					unsafe {
						if vec::uniq_len(&head.selector.data)==0 || 
							vec::uniq_len(&next_entry.selector.data)==0 {
							return (None,CSS_INVALID);
						}
						if css_selector_hash::is_string_caseless_equal(
							head.selector.data[0].qname.name,
							next_entry.selector.data[0].qname.name) == true {

							return (head.next,CSS_OK);
						}
						head = next_entry ;
						loop ;
					}
				}
			}
		}
	}

	pub fn _iterate_classes(current : @mut hash_entry) 
							-> (Option<@mut hash_entry>,css_result) {

		let mut head = current;

		let mut current_refer = css_selector_hash::_class_name(current.selector);

		loop {
			match head.next {
				None=>{
					return (None,CSS_OK);
				},
				Some(next_entry)=>{
					unsafe {
						let mut name = css_selector_hash::_class_name(next_entry.selector);
						if( name.len()==0){
							loop;
						}
						if css_selector_hash::is_string_caseless_equal(name,current_refer) == true {
							return (current.next,CSS_OK);
						}
						head = next_entry ;
						loop ;
					}
				}
			}
		}
		return (None,CSS_OK);
	}

	pub fn _iterate_ids(current : @mut hash_entry) 
							-> (Option<@mut hash_entry>,css_result) {

		let mut head = current;

		let mut current_refer = css_selector_hash::_id_name(current.selector);

		loop {
			match head.next {
				None=>{
					return (None,CSS_OK);
				},
				Some(next_entry)=>{
					unsafe {
						let mut name = css_selector_hash::_id_name(next_entry.selector);
						if( name.len()==0){
							loop;
						}
						if css_selector_hash::is_string_caseless_equal(name,current_refer) == true {
							return (current.next,CSS_OK);
						}
						head = next_entry ;
						loop ;
					}
				}
			}
		}
		return (None,CSS_OK);
	}

	pub fn _iterate_universal(current : @mut hash_entry) 
							-> (Option<@mut hash_entry>,css_result) {

		if current.next.is_some() {
			return (current.next,CSS_OK);
		}
		(None,CSS_OK)
	}
}


/////////////////////////////////////////////////////