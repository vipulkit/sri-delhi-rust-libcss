#[link(name = "css_fpmath", vers = "0.1")];
#[crate_type = "lib"];


static CSS_RADIX_POINT : int = 10 ;
static INT_MIN : int = int::min_value ;
static INT_MAX : int = int::max_value ;

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

static F_PI_2: int =	0x00000648;	/* 1.5708 (PI/2) */
static F_PI: int =	0x00000c91;	/* 3.1415 (PI) */
static F_3PI_2: int =	0x000012d9;	/* 4.7124 (3PI/2) */
static F_2PI: int =	0x00001922;	/* 6.2831 (2 PI) */

static F_90: int =	0x00016800;	/*  90 */
static F_180: int =	0x0002d000;	/* 180 */
static F_270: int =	0x00043800;	/* 270 */
static F_360: int =	0x0005a000;	/* 360 */

static F_0_5: int =	0x00000200;	/* 0.5 */
static F_1: int =	0x00000400;	/*   1 */
static F_10: int =	0x00002800;	/*  10 */
static F_72: int =	0x00012000;	/*  72 */
static F_100: int =	0x00019000;	/* 100 */
static F_200: int =	0x00032000;	/* 200 */
static F_255: int =	0x0003FC00;	/* 255 */
static F_300: int =	0x0004b000;	/* 300 */
static F_400: int =	0x00064000;	/* 400 */
