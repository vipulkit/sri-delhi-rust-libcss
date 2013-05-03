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


