
use include::types::*;
use include::font_face::*;
use bytecode::bytecode::*;
use utils::errors::*;
use select::common::*;
use select::dispatch::*;
use stylesheet::*;

use core::managed::*;

/*
 * Container for stylesheet selection info
 */
pub struct css_select_sheet {
	sheet:@mut css_stylesheet,
	origin:css_origin,
	media:u64
}

/*
 * CSS selection context
 */
struct css_select_ctx {
	sheets:~[@mut css_select_sheet],

	/* Useful interned strings */
	universal:~str,
	first_child:~str,
	link:~str,
	visited:~str,
	hover:~str,
	active:~str,
	focus:~str,
	nth_child:~str,
	nth_last_child:~str,
	nth_of_type:~str,
	nth_last_of_type:~str,
	last_child:~str,
	first_of_type:~str,
	last_of_type:~str,
	only_child:~str,
	only_of_type:~str,
	root:~str,
	empty:~str,
	target:~str,
	lang:~str,
	enabled:~str,
	disabled:~str,
	checked:~str,
	first_line:~str,
	first_letter:~str,
	before:~str,
	after:~str
}

/*
 * Container for selected font faces
 */
pub struct css_select_font_faces_list {
	font_faces:~[Option<@mut css_font_face>]
}

/*
 * Font face selection state
 */
pub struct css_select_font_faces_state {
	font_family:~str,
	media:u64,

	ua_font_faces:css_select_font_faces_list,
	user_font_faces:css_select_font_faces_list,
	author_font_faces:css_select_font_faces_list
}

//////////////////////////////////////////////////////////////////
// Start of CSS Selector internal functions
//////////////////////////////////////////////////////////////////
impl css_select_ctx {

	pub fn css_select_ctx_create() -> (css_error,Option<~css_select_ctx>) {
		let mut error : css_error ;
		let mut result = ~css_select_ctx {
			sheets:~[],

			universal:~"",
			first_child:~"",
			link:~"",
			visited:~"",
			hover:~"",
			active:~"",
			focus:~"",
			nth_child:~"",
			nth_last_child:~"",
			nth_of_type:~"",
			nth_last_of_type:~"",
			last_child:~"",
			first_of_type:~"",
			last_of_type:~"",
			only_child:~"",
			only_of_type:~"",
			root:~"",
			empty:~"",
			target:~"",
			lang:~"",
			enabled:~"",
			disabled:~"",
			checked:~"",
			first_line:~"",
			first_letter:~"",
			before:~"",
			after:~""
		};

		error = css_select_ctx::intern_strings(&mut result);
		match error {
			CSS_OK => {},
			x => {
				return (x,None) ;
			}
		}
		(CSS_OK,Some(result))
	}

	pub fn css_select_ctx_destroy(&mut self) -> css_error {
		// need to check , if final outcome of select contains lwc_string
		// then we will need destroy function , to unref lwc_strings
		CSS_OK
	}

	pub fn intern_strings(ctx: &mut ~css_select_ctx) -> css_error {

		/* Universal selector */
		ctx.universal = ~"*" ;

		/* Pseudo classes */
		ctx.first_child = ~"first_child" ;
		ctx.link = ~"link" ;
		ctx.visited = ~"visited" ;
		ctx.hover = ~"hover" ;
		ctx.active = ~"active" ;
		ctx.focus = ~"focus" ;
		ctx.nth_child = ~"nth_child" ;
		ctx.nth_last_child = ~"nth_last_child" ;
		ctx.nth_of_type = ~"nth_of_type" ;
		ctx.nth_last_of_type = ~"nth_last_of_type" ;
		ctx.last_child = ~"last_child" ;
		ctx.first_of_type = ~"first_of_type" ;
		ctx.last_of_type = ~"last_of_type" ;
		ctx.only_child = ~"only_child" ;
		ctx.only_of_type = ~"only_of_type" ;
		ctx.root = ~"root" ;
		ctx.empty = ~"empty" ;
		ctx.target = ~"target" ;
		ctx.lang = ~"lang" ;
		ctx.enabled = ~"enabled" ;
		ctx.disabled = ~"disabled" ;
		ctx.checked = ~"checked" ;

		/* Pseudo elements */
		ctx.first_line = ~"first_line" ;
		ctx.first_letter = ~"first_letter" ;
		ctx.before = ~"before" ;
		ctx.after = ~"after" ;

		CSS_OK
	}

	pub fn css_select_ctx_append_sheet(&mut self,
									sheet:@mut css_stylesheet,
									origin:css_origin,
									media:u64) 
									-> css_error {

		self.css_select_ctx_insert_sheet(sheet,origin,media)
	}

	pub fn css_select_ctx_insert_sheet(&mut self,
									csheet:@mut css_stylesheet,
									corigin:css_origin,
									cmedia:u64) 
									-> css_error {

		if (csheet.inline_style) {
			return CSS_INVALID ;
		}

		let mut select_sheet = @mut css_select_sheet{
			sheet:csheet,
			origin:corigin,
			media:cmedia
		};

		self.sheets.push(select_sheet);
		CSS_OK
	}

	pub fn css_select_ctx_remove_sheet(&mut self, csheet:@mut css_stylesheet)-> css_error {

		let mut i = self.sheets.len() ;
		while (i>0) {
			i = i - 1 ;
			if ( mut_ptr_eq(self.sheets[i].sheet,csheet) ) {
				self.sheets.remove(i);
				return CSS_OK ;
			}
		}
		CSS_INVALID
	}

	pub fn css_select_ctx_count_sheets(&mut self) -> (css_error,uint) {

		(CSS_OK,self.sheets.len())
	}

	pub fn css_select_ctx_get_sheet(&mut self, index:uint) 
								-> (css_error,Option<@mut css_stylesheet>) {

		if ( index >= self.sheets.len() ) {
			return (CSS_INVALID,None) ;
		}

		(CSS_OK,Some(self.sheets[index].sheet))
	} 

	// pub fn css_select_style(&mut self) -> css_error {

	// }

	pub fn css_select_results_destroy(results: &mut ~[@mut css_select_results] ) -> css_error {
		results.clear() ;
		CSS_OK
	}

	pub fn css_select_font_faces(&mut self,
								media:u64,
								font_family:~str) 
								-> (css_error,Option<css_select_font_faces_results>) {

		let mut results : Option<css_select_font_faces_results> = None ;
		let mut error = CSS_OK ;

		if(font_family.len()==0) {
			return (CSS_BADPARM,results) ;
		}

		let mut state = @mut css_select_font_faces_state {
			font_family:copy font_family,
			media:media,

			ua_font_faces:css_select_font_faces_list{font_faces:~[]},
			user_font_faces:css_select_font_faces_list{font_faces:~[]},
			author_font_faces:css_select_font_faces_list{font_faces:~[]}
		};

		for self.sheets.each |select_sheet| {

			if ((select_sheet.media & media) != 0 ) && 
				(select_sheet.sheet.disabled == false ) {

				error = self.select_font_faces_from_sheet(select_sheet.sheet,
														select_sheet.origin,state);
				match error {
					CSS_OK=>{} ,
					x => {
						return (x,None) ;
					}
				}
			}
		}

		(error,results)
	}


	pub fn select_font_faces_from_sheet(&self,
										sheet:@mut css_stylesheet,
										origin: css_origin,
										state:@mut css_select_font_faces_state)
										-> css_error {

		CSS_OK
	}

	pub fn _select_font_face_from_rule(rule:@mut css_rule_font_face,
									origin: css_origin,
									state:@mut css_select_font_faces_state) 
									-> css_error {


		CSS_OK
	}

	pub fn _rule_applies_to_media(rule: Option<CSS_RULE_DATA_TYPE>, media:u64) -> bool {

		let mut applies : bool = true;
		let mut ancestor = rule;

		loop {	
			match ancestor {
				None=>{
					break ;
				},
				Some(ancestor_rule)=> {
					match ancestor_rule {
			            RULE_MEDIA(r)=>{
			                if( ( r.media & media ) == 0 ) {
			                	applies = false ;
			                	return applies ;
			                }

			                if r.base.parent_stylesheet.is_none() {
			                	ancestor = r.base.parent_rule ;
			                }
			                else {
			                	ancestor = None ;
			                }
			                loop ;
			            },
			            _ => {
			            	let mut ancestor_base = css_stylesheet::css__stylesheet_get_base_rule(ancestor_rule);
			            	if ancestor_base.parent_stylesheet.is_none() {
			                	ancestor = ancestor_base.parent_rule ;
			                }
			                else {
			                	ancestor = None ;
			                }
			                loop ;
			            }
			        }
		    	}
	    	}
    	}
		applies
	}

	pub fn cascade_style(style:@mut css_style, state:@mut css_select_state) -> css_error {
		let mut s = style;

		while (s.used > 0) {
			let mut op: u32;
			let mut error : css_error ;
			let mut opv = peek_bytecode(s);

			advance_bytecode(s);

			op = getOpcode(opv) as u32;

			let mut dispatch_cascade = dispatch_table::get_cascade_ptr(op as uint) ;
            error =  dispatch_cascade(opv, s, state);

			match error {
				CSS_OK => {},
				x => {
					return x ;
				}
			}
		}

		CSS_OK
	}
}


//////////////////////////////////////////////////////////////////