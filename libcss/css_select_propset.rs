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
				content:None
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

pub fn set_background_color(style:@mut css_computed_style,
					ftype:u8, 
					color:u32) {

	let mut bits = style.bits[CSS_BACKGROUND_COLOR_INDEX];
	let mask_complement = (CSS_BACKGROUND_COLOR_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			((ftype & 0x3) << CSS_BACKGROUND_COLOR_SHIFT);

	style.background_color = color;
	style.bits[CSS_BACKGROUND_COLOR_INDEX] = bits ;
}

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

pub fn set_height( style: @mut css_computed_style,
				ftype:u8, 
				length:i32,
				unit:css_unit) {

	let mut bits = style.bits[CSS_HEIGHT_INDEX];
	let mask_complement = (CSS_HEIGHT_MASK as u8) ^ 0xff ;

	bits = (bits & mask_complement) |
			( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_HEIGHT_SHIFT);
	style.bits[CSS_HEIGHT_INDEX] = bits ;

	style.height = length;
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

//////////////////////////
