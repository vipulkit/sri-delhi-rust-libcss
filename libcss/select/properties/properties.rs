use include::types::*;
use bytecode::bytecode::*;
use bytecode::opcodes::*;
use stylesheet::*;
use utils::errors::*;
use select::select::*;
use select::common::*;


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