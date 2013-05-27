
use include::types::*;
use include::font_face::*;
use utils::errors::*;
use select::common::*;
use stylesheet::*;

/*
 * Container for stylesheet selection info
 */
pub struct css_select_sheet {
	sheet:Option<@mut css_stylesheet>,
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

	pub fn css_select_ctx_create() -> (css_error,Option<@mut css_select_ctx>) {
		let mut error : css_error ;
		let mut result = @mut css_select_ctx {
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

		error = css_select_ctx::intern_strings(result);
		match error {
			CSS_OK => {},
			x => {
				return (x,None) ;
			}
		}
		(CSS_OK,Some(result))
	}

	pub fn intern_strings(ctx:@mut css_select_ctx) -> css_error {
		CSS_OK
	}
}


//////////////////////////////////////////////////////////////////