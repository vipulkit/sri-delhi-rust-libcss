#[link(name = "css_select_computed", vers = "0.1")];
#[crate_type = "lib"];


pub struct css_computed_counter {
	name:~str ,
	value:u8
}

pub enum css_computed_content_item_type {
	TYPE_STRING,
	TYPE_URI,
	TYPE_ATTR,
	TYPE_COUNTER,
	TYPE_COUNTERS_WITH_SEP
}

pub struct css_computed_content_item_counter {
	name:~str,
	sep:~str,
	style:Option<u8>
}

pub struct css_computed_content_item {
	computed_type:u8,
	item_type:css_computed_content_item_type,

	data:Option<~str>,
	counters_data:Option<css_computed_content_item_counter>
}

pub struct css_computed_uncommon {
/*
 * border_spacing		  1 + 2(4)	  2(4)
 * clip				  2 + 4(4) + 4	  4(4)
 * letter_spacing		  2 + 4		  4
 * outline_color		  2		  4
 * outline_width		  3 + 4		  4
 * word_spacing			  2 + 4		  4
 * 				---		---
 * 				 52 bits	 40 bytes
 *
 * Encode counter_increment and _reset as an array of name, value pairs,
 * terminated with a blank entry.
 *
 * counter_increment		  1		  sizeof(ptr)
 * counter_reset		  1		  sizeof(ptr)
 * 				---		---
 * 				  2 bits	  2sizeof(ptr) bytes
 *
 * Encode cursor uri(s) as an array of string objects, terminated with a
 * blank entry.
 *
 * cursor			  5		  sizeof(ptr)
 * 				---		---
 * 				  5 bits	  sizeof(ptr) bytes
 *
 * Encode content as an array of content items, terminated with a blank entry.
 *
 * content			  2		  sizeof(ptr)
 * 				---		---
 * 				  2 bits	  sizeof(ptr)
 *
 * 				___		___
 * 				 61 bits	 40 + 4sizeof(ptr) bytes
 *
 * 				  8 bytes	 40 + 4sizeof(ptr) bytes
 * 				===================
 * 				 48 + 4sizeof(ptr) bytes
 *
 * Bit allocations:
 *
 *    76543210
 *  1 llllllcc	letter-spacing | outline-color
 *  2 ooooooob	outline-width  | border-spacing
 *  3 bbbbbbbb	border-spacing
 *  4 wwwwwwir	word-spacing   | counter-increment | counter-reset
 *  5 uuuuu...	cursor         | <unused>
 *  6 cccccccc	clip
 *  7 cccccccc	clip
 *  8 ccccccoo	clip           | content
 */
	bits:~[u8, ..8],

	border_spacing:~[u8, ..2],

	clip:~[u8, ..4],

	letter_spacing:u8,

 	outline_color:u32,
	outline_width:u8,

	word_spacing:u8,

	counter_increment:@mut css_computed_counter,
	counter_reset:@mut css_computed_counter,

	cursor:~[~str],

	content:@mut css_computed_content_item,
}

pub struct css_computed_page {
/*
 * page_break_after		  3
 * page_break_before		  3
 * page_break_inside		  2
 * 				---
 *				  8 bits
 */
    bits:~[u8, ..2]
} 
    
pub struct css_computed_style {
/*
 * background_attachment	  2
 * background_repeat		  3
 * border_collapse		  2
 * border_top_style		  4
 * border_right_style		  4
 * border_bottom_style		  4
 * border_left_style		  4
 * caption_side			  2
 * clear			  3
 * direction			  2
 * display			  5
 * empty_cells			  2
 * float			  2
 * font_style			  2
 * font_variant			  2
 * font_weight			  4
 * list_style_position		  2
 * list_style_type		  4
 * overflow			  3
 * outline_style		  4
 * position			  3
 * table_layout			  2
 * text_align			  4
 * text_decoration		  5
 * text_transform		  3
 * unicode_bidi			  2
 * visibility			  2
 * white_space			  3
 *				---
 *				 84 bits
 *
 * Colours are 32bits of AARRGGBB
 * Dimensions are encoded as a fixed point value + 4 bits of unit data
 *
 * background_color		  2		  4
 * background_image		  1		  sizeof(ptr)
 * background_position		  1 + 2(4)	  2(4)
 * border_top_color		  2		  4
 * border_right_color		  2		  4
 * border_bottom_color		  2		  4
 * border_left_color		  2		  4
 * border_top_width		  3 + 4		  4
 * border_right_width		  3 + 4		  4
 * border_bottom_width		  3 + 4		  4
 * border_left_width		  3 + 4		  4
 * top				  2 + 4		  4
 * right			  2 + 4		  4
 * bottom			  2 + 4		  4
 * left				  2 + 4		  4
 * color			  1		  4
 * font_size			  4 + 4		  4
 * height			  2 + 4		  4
 * line_height			  2 + 4		  4
 * list_style_image		  1		  sizeof(ptr)
 * margin_top			  2 + 4		  4
 * margin_right			  2 + 4		  4
 * margin_bottom		  2 + 4		  4
 * margin_left			  2 + 4		  4
 * max_height			  2 + 4		  4
 * max_width			  2 + 4		  4
 * min_height			  1 + 4		  4
 * min_width			  1 + 4		  4
 * padding_top			  1 + 4		  4
 * padding_right		  1 + 4		  4
 * padding_bottom		  1 + 4		  4
 * padding_left			  1 + 4		  4
 * text_indent			  1 + 4		  4
 * vertical_align		  4 + 4		  4
 * width			  2 + 4		  4
 * z_index			  2		  4
 * 				---		---
 *				181 bits	140 + 2sizeof(ptr) bytes
 *
 * Encode font family as an array of string objects, terminated with a 
 * blank entry.
 *
 * font_family			  3		  sizeof(ptr)
 * 				---		---
 * 				  3 bits	  sizeof(ptr)
 *
 * Encode quotes as an array of string objects, terminated with a blank entry.
 *
 * quotes			  1		  sizeof(ptr)
 * 				---		---
 * 				  1 bit		  sizeof(ptr) bytes
 *
 * 				___		___
 *				269 bits	140 + 4sizeof(ptr) bytes
 *
 *				 34 bytes	140 + 4sizeof(ptr) bytes
 *				===================
 *				174 + 4sizeof(ptr) bytes
 *
 * Bit allocations:
 *
 *    76543210
 *  1 vvvvvvvv	vertical-align
 *  2 ffffffff	font-size
 *  3 ttttttti	border-top-width    | background-image
 *  4 rrrrrrrc	border-right-width  | color
 *  5 bbbbbbbl	border-bottom-width | list-style-image
 *  6 lllllllq	border-left-width   | quotes
 *  7 ttttttcc	top                 | border-top-color
 *  8 rrrrrrcc	right               | border-right-color
 *  9 bbbbbbcc	bottom              | border-bottom-color
 * 10 llllllcc	left                | border-left-color
 * 11 hhhhhhbb	height              | background-color
 * 12 llllllzz	line-height         | z-index
 * 13 ttttttbb	margin-top          | background-attachment
 * 14 rrrrrrbb	margin-right        | border-collapse
 * 15 bbbbbbcc	margin-bottom       | caption-side
 * 16 lllllldd	margin-left         | direction
 * 17 mmmmmmee	max-height          | empty-cells
 * 18 mmmmmmff	max-width           | float
 * 19 wwwwwwff	width               | font-style
 * 20 mmmmmbbb	min-height          | background-repeat
 * 21 mmmmmccc	min-width           | clear
 * 22 tttttooo	padding-top         | overflow
 * 23 rrrrrppp	padding-right       | position
 * 24 bbbbbo..	padding-bottom      | opacity               | <unused>
 * 25 lllllttt	padding-left        | text-transform
 * 26 tttttwww	text-indent         | white-space
 * 27 bbbbbbbb	background-position
 * 28 bdddddff	background-position | display               | font-variant
 * 29 tttttfff	text-decoration     | font-family
 * 30 ttttrrrr	border-top-style    | border-right-style
 * 31 bbbbllll	border-bottom-style | border-left-style
 * 32 ffffllll	font-weight         | list-style-type
 * 33 oooottuu	outline-style       | table-layout          | unicode-bidi
 * 34 vvlltttt	visibility          | list-style-position   | text-align
 */
	bits:~[u8, ..34],

	unused:~[u8, ..2],

	background_color:u32,

	background_image:~str,

	background_position:[u8, ..2],

	border_color:[u32, ..4],
	border_width:[u8, ..4],

	top:u8,
	right:u8,
	bottom:u8,
	left:u8,

	color:u32,

	font_size:u8,

	height:u8,

	line_height:u8,

	list_style_image:~str,

	margin:~[u8, ..4],

	max_height:u8,
	max_width:u8,

	min_height:u8,
	min_width:u8,

	opacity:u8,

	padding:~[u8, ..4],

	text_indent:u8,

	vertical_align:u8,

	width:u8,

	z_index:i32,

	//lwc_string **font_family;

	//lwc_string **quotes;

	uncommon:Option<@mut css_computed_uncommon>, /**< Uncommon properties */
	// void *aural;			/**< Aural properties */
	page:@mut css_computed_page	/* *< Page properties */

}

