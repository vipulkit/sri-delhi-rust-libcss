// use wapcaplet::*;
// use std::arc;

use include::properties::*;

use include::types::*;
use bytecode::bytecode::*;
use bytecode::opcodes::*;
use stylesheet::*;
use utils::errors::*;
use select::select::*;
use select::common::*;

type border_color_fn = @extern fn (@mut css_computed_style, u8, css_color) -> css_result;
type uri_none_fn= @extern fn (@mut css_computed_style, u8, ~str) -> css_result;
type border_style_fn =  @extern fn (@mut css_computed_style, u8) -> css_result;
type helper_fn =  @extern fn (@mut css_computed_style, u8, css_fixed, css_unit) -> css_result;
type number_fn= @extern fn (@mut css_computed_style, u8, css_fixed) -> css_result;
type css_color = u32;

// Azimuth.c
///////////////////////////////////////////////////////////////////
pub fn css__cascade_azimuth(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_result
{
	let mut val : i32 = 0;
	let mut unit : u32 = UNIT_DEG ;
	let mut az : u16 ;

	if( isInherit(opv) == false  ) {
		let mut azimuth_mask = (AZIMUTH_BEHIND as u16) ^ 0xFFFF ; 
		az = (getValue(opv) & azimuth_mask) ;
			if ( az == AZIMUTH_ANGLE) {
				val = style.bytecode[style.used+1] as i32 ;
				style.used += 1;
				unit = style.bytecode[style.used+1] as u32;
				style.used += 1;
			}
		/* * \todo azimuth behind */
	}

	// todo : unit = css__to_css_unit(unit); 
	if ( css__outranks_existing( (getOpcode(opv) as u16),
								isImportant(opv),
								state,
								isInherit(opv) ) ) {
		return CSS_OK ;
	}
	
	CSS_OK
}

pub fn css__set_azimuth_from_hint(hint: @mut css_hint, 
		style:@mut css_computed_style) -> css_result {

	CSS_OK
}

pub fn css__initial_azimuth(state:@mut css_select_state) -> css_result {

	CSS_OK
}

pub fn css__compose_azimuth(parent:@mut css_computed_style,
							child:@mut css_computed_style,
							result:@mut css_computed_style) -> css_result {

	CSS_OK
}

///////////////////////////////////////////////////////////////////


/* HELPERS --- Useful helpers */
///////////////////////////////////////////////////////////////////

pub fn css__to_css_unit(u:u32) -> css_unit {
	match u {
		UNIT_PX => CSS_UNIT_PX,
		UNIT_EX => CSS_UNIT_EX,
		UNIT_EM => CSS_UNIT_EM,
		UNIT_IN => CSS_UNIT_IN,
		UNIT_CM => CSS_UNIT_CM,
		UNIT_MM => CSS_UNIT_MM,
		UNIT_PT => CSS_UNIT_PT,
		UNIT_PC => CSS_UNIT_PC,
		UNIT_PCT => CSS_UNIT_PCT,
		UNIT_DEG => CSS_UNIT_DEG,
		UNIT_GRAD => CSS_UNIT_GRAD,
		UNIT_RAD => CSS_UNIT_RAD,
		UNIT_MS => CSS_UNIT_MS,
		UNIT_S => CSS_UNIT_S,
		UNIT_HZ => CSS_UNIT_HZ,
		UNIT_KHZ => CSS_UNIT_KHZ,
		_ => CSS_UNIT_PX
	}
}


/******************************************************************************
 * Utilities below here							      *
 ******************************************************************************/
pub fn css__cascade_bg_border_color(opv:u32, style:@mut css_style, state:@mut css_select_state, 
		 fun:border_color_fn) -> css_result {
	
	let mut value = CSS_BACKGROUND_COLOR_INHERIT;
	let mut color:css_color= 0;

	// assert(CSS_BACKGROUND_COLOR_INHERIT == (enum css_background_color_e)CSS_BORDER_COLOR_INHERIT);
	// assert(CSS_BACKGROUND_COLOR_COLOR == (enum css_background_color_e)CSS_BORDER_COLOR_COLOR);
	// assert(CSS_BACKGROUND_COLOR_CURRENT_COLOR == (enum css_background_color_e)CSS_BORDER_COLOR_CURRENT_COLOR);

	if !isInherit(opv)  {
		match getValue(opv) {
			BACKGROUND_COLOR_TRANSPARENT => value = CSS_BACKGROUND_COLOR_COLOR,
			BACKGROUND_COLOR_CURRENT_COLOR => value = CSS_BACKGROUND_COLOR_CURRENT_COLOR,
			BACKGROUND_COLOR_SET => {
				value = CSS_BACKGROUND_COLOR_COLOR;
				color = copy style.bytecode[style.used];
				advance_bytecode(style)
			}
			_ => fail!(~"Invalid css__cascade_bg_border_color match code")
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		return (*fun)(state.computed, value as u8, color)
	}

	CSS_OK
}


pub fn css__cascade_uri_none(opv:u32, style:@mut css_style, state:@mut css_select_state, fun:Option<uri_none_fn>) -> css_result {
	
	let mut value = CSS_BACKGROUND_IMAGE_INHERIT;
	let mut uri: Option<~str> = None;
	//let mut error:css_result;

	if !isInherit(opv) {
		match getValue(opv) {
			BACKGROUND_IMAGE_NONE => value = CSS_BACKGROUND_IMAGE_NONE,
			BACKGROUND_IMAGE_URI => {
				value = CSS_BACKGROUND_IMAGE_IMAGE;
				let (error_ret, ret_uri) = style.sheet.unwrap().css__stylesheet_string_get(style.bytecode[style.used] as uint);
				uri = ret_uri;
				advance_bytecode(style)	
			},
			_ => fail!(~"Invalid css__cascade_uri_none match code")
		}
	}

	// \todo lose fun != NULL once all properties have set routines 
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			return (*fun_fn)(state.computed, value as u8, uri.unwrap())
		},
		None => {}
	}

	CSS_OK
}


pub fn css__cascade_border_style(opv:u32, _:@mut css_style,	state:@mut css_select_state, fun:border_style_fn) -> css_result {
	
	let mut value = CSS_BORDER_STYLE_INHERIT;

	if !isInherit(opv)  {
		match getValue(opv) {
			BORDER_STYLE_NONE => value = CSS_BORDER_STYLE_NONE,
			BORDER_STYLE_HIDDEN => value = CSS_BORDER_STYLE_HIDDEN,
			BORDER_STYLE_DOTTED => value = CSS_BORDER_STYLE_DOTTED,
			BORDER_STYLE_DASHED => value = CSS_BORDER_STYLE_DASHED,
			BORDER_STYLE_SOLID => value = CSS_BORDER_STYLE_SOLID,
			BORDER_STYLE_DOUBLE => value = CSS_BORDER_STYLE_DOUBLE,
			BORDER_STYLE_GROOVE => value = CSS_BORDER_STYLE_GROOVE,
			BORDER_STYLE_RIDGE => value = CSS_BORDER_STYLE_RIDGE,
			BORDER_STYLE_INSET => value = CSS_BORDER_STYLE_INSET,
			BORDER_STYLE_OUTSET => value = CSS_BORDER_STYLE_OUTSET,
			_ => fail!(~"Invalid css__cascade_border_style match code")
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		return (*fun)(state.computed, value as u8)
	}

	CSS_OK
}


pub fn css__cascade_border_width(opv:u32, style:@mut css_style, state:@mut css_select_state, fun:helper_fn) -> css_result {
	
	let mut value = CSS_BORDER_WIDTH_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;

	if !isInherit(opv) {
		match getValue(opv) {
			BORDER_WIDTH_SET => {
				value = CSS_BORDER_WIDTH_WIDTH;
				length = copy style.bytecode[style.used];
				advance_bytecode(style);
				unit =  style.bytecode[style.used];
				advance_bytecode(style)				
			},	
			BORDER_WIDTH_THIN => value = CSS_BORDER_WIDTH_THIN,				
			BORDER_WIDTH_MEDIUM => value = CSS_BORDER_WIDTH_MEDIUM,
			BORDER_WIDTH_THICK => value = CSS_BORDER_WIDTH_THICK,
			_ => fail!(~"Invalid css__cascade_border_width match code")
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		return (*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length_auto(opv:u32, style:@mut css_style, state:@mut css_select_state,fun:helper_fn) -> css_result {
	
	let mut value = CSS_BOTTOM_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		match getValue(opv) {
			BOTTOM_SET => {
				value = CSS_BOTTOM_SET;
				length = copy style.bytecode[style.used];
				advance_bytecode(style);
				unit =  style.bytecode[style.used];
				advance_bytecode(style)				
			},	
			BOTTOM_AUTO => value = CSS_BOTTOM_AUTO,				
			_ => fail!(~"Invalid css__cascade_length_auto match code")
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		return (*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK

}


pub fn css__cascade_length_normal(opv:u32, style:@mut css_style, state:@mut css_select_state,fun:helper_fn) -> css_result {
	
	let mut value = CSS_LETTER_SPACING_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		match getValue(opv) {
			LETTER_SPACING_SET => {
				value = CSS_LETTER_SPACING_SET;
				length = copy style.bytecode[style.used];
				advance_bytecode(style);
				unit =  style.bytecode[style.used];
				advance_bytecode(style)				
			},	
			LETTER_SPACING_NORMAL => value = CSS_LETTER_SPACING_NORMAL,				
			_ => fail!(~"Invalid css__cascade_length_normal match code")
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		return (*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length_none(opv:u32, style:@mut css_style, state:@mut css_select_state,fun:helper_fn) -> css_result {

	let mut value = CSS_MAX_HEIGHT_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		match getValue(opv) {
			MAX_HEIGHT_SET => {
				value = CSS_MAX_HEIGHT_SET;
				length = copy style.bytecode[style.used];
				advance_bytecode(style);
				unit =  style.bytecode[style.used];
				advance_bytecode(style)				
			},	
			MAX_HEIGHT_NONE => value = CSS_MAX_HEIGHT_NONE,				
			_ => fail!(~"Invalid css__cascade_length_normal match code")
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		return (*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length(opv:u32, style:@mut css_style, state:@mut css_select_state,fun:Option<helper_fn>) -> css_result {

	let mut value = CSS_MIN_HEIGHT_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		value = CSS_MIN_HEIGHT_SET;
		length = copy style.bytecode[style.used];
		advance_bytecode(style);
		unit =  style.bytecode[style.used];
		advance_bytecode(style)				
	}

	unit = css__to_css_unit(unit) as u32;

	// \todo lose fun != NULL once all properties have set routines */
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			return (*fun_fn)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
		},
		None => {}
	}

	CSS_OK
}


pub fn css__cascade_number(opv:u32, style:@mut css_style, state:@mut css_select_state,fun:Option<number_fn>) -> css_result {
	let mut value = 0;
	let mut length = 0;
	
	// \todo values */

	if !isInherit(opv) {
		value = 0;
		length = copy style.bytecode[style.used];
		advance_bytecode(style);
	}
	

	// \todo lose fun != NULL once all properties have set routines */
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			return (*fun_fn)(state.computed, value, length as i32)
		},
		None => {}
	}

	CSS_OK
}

///////////////////////////////////////////////////////////////////