
use include::types::*;
use include::font_face::*;
use utils::errors::*;
use select::common::*;
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
	n_sheets:u32,

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


pub fn css__outranks_existing(op:u16, 
							important:bool, 
							state: @mut css_select_state,
							inherit:bool) -> bool {
	true 
}

pub fn advance_bytecode(style: @mut css_style) {
	unsafe{
	 	if (style.bytecode.len() - style.used > 1) {
			style.used += 1	
		}
		else {
			fail!(~"Advancing Bytecode vector after end index")
		}
	}
}	

pub fn peek_bytecode(style: @mut css_style) -> u32 {
	unsafe{
		if style.bytecode.len() - style.used > 0 {
			style.bytecode[style.used] 
		}
		else {
			fail!(~"Advancing Bytecode vector after end index")
		}
	}
}


//////////////////////////////////////////////////////////////////
// Start of CSS Selector internal functions
//////////////////////////////////////////////////////////////////
impl css_select_ctx {

	pub fn css_select_ctx_create() -> (css_error,Option<~css_select_ctx>) {
		let mut error : css_error ;
		let mut result = ~css_select_ctx {
			n_sheets:0,

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
}


//////////////////////////////////////////////////////////////////