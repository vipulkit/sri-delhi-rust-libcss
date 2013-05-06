#[link(name = "css_select_propset", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_select_const;
extern mod css_fpmath;
extern mod std ;

use css_enum::* ;
use css_select_const::*;
use css_fpmath::*;

pub fn set_font_size(style:@mut css_computed_style,
					ftype:u8, 
					length:i32, 
					unit:css_unit) 
					-> css_result {

	let mut bits = style.bits[CSS_FONT_SIZE_INDEX];
	let mask_complement = (CSS_FONT_SIZE_MASK as u8) ^ 0xff ;
	bits = ( bits & mask_complement ) |
			( ( (ftype & 0xf) | ( (unit as u8) << 4) ) << CSS_FONT_SIZE_SHIFT);
	style.bits[CSS_FONT_SIZE_INDEX] = bits ;

	style.font_size = length;

	CSS_OK
}


//////////////////////////
