#[link(name = "css_select_computed", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_select_const;
extern mod css_fpmath;
extern mod std ;

use css_enum::* ;
use css_select_const::*;
use css_fpmath::*;

pub struct css_computed_counter {
	name:~str ,
	value:i32
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

	border_spacing:~[i32, ..2],

	clip:~[i32, ..4],

	letter_spacing:i32,

 	outline_color:u32,
	outline_width:i32,

	word_spacing:i32,

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
    bits:~[u8, ..1]
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

	background_position:[i32, ..2],

	border_color:[u32, ..4],
	border_width:[i32, ..4],

	top:i32,
	right:i32,
	bottom:i32,
	left:i32,

	color:u32,

	font_size:i32,

	height:i32,

	line_height:i32,

	list_style_image:~str,

	margin:~[i32, ..4],

	max_height:i32,
	max_width:i32,

	min_height:i32,
	min_width:i32,

	opacity:i32,

	padding:~[i32, ..4],

	text_indent:i32,

	vertical_align:i32,

	width:i32,

	z_index:i32,

	//lwc_string **font_family;

	//lwc_string **quotes;
	quotes:~[~str],

	uncommon:Option<@mut css_computed_uncommon>, /**< Uncommon properties */
	// void *aural;			/**< Aural properties */
	page:@mut css_computed_page	/* *< Page properties */

}


pub struct css_computed_clip_rect {
	top:i32,
	right:i32,
	bottom:i32,
	left:i32,

	tunit:css_unit,
	runit:css_unit,
	bunit:css_unit,
	lunit:css_unit,

	top_auto:bool,
	right_auto:bool,
	bottom_auto:bool,
	left_auto:bool
}


////////////////////////////////////

pub fn css_computed_letter_spacing(
		style : @mut css_computed_style) 
		-> (u8,Option<i32>,Option<css_unit>) {

    let mut length :Option<i32> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            (CSS_LETTER_SPACING_NORMAL as u8,length,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_LETTER_SPACING_INDEX];
            bits = bits & (CSS_LETTER_SPACING_MASK as u8);
            bits = bits >> CSS_LETTER_SPACING_SHIFT;

            if (bits&3) == (CSS_LETTER_SPACING_SET as u8) { 
                length = Some(uncommon_struct.letter_spacing);
                unit = Some(unsafe { cast::transmute((bits >> 2)as int) }) ;
            }

            ((bits&3),length,unit)
        }
    }
}

pub fn css_computed_outline_color(
                    style: @mut css_computed_style) 
                    -> (u8,Option<u32>) {

    let mut color : Option<u32> = None;
    match style.uncommon {
        None=>{
            (CSS_OUTLINE_COLOR_INVERT as u8,color)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_OUTLINE_COLOR_INDEX];
            bits = bits & (CSS_OUTLINE_COLOR_MASK as u8) ;
            bits = bits >> CSS_OUTLINE_COLOR_SHIFT ;

            if (bits&3) == (CSS_OUTLINE_COLOR_COLOR as u8) { 
                color = Some(uncommon_struct.outline_color) ;
            }

            ((bits&3),color)
        }
    }
}


pub fn css_computed_outline_width(
        style : @mut css_computed_style) 
        -> (u8,Option<i32>,Option<css_unit>) {

    let mut width :Option<i32> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            width = Some(css_int_to_fixed(2));
            unit = Some(CSS_UNIT_PX);
            (CSS_OUTLINE_WIDTH_WIDTH as u8,width,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_OUTLINE_WIDTH_INDEX];
            bits = bits & (CSS_OUTLINE_WIDTH_MASK as u8);
            bits = bits >> CSS_OUTLINE_WIDTH_SHIFT;

            if (bits&7) == (CSS_OUTLINE_WIDTH_WIDTH as u8) { 
                width = Some(uncommon_struct.outline_width);
                unit = Some(unsafe { cast::transmute((bits >> 3)as int) }) ;
            }

            ((bits&3),width,unit)
        }
    }
}

pub struct border_spacing_result {
    hlength:i32,
    vlength:i32,
    hunit:css_unit,
    vunit:css_unit,
    result:u8
}

pub fn css_computed_border_spacing(
                    style : @mut css_computed_style) 
                    -> border_spacing_result {

    let mut result = 
            border_spacing_result{
                hlength:0,
                vlength:0,
                hunit:CSS_UNIT_PX,
                vunit:CSS_UNIT_PX,
                result:CSS_BORDER_SPACING_SET as u8
            };

    match style.uncommon {
        None=>{
            result
        },
        Some(uncommon_struct)=>{
            let mut bits = uncommon_struct.bits[CSS_BORDER_SPACING_INDEX];
            bits = bits & (CSS_BORDER_SPACING_MASK as u8);
            bits = bits >> CSS_BORDER_SPACING_SHIFT ;

            if bits == (CSS_BORDER_SPACING_SET as u8) { 
                let mut bits1 = uncommon_struct.bits[2];
                bits1 = bits1 & (CSS_BORDER_SPACING_MASK1 as u8);
                bits1 = bits1 >> CSS_BORDER_SPACING_SHIFT1 ;

                result.hlength = uncommon_struct.border_spacing[CSS_BORDER_SPACING_INDEX1];
                result.hunit = unsafe { cast::transmute((bits1 >> 4)as int) } ;

                result.vlength = uncommon_struct.border_spacing[1];
                result.vunit = unsafe { cast::transmute((bits1 & 0xf)as int) } ;
            }

            result.result = bits ;
            result
        }
    }
}

pub fn css_computed_word_spacing(
                    style : @mut css_computed_style)
                    -> (u8,Option<i32>,Option<css_unit>) {

    let mut length :Option<i32> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            (CSS_WORD_SPACING_NORMAL as u8,length,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_WORD_SPACING_INDEX];
            bits = bits & (CSS_WORD_SPACING_MASK as u8);
            bits = bits >> CSS_WORD_SPACING_SHIFT;

            if (bits&3) == (CSS_WORD_SPACING_SET as u8) { 
                length = Some(uncommon_struct.word_spacing);
                unit = Some(unsafe { cast::transmute((bits >> 2)as int) }) ;
            }

            ((bits&3),length,unit)
        }
    }

}

pub fn css_computed_counter_increment(
                        style : @mut css_computed_style)
                        -> (u8,Option<@mut css_computed_counter>) {

    let mut counter :Option<@mut css_computed_counter> = None;
    match style.uncommon {
        None=>{
            (CSS_COUNTER_INCREMENT_NONE as u8,counter)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_COUNTER_INCREMENT_INDEX];
            bits = bits & (CSS_COUNTER_INCREMENT_MASK as u8);
            bits = bits >> CSS_COUNTER_INCREMENT_SHIFT;

            counter = Some(uncommon_struct.counter_increment);

            (bits,counter)
        }
    }  
}

pub fn css_computed_counter_reset(
                        style : @mut css_computed_style)
                        -> (u8,Option<@mut css_computed_counter>) {

    let mut counter :Option<@mut css_computed_counter> = None;
    match style.uncommon {
        None=>{
            (CSS_COUNTER_RESET_NONE as u8,counter)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_COUNTER_RESET_INDEX];
            bits = bits & (CSS_COUNTER_RESET_MASK as u8);
            bits = bits >> CSS_COUNTER_RESET_SHIFT;

            counter = Some(uncommon_struct.counter_reset);

            (bits,counter)
        }
    }  
}

pub fn css_computed_cursor(
                style : @mut css_computed_style)
                -> (u8,Option<~[~str]>) {

    let mut urls :Option<~[~str]> = None;
    match style.uncommon {
        None=>{
            (CSS_CURSOR_AUTO as u8,urls)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_CURSOR_INDEX];
            bits = bits & (CSS_CURSOR_MASK as u8);
            bits = bits >> CSS_CURSOR_SHIFT;

            urls = Some(copy uncommon_struct.cursor);

            (bits,urls)
        }
    }  
}

pub fn css_computed_clip(
            style : @mut css_computed_style) 
            -> (u8,Option<css_computed_clip_rect>) {

    let mut result : css_computed_clip_rect = 
        css_computed_clip_rect{
            top:0,
            right:0,
            bottom:0,
            left:0,
            tunit:CSS_UNIT_PX,
            runit:CSS_UNIT_PX,
            bunit:CSS_UNIT_PX,
            lunit:CSS_UNIT_PX,
            top_auto:false,
            right_auto:false,
            bottom_auto:false,
            left_auto:false
    } ;

    match style.uncommon {
        None=>{
            ((CSS_CLIP_AUTO as u8),None)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_CLIP_INDEX];
            bits = bits & (CSS_CLIP_MASK as u8);
            bits = bits >> CSS_CLIP_SHIFT;

            if (bits&0x3) == (CSS_CLIP_RECT as u8) {
                let mut bits1 : u8 ;

                result.left_auto = (bits & 0x4)!=0;
                result.bottom_auto = (bits & 0x8)!=0;
                result.right_auto = (bits & 0x10)!=0;
                result.top_auto = (bits & 0x20)!=0;

                if (result.top_auto == false ||
                        result.right_auto == false) {
                    /* 8bits: ttttrrrr : top | right */
                    bits1 = uncommon_struct.bits[CSS_CLIP_INDEX1];
                    bits1 &= (CSS_CLIP_MASK1 as u8);
                    bits1 >>= CSS_CLIP_SHIFT1;
                } 
                else {
                    bits1 = 0;
                }

                result.top = uncommon_struct.clip[0];
                result.tunit = unsafe { cast::transmute((bits1 >> 4)as int)};

                result.right = uncommon_struct.clip[1];
                result.runit = unsafe { cast::transmute((bits1 & 0xf)as int)};

                if (result.bottom_auto == false ||
                        result.left_auto == false) {
                    /* 8bits: bbbbllll : bottom | left */
                    bits1 = uncommon_struct.bits[CSS_CLIP_INDEX2];
                    bits1 &= (CSS_CLIP_MASK2 as u8);
                    bits1 >>= CSS_CLIP_SHIFT2;
                } 
                else {
                    bits1 = 0;
                }

                result.bottom = uncommon_struct.clip[2];
                result.bunit = unsafe { cast::transmute((bits1 >> 4)as int)};

                result.left = uncommon_struct.clip[3];
                result.lunit = unsafe { cast::transmute((bits1 & 0xf)as int)} ;
            }

            ((bits&0x3),Some(result))
        }
    }
}

pub fn css_computed_content(
                style : @mut css_computed_style)
                -> (u8,Option<@mut css_computed_content_item>) {

    match style.uncommon {
        None=>{
            (CSS_CONTENT_NORMAL as u8,None)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_CONTENT_INDEX];
            bits = bits & (CSS_CONTENT_MASK as u8);
            bits = bits >> CSS_CONTENT_SHIFT;

            (bits,Some(uncommon_struct.content))
        }
    }  
}

pub fn css_computed_vertical_align(
                    style : @mut css_computed_style) 
                        ->(u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_VERTICAL_ALIGN_INDEX];
    bits = bits & (CSS_VERTICAL_ALIGN_MASK as u8);
    bits = bits >> CSS_VERTICAL_ALIGN_SHIFT;

    if ((bits&0xf)==(CSS_VERTICAL_ALIGN_SET as u8)) {
        length = Some(style.vertical_align);
        unit = Some(unsafe { cast::transmute((bits >> 4) as int)});
        return ((bits&0xf),length,unit);
    }

    ((bits&0xf),length,unit)
}

pub fn css_computed_font_size(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_FONT_SIZE_INDEX];
    bits = bits & (CSS_FONT_SIZE_MASK as u8);
    bits = bits >> CSS_FONT_SIZE_SHIFT;

    if ((bits&0xf)==(CSS_FONT_SIZE_DIMENSION as u8)) {
        length = Some(style.font_size);
        unit = Some(unsafe { cast::transmute((bits >> 4) as int)});
        return ((bits&0xf),length,unit);
    }

    ((bits&0xf),length,unit)
}

pub fn css_computed_border_top_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_TOP_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_TOP_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[0]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_border_right_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_RIGHT_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[1]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_border_bottom_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_BOTTOM_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[2]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_border_left_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_LEFT_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_LEFT_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[3]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_background_image(
                                    style:@mut css_computed_style)
                                    -> (u8,~str) {

    let mut url : ~str ;
    let mut bits = style.bits[CSS_BACKGROUND_IMAGE_INDEX];
    bits = bits & (CSS_BACKGROUND_IMAGE_MASK as u8);
    bits = bits >> CSS_BACKGROUND_IMAGE_SHIFT;

    url = copy style.background_image;

    (bits,url)
}

pub fn css_computed_color(
                        style : @mut css_computed_style)
                        ->(u8,u32) {

    let mut bits = style.bits[CSS_COLOR_INDEX];
    bits = bits & (CSS_COLOR_MASK as u8);
    bits = bits >> CSS_COLOR_SHIFT ;

    (bits,style.color)
}

pub fn css_computed_list_style_image(
                                    style : @mut css_computed_style)
                                    ->(u8,~str) {

    let mut url : ~str ;
    let mut bits = style.bits[CSS_LIST_STYLE_IMAGE_INDEX];
    bits = bits & (CSS_LIST_STYLE_IMAGE_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_IMAGE_SHIFT;

    url = copy style.list_style_image;

    (bits,url)
}

pub fn css_computed_quotes(
                        style:@mut css_computed_style)
                        -> (u8,~[~str]) {

    let mut result : ~[~str] ;
    let mut bits = style.bits[CSS_QUOTES_INDEX];
    bits = bits & (CSS_QUOTES_MASK as u8);
    bits = bits >> CSS_QUOTES_SHIFT;

    result = copy style.quotes;
    (bits,result)
}

pub fn css_computed_top(
                    style : @mut css_computed_style)
                    -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_TOP_INDEX];
    bits = bits & (CSS_TOP_MASK as u8);
    bits = bits >> CSS_TOP_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_TOP_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut bottom : u8 = style.bits[CSS_BOTTOM_INDEX];
        bottom = bottom & (CSS_BOTTOM_MASK as u8);
        bottom = bottom >> CSS_BOTTOM_SHIFT;  

        if( (bits&0x3)==(CSS_TOP_AUTO as u8) &&
            (bottom&0x3)==(CSS_BOTTOM_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_TOP_AUTO as u8) ) {
            length = Some(-style.bottom);
            unit = Some(unsafe { cast::transmute((bottom >> 2) as int)});
        }
        else {
            length = Some(style.top);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_TOP_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_TOP_SET as u8) ) {
        length = Some(style.top);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_right(
                        style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_RIGHT_INDEX];
    bits = bits & (CSS_RIGHT_MASK as u8);
    bits = bits >> CSS_RIGHT_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_RIGHT_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut left : u8 = style.bits[CSS_LEFT_INDEX];
        left = left & (CSS_LEFT_MASK as u8);
        left = left >> CSS_LEFT_SHIFT;  

        if( (bits&0x3)==(CSS_RIGHT_AUTO as u8) &&
            (left&0x3)==(CSS_LEFT_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_RIGHT_AUTO as u8) ) {
            length = Some(-style.left);
            unit = Some(unsafe { cast::transmute((left >> 2) as int)});
        }
        else {
            length = Some(style.right);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_RIGHT_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_RIGHT_SET as u8) ) {
        length = Some(style.right);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_bottom(
                        style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_BOTTOM_INDEX];
    bits = bits & (CSS_BOTTOM_MASK as u8);
    bits = bits >> CSS_BOTTOM_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_BOTTOM_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut top : u8 = style.bits[CSS_TOP_INDEX];
        top = top & (CSS_TOP_MASK as u8);
        top = top >> CSS_TOP_SHIFT;  

        if( (bits&0x3)==(CSS_BOTTOM_AUTO as u8) &&
            (top&0x3)==(CSS_TOP_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_BOTTOM_AUTO as u8) || 
                   (top&0x3)==(CSS_TOP_AUTO as u8) ) {
            length = Some(-style.top);
            unit = Some(unsafe { cast::transmute((top >> 2) as int)});
        }
        else {
            length = Some(style.bottom);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_BOTTOM_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_BOTTOM_SET as u8) ) {
        length = Some(style.bottom);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_left(
                        style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_LEFT_INDEX];
    bits = bits & (CSS_LEFT_MASK as u8);
    bits = bits >> CSS_LEFT_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_LEFT_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut right : u8 = style.bits[CSS_RIGHT_INDEX];
        right = right & (CSS_RIGHT_MASK as u8);
        right = right >> CSS_RIGHT_SHIFT;  

        if( (bits&0x3)==(CSS_LEFT_AUTO as u8) &&
            (right&0x3)==(CSS_RIGHT_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_LEFT_AUTO as u8) ) {
            length = Some(-style.right);
            unit = Some(unsafe { cast::transmute((right >> 2) as int)});
        }
        else {
            length = Some(style.left);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_LEFT_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_LEFT_SET as u8) ) {
        length = Some(style.left);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_border_top_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_TOP_COLOR_INDEX];
    bits = bits & (CSS_BORDER_TOP_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_COLOR_SHIFT; 

    (bits,style.border_color[0]) 
}

pub fn css_computed_border_right_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_RIGHT_COLOR_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_COLOR_SHIFT; 

    (bits,style.border_color[1]) 
}

pub fn css_computed_border_bottom_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_BOTTOM_COLOR_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_COLOR_SHIFT; 

    (bits,style.border_color[2]) 
}

pub fn css_computed_border_left_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_LEFT_COLOR_INDEX];
    bits = bits & (CSS_BORDER_LEFT_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_COLOR_SHIFT; 

    (bits,style.border_color[3]) 
}

pub fn css_computed_height(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_HEIGHT_INDEX];
    bits = bits & (CSS_HEIGHT_MASK as u8);
    bits = bits >> CSS_HEIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_HEIGHT_SET as u8) ) {
        length = Some(style.height);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_line_height(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_LINE_HEIGHT_INDEX];
    bits = bits & (CSS_LINE_HEIGHT_MASK as u8);
    bits = bits >> CSS_LINE_HEIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_LINE_HEIGHT_NUMBER as u8) || 
         (bits&0x3) == (CSS_LINE_HEIGHT_DIMENSION as u8)) {
        length = Some(style.line_height);
    }

    if ( (bits&0x3) == (CSS_LINE_HEIGHT_DIMENSION as u8) ) {
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_background_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_COLOR_INDEX];
    bits = bits & (CSS_BACKGROUND_COLOR_MASK as u8);
    bits = bits >> CSS_BACKGROUND_COLOR_SHIFT; 

    (bits,style.background_color) 
}

pub fn css_computed_z_index(style: @mut css_computed_style)
                            -> (u8,i32) {

    let mut bits : u8 = style.bits[CSS_Z_INDEX_INDEX];
    bits = bits & (CSS_Z_INDEX_MASK as u8);
    bits = bits >> CSS_Z_INDEX_SHIFT; 

    (bits,style.z_index) 
}

pub fn css_computed_margin_top(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_TOP_INDEX];
    bits = bits & (CSS_MARGIN_TOP_MASK as u8);
    bits = bits >> CSS_MARGIN_TOP_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[0]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_margin_right(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_RIGHT_INDEX];
    bits = bits & (CSS_MARGIN_RIGHT_MASK as u8);
    bits = bits >> CSS_MARGIN_RIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[1]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_margin_bottom(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_BOTTOM_INDEX];
    bits = bits & (CSS_MARGIN_BOTTOM_MASK as u8);
    bits = bits >> CSS_MARGIN_BOTTOM_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[2]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_margin_left(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_LEFT_INDEX];
    bits = bits & (CSS_MARGIN_LEFT_MASK as u8);
    bits = bits >> CSS_MARGIN_LEFT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[3]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}











// required in function , written earlier in this file

pub fn css_computed_position(
                            style: @mut css_computed_style)
                            -> u8 {

    let mut bits : u8 = style.bits[CSS_POSITION_INDEX];
    bits = bits & (CSS_POSITION_MASK as u8);
    bits = bits >> CSS_POSITION_SHIFT;  

    bits
}


