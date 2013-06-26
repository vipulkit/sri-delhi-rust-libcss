use include::properties::*;

use stylesheet::*;

use core::managed::*;

use include::types::*;
use include::fpmath::*;
use bytecode::bytecode::*;
use bytecode::opcodes::*;
use stylesheet::*;
use utils::errors::*;
use select::common::*;
use select::propset::*;
use select::computed::*;

/* HELPERS --- Useful helpers */
///////////////////////////////////////////////////////////////////

// moved from select , to resolve cyclic dependency between modules
pub fn css__outranks_existing(op:u16, 
							important:bool, 
							state: @mut css_select_state,
							inherit:bool) -> bool {
	let mut existing = copy state.props[op][state.current_pseudo as uint];
	let mut outranks : bool = false;

	/* Sorting on origin & importance gives the following:
	 * 
	 *           | UA, - | UA, i | USER, - | USER, i | AUTHOR, - | AUTHOR, i
	 *           |----------------------------------------------------------
	 * UA    , - |   S       S       Y          Y         Y           Y
	 * UA    , i |   S       S       Y          Y         Y           Y
	 * USER  , - |   -       -       S          Y         Y           Y
	 * USER  , i |   -       -       -          S         -           -
	 * AUTHOR, - |   -       -       -          Y         S           Y
	 * AUTHOR, i |   -       -       -          Y         -           S
	 *
	 * Where the columns represent the origin/importance of the property 
	 * being considered and the rows represent the origin/importance of 
	 * the existing property.
	 *
	 * - means that the existing property must be preserved
	 * Y means that the new property must be applied
	 * S means that the specificities of the rules must be considered.
	 *
	 * If specificities are considered, the highest specificity wins.
	 * If specificities are equal, then the rule defined last wins.
	 *
	 * We have no need to explicitly consider the ordering of rules if
	 * the specificities are the same because:
	 *
	 * a) We process stylesheets in order
	 * b) The selector hash chains within a sheet are ordered such that 
	 *    more specific rules come after less specific ones and, when
	 *    specificities are identical, rules defined later occur after
	 *    those defined earlier.
	 *
	 * Therefore, where we consider specificity, below, the property 
	 * currently being considered will always be applied if its specificity
	 * is greater than or equal to that of the existing property.
	 */

	if !existing.set {
		/* Property hasn't been set before, new one wins */
		outranks = true;
	} 
	else {
		assert!( (CSS_ORIGIN_UA as uint) < (CSS_ORIGIN_USER as uint) );
		assert!( (CSS_ORIGIN_USER as uint) < (CSS_ORIGIN_AUTHOR as uint) );

		if (existing.origin < (state.current_origin as u8) ) {
			/* New origin has more weight than existing one.
			 * Thus, new property wins, except when the existing 
			 * one is USER, i. */
			if ( (existing.important == false) ||
					(existing.origin != (CSS_ORIGIN_USER as u8) ) ) {
				outranks = true;
			}
		} 
		else if (existing.origin == (state.current_origin as u8) ) {
			/* Origins are identical, consider importance, except 
			 * for UA stylesheets, when specificity is always 
			 * considered (as importance is meaningless) */
			if (existing.origin == (CSS_ORIGIN_UA as u8) ) {
				if (state.current_specificity >=
						existing.specificity) {
					outranks = true;
				}
			} 
			else if ((existing.important == false) && important) {
				/* New is more important than old. */
				outranks = true;
			} 
			else if ( existing.important && (important == false)) {
				/* Old is more important than new */
			} 
			else {
				/* Same importance, consider specificity */
				if (state.current_specificity >=
						existing.specificity) {
					outranks = true;
				}
			}
		} else {
			/* Existing origin has more weight than new one.
			 * Thus, existing property wins, except when the new
			 * one is USER, i. */
			if ( ((state.current_origin as u8) == (CSS_ORIGIN_USER as u8)) &&
					important) {
				outranks = true;
			}
		}
	}

	if (outranks) {
		/* The new property is about to replace the old one.
		 * Update our state to reflect this. */
		existing.set = true;
		existing.specificity = state.current_specificity;
		existing.origin = (state.current_origin as u8);
		existing.important = important;
		existing.inherit = inherit;
	}

	// update existing in proptable of the select state machine 
	state.props[op][state.current_pseudo as uint] = existing ;
	outranks
}

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
		 fun:@extern fn (@mut css_computed_style, u8, css_color)) -> css_error {
	
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
				color = peek_bytecode(style);
				advance_bytecode(style)
			}
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		(*fun)(state.computed, value as u8, color)
	}

	CSS_OK
}


pub fn css__cascade_uri_none(opv:u32, style:@mut css_style, state:@mut css_select_state, 
	fun:Option<@extern fn (@mut css_computed_style, u8, ~str)>) -> css_error {
	
	let mut value : uint = CSS_BACKGROUND_IMAGE_INHERIT as uint;
	let mut uri: Option<~str> = None;
	//let mut error:css_error;

	if !isInherit(opv) {
		match getValue(opv) {
			BACKGROUND_IMAGE_NONE => value = CSS_BACKGROUND_IMAGE_NONE as uint,
			BACKGROUND_IMAGE_URI => {
				value = CSS_BACKGROUND_IMAGE_IMAGE;
				let (_, ret_uri) = style.sheet.get().css__stylesheet_string_get(peek_bytecode(style) as uint);
				uri = ret_uri;
				advance_bytecode(style)	
			},
			_ => {}
		}
	}

	// \todo lose fun != NULL once all properties have set routines 
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			if uri.is_some() {
				(*fun_fn)(state.computed, value as u8, uri.unwrap())	
			}
			else {
				(*fun_fn)(state.computed, value as u8, ~"")	
			}
		},
		None => {}
	}

	CSS_OK
}


pub fn css__cascade_border_style(opv:u32, _:@mut css_style,	state:@mut css_select_state, 
	fun:@extern fn (@mut css_computed_style, u8) ) -> css_error {
	
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
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		(*fun)(state.computed, value as u8)
	}

	CSS_OK
}


pub fn css__cascade_border_width(opv:u32, style:@mut css_style, state:@mut css_select_state, 
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit)) -> css_error {
	
	let mut value = CSS_BORDER_WIDTH_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;

	if !isInherit(opv) {
		match getValue(opv) {
			BORDER_WIDTH_SET => {
				value = CSS_BORDER_WIDTH_WIDTH;
				length = peek_bytecode(style);
				advance_bytecode(style);
				unit =  peek_bytecode(style);
				advance_bytecode(style)				
			},	
			BORDER_WIDTH_THIN => value = CSS_BORDER_WIDTH_THIN,				
			BORDER_WIDTH_MEDIUM => value = CSS_BORDER_WIDTH_MEDIUM,
			BORDER_WIDTH_THICK => value = CSS_BORDER_WIDTH_THICK,
			_ => {}
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		(*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length_auto(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) ) -> css_error {
	
	let mut value = CSS_BOTTOM_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		match getValue(opv) {
			BOTTOM_SET => {
				value = CSS_BOTTOM_SET;
				length = peek_bytecode(style);
				advance_bytecode(style);
				unit =  peek_bytecode(style);
				advance_bytecode(style)				
			},	
			BOTTOM_AUTO => value = CSS_BOTTOM_AUTO,				
			_ => {}
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		(*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK

}


pub fn css__cascade_length_normal(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) ) -> css_error {
	
	let mut value = CSS_LETTER_SPACING_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		match getValue(opv) {
			LETTER_SPACING_SET => {
				value = CSS_LETTER_SPACING_SET;
				length = peek_bytecode(style);
				advance_bytecode(style);
				unit =  peek_bytecode(style);
				advance_bytecode(style)				
			},	
			LETTER_SPACING_NORMAL => value = CSS_LETTER_SPACING_NORMAL,				
			_ => {}
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		(*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length_none(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) ) -> css_error {

	let mut value = CSS_MAX_HEIGHT_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		match getValue(opv) {
			MAX_HEIGHT_SET => {
				value = CSS_MAX_HEIGHT_SET;
				length = peek_bytecode(style);
				advance_bytecode(style);
				unit =  peek_bytecode(style);
				advance_bytecode(style)				
			},	
			MAX_HEIGHT_NONE => value = CSS_MAX_HEIGHT_NONE,				
			_ => {}
		}
	}

	unit = css__to_css_unit(unit) as u32;

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		(*fun)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
	}

	CSS_OK
}


pub fn css__cascade_length(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:Option<@extern fn (@mut css_computed_style, u8, css_fixed, css_unit) >) -> css_error {

	let mut value = CSS_MIN_HEIGHT_INHERIT;
	let mut length = 0;
	let mut unit = UNIT_PX;
	
	if !isInherit(opv) {
		value = CSS_MIN_HEIGHT_SET;
		length = peek_bytecode(style);
		advance_bytecode(style);
		unit =  peek_bytecode(style);
		advance_bytecode(style)				
	}

	unit = css__to_css_unit(unit) as u32;

	// \todo lose fun != NULL once all properties have set routines */
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			(*fun_fn)(state.computed, value as u8, length as i32, unsafe { cast::transmute(unit as uint) } )
		},
		None => {}
	}

	CSS_OK
}


pub fn css__cascade_number(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:Option<@extern fn (@mut css_computed_style, u8, css_fixed) -> css_error>) -> css_error {

	let mut value = 0;
	let mut length = 0;
	
	// \todo values */

	if !isInherit(opv) {
		value = 0;
		length = peek_bytecode(style);
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
		fun:Option<@extern fn (@mut css_computed_style, u8)>) -> css_error {
	
	let mut value = CSS_PAGE_BREAK_AFTER_INHERIT;

	if !isInherit(opv) {
		match getValue(opv) {
			PAGE_BREAK_AFTER_AUTO => value = CSS_PAGE_BREAK_AFTER_AUTO,	
			PAGE_BREAK_AFTER_ALWAYS => value = CSS_PAGE_BREAK_AFTER_ALWAYS,
			PAGE_BREAK_AFTER_AVOID => value = CSS_PAGE_BREAK_AFTER_AVOID,
			PAGE_BREAK_AFTER_LEFT => value = CSS_PAGE_BREAK_AFTER_LEFT,
			PAGE_BREAK_AFTER_RIGHT => value = CSS_PAGE_BREAK_AFTER_RIGHT,				
			_ => {}
		}
	}

	// \todo lose fun != None */
	match fun {
		Some(fun_fn) => if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			(*fun_fn)(state.computed, value as u8)
		},
		None => {}
	}

	CSS_OK
	
}

pub fn css__cascade_counter_increment_reset(opv:u32, style:@mut css_style, state:@mut css_select_state,
	fun:@extern fn (@mut css_computed_style, u8, ~[@mut css_computed_counter]) ) -> css_error {

	let mut value : uint = CSS_COUNTER_INCREMENT_INHERIT as uint;
	let mut counters:~[@mut css_computed_counter] = ~[];
	
	if !isInherit(opv) {
		match getValue(opv) {
			COUNTER_INCREMENT_NAMED => {
				let mut v = getValue(opv) as u32;

				while v != COUNTER_INCREMENT_NONE as u32{
					
					let (result, name_option) = style.sheet.get().css__stylesheet_string_get((peek_bytecode(style)) as uint);
					advance_bytecode(style);
					match result {
						CSS_OK => {
							let val = peek_bytecode(style);
							advance_bytecode(style);

							let temp = @mut css_computed_counter{name:name_option.unwrap(),value:val as i32};
							counters.push(temp);

							v = peek_bytecode(style);
							advance_bytecode(style);
						},
						_ => return result
					}
						
				}
			},	
			COUNTER_INCREMENT_NONE => value = CSS_COUNTER_INCREMENT_NONE,
			_ => {}
		}
	}

	
	/* If we have some counters, terminate the array with a blank entry */
	// if !counters.is_empty() {
	// 	let temp = @mut css_computed_counter{name:~"",value:0};
	// 	counters.push(temp);
	// }

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,	isInherit(opv)) {
		(*fun)(state.computed, value as u8, counters)
	}
	
	CSS_OK
}


///////////////////////////////////////////////////////////////////


// Azimuth
///////////////////////////////////////////////////////////////////
pub fn css__cascade_azimuth(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error
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
		_:@mut css_computed_style) -> css_error {

	CSS_OK
}

pub fn css__initial_azimuth(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_azimuth(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////


// background_attachment
///////////////////////////////////////////////////////////////////

pub fn css__cascade_background_attachment(opv:u32, _:@mut css_style, 
										state:@mut css_select_state
										) -> css_error {

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
												) -> css_error {

	set_background_attachment(style, hint.status);
	CSS_OK
}

pub fn css__initial_background_attachment(state:@mut css_select_state) -> css_error {

	set_background_attachment(state.computed, 
		(CSS_BACKGROUND_ATTACHMENT_SCROLL as u8) );
	CSS_OK
}

pub fn css__compose_background_attachment(parent:@mut css_computed_style,
										child:@mut css_computed_style,
										result:@mut css_computed_style
										) -> css_error {

	let mut ftype : u8 = css_computed_background_attachment(child);

	if (ftype == (CSS_BACKGROUND_ATTACHMENT_INHERIT as u8) ) {
		ftype = css_computed_background_attachment(parent);
	}

	set_background_attachment(result, (ftype as u8) );
	CSS_OK
}


///////////////////////////////////////////////////////////////////

// background_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_bg_border_color(opv, style, state, @set_background_color);
}

pub fn css__set_background_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COLOR=>{
			set_background_color(style, hint.status, hint.color.get_or_default(0));
			CSS_OK
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_background_color(state:@mut css_select_state) -> css_error {

	set_background_color(state.computed, 
		(CSS_BACKGROUND_COLOR_COLOR as u8), 0);
	CSS_OK
}

pub fn css__compose_background_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

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

// background_image
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_image(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {
	return css__cascade_uri_none(opv, style, state, Some(@set_background_image) );
}

pub fn css__set_background_image_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		STRING=>{
			match hint.string {
				Some(copy x)=>{
					set_background_image(style, hint.status, x);
				},
				None=>{
					set_background_image(style, hint.status, ~"");
				}
			}
			CSS_OK
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_background_image(state:@mut css_select_state) -> css_error {

	set_background_image(state.computed, 
		(CSS_BACKGROUND_IMAGE_NONE as u8), ~"");
	CSS_OK
}

pub fn css__compose_background_image(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,url) = css_computed_background_image(child);

	if (ftype == (CSS_BACKGROUND_IMAGE_INHERIT as u8) ) {
		let mut (ftype2,url2) = css_computed_background_image(parent);
		set_background_image(result, ftype2, url2);
		CSS_OK
	}
	else {
		set_background_image(result, ftype, url);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// background_position
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_position(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

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
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH_H_V=>{
			match hint.position {
				Some(x)=>{
					set_background_position(style, hint.status, 
						x.h.value, x.h.unit,
						x.v.value, x.v.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_background_position(state:@mut css_select_state) -> css_error {

	set_background_position(state.computed, 
		(CSS_BACKGROUND_POSITION_SET as u8), 0,CSS_UNIT_PCT , 0, CSS_UNIT_PCT);
	CSS_OK
}

pub fn css__compose_background_position(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									final:@mut css_computed_style
									) -> css_error {

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

// background_repeat
///////////////////////////////////////////////////////////////////
pub fn css__cascade_background_repeat(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

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
										) -> css_error {

	set_background_repeat(style, hint.status);
	CSS_OK
}

pub fn css__initial_background_repeat(state:@mut css_select_state) -> css_error {

	set_background_repeat(state.computed, 
			(CSS_BACKGROUND_REPEAT_REPEAT as u8));
	CSS_OK
}

pub fn css__compose_background_repeat(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

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

// border_bottom_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_bottom_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_bg_border_color(opv, style, state,
			@set_border_bottom_color);
}

pub fn css__set_border_bottom_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COLOR=>{
			match hint.color {
				Some(x)=>{
					set_border_bottom_color(style, hint.status, x);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_bottom_color(state:@mut css_select_state) -> css_error {

	set_border_bottom_color(state.computed, 
			(CSS_BORDER_COLOR_CURRENT_COLOR as u8), 0);
	CSS_OK
}

pub fn css__compose_border_bottom_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,color) = css_computed_border_bottom_color(child);

	if (ftype == (CSS_BORDER_COLOR_INHERIT as u8) ) {
		let mut (ftype2,color2) = css_computed_border_bottom_color(parent);
		set_border_bottom_color(result, ftype2, color2 );
		CSS_OK
	}
	else {
		set_border_bottom_color(result, ftype, color );
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////
// caption_side
///////////////////////////////////////////////////////////////////
pub fn css__cascade_caption_side(opv:u32, _:@mut css_style, state:@mut css_select_state) -> css_error {
	
	let mut value = CSS_CAPTION_SIDE_INHERIT;

	if !isInherit(opv) {
		match getValue(opv) {
			CAPTION_SIDE_TOP => value = CSS_CAPTION_SIDE_TOP,	
			CAPTION_SIDE_BOTTOM => value = CSS_CAPTION_SIDE_BOTTOM,
			_ => {}
		}
	}

	// \todo lose fun != None */
	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			set_caption_side(state.computed, value as u8)
	}

	CSS_OK
}

pub fn css__set_caption_side_from_hint(hint:@mut css_hint, 
										style:@mut css_computed_style) 
										-> css_error {
	set_caption_side(style, hint.status);
	CSS_OK
}

pub fn css__initial_caption_side(state:@mut css_select_state) -> css_error {
	set_caption_side(state.computed, CSS_CAPTION_SIDE_TOP as u8);
	CSS_OK
}

pub fn css__compose_caption_side(parent:@mut css_computed_style, 
								child:@mut css_computed_style,
								result:@mut css_computed_style) -> css_error{

	let mut cap_type = css_computed_caption_side(child);

	if cap_type ==	CSS_CAPTION_SIDE_INHERIT as u8 {
		cap_type = css_computed_caption_side(parent)
	}
		
	set_caption_side(result, cap_type);
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// clear
///////////////////////////////////////////////////////////////////
pub fn css__cascade_clear(opv:u32, _:@mut css_style, state:@mut css_select_state) -> css_error {

	let mut value = CSS_CLEAR_INHERIT;

	if !isInherit(opv) {
		match getValue(opv) {
			CLEAR_NONE => value = CSS_CLEAR_NONE,	
			CLEAR_LEFT => value = CSS_CLEAR_LEFT,
			CLEAR_RIGHT => value = CSS_CLEAR_RIGHT,	
			CLEAR_BOTH => value = CSS_CLEAR_BOTH,
			_ => {}
		}
	}

	// \todo lose fun != None */
	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			set_clear(state.computed, value as u8) ;
	}

	CSS_OK
}

pub fn css__set_clear_from_hint(hint:@mut css_hint, style:@mut css_computed_style) 
								-> css_error {
	set_clear(style, hint.status);
	CSS_OK
}

pub fn css__initial_clear(state:@mut css_select_state ) -> css_error {
	set_clear(state.computed, CSS_CLEAR_NONE as u8);
	CSS_OK
}

pub fn css__compose_clear(parent:@mut css_computed_style, 
						child:@mut css_computed_style,
						result:@mut css_computed_style) -> css_error {

	let mut clear_type = css_computed_clear(child);

	if clear_type == CSS_CLEAR_INHERIT as u8 {
		clear_type = css_computed_clear(parent);
	}

	set_clear(result, clear_type) ;
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// clip
///////////////////////////////////////////////////////////////////

pub fn css__cascade_clip(opv:u32, style:@mut css_style, state:@mut css_select_state) -> css_error {

	let mut value = CSS_CLIP_INHERIT;
	let rect = 
        @mut css_computed_clip_rect{
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

	if !isInherit(opv) {
		match getValue(opv) & CLIP_SHAPE_MASK {
			CLIP_SHAPE_RECT => {
				if (getValue(opv) & CLIP_RECT_TOP_AUTO) != 0 {
					rect.top_auto = true;
				} 
				else {
					rect.top = peek_bytecode(style) as i32;
					advance_bytecode(style);
					rect.tunit = css__to_css_unit(peek_bytecode(style));
					advance_bytecode(style);
				}
				
				if (getValue(opv) & CLIP_RECT_RIGHT_AUTO) != 0 {
					rect.right_auto = true;
				}
				else {
					rect.right = peek_bytecode(style) as i32;
					advance_bytecode(style);
					rect.runit = css__to_css_unit(peek_bytecode(style));
					advance_bytecode(style);
				}

				if (getValue(opv) & CLIP_RECT_BOTTOM_AUTO) != 0 {
					rect.bottom_auto = true;
				}
				else {
					rect.bottom = peek_bytecode(style) as i32;
					advance_bytecode(style);
					rect.bunit = css__to_css_unit(peek_bytecode(style));
					advance_bytecode(style);
				}

				if (getValue(opv) & CLIP_RECT_LEFT_AUTO) != 0 {
					rect.left_auto = true;
				}
				else {
					rect.left = peek_bytecode(style) as i32;
					advance_bytecode(style);
					rect.lunit = css__to_css_unit(peek_bytecode(style));
					advance_bytecode(style);
				}
				value = CSS_CLIP_RECT;
			},	
			CLIP_AUTO => value = CSS_CLIP_AUTO,
			_ => {}
		}
	}

	rect.tunit = css__to_css_unit(rect.tunit as u32);
	rect.runit = css__to_css_unit(rect.runit as u32);
	rect.bunit = css__to_css_unit(rect.bunit as u32);
	rect.lunit = css__to_css_unit(rect.lunit as u32);


	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
			set_clip(state.computed, value as u8, rect)
	}

	CSS_OK
}	
			
pub fn css__set_clip_from_hint(hint:@mut css_hint, style:@mut css_computed_style) 
								-> css_error {
	set_clip(style, hint.status, hint.clip.unwrap()) ;
	CSS_OK
}

pub fn css__initial_clip(state:@mut css_select_state) -> css_error{

	let rect = @mut css_computed_clip_rect{
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
    };

	set_clip(state.computed, CSS_CLIP_AUTO as u8, rect) ;
	CSS_OK
}

pub fn css__compose_clip(parent:@mut css_computed_style, 
						child:@mut css_computed_style,
						result:@mut css_computed_style) 
						-> css_error {

	
	let mut (clip_type, rect) = css_computed_clip(child);

	if (match child.uncommon { None => true, _ => false} && match parent.uncommon { Some(_) => true,  None => false }) 
		|| clip_type == CSS_CLIP_INHERIT as u8 || ( match child.uncommon {Some(_) => true, None => false} && 
			!mut_ptr_eq(result,child)) {
		
		if (match child.uncommon { None => true, _ => false} && match parent.uncommon { Some(_) => true,  None => false }) || 
		   clip_type == CSS_CLIP_INHERIT as u8 {
			let (clip_type_ret, rect_ret) = css_computed_clip(parent);
			clip_type = clip_type_ret;
			rect = rect_ret
		}

		set_clip(result, clip_type, rect.unwrap());
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////


// border_bottom_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_bottom_style(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_style(opv, style, state, @set_border_bottom_style);
}

pub fn css__set_border_bottom_style_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_border_bottom_style(style, hint.status);
	CSS_OK
}

pub fn css__initial_border_bottom_style(state:@mut css_select_state) -> css_error {

	set_border_bottom_style(state.computed, (CSS_BORDER_STYLE_NONE as u8) );
	CSS_OK
}

pub fn css__compose_border_bottom_style(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_border_bottom_style(child);

	if (ftype == (CSS_BORDER_STYLE_INHERIT as u8) ) {
		ftype = css_computed_border_bottom_style(parent);
		set_border_bottom_style(result, ftype );
		CSS_OK
	}
	else {
		set_border_bottom_style(result, ftype );
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////
// border_bottom_width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_bottom_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_width(opv, style, state, @set_border_bottom_width);
}

pub fn css__set_border_bottom_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_border_bottom_width(style, hint.status, x.value , x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_bottom_width(state:@mut css_select_state) -> css_error {

	set_border_bottom_width(state.computed, 
						(CSS_BORDER_WIDTH_MEDIUM as u8),
						0, 
						CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_border_bottom_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_border_bottom_width(child);

	if (ftype == (CSS_BORDER_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_border_bottom_width(parent);
		set_border_bottom_width(result, 
								ftype2, 
								olength2.get_or_default( olength.get_or_default(0) ), 
								ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_border_bottom_width(result, ftype, 
							olength.get_or_default(0), 
							ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// border_collapse
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_collapse(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_BORDER_COLLAPSE_INHERIT as u16;

	if (isInherit(opv) == false) {
		let mut match_val = getValue(opv) ; 
		if ( match_val == (BORDER_COLLAPSE_SEPARATE as u16) ){ 
			value = CSS_BORDER_COLLAPSE_SEPARATE as u16;
		}
		if ( match_val == (BORDER_COLLAPSE_COLLAPSE as u16) ){ 
			value = CSS_BORDER_COLLAPSE_COLLAPSE as u16;
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), 
								isImportant(opv), 
								state,
								isInherit(opv))) {
		set_border_collapse(state.computed, (value as u8) );
		CSS_OK
	}
	else {
		CSS_OK
	}
}

pub fn css__set_border_collapse_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_border_collapse(style, hint.status);
	CSS_OK
}

pub fn css__initial_border_collapse(state:@mut css_select_state) -> css_error {


	set_border_collapse(state.computed, (CSS_BORDER_COLLAPSE_SEPARATE as u8) );
	CSS_OK
}

pub fn css__compose_border_collapse(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_border_collapse(child);

	if (ftype == (CSS_BORDER_COLLAPSE_INHERIT as u8) ) {
		ftype = css_computed_border_collapse(parent);
		set_border_collapse(result, ftype);
		CSS_OK
	}
	else {
		set_border_collapse(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// border_left_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_left_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_bg_border_color(opv, style, state, 
			@set_border_left_color);
}

pub fn css__set_border_left_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COLOR=>{
			match hint.color {
				Some(x)=>{
					set_border_left_color(style, hint.status, x);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_left_color(state:@mut css_select_state) -> css_error {


	set_border_left_color(state.computed, 
			(CSS_BORDER_COLOR_CURRENT_COLOR as u8), 0);
	CSS_OK
}

pub fn css__compose_border_left_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,color) = css_computed_border_left_color(child);

	if (ftype == (CSS_BORDER_COLOR_INHERIT as u8) ) {
		let mut (ftype2,color2) = css_computed_border_left_color(parent);
		set_border_left_color(result, ftype2, color2);
		CSS_OK
	}
	else {
		set_border_left_color(result, ftype, color);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// border_left_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_left_style(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_style(opv, style, state, @set_border_left_style);
}

pub fn css__set_border_left_style_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_border_left_style(style, hint.status);
	CSS_OK
}

pub fn css__initial_border_left_style(state:@mut css_select_state) -> css_error {


	set_border_left_style(state.computed, (CSS_BORDER_STYLE_NONE as u8) );
	CSS_OK
}

pub fn css__compose_border_left_style(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_border_left_style(child);

	if (ftype == (CSS_BORDER_STYLE_INHERIT as u8) ) {
		ftype = css_computed_border_left_style(parent);
	}

	set_border_left_style(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// border_left_width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_left_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_width(opv, style, state, @set_border_left_width);
}

pub fn css__set_border_left_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_border_left_width(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_left_width(state:@mut css_select_state) -> css_error {


	set_border_left_width(state.computed, 
						(CSS_BORDER_WIDTH_MEDIUM as u8),
						0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_border_left_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_border_left_width(child);

	if (ftype == (CSS_BORDER_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_border_left_width(parent);
		set_border_left_width(result, 
							ftype2, 
							olength2.get_or_default( olength.get_or_default(0) ), 
							ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_border_left_width(result, ftype, 
			olength.get_or_default(0), 
			ounit.get_or_default(CSS_UNIT_PX) );
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// border_right_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_right_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_bg_border_color(opv, style, state, 
			@set_border_right_color);
}

pub fn css__set_border_right_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COLOR=>{
			match hint.color {
				Some(x)=>{
					set_border_right_color(style, hint.status, x);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_right_color(state:@mut css_select_state) -> css_error {


	set_border_right_color(state.computed, 
			(CSS_BORDER_COLOR_CURRENT_COLOR as u8), 0);
	CSS_OK
}

pub fn css__compose_border_right_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,color) = css_computed_border_right_color(child);

	if (ftype == (CSS_BORDER_COLOR_INHERIT as u8) ) {
		let mut (ftype2,color2) = css_computed_border_right_color(parent);
		set_border_right_color(result, ftype2, color2);
		CSS_OK
	}
	else {
		set_border_right_color(result, ftype, color);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// border_right_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_right_style(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_style(opv, style, state, @set_border_right_style);
}

pub fn css__set_border_right_style_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_border_right_style(style, hint.status);
	CSS_OK
}

pub fn css__initial_border_right_style(state:@mut css_select_state) -> css_error {


	set_border_right_style(state.computed, (CSS_BORDER_STYLE_NONE as u8) );
	CSS_OK
}

pub fn css__compose_border_right_style(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_border_right_style(child);

	if (ftype == (CSS_BORDER_STYLE_INHERIT as u8) ) {
		ftype = css_computed_border_right_style(parent);
	}

	set_border_right_style(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// border_right_width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_right_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_width(opv, style, state, @set_border_right_width);
}

pub fn css__set_border_right_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_border_right_width(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_right_width(state:@mut css_select_state) -> css_error {


	set_border_right_width(state.computed, 
				(CSS_BORDER_WIDTH_MEDIUM as u8),
				0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_border_right_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_border_right_width(child);

	if (ftype == (CSS_BORDER_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_border_right_width(parent);
		set_border_right_width(result, 
							ftype2, 
							olength2.get_or_default( olength.get_or_default(0) ), 
							ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_border_right_width(result, ftype, 
			olength.get_or_default(0), 
			ounit.get_or_default(CSS_UNIT_PX) );
		CSS_OK
	}
}	

///////////////////////////////////////////////////////////////////

// border_spacing
///////////////////////////////////////////////////////////////////


pub fn css__cascade_border_spacing(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_BORDER_SPACING_INHERIT as u16;
	let mut hlength :i32 = 0;
	let mut vlength :i32 = 0;
	let mut hunit : u32 = UNIT_PX as u32;
	let mut vunit : u32 = UNIT_PX as u32;

	if (isInherit(opv) == false) {
		value = CSS_BORDER_SPACING_SET as u16;
		hlength = peek_bytecode(style) as i32;
		advance_bytecode(style);
		hunit = peek_bytecode(style) ;
		advance_bytecode(style);

		vlength = peek_bytecode(style) as i32;
		advance_bytecode(style);
		vunit = peek_bytecode(style) ;
		advance_bytecode(style);
	}

	hunit = css__to_css_unit(hunit) as u32;
	vunit = css__to_css_unit(vunit) as u32;

	if (css__outranks_existing( (getOpcode(opv) as u16), 
								isImportant(opv), state,
								isInherit(opv))) {
		set_border_spacing(state.computed, 
							(value as u8),
							hlength, 
							unsafe { cast::transmute(hunit as uint) }, 
							vlength, 
							unsafe { cast::transmute(vunit as uint) });
	}

	CSS_OK
}

pub fn css__set_border_spacing_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH_H_V=>{
			match hint.position {
				Some(x)=>{
					set_border_spacing(style, hint.status,
										x.h.value, x.h.unit,
										x.v.value, x.v.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_spacing(state:@mut css_select_state) -> css_error {

	set_border_spacing(state.computed, (CSS_BORDER_SPACING_SET as u8),
			0, CSS_UNIT_PX, 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_border_spacing(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut rect = css_computed_border_spacing(child);

	if ( (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			rect.result == (CSS_BORDER_SPACING_INHERIT as u8) ||
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) )) {
		
		if ((child.uncommon.is_none() && parent.uncommon.is_some() ) || 
				rect.result == (CSS_BORDER_SPACING_INHERIT as u8) ) {
			rect = css_computed_border_spacing(parent);
		}

		set_border_spacing(result, rect.result , rect.hlength, rect.hunit, 
				rect.vlength, rect.vunit);
	}

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// border_top_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_top_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_bg_border_color(opv, style, state, @set_border_top_color);
}

pub fn css__set_border_top_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COLOR=>{
			match hint.color {
				Some(x)=>{
					set_border_top_color(style, hint.status, x);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_top_color(state:@mut css_select_state) -> css_error {


	set_border_top_color(state.computed, 
		(CSS_BORDER_COLOR_CURRENT_COLOR as u8), 0);
	CSS_OK
}

pub fn css__compose_border_top_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,color) = css_computed_border_top_color(child);

	if (ftype == (CSS_BORDER_COLOR_INHERIT as u8) ) {
		let mut (ftype2,color2) = css_computed_border_top_color(parent);
		set_border_top_color(result, ftype2, color2);
		CSS_OK
	}
	else {
		set_border_top_color(result, ftype, color);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// border_top_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_top_style(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_style(opv, style, state, @set_border_top_style);
}

pub fn css__set_border_top_style_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_border_top_style(style, hint.status);
	CSS_OK
}

pub fn css__initial_border_top_style(state:@mut css_select_state) -> css_error {


	set_border_top_style(state.computed, (CSS_BORDER_STYLE_NONE as u8) );
	CSS_OK
}

pub fn css__compose_border_top_style(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_border_top_style(child);

	if (ftype == (CSS_BORDER_STYLE_INHERIT as u8) ) {
		ftype = css_computed_border_top_style(parent);
	}

	set_border_top_style(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// border_top_width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_border_top_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_width(opv, style, state, @set_border_top_width);
}

pub fn css__set_border_top_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_border_top_width(style, hint.status,x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_border_top_width(state:@mut css_select_state) -> css_error {


	set_border_top_width(state.computed, 
			(CSS_BORDER_WIDTH_MEDIUM as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_border_top_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_border_top_width(child);

	if (ftype == (CSS_BORDER_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_border_top_width(parent);
		set_border_top_width(result, 
							ftype2, 
							olength2.get_or_default( olength.get_or_default(0) ), 
							ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_border_top_width(result, ftype, 
			olength.get_or_default(0), 
			ounit.get_or_default(CSS_UNIT_PX) );
		CSS_OK
	}
}	

///////////////////////////////////////////////////////////////////

// bottom
///////////////////////////////////////////////////////////////////
pub fn css__cascade_bottom(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_bottom);
}

pub fn css__set_bottom_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_bottom(style, hint.status,x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_bottom(state:@mut css_select_state) -> css_error {

	set_bottom(state.computed, (CSS_BOTTOM_AUTO as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_bottom(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_bottom(child);

	if (ftype == (CSS_BOTTOM_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_bottom(parent);
		set_bottom(result, 
				ftype2, 
				olength2.get_or_default( olength.get_or_default(0) ), 
				ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_bottom(result, ftype, 
			olength.get_or_default(0), 
			ounit.get_or_default(CSS_UNIT_PX) );
		CSS_OK
	}
}	

///////////////////////////////////////////////////////////////////


// break_after
///////////////////////////////////////////////////////////////////
pub fn css__cascade_break_after(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			BREAK_AFTER_AUTO => {},
			BREAK_AFTER_ALWAYS => {},
			BREAK_AFTER_AVOID => {},
			BREAK_AFTER_LEFT => {},
			BREAK_AFTER_RIGHT => {},
			BREAK_AFTER_PAGE => {},
			BREAK_AFTER_COLUMN => {},
			BREAK_AFTER_AVOID_PAGE => {},
			BREAK_AFTER_AVOID_COLUMN => {},
			/* \todo convert to public values */
		_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		// todo set computed elevation
		return CSS_OK ;
	}

	CSS_OK
}

pub fn css__set_break_after_from_hint(_:@mut  css_hint, 
										_:@mut css_computed_style
										) -> css_error {

	CSS_OK
}

pub fn css__initial_break_after(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_break_after(_:@mut css_computed_style,
									_:@mut css_computed_style,
									_:@mut css_computed_style
									) -> css_error {

	CSS_OK
}	

///////////////////////////////////////////////////////////////////
// color
///////////////////////////////////////////////////////////////////
pub fn  css__cascade_color(opv:u32, style:@mut css_style, state:@mut css_select_state) -> css_error {
	
println("color test: inside css__cascade_color");
	let mut inherit = isInherit(opv);
	let mut value = CSS_COLOR_INHERIT;
	let mut color = 0;

	if !inherit {
		match getValue(opv) {
			COLOR_TRANSPARENT => {
				value = CSS_COLOR_COLOR
			},	
			COLOR_CURRENT_COLOR => {
				value = CSS_COLOR_INHERIT; //color: currentColor always computes to inherit 
				inherit = true
			}, 
			COLOR_SET => {
				value = CSS_COLOR_COLOR;
				color = peek_bytecode(style);
				advance_bytecode(style);
			},
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, inherit) {
		set_color(state.computed, value as u8, color)
	}

	CSS_OK

}

pub fn css__set_color_from_hint(hint:@mut css_hint, style:@mut css_computed_style) 
								-> css_error {

								set_color(style, hint.status, hint.color.get()) ;
	CSS_OK
}

pub fn css__initial_color(state:@mut css_select_state) -> css_error {
		
	let mut hint = @mut css_hint{
		        hint_type:HINT_LENGTH,
		        status:0,
		        clip:None,
		        content:None,
		        counters:None,
		        length:None,
		        position:None,
		        color:None,
		        fixed:None,
		        integer:None,
		        string:None,
		        strings:None
	};

	let mut error : css_error ;
			
	error = (*(state.handler.get().ua_default_for_property))(CSS_PROP_COLOR as u32,hint);
	match  error {
		CSS_OK=>{},
		x => { 
			return x ;
		}
	}

	css__set_color_from_hint(hint,state.computed)	
}

pub fn css__compose_color(parent:@mut css_computed_style, 
						child:@mut css_computed_style,
						result:@mut css_computed_style) 
						-> css_error {
	
println("color test: inside css__compose_color");
	let (color_type, color) = css_computed_color(child);
	
	if color_type == (CSS_COLOR_INHERIT as u8) {
		let (p_color_type, p_color) = css_computed_color(parent);
		set_color(result, p_color_type, p_color.get_or_default(color.get_or_default(0)));
	}
	else {
		set_color(result, color_type, color.get_or_default(0));
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// column_count
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_count(opv:u32, style:@mut css_style, 
		state:@mut css_select_state ) -> css_error {

	// let mut count = 0;

	if !isInherit(opv) {
		match getValue(opv) {
			COLUMN_COUNT_SET => {
				// count = peek_bytecode(style);
				advance_bytecode(style);
			},	
			COLUMN_COUNT_AUTO => {
				// \todo convert to public values */
			},
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_count_from_hint(_:@mut css_hint, _:@mut css_computed_style) 
										-> css_error {
	// DO NOTHING
	CSS_OK
}

pub fn css__initial_column_count(_:@mut css_select_state) -> css_error {
	
	CSS_OK
}

pub fn css__compose_column_count(_:@mut css_computed_style, _:@mut css_computed_style,
								_:@mut css_computed_style) 
								-> css_error {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// column_count
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_fill(opv:u32, _:@mut css_style, 
		state:@mut css_select_state ) -> css_error {

	if !isInherit(opv) {
		match getValue(opv) {
			COLUMN_FILL_BALANCE | COLUMN_FILL_AUTO => {
				// \todo convert to public values */
			},	
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_fill_from_hint(_:@mut css_hint, _:@mut css_computed_style) 
									-> css_error {
	// DO NOTHING
	CSS_OK
}

pub fn css__initial_column_fill(_:@mut css_select_state) -> css_error {
	
	CSS_OK
}

pub fn css__compose_column_fill(_:@mut css_computed_style, _:@mut css_computed_style,
								_:@mut css_computed_style) 
								-> css_error {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// column_gap
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_gap(opv:u32, style:@mut css_style, 
		state:@mut css_select_state ) -> css_error {

	// let mut length = 0;
	// let mut unit = UNIT_PX;

	if !isInherit(opv) {
		match getValue(opv) {
			COLUMN_GAP_SET => {
				// length = peek_bytecode(style);
				advance_bytecode(style);
				// unit = peek_bytecode(style);
				advance_bytecode(style)
			},
			COLUMN_GAP_NORMAL => {
				//** \todo convert to public values */	
			},	
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_gap_from_hint(_:@mut css_hint, _:@mut css_computed_style)
									-> css_error {
	// DO NOTHING
	CSS_OK
}

pub fn css__initial_column_gap(_:@mut css_select_state) -> css_error {
	
	CSS_OK
}

pub fn css__compose_column_gap(_:@mut css_computed_style, _:@mut css_computed_style,
								_:@mut css_computed_style) 
								-> css_error  {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// column_rule_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_rule_color(opv:u32, style:@mut css_style, 
		state:@mut css_select_state ) -> css_error {

	// let mut color = 0;
	
	if !isInherit(opv) {
		match getValue(opv) {
			COLUMN_RULE_COLOR_SET => {
				// color = peek_bytecode(style);
				advance_bytecode(style)
			},
			COLUMN_RULE_COLOR_TRANSPARENT | COLUMN_RULE_COLOR_CURRENT_COLOR => {
				//** \todo convert to public values */	
			},	
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_rule_color_from_hint(_:@mut css_hint, 
									_:@mut css_computed_style) -> css_error {
	// DO NOTHING
	CSS_OK
}

pub fn css__initial_column_rule_color(_:@mut css_select_state) -> css_error {
	
	CSS_OK
}

pub fn css__compose_column_rule_color(_:@mut css_computed_style, _:@mut css_computed_style,
									_:@mut css_computed_style) 
									-> css_error {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// column_rule_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_rule_style(opv:u32, _:@mut css_style, 
		state:@mut css_select_state ) -> css_error {

	if !isInherit(opv) {
		match getValue(opv) {
			COLUMN_RULE_STYLE_NONE | COLUMN_RULE_STYLE_HIDDEN |
			COLUMN_RULE_STYLE_DOTTED | COLUMN_RULE_STYLE_DASHED |
			COLUMN_RULE_STYLE_SOLID | COLUMN_RULE_STYLE_DOUBLE | 
			COLUMN_RULE_STYLE_GROOVE | COLUMN_RULE_STYLE_RIDGE | 
			COLUMN_RULE_STYLE_INSET | COLUMN_RULE_STYLE_OUTSET => {
				//** \todo convert to public values */	
			},	
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_rule_style_from_hint(_:@mut css_hint, 
										_:@mut css_computed_style) 
										-> css_error {
	// DO NOTHING
	CSS_OK
}

pub fn css__initial_column_rule_style(_:@mut css_select_state) -> css_error {
	
	CSS_OK
}

pub fn css__compose_column_rule_style(_:@mut css_computed_style, 
									_:@mut css_computed_style,
									_:@mut css_computed_style) 
									-> css_error {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// column_rule_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_rule_width(opv:u32, style:@mut css_style, 
		state:@mut css_select_state ) -> css_error {

	// let mut length = 0;
	// let mut unit = UNIT_PX;

	if !isInherit(opv) {
		match getValue(opv) {
			COLUMN_RULE_WIDTH_SET => {
				// length = peek_bytecode(style);
				advance_bytecode(style);
				// unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			COLUMN_RULE_WIDTH_THIN |
			COLUMN_RULE_WIDTH_MEDIUM | 
			COLUMN_RULE_WIDTH_THICK => {
				//** \todo convert to public values */	
			},	
			_ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
		// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_rule_width_from_hint(_:@mut css_hint, 
											_:@mut css_computed_style) 
											-> css_error {
	// DO NOTHING
	CSS_OK
}

pub fn css__initial_column_rule_width(_:@mut css_select_state) -> css_error {
	
	CSS_OK
}

pub fn css__compose_column_rule_width(_:@mut css_computed_style, 
									_:@mut css_computed_style,
									_:@mut css_computed_style) 
									-> css_error {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////


// break_before
///////////////////////////////////////////////////////////////////
pub fn css__cascade_break_before(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	if (isInherit(opv) == false) {
		match getValue(opv) {
			BREAK_BEFORE_AUTO => {},
			BREAK_BEFORE_ALWAYS => {},
			BREAK_BEFORE_AVOID => {},
			BREAK_BEFORE_LEFT => {},
			BREAK_BEFORE_RIGHT => {},
			BREAK_BEFORE_PAGE => {},
			BREAK_BEFORE_COLUMN => {},
			BREAK_BEFORE_AVOID_PAGE => {},
			BREAK_BEFORE_AVOID_COLUMN => {},
			/* \todo convert to public values */
		_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		/* \todo set computed elevation */
	}
	CSS_OK
}

pub fn css__set_break_before_from_hint(_:@mut  css_hint, 
										_:@mut css_computed_style
										) -> css_error {

	CSS_OK
}

pub fn css__initial_break_before(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_break_before(_:@mut css_computed_style,
									_:@mut css_computed_style,
									_:@mut css_computed_style
									) -> css_error {

	CSS_OK
}	
///////////////////////////////////////////////////////////////////

// break_inside
///////////////////////////////////////////////////////////////////
pub fn css__cascade_break_inside(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	if (isInherit(opv) == false) {
		match getValue(opv) {
			BREAK_INSIDE_AUTO => {},
			BREAK_INSIDE_AVOID => {},
			BREAK_INSIDE_AVOID_PAGE => {},
			BREAK_INSIDE_AVOID_COLUMN => {},
			/*  \todo convert to public values */
		_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		/* \todo set computed elevation */
	}
	CSS_OK
}

pub fn css__set_break_inside_from_hint(_:@mut  css_hint, 
										_:@mut css_computed_style
										) -> css_error {

	CSS_OK
}

pub fn css__initial_break_inside(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_break_inside(_:@mut css_computed_style,
									_:@mut css_computed_style,
									_:@mut css_computed_style
									) -> css_error {

	CSS_OK
}	
///////////////////////////////////////////////////////////////////

// direction
///////////////////////////////////////////////////////////////////
pub fn css__cascade_direction(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = (CSS_DIRECTION_INHERIT as u16);

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			DIRECTION_LTR => {
				value = (CSS_DIRECTION_LTR as u16);
			},
			DIRECTION_RTL => {
				value = (CSS_DIRECTION_RTL as u16);
			}
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16) , isImportant(opv), state,
			isInherit(opv))) {
		set_direction(state.computed, (value as u8) );
	}
	CSS_OK
}

pub fn css__set_direction_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_direction(style, hint.status);
	CSS_OK
}

pub fn css__initial_direction(state:@mut css_select_state) -> css_error {


	set_direction(state.computed, (CSS_DIRECTION_LTR as u8) );
	CSS_OK
}

pub fn css__compose_direction(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_direction(child);

	if (ftype == (CSS_DIRECTION_INHERIT as u8) ) {
		ftype = css_computed_direction(parent);
	}

	set_direction(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// display
///////////////////////////////////////////////////////////////////
pub fn css__cascade_display(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_DISPLAY_INHERIT as u16;

	if (isInherit(opv) == false) {
		match getValue(opv) {
			DISPLAY_INLINE => {
				value = (CSS_DISPLAY_INLINE as u16);
			},
			DISPLAY_BLOCK => {
				value = (CSS_DISPLAY_BLOCK as u16);
			},
			DISPLAY_LIST_ITEM => {
				value = (CSS_DISPLAY_LIST_ITEM as u16);
			},
			DISPLAY_RUN_IN => {
				value = (CSS_DISPLAY_RUN_IN as u16);
			},
			DISPLAY_INLINE_BLOCK => {
				value = (CSS_DISPLAY_INLINE_BLOCK as u16);
			},
			DISPLAY_TABLE => {
				value = (CSS_DISPLAY_TABLE as u16);
			},
			DISPLAY_INLINE_TABLE => {
				value = (CSS_DISPLAY_INLINE_TABLE as u16);
			},
			DISPLAY_TABLE_ROW_GROUP => {
				value = (CSS_DISPLAY_TABLE_ROW_GROUP as u16);
			},
			DISPLAY_TABLE_HEADER_GROUP => {
				value = (CSS_DISPLAY_TABLE_HEADER_GROUP as u16);
			},
			DISPLAY_TABLE_FOOTER_GROUP => {
				value = (CSS_DISPLAY_TABLE_FOOTER_GROUP as u16);
			},
			DISPLAY_TABLE_ROW => {
				value = (CSS_DISPLAY_TABLE_ROW as u16);
			},
			DISPLAY_TABLE_COLUMN_GROUP => {
				value = (CSS_DISPLAY_TABLE_COLUMN_GROUP as u16);
			},
			DISPLAY_TABLE_COLUMN => {
				value = (CSS_DISPLAY_TABLE_COLUMN as u16);
			},
			DISPLAY_TABLE_CELL => {
				value = (CSS_DISPLAY_TABLE_CELL as u16);
			},
			DISPLAY_TABLE_CAPTION => {
				value = (CSS_DISPLAY_TABLE_CAPTION as u16);
			},
			DISPLAY_NONE => {
				value = (CSS_DISPLAY_NONE as u16);
			}
			_=>{}
		}
	}
	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_display(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_display_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_display(style, hint.status);
	CSS_OK
}

pub fn css__initial_display(state:@mut css_select_state) -> css_error {


	set_display(state.computed, (CSS_DISPLAY_INLINE as u8) );
	CSS_OK
}

pub fn css__compose_display(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_display_static(child);

	if (ftype == (CSS_DISPLAY_INHERIT as u8) ) {
		ftype = css_computed_display_static(parent);
	}

	set_display(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// elevation
///////////////////////////////////////////////////////////////////
pub fn css__cascade_elevation(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	// let mut val :i32  = 0;
	// let mut unit : u32 = UNIT_DEG ;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			ELEVATION_ANGLE =>{
				// val = peek_bytecode(style) as i32 ;
				advance_bytecode(style);

				// unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			ELEVATION_BELOW |
			ELEVATION_LEVEL |
			ELEVATION_ABOVE |
			ELEVATION_HIGHER |
			ELEVATION_LOWER => {},
				/* \todo convert to public values */
			_=>{}
		}
	}

	//unit = css__to_css_unit(unit);

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		
	}
	CSS_OK
}

pub fn css__set_elevation_from_hint(_:@mut  css_hint, 
										_:@mut css_computed_style
										) -> css_error {

	CSS_OK
}

pub fn css__initial_elevation(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_elevation(_:@mut css_computed_style,
									_:@mut css_computed_style,
									_:@mut css_computed_style
									) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// empty_cells
///////////////////////////////////////////////////////////////////
pub fn css__cascade_empty_cells(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_EMPTY_CELLS_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			EMPTY_CELLS_SHOW => {
				value = CSS_EMPTY_CELLS_SHOW as u16;
			},
			EMPTY_CELLS_HIDE => {
				value = CSS_EMPTY_CELLS_HIDE as u16;
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_empty_cells(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_empty_cells_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_empty_cells(style, hint.status);
	CSS_OK
}

pub fn css__initial_empty_cells(state:@mut css_select_state) -> css_error {


	set_empty_cells(state.computed, (CSS_EMPTY_CELLS_SHOW as u8) );
	CSS_OK
}

pub fn css__compose_empty_cells(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_empty_cells(child);

	if (ftype == (CSS_EMPTY_CELLS_INHERIT as u8) ) {
		ftype = css_computed_empty_cells(parent);
	}

	set_empty_cells(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// float
///////////////////////////////////////////////////////////////////
pub fn css__cascade_float(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_FLOAT_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			FLOAT_LEFT => {
				value = CSS_FLOAT_LEFT as u16;
			},
			FLOAT_RIGHT => {
				value = CSS_FLOAT_RIGHT as u16;
			},
			FLOAT_NONE => {
				value = CSS_FLOAT_NONE as u16;
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_float(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_float_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_float(style, hint.status);
	CSS_OK
}

pub fn css__initial_float(state:@mut css_select_state) -> css_error {


	set_float(state.computed, (CSS_FLOAT_NONE as u8) );
	CSS_OK
}

pub fn css__compose_float(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_float(child);

	if (ftype == (CSS_FLOAT_INHERIT as u8) ) {
		ftype = css_computed_float(parent);
	}

	set_float(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// font_family
///////////////////////////////////////////////////////////////////
pub fn css__cascade_font_family(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_FONT_FAMILY_INHERIT as u16;
	let mut fonts : ~[~str] = ~[] ;

	if (isInherit(opv) == false) {
		let mut v : u32 = getValue(opv) as u32;

		while (v != (FONT_FAMILY_END as u32) ) {
			let mut font : Option<~str> = None  ;

			match (v as u16) {
				FONT_FAMILY_STRING | 
				FONT_FAMILY_IDENT_LIST => {
					match style.sheet {
						None =>{
							return CSS_BADPARM ;
						},
						Some(css_sheet) => {
							let mut (res,ofont) = css_sheet.css__stylesheet_string_get(
																peek_bytecode(style) as uint ) ;
							match res {
				        		CSS_OK=>{ 
				        			font = ofont ;
				        		},
				        		x => {
				        			return x ;
				        		}
		    				}
							advance_bytecode(style);
						}
					}
				},
				FONT_FAMILY_SERIF => {
					if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
						value = (CSS_FONT_FAMILY_SERIF as u16);
					}
				},
				FONT_FAMILY_SANS_SERIF => {
					if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
						value = (CSS_FONT_FAMILY_SANS_SERIF as u16);
					}
				},
				FONT_FAMILY_CURSIVE => {
					if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
						value = (CSS_FONT_FAMILY_CURSIVE as u16);
					}
				},
				FONT_FAMILY_FANTASY => {
					if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
						value = (CSS_FONT_FAMILY_FANTASY as u16);
					}
				},
				FONT_FAMILY_MONOSPACE => {
					if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
						value = (CSS_FONT_FAMILY_MONOSPACE as u16);
					}
				},
				_=>{}
			}

			/* Only use family-names which occur before the first
			 * generic-family. Any values which occur after the
			 * first generic-family are ignored. */
			/* \todo Do this at bytecode generation time? */
			if ( (value == (CSS_FONT_FAMILY_INHERIT as u16)) 
						&& (font.is_some() )  ) {
				
				fonts.push(font.get()) ;
			}

			v = peek_bytecode(style);
			advance_bytecode(style);
		}
	}

	/* Terminate array with blank entry, if needed */
	if (fonts.len() > 0 ) {
		
		if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
			/* The stylesheet doesn't specify a generic family,
			 * but it has specified named fonts.
			 * Fall back to the user agent's default family.
			 * We don't want to inherit, because that will 
			 * incorrectly overwrite the named fonts list too.
			 */
			let mut hint = @mut css_hint{
		        hint_type:HINT_LENGTH,
		        status:0,
		        clip:None,
		        content:None,
		        counters:None,
		        length:None,
		        position:None,
		        color:None,
		        fixed:None,
		        integer:None,
		        string:None,
		        strings:None
		    };
			let mut error : css_error;

			match state.handler {
				None=> {
					return CSS_BADPARM ;
				},
				Some(fnhandler) => {
					error = (*(fnhandler.ua_default_for_property))(
						(CSS_PROP_FONT_FAMILY as u32), hint);
				    match error {
				        CSS_OK=>{
				        	value = hint.status as u16 ;
				        },
				        x => { 
				        	return x ;
				        }
				    }
				}
			}

			if (value == (CSS_FONT_FAMILY_INHERIT as u16) ) {
				/* No sane UA default: assume sans-serif */
				value = (CSS_FONT_FAMILY_SANS_SERIF as u16);
			}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), 
							state, isInherit(opv))) {

		set_font_family(state.computed, (value as u8) , fonts);
	} 

	CSS_OK
}

pub fn css__set_font_family_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		STRINGS_VECTOR=>{
			match hint.strings {
				Some(copy x)=>{
					set_font_family(style, hint.status, x);
				},
				None=>{
					set_font_family(style, hint.status, ~[]);
				}
			}
			hint.strings = Some(~[]);
		},
		_=>{
			return CSS_INVALID ;
		}
	}

	CSS_OK
}

pub fn css__initial_font_family(state:@mut css_select_state) -> css_error {


	let mut hint = @mut css_hint{
        hint_type:HINT_LENGTH,
        status:0,
        clip:None,
        content:None,
        counters:None,
        length:None,
        position:None,
        color:None,
        fixed:None,
        integer:None,
        string:None,
        strings:None
    };
	let mut error : css_error;

	match state.handler {
		None=> {
			return CSS_BADPARM ;
		},
		Some(fnhandler) => {
			error = (*(fnhandler.ua_default_for_property))(
				(CSS_PROP_FONT_FAMILY as u32), hint);
		    match error {
		        CSS_OK=>{},
		        _=> return error
		    }
		}
	}

	css__set_font_family_from_hint(hint, state.computed);
	CSS_OK
}

pub fn css__compose_font_family(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	//lwc_string **names = NULL;
	let mut (ftype,ffamily) = css_computed_font_family(child);

	if (ftype == (CSS_FONT_FAMILY_INHERIT as u8) || !mut_ptr_eq(result,child)) {

		if ( ftype == (CSS_FONT_FAMILY_INHERIT as u8) ) {
			let mut (ftype2,ffamily2) = css_computed_font_family(parent);
			set_font_family(result, ftype2, ffamily2);
		}
		else {
			set_font_family(result, ftype, ffamily);
		}
	}

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// font_size
///////////////////////////////////////////////////////////////////
pub fn css__cascade_font_size(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_FONT_SIZE_INHERIT as u16;
	let mut size : i32 = 0;
	let mut unit : u32 = UNIT_PX as u32;

	if (isInherit(opv) == false) {
		match getValue(opv) {
			FONT_SIZE_DIMENSION => {
				value = (CSS_FONT_SIZE_DIMENSION as u16);

				size = peek_bytecode(style) as i32;
				advance_bytecode(style);

				unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			FONT_SIZE_XX_SMALL => {
				value = (CSS_FONT_SIZE_XX_SMALL as u16);
			},
			FONT_SIZE_X_SMALL => {
				value = (CSS_FONT_SIZE_X_SMALL as u16);
			},
			FONT_SIZE_SMALL => {
				value = (CSS_FONT_SIZE_SMALL as u16);
			},
			FONT_SIZE_MEDIUM => {
				value = (CSS_FONT_SIZE_MEDIUM as u16);
			},
			FONT_SIZE_LARGE => {
				value = (CSS_FONT_SIZE_LARGE as u16);
			},
			FONT_SIZE_X_LARGE => {
				value = (CSS_FONT_SIZE_X_LARGE as u16);
			},
			FONT_SIZE_XX_LARGE => {
				value = (CSS_FONT_SIZE_XX_LARGE as u16);
			},
			FONT_SIZE_LARGER => {
				value = (CSS_FONT_SIZE_LARGER as u16);
			},
			FONT_SIZE_SMALLER => {
				value = (CSS_FONT_SIZE_SMALLER as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_font_size(state.computed, (value as u8), size, css__to_css_unit(unit) );
	}

	CSS_OK
}

pub fn css__set_font_size_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_font_size(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_font_size(state:@mut css_select_state) -> css_error {

	set_font_size(state.computed, (CSS_FONT_SIZE_MEDIUM as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_font_size(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_font_size(child);

	if (ftype == (CSS_FONT_SIZE_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_font_size(parent);
		set_font_size(result,
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_font_size(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// font_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_font_style(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_FONT_STYLE_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			FONT_STYLE_NORMAL => {
				value = ( CSS_FONT_STYLE_NORMAL as u16);
			},
			FONT_STYLE_ITALIC => {
				value = ( CSS_FONT_STYLE_ITALIC as u16);
			},
			FONT_STYLE_OBLIQUE => {
				value = ( CSS_FONT_STYLE_OBLIQUE as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_font_style(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_font_style_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_font_style(style, hint.status);
	CSS_OK
}

pub fn css__initial_font_style(state:@mut css_select_state) -> css_error {


	set_font_style(state.computed, (CSS_FONT_STYLE_NORMAL as u8) );
	CSS_OK
}

pub fn css__compose_font_style(parent:@mut css_computed_style,
							child:@mut css_computed_style,
							result:@mut css_computed_style
							) -> css_error {

	let mut ftype = css_computed_font_style(child);

	if (ftype == (CSS_FONT_STYLE_INHERIT as u8) ) {
		ftype = css_computed_font_style(parent);
	}

	set_font_style(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////


// font_variant
///////////////////////////////////////////////////////////////////
pub fn css__cascade_font_variant(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = (CSS_FONT_VARIANT_INHERIT as u16);

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			FONT_VARIANT_NORMAL => {
				value = ( CSS_FONT_VARIANT_NORMAL as u16);
			},
			FONT_VARIANT_SMALL_CAPS => {
				value = ( CSS_FONT_VARIANT_SMALL_CAPS as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_font_variant(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_font_variant_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_font_variant(style, hint.status);
	CSS_OK
}

pub fn css__initial_font_variant(state:@mut css_select_state) -> css_error {


	set_font_variant(state.computed, (CSS_FONT_VARIANT_NORMAL as u8) );
	CSS_OK
}

pub fn css__compose_font_variant(parent:@mut css_computed_style,
							child:@mut css_computed_style,
							result:@mut css_computed_style
							) -> css_error {

	let mut ftype = css_computed_font_variant(child);

	if (ftype == (CSS_FONT_VARIANT_INHERIT as u8) ) {
		ftype = css_computed_font_variant(parent);
	}

	set_font_variant(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////


// font_weight
///////////////////////////////////////////////////////////////////
pub fn css__cascade_font_weight(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_FONT_WEIGHT_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			FONT_WEIGHT_NORMAL => {
				value = (CSS_FONT_WEIGHT_NORMAL as u16);
			},
			FONT_WEIGHT_BOLD => {
				value = (CSS_FONT_WEIGHT_BOLD as u16);
			},
			FONT_WEIGHT_BOLDER => {
				value = (CSS_FONT_WEIGHT_BOLDER as u16);
			},
			FONT_WEIGHT_LIGHTER => {
				value = (CSS_FONT_WEIGHT_LIGHTER as u16);
			},
			FONT_WEIGHT_100 => {
				value = (CSS_FONT_WEIGHT_100 as u16);
			},
			FONT_WEIGHT_200 => {
				value = (CSS_FONT_WEIGHT_200 as u16);
			},
			FONT_WEIGHT_300 => {
				value = (CSS_FONT_WEIGHT_300 as u16);
			},
			FONT_WEIGHT_400 => {
				value = (CSS_FONT_WEIGHT_400 as u16);
			},
			FONT_WEIGHT_500 => {
				value = (CSS_FONT_WEIGHT_500 as u16);
			},
			FONT_WEIGHT_600 => {
				value = (CSS_FONT_WEIGHT_600 as u16);
			},
			FONT_WEIGHT_700 => {
				value = (CSS_FONT_WEIGHT_700 as u16);
			},
			FONT_WEIGHT_800 => {
				value = (CSS_FONT_WEIGHT_800 as u16);
			},
			FONT_WEIGHT_900 => {
				value = (CSS_FONT_WEIGHT_900 as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_font_weight(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_font_weight_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_font_weight(style, hint.status);
	CSS_OK
}

pub fn css__initial_font_weight(state:@mut css_select_state) -> css_error {


	set_font_weight(state.computed, (CSS_FONT_WEIGHT_NORMAL as u8) );
	CSS_OK
}

pub fn css__compose_font_weight(parent:@mut css_computed_style,
							child:@mut css_computed_style,
							result:@mut css_computed_style
							) -> css_error {

	let mut ftype = css_computed_font_weight(child);

	if (ftype == (CSS_FONT_WEIGHT_INHERIT as u8) ) {
		ftype = css_computed_font_weight(parent);
	}

	set_font_weight(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// height
///////////////////////////////////////////////////////////////////
pub fn css__cascade_height(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_height);
}

pub fn css__set_height_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_height(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_height(state:@mut css_select_state) -> css_error {

	set_height(state.computed, (CSS_HEIGHT_AUTO as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_height(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_height(child);

	if (ftype == (CSS_HEIGHT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_height(parent);
		set_height(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_height(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// left
///////////////////////////////////////////////////////////////////
pub fn css__cascade_left(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_left);
}

pub fn css__set_left_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_left(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_left(state:@mut css_select_state) -> css_error {

	set_left(state.computed, (CSS_LEFT_AUTO as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_left(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_left(child);

	if (ftype == (CSS_LEFT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_left(parent);
		set_left(result, 
				ftype2, 
				olength2.get_or_default( olength.get_or_default(0) ), 
				ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_left(result, 
				ftype, 
				olength.get_or_default(0), 
				ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// letter-spacing
///////////////////////////////////////////////////////////////////
pub fn css__cascade_letter_spacing(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_normal(opv, style, state, @set_letter_spacing);
}

pub fn css__set_letter_spacing_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_letter_spacing(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_letter_spacing(state:@mut css_select_state) -> css_error {

	set_letter_spacing(state.computed, (CSS_LETTER_SPACING_NORMAL as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_letter_spacing(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_letter_spacing(child);

	if (  (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_LETTER_SPACING_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

			if ( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_LETTER_SPACING_INHERIT as u8) ) {

				let mut (ftype2,olength2,ounit2) = css_computed_letter_spacing(parent);
				set_letter_spacing(result, 
								ftype2, 
								olength2.get_or_default( olength.get_or_default(0) ), 
								ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
			}
			else {

				set_letter_spacing(result, 
									ftype, 
									olength.get_or_default(0), 
									ounit.get_or_default(CSS_UNIT_PX));
			}
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// line-height
///////////////////////////////////////////////////////////////////
pub fn css__cascade_line_height(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_LINE_HEIGHT_INHERIT as u16;
	let mut val : i32 = 0;
	let mut unit : u32 = UNIT_PX as u32;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			LINE_HEIGHT_NUMBER => {
				value = CSS_LINE_HEIGHT_NUMBER as u16;
				val = peek_bytecode(style) as i32;
				advance_bytecode(style);
			},
			LINE_HEIGHT_DIMENSION => {
				value = CSS_LINE_HEIGHT_DIMENSION as u16;
				val = peek_bytecode(style) as i32;
				advance_bytecode(style);
				unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			LINE_HEIGHT_NORMAL => {
				value = CSS_LINE_HEIGHT_NORMAL as u16;
			},
			_=>{} 
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_line_height(state.computed, (value as u8) , val, css__to_css_unit(unit) );
	}

	CSS_OK
}

pub fn css__set_line_height_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_line_height(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_line_height(state:@mut css_select_state) -> css_error {

	set_line_height(state.computed, (CSS_LINE_HEIGHT_NORMAL as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_line_height(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_line_height(child);

	if (ftype == (CSS_LINE_HEIGHT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_line_height(parent);
		set_line_height(result, 
						ftype2, 
						olength2.get_or_default( olength.get_or_default(0) ), 
						ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_line_height(result, 
						ftype, 
						olength.get_or_default(0), 
						ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// line_style_image
///////////////////////////////////////////////////////////////////
pub fn css__cascade_list_style_image(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return  css__cascade_uri_none(opv, style, state, Some(@set_list_style_image) );
}

pub fn css__set_list_style_image_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		STRING=>{
			match hint.string {
				Some(copy x)=>{
					set_list_style_image(style, hint.status, x);
				},
				None=>{
					set_list_style_image(style, hint.status, ~"");
				}
			}
			hint.string = None ;
			CSS_OK
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_list_style_image(state:@mut css_select_state) -> css_error {

	set_list_style_image(state.computed, 
			(CSS_LIST_STYLE_IMAGE_NONE as u8) , ~"" );
	CSS_OK
}

pub fn css__compose_list_style_image(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,url) = css_computed_list_style_image(child);

	if (ftype == (CSS_LIST_STYLE_IMAGE_INHERIT as u8) ) {
		let mut (ftype2,url2) = css_computed_list_style_image(parent);
		set_list_style_image(result, ftype2, url2);
		CSS_OK
	}
	else {
		set_list_style_image(result, ftype, url);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// list_style_position
///////////////////////////////////////////////////////////////////
pub fn css__cascade_list_style_position(opv:u32, _:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	let mut value = CSS_LIST_STYLE_POSITION_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			LIST_STYLE_POSITION_INSIDE => {
				value = CSS_LIST_STYLE_POSITION_INSIDE as u16;
			},
			LIST_STYLE_POSITION_OUTSIDE => {
				value = CSS_LIST_STYLE_POSITION_OUTSIDE as u16;
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_list_style_position(state.computed, (value as u8) );
	}
	CSS_OK
}

pub fn css__set_list_style_position_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_list_style_position(style, hint.status);
	CSS_OK
}

pub fn css__initial_list_style_position(state:@mut css_select_state) -> css_error {

	set_list_style_position(state.computed, 
			(CSS_LIST_STYLE_POSITION_OUTSIDE as u8) );
	CSS_OK
}

pub fn css__compose_list_style_position(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_list_style_position(child);

	if (ftype == (CSS_LIST_STYLE_POSITION_INHERIT as u8) ) {
		ftype = css_computed_list_style_position(parent);
		
		set_list_style_position(result, ftype);
		CSS_OK
	}
	else {
		set_list_style_position(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////



// list_style_type
///////////////////////////////////////////////////////////////////
pub fn css__cascade_list_style_type(opv:u32, _:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	let mut value = CSS_LIST_STYLE_TYPE_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			LIST_STYLE_TYPE_DISC => {
				value = ( CSS_LIST_STYLE_TYPE_DISC as u16) ;
			},
			LIST_STYLE_TYPE_CIRCLE => {
				value = ( CSS_LIST_STYLE_TYPE_CIRCLE as u16) ;
			},
			LIST_STYLE_TYPE_SQUARE => {
				value = ( CSS_LIST_STYLE_TYPE_SQUARE as u16) ;
			},
			LIST_STYLE_TYPE_DECIMAL => {
				value = ( CSS_LIST_STYLE_TYPE_DECIMAL as u16) ;
			},
			LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO => {
				value = ( CSS_LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as u16) ;
			},
			LIST_STYLE_TYPE_LOWER_ROMAN => {
				value = ( CSS_LIST_STYLE_TYPE_LOWER_ROMAN as u16) ;
			},
			LIST_STYLE_TYPE_UPPER_ROMAN => {
				value = ( CSS_LIST_STYLE_TYPE_UPPER_ROMAN as u16) ;
			},
			LIST_STYLE_TYPE_LOWER_GREEK => {
				value = ( CSS_LIST_STYLE_TYPE_LOWER_GREEK as u16) ;
			},
			LIST_STYLE_TYPE_LOWER_LATIN => {
				value = ( CSS_LIST_STYLE_TYPE_LOWER_LATIN as u16) ;
			},
			LIST_STYLE_TYPE_UPPER_LATIN => {
				value = ( CSS_LIST_STYLE_TYPE_UPPER_LATIN as u16) ;
			},
			LIST_STYLE_TYPE_ARMENIAN => {
				value = ( CSS_LIST_STYLE_TYPE_ARMENIAN as u16) ;
			},
			LIST_STYLE_TYPE_GEORGIAN => {
				value = ( CSS_LIST_STYLE_TYPE_GEORGIAN as u16) ;
			},
			LIST_STYLE_TYPE_LOWER_ALPHA => {
				value = ( CSS_LIST_STYLE_TYPE_LOWER_ALPHA as u16) ;
			},
			LIST_STYLE_TYPE_UPPER_ALPHA => {
				value = ( CSS_LIST_STYLE_TYPE_UPPER_ALPHA as u16) ;
			},
			LIST_STYLE_TYPE_NONE => {
				value = ( CSS_LIST_STYLE_TYPE_NONE as u16) ;
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16) , isImportant(opv), state,
			isInherit(opv))) {
		set_list_style_type(state.computed, (value as u8) );
	}
	CSS_OK
}

pub fn css__set_list_style_type_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_list_style_type(style, hint.status);
	CSS_OK
}

pub fn css__initial_list_style_type(state:@mut css_select_state) -> css_error {

	set_list_style_type(state.computed, (CSS_LIST_STYLE_TYPE_DISC as u8) );
	CSS_OK
}

pub fn css__compose_list_style_type(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_list_style_type(child);

	if (ftype == (CSS_LIST_STYLE_TYPE_INHERIT as u8) ) {
		ftype = css_computed_list_style_type(parent);
		
		set_list_style_type(result, ftype);
		CSS_OK
	}
	else {
		set_list_style_type(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// margin-bottom
///////////////////////////////////////////////////////////////////
pub fn css__cascade_margin_bottom(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_margin_bottom);
}

pub fn css__set_margin_bottom_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_margin_bottom(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_margin_bottom(state:@mut css_select_state) -> css_error {

	set_margin_bottom(state.computed, (CSS_MARGIN_SET as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_margin_bottom(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_margin_bottom(child);

	if (ftype == (CSS_MARGIN_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_margin_bottom(parent);
		set_margin_bottom(result, 
						ftype2, 
						olength2.get_or_default( olength.get_or_default(0) ), 
						ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_margin_bottom(result, 
						ftype, 
						olength.get_or_default(0), 
						ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// margin-left
///////////////////////////////////////////////////////////////////
pub fn css__cascade_margin_left(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_margin_left);
}

pub fn css__set_margin_left_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_margin_left(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_margin_left(state:@mut css_select_state) -> css_error {

	set_margin_left(state.computed, (CSS_MARGIN_SET as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_margin_left(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_margin_left(child);

	if (ftype == (CSS_MARGIN_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_margin_left(parent);
		set_margin_left(result, 
						ftype2, 
						olength2.get_or_default( olength.get_or_default(0) ), 
						ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_margin_left(result, 
						ftype, 
						olength.get_or_default(0), 
						ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////


// margin-right
///////////////////////////////////////////////////////////////////
pub fn css__cascade_margin_right(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_margin_right);
}

pub fn css__set_margin_right_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_margin_right(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_margin_right(state:@mut css_select_state) -> css_error {

	set_margin_right(state.computed, (CSS_MARGIN_SET as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_margin_right(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_margin_right(child);

	if (ftype == (CSS_MARGIN_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_margin_right(parent);
		set_margin_right(result, 
						ftype2, 
						olength2.get_or_default( olength.get_or_default(0) ), 
						ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_margin_right(result, 
						ftype, 
						olength.get_or_default(0), 
						ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// margin-top
///////////////////////////////////////////////////////////////////
pub fn css__cascade_margin_top(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_margin_top);
}

pub fn css__set_margin_top_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_margin_top(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_margin_top(state:@mut css_select_state) -> css_error {

	set_margin_top(state.computed, (CSS_MARGIN_SET as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_margin_top(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_margin_top(child);

	if (ftype == (CSS_MARGIN_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_margin_top(parent);
		set_margin_top(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_margin_top(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// max_height
///////////////////////////////////////////////////////////////////
pub fn css__cascade_max_height(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_none(opv, style, state, @set_max_height);
}

pub fn css__set_max_height_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_max_height(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_max_height(state:@mut css_select_state) -> css_error {

	set_max_height(state.computed, (CSS_MAX_HEIGHT_NONE as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_max_height(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_max_height(child);

	if (ftype == (CSS_MAX_HEIGHT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_max_height(parent);
		set_max_height(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_max_height(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// max_width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_max_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_none(opv, style, state, @set_max_width);
}

pub fn css__set_max_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_max_width(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_max_width(state:@mut css_select_state) -> css_error {

	set_max_width(state.computed, (CSS_MAX_WIDTH_NONE as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_max_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_max_width(child);

	if (ftype == (CSS_MAX_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_max_width(parent);
		set_max_width(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_max_width(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// min-height
///////////////////////////////////////////////////////////////////
pub fn css__cascade_min_height(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length(opv, style, state, Some(@set_min_height) );
}

pub fn css__set_min_height_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_min_height(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_min_height(state:@mut css_select_state) -> css_error {

	set_min_height(state.computed, (CSS_MIN_HEIGHT_SET as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_min_height(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_min_height(child);

	if (ftype == (CSS_MIN_HEIGHT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_min_height(parent);
		set_min_height(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_min_height(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// min-width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_min_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length(opv, style, state, Some(@set_min_width) );
}

pub fn css__set_min_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_min_width(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_min_width(state:@mut css_select_state) -> css_error {

	set_min_width(state.computed, (CSS_MIN_WIDTH_SET as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_min_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_min_width(child);

	if (ftype == (CSS_MIN_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_min_width(parent);
		set_min_width(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_min_width(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// opacity
///////////////////////////////////////////////////////////////////
pub fn css__cascade_opacity(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_OPACITY_INHERIT as u16;
	let mut opacity : i32 = 0;

	if (isInherit(opv) == false) {
		value = CSS_Z_INDEX_SET as u16;

		opacity = peek_bytecode(style) as i32;
		advance_bytecode(style);
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_opacity(state.computed, (value as u8), opacity);
	}
	CSS_OK
}

pub fn css__set_opacity_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		FIXED=>{
			match hint.fixed {
				Some(x)=>{
					set_opacity(style, hint.status, x);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_opacity(state:@mut css_select_state) -> css_error {

	set_opacity(state.computed, (CSS_OPACITY_SET as u8), css_int_to_fixed(1));
	CSS_OK
}

pub fn css__compose_opacity(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength) = css_computed_opacity(child);

	if (ftype == (CSS_OPACITY_INHERIT as u8) ) {
		let mut (ftype2,olength2) = css_computed_opacity(parent);
		set_opacity(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ) );
		CSS_OK
	}
	else {
		set_opacity(result, 
					ftype, 
					olength.get_or_default(0) );
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// Orphans
///////////////////////////////////////////////////////////////////
pub fn css__cascade_orphans(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error
{
	return css__cascade_number(opv, style, state, None );
}

pub fn css__set_orphans_from_hint(_: @mut css_hint, 
		_:@mut css_computed_style) -> css_error {

	CSS_OK
}

pub fn css__initial_orphans(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_orphans(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// outline_color
///////////////////////////////////////////////////////////////////
pub fn css__cascade_outline_color(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_OUTLINE_COLOR_INHERIT as u16;
	let mut color : u32 = 0;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			OUTLINE_COLOR_TRANSPARENT => {
				value = ( CSS_OUTLINE_COLOR_COLOR as u16);
			},
			OUTLINE_COLOR_CURRENT_COLOR => {
				value = ( CSS_OUTLINE_COLOR_CURRENT_COLOR as u16);
			},
			OUTLINE_COLOR_SET => {
				value = ( CSS_OUTLINE_COLOR_COLOR as u16);
				color = peek_bytecode(style);
				advance_bytecode(style);
			},
			OUTLINE_COLOR_INVERT => {
				value = ( CSS_OUTLINE_COLOR_INVERT as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing((getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_outline_color(state.computed, (value as u8), color);
	}

	CSS_OK
}

pub fn css__set_outline_color_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COLOR=>{
			match hint.color {
				Some(x)=>{
					set_outline_color(style, hint.status, x);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_outline_color(state:@mut css_select_state) -> css_error {

	set_outline_color(state.computed, (CSS_OUTLINE_COLOR_INVERT as u8), 0);
	CSS_OK
}

pub fn css__compose_outline_color(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,ocolor) = css_computed_outline_color(child);

	if (  (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_OUTLINE_COLOR_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

			if ( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_OUTLINE_COLOR_INHERIT as u8) ) {

				let mut (ftype2,ocolor2) = css_computed_outline_color(parent);
				set_outline_color(result, 
								ftype2, 
								ocolor2.get_or_default( ocolor.get_or_default(0) ) );
			}
			else {
				set_outline_color(result, ftype, ocolor.get_or_default(0));
			}
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// outline_style
///////////////////////////////////////////////////////////////////
pub fn css__cascade_outline_style(opv:u32, style:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	return css__cascade_border_style(opv, style, state, @set_outline_style);
}

pub fn css__set_outline_style_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_outline_style(style, hint.status);
	CSS_OK
}

pub fn css__initial_outline_style(state:@mut css_select_state) -> css_error {

	set_outline_style(state.computed, (CSS_OUTLINE_STYLE_NONE as u8) );
	CSS_OK
}

pub fn css__compose_outline_style(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_outline_style(child);

	if (ftype == (CSS_OUTLINE_STYLE_INHERIT as u8) ) {
		ftype = css_computed_outline_style(parent);
		
		set_outline_style(result, ftype);
		CSS_OK
	}
	else {
		set_outline_style(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// outline-width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_outline_width(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_border_width(opv, style, state, @set_outline_width);
}

pub fn css__set_outline_width_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_outline_width(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_outline_width(state:@mut css_select_state) -> css_error {

	set_outline_width(state.computed, (CSS_OUTLINE_WIDTH_MEDIUM as u8),
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_outline_width(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_outline_width(child);

	if ( (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_OUTLINE_WIDTH_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

		if( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_OUTLINE_WIDTH_INHERIT as u8) ) {

			let mut (ftype2,olength2,ounit2) = css_computed_outline_width(parent);
			set_outline_width(result, 
							ftype2, 
							olength2.get_or_default( olength.get_or_default(0) ), 
							ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		}
		else {
			set_outline_width(result, 
							ftype, 
							olength.get_or_default(0), 
							ounit.get_or_default(CSS_UNIT_PX));
		}
	}
	CSS_OK	
}

///////////////////////////////////////////////////////////////////

// overflow
///////////////////////////////////////////////////////////////////
pub fn css__cascade_overflow(opv:u32, _:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	let mut value = CSS_OVERFLOW_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			OVERFLOW_VISIBLE => {
				value = (CSS_OVERFLOW_VISIBLE as u16);
			},
			OVERFLOW_HIDDEN => {
				value = (CSS_OVERFLOW_HIDDEN as u16);
			},
			OVERFLOW_SCROLL => {
				value = (CSS_OVERFLOW_SCROLL as u16);
			},
			OVERFLOW_AUTO => {
				value = (CSS_OVERFLOW_AUTO as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16) , isImportant(opv), state,
			isInherit(opv))) {
		set_overflow(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_overflow_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_overflow(style, hint.status);
	CSS_OK
}

pub fn css__initial_overflow(state:@mut css_select_state) -> css_error {

	set_overflow(state.computed, (CSS_OVERFLOW_VISIBLE as u8) );
	CSS_OK
}

pub fn css__compose_overflow(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_overflow(child);

	if (ftype == (CSS_OVERFLOW_INHERIT as u8) ) {
		ftype = css_computed_overflow(parent);
		
		set_overflow(result, ftype);
		CSS_OK
	}
	else {
		set_overflow(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// padding_bottom
///////////////////////////////////////////////////////////////////
pub fn css__cascade_padding_bottom(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length(opv, style, state, Some(@set_padding_bottom) );
}

pub fn css__set_padding_bottom_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_padding_bottom(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_padding_bottom(state:@mut css_select_state) -> css_error {

	set_padding_bottom(state.computed, (CSS_PADDING_SET as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_padding_bottom(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_padding_bottom(child);

	if (ftype == (CSS_PADDING_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_padding_bottom(parent);
		set_padding_bottom(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_padding_bottom(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// padding_left
///////////////////////////////////////////////////////////////////
pub fn css__cascade_padding_left(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length(opv, style, state, Some(@set_padding_left) );
}

pub fn css__set_padding_left_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_padding_left(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_padding_left(state:@mut css_select_state) -> css_error {

	set_padding_left(state.computed, (CSS_PADDING_SET as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_padding_left(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_padding_left(child);

	if (ftype == (CSS_PADDING_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_padding_left(parent);
		set_padding_left(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_padding_left(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// padding_right
///////////////////////////////////////////////////////////////////
pub fn css__cascade_padding_right(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length(opv, style, state, Some(@set_padding_right) );
}

pub fn css__set_padding_right_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_padding_right(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_padding_right(state:@mut css_select_state) -> css_error {

	set_padding_right(state.computed, (CSS_PADDING_SET as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_padding_right(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_padding_right(child);

	if (ftype == (CSS_PADDING_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_padding_right(parent);
		set_padding_right(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_padding_right(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// padding_top
///////////////////////////////////////////////////////////////////
pub fn css__cascade_padding_top(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length(opv, style, state, Some(@set_padding_top) );
}

pub fn css__set_padding_top_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_padding_top(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_padding_top(state:@mut css_select_state) -> css_error {

	set_padding_top(state.computed, (CSS_PADDING_SET as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_padding_top(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_padding_top(child);

	if (ftype == (CSS_PADDING_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_padding_top(parent);
		set_padding_top(result, 
					ftype2, 
					olength2.get_or_default( olength.get_or_default(0) ), 
					ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_padding_top(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// page_break_after
///////////////////////////////////////////////////////////////////
pub fn css__cascade_page_break_after(opv:u32, style:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	return css__cascade_page_break_after_before_inside(opv, style, state,
			Some(@set_page_break_after) );
}

pub fn css__set_page_break_after_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_page_break_after(style, hint.status);
	CSS_OK
}

pub fn css__initial_page_break_after(state:@mut css_select_state) -> css_error {

	set_page_break_after(state.computed, (CSS_PAGE_BREAK_AFTER_AUTO as u8) );
	CSS_OK
}

pub fn css__compose_page_break_after(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_page_break_after(child);

	if (ftype == (CSS_PAGE_BREAK_AFTER_INHERIT as u8) ) {
		ftype = css_computed_page_break_after(parent);
		
		set_page_break_after(result, ftype);
		CSS_OK
	}
	else {
		set_page_break_after(result, ftype);
		CSS_OK
	}
}
///////////////////////////////////////////////////////////////////

// page_break_before
///////////////////////////////////////////////////////////////////
pub fn css__cascade_page_break_before(opv:u32, style:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	return css__cascade_page_break_after_before_inside(opv, style, state, 
			Some(@set_page_break_before) );
}

pub fn css__set_page_break_before_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_page_break_before(style, hint.status);
	CSS_OK
}

pub fn css__initial_page_break_before(state:@mut css_select_state) -> css_error {

	set_page_break_before(state.computed, (CSS_PAGE_BREAK_BEFORE_AUTO as u8) );
	CSS_OK
}

pub fn css__compose_page_break_before(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_page_break_before(child);

	if (ftype == (CSS_PAGE_BREAK_BEFORE_INHERIT as u8) ) {
		ftype = css_computed_page_break_before(parent);
		
		set_page_break_before(result, ftype);
		CSS_OK
	}
	else {
		set_page_break_before(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// page_break_inside
///////////////////////////////////////////////////////////////////
pub fn css__cascade_page_break_inside(opv:u32, style:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	return css__cascade_page_break_after_before_inside(opv, style, state, 
			Some(@set_page_break_inside) );
}

pub fn css__set_page_break_inside_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_page_break_inside(style, hint.status);
	CSS_OK
}

pub fn css__initial_page_break_inside(state:@mut css_select_state) -> css_error {

	set_page_break_inside(state.computed, (CSS_PAGE_BREAK_INSIDE_AUTO as u8) );
	CSS_OK
}

pub fn css__compose_page_break_inside(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_page_break_inside(child);

	if (ftype == (CSS_PAGE_BREAK_INSIDE_INHERIT as u8) ) {
		ftype = css_computed_page_break_inside(parent);
		
		set_page_break_inside(result, ftype);
		CSS_OK
	}
	else {
		set_page_break_inside(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// pause_after
///////////////////////////////////////////////////////////////////
pub fn css__cascade_pause_after(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	css__cascade_length(opv, style, state, None)
}

pub fn css__set_pause_after_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_pause_after(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_pause_after(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// pause_before
///////////////////////////////////////////////////////////////////
pub fn css__cascade_pause_before(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	css__cascade_length(opv, style, state, None)
}

pub fn css__set_pause_before_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_pause_before(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_pause_before(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// pitch
///////////////////////////////////////////////////////////////////
pub fn css__cascade_pitch(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	// let mut freq : i32 = 0;
	// let mut unit : u32 = UNIT_HZ as u32;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			PITCH_FREQUENCY  => {
				// freq = peek_bytecode(style) as i32 ;
				advance_bytecode(style);
				// unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			PITCH_X_LOW  |
			PITCH_LOW  |
			PITCH_MEDIUM  |
			PITCH_HIGH  |
			PITCH_X_HIGH  => {
				/* \todo convert to public values */
			},
			_ =>{}
		}
	}

	//unit = css__to_css_unit(unit);

	if (css__outranks_existing( (getOpcode(opv) as u16) , isImportant(opv), state,
			isInherit(opv))) {
		/* \todo pitch */
	}

	CSS_OK
}

pub fn css__set_pitch_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

pub fn css__initial_pitch(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_pitch(_:@mut css_computed_style,
						_:@mut css_computed_style,
						_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// pitch_range
///////////////////////////////////////////////////////////////////
pub fn css__cascade_pitch_range(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	css__cascade_number(opv, style, state, None)
}

pub fn css__set_pitch_range_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_pitch_range(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_pitch_range(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// play_during
///////////////////////////////////////////////////////////////////
pub fn css__cascade_play_during(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	// let mut uri : ~str;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			PLAY_DURING_URI => {
				let (result, _) = style.sheet.get().css__stylesheet_string_get(peek_bytecode(style) as uint);
				match result {
					CSS_OK => {
						// uri = str_option.unwrap();
					},
					x => { 
						return x ;
					}
				}
				advance_bytecode(style);
			},
			PLAY_DURING_AUTO |
			PLAY_DURING_NONE => {
				/* \todo convert to public values */
			},
			_=>{}
		}

		/* \todo mix & repeat */
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		/* \todo play-during */
	}

	CSS_OK
}

pub fn css__set_play_during_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_play_during(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_play_during(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// position
///////////////////////////////////////////////////////////////////
pub fn css__cascade_position(opv:u32, _:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	let mut value = (CSS_POSITION_INHERIT  as u16) ;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			POSITION_STATIC => {
				value = ( CSS_POSITION_STATIC as u16) ;
			},
			POSITION_RELATIVE => {
				value = ( CSS_POSITION_RELATIVE as u16) ;
			},
			POSITION_ABSOLUTE => {
				value = ( CSS_POSITION_ABSOLUTE as u16) ;
			},
			POSITION_FIXED => {
				value = ( CSS_POSITION_FIXED as u16) ;
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_position(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_position_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_position(style, hint.status);
	CSS_OK
}

pub fn css__initial_position(state:@mut css_select_state) -> css_error {

	set_position(state.computed, (CSS_POSITION_STATIC as u8) );
	CSS_OK
}

pub fn css__compose_position(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_position(child);

	if (ftype == (CSS_POSITION_INHERIT as u8) ) {
		ftype = css_computed_position(parent);
		
		set_position(result, ftype);
		CSS_OK
	}
	else {
		set_position(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// quotes
///////////////////////////////////////////////////////////////////
pub fn css__cascade_quotes(opv:u32, style:@mut css_style, 
						state:@mut css_select_state) -> css_error {

	
	let mut value : u16 = CSS_QUOTES_INHERIT as u16;
	let mut quotes : ~[~str] = ~[] ;

	if (isInherit(opv) == false) {
		let mut v : u32 = getValue(opv) as u32 ;

		value = CSS_QUOTES_STRING as u16;

		while (v != (QUOTES_NONE as u32) ) {

			if style.sheet.is_none() {
				return CSS_BADPARM ;
			}

			let mut (result1,o_open)  = style.sheet.get().css__stylesheet_string_get( 
														peek_bytecode(style) as uint );
			advance_bytecode(style);

			let mut (result2,o_close) = style.sheet.get().css__stylesheet_string_get( 
														peek_bytecode(style) as uint );
			advance_bytecode(style);

			match result1 {
				CSS_OK=>{} ,
				x => { return x ; }
			}

			match result2 {
				CSS_OK=>{} ,
				x => { return x ; }
			}

			if o_open.is_none()  { return CSS_BADPARM ;}
			if o_close.is_none() { return CSS_BADPARM ;}

			quotes.push( o_open.unwrap()  );
			quotes.push( o_close.unwrap() );

			v = peek_bytecode(style);
			advance_bytecode(style);
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {

		set_quotes(state.computed, (value as u8), quotes);
	} 

	CSS_OK
}

pub fn css__set_quotes_from_hint(hint:@mut  css_hint, 
								style:@mut css_computed_style
								) -> css_error {

	match hint.hint_type {
		STRINGS_VECTOR => {
			match hint.strings {
				Some(copy x)=>{
					set_quotes(style, hint.status, x);
				},
				None=>{
					set_quotes(style, hint.status, ~[] );
				}
			} 
			hint.strings= None ;
			CSS_OK 
		},
		_ => {
			CSS_INVALID
		}
	}
}

pub fn css__initial_quotes(state:@mut css_select_state) -> css_error {

	let mut hint = @mut css_hint{
        hint_type:HINT_LENGTH,
        status:0,
        clip:None,
        content:None,
        counters:None,
        length:None,
        position:None,
        color:None,
        fixed:None,
        integer:None,
        string:None,
        strings:None
    };
	let mut error : css_error;

	match state.handler {
		None=> {
			return CSS_BADPARM ;
		},
		Some(fnhandler) => {
			error = (*(fnhandler.ua_default_for_property))(
				(CSS_PROP_QUOTES as u32), hint);
		    match error {
		        CSS_OK=>{},
		        _=> return error
		    }
		}
	}

	css__set_quotes_from_hint(hint, state.computed);
	CSS_OK
}

pub fn css__compose_quotes(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,quotes) = css_computed_quotes(child) ;

	if( (ftype == (CSS_QUOTES_INHERIT as u8) ) ||  !mut_ptr_eq(result,child) ) { 

		if ( ftype == (CSS_QUOTES_INHERIT as u8) ) {
			let mut (ftype2,quotes2) = css_computed_quotes(parent) ;

			set_quotes(result,ftype2,quotes2);
		}
		else {
			set_quotes(result,ftype,quotes) ;
		}

	}

	CSS_OK
}

///////////////////////////////////////////////////////////////////


// richness
///////////////////////////////////////////////////////////////////
pub fn css__cascade_richness(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	return css__cascade_number(opv, style, state, None);
}

pub fn css__set_richness_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_richness(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_richness(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// right
///////////////////////////////////////////////////////////////////
pub fn css__cascade_right(opv:u32, style:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	return css__cascade_length_auto(opv, style, state, @set_right);
}

pub fn css__set_right_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_right(style, hint.status, x.value , x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_right(state:@mut css_select_state) -> css_error {

	set_right(state.computed, (CSS_RIGHT_AUTO as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_right(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_right(child);

	if (ftype == (CSS_RIGHT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_right(parent);
		set_right(result, 
				ftype2, 
				olength2.get_or_default( olength.get_or_default(0) ), 
				ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_right(result, 
				ftype, 
				olength.get_or_default(0), 
				ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// speak
///////////////////////////////////////////////////////////////////
pub fn css__cascade_speak(opv:u32 , 
							_:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
				SPEAK_NORMAL	|
				SPEAK_NONE		|
				SPEAK_SPELL_OUT => {
				/* \todo convert to public values */
				},
			_ => {}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16) , isImportant(opv), state,
			isInherit(opv))) {
		/* \todo speak */
	}

	CSS_OK
}

pub fn css__set_speak_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_speak(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_speak(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// speak_header
///////////////////////////////////////////////////////////////////
pub fn css__cascade_speak_header(opv:u32 , 
								_:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			SPEAK_HEADER_ONCE |
			SPEAK_HEADER_ALWAYS => {
				/* \todo convert to public values */
			},
			_=>{} 
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		/* \todo speak-header */
	}

	CSS_OK
}

pub fn css__set_speak_header_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_speak_header(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_speak_header(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// speak_numeral
///////////////////////////////////////////////////////////////////
pub fn css__cascade_speak_numeral(opv:u32 , 
								_:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			SPEAK_NUMERAL_DIGITS |
			SPEAK_NUMERAL_CONTINUOUS => {
				/* \todo convert to public values */
			},
			_=>{} 
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		/* \todo speak-numeral */
	}

	CSS_OK
}

pub fn css__set_speak_numeral_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_speak_numeral(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_speak_numeral(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// speak_punctuation
///////////////////////////////////////////////////////////////////
pub fn css__cascade_speak_punctuation(opv:u32 , 
									_:@mut css_style ,
									state: @mut css_select_state 
									) -> css_error {

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			SPEAK_PUNCTUATION_CODE |
			SPEAK_PUNCTUATION_NONE => {
				/* \todo convert to public values */
			},
			_=>{} 
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		/* \todo speak-punctuation */
	}

	CSS_OK
}

pub fn css__set_speak_punctuation_from_hint(_: @mut css_hint, 
											_:@mut css_computed_style) 
											-> css_error {

	CSS_OK
}

pub fn css__initial_speak_punctuation(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_speak_punctuation(_:@mut css_computed_style,
									_:@mut css_computed_style,
									_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// speech_rate
///////////////////////////////////////////////////////////////////
pub fn css__cascade_speech_rate(opv:u32 , 
								style:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	// let mut rate : i32 = 0;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			SPEECH_RATE_SET => {
				// rate = peek_bytecode(style) as i32;
				advance_bytecode(style);
			},
			SPEECH_RATE_X_SLOW 	|
			SPEECH_RATE_SLOW 	|
			SPEECH_RATE_MEDIUM 	|
			SPEECH_RATE_FAST  	|
			SPEECH_RATE_X_FAST 	|
			SPEECH_RATE_FASTER 	|
			SPEECH_RATE_SLOWER => {
				/* \todo convert to public values */
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16) , isImportant(opv), state,
			isInherit(opv))) {
		/* \todo speech-rate */
	}

	CSS_OK
}

pub fn css__set_speech_rate_from_hint(_: @mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error {

	CSS_OK
}

pub fn css__initial_speech_rate(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_speech_rate(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// stress
///////////////////////////////////////////////////////////////////
pub fn css__cascade_stress(opv:u32 , 
							style:@mut css_style ,
							state: @mut css_select_state 
							) -> css_error {

	return css__cascade_number(opv, style, state, None);
}

pub fn css__set_stress_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_stress(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_stress(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// table_layout
///////////////////////////////////////////////////////////////////
pub fn css__cascade_table_layout(opv:u32, _:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	
	let mut value = CSS_TABLE_LAYOUT_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			TABLE_LAYOUT_AUTO => {
				value = CSS_TABLE_LAYOUT_AUTO as u16;
			},
			TABLE_LAYOUT_FIXED => {
				value = CSS_TABLE_LAYOUT_FIXED as u16;
			},
			_=>{}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_table_layout(state.computed, (value as u8) );
	}

	CSS_OK
}

pub fn css__set_table_layout_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_table_layout(style, hint.status);
	CSS_OK
}

pub fn css__initial_table_layout(state:@mut css_select_state) -> css_error {

	set_table_layout(state.computed, (CSS_TABLE_LAYOUT_AUTO as u8) );
	CSS_OK
}

pub fn css__compose_table_layout(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut ftype = css_computed_table_layout(child);

	if (ftype == (CSS_TABLE_LAYOUT_INHERIT as u8) ) {
		ftype = css_computed_table_layout(parent);
		
		set_table_layout(result, ftype);
		CSS_OK
	}
	else {
		set_table_layout(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// text-align
///////////////////////////////////////////////////////////////////
pub fn css__cascade_text_align(opv:u32, _:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	let mut value = CSS_TEXT_ALIGN_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			TEXT_ALIGN_LEFT => {
				value = (CSS_TEXT_ALIGN_LEFT as u16);
			},
			TEXT_ALIGN_RIGHT => {
				value = (CSS_TEXT_ALIGN_RIGHT as u16);
			},
			TEXT_ALIGN_CENTER => {
				value = (CSS_TEXT_ALIGN_CENTER as u16);
			},
			TEXT_ALIGN_JUSTIFY => {
				value = (CSS_TEXT_ALIGN_JUSTIFY as u16);
			},
			TEXT_ALIGN_LIBCSS_LEFT => {
				value = (CSS_TEXT_ALIGN_LIBCSS_LEFT as u16);
			},
			TEXT_ALIGN_LIBCSS_CENTER => {
				value = (CSS_TEXT_ALIGN_LIBCSS_CENTER as u16);
			},
			TEXT_ALIGN_LIBCSS_RIGHT => {
				value = (CSS_TEXT_ALIGN_LIBCSS_RIGHT as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_text_align(state.computed, value as u8);
	}


	CSS_OK
}

pub fn css__set_text_align_from_hint(hint:@mut  css_hint, 
									style:@mut css_computed_style
									) -> css_error {

	set_text_align(style, hint.status);
	CSS_OK
}

pub fn css__initial_text_align(state:@mut css_select_state) -> css_error {

	set_text_align(state.computed, (CSS_TEXT_ALIGN_DEFAULT as u8) );
	CSS_OK
}

pub fn css__compose_text_align(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut ftype = css_computed_text_align(child);

	if (ftype == (CSS_TEXT_ALIGN_INHERIT as u8) ) {
		ftype = css_computed_text_align(parent);
	}
	else if(ftype == (CSS_TEXT_ALIGN_INHERIT_IF_NON_MAGIC as u8) ) {
		/* This is purely for the benefit of HTML tables */
		ftype = css_computed_text_align(parent);

		/* If the parent's text-align is a magical one, 
		 * then reset to the default value. Otherwise, 
		 * inherit as normal. */
		if (ftype == (CSS_TEXT_ALIGN_LIBCSS_LEFT as u8) ||
				ftype == (CSS_TEXT_ALIGN_LIBCSS_CENTER as u8) ||
				ftype == (CSS_TEXT_ALIGN_LIBCSS_RIGHT as u8)) {
			ftype = CSS_TEXT_ALIGN_DEFAULT as u8 ;
		}
	}
		
	set_text_align(result, ftype);
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// text_decoration
///////////////////////////////////////////////////////////////////
pub fn css__cascade_text_decoration(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_TEXT_DECORATION_INHERIT as u16;

	if (isInherit(opv) == false) {
		if (getValue(opv) == TEXT_DECORATION_NONE) {
			value = ( CSS_TEXT_DECORATION_NONE as u16);
		} 
		else {
			if(value == 0) {
				return CSS_BADPARM ;
			}

			if ( (getValue(opv) & TEXT_DECORATION_UNDERLINE) == 0 ) {
				value |= ( CSS_TEXT_DECORATION_UNDERLINE as u16);
			}
			if ( (getValue(opv) & TEXT_DECORATION_OVERLINE) == 0 ) {
				value |= ( CSS_TEXT_DECORATION_OVERLINE as u16);
			}
			if ( (getValue(opv) & TEXT_DECORATION_LINE_THROUGH) == 0 ) {
				value |= ( CSS_TEXT_DECORATION_LINE_THROUGH as u16);
			}
			if ( (getValue(opv) & TEXT_DECORATION_BLINK) == 0 ) {
				value |= ( CSS_TEXT_DECORATION_BLINK as u16);
			}
		}
	}

	if (css__outranks_existing( (getOpcode(opv) as u16), isImportant(opv), state,
			isInherit(opv))) {
		set_text_decoration(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_text_decoration_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_text_decoration(style, hint.status);
	CSS_OK
}

pub fn css__initial_text_decoration(state:@mut css_select_state) -> css_error {

	set_text_decoration(state.computed, (CSS_TEXT_DECORATION_NONE as u8) );
	CSS_OK
}

pub fn css__compose_text_decoration(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_text_decoration(child);

	if (ftype == (CSS_TEXT_DECORATION_INHERIT as u8) ) {
		ftype = css_computed_text_decoration(parent);
		
		set_text_decoration(result, ftype);
		CSS_OK
	}
	else {
		set_text_decoration(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// text_indent
///////////////////////////////////////////////////////////////////
pub fn css__cascade_text_indent(opv:u32, style:@mut css_style, 
										state:@mut css_select_state) -> css_error {

	
	return css__cascade_length(opv, style, state, Some(@set_text_indent) );
}

pub fn css__set_text_indent_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_text_indent(style, hint.status, x.value , x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_text_indent(state:@mut css_select_state) -> css_error {

	set_text_indent(state.computed, (CSS_TEXT_INDENT_SET as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_text_indent(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_text_indent(child);

	if (ftype == (CSS_TEXT_INDENT_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_text_indent(parent);
		set_text_indent(result, 
						ftype2, 
						olength2.get_or_default( olength.get_or_default(0) ), 
						ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_text_indent(result, 
					ftype, 
					olength.get_or_default(0), 
					ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// text_transform
///////////////////////////////////////////////////////////////////
pub fn css__cascade_text_transform(opv:u32, _:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	let mut value = CSS_TEXT_TRANSFORM_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			TEXT_TRANSFORM_CAPITALIZE => {
				value = (CSS_TEXT_TRANSFORM_CAPITALIZE as u16);
			},
			TEXT_TRANSFORM_UPPERCASE => {
				value = (CSS_TEXT_TRANSFORM_UPPERCASE as u16);
			},
			TEXT_TRANSFORM_LOWERCASE => {
				value = (CSS_TEXT_TRANSFORM_LOWERCASE as u16);
			},
			TEXT_TRANSFORM_NONE => {
				value = (CSS_TEXT_TRANSFORM_NONE as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_text_transform(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_text_transform_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	set_text_transform(style, hint.status);
	CSS_OK
}

pub fn css__initial_text_transform(state:@mut css_select_state) -> css_error {

	set_text_transform(state.computed, (CSS_TEXT_TRANSFORM_NONE as u8) );
	CSS_OK
}

pub fn css__compose_text_transform(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut ftype = css_computed_text_transform(child);

	if (ftype == (CSS_TEXT_TRANSFORM_INHERIT as u8) ) {
		ftype = css_computed_text_transform(parent);
		
		set_text_transform(result, ftype);
		CSS_OK
	}
	else {
		set_text_transform(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// top
///////////////////////////////////////////////////////////////////
pub fn css__cascade_top(opv:u32, style:@mut css_style, 
						state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_top) ;
}

pub fn css__set_top_from_hint(hint:@mut  css_hint, 
							style:@mut css_computed_style
							) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_top(style, hint.status, x.value , x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_top(state:@mut css_select_state) -> css_error {

	set_top(state.computed, (CSS_TOP_AUTO as u8), 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_top(parent:@mut css_computed_style,
						child:@mut css_computed_style,
						result:@mut css_computed_style
						) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_top(child);

	if (ftype == (CSS_TOP_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_top(parent);
		set_top(result, 
				ftype2, 
				olength2.get_or_default( olength.get_or_default(0) ), 
				ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_top(result, 
				ftype, 
				olength.get_or_default(0), 
				ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// unicode_bidi
///////////////////////////////////////////////////////////////////
pub fn css__cascade_unicode_bidi(opv:u32, _:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	let mut value = CSS_UNICODE_BIDI_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			UNICODE_BIDI_NORMAL => {
				value = (CSS_UNICODE_BIDI_NORMAL as u16);
			},
			UNICODE_BIDI_EMBED => {
				value = (CSS_UNICODE_BIDI_EMBED as u16);
			},
			UNICODE_BIDI_BIDI_OVERRIDE => {
				value = (CSS_UNICODE_BIDI_BIDI_OVERRIDE as u16);
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_unicode_bidi(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_unicode_bidi_from_hint(hint:@mut  css_hint, 
									style:@mut css_computed_style
									) -> css_error {

	set_unicode_bidi(style, hint.status);
	CSS_OK
}

pub fn css__initial_unicode_bidi(state:@mut css_select_state) -> css_error {

	set_unicode_bidi(state.computed, (CSS_UNICODE_BIDI_NORMAL as u8) );
	CSS_OK
}

pub fn css__compose_unicode_bidi(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut ftype = css_computed_unicode_bidi(child);

	if (ftype == (CSS_UNICODE_BIDI_INHERIT as u8) ) {
		ftype = css_computed_unicode_bidi(parent);
		
		set_unicode_bidi(result, ftype);
		CSS_OK
	}
	else {
		set_unicode_bidi(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// vertical_align
///////////////////////////////////////////////////////////////////
pub fn css__cascade_vertical_align(opv:u32, style:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	let mut value : u16 = CSS_VERTICAL_ALIGN_INHERIT as u16;
	let mut length : i32 = 0;
	let mut unit : u32 = UNIT_PX as u32;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			VERTICAL_ALIGN_SET => {
				value = (CSS_VERTICAL_ALIGN_SET as u16) ;

				length = peek_bytecode(style) as i32 ;
				advance_bytecode(style);
				unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			VERTICAL_ALIGN_BASELINE => {
				value = (CSS_VERTICAL_ALIGN_BASELINE as u16) ;
			},
			VERTICAL_ALIGN_SUB => {
				value = (CSS_VERTICAL_ALIGN_SUB as u16) ;
			},
			VERTICAL_ALIGN_SUPER => {
				value = (CSS_VERTICAL_ALIGN_SUPER as u16) ;
			},
			VERTICAL_ALIGN_TOP => {
				value = (CSS_VERTICAL_ALIGN_TOP as u16) ;
			},
			VERTICAL_ALIGN_TEXT_TOP => {
				value = (CSS_VERTICAL_ALIGN_TEXT_TOP as u16) ;
			},
			VERTICAL_ALIGN_MIDDLE => {
				value = (CSS_VERTICAL_ALIGN_MIDDLE as u16) ;
			},
			VERTICAL_ALIGN_BOTTOM => {
				value = (CSS_VERTICAL_ALIGN_BOTTOM as u16) ;
			},
			VERTICAL_ALIGN_TEXT_BOTTOM => {
				value = (CSS_VERTICAL_ALIGN_TEXT_BOTTOM as u16) ;
			},
			_ =>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_vertical_align(state.computed, value as u8, length, css__to_css_unit(unit) );
	}

	CSS_OK
}

pub fn css__set_vertical_align_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_vertical_align(style, hint.status, x.value , x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_vertical_align(state:@mut css_select_state) -> css_error {

	set_vertical_align(state.computed, (CSS_VERTICAL_ALIGN_BASELINE as u8),
					 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_vertical_align(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_vertical_align(child);

	if (ftype == (CSS_VERTICAL_ALIGN_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_vertical_align(parent);
		set_vertical_align(result, 
						ftype2, 
						olength2.get_or_default( olength.get_or_default(0) ), 
						ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_vertical_align(result, 
						ftype, 
						olength.get_or_default(0), 
						ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// visibility
///////////////////////////////////////////////////////////////////
pub fn css__cascade_visibility(opv:u32, _:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	let mut value = CSS_VISIBILITY_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			VISIBILITY_VISIBLE => {
				value = (CSS_VISIBILITY_VISIBLE as u16) ;
			},
			VISIBILITY_HIDDEN => {
				value = (CSS_VISIBILITY_HIDDEN as u16) ;
			},
			VISIBILITY_COLLAPSE => {
				value = (CSS_VISIBILITY_COLLAPSE as u16) ;
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_visibility(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_visibility_from_hint(hint:@mut  css_hint, 
									style:@mut css_computed_style
									) -> css_error {

	set_visibility(style, hint.status);
	CSS_OK
}

pub fn css__initial_visibility(state:@mut css_select_state) -> css_error {

	set_visibility(state.computed, (CSS_VISIBILITY_VISIBLE as u8) );
	CSS_OK
}

pub fn css__compose_visibility(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut ftype = css_computed_visibility(child);

	if (ftype == (CSS_VISIBILITY_INHERIT as u8) ) {
		ftype = css_computed_visibility(parent);
		
		set_visibility(result, ftype);
		CSS_OK
	}
	else {
		set_visibility(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// voice_family
///////////////////////////////////////////////////////////////////
pub fn css__cascade_voice_family(opv:u32 , 
								style:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	let mut value : u16 = 0;
	let mut voices : ~[~str] = ~[];

	if (isInherit(opv) == false) {
		let mut v : u32 = getValue(opv) as u32;

		while (v != (VOICE_FAMILY_END as u32) ) {

			match (v as u16) {
				VOICE_FAMILY_STRING 	|
				VOICE_FAMILY_IDENT_LIST => {

					if style.sheet.is_none() {
						return CSS_BADPARM ;
					}
					let mut (result,o_voice)  = style.sheet.get().css__stylesheet_string_get( 
																peek_bytecode(style) as uint );
					match result {
						CSS_OK=>{} ,
						x => { return x ; }
					}
					if o_voice.is_none()  { return CSS_BADPARM ;}

					voices.push( o_voice.unwrap() );
					advance_bytecode(style);
				},
				VOICE_FAMILY_MALE => {
					if (value == 0) {
						value = 1;
					}
				},
				VOICE_FAMILY_FEMALE => {
					if (value == 0) {
						value = 1;
					}
				},
				VOICE_FAMILY_CHILD => {
					if (value == 0) {
						value = 1;
					}
				},
				_ => {}
			}

			/* Only use family-names which occur before the first
			 * generic-family. Any values which occur after the
			 * first generic-family are ignored. */
			/* \todo Do this at bytecode generation time? */

			v = peek_bytecode(style);
			advance_bytecode(style);
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		/* \todo voice-family */
		// set voice family in propset
	} 

	CSS_OK
}

pub fn css__set_voice_family_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_voice_family(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_voice_family(_:@mut css_computed_style,
								_:@mut css_computed_style,
								_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// volume
///////////////////////////////////////////////////////////////////
pub fn css__cascade_volume(opv:u32 , 
								style:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	// let mut val : i32 = 0;
	// let mut unit : u32 = UNIT_PCT as u32;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			VOLUME_NUMBER => {
				// val = peek_bytecode(style) as i32;
				advance_bytecode(style);
			},
			VOLUME_DIMENSION => {
				// val = peek_bytecode(style) as i32;
				advance_bytecode(style);
				// unit = peek_bytecode(style);
				advance_bytecode(style);
			},
			VOLUME_SILENT 	|
			VOLUME_X_SOFT 	|
			VOLUME_SOFT 	|
			VOLUME_MEDIUM 	|
			VOLUME_LOUD 	|
			VOLUME_X_LOUD => {
				/* \todo convert to public values */
			},
			_ => {}
		}
	}

	// unit = css__to_css_unit(unit) as u32;

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		/* \todo volume */
	}
	CSS_OK
}

pub fn css__set_volume_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_volume(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_volume(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// white_space
///////////////////////////////////////////////////////////////////
pub fn css__cascade_white_space(opv:u32, _:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	let mut value = CSS_WHITE_SPACE_INHERIT as u16;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			WHITE_SPACE_NORMAL => {
				value = ( CSS_WHITE_SPACE_NORMAL as u16) ;
			},
			WHITE_SPACE_PRE => {
				value = ( CSS_WHITE_SPACE_PRE as u16) ;
			},
			WHITE_SPACE_NOWRAP => {
				value = ( CSS_WHITE_SPACE_NOWRAP as u16) ;
			},
			WHITE_SPACE_PRE_WRAP => {
				value = ( CSS_WHITE_SPACE_PRE_WRAP as u16) ;
			},
			WHITE_SPACE_PRE_LINE => {
				value = ( CSS_WHITE_SPACE_PRE_LINE as u16) ;
			},
			_=>{}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_white_space(state.computed, value as u8);
	}

	CSS_OK
}

pub fn css__set_white_space_from_hint(hint:@mut  css_hint, 
									style:@mut css_computed_style
									) -> css_error {

	set_white_space(style, hint.status);
	CSS_OK
}

pub fn css__initial_white_space(state:@mut css_select_state) -> css_error {

	set_white_space(state.computed, (CSS_WHITE_SPACE_NORMAL as u8) );
	CSS_OK
}

pub fn css__compose_white_space(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut ftype = css_computed_white_space(child);

	if (ftype == (CSS_WHITE_SPACE_INHERIT as u8) ) {
		ftype = css_computed_white_space(parent);
		
		set_white_space(result, ftype);
		CSS_OK
	}
	else {
		set_white_space(result, ftype);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_width(opv:u32, style:@mut css_style, 
						state:@mut css_select_state) -> css_error {

	return css__cascade_length_auto(opv, style, state, @set_width);	
}

pub fn css__set_width_from_hint(hint:@mut  css_hint, 
								style:@mut css_computed_style
								) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_width(style, hint.status, x.value , x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_width(state:@mut css_select_state) -> css_error {

	set_width(state.computed, (CSS_WIDTH_AUTO as u8) , 0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_width(parent:@mut css_computed_style,
							child:@mut css_computed_style,
							result:@mut css_computed_style
							) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_width(child);

	if (ftype == (CSS_WIDTH_INHERIT as u8) ) {
		let mut (ftype2,olength2,ounit2) = css_computed_width(parent);
		set_width(result, 
				ftype2, 
				olength2.get_or_default( olength.get_or_default(0) ), 
				ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
		CSS_OK
	}
	else {
		set_width(result, ftype, 
				olength.get_or_default(0), 
				ounit.get_or_default(CSS_UNIT_PX));
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// windows
///////////////////////////////////////////////////////////////////
pub fn css__cascade_windows(opv:u32 , 
								style:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	return css__cascade_number(opv, style, state, None);
}

pub fn css__set_windows_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_windows(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_windows(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// word_spacing
///////////////////////////////////////////////////////////////////
pub fn css__cascade_word_spacing(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_length_normal(opv, style, state, @set_word_spacing);
}

pub fn css__set_word_spacing_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		HINT_LENGTH=>{
			match hint.length {
				Some(x)=>{
					set_word_spacing(style, hint.status, x.value, x.unit);
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_word_spacing(state:@mut css_select_state) -> css_error {

	set_word_spacing(state.computed, (CSS_WORD_SPACING_NORMAL as u8), 
			0, CSS_UNIT_PX);
	CSS_OK
}

pub fn css__compose_word_spacing(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {

	let mut (ftype,olength,ounit) = css_computed_word_spacing(child);

	if (  (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_WORD_SPACING_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

			if ( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_WORD_SPACING_INHERIT as u8) ) {

				let mut (ftype2,olength2,ounit2) = css_computed_word_spacing(parent);
				set_word_spacing(result, 
								ftype2, 
								olength2.get_or_default( olength.get_or_default(0) ), 
								ounit2.get_or_default( ounit.get_or_default(CSS_UNIT_PX) ));
			}
			else {
				set_word_spacing(result, 
								ftype, 
								olength.get_or_default(0), 
								ounit.get_or_default(CSS_UNIT_PX));
			}
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// cue_after
///////////////////////////////////////////////////////////////////
pub fn css__cascade_cue_after(opv:u32 , 
								style:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	return css__cascade_uri_none(opv, style, state, None);
}

pub fn css__set_cue_after_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_cue_after(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_cue_after(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////


// cue_before
///////////////////////////////////////////////////////////////////
pub fn css__cascade_cue_before(opv:u32 , 
								style:@mut css_style ,
								state: @mut css_select_state 
								) -> css_error {

	return css__cascade_uri_none(opv, style, state, None);
}

pub fn css__set_cue_before_from_hint(_: @mut css_hint, 
								_:@mut css_computed_style) 
								-> css_error {

	CSS_OK
}

pub fn css__initial_cue_before(_:@mut css_select_state) -> css_error {

	CSS_OK
}

pub fn css__compose_cue_before(_:@mut css_computed_style,
							_:@mut css_computed_style,
							_:@mut css_computed_style) -> css_error {

	CSS_OK
}

///////////////////////////////////////////////////////////////////

// z_index
///////////////////////////////////////////////////////////////////
pub fn css__cascade_z_index(opv:u32, style:@mut css_style, 
							state:@mut css_select_state) -> css_error {

	let mut value : u16= CSS_Z_INDEX_INHERIT as u16;
	let mut index : i32 = 0;

	if (isInherit(opv) == false) {
		match (getValue(opv)) {
			Z_INDEX_SET =>  {
				value = (CSS_Z_INDEX_SET  as u16) ;

				index = peek_bytecode(style) as i32;
				advance_bytecode(style);
			},
			Z_INDEX_AUTO =>  {
				value = (CSS_Z_INDEX_AUTO  as u16) ;
			},
			_=>{}
		}
	}

	if (css__outranks_existing( getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {
		set_z_index(state.computed, value as u8, index);
	}

	CSS_OK
}

pub fn css__set_z_index_from_hint(hint:@mut  css_hint, 
								style:@mut css_computed_style
								) -> css_error {

	match hint.hint_type {
		INTEGER_TYPE=>{
			match hint.integer {
				Some(x)=>{
					set_z_index(style, hint.status, x );
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_z_index(state:@mut css_select_state) -> css_error {

	set_z_index(state.computed, (CSS_Z_INDEX_AUTO as u8) , 0);
	CSS_OK
}

pub fn css__compose_z_index(parent:@mut css_computed_style,
							child:@mut css_computed_style,
							result:@mut css_computed_style
							) -> css_error {

	let mut (ftype,index) = css_computed_z_index(child);

	if (ftype == (CSS_Z_INDEX_INHERIT as u8) ) {
		let mut (ftype2,index2) = css_computed_z_index(parent);
		
		set_z_index(result, ftype2,index2);
		CSS_OK
	}
	else {
		set_z_index(result, ftype, index);
		CSS_OK
	}
}

///////////////////////////////////////////////////////////////////

// counter_increment
///////////////////////////////////////////////////////////////////
pub fn css__cascade_counter_increment(opv:u32, style:@mut css_style, 
									state:@mut css_select_state) -> css_error {

	return css__cascade_counter_increment_reset(opv, style, state, 
			@set_counter_increment);
}

pub fn css__set_counter_increment_from_hint(hint:@mut  css_hint, 
											style:@mut css_computed_style
											) -> css_error {

	match hint.hint_type {
		COUNTER=>{
			match hint.counters {
				Some(copy x)=>{
					set_counter_increment(style, hint.status, x);
					if hint.status == (CSS_COUNTER_INCREMENT_NAMED as u8) {
						hint.counters = None ;
					}
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_counter_increment(state:@mut css_select_state) -> css_error {

	set_counter_increment(state.computed, 
			(CSS_COUNTER_INCREMENT_NONE as u8), ~[]);
	CSS_OK
}

pub fn css__compose_counter_increment(parent:@mut css_computed_style,
									child:@mut css_computed_style,
									result:@mut css_computed_style
									) -> css_error {



	let mut (ftype,ocounters) = css_computed_counter_increment(child);

	if (  (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_COUNTER_INCREMENT_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

			if ( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_COUNTER_INCREMENT_INHERIT as u8) ) {

				let mut (ftype2,ocounters2) = css_computed_counter_increment(parent);
				set_counter_increment(result, ftype2, ocounters2 );
			}
			else {

				set_counter_increment(result, ftype, ocounters );
			}
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////

// counter_reset
///////////////////////////////////////////////////////////////////
pub fn css__cascade_counter_reset(opv:u32, style:@mut css_style, 
								state:@mut css_select_state) -> css_error {

	return css__cascade_counter_increment_reset(opv, style, state,
			@set_counter_reset);
}

pub fn css__set_counter_reset_from_hint(hint:@mut  css_hint, 
										style:@mut css_computed_style
										) -> css_error {

	match hint.hint_type {
		COUNTER=>{
			match hint.counters {
				Some(copy x)=>{
					set_counter_reset(style, hint.status, x);
					if hint.status == (CSS_COUNTER_INCREMENT_NAMED as u8) {
						hint.counters = None ;
					}
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_counter_reset(state:@mut css_select_state) -> css_error {

	set_counter_reset(state.computed, 
			(CSS_COUNTER_RESET_NONE as u8), ~[]);
	CSS_OK
}

pub fn css__compose_counter_reset(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut (ftype,ocounters) = css_computed_counter_reset(child);

	if (  (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_COUNTER_RESET_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

			if ( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_COUNTER_RESET_INHERIT as u8) ) {

				let mut (ftype2,ocounters2) = css_computed_counter_reset(parent);
				set_counter_reset(result, ftype2, ocounters2 );
			}
			else {
				set_counter_reset(result, ftype, ocounters );
			}
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////


// cursor
///////////////////////////////////////////////////////////////////
pub fn css__cascade_cursor(opv:u32, style:@mut css_style, 
							state:@mut css_select_state) -> css_error {

	let mut value : u16= CSS_CURSOR_INHERIT as u16;
	let mut uris : ~[~str] = ~[] ;

	if (isInherit(opv) == false) {
		let mut v : u32 = getValue(opv) as u32;

		while (v == (CURSOR_URI as u32) ) {

			if style.sheet.is_none() {
				return CSS_BADPARM ;
			}
			let mut (result,o_url)  = style.sheet.get().css__stylesheet_string_get( 
														peek_bytecode(style) as uint );
			advance_bytecode(style);
			match result {
				CSS_OK=>{} ,
				x => { return x ; }
			}
			if o_url.is_none()  { return CSS_BADPARM ;}
			uris.push( o_url.unwrap()  );

			v = peek_bytecode(style);
			advance_bytecode(style);
		}

		match (v as u16) {
			CURSOR_AUTO => {
				value = ( CSS_CURSOR_AUTO as u16) ;
			},
			CURSOR_CROSSHAIR => {
				value = ( CSS_CURSOR_CROSSHAIR as u16) ;
			},
			CURSOR_DEFAULT => {
				value = ( CSS_CURSOR_DEFAULT as u16) ;
			},
			CURSOR_POINTER => {
				value = ( CSS_CURSOR_POINTER as u16) ;
			},
			CURSOR_MOVE => {
				value = ( CSS_CURSOR_MOVE as u16) ;
			},
			CURSOR_E_RESIZE => {
				value = ( CSS_CURSOR_E_RESIZE as u16) ;
			},
			CURSOR_NE_RESIZE => {
				value = ( CSS_CURSOR_NE_RESIZE as u16) ;
			},
			CURSOR_NW_RESIZE => {
				value = ( CSS_CURSOR_NW_RESIZE as u16) ;
			},
			CURSOR_N_RESIZE => {
				value = ( CSS_CURSOR_N_RESIZE as u16) ;
			},
			CURSOR_SE_RESIZE => {
				value = ( CSS_CURSOR_SE_RESIZE as u16) ;
			},
			CURSOR_SW_RESIZE => {
				value = ( CSS_CURSOR_SW_RESIZE as u16) ;
			},
			CURSOR_S_RESIZE => {
				value = ( CSS_CURSOR_S_RESIZE as u16) ;
			},
			CURSOR_W_RESIZE => {
				value = ( CSS_CURSOR_W_RESIZE as u16) ;
			},
			CURSOR_TEXT => {
				value = ( CSS_CURSOR_TEXT as u16) ;
			},
			CURSOR_WAIT => {
				value = ( CSS_CURSOR_WAIT as u16) ;
			},
			CURSOR_HELP => {
				value = ( CSS_CURSOR_HELP as u16) ;
			},
			CURSOR_PROGRESS => {
				value = ( CSS_CURSOR_PROGRESS as u16) ;
			},
			_ => {}
		}
	}

	if (css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state,
			isInherit(opv))) {

		set_cursor(state.computed, value as u8, uris);
	} 

	CSS_OK
}

pub fn css__set_cursor_from_hint(hint:@mut  css_hint, 
								style:@mut css_computed_style
								) -> css_error {

	match hint.hint_type {
		STRINGS_VECTOR=>{
			match hint.strings {
				Some(copy x)=>{
					set_cursor(style, hint.status, x);
					hint.counters = None ;
					CSS_OK
				},
				None=>{
					CSS_BADPARM
				}
			}
		},
		_=>{
			CSS_INVALID 
		}
	}
}

pub fn css__initial_cursor(state:@mut css_select_state) -> css_error {

	set_cursor(state.computed, 
			(CSS_CURSOR_AUTO as u8), ~[]);
	CSS_OK
}

pub fn css__compose_cursor(parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style
								) -> css_error {

	let mut (ftype,ourl) = css_computed_cursor(child);

	if (  (child.uncommon.is_none() && parent.uncommon.is_some() ) || 
			ftype == (CSS_CURSOR_INHERIT as u8) || 
			(child.uncommon.is_some() && !mut_ptr_eq(result,child) ) ) {

			if ( ( child.uncommon.is_none() && parent.uncommon.is_some() ) ||
					ftype == (CSS_CURSOR_INHERIT as u8) ) {

				let mut (ftype2,ourl2) = css_computed_cursor(parent);
				set_cursor(result, ftype2, ourl2.get_or_default( ourl.get_or_default(~[]) ) );
			}
			else {
				set_cursor(result, ftype, ourl.get_or_default(~[]) );
			}
	}
	CSS_OK
}

///////////////////////////////////////////////////////////////////
// content
///////////////////////////////////////////////////////////////////
pub fn css__cascade_content(opv:u32, style:@mut css_style, 
    state:@mut css_select_state ) -> css_error {

    let mut value = CSS_CONTENT_INHERIT;
    let mut content:~[@mut css_computed_content_item]=~[];
    // uint32_t n_contents = 0;

    if !isInherit(opv) {
        let mut v = getValue(opv);
        match v {
            CONTENT_NORMAL => value = CSS_CONTENT_NORMAL,
            CONTENT_NONE => value = CSS_CONTENT_NONE,
            _ => {
                value = CSS_CONTENT_SET;
                
                while (v != CONTENT_NORMAL) {
                    
                    let temp:@mut css_computed_content_item=
                     @mut css_computed_content_item{item_type:CSS_COMPUTED_CONTENT_NONE,data:None, counters_data:None};
                       
                    let (result, he_option) = style.sheet.get().css__stylesheet_string_get(peek_bytecode(style) as uint);
                    
                    match result {
                        CSS_OK => {},
                        error => return error
                    }

                    match (v & 0xff) {

                        CONTENT_COUNTER => {

                            advance_bytecode(style);

                            temp.item_type =  CSS_COMPUTED_CONTENT_COUNTER;
                            temp.data = None;
                            temp.counters_data = Some(css_computed_content_item_counter {
                                name:he_option.unwrap(), 
                                sep:None,
                                style:(v >> CONTENT_COUNTER_STYLE_SHIFT) as u8
                            } );
                            
                            content.push(temp)
                        },                          
                        CONTENT_COUNTERS => {

                            advance_bytecode(style);

                            let (result, sep_option) = style.sheet.get().css__stylesheet_string_get(peek_bytecode(style) as uint);
                            
                            match result {
                                CSS_OK => {},
                                error => return error
                            } 
                            
                            advance_bytecode(style);

                            temp.item_type =  CSS_COMPUTED_CONTENT_COUNTERS;
                            temp.data = None;
                            temp.counters_data = Some(css_computed_content_item_counter {
                                name:he_option.unwrap(), 
                                sep:sep_option,
                                style:(v >> CONTENT_COUNTERS_STYLE_SHIFT) as u8
                            } );
                        },                      
                        CONTENT_URI => {

                            advance_bytecode(style);

                            temp.item_type = CSS_COMPUTED_CONTENT_URI;
                            temp.data = he_option;
                            temp.counters_data = None
                        },  
                        CONTENT_ATTR => {
                            
                            advance_bytecode(style);

                            temp.item_type = CSS_COMPUTED_CONTENT_ATTR;
                            temp.data = he_option;
                            temp.counters_data = None
                        },
                        CONTENT_STRING => {

                            advance_bytecode(style);

                            temp.item_type =  CSS_COMPUTED_CONTENT_STRING;
                            temp.data = he_option;
                            temp.counters_data = None
                        },
                        CONTENT_OPEN_QUOTE => {
                            temp.item_type =  CSS_COMPUTED_CONTENT_OPEN_QUOTE;
                            temp.data = None;
                            temp.counters_data = None
                        },
                        CONTENT_CLOSE_QUOTE => {
                            temp.item_type =  CSS_COMPUTED_CONTENT_CLOSE_QUOTE;
                            temp.data = None;
                            temp.counters_data = None
                        },
                        CONTENT_NO_OPEN_QUOTE => {
                            temp.item_type =  CSS_COMPUTED_CONTENT_NO_OPEN_QUOTE;
                            temp.data = None;
                            temp.counters_data = None
                        },                  
                        CONTENT_NO_CLOSE_QUOTE => {
                            temp.item_type =  CSS_COMPUTED_CONTENT_NO_CLOSE_QUOTE;
                            temp.data = None;
                            temp.counters_data = None
                        }
                        _ => {}
                    }
                        
                    content.push(temp);                 
                    v = peek_bytecode(style) as u16;
                    advance_bytecode(style)
                }
            }
        }
    }

    /* If we have some content, terminate the array with a blank entry */
    if !content.is_empty() {
        let temp = @mut css_computed_content_item{item_type:CSS_COMPUTED_CONTENT_NONE,data:None, counters_data:None};
        content.push(temp);
    }

   
    if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
        set_content(state.computed, value as u8, content)     
    } 
        
    CSS_OK
}


pub fn css__set_content_from_hint(hint:@mut css_hint, 
								style:@mut css_computed_style) 
								-> css_error{

    set_content(style, hint.status, ~[hint.content.unwrap()]);
    CSS_OK
}

pub fn css__initial_content(state:@mut css_select_state) -> css_error {
    set_content(state.computed, CSS_CONTENT_NORMAL as u8 , ~[]);
    CSS_OK
}


pub fn css__compose_content( parent:@mut css_computed_style, 
							child:@mut css_computed_style,
    						result:@mut css_computed_style) 
							-> css_error {

    let mut (content_type, items) = css_computed_content(child);
        
    if ((match child.uncommon {None => true, _ => false } ) && (match parent.uncommon {Some(_) => true, None => false } )) ||
            content_type == CSS_CONTENT_INHERIT as u8 || (match child.uncommon { Some(_) => true, _ => false} && 
                !mut_ptr_eq(result,child)) {
        
        if (match child.uncommon { None => true, _ => false} && match parent.uncommon { Some(_) => true,  None => false }) || 
           content_type == CSS_CONTENT_INHERIT as u8 {
            let (p_content_type,p_items) = css_computed_content(parent);
            content_type = p_content_type;
            items = p_items
        }
        set_content(result, content_type, items) ;
    }
   	CSS_OK
}


/////////////////////////////////////////////////////////////////

// column_span
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_span(opv:u32, _:@mut css_style, 
    state:@mut css_select_state ) -> css_error {
  
	if !isInherit(opv) {
		match getValue(opv) {
	    	COLUMN_SPAN_NONE |
	    	COLUMN_SPAN_ALL  => {
	        	//* \todo convert to public values */  
	      	},  
	      	_ => {}
	    }
	}

  if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
    // \todo set computed elevation */
  }

  CSS_OK
}

pub fn css__set_column_span_from_hint(_:@mut css_hint, 
									_:@mut css_computed_style)
									-> css_error {
  // DO NOTHING
  CSS_OK
}

pub fn css__initial_column_span(_:@mut css_select_state) -> css_error {
  
  CSS_OK
}

pub fn css__compose_column_span(_:@mut css_computed_style, 
								_:@mut css_computed_style,
  								_:@mut css_computed_style
  								) -> css_error {
  //DO NOTHING
  CSS_OK
}

/////////////////////////////////////////////////////////////////// 

// column_width
///////////////////////////////////////////////////////////////////
pub fn css__cascade_column_width(opv:u32, style:@mut css_style, 
    							state:@mut css_select_state ) 
								-> css_error {

	// let mut length : i32 = 0;
	// let mut unit : u32 = UNIT_PX;

  	if !isInherit(opv) {
	    match getValue(opv) {
		    COLUMN_WIDTH_SET => {
		        // length = peek_bytecode(style) as i32;
		        advance_bytecode(style);
		        // unit = peek_bytecode(style);
		        advance_bytecode(style);
		    },
		    COLUMN_WIDTH_AUTO => {
		        //* \todo convert to public values */  
		    },  
		    _ => {}
		}
	}

	if css__outranks_existing(getOpcode(opv) as u16, isImportant(opv), state, isInherit(opv)) {
    	// \todo set computed elevation */
	}

	CSS_OK
}

pub fn css__set_column_width_from_hint(_:@mut css_hint, 
									_:@mut css_computed_style) 
									-> css_error{
	//DO NOTHING
	CSS_OK
}

pub fn css__initial_column_width(_:@mut css_select_state) -> css_error {
  
	CSS_OK
}

pub fn css__compose_column_width(_:@mut css_computed_style, 
								_:@mut css_computed_style,
  								_:@mut css_computed_style) 
								-> css_error {
	//DO NOTHING
	CSS_OK
}

///////////////////////////////////////////////////////////////////