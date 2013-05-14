use include::properties::*;

pub enum flag {
	FLAG_IMPORTANT			= (1<<0),
	FLAG_INHERIT			= (1<<1)
}

pub enum unit {
	UNIT_PX   = 0,
	UNIT_EX   = 1,
	UNIT_EM   = 2,
	UNIT_IN   = 3,
	UNIT_CM   = 4,
	UNIT_MM   = 5,
	UNIT_PT   = 6,
	UNIT_PC   = 7,

	UNIT_PCT  = (1 << 8),

	UNIT_DEG  = (1 << 9) + 0,
	UNIT_GRAD = (1 << 9) + 1,
	UNIT_RAD  = (1 << 9) + 2,

	UNIT_MS   = (1 << 10) + 0,
	UNIT_S    = (1 << 10) + 1,

	UNIT_HZ   = (1 << 11) + 0,
	UNIT_KHZ  = (1 << 11) + 1
} 

pub enum shape {
	SHAPE_RECT = 0
} 

pub static   UNIT_ANGLE :  unit = UNIT_DEG ;	//< Default level >
pub static   UNIT_TIME  :  unit = UNIT_MS  ;	//< Default level >
pub static   UNIT_FREQ  :  unit = UNIT_HZ  ;	//< Default level >



pub fn buildOPV(opcode : css_properties_e , flags : u8 , value : u16 ) -> u32 {

	(( (opcode as int)  & 0x3ff) | ((flags as int)<< 10) | (((value as int)& 0x3fff)  << 18) ) as u32
}

pub fn buildOPV_flag(opcode : css_properties_e , flags :flag , value : u16 ) -> u32 {

	(( (opcode as int)  & 0x3ff) | ((flags as int)<< 10) | (((value as int)& 0x3fff)  << 18) ) as u32
}

pub fn getOpcode(OPV : u32 ) -> css_properties_e {

	//((OPV & 0x3ff) as int) as opcode_t
	let op_code : int = (OPV & 0x3ff) as int ;
	unsafe { cast::transmute(&op_code) }
}

pub fn getFlags(OPV : u32 ) -> u8 {

	((OPV >> 10) & 0xff) as u8
}

pub fn getValue(OPV : u32 ) -> u16 {

	(OPV >> 18) as u16
}

pub fn isImportant(OPV : u32 ) -> bool {

	if (getFlags(OPV) & 0x1)==0 {
		false
	}
	else {
		true
	}
}

pub fn isInherit(OPV : u32 ) -> bool {

	if (getFlags(OPV) & 0x2)==0 {
		false 
	}
	else {
		true
	}
}