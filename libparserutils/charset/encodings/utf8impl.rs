use utils::errors::*;

pub fn parserutils_charset_utf8_from_ucs4(mut ucs4: u32) -> (Option<~[u8]>, parserutils_error) {
	let mut output : ~[u8] = ~[];

	let l = {
		if (ucs4 < 0x80) {
			1
		} else if (ucs4 < 0x800) {			
			2
		} else if (ucs4 < 0x10000) {		
			3
		} else if (ucs4 < 0x200000) {		
			4
		} else if (ucs4 < 0x4000000) {		
			5
		} else if (ucs4 <= 0x7FFFFFFF) {	
			6
		} else {							
			return (None, PARSERUTILS_INVALID);
		}	

	};
	if (l==1) {
		output.push(ucs4 as u8);
	}
	else {
		vec::grow(&mut output, l, &0u8);

		let mut count=l;
		while (count > 1) {
			output[count-1] = (0x80 | (ucs4 & 0x3f)) as u8;

			ucs4 >>= 6;
			count -= 1;
		}
		output[0] = (!((1u32 << (8-l)) - 1u32) | ucs4) as u8;
	}
	(Some(output), PARSERUTILS_OK)
}