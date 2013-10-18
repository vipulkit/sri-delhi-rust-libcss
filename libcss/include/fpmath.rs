
use std::{int};

pub static CSS_RADIX_POINT : int = 10 ;
pub static INT_MIN : int = int::min_value ;
pub static INT_MAX : int = int::max_value ;
pub static I32_MAX : i32 = 2147483647 ;

#[inline]
pub fn css_int_to_fixed(a:int) -> i32 {

    let mut xx:i64 = (a as i64) << CSS_RADIX_POINT;

    if (xx < INT_MIN as i64) {
        xx = INT_MIN as i64;
    }

    if (xx > INT_MAX as i64) {
        xx = INT_MAX as i64;
    }
    
    xx as i32
}

#[inline]
pub fn css_divide_fixed(x : i32, y : i32) -> i32 {
    let mut xx :i64 = ((x as i64) << CSS_RADIX_POINT) / (y as i64);

    if (xx < INT_MIN as i64) {
    xx = INT_MIN as i64;
    }

    if (xx > INT_MAX as i64) {
    xx = INT_MAX as i64;
    }

    xx as i32
}

#[inline]
pub fn css_multiply_fixed(x: i32 , y: i32) -> i32 {
    // debug!((fmt!("x = %?", x)));
	// debug!((fmt!("y = %?", y)));
	
	let mut xx: i64 = ((x as i64)*(y as i64)) >> CSS_RADIX_POINT;
    
    if xx < (INT_MIN as i64) {
        xx = INT_MIN as i64;
    }

    if xx > (INT_MAX as i64) {
        xx = INT_MAX as i64;
    }
    // debug!((fmt!("xx = %?", xx)));
	// debug!((fmt!("res_xx = %?", xx as i32)));
    xx as i32
}

#[inline]
pub fn css_add_fixed(x: i32 , y: i32) -> i32 {
    let mut ux: i32 = x;
    let uy: i32 = y;
    let mut res = ux + uy;
    
    /* Calculate overflowed result. (Don't change the sign bit of ux) */
    ux = (ux >> 31) + INT_MAX as i32;
    
    /* Force compiler to use cmovns instruction */
    if ((ux ^ uy) | ((uy ^ res)^ (1 as i32)) >= 0) {
        res = ux;
    }
        
    return res;
}

#[inline]
pub fn css_subtract_fixed(x: i32 , y: i32) -> i32{
    // debug!((fmt!("x = %?", x)));
	// debug!((fmt!("y = %?", y)));
	let mut ux: i32 = x;
    let uy: i32 = y;
    let mut res = ux - uy;
    
    ux = (ux >> 31) + I32_MAX;
	// debug!((fmt!("INT_MAX = %?", I32_MAX)));
	// debug!((fmt!("INT_MAX = %?", 0x7FFF)));
    // debug!((fmt!("ux = %?", ux)));
    /* Force compiler to use cmovns instruction */
    // debug!((fmt!("ux ^ uy = %?", ux ^ uy)));
	// debug!((fmt!("ux ^ res = %?", ux ^ res)));
	// debug!((fmt!("(ux ^ uy) & (ux ^ res) = %?", (ux ^ uy) & (ux ^ res))));
	if (((ux ^ uy) & (ux ^ res)) < 0) {
        res = ux;
    }
        
        
    return res;
}

pub static F_PI_2: i32 =    0x00000648; /* 1.5708 (PI/2) */
pub static F_PI: i32 =  0x00000c91; /* 3.1415 (PI) */
pub static F_3PI_2: i32 =   0x000012d9; /* 4.7124 (3PI/2) */
pub static F_2PI: i32 = 0x00001922; /* 6.2831 (2 PI) */

pub static F_90: i32 =  0x00016800; /*  90 */
pub static F_180: i32 = 0x0002d000; /* 180 */
pub static F_270: i32 = 0x00043800; /* 270 */
pub static F_360: i32 = 0x0005a000; /* 360 */

pub static F_0_5: i32 = 0x00000200; /* 0.5 */
pub static F_1: i32 =   0x00000400; /*   1 */
pub static F_10: i32 =  0x00002800; /*  10 */
pub static F_72: i32 =  0x00012000; /*  72 */
pub static F_100: i32 = 0x00019000; /* 100 */
pub static F_200: i32 = 0x00032000; /* 200 */
pub static F_255: i32 = 0x0003FC00; /* 255 */
pub static F_300: i32 = 0x0004b000; /* 300 */
pub static F_400: i32 = 0x00064000; /* 400 */

#[inline]
pub fn FLTTOFIX(a:float)->i32 {
     return (a * ((1 << CSS_RADIX_POINT) as float)) as i32;
}


