#[link(name = "css_select_propset", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_select_const;
extern mod css_fpmath;
extern mod std ;

use css_enum::* ;
use css_select_const::*;
use css_fpmath::*;

pub fn ENSURE_UNCOMMON ( style:@mut css_computed_style){
	match style.uncommon {
		Some(_)=>{},
		None=>{
			let mut uncommon_struct = @mut css_computed_uncommon {
				bits:~[ 
						((CSS_LETTER_SPACING_INHERIT as u8) << 2) | (CSS_OUTLINE_COLOR_INVERT as u8),
	  					((CSS_OUTLINE_WIDTH_MEDIUM as u8) << 1) | (CSS_BORDER_SPACING_INHERIT as u8),
	  					0,
	  					((CSS_WORD_SPACING_INHERIT as u8) << 2) | 
						((CSS_COUNTER_INCREMENT_NONE as u8) << 1) | (CSS_COUNTER_RESET_NONE as u8),
	  					((CSS_CURSOR_INHERIT as u8) << 3) | 0,
	  					0,
	  					0,
	  					((CSS_CLIP_AUTO as u8) << 2) | (CSS_CONTENT_NORMAL as u8)
				],
				border_spacing:~[0,0],
				clip:~[0,0,0,0],
				letter_spacing:0,
				outline_color:0,
				outline_width:0,
				word_spacing:0,
				counter_increment:~[],
				counter_reset:~[],
				cursor:~[],
				content:~[]
			};
			style.uncommon = Some(uncommon_struct);
		}
	}
}

pub fn ENSURE_PAGE ( style:@mut css_computed_style){
	match style.uncommon {
		Some(_)=>{},
		None=>{
			let mut page_struct = @mut css_computed_page {
				bits:~[ 
						( ((CSS_PAGE_BREAK_INSIDE_AUTO as u8) <<  6) | 
        				  ((CSS_PAGE_BREAK_BEFORE_AUTO as u8) << 3) |
        				   (CSS_PAGE_BREAK_AFTER_AUTO as u8) ) ]
			};
			style.page = Some(page_struct);
		}
	}
}

pub fn set_letter_spacing(style:@mut css_computed_style,
					ftype:u8, 
					length:i32, 
					unit:css_unit)  {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_LETTER_SPACING_INDEX];
	let mask_complement = (CSS_LETTER_SPACING_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3) | ( (unit as u8) << 2) ) << CSS_LETTER_SPACING_SHIFT);
	style.uncommon.get().bits[CSS_LETTER_SPACING_INDEX] = bits ;

	style.uncommon.get().letter_spacing = length;
}

pub fn set_outline_color(style:@mut css_computed_style,
					ftype:u8, 
					color:u32)  {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_OUTLINE_COLOR_INDEX];
	let mask_complement = (CSS_OUTLINE_COLOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_OUTLINE_COLOR_SHIFT);
	style.uncommon.get().bits[CSS_OUTLINE_COLOR_INDEX] = bits ;

	style.uncommon.get().outline_color = color;
}

pub fn set_outline_width(style:@mut css_computed_style,
					ftype:u8, 
					length:i32, 
					unit:css_unit)  {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_OUTLINE_WIDTH_INDEX];
	let mask_complement = (CSS_OUTLINE_WIDTH_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ((ftype & 0x7)|((unit as u8)<<3))  << CSS_OUTLINE_WIDTH_SHIFT);
	style.uncommon.get().bits[CSS_OUTLINE_WIDTH_INDEX] = bits ;

	style.uncommon.get().outline_width = length;
}

pub fn set_border_spacing(style:@mut css_computed_style,
					ftype:u8, 
					hlength:i32, 
					hunit:css_unit,
					vlength:i32,
					vunit:css_unit)  {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_BORDER_SPACING_INDEX];
	let mask_complement = (CSS_BORDER_SPACING_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_BORDER_SPACING_SHIFT);
	style.uncommon.get().bits[CSS_BORDER_SPACING_INDEX] = bits ;

	bits = style.uncommon.get().bits[CSS_BORDER_SPACING_INDEX1];
	let mask_complement = (CSS_BORDER_SPACING_MASK1 as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( ( (hunit as u8) << 4)|(vunit as u8) )  << CSS_BORDER_SPACING_SHIFT1);
	style.uncommon.get().bits[CSS_BORDER_SPACING_INDEX1] = bits ;

	style.uncommon.get().border_spacing[0] = hlength;
	style.uncommon.get().border_spacing[1] = vlength;
}

pub fn set_word_spacing(style:@mut css_computed_style,
					ftype:u8, 
					length:i32, 
					unit:css_unit)  {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_WORD_SPACING_INDEX];
	let mask_complement = (CSS_WORD_SPACING_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ((ftype & 0x3)|((unit as u8)<<2))  << CSS_WORD_SPACING_SHIFT);
	style.uncommon.get().bits[CSS_WORD_SPACING_INDEX] = bits ;

	style.uncommon.get().word_spacing = length;
}

pub fn set_counter_increment(style: @mut css_computed_style,
							ftype:u8,
							counters:~[@mut css_computed_counter]) {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_COUNTER_INCREMENT_INDEX];
	let mask_complement = (CSS_COUNTER_INCREMENT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_COUNTER_INCREMENT_SHIFT);
	style.uncommon.get().bits[CSS_COUNTER_INCREMENT_INDEX] = bits ;

	style.uncommon.get().counter_increment = counters ;
}

pub fn set_counter_reset(style: @mut css_computed_style,
						ftype:u8,
						counters:~[@mut css_computed_counter]) {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_COUNTER_RESET_INDEX];
	let mask_complement = (CSS_COUNTER_RESET_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_COUNTER_RESET_SHIFT);
	style.uncommon.get().bits[CSS_COUNTER_RESET_INDEX] = bits ;

	style.uncommon.get().counter_reset = counters ;
}

pub fn set_cursor(style:@mut css_computed_style,
					ftype:u8, 
					urls:~[~str])  {

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_CURSOR_INDEX];
	let mask_complement = (CSS_CURSOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1f)  << CSS_CURSOR_SHIFT);
	style.uncommon.get().bits[CSS_CURSOR_INDEX] = bits ;

	style.uncommon.get().cursor = urls;
}

pub fn set_clip(style: @mut css_computed_style,
				ftype:u8, 
				rect:css_computed_clip_rect){

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_CLIP_INDEX];
	let mut mask_complement = (CSS_CLIP_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_CLIP_SHIFT);
	style.uncommon.get().bits[CSS_CLIP_INDEX] = bits ;

	if ( ftype == (CSS_CLIP_RECT as u8) ){
		bits = bits | 
				((
				( if (rect.top_auto	  ) { 0x20 } else { 0 } ) |
				( if (rect.right_auto ) { 0x10 } else { 0 } ) |
				( if (rect.bottom_auto) { 0x8  } else { 0 } ) |
				( if (rect.left_auto  ) { 0x4  } else { 0 } ) ) 
				<< CSS_CLIP_SHIFT );
		// syncing bits 
		style.uncommon.get().bits[CSS_CLIP_INDEX] = bits ;

		bits = style.uncommon.get().bits[CSS_CLIP_INDEX1];
		mask_complement = (CSS_CLIP_MASK1 as u8) ^ 0xff ;
		bits = ( bits & mask_complement ) |
			( (((rect.tunit as u8) << 4) | (rect.runit as u8) )  
			<< CSS_CLIP_SHIFT1);
		style.uncommon.get().bits[CSS_CLIP_INDEX1] = bits ;

		bits = style.uncommon.get().bits[CSS_CLIP_INDEX2];
		mask_complement = (CSS_CLIP_MASK2 as u8) ^ 0xff ;
		bits = ( bits & mask_complement ) |
			( (((rect.bunit as u8) << 4) | (rect.lunit as u8) )  
			<< CSS_CLIP_SHIFT2);
		style.uncommon.get().bits[CSS_CLIP_INDEX2] = bits ;

		style.uncommon.get().clip[0] = rect.top;
		style.uncommon.get().clip[1] = rect.right;
		style.uncommon.get().clip[2] = rect.bottom;
		style.uncommon.get().clip[3] = rect.left;
	}
}

pub fn set_content(style: @mut css_computed_style,
				ftype:u8, 
				content:~[@mut css_computed_content_item]){

	ENSURE_UNCOMMON(style);

	let mut bits = style.uncommon.get().bits[CSS_CONTENT_INDEX];
	let mut mask_complement = (CSS_CONTENT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_CONTENT_SHIFT);
	style.uncommon.get().bits[CSS_CONTENT_INDEX] = bits ;

	style.uncommon.get().content = content ;
}

pub fn set_vertical_align(style:@mut css_computed_style,
					ftype:u8, 
					length:i32, 
					unit:css_unit)  {

	let mut bits = style.bits[CSS_VERTICAL_ALIGN_INDEX];
	let mask_complement = (CSS_VERTICAL_ALIGN_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0xf) | ( (unit as u8) << 4) ) << CSS_VERTICAL_ALIGN_SHIFT);
	style.bits[CSS_VERTICAL_ALIGN_INDEX] = bits ;

	style.vertical_align = length;

}

pub fn set_font_size(style:@mut css_computed_style,
					ftype:u8, 
					length:i32, 
					unit:css_unit)  {

	let mut bits = style.bits[CSS_FONT_SIZE_INDEX];
	let mask_complement = (CSS_FONT_SIZE_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0xf) | ( (unit as u8) << 4) ) << CSS_FONT_SIZE_SHIFT);
	style.bits[CSS_FONT_SIZE_INDEX] = bits ;

	style.font_size = length;
}

pub fn set_border_top_width(style:@mut css_computed_style,
						ftype:u8, 
						length:i32, 
						unit:css_unit)  {

	let mut bits = style.bits[CSS_BORDER_TOP_WIDTH_INDEX];
	let mask_complement = (CSS_BORDER_TOP_WIDTH_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_TOP_WIDTH_SHIFT);
	style.bits[CSS_BORDER_TOP_WIDTH_INDEX] = bits ;

	style.border_width[0] = length;
}

pub fn set_border_right_width(style:@mut css_computed_style,
						ftype:u8, 
						length:i32, 
						unit:css_unit)  {

	let mut bits = style.bits[CSS_BORDER_RIGHT_WIDTH_INDEX];
	let mask_complement = (CSS_BORDER_RIGHT_WIDTH_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_RIGHT_WIDTH_SHIFT);
	style.bits[CSS_BORDER_RIGHT_WIDTH_INDEX] = bits ;

	style.border_width[1] = length;
}

pub fn set_border_bottom_width(style:@mut css_computed_style,
						ftype:u8, 
						length:i32, 
						unit:css_unit)  {

	let mut bits = style.bits[CSS_BORDER_BOTTOM_WIDTH_INDEX];
	let mask_complement = (CSS_BORDER_BOTTOM_WIDTH_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_BOTTOM_WIDTH_SHIFT);
	style.bits[CSS_BORDER_BOTTOM_WIDTH_INDEX] = bits ;

	style.border_width[2] = length;
}

pub fn set_border_left_width(style:@mut css_computed_style,
						ftype:u8, 
						length:i32, 
						unit:css_unit)  {

	let mut bits = style.bits[CSS_BORDER_LEFT_WIDTH_INDEX];
	let mask_complement = (CSS_BORDER_LEFT_WIDTH_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_LEFT_WIDTH_SHIFT);
	style.bits[CSS_BORDER_LEFT_WIDTH_INDEX] = bits ;

	style.border_width[3] = length;
}

pub fn set_background_image(style:@mut css_computed_style,
							ftype:u8,
							url:~str) {

	let mut bits = style.bits[CSS_BACKGROUND_IMAGE_INDEX];
	let mask_complement = (CSS_BACKGROUND_IMAGE_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_BACKGROUND_IMAGE_SHIFT);
	style.bits[CSS_BACKGROUND_IMAGE_INDEX] = bits ;

	style.background_image = url;
}

pub fn set_color(style:@mut css_computed_style,
							ftype:u8,
							color:u32) {

	let mut bits = style.bits[CSS_COLOR_INDEX];
	let mask_complement = (CSS_COLOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_COLOR_SHIFT);
	style.bits[CSS_COLOR_INDEX] = bits ;

	style.color = color;
}

pub fn set_list_style_image(style:@mut css_computed_style,
							ftype:u8,
							url:~str) {

	let mut bits = style.bits[CSS_LIST_STYLE_IMAGE_INDEX];
	let mask_complement = (CSS_LIST_STYLE_IMAGE_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_LIST_STYLE_IMAGE_SHIFT);
	style.bits[CSS_LIST_STYLE_IMAGE_INDEX] = bits ;

	style.list_style_image = url;
}

pub fn set_quotes(style:@mut css_computed_style,
				ftype:u8,
				quotes:~[~str]) {

	let mut bits = style.bits[CSS_QUOTES_INDEX];
	let mask_complement = (CSS_QUOTES_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x1)  << CSS_QUOTES_SHIFT);
	style.bits[CSS_QUOTES_INDEX] = bits ;

	style.quotes = quotes ;
}

pub fn set_top(style:@mut css_computed_style,
				ftype:u8,
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_TOP_INDEX];
	let mask_complement = (CSS_TOP_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_TOP_SHIFT);
	style.bits[CSS_TOP_INDEX] = bits ;

	style.top = length ;
}

pub fn set_right(style:@mut css_computed_style,
				ftype:u8,
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_RIGHT_INDEX];
	let mask_complement = (CSS_RIGHT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_RIGHT_SHIFT);
	style.bits[CSS_RIGHT_INDEX] = bits ;

	style.right = length ;
}

pub fn set_bottom(style:@mut css_computed_style,
				ftype:u8,
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_BOTTOM_INDEX];
	let mask_complement = (CSS_BOTTOM_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_BOTTOM_SHIFT);
	style.bits[CSS_BOTTOM_INDEX] = bits ;

	style.bottom = length ;
}

pub fn set_left(style:@mut css_computed_style,
				ftype:u8,
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_LEFT_INDEX];
	let mask_complement = (CSS_LEFT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_LEFT_SHIFT);
	style.bits[CSS_LEFT_INDEX] = bits ;

	style.left = length ;
}

pub fn set_border_top_color(style:@mut css_computed_style,
						ftype:u8,
						color:u32) {

	let mut bits = style.bits[CSS_BORDER_TOP_COLOR_INDEX];
	let mask_complement = (CSS_BORDER_TOP_COLOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_BORDER_TOP_COLOR_SHIFT);
	style.bits[CSS_BORDER_TOP_COLOR_INDEX] = bits ;

	style.border_color[0] = color ;
}

pub fn set_border_right_color(style:@mut css_computed_style,
						ftype:u8,
						color:u32) {

	let mut bits = style.bits[CSS_BORDER_RIGHT_COLOR_INDEX];
	let mask_complement = (CSS_BORDER_RIGHT_COLOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_BORDER_RIGHT_COLOR_SHIFT);
	style.bits[CSS_BORDER_RIGHT_COLOR_INDEX] = bits ;

	style.border_color[1] = color ;
}

pub fn set_border_bottom_color(style:@mut css_computed_style,
						ftype:u8,
						color:u32) {

	let mut bits = style.bits[CSS_BORDER_BOTTOM_COLOR_INDEX];
	let mask_complement = (CSS_BORDER_BOTTOM_COLOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_BORDER_BOTTOM_COLOR_SHIFT);
	style.bits[CSS_BORDER_BOTTOM_COLOR_INDEX] = bits ;

	style.border_color[2] = color ;
}

pub fn set_border_left_color(style:@mut css_computed_style,
						ftype:u8,
						color:u32) {

	let mut bits = style.bits[CSS_BORDER_LEFT_COLOR_INDEX];
	let mask_complement = (CSS_BORDER_LEFT_COLOR_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_BORDER_LEFT_COLOR_SHIFT);
	style.bits[CSS_BORDER_LEFT_COLOR_INDEX] = bits ;

	style.border_color[3] = color ;
}

pub fn set_height(style:@mut css_computed_style,
				ftype:u8,
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_HEIGHT_INDEX];
	let mask_complement = (CSS_HEIGHT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_HEIGHT_SHIFT);
	style.bits[CSS_HEIGHT_INDEX] = bits ;

	style.height = length ;
}

pub fn set_line_height(style:@mut css_computed_style,
					ftype:u8,
					length:i32,
					unit:css_unit) {

	let mut bits = style.bits[CSS_LINE_HEIGHT_INDEX];
	let mask_complement = (CSS_LINE_HEIGHT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_LINE_HEIGHT_SHIFT);
	style.bits[CSS_LINE_HEIGHT_INDEX] = bits ;

	style.line_height = length ;
}

pub fn set_background_color(style:@mut css_computed_style,
					ftype:u8, 
					color:u32) {

	let mut bits = style.bits[CSS_BACKGROUND_COLOR_INDEX];
	let mask_complement = (CSS_BACKGROUND_COLOR_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			((ftype & 0x3) << CSS_BACKGROUND_COLOR_SHIFT);

	style.bits[CSS_BACKGROUND_COLOR_INDEX] = bits ;
	style.background_color = color;
}

pub fn set_z_index(style:@mut css_computed_style,
				ftype:u8,
				z_index:i32) {

	let mut bits = style.bits[CSS_Z_INDEX_INDEX];
	let mask_complement = (CSS_Z_INDEX_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			((ftype & 0x3) << CSS_Z_INDEX_SHIFT);

	style.bits[CSS_Z_INDEX_INDEX] = bits ;
	style.z_index = z_index;
}

pub fn set_margin_top(style:@mut css_computed_style,
					ftype:u8,
					length:i32,
					unit:css_unit) {

	let mut bits = style.bits[CSS_MARGIN_TOP_INDEX];
	let mask_complement = (CSS_MARGIN_TOP_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_TOP_SHIFT);
	style.bits[CSS_MARGIN_TOP_INDEX] = bits ;

	style.margin[0] = length ;
}

pub fn set_margin_right(style:@mut css_computed_style,
					ftype:u8,
					length:i32,
					unit:css_unit) {

	let mut bits = style.bits[CSS_MARGIN_RIGHT_INDEX];
	let mask_complement = (CSS_MARGIN_RIGHT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_RIGHT_SHIFT);
	style.bits[CSS_MARGIN_RIGHT_INDEX] = bits ;

	style.margin[1] = length ;
}

pub fn set_margin_bottom(style:@mut css_computed_style,
					ftype:u8,
					length:i32,
					unit:css_unit) {

	let mut bits = style.bits[CSS_MARGIN_BOTTOM_INDEX];
	let mask_complement = (CSS_MARGIN_BOTTOM_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_BOTTOM_SHIFT);
	style.bits[CSS_MARGIN_BOTTOM_INDEX] = bits ;

	style.margin[2] = length ;
}

pub fn set_margin_left(style:@mut css_computed_style,
					ftype:u8,
					length:i32,
					unit:css_unit) {

	let mut bits = style.bits[CSS_MARGIN_LEFT_INDEX];
	let mask_complement = (CSS_MARGIN_LEFT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_LEFT_SHIFT);
	style.bits[CSS_MARGIN_LEFT_INDEX] = bits ;

	style.margin[3] = length ;
}

pub fn set_background_attachment(style:@mut css_computed_style,
					ftype:u8) {

	let mut bits = style.bits[CSS_BACKGROUND_ATTACHMENT_INDEX];
	let mask_complement = (CSS_BACKGROUND_ATTACHMENT_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_BACKGROUND_ATTACHMENT_SHIFT);
	style.bits[CSS_BACKGROUND_ATTACHMENT_INDEX] = bits ;
}

pub fn set_border_collapse(style:@mut css_computed_style,
					ftype:u8) {

	let mut bits = style.bits[CSS_BORDER_COLLAPSE_INDEX];
	let mask_complement = (CSS_BORDER_COLLAPSE_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_BORDER_COLLAPSE_SHIFT);
	style.bits[CSS_BORDER_COLLAPSE_INDEX] = bits ;
}

pub fn set_caption_side(style:@mut css_computed_style,
					ftype:u8) {

	let mut bits = style.bits[CSS_CAPTION_SIDE_INDEX];
	let mask_complement = (CSS_CAPTION_SIDE_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_CAPTION_SIDE_SHIFT);
	style.bits[CSS_CAPTION_SIDE_INDEX] = bits ;
}

pub fn set_direction(style:@mut css_computed_style,
					ftype:u8) {

	let mut bits = style.bits[CSS_DIRECTION_INDEX];
	let mask_complement = (CSS_DIRECTION_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_DIRECTION_SHIFT);
	style.bits[CSS_DIRECTION_INDEX] = bits ;
}

pub fn set_max_height( style: @mut css_computed_style,
				ftype:u8, 
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_MAX_HEIGHT_INDEX];
	let mask_complement = (CSS_MAX_HEIGHT_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_MAX_HEIGHT_SHIFT);
	style.bits[CSS_MAX_HEIGHT_INDEX] = bits ;

	style.max_height = length;
}

pub fn set_max_width( style: @mut css_computed_style,
				ftype:u8, 
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_MAX_WIDTH_INDEX];
	let mask_complement = (CSS_MAX_WIDTH_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_MAX_WIDTH_SHIFT);
	style.bits[CSS_MAX_WIDTH_INDEX] = bits ;

	style.max_width = length;
}

pub fn set_width( style: @mut css_computed_style,
				ftype:u8, 
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_WIDTH_INDEX];
	let mask_complement = (CSS_WIDTH_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_WIDTH_SHIFT);
	style.bits[CSS_WIDTH_INDEX] = bits ;

	style.width = length;
}

pub fn set_empty_cells(style:@mut css_computed_style,
					ftype:u8) {

	let mut bits = style.bits[CSS_EMPTY_CELLS_INDEX];
	let mask_complement = (CSS_EMPTY_CELLS_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( (ftype & 0x3)  << CSS_EMPTY_CELLS_SHIFT);
	style.bits[CSS_EMPTY_CELLS_INDEX] = bits ;
}














/////////////
pub fn set_background_position(style : @mut css_computed_style,
							ftype:u8,
							hlength:i32,
							hunit:css_unit,
							vlength:i32,
							vunit:css_unit) {

	let mut bits = style.bits[CSS_BACKGROUND_POSITION_INDEX];
	let mask_complement = (CSS_BACKGROUND_POSITION_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			((ftype & 0x1) << CSS_BACKGROUND_POSITION_SHIFT);
	style.bits[CSS_BACKGROUND_POSITION_INDEX] = bits ;

	bits = style.bits[CSS_BACKGROUND_POSITION_INDEX1];
	let mask_complement = (CSS_BACKGROUND_POSITION_MASK1 as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			( (( (hunit as u8) << 4) | (vunit as u8) ) << CSS_BACKGROUND_POSITION_SHIFT1);
	style.bits[CSS_BACKGROUND_POSITION_INDEX1] = bits ;

	style.background_position[0] = hlength;
	style.background_position[1] = vlength;
}


//////////////////////////
