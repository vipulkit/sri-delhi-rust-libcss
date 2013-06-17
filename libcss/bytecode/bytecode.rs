use include::properties::*;

pub enum flag {
    FLAG_IMPORTANT          = (1<<0),
    FLAG_INHERIT            = (1<<1)
}

pub static  UNIT_PX : u32  = 0;
pub static  UNIT_EX : u32  = 1;
pub static  UNIT_EM : u32  = 2;
pub static  UNIT_IN : u32  = 3;
pub static  UNIT_CM : u32  = 4;
pub static  UNIT_MM : u32  = 5;
pub static  UNIT_PT : u32  = 6;
pub static  UNIT_PC : u32  = 7;

pub static  UNIT_PCT : u32 = (1 << 8);

pub static  UNIT_DEG : u32 = (1 << 9) + 0;
pub static  UNIT_GRAD: u32 = (1 << 9) + 1;
pub static  UNIT_RAD : u32 = (1 << 9) + 2;

pub static  UNIT_MS  : u32 = (1 << 10) + 0;
pub static  UNIT_S   : u32 = (1 << 10) + 1;

pub static  UNIT_HZ  : u32 = (1 << 11) + 0;
pub static  UNIT_KHZ : u32 = (1 << 11) + 1;

pub static   UNIT_ANGLE :  u32 = UNIT_DEG ; //< Default level >
pub static   UNIT_TIME  :  u32 = UNIT_MS  ; //< Default level >
pub static   UNIT_FREQ  :  u32 = UNIT_HZ  ; //< Default level >

pub enum shape {
    SHAPE_RECT = 0
} 





pub fn buildOPV(opcode : css_properties_e , flags : u8 , value : u16 ) -> u32 {

    (( (opcode as int)  & 0x3ff) | ((flags as int)<< 10) | (((value as int)& 0x3fff)  << 18) ) as u32
}

pub fn buildOPV_flag(opcode : css_properties_e , flags :flag , value : u16 ) -> u32 {

    (( (opcode as int)  & 0x3ff) | ((flags as int)<< 10) | (((value as int)& 0x3fff)  << 18) ) as u32
}

pub fn getOpcode(OPV : u32 ) -> css_properties_e {
    //((OPV & 0x3ff) as int) as opcode_t
    let op_code = (OPV & 0x00003ff);
    unsafe { cast::transmute(op_code as uint) }
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