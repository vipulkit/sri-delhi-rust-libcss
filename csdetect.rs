#[link(name = "csdetect", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod riconv;
extern mod parserutils;
use parserutils::*;
use std::arc;

pub enum css_charset_source {
	CSS_CHARSET_DEFAULT=0,
	CSS_CHARSET_REFERRED=1,
	CSS_CHARSET_METADATA=2,
	CSS_CHARSET_DOCUMENT=3,
	CSS_CHARSET_DICTATED=4
}

pub fn try_utf32_charset(data : &~[u8], lpu_arc: arc::ARC<~lpu>) -> (Option<u16>, parserutils_error) {

	let mut charset: u16 = 0;
	let CHARSET_BE : &[u8] = ['0' as u8, '0' as u8, '0' as u8, '@' as u8, '0' as u8, '0' as u8, '0' as u8, 'c' as u8, '0' as u8, '0' as u8, '0' as u8, 'h' as u8, '0' as u8, '0' as u8, '0' as u8, 'a' as u8, '0' as u8, '0' as u8, '0' as u8, 'r' as u8, '0' as u8, '0' as u8, '0' as u8, 's' as u8, '0' as u8, '0' as u8, '0' as u8, 'e' as u8, '0' as u8, '0' as u8, '0' as u8, 't' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '"' as u8] ; 
	let CHARSET_LE : &[u8] = [ '@' as u8,'0' as u8,'0' as u8,'0' as u8,'c' as u8,'0' as u8,'0' as u8,'0' as u8,'h' as u8,'0' as u8,'0' as u8,'0' as u8,'a' as u8,'0' as u8,'0' as u8,'0' as u8,'r' as u8,'0' as u8,'0' as u8,'0' as u8,'s' as u8,'0' as u8,'0' as u8,'0' as u8,'e' as u8,'0' as u8,'0' as u8,'0' as u8,'t' as u8,'0' as u8,'0' as u8,' ' as u8,'0' as u8,'0' as u8,'0' as u8,'"' as u8,'0' as u8,'0' as u8,'0' as u8, ] ;

	let UTF32LE: &[u8] = ['U' as u8 , 'T' as u8 , 'F' as u8 , '-' as u8 , '3' as u8 , '2' as u8 , 'L' as u8 , 'E' as u8];
	let UTF32BE: &[u8] = ['U' as u8 , 'T' as u8 , 'F' as u8 , '-' as u8 , '3' as u8 , '2' as u8 , 'B' as u8 , 'E' as u8];
	let UTF32 : &[u8] = ['U' as u8 , 'T' as u8 , 'F' as u8 , '-' as u8 , '3' as u8 , '2' as u8];
	// Here, when the data.len() is equals to CHARSET_LE.len() then it returns, then how would the next case would paas when again we re asking it to pass when length are equal ??
	if data.len() <= CHARSET_LE.len() {
		return (None, PARSERUTILS_BADPARAM);
	}

	if (memcmp(data, CHARSET_LE, CHARSET_LE.len()) == 0) {
		let startIndex : uint = data.len() + CHARSET_LE.len();
		let mut endIndex : uint = startIndex;

		// values are only for initialization
		let mut buffMemory: ~[u8] = ~[];
		let mut buffMemoryIndex: uint = 0;
		
		while endIndex < (CHARSET_LE.len() -1) {
			let value1 : u8 = data[endIndex] | data[endIndex + 1] << 8 | data[endIndex + 2] << 16 | data[endIndex + 3] << 24 ;
	
			if value1 > 0x007f {
				break;
			}	

			if (value1 == '"' as u8) && (endIndex < data.len() + CHARSET_LE.len() - 8) {
				let value2 = data[endIndex + 4] | data[endIndex + 5] << 8 | data[endIndex + 6] << 16 | data[endIndex + 7] << 24 ;
				if value2 == ';' as u8 {
					break;
				}
			}			
		
			if buffMemoryIndex < 8 {
				if value1 >= 'a' as u8 && value1 <= 'z' as u8 {
					buffMemory.push((value1 & !0x20) as u8);	
				}
				else {
					buffMemory.push(value1 as u8);	
				}
				buffMemoryIndex += 1;	
			}	
			
			endIndex += 4;	
		} // while loop ends		
		
		// After while loop ends
		if (endIndex == data.len() - 4) {
			return (None, PARSERUTILS_NEEDDATA);
		}

		if (buffMemory.len() ==(str::len(~"UTF-32LE")) && memcmp(&buffMemory, UTF32LE, buffMemory.len()) == 0) ||
			(buffMemory.len() == (str::len(~"UTF-32")) && memcmp(&buffMemory, UTF32, buffMemory.len()) == 0) {

				charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32LE");
		}
	}
	
	else if (memcmp(data, CHARSET_BE, CHARSET_BE.len()) == 0) {
	
		let startIndex : uint = CHARSET_BE.len() - 1;
		let mut endIndex : uint = startIndex;

		// values are only for initialization
		let mut buffMemory : ~[u8] = ~[];
		let mut buffMemoryIndex : u8 = 0;
		
		while (endIndex < (data.len() - 4)) {
			let value1 : u8 = data[endIndex + 3] | data[endIndex + 2] << 8 | data[endIndex + 1] << 16 | data[endIndex] << 24 ;
			
			if value1 > 0x007f {
				break;
			}	
			
			if (value1 == '"' as u8) && (endIndex < data.len() + CHARSET_BE.len() - 8) {
				let value2 = data[endIndex + 7] | data[endIndex + 6] << 8 | data[endIndex + 5] << 16 | data[endIndex + 4] << 24 ;
				if value2 == ';' as u8 {
					break;
				}
			}			
		
			if buffMemoryIndex < 8 {
				if value1 >= 'a' as u8 && value1 <= 'z' as u8 {
					buffMemory.push((value1 & !0x20) as u8);			
				}
				else {
					buffMemory.push(value1 as u8);	
				}
				buffMemoryIndex += 1;	
			}	
			endIndex += 4;	
		} // while loop ends

		if (endIndex == data.len() - 4)
		{
			return (None, PARSERUTILS_NEEDDATA);
		}

		if (buffMemory.len() ==(str::len(~"UTF-32BE")) && memcmp(&buffMemory, UTF32BE, buffMemory.len()) == 0) ||
			(buffMemory.len() == (str::len(~"UTF-32")) && memcmp(&buffMemory, UTF32, buffMemory.len()) == 0) {

				charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32BE");
		}
	}// else if terminates
	(Some(charset) , PARSERUTILS_OK)
}	
	

pub fn try_utf16_charset(data : &~[u8], lpu_arc: arc::ARC<~lpu>) -> (Option<u16>, parserutils_error) {
	let mut charset: u16 = 0;
	let CHARSET_BE : &[u8] = ['0' as u8, '@' as u8, '0' as u8, 'c' as u8, '0' as u8, 'h' as u8, '0' as u8, 'a' as u8, '0' as u8, 'r' as u8, '0' as u8, 's' as u8, '0' as u8, 'e' as u8, '0' as u8, 't' as u8, '0' as u8, ' ' as u8,'0' as u8, '"' as u8] ; 
	let CHARSET_LE : &[u8] = ['@' as u8, '0' as u8, 'c' as u8, '0' as u8, 'h' as u8, '0' as u8, 'a' as u8, '0' as u8, 'r' as u8, '0' as u8, 's' as u8, '0' as u8, 'e' as u8, '0' as u8, 't' as u8, '0' as u8, ' ' as u8, '0' as u8, '"' as u8, '0' as u8] ; 
	
	let UTF16LE: &[u8] = ['U' as u8 , 'T' as u8 , 'F' as u8 , '-' as u8 , '1' as u8 , '6' as u8 , 'L' as u8 , 'E' as u8];
	let UTF16BE: &[u8] = ['U' as u8 , 'T' as u8 , 'F' as u8 , '-' as u8 , '1' as u8 , '6' as u8 , 'B' as u8 , 'E' as u8];
	let UTF16 : &[u8] = ['U' as u8 , 'T' as u8 , 'F' as u8 , '-' as u8 , '1' as u8 , '6' as u8];

	if data.len() <= CHARSET_LE.len() {
		return (None, PARSERUTILS_BADPARAM);
	}

	if (memcmp(data, CHARSET_LE, CHARSET_LE.len()) == 0) 
	{
		let startIndex : uint = CHARSET_LE.len() - 1 ;
		let mut endIndex : uint = startIndex;

		// values are only for initialization
		let mut buffMemory : ~[u8] = ~[];
		let mut buffMemoryIndex: uint = 0;

		while endIndex < (data.len()- 2) {
			let value1 : u16 = (data[endIndex] | data[endIndex + 1] << 8) as u16 ;

			if value1 > 0x007f {
				break;
			}	

			if (value1 == '"' as u16) && (endIndex < data.len() + CHARSET_LE.len() - 4)	{
				let value2 : u16 = (data[endIndex + 2] | data[endIndex + 3] << 8) as u16 ;
				if value2 == ';' as u16	{
					break;
				}
			}			
		
			if buffMemoryIndex < 8 {
				if value1 >= 'a' as u16 && value1 <= 'z' as u16	{
					buffMemory.push((value1 & !0x20) as u8);			
				}
				else {
					buffMemory.push(value1 as u8);	
				}
				buffMemoryIndex += 1;
			}	
			// termination conditioning for while loop	
			endIndex += 2;	
		} // while loop ends		
		
		// After while loop ends
		if (endIndex == data.len() + CHARSET_LE.len() - 2) {
			return (None, PARSERUTILS_NEEDDATA);
		}

		if (buffMemory.len() ==(str::len(~"UTF-16LE")) && memcmp(&buffMemory, UTF16LE, buffMemory.len()) == 0) ||
			(buffMemory.len() == (str::len(~"UTF-16")) && memcmp(&buffMemory, UTF16, buffMemory.len()) == 0) {

				charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16LE");
		}
	}

	else if (memcmp(data, CHARSET_BE, CHARSET_BE.len()) == 0) {

		let startIndex : uint = (CHARSET_BE.len() - 1);
		let mut endIndex : uint = startIndex;
		
		// values are only for initialization
		let mut buffMemory : ~[u8] = ~[];
		let mut buffMemoryIndex : u8 = 0;

		while endIndex < (data.len() - 2) {
			// Since it is Big-endian, data at MSB would be at lower address space
			// let value1 : u16 = (data[endIndex + 1] | data[endIndex] << 8) as u16 ;
			let mut value1 : u16 = data[endIndex] as u16;

			if value1 > 0x007f {
				break;
			}

			if (value1 == '"' as u16) && (endIndex < data.len() - 4) {
				let value2 = (data[endIndex + 3] | data[endIndex + 2] << 8) as u16;
				if value2 == ';' as u16 {
					break;
				}
			}			
		
			if buffMemoryIndex < 8 {
				if value1 >= 'a' as u16 && value1 <= 'z' as u16 {
					buffMemory.push((value1 & !0x20) as u8);			
				}
				else {
					buffMemory.push(value1 as u8);	
				}
				buffMemoryIndex += 1;	
			}				
			// termination conditioning for while loop	
			endIndex += 2;	
		} // while loop ends		
		
		if (endIndex == data.len()- 2)
		{
			return (None, PARSERUTILS_NEEDDATA);
		}		

		if (buffMemory.len() ==(str::len(~"UTF-16BE")) && memcmp(&buffMemory, UTF16BE, buffMemory.len()) == 0) ||
			(buffMemory.len() == (str::len(~"UTF-16")) && memcmp(&buffMemory, UTF16, buffMemory.len()) == 0) {

				charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16BE");
		}
	}// else if terminates
	(Some(charset) , PARSERUTILS_OK)
}

pub fn  try_ascii_compatible_charset(data : &~[u8], lpu_arc: arc::ARC<~lpu>) -> (Option<u16>, parserutils_error) {

	let mut charset : u16 = 0;
	let CHARSET : ~[u8] = ~[ '@' as u8, 'c' as u8, 'h' as u8, 'a' as u8 , 'r' as u8, 's' as u8, 'e' as u8, 't' as u8, ' ' as u8 , '\"'  as u8] ;

	if (data.len() <= CHARSET.len() ) {
		return (None, PARSERUTILS_NEEDDATA);
	}

	// Look for @charset, assuming ASCII-compatible source data 
	//if ( memcmp(data, CHARSET, CHARSET.len() ) == 0) 
	let retVal : int = memcmp(data, CHARSET, CHARSET.len());
	if (retVal == 0) 
	{
		let mut indexVal = CHARSET.len()-1;
		// Looking for "; at the end of charset declaration
		while (indexVal < data.len()) 
		{
			//if data[indexVal] == ('"' as u8) && data[indexVal+1] == (';' as u8) && indexVal < (data.len()-1)  
			if data[indexVal] == ('"' as u8) && data[indexVal+1] == (';' as u8) && indexVal < (data.len())  
			{
				
				break ;
			}
			indexVal = indexVal + 1 ;
		}
		// if this condition is true then, the input CSS file doesn't have anything except <charset>  string
		if indexVal == data.len() {
			return (None, PARSERUTILS_NEEDDATA);
		}
		// Convert to MIB enum 

		charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(str::from_bytes(data.slice(CHARSET.len(), data.len()-1)));

		// Any non-ASCII compatible charset must be ignored, as
		// we've just used an ASCII parser to read it. 
		if (charset == arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32LE") || 
			charset == arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16") ||
			charset == arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
		{
			charset = 0;
		}
	}
	(Some(charset),PARSERUTILS_OK)
}

pub fn css_charset_read_bom_or_charset(data : &~[u8], lpu_arc: arc::ARC<~lpu>)
 -> (Option<u16>, parserutils_error) {

	//let mut err : parserutils_error ;
	let mut charset : u16  = 0;
	//let mut parser : @lpu = lpu();

	if (data.len()<4) {
		return (None, PARSERUTILS_BADPARAM);
	}

	// Look for BOM 
	if (data[0] == 0x00 && data[1] == 0x00 && 
			data[2] == 0xFE && data[3] == 0xFF) {
		charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32BE");
	} else if (data[0] == 0xFF && data[1] == 0xFE &&
			data[2] == 0x00 && data[3] == 0x00) {
		charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-32LE");
	} else if (data[0] == 0xFE && data[1] == 0xFF) {
		charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16BE");
	} else if (data[0] == 0xFF && data[1] == 0xFE) {
		charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-16LE");
	} else if (data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF) {
		charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-8");
	}

	if (charset!=0) {
		return (Some(charset), PARSERUTILS_OK);
	}
	
	let (option_return , err): (Option<u16>, parserutils_error) = try_utf32_charset(data, lpu_arc.clone());
	match(err) {
		PARSERUTILS_OK => return (option_return , err) ,
		_ => {}	
	}

	let (option_return , err): (Option<u16>, parserutils_error) = try_utf16_charset(data, lpu_arc.clone());
	match(err) {
		PARSERUTILS_OK => return (option_return , err) ,
		_ => {}	
	}
	
	try_ascii_compatible_charset(data, lpu_arc.clone())
}

pub fn css__charset_extract(data : &~[u8] ,	mibenum : u16 , source : css_charset_source, lpu_arc: arc::ARC<~lpu>)
	-> (Option<u16>, Option<css_charset_source>, parserutils_error) {

	let mut charset : u16 = 0;
	let mut src :css_charset_source;

	if (data.len()==(0 as uint))  || mibenum==(0 as u16){
		return (None ,None, PARSERUTILS_BADPARAM);
	}

	/*match source {
		//CSS_CHARSET_DEFAULT => return (None ,None, PARSERUTILS_BADPARAM),
		CSS_CHARSET_DEFAULT => return (Some(mibenum) ,Some(source), PARSERUTILS_OK),
		_ => {}
	}*/

	// If the charset was dictated by the client, we've nothing to detect 
	match (source)  {
		CSS_CHARSET_DICTATED => {
			charset=mibenum ;
			return (Some(charset), Some(CSS_CHARSET_DICTATED), PARSERUTILS_OK);
		}
		_ => {}
	}

	// Look for a BOM and/or @charset 
	let (option_return , err): (Option<u16>, parserutils_error) = css_charset_read_bom_or_charset(data, lpu_arc.clone());
	match(err) {
		PARSERUTILS_OK => {} ,
		_ => {
           
			return (None, None, PARSERUTILS_BADPARAM);
		}
	}

	if charset!=0 {
		//mibenum = charset;
		src = CSS_CHARSET_DOCUMENT ; // CSS_CHARSET_DOCUMENT;
		return (Some(charset), Some(src), PARSERUTILS_OK);
	}

	// If we've already got a charset from the linking mechanism or 
	//  referring document, then we've nothing further to do 
	match (source) {
		CSS_CHARSET_DEFAULT => {},
		_ => {
			src= CSS_CHARSET_DEFAULT;
			return (Some(charset), Some(src), PARSERUTILS_OK);
		}
	}
	
	// We've not yet found a charset, so use the default fallback 
	charset = arc::get(&lpu_arc).parserutils_charset_mibenum_from_name(~"UTF-8");

	if charset==0 {
		
		return (None, None, PARSERUTILS_BADENCODING) ;
	}

	src = CSS_CHARSET_DEFAULT ; // CSS_CHARSET_DEFAULT;
	(Some(charset) , Some(src) , PARSERUTILS_OK)
}
