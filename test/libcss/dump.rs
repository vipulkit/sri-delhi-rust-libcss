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

/**
 * Opcode names, indexed by opcode
 */
// pub static opcode_names: ~[~str] = ~[
pub fn opcode_names() -> ~[~str] {

	~[
		~"azimuth",
		~"background-attachment",
		~"background-color",
		~"background-image",
		~"background-position",
		~"background-repeat",
		~"border-collapse",
		~"border-spacing",
		~"border-top-color",
		~"border-right-color",
		~"border-bottom-color",
		~"border-left-color",
		~"border-top-style",
		~"border-right-style",
		~"border-bottom-style",
		~"border-left-style",
		~"border-top-width",
		~"border-right-width",
		~"border-bottom-width",
		~"border-left-width",
		~"bottom",
		~"caption-side",
		~"clear",
		~"clip",
		~"color",
		~"content",
		~"counter-increment",
		~"counter-reset",
		~"cue-after",
		~"cue-before",
		~"cursor",
		~"direction",
		~"display",
		~"elevation",
		~"empty-cells",
		~"float",
		~"font-family",
		~"font-size",
		~"font-style",
		~"font-variant",
		~"font-weight",
		~"height",
		~"left",
		~"letter-spacing",
		~"line-height",
		~"list-style-image",
		~"list-style-position",
		~"list-style-type",
		~"margin-top",
		~"margin-right",
		~"margin-bottom",
		~"margin-left",
		~"max-height",
		~"max-width",
		~"min-height",
		~"min-width",
		~"orphans",
		~"outline-color",
		~"outline-style",
		~"outline-width",
		~"overflow",
		~"padding-top",
		~"padding-right",
		~"padding-bottom",
		~"padding-left",
		~"page-break-after",
		~"page-break-before",
		~"page-break-inside",
		~"pause-after",
		~"pause-before",
		~"pitch-range",
		~"pitch",
		~"play-during",
		~"position",
		~"quotes",
		~"richness",
		~"right",
		~"speak-header",
		~"speak-numeral",
		~"speak-punctuation",
		~"speak",
		~"speech-rate",
		~"stress",
		~"table-layout",
		~"text-align",
		~"text-decoration",
		~"text-indent",
		~"text-transform",
		~"top",
		~"unicode-bidi",
		~"vertical-align",
		~"visibility",
		~"voice-family",
		~"volume",
		~"white-space",
		~"widows",
		~"width",
		~"word-spacing",
		~"z-index",
		~"opacity",
		~"break-after",
		~"break-before",
		~"break-inside",
		~"column-count",
		~"column-fill",
		~"column-gap",
		~"column-rule-color",
		~"column-rule-style",
		~"column-rule-width",
		~"column-span",
		~"column-width",
	]
}


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
	let mut op: css_properties_e = CSS_PROP_AZIMUTH; //initilaisation not needed
	let mut value: u32 = 0;
	let opcode_names = opcode_names();

	for bytecode.each|&opv| {
		op = getOpcode(opv);
		ptr.push_char('|');

		let mut i: u32 = 0;
		while i<depth {
			ptr.push_char(' ');
			i+=1;
		}
		str::push_str(ptr , opcode_names[op as int]);
		ptr.push_char(':');
		ptr.push_char(' ');
		
		if isInherit(opv) {
			str::push_str(ptr , &"inherit");
		}
		else {

			value = getValue(opv) as u32;

			if op as int == CSS_PROP_AZIMUTH as int {
				let val = (value & !(AZIMUTH_BEHIND as u32));

				if val as int == AZIMUTH_ANGLE as int {
					// TODO
				}
				else if val as int == AZIMUTH_LEFTWARDS as int {
					str::push_str(ptr , &"leftwards");
				}
				else if val as int == AZIMUTH_RIGHTWARDS as int {
					str::push_str(ptr , &"rightwards");
				}
				else if val as int == AZIMUTH_LEFT_SIDE as int {
					str::push_str(ptr , &"left-side");
				}
				else if val as int == AZIMUTH_FAR_LEFT as int {
					str::push_str(ptr , &"far-left");
				}
				else if val as int == AZIMUTH_LEFT as int {
					str::push_str(ptr , &"left");
				}
				else if val as int == AZIMUTH_CENTER_LEFT as int {
					str::push_str(ptr , &"center-left");
				}
				else if val as int == AZIMUTH_CENTER as int {
					str::push_str(ptr , &"center");
				}
				else if val as int == AZIMUTH_CENTER_RIGHT as int {
					str::push_str(ptr , &"center-right");
				}
				else if val as int == AZIMUTH_RIGHT as int {
					str::push_str(ptr , &"right");
				}
				else if val as int == AZIMUTH_FAR_RIGHT as int {
					str::push_str(ptr , &"far-right");
				}
				else if val as int == AZIMUTH_RIGHT_SIDE as int {
					str::push_str(ptr , &"right-side");
				}

				if (value & (AZIMUTH_BEHIND as u32) > 0) {
					str::push_str(ptr , &"behind");
				}
			}

			else if op as int == CSS_PROP_BACKGROUND_ATTACHMENT as int {
				if value as int == BACKGROUND_ATTACHMENT_FIXED as int {
					str::push_str(ptr , &"fixed");
				}
				else if value as int == BACKGROUND_ATTACHMENT_SCROLL as int {
					str::push_str(ptr , &"scroll");
				}
			}

			else if op as int == (CSS_PROP_BORDER_TOP_COLOR as int | CSS_PROP_BORDER_RIGHT_COLOR as int 
			| CSS_PROP_BORDER_BOTTOM_COLOR as int | CSS_PROP_BORDER_LEFT_COLOR as int 
			| CSS_PROP_BACKGROUND_COLOR as int | CSS_PROP_COLUMN_RULE_COLOR as int) {

				// TODO
				// assert!(BACKGROUND_COLOR_TRANSPARENT == )
				// assert!(BACKGROUND_COLOR_CURRENT_COLOR == )
				// assert!(BACKGROUND_COLOR_SET == )

				if value as int == BACKGROUND_COLOR_TRANSPARENT as int {
					str::push_str(ptr , &"transparent");
				}

				else if value as int == BACKGROUND_COLOR_CURRENT_COLOR as int {
					str::push_str(ptr , &"currentColor");	
				}
				// TODO
				// else if value as int == BACKGROUND_COLOR_SET as int {
				// 	str::push_str(ptr , &"currentColor");	
				// }
			}
			
			else if op as int == (CSS_PROP_BACKGROUND_IMAGE as int | CSS_PROP_CUE_AFTER as int 
			| CSS_PROP_CUE_BEFORE as int | CSS_PROP_LIST_STYLE_IMAGE as int 
			| CSS_PROP_BACKGROUND_COLOR as int | CSS_PROP_COLUMN_RULE_COLOR as int) {
				// some asserts

				if value as int == BACKGROUND_IMAGE_NONE as int {
					str::push_str(ptr , &"none");
				}

				else if value as int == BACKGROUND_IMAGE_URI as int {
					// TODO
				}
			}

			else if op as int == CSS_PROP_BACKGROUND_POSITION as int {

				let val = value & 0xf0;

				if val as int == BACKGROUND_POSITION_HORZ_SET as int {
					// TODO
				}

				else if val as int == BACKGROUND_POSITION_HORZ_CENTER as int {
					
					str::push_str(ptr , &"center");
				}


				else if val as int == BACKGROUND_POSITION_HORZ_RIGHT as int {
					
					str::push_str(ptr , &"right");
				}


				else if val as int == BACKGROUND_POSITION_HORZ_LEFT as int {
					
					str::push_str(ptr , &"left");
				}

				ptr.push_char(' ');

				let val = value & 0x0f;

				if val as int == BACKGROUND_POSITION_VERT_SET as int {
					// TODO
				}

				else if val as int == BACKGROUND_POSITION_VERT_CENTER as int {
					
					str::push_str(ptr , &"center");
				}


				else if val as int == BACKGROUND_POSITION_VERT_BOTTOM as int {
					
					str::push_str(ptr , &"bottom");
				}


				else if val as int == BACKGROUND_POSITION_VERT_TOP as int {
					
					str::push_str(ptr , &"top");
				}
			}

			else if op as int == CSS_PROP_BACKGROUND_REPEAT as int {

				if value as int == BACKGROUND_REPEAT_NO_REPEAT as int {
					str::push_str(ptr , &"no-repeat");
				}

				else if value as int == BACKGROUND_REPEAT_REPEAT_X as int {
					str::push_str(ptr , &"repeat-x");
				}

				else if value as int == BACKGROUND_REPEAT_REPEAT_Y as int {
					str::push_str(ptr , &"repeat-y");
				}

				else if value as int == BACKGROUND_REPEAT_REPEAT as int {
					str::push_str(ptr , &"repeat-repeat");
				}
			}

			else if op as int == CSS_PROP_BORDER_COLLAPSE as int {
				
				if value as int == BORDER_COLLAPSE_SEPARATE as int {
					str::push_str(ptr , &"separate");
				}

				else if value as int == BORDER_COLLAPSE_COLLAPSE as int {
					str::push_str(ptr , &"collapse");
				}
			}

			else if op as int == CSS_PROP_BORDER_SPACING as int {

				if value as int == BORDER_SPACING_SET as int {
					// TODO
				}
			}

			else if op as int == (CSS_PROP_BORDER_TOP_STYLE as int | CSS_PROP_BORDER_RIGHT_STYLE as int | 
			CSS_PROP_BORDER_BOTTOM_STYLE as int | CSS_PROP_BORDER_LEFT_STYLE as int | 
			CSS_PROP_COLUMN_RULE_STYLE as int | CSS_PROP_OUTLINE_STYLE as int) {

				// TODO some asserts

				if value as int == BORDER_STYLE_NONE as int {
					str::push_str(ptr , &"none");
				}

				else if value as int == BORDER_STYLE_HIDDEN as int {
					str::push_str(ptr , &"hidden");
				}

				else if value as int == BORDER_STYLE_DOTTED as int {
					str::push_str(ptr , &"dotted");
				}

				else if value as int == BORDER_STYLE_DASHED as int {
					str::push_str(ptr , &"dashed");
				}

				else if value as int == BORDER_STYLE_SOLID as int {
					str::push_str(ptr , &"solid");
				}

				else if value as int == BORDER_STYLE_DOUBLE as int {
					str::push_str(ptr , &"double");
				}

				else if value as int == BORDER_STYLE_GROOVE as int {
					str::push_str(ptr , &"groove");
				}

				else if value as int == BORDER_STYLE_RIDGE as int {
					str::push_str(ptr , &"ridge");
				}

				else if value as int == BORDER_STYLE_INSET as int {
					str::push_str(ptr , &"inset");
				}

				else if value as int == BORDER_STYLE_OUTSET as int {
					str::push_str(ptr , &"outset");
				}
			}

			else if op as int == (CSS_PROP_BORDER_TOP_WIDTH as int | CSS_PROP_BORDER_RIGHT_WIDTH as int | 
			CSS_PROP_BORDER_BOTTOM_WIDTH as int | CSS_PROP_BORDER_LEFT_WIDTH as int | 
			CSS_PROP_COLUMN_RULE_WIDTH as int | CSS_PROP_OUTLINE_WIDTH as int) {

				// TODO asserts

				if value as int == BORDER_WIDTH_SET as int {
					// TODO
				}

				else if value as int == BORDER_WIDTH_THIN as int {
					str::push_str(ptr , &"thin");
				}

				else if value as int == BORDER_WIDTH_MEDIUM as int {
					str::push_str(ptr , &"medium");
				}

				else if value as int == BORDER_WIDTH_THICK as int {
					str::push_str(ptr , &"thick");
				}
			}

			else if op as int == (CSS_PROP_MARGIN_TOP as int | CSS_PROP_MARGIN_RIGHT as int | 
			CSS_PROP_MARGIN_BOTTOM as int | CSS_PROP_MARGIN_LEFT as int | 
			CSS_PROP_BOTTOM as int | CSS_PROP_LEFT as int | CSS_PROP_RIGHT as int | 
			CSS_PROP_TOP as int | CSS_PROP_HEIGHT as int | CSS_PROP_WIDTH as int | 
			CSS_PROP_COLUMN_WIDTH as int) {

				// TODO asserts

				if value as int == BOTTOM_SET as int {
					// TODO
				}

				else if value as int == BOTTOM_AUTO as int {
					str::push_str(ptr , &"auto");
				}
			}

			else if op as int == (CSS_PROP_BREAK_AFTER as int | CSS_PROP_BREAK_BEFORE as int) {

				// TODO Some asserts

				if value as int == BREAK_AFTER_AUTO as int {
					str::push_str(ptr , &"auto");
				}

				else if value as int == BREAK_AFTER_ALWAYS as int {
					str::push_str(ptr , &"always");
				}

				else if value as int == BREAK_AFTER_AVOID as int {
					str::push_str(ptr , &"avoid");
				}

				else if value as int == BREAK_AFTER_LEFT as int {
					str::push_str(ptr , &"left");
				}

				else if value as int == BREAK_AFTER_RIGHT as int {
					str::push_str(ptr , &"right");
				}

				else if value as int == BREAK_AFTER_PAGE as int {
					str::push_str(ptr , &"page");
				}

				else if value as int == BREAK_AFTER_COLUMN as int {
					str::push_str(ptr , &"column");
				}

				else if value as int == BREAK_AFTER_AVOID_PAGE as int {
					str::push_str(ptr , &"avoid-page");
				}

				else if value as int == BREAK_AFTER_AVOID_COLUMN as int {
					str::push_str(ptr , &"avoid-column");
				}
			}

			else if op as int == CSS_PROP_BREAK_INSIDE as int {
				
				if value as int == BREAK_INSIDE_AUTO as int {
					str::push_str(ptr , &"auto");
				}

				else if value as int == BREAK_INSIDE_AVOID as int {
					str::push_str(ptr , &"avoid");
				}

				else if value as int == BREAK_INSIDE_AVOID_PAGE as int {
					str::push_str(ptr , &"avoid-page");
				}

				else if value as int == BREAK_INSIDE_AVOID_COLUMN as int {
					str::push_str(ptr , &"avoid-column");
				}
			}

			else if op as int == CSS_PROP_CAPTION_SIDE as int {
				
				if value as int == CAPTION_SIDE_TOP as int {
					str::push_str(ptr , &"top");
				}

				else if value as int == CAPTION_SIDE_BOTTOM as int {
					str::push_str(ptr , &"bottom");
				}

			}

			else if op as int == CSS_PROP_CLEAR as int {
				
				if value as int == CLEAR_NONE as int {
					str::push_str(ptr , &"none");
				}

				else if value as int == CLEAR_LEFT as int {
					str::push_str(ptr , &"left");
				}

				else if value as int == CLEAR_RIGHT as int {
					str::push_str(ptr , &"right");
				}

				else if value as int == CLEAR_BOTH as int {
					str::push_str(ptr , &"both");
				}
			}

			else if op as int == CSS_PROP_CLIP as int {
				
				// TODO
			}
			// TODO

		}
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
