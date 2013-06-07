#[link(name = "testutils",vers = "0.1")];
#[crate_type = "lib"];

extern mod css;
extern mod wapcaplet;

use css::stylesheet::*;
use css::parse::font_face::*;
use css::bytecode::bytecode::*;
use css::include::font_face::*;
use css::include::properties::*;
use wapcaplet::*;

pub static CSS_RADIX_POINT : int = 10 ;
pub static INT_MIN : int = int::min_value ;
pub static INT_MAX : int = int::max_value ;

pub fn dump_sheet(sheet: &css_stylesheet) -> ~str {
	
	let mut buf: ~str = ~"";
	let mut rule: Option<CSS_RULE_DATA_TYPE> = sheet.rule_list ;
	
	while rule.is_some() {
		match rule.unwrap() {

    		RULE_SELECTOR(css_rule_selector_x)=>{
    			// dump_rule_selector(css_rule_selector_x,	buf, 1);
				rule = css_rule_selector_x.base.next;
    		},
    		RULE_CHARSET(css_rule_charset_x)=>{
    			buf = str::append(buf , dump_rule_charset(css_rule_charset_x));
				rule = css_rule_charset_x.base.next;
    		},
    		RULE_IMPORT(css_rule_import_x)=>{
    			buf = str::append(buf , dump_rule_import(css_rule_import_x));
				rule = css_rule_import_x.base.next;
    		},
    		RULE_MEDIA(css_rule_media_x)=>{
    			// dump_rule_media(css_rule_media_x, buf);
				rule = css_rule_media_x.base.next;
    		},
    		RULE_FONT_FACE(css_rule_font_face_x)=>{
    			buf = str::append(buf , dump_rule_font_face(css_rule_font_face_x));
				rule = css_rule_font_face_x.base.next;
    		},
    		RULE_PAGE(css_rule_page_x)=>{
				// dump_rule_page(css_rule_page_x,	buf);
				rule = css_rule_page_x.base.next; 
    		},
    		RULE_UNKNOWN(css_rule_x)=>{
    			buf = str::append(buf , ~"Unhandled rule type ");
    			// buf = str::append(buf , fmt!("%d" , rule.unwrap() as int));
    			buf.push_char('\n');
    			rule = css_rule_x.next;
    		}
		}
	}
	buf
}

fn dump_rule_selector(s:@mut css_rule_selector, depth:u32)->~str {
	let mut i:u32=0;
	let mut iter :uint =0 ;
	let mut buf :~str= ~"";
	buf.push_char('|');
	while iter < depth as uint {
		buf.push_char(' ');
		iter += 1;
	}
	iter = 0;
	while i < s.base.index as u32 {
		buf += dump_selector_list(s.selectors[i]);
		if i != (s.base.index -1) as u32 {
			buf.push_char(',');
			buf.push_char(' ');
			
		}
		i += 1; 
	}
	buf.push_char('\n' );
	if s.style.is_some() {
		dump_bytecode(s.style.unwrap(),&mut buf, depth +1);
	}
	return buf;
}

fn dump_rule_charset(s:@mut css_rule_charset) -> ~str {
	let mut buf = ~"";
	buf = str::append(buf , ~"| @charset(");
	unsafe{buf = str::append(buf , s.encoding);}
	buf.push_char(')');
	buf.push_char('\n');
	buf
}

fn dump_rule_import(s:@mut css_rule_import) -> ~str {
	let mut buf = ~"";
	buf = str::append(buf , ~" @import url(");
	unsafe{buf = str::append(buf , s.url);}
	buf.push_char('\n');
	buf
}

fn dump_rule_media(s:@mut css_rule_media) -> ~str{
~""
}

fn dump_rule_page(s:@ mut css_rule_page) -> ~str{
~""
}

fn dump_rule_font_face(s:@mut css_rule_font_face) -> ~str{
	let mut buf = ~"";
	buf = str::append(buf , ~"| @font-face ");
	if s.font_face.is_some() {
		buf = str::append(buf , dump_font_face(s.font_face.unwrap()));
	}
	buf.push_char('\n');
	buf
}

fn dump_selector_list(list:@mut css_selector/*, buf:&mut ~str*//*, index:&mut uint*/)->~str {
	let mut buf :~str = ~"";
	if list.combinator.is_some() {
		buf += dump_selector_list(list.combinator.unwrap()/*,buf*//*,index*/);
	}
	match list.data[0].combinator_type {//TODO
		CSS_COMBINATOR_NONE=> {
			
		},
    	CSS_COMBINATOR_ANCESTOR=>{
    		(&mut buf).push_char(' ' );
    		
    	},
    	CSS_COMBINATOR_PARENT=>{
    		(&mut buf).push_char(' ' );
    		(&mut buf).push_char('>' );
    		(&mut buf).push_char(' ' );
			

    	},
    	CSS_COMBINATOR_SIBLING=>{
    		(&mut buf).push_char(' ' );
    		(&mut buf).push_char('+' );
    		(&mut buf).push_char(' ' );
    	},
   		CSS_COMBINATOR_GENERIC_SIBLING=>{
   			(&mut buf).push_char(' ' );
    		(&mut buf).push_char('+' );
    		(&mut buf).push_char(' ' );
   		}

	}
	buf = str::append(buf, dump_selector(list/*, buf*/));
	buf
}
fn dump_selector(selector:@mut css_selector/*, buf:&mut ~str*//*, index:&mut uint*/)->~str {
	let mut buf:~str = ~"";
	let mut d:~[@mut css_selector_detail] = copy selector.data;
	let mut iter:uint = 0;
	while iter < d.len() {
		buf += dump_selector_detail(d[iter], /*buf,*/iter < d.len()/*, index*/);
		iter += 1;
	}	
	buf
}
fn dump_selector_detail(detail:@mut css_selector_detail, detail_next:bool )->~str {
	//let mut curr_pos:uint = 4 ;
	let mut buf :~str = ~"";
	if detail.negate {
		buf = str::append(buf,~":not(");
	}
	match detail.selector_type {
		CSS_SELECTOR_ELEMENT=>{
			unsafe{
				if detail.qname.name.len() == 1 && 
			   detail.qname.name[0] == '*' as u8 && !detail_next {
			   		buf = str::append(buf,detail.qname.name);
			   		
			   }
			   else if detail.qname.name.len() != 1 ||
			           detail.qname.name[0] != '*' as u8 { 
			           		buf = str::append(buf,detail.qname.name)
			   }
			}

		},
    	CSS_SELECTOR_CLASS=> {
    		(&mut buf).push_char('.' );
			let mut String:~str =copy detail.qname.name; 
			buf = str::append(buf,String);
    	},
    	CSS_SELECTOR_ID =>{
    		(&mut buf).push_char('#' );
			let mut String:~str =copy detail.qname.name; 
			buf = str::append(buf,String);
    	},
    	CSS_SELECTOR_PSEUDO_CLASS =>{
    		(&mut buf).push_char(':' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			match detail.value_type {
				CSS_SELECTOR_DETAIL_VALUE_STRING=> {
					if detail.string.is_some() {
						(&mut buf).push_char('(' );
						let mut String =copy (detail.string);
						buf = str::append(buf,String.unwrap());
						(&mut buf).push_char(')' );
					}
				} ,
				_=>{
					let mut string:~str = fmt!("%?n%?",copy detail.a,copy detail.b);
					buf = str::append(buf,string);
				}
			}
    	},
    	CSS_SELECTOR_PSEUDO_ELEMENT=> {
    		(&mut buf).push_char(':' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			match detail.value_type {
				CSS_SELECTOR_DETAIL_VALUE_STRING=> {
					if detail.string.is_some() {
						(&mut buf).push_char('(' );
						let mut String =copy (detail.string);
						buf = str::append(buf,String.unwrap());
						(&mut buf).push_char(')' );
					}
				} ,
				_=>{
					let string = fmt!("%?n%?",copy detail.a,copy detail.b);
					buf = str::append(buf,string);
				}
			}
    	},
    	CSS_SELECTOR_ATTRIBUTE=>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_EQUAL =>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char('=' );
			(&mut buf).push_char('"' );
			let mut String =copy (detail.string);
			buf = str::append(buf,String.unwrap());
			(&mut buf).push_char('"' );
			(&mut buf).push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_DASHMATCH=>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char('|' );
			(&mut buf).push_char('=' );
			(&mut buf).push_char('"' );
			let mut String = copy detail.string; 
			buf = str::append(buf,String.unwrap());
			(&mut buf).push_char('"' );
			(&mut buf).push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_INCLUDES=>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char('~' );
			(&mut buf).push_char('=' );
			(&mut buf).push_char('"' );
			let mut String = copy detail.string;
			buf = str::append(buf,String.unwrap());
			(&mut buf).push_char('"' );
			(&mut buf).push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_PREFIX=>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char('^' );
			(&mut buf).push_char('=' );
			(&mut buf).push_char('"' );
			let mut String = copy detail.string;
			buf = str::append(buf,String.unwrap());
			(&mut buf).push_char('"' );
			(&mut buf).push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_SUFFIX=>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char('$' );
			(&mut buf).push_char('=' );
			(&mut buf).push_char('"' );
			let mut String = copy detail.string;
			buf = str::append(buf,String.unwrap());
			(&mut buf).push_char('"' );
			(&mut buf).push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_SUBSTRING=>{
    		(&mut buf).push_char('[' );
			let mut String:~str =copy detail.qname.name;
			buf = str::append(buf,String);
			(&mut buf).push_char('*' );
			(&mut buf).push_char('=' );
			(&mut buf).push_char('"' );
			let mut String = copy detail.string;
			buf = str::append(buf,String.unwrap());
			(&mut buf).push_char('"' );
			(&mut buf).push_char(']' );
    	}
	}
	if detail.negate {
		(&mut buf).push_char(')' );
	}
	buf
}

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

fn dump_bytecode(style:@mut css_style,buf:&mut ~str, depth:u32 ) {

}

fn dump_number(val: i32 , buf: ~str) -> ~str {
	let mut ptr: ~str = ~"";
	if css_int_to_fixed((val >> 10) as int) == val {
		ptr = str::append(buf , fmt!("%?" , val >> 10));
	}
	else {
		ptr = str::append(ptr , dump_css_fixed(val , buf));
	}
	ptr
}

fn dump_css_fixed(a: i32 , buf: ~str) -> ~str {
    let b: u32;
    let mut ptr = buf;
    if a < 0 {
        b = -a as u32;
    }
    else {
        b = a as u32;
    }
    let mut unitpart:u32 = b >> 10;
    let mut fracpart:u32 = ((b & 0x3ff)*1000 + 500)/(1 << 10);
    let mut flen: uint = 0;
    let mut tmp: ~[char] = ~[];

    if a < 0 {
        ptr.push_char('-');
    }
    let string_number = ~"0123456789";

    loop {
        tmp.push(string_number[unitpart%10] as char);
        unitpart /= 10;
        if unitpart == 0 || tmp.len() >= 20 {
            break;    
        }
    }
    
    while (tmp.len() > 0) {
        ptr.push_char(tmp.pop());
    }
    ptr.push_char('.');
    loop {
        tmp.push(string_number[fracpart%10] as char);
        fracpart /= 10;
        if !(tmp.len() < 20 && fracpart != 0 ) {
            break;    
        }
    }

    while (tmp.len() > 0) {
        ptr.push_char(tmp.pop());
        flen += 1;
    }
    
    
    while flen < 3 {
        ptr.push_char('0');
        flen += 1;
    }
    
    return ptr;
}

fn dump_unit(val: i32 , unit: u32 , buf: ~str) -> ~str {
	let mut ptr = buf;
	ptr = dump_number(val , ptr);

	match unit {
		UNIT_PX => {
			ptr = str::append(ptr , ~"px");
		},
		UNIT_EX => {
			ptr = str::append(ptr , ~"ex");
		},
		UNIT_EM => {
			ptr = str::append(ptr , ~"em");
		},
		UNIT_IN => {
			ptr = str::append(ptr , ~"in");
		},
		UNIT_CM => {
			ptr = str::append(ptr , ~"cm");
		},
		UNIT_MM => {
			ptr = str::append(ptr , ~"mm");
		},
		UNIT_PT => {
			ptr = str::append(ptr , ~"pt");
		},
		UNIT_PC => {
			ptr = str::append(ptr , ~"pc");
		},
		UNIT_PCT => {
			ptr = str::append(ptr , ~"pct");
		},
		UNIT_DEG => {
			ptr = str::append(ptr , ~"deg");
		},
		UNIT_GRAD => {
			ptr = str::append(ptr , ~"grad");
		},
		UNIT_RAD => {
			ptr = str::append(ptr , ~"rad");
		},
		UNIT_MS => {
			ptr = str::append(ptr , ~"ms");
		},
		UNIT_S => {
			ptr = str::append(ptr , ~"s");
		},
		UNIT_HZ => {
			ptr = str::append(ptr , ~"Hz");
		},
		UNIT_KHZ => {
			ptr = str::append(ptr , ~"kHz");
		},
		_ => {}
	}
	ptr
}

fn dump_font_face(font_face: @mut css_font_face) -> ~str {

	let mut ptr = ~"";
	let mut style: u8;
	let mut weight: u8;

	if font_face.font_family.is_some() {
		ptr.push_char('\n');
		ptr = str::append(ptr , ~"| font_family: ");
		unsafe{ptr = str::append(ptr , lwc_string_data(font_face.font_family.get_ref().clone()));}
	}
	ptr = str::append(ptr , ~"\n| font-style: ");

	style = css_font_face_font_style(font_face) as u8;

	if style as int == CSS_FONT_STYLE_INHERIT as int {
		ptr = str::append(ptr , ~"unspecified");
	}
	else if style as int == CSS_FONT_STYLE_NORMAL as int {
		ptr = str::append(ptr , ~"normal");
	}
	else if style as int == CSS_FONT_STYLE_ITALIC as int {
		ptr = str::append(ptr , ~"italic");
	}
	else if style as int == CSS_FONT_STYLE_OBLIQUE as int {
		ptr = str::append(ptr , ~"oblique");
	}

	ptr = str::append(ptr , ~"\n| font-weight: ");

	weight = css_font_face_font_weight(font_face) as u8;

	if weight as int == CSS_FONT_WEIGHT_INHERIT as int {
		ptr = str::append(ptr , ~"unspecified");
	}
	else if weight as int == CSS_FONT_WEIGHT_NORMAL as int {
		ptr = str::append(ptr , ~"normal");
	}
	else if weight as int == CSS_FONT_WEIGHT_BOLD as int {
		ptr = str::append(ptr , ~"normal");
	}
	else if weight as int == CSS_FONT_WEIGHT_100 as int {
		ptr = str::append(ptr , ~"100");
	}
	else if weight as int == CSS_FONT_WEIGHT_200 as int {
		ptr = str::append(ptr , ~"200");
	}
	else if weight as int == CSS_FONT_WEIGHT_300 as int {
		ptr = str::append(ptr , ~"300");
	}
	else if weight as int == CSS_FONT_WEIGHT_400 as int {
		ptr = str::append(ptr , ~"400");
	}
	else if weight as int == CSS_FONT_WEIGHT_500 as int {
		ptr = str::append(ptr , ~"500");
	}
	else if weight as int == CSS_FONT_WEIGHT_600 as int {
		ptr = str::append(ptr , ~"600");
	}
	else if weight as int == CSS_FONT_WEIGHT_700 as int {
		ptr = str::append(ptr , ~"700");
	}
	else if weight as int == CSS_FONT_WEIGHT_800 as int {
		ptr = str::append(ptr , ~"800");
	}
	else if weight as int == CSS_FONT_WEIGHT_900 as int {
		ptr = str::append(ptr , ~"900");
	}
	else {
		ptr = str::append(ptr , ~"Unhandled weight");
		ptr = str::append(ptr , fmt!("%d" , weight as int));
		ptr.push_char('\n');
	}

	unsafe {
		if !font_face.srcs.is_empty() {
			for vec::each(font_face.srcs) |i|{
				ptr = str::append(copy ptr , ~"\n| src: ");
				let format = css_font_face_src_format(i);
				ptr = str::append(copy ptr , ~"\n| format: ");

				if format as int == CSS_FONT_FACE_FORMAT_UNSPECIFIED as int {
					ptr = str::append(copy ptr , ~"unspecified");
				}
				else if format as int == CSS_FONT_FACE_FORMAT_WOFF as int {
					ptr = str::append(copy ptr , ~"WOFF");
				}
				else if format as int == CSS_FONT_FACE_FORMAT_OPENTYPE as int {
					ptr = str::append(copy ptr , ~"OTF");
				}
				else if format as int == CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE as int {
					ptr = str::append(copy ptr , ~"EOTF");
				}
				else if format as int == CSS_FONT_FACE_FORMAT_SVG as int {
					ptr = str::append(copy ptr , ~"SVG");
				}
				else if format as int == CSS_FONT_FACE_FORMAT_UNKNOWN as int {
					ptr = str::append(copy ptr , ~"unknown");
				}
				else {
					ptr = str::append(copy ptr , ~"UNEXPECTED");
				}

				if i.location.is_some() {
					ptr = str::append(copy ptr , ~"\n| location: ");

					let location = css_font_face_src_location_type(i);
					
					if location as int == CSS_FONT_FACE_LOCATION_TYPE_LOCAL as int {
						ptr = str::append(copy ptr , ~"local");
					}
					else if location as int == CSS_FONT_FACE_LOCATION_TYPE_URI as int {
						ptr = str::append(copy ptr , ~"url");
					}
					else {
						ptr = str::append(copy ptr , ~"UNKNOWN");
					}

					ptr = str::append(copy ptr , lwc_string_data(i.location.get_ref().clone()));
				}

			}
		}
	}

	ptr
}