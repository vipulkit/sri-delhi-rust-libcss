#[link(name = "dump",vers = "0.1")];
#[crate_type = "lib"];

extern mod css;
extern mod wapcaplet;

use css::stylesheet::*;
use css::parse::font_face::*;
use css::bytecode::bytecode::*;
use css::bytecode::opcodes::*;
use css::include::font_face::*;
use css::include::fpmath::*;
use css::include::properties::*;
use wapcaplet::*;

pub fn dump_sheet(sheet: &css_stylesheet) -> ~str {
	
	let mut ptr: ~str = ~"";
	let mut rule: Option<CSS_RULE_DATA_TYPE> = sheet.rule_list ;
	
	while rule.is_some() {
		match rule.unwrap() {

    		RULE_SELECTOR(css_rule_selector_x)=>{
    			dump_rule_selector(css_rule_selector_x,	&mut ptr, 1);
				rule = css_rule_selector_x.base.next;
    		},
    		RULE_CHARSET(css_rule_charset_x)=>{
    			dump_rule_charset(css_rule_charset_x, &mut ptr);
				rule = css_rule_charset_x.base.next;
    		},
    		RULE_IMPORT(css_rule_import_x)=>{
    			dump_rule_import(css_rule_import_x, &mut ptr);
				rule = css_rule_import_x.base.next;
    		},
    		RULE_MEDIA(css_rule_media_x)=>{
    			dump_rule_media(css_rule_media_x, &mut ptr);
				rule = css_rule_media_x.base.next;
    		},
    		RULE_FONT_FACE(css_rule_font_face_x)=>{
    			dump_rule_font_face(css_rule_font_face_x, &mut ptr);
				rule = css_rule_font_face_x.base.next;
    		},
    		RULE_PAGE(css_rule_page_x)=>{
				dump_rule_page(css_rule_page_x,	&mut ptr);
				rule = css_rule_page_x.base.next; 
    		},
    		RULE_UNKNOWN(css_rule_x)=>{
    			ptr += &"Unhandled rule type ";
    			// add rule.type
    			ptr.push_char('\n');
    			rule = css_rule_x.next;
    		}
		}
	}
	ptr
}

fn dump_rule_selector(s:@mut css_rule_selector, ptr:&mut ~str, depth:u32){
	let mut i = 0;

	ptr.push_char('|');
	while i < depth as uint {
		ptr.push_char(' ');
		i += 1;
	}
	
	i = 0;
	while i < s.base.index {
		dump_selector_list(s.selectors[i], ptr);
		if i != (s.base.index -1) {
			ptr.push_char(',');
			ptr.push_char(' ');
		}
		i += 1; 
	}
	ptr.push_char('\n');
	if s.style.is_some() {
		dump_bytecode(s.style.unwrap() , ptr, depth +1);
	}
}

fn dump_rule_charset(s:@mut css_rule_charset, ptr:&mut ~str) {
	str::push_str(ptr , &"| @charset(");
	str::push_str(ptr , copy s.encoding);
	ptr.push_char(')');
	ptr.push_char('\n');
}

fn dump_rule_import(s:@mut css_rule_import, ptr:&mut ~str){
	str::push_str(ptr , &"| @import url(");
	str::push_str(ptr, copy s.url);
	ptr.push_char('\n');
}

// TODO
fn dump_rule_media(s:@mut css_rule_media, ptr: &mut ~str) {
	str::push_str(ptr, &"| @media ");
	ptr.push_char('\n');

	let mut rule = s.first_child;
	
	while rule.is_some() {
		let rule_type = rule.unwrap();
		match rule_type {
			RULE_SELECTOR(x) => {
				 dump_rule_selector(x, ptr, 2);
				 rule = x.base.next;
			},
			_ =>{
				fail!(~"Only selector type expected");
			}
		}
	}
}

fn dump_rule_page(s:@ mut css_rule_page, ptr:&mut ~str){
	str::push_str(ptr , &"| @page ");

	if s.selector.is_some() {
		dump_selector_list(s.selector.unwrap(), ptr);
	}

	ptr.push_char('\n');

	if s.style.is_some() {
		dump_bytecode(s.style.unwrap() , ptr, 2);
	}	
}

fn dump_rule_font_face(s:@mut css_rule_font_face, ptr:&mut ~str){
	str::push_str(ptr , &"| @font-face ");
	if s.font_face.is_some() {
		dump_font_face(s.font_face.unwrap(), ptr);
	}
	ptr.push_char('\n');
}

fn dump_selector_list(list:@mut css_selector, ptr:&mut ~str){
	if list.combinator.is_some() {
		dump_selector_list(list.combinator.unwrap(),ptr);
	}
	match list.data[0].combinator_type {
		CSS_COMBINATOR_NONE=> {
			
		},
    	CSS_COMBINATOR_ANCESTOR=>{
    		ptr.push_char(' ');
    		
    	},
    	CSS_COMBINATOR_PARENT=>{
    		ptr.push_char(' ');
    		ptr.push_char('>');
    		ptr.push_char(' ');
			

    	},
    	CSS_COMBINATOR_SIBLING=>{
    		ptr.push_char(' ');
    		ptr.push_char('+');
    		ptr.push_char(' ');
    	},
   		CSS_COMBINATOR_GENERIC_SIBLING=>{
   			ptr.push_char(' ');
    		ptr.push_char('+');
    		ptr.push_char(' ');
   		}

	}
	dump_selector(list, ptr);
}

fn dump_selector(selector:@mut css_selector, ptr:&mut ~str){
	let mut d:~[@mut css_selector_detail] = copy selector.data;
	let mut iter:uint = 0;
	while iter < d.len() {
		dump_selector_detail(d[iter], ptr, iter < d.len());
		iter += 1;
	}	
}

fn dump_selector_detail(detail:@mut css_selector_detail, ptr: &mut ~str, detail_next:bool ) {

	if detail.negate {
		str::push_str(ptr,&":not(");
	}
	match detail.selector_type {
		CSS_SELECTOR_ELEMENT=>{
			unsafe{
				if detail.qname.name.len() == 1 && detail.qname.name[0] == '*' as u8 && !detail_next {
			   	
			   		str::push_str(ptr,copy detail.qname.name);
			   	}
			   	else if detail.qname.name.len() != 1 ||
		        
		           detail.qname.name[0] != '*' as u8 { 
		           str::push_str(ptr,copy detail.qname.name)
			   	}
			}
		},

    	CSS_SELECTOR_CLASS=> {

    		ptr.push_char('.');
			str::push_str(ptr,copy detail.qname.name);
    	},

    	CSS_SELECTOR_ID =>{
    		
    		ptr.push_char('#');
			str::push_str(ptr,copy detail.qname.name);
    	},

    	CSS_SELECTOR_PSEUDO_CLASS | CSS_SELECTOR_PSEUDO_ELEMENT =>{
    		ptr.push_char(':' );
			str::push_str(ptr,copy detail.qname.name);
			match detail.value_type {
				CSS_SELECTOR_DETAIL_VALUE_STRING=> {
					if detail.string.is_some() {
						ptr.push_char('(' );
						//let String = copy detail.string;
						str::push_str(ptr, (copy detail.string).unwrap());
						ptr.push_char(')' );
					}
				} ,
				_=>{
					str::push_str(ptr,fmt!("%?n+%?",copy detail.a,copy detail.b));
				}
			}
    	},

    	CSS_SELECTOR_ATTRIBUTE=>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_EQUAL =>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('=');
			ptr.push_char('"');
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"');
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_DASHMATCH=>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('|');
			ptr.push_char('=');
			ptr.push_char('"');
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"');
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_INCLUDES=>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('~');
			ptr.push_char('=');
			ptr.push_char('"');
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"');
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_PREFIX=>{
    		ptr.push_char('[' );
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('^' );
			ptr.push_char('=' );
			ptr.push_char('"' );
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"' );
			ptr.push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_SUFFIX=>{
    		ptr.push_char('[' );
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('$' );
			ptr.push_char('=' );
			ptr.push_char('"' );
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"' );
			ptr.push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_SUBSTRING=>{
    		ptr.push_char('[' );
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('*' );
			ptr.push_char('=' );
			ptr.push_char('"' );
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"' );
			ptr.push_char(']' );
    	}
	}
	if detail.negate {
		ptr.push_char(')');
	}
}

fn dump_bytecode(style:@mut css_style, ptr:&mut ~str, depth:u32 ){
	
	let mut bytecode = copy style.bytecode;
	let mut length: uint = (style.used * 4);
	let mut offset: u32 = 0;
	let mut op: css_properties_e;

	while (offset as uint) < (length) {
		
	}

}

fn dump_number(val: i32 , ptr: &mut ~str){
	if css_int_to_fixed((val >> 10) as int) == val {
		str::push_str(ptr , fmt!("%?" , val >> 10));
	}
	else {
		dump_css_fixed(val , ptr);
	}
}

fn dump_css_fixed(a: i32 , ptr: &mut ~str){
    let b: u32;
    if a < 0 {
        b = -a as u32;
    }
    else {
        b = a as u32;
    }
    let mut unitpart:u32 = b >> 0;
    let mut fracpart:u32 = ((b & 0x3ff)*1000 + 500)/(1 << 10);
    let mut flen: uint = 0;
    let mut tmp: ~[char] = ~[];

    if a < 0 {
        ptr.push_char('-');
    }
    let string_number = &"0123456789";

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
    
}

fn dump_unit(val: i32 , unit: u32 , ptr: &mut ~str) {
	dump_number(val, ptr);

	match unit {
		UNIT_PX => {
			str::push_str(ptr , &"px");
		},
		UNIT_EX => {
			str::push_str(ptr , &"ex");
		},
		UNIT_EM => {
			str::push_str(ptr , &"em");
		},
		UNIT_IN => {
			str::push_str(ptr , &"in");
		},
		UNIT_CM => {
			str::push_str(ptr , &"cm");
		},
		UNIT_MM => {
			str::push_str(ptr , &"mm");
		},
		UNIT_PT => {
			str::push_str(ptr , &"pt");
		},
		UNIT_PC => {
			str::push_str(ptr , &"pc");
		},
		UNIT_PCT => {
			str::push_str(ptr , &"pct");
		},
		UNIT_DEG => {
			str::push_str(ptr , &"deg");
		},
		UNIT_GRAD => {
			str::push_str(ptr , &"grad");
		},
		UNIT_RAD => {
			str::push_str(ptr , &"rad");
		},
		UNIT_MS => {
			str::push_str(ptr , &"ms");
		},
		UNIT_S => {
			str::push_str(ptr , &"s");
		},
		UNIT_HZ => {
			str::push_str(ptr , &"Hz");
		},
		UNIT_KHZ => {
			str::push_str(ptr , &"kHz");
		},
		_ => {}
	}
}

fn dump_font_face(font_face: @mut css_font_face, ptr: &mut ~str){

	let mut style: u8;
	let mut weight: u8;

	if font_face.font_family.is_some() {
		ptr.push_char('\n');
		str::push_str(ptr , &"| font_family: ");
		unsafe{str::push_str(ptr , lwc_string_data(font_face.font_family.get_ref().clone()));}
	}
	str::push_str(ptr , &"\n| font-style: ");

	style = css_font_face_font_style(font_face) as u8;

	if style as int == CSS_FONT_STYLE_INHERIT as int {
		str::push_str(ptr , &"unspecified");
	}
	else if style as int == CSS_FONT_STYLE_NORMAL as int {
		str::push_str(ptr , &"normal");
	}
	else if style as int == CSS_FONT_STYLE_ITALIC as int {
		str::push_str(ptr , &"italic");
	}
	else if style as int == CSS_FONT_STYLE_OBLIQUE as int {
		str::push_str(ptr , &"oblique");
	}

	str::push_str(ptr , &"\n| font-weight: ");

	weight = css_font_face_font_weight(font_face) as u8;

	if weight as int == CSS_FONT_WEIGHT_INHERIT as int {
		str::push_str(ptr , &"unspecified");
	}
	else if weight as int == CSS_FONT_WEIGHT_NORMAL as int {
		str::push_str(ptr , &"normal");
	}
	else if weight as int == CSS_FONT_WEIGHT_BOLD as int {
		str::push_str(ptr , &"normal");
	}
	else if weight as int == CSS_FONT_WEIGHT_100 as int {
		str::push_str(ptr , &"100");
	}
	else if weight as int == CSS_FONT_WEIGHT_200 as int {
		str::push_str(ptr , &"200");
	}
	else if weight as int == CSS_FONT_WEIGHT_300 as int {
		str::push_str(ptr , &"300");
	}
	else if weight as int == CSS_FONT_WEIGHT_400 as int {
		str::push_str(ptr , &"400");
	}
	else if weight as int == CSS_FONT_WEIGHT_500 as int {
		str::push_str(ptr , &"500");
	}
	else if weight as int == CSS_FONT_WEIGHT_600 as int {
		str::push_str(ptr , &"600");
	}
	else if weight as int == CSS_FONT_WEIGHT_700 as int {
		str::push_str(ptr , &"700");
	}
	else if weight as int == CSS_FONT_WEIGHT_800 as int {
		str::push_str(ptr , &"800");
	}
	else if weight as int == CSS_FONT_WEIGHT_900 as int {
		str::push_str(ptr , &"900");
	}
	else {
		str::push_str(ptr , &"Unhandled weight");
		str::push_str(ptr , fmt!("%d" , weight as int));
		ptr.push_char('\n');
	}

	unsafe {
		for vec::each_mut(font_face.srcs) |i| {
			str::push_str(ptr , &"\n| src: ");
			let format = css_font_face_src_format(i);
			str::push_str(ptr , &"\n| format: ");

			if format as int == CSS_FONT_FACE_FORMAT_UNSPECIFIED as int {
				str::push_str(ptr , &"unspecified");
			}
			else if format as int == CSS_FONT_FACE_FORMAT_WOFF as int {
				str::push_str(ptr , &"WOFF");
			}
			else if format as int == CSS_FONT_FACE_FORMAT_OPENTYPE as int {
				str::push_str(ptr , &"OTF");
			}
			else if format as int == CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE as int {
				str::push_str(ptr , &"EOTF");
			}
			else if format as int == CSS_FONT_FACE_FORMAT_SVG as int {
				str::push_str(ptr , &"SVG");
			}
			else if format as int == CSS_FONT_FACE_FORMAT_UNKNOWN as int {
				str::push_str(ptr , &"unknown");
			}
			else {
				str::push_str(ptr , &"UNEXPECTED");
			}

			if i.location.is_some() {
				str::push_str(ptr , &"\n| location: ");

				let location = css_font_face_src_location_type(i);
				
				if location as int == CSS_FONT_FACE_LOCATION_TYPE_LOCAL as int {
					str::push_str(ptr , &"local");
				}
				else if location as int == CSS_FONT_FACE_LOCATION_TYPE_URI as int {
					str::push_str(ptr , &"url");
				}
				else {
					str::push_str(ptr , &"UNKNOWN");
				}

				str::push_str(ptr , lwc_string_data(i.location.get_ref().clone()));
			}

		}
	}

}

fn dump_counter(name: ~str , value: u32 , ptr: &mut ~str) {
	
	str::push_str(ptr , &"counter(");
	str::push_str(ptr , name);
	let val = value >> CONTENT_COUNTER_STYLE_SHIFT;

	if val as int == LIST_STYLE_TYPE_DISC as int {
		str::push_str(ptr , &", disc");
	}
	else if val as int == LIST_STYLE_TYPE_CIRCLE as int {
		str::push_str(ptr , &", circle");
	}
	else if val as int == LIST_STYLE_TYPE_SQUARE as int {
		str::push_str(ptr , &", square");
	}
	// else if (val as int == LIST_STYLE_TYPE_DECIMAL as int) {

	// }
	else if (val as int == LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as int) {
		str::push_str(ptr , &", decimal-leading-zero");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_ROMAN as int {
		str::push_str(ptr , &", lower-roman");
	}
	else if val as int == LIST_STYLE_TYPE_UPPER_ROMAN as int {
		str::push_str(ptr , &", upper-roman");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_GREEK as int {
		str::push_str(ptr , &", lower-greek");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_LATIN as int {
		str::push_str(ptr , &", lower-latin");
	}
	else if val as int == LIST_STYLE_TYPE_UPPER_LATIN as int {
		str::push_str(ptr , &", upper-latin");
	}
	else if val as int == LIST_STYLE_TYPE_ARMENIAN as int {
		str::push_str(ptr , &", armenian");
	}
	else if val as int == LIST_STYLE_TYPE_GEORGIAN as int {
		str::push_str(ptr , &", georgian");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_ALPHA as int {
		str::push_str(ptr , &", lower-alpha");
	}
	else if val as int == LIST_STYLE_TYPE_UPPER_ALPHA as int {
		str::push_str(ptr , &", upper-alpha");
	}
	else if val as int == LIST_STYLE_TYPE_NONE as int {
		str::push_str(ptr , &", none");
	}
	ptr.push_char(')');
}

fn dump_counters(name: ~str , separator: ~str , value: u32 , ptr: &mut ~str) {

	str::push_str(ptr , &"counter(");
	str::push_str(ptr , name);
	str::push_str(ptr , separator);
	let val = value >> CONTENT_COUNTER_STYLE_SHIFT;

	if val as int == LIST_STYLE_TYPE_DISC as int {
		str::push_str(ptr , &", disc");
	}
	else if val as int == LIST_STYLE_TYPE_CIRCLE as int {
		str::push_str(ptr , &", circle");
	}
	else if val as int == LIST_STYLE_TYPE_SQUARE as int {
		str::push_str(ptr , &", square");
	}
	// else if (val as int == LIST_STYLE_TYPE_DECIMAL as int) {}
	else if (val as int == LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as int) {
		str::push_str(ptr , &", decimal-leading-zero");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_ROMAN as int {
		str::push_str(ptr , &", lower-roman");
	}
	else if val as int == LIST_STYLE_TYPE_UPPER_ROMAN as int {
		str::push_str(ptr , &", upper-roman");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_GREEK as int {
		str::push_str(ptr , &", lower-greek");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_LATIN as int {
		str::push_str(ptr , &", lower-latin");
	}
	else if val as int == LIST_STYLE_TYPE_UPPER_LATIN as int {
		str::push_str(ptr , &", upper-latin");
	}
	else if val as int == LIST_STYLE_TYPE_ARMENIAN as int {
		str::push_str(ptr , &", armenian");
	}
	else if val as int == LIST_STYLE_TYPE_GEORGIAN as int {
		str::push_str(ptr , &", georgian");
	}
	else if val as int == LIST_STYLE_TYPE_LOWER_ALPHA as int {
		str::push_str(ptr , &", lower-alpha");
	}
	else if val as int == LIST_STYLE_TYPE_UPPER_ALPHA as int {
		str::push_str(ptr , &", upper-alpha");
	}
	else if val as int == LIST_STYLE_TYPE_NONE as int {
		str::push_str(ptr , &", none");
	}
	ptr.push_char(')');
}
