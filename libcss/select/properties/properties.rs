use include::properties::*;

use stylesheet::*;

use include::types::*;
use include::fpmath::*;
use bytecode::bytecode::*;
use bytecode::opcodes::*;
use stylesheet::*;
use utils::errors::*;
use select::select::*;
use select::common::*;
use select::propset::*;
use select::computed::*;

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
		 fun:@extern fn (@mut css_computed_style, u8, css_color)) -> css_result {
	
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
		(*fun)(state.computed, value as u8, color)
	}

	CSS_OK
}


pub fn css__cascade_uri_none(opv:u32, style:@mut css_style, state:@mut css_select_state, 
	fun:Option<@extern fn (@mut css_computed_style, u8, ~str)>) -> css_result {
	
	let mut value = CSS_BACKGROUND_IMAGE_INHERIT;
	let mut uri: Option<~str> = None;
	//let mut error:css_result;

	if !isInherit(opv) {
		match getValue(opv) {
			BACKGROUND_IMAGE_NONE => value = CSS_BACKGROUND_IMAGE_NONE,
			BACKGROUND_IMAGE_URI => {
				value = CSS_BACKGROUND_IMAGE_IMAGE;
				let (_, ret_uri) = style.sheet.unwrap().css__stylesheet_string_get(style.bytecode[style.used] as uint);
				uri = ret_uri;
				advance_bytecode(style)	
			},
			_ => fail!(~"Invalid css__cascade_uri_none match code")
		}
	}

	// \todo lose fun != NULL once all properties have set routines 
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			(*fun_fn)(state.computed, value as u8, uri.unwrap())
		},
		None => {}
	}

	CSS_OK
}


pub fn css__cascade_border_style(opv:u32, _:@mut css_style,	state:@mut css_select_state, 
	fun:@extern fn (@mut css_computed_style, u8) -> css_result) -> css_result {
	
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


pub fn css__cascade_border_width(opv:u32, style:@mut css_style, state:@mut css_select_state, 
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) -> css_result) -> css_result {
	
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


pub fn css__cascade_length_auto(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) -> css_result) -> css_result {
	
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


pub fn css__cascade_length_normal(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) -> css_result) -> css_result {
	
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


pub fn css__cascade_length_none(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) -> css_result) -> css_result {

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
			_ => fail!(~"Invalid css__cascade_length_none match code")
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		return (*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:Option<@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) -> css_result>) -> css_result {

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


pub fn css__cascade_number(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:Option<@extern fn (@mut css_computed_style, u8, css_fixed) -> css_result>) -> css_result {

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

pub fn css__cascade_page_break_after_before_inside(opv:u32, _:@mut css_style, state:@mut css_select_state,
		fun:Option<@extern fn (@mut css_computed_style, u8)-> css_result>) -> css_result {
	
	let mut value = CSS_PAGE_BREAK_AFTER_INHERIT;

	if !isInherit(opv) {
		match getValue(opv) {
			PAGE_BREAK_AFTER_AUTO => value = CSS_PAGE_BREAK_AFTER_AUTO,	
			PAGE_BREAK_AFTER_ALWAYS => value = CSS_PAGE_BREAK_AFTER_ALWAYS,
			PAGE_BREAK_AFTER_AVOID => value = CSS_PAGE_BREAK_AFTER_AVOID,
			PAGE_BREAK_AFTER_LEFT => value = CSS_PAGE_BREAK_AFTER_LEFT,
			PAGE_BREAK_AFTER_RIGHT => value = CSS_PAGE_BREAK_AFTER_RIGHT,				
			_ => fail!(~"Invalid css__cascade_length_none match code")
		}
	}

	// \todo lose fun != None */
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			return (*fun_fn)(state.computed, value as u8)
		},
		None => {}
	}

	CSS_OK
	
}

pub fn css__cascade_counter_increment_reset(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, ~[~css_computed_counter]) -> css_result) -> css_result {

	let mut value = CSS_COUNTER_INCREMENT_INHERIT;
	let mut counters:~[~css_computed_counter] = ~[];
	
	if !isInherit(opv) {
		match getValue(opv) {
			COUNTER_INCREMENT_NAMED => {
				let mut v = getValue(opv) as u32;

				while v != COUNTER_INCREMENT_NONE as u32{
					
					let (result, name_option) = style.sheet.unwrap().css__stylesheet_string_get((copy style.bytecode[style.used]) as uint);
					advance_bytecode(style);
					match result {
						CSS_OK => {
							let val = copy style.bytecode[style.used];
							advance_bytecode(style);

							let temp = ~css_computed_counter{name:name_option.unwrap(),value:val as i32};
							counters.push(temp);

							v = copy style.bytecode[style.used];
							advance_bytecode(style);
						}
						_ => return result
					}
						
				}
			},	
			COUNTER_INCREMENT_NONE => value = CSS_COUNTER_INCREMENT_NONE,
			_ => fail!(~"Invalid css__cascade_counter_increment_reset match code")
		}
	}

	
	/* If we have some counters, terminate the array with a blank entry */
	if !counters.is_empty() {
		let temp = ~css_computed_counter{name:~"",value:0};
		counters.push(temp);
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		return (*fun)(state.computed, value as u8, counters)
	}
	
	CSS_OK
}


///////////////////////////////////////////////////////////////////


// Azimuth.c
///////////////////////////////////////////////////////////////////
pub fn css__cascade_azimuth(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_result
{
	//not used let mut val : i32 ;
	//let mut unit : u32 ;
	let mut az : u16 ;

	if( isInherit(opv) == false  ) {
		let mut azimuth_mask = (AZIMUTH_BEHIND as u16) ^ 0xFFFF ; 
		az = (getValue(opv) & azimuth_mask) ;
			if ( az == AZIMUTH_ANGLE) {
				//not used val = peek_bytecode(style) as i32 ;
				advance_bytecode(style);
				//unit = peek_bytecode(style) as u32;
				advance_bytecode(style);
			}
		/* * \todo azimuth behind */
	}

	// not used unit = css__to_css_unit(unit); 
	if ( css__outranks_existing( (getOpcode(opv) as u16),
								isImportant(opv),
								state,
								isInherit(opv) ) ) {
		return CSS_OK ;
	}
	
	CSS_OK
}

pub fn css__set_azimuth_from_hint(_: @mut css_hint, 
		_:@mut css_computed_style) -> css_result {

	CSS_OK
}

pub fn css__initial_azimuth(_:@mut css_select_state) -> css_result {

	CSS_OK
}

pub fn css__compose_azimuth(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_result {

	CSS_OK
}

///////////////////////////////////////////////////////////////////


// background_attachment.c
///////////////////////////////////////////////////////////////////

pub fn css__cascade_background_attachment(opv:u32, style:@mut css_style, 
										state:@mut css_select_state
										) -> css_result {

	let mut value : u16 = (CSS_BACKGROUND_ATTACHMENT_INHERIT as u16);

	if (isInherit(opv) == false) {
		let mut attachment = getValue(opv) ;
		if ( attachment == BACKGROUND_ATTACHMENT_FIXED ) {
			value = (CSS_BACKGROUND_ATTACHMENT_FIXED as u16);
		}
		else if ( attachment == BACKGROUND_ATTACHMENT_SCROLL ) {
			value = (CSS_BACKGROUND_ATTACHMENT_SCROLL as u16);
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16),
							isImportant(opv), 
							state,
							isInherit(opv) ) ) {
		set_background_attachment(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_background_attachment_from_hint(hint:@mut css_hint, 
												style:@mut css_computed_style
												) -> css_result {

	set_background_attachment(style, hint.status);
	CSS_OK
}

pub fn css__initial_background_attachment(state:@mut css_select_state) -> css_result {

	set_background_attachment(state.computed, 
		(CSS_BACKGROUND_ATTACHMENT_SCROLL as u8) );
	CSS_OK
}

pub fn css__compose_background_attachment(parent:@mut css_computed_style,
										child:@mut css_computed_style,
										result:@mut css_computed_style
										) -> css_result {

	let mut ftype : u8 = css_computed_background_attachment(child);

	if (ftype == (CSS_BACKGROUND_ATTACHMENT_INHERIT as u8) ) {
		ftype = css_computed_background_attachment(parent);
	}

	set_background_attachment(result, (ftype as u8) );
	CSS_OK
}


///////////////////////////////////////////////////////////////////

// background_color.c
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_result {

	return css__cascade_bg_border_color(opv, style, state, @set_background_color);
}

pub fn css__set_background_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_result {
	match hint.hint_type {
		COLOR=>{
			set_background_color(style, hint.status, hint.color.get_or_default(0));
			CSS_OK
		}
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_background_color(state:@mut css_select_state) -> css_result {

	set_background_color(state.computed, 
		(CSS_BACKGROUND_COLOR_COLOR as u8), 0);
	CSS_OK
}

pub fn css__compose_background_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_result {

	let mut (ftype,ocolor) = css_computed_background_color(child);

	if (ftype == (CSS_BACKGROUND_COLOR_INHERIT as u8) ) {
		let mut (ftype2,ocolor2) = css_computed_background_color(parent);
		let mut color = ocolor2.get_or_default( ocolor.get_or_default(0) );
		set_background_color(result, ftype2, color);
		CSS_OK
	}
	else {
		let mut color = ocolor.get_or_default(0);
		set_background_color(result, ftype, color);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// background_image.c
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_image(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_result {

	return css__cascade_uri_none(opv, style, state, Some(@set_background_image) );
}

pub fn css__set_background_image_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_result {
	match hint.hint_type {
		STRING=>{
			match hint.string {
				Some(copy x)=>{
					set_background_image(style, hint.status, x);
				}
				None=>{
					set_background_image(style, hint.status, ~"");
				}
			}
			CSS_OK
		}
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_background_image(state:@mut css_select_state) -> css_result {

	set_background_image(state.computed, 
		(CSS_BACKGROUND_IMAGE_NONE as u8), ~"");
	CSS_OK
}

pub fn css__compose_background_image(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_result {

	let mut (ftype,url) = css_computed_background_image(child);

	if (ftype == (CSS_BACKGROUND_IMAGE_INHERIT as u8) ) {
		let mut (ftype2,url2) = css_computed_background_image(parent);
		set_background_image(result, ftype, url2);
		CSS_OK
	}
	else {
		set_background_image(result, ftype, url);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// background_position.c
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_position(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_result {

	let mut  value : u16 = CSS_BACKGROUND_POSITION_INHERIT as u16;
	let mut hlength : i32 = 0;
	let mut vlength : i32  = 0;
	let mut hunit : u32 = UNIT_PX as u32;
	let mut vunit : u32 = UNIT_PX as u32;

	if ( isInherit(opv) == false) {
		value = CSS_BACKGROUND_POSITION_SET as u16;

		let mut compare = getValue(opv) & 0xf0 ;
		if( compare == (BACKGROUND_POSITION_HORZ_SET as u16) ) {
			hlength = peek_bytecode(style) as i32 ;
			advance_bytecode(style);
			hunit = peek_bytecode(style) as u32 ;
			advance_bytecode(style);
		}
		else if( compare == (BACKGROUND_POSITION_HORZ_CENTER as u16) ) {
			hlength = css_int_to_fixed(50);
			hunit = UNIT_PCT as u32;
		}
		else if( compare == (BACKGROUND_POSITION_HORZ_RIGHT as u16) ) {
			hlength = css_int_to_fixed(100);
			hunit = UNIT_PCT as u32;
		}
		else if( compare == (BACKGROUND_POSITION_HORZ_LEFT as u16) ) {
			hlength = css_int_to_fixed(0);
			hunit = UNIT_PCT as u32;
		}

		compare = getValue(opv) & 0x0f ;
		if( compare == (BACKGROUND_POSITION_VERT_SET as u16) ) {
			vlength =  peek_bytecode(style) as i32 ;
			advance_bytecode(style);
			vunit =  peek_bytecode(style) as u32 ;
			advance_bytecode(style);
		}
		else if( compare == (BACKGROUND_POSITION_VERT_CENTER as u16) ) {
			vlength = css_int_to_fixed(50);
			vunit = UNIT_PCT as u32;
		}
		else if( compare == (BACKGROUND_POSITION_VERT_BOTTOM as u16) ) {
			vlength = css_int_to_fixed(100);
			vunit = UNIT_PCT as u32;
		}
		else if( compare == (BACKGROUND_POSITION_VERT_TOP as u16) ) {
			vlength = css_int_to_fixed(0);
			vunit = UNIT_PCT as u32;
		}
	}

	hunit = css__to_css_unit(hunit) as u32;
	vunit = css__to_css_unit(vunit) as u32;

	if (css__outranks_existing( (getOpcode(opv) as u16), 
								isImportant(opv), 
								state,
								isInherit(opv) ) ) {
		set_background_position(state.computed, 
							 	(value as u8),
								hlength, 
								unsafe { cast::transmute(hunit as uint) }, 
								vlength, 
								unsafe { cast::transmute(vunit as uint) });
	}

	CSS_OK
}

pub fn css__set_background_position_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_result {
	match hint.hint_type {
		HINT_LENGTH_H_V=>{
			match hint.position {
				Some(copy x)=>{
					set_background_position(style, hint.status, 
						x.h.value, x.h.unit,
						x.v.value, x.v.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		}
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_background_position(state:@mut css_select_state) -> css_result {

	set_background_position(state.computed, 
		(CSS_BACKGROUND_POSITION_SET as u8), 0,CSS_UNIT_PCT , 0, CSS_UNIT_PCT);
	CSS_OK
}

pub fn css__compose_background_position(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									final:@mut css_computed_style
									) -> css_result {

	let mut result = css_computed_background_position(child);

	if (result.result == (CSS_BACKGROUND_POSITION_INHERIT as u8) ) {
		result = css_computed_background_position(parent);
		
		set_background_position(final, result.result, result.hlength,result.hunit,
								result.vlength, result.vunit);
		CSS_OK
	}
	else {
		set_background_position(final, result.result, result.hlength,result.hunit,
								result.vlength, result.vunit);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// background_repeat.c
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_repeat(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_result {

	let mut value : u16  = CSS_BACKGROUND_REPEAT_INHERIT as u16;

	if (isInherit(opv) == false) {
		let mut match_val = getValue(opv) ;
		if (match_val == (BACKGROUND_REPEAT_NO_REPEAT as u16) ){
			value = (CSS_BACKGROUND_REPEAT_NO_REPEAT as u16);
		}
		else if (match_val == (BACKGROUND_REPEAT_REPEAT_X as u16) ){
			value = (CSS_BACKGROUND_REPEAT_REPEAT_X as u16);
		}
		else if (match_val == (BACKGROUND_REPEAT_REPEAT_Y as u16) ){
			value = (CSS_BACKGROUND_REPEAT_REPEAT_Y as u16);
		}
		else if (match_val == (BACKGROUND_REPEAT_REPEAT as u16) ){
			value = (CSS_BACKGROUND_REPEAT_REPEAT as u16);
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_background_repeat(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_background_repeat_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_result {
	set_background_repeat(style, hint.status);
	CSS_OK
}

pub fn css__initial_background_repeat(state:@mut css_select_state) -> css_result {

	set_background_repeat(state.computed, 
			(CSS_BACKGROUND_REPEAT_REPEAT as u8));
	CSS_OK
}

pub fn css__compose_background_repeat(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_result {

	let mut ftype = css_computed_background_repeat(child);

	if (ftype == (CSS_BACKGROUND_REPEAT_INHERIT as u8) ) {
		ftype = css_computed_background_repeat(parent);
		set_background_repeat(result, ftype);
		CSS_OK
	}
	else {
		set_background_repeat(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////