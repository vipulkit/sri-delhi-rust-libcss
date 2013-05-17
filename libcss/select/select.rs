use include::types::*;
use select::common::*;
use stylesheet::*;

pub fn css__outranks_existing(op:u16, 
							important:bool, 
							state: @mut css_select_state,
							inherit:bool) -> bool {
	true 
}

pub fn advance_bytecode(style: @mut css_style) {
	unsafe{
	 	if style.bytecode.len() - style.used > 1 {
			style.used += 1	
		}
		else {
			fail!(~"Advancing Bytecode vector after end index")
		}
	}
}	