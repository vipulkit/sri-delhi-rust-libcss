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


pub fn dump_sheet(sheet: @mut css_stylesheet) -> ~str {
    
    debug!("Entering: dump_sheet");

    
    unsafe {
        // debug!("Entering: unsafe");
        // // debug!(fmt!("sheet.selectors == %?" , sheet.selectors));
        // debug!(fmt!("sheet.rule_count == %?" , sheet.rule_count));
        // // debug!(fmt!("sheet.last_rule == %?" , sheet.last_rule));
        // debug!(fmt!("sheet.disabled == %?" , sheet.disabled));
        // debug!(fmt!("sheet.url == %?" , sheet.url));
        // debug!(fmt!("sheet.title == %?" , sheet.title));
        // debug!(fmt!("sheet.level == %?" , sheet.level));
        // debug!(fmt!("sheet.quirks_allowed == %?" , sheet.quirks_allowed));
        // debug!(fmt!("sheet.quirks_used == %?" , sheet.quirks_used));
        // debug!(fmt!("sheet.inline_style == %?" , sheet.inline_style));
        // debug!(fmt!("sheet.cached_style == %?" , sheet.cached_style));
        // debug!(fmt!("sheet.string_vector == %?" , sheet.string_vector));
        // debug!(fmt!("sheet.resolve == %?" , sheet.resolve));
        // debug!(fmt!("sheet.import == %?" , sheet.import));
        // debug!(fmt!("sheet.font == %?" , sheet.font));
        // debug!(fmt!("sheet.color == %?" , sheet.color));
        
        // debug!(fmt!("sheet.rule_list == %?" , sheet.rule_list));
    }
    let mut rule: Option<CSS_RULE_DATA_TYPE> = sheet.rule_list ;
    let mut ptr: ~str = ~"";
    //debug!(fmt!("rule == %?" , rule));
    while rule.is_some() {
        //debug!(fmt!("rule == %?" , rule.unwrap()));
        match rule.unwrap() {

            RULE_SELECTOR(css_rule_selector_x)=>{
                dump_rule_selector(css_rule_selector_x, &mut ptr, 1);
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
                dump_rule_page(css_rule_page_x, &mut ptr);
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
    
    debug!(fmt!("ptr == %?" , ptr));

    ptr


}

fn dump_rule_selector(s:@mut css_rule_selector, ptr:&mut ~str, depth:u32){
    debug!("Entering: dump_rule_selector");
    let mut i = 0;

    ptr.push_char('|');
    while i < depth as uint {
        ptr.push_char(' ');
        i += 1;
    }
    
    unsafe { 
        for s.selectors.eachi |i , &sel| {
            dump_selector_list(sel, ptr);
            
            if !(i == s.selectors.len() - 1) {
                ptr.push_char(',');
                ptr.push_char(' ');
            }
        }
    }
    ptr.push_char('\n');
    if s.style.is_some() {
        dump_bytecode(s.style.unwrap() , ptr, depth +1);
    }

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_charset(s:@mut css_rule_charset, ptr:&mut ~str) {
    debug!("Entering: dump_rule_charset");
    str::push_str(ptr , &"| @charset(");
    str::push_str(ptr , copy s.encoding);
    ptr.push_char(')');
    ptr.push_char('\n');

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_import(s:@mut css_rule_import, ptr:&mut ~str){
    debug!("Entering: dump_rule_import");
    str::push_str(ptr , &"| @import url(");
    str::push_str(ptr, copy s.url);
    ptr.push_char('\n');

    debug!(fmt!("ptr == %?" , ptr));
}

// TODO
fn dump_rule_media(s:@mut css_rule_media, ptr: &mut ~str) {
    debug!("Entering: dump_rule_media");
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

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_page(s:@ mut css_rule_page, ptr:&mut ~str){
    debug!("Entering: dump_rule_page");
    str::push_str(ptr , &"| @page ");

    if s.selector.is_some() {
        dump_selector_list(s.selector.unwrap(), ptr);
    }

    ptr.push_char('\n');

    if s.style.is_some() {
        dump_bytecode(s.style.unwrap() , ptr, 2);
    }   

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_font_face(s:@mut css_rule_font_face, ptr:&mut ~str){
    debug!("Entering: dump_rule_font_face");
    str::push_str(ptr , &"| @font-face ");
    if s.font_face.is_some() {
        dump_font_face(s.font_face.unwrap(), ptr);
    }
    ptr.push_char('\n');

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_selector_list(list:@mut css_selector, ptr:&mut ~str){
    debug!("Entering: dump_selector_list");
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

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_selector(selector:@mut css_selector, ptr:&mut ~str){
    debug!("Entering: dump_selector");
    let mut d:~[@mut css_selector_detail] = copy selector.data;
    let mut iter:uint = 0;
    while iter < d.len() {
        dump_selector_detail(d[iter], ptr, (iter != (d.len() - 1)));
        iter += 1;
    }   

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_selector_detail(detail:@mut css_selector_detail, ptr: &mut ~str, detail_next:bool ) {
    debug!("Entering: dump_selector_detail");
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

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_bytecode(style:@mut css_style, ptr:&mut ~str, depth:u32 ){
    
    debug!("Entering: dump_bytecode");
    let mut bytecode = copy style.bytecode;
    let mut op: css_properties_e;
    let mut value: u32;
    let opcode_names = opcode_names();
    let mut iterator = 0;
    
    // for bytecode.each|&opv| {
    while iterator < bytecode.len() {
    
        let mut opv = bytecode[iterator];
        debug!(fmt!("iterator == %?" , iterator));
        debug!(fmt!("opv == %?" , opv));    
        debug!(fmt!("bytecode == %?" , bytecode));

        iterator += 1;
        op = getOpcode(opv);
        debug!(fmt!("op == %?" , op));    
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
            debug!(fmt!("dump_bytecode:: value == %?" , value));

            if op as int == CSS_PROP_AZIMUTH as int {
                
                let val = (value & !(AZIMUTH_BEHIND as u32));

                if val as int == AZIMUTH_ANGLE as int {

                    let some_val: i32 = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit: u32 = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val , unit , ptr);

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

            else if (op as int == CSS_PROP_BORDER_TOP_COLOR as int || op as int == CSS_PROP_BORDER_RIGHT_COLOR as int 
            || op as int == CSS_PROP_BORDER_BOTTOM_COLOR as int || op as int == CSS_PROP_BORDER_LEFT_COLOR as int 
            || op as int == CSS_PROP_BACKGROUND_COLOR as int || op as int == CSS_PROP_COLUMN_RULE_COLOR as int) {

                assert!(BACKGROUND_COLOR_TRANSPARENT as int == BORDER_COLOR_TRANSPARENT as int);
                assert!(BACKGROUND_COLOR_CURRENT_COLOR as int == BORDER_COLOR_CURRENT_COLOR as int);
                assert!(BACKGROUND_COLOR_SET as int == BORDER_COLOR_SET as int);

                if value as int == BACKGROUND_COLOR_TRANSPARENT as int {
                    str::push_str(ptr , &"transparent");
                }

                else if value as int == BACKGROUND_COLOR_CURRENT_COLOR as int {
                    str::push_str(ptr , &"currentColor");   
                }
                else if value as int == BACKGROUND_COLOR_SET as int {
                    
                    let colour: u32 = bytecode[iterator];
                    iterator += 1;
                    let string = fmt!("#%08x" , colour as uint);
                    str::push_str(ptr , string);
                }
            }
            
            else if op as int == CSS_PROP_BACKGROUND_IMAGE as int || op as int == CSS_PROP_CUE_AFTER as int 
            || op as int == CSS_PROP_CUE_BEFORE as int || op as int == CSS_PROP_LIST_STYLE_IMAGE as int 
            || op as int == CSS_PROP_BACKGROUND_COLOR as int || op as int == CSS_PROP_COLUMN_RULE_COLOR as int {
                
                assert!(BACKGROUND_IMAGE_NONE as int == CUE_AFTER_NONE as int);
                assert!(BACKGROUND_IMAGE_URI as int == CUE_AFTER_URI as int);
                assert!(BACKGROUND_IMAGE_NONE as int == CUE_BEFORE_NONE as int);
                assert!(BACKGROUND_IMAGE_URI as int == CUE_BEFORE_URI as int);
                assert!(BACKGROUND_IMAGE_NONE as int == LIST_STYLE_IMAGE_NONE as int);
                assert!(BACKGROUND_IMAGE_URI as int == LIST_STYLE_IMAGE_URI as int);

                if value as int == BACKGROUND_IMAGE_NONE as int {
                    str::push_str(ptr , &"none");
                }

                else if value as int == BACKGROUND_IMAGE_URI as int {
                    
                    let snum = bytecode[iterator];

                    let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);
                    iterator += 1;

                    if option_string.is_some() {
                        str::push_str(ptr , &"url('");
                        str::push_str(ptr , option_string.unwrap());
                        str::push_str(ptr , &"')");    
                    }

                }
            }

            else if op as int == CSS_PROP_BACKGROUND_POSITION as int {

                let val = value & 0xf0;

                if val as int == BACKGROUND_POSITION_HORZ_SET as int {

                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val , unit , ptr);
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
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val , unit , ptr);
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
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val as i32 , unit , ptr);

                    let some_val = bytecode[iterator];
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val as i32 , unit , ptr);
                }
            }

            else if op as int == CSS_PROP_BORDER_TOP_STYLE as int || op as int == CSS_PROP_BORDER_RIGHT_STYLE as int || 
            op as int == CSS_PROP_BORDER_BOTTOM_STYLE as int || op as int == CSS_PROP_BORDER_LEFT_STYLE as int || 
            op as int == CSS_PROP_COLUMN_RULE_STYLE as int || op as int == CSS_PROP_OUTLINE_STYLE as int {

                assert!(BORDER_STYLE_NONE as int == OUTLINE_STYLE_NONE as int);
                assert!(BORDER_STYLE_NONE as int == COLUMN_RULE_STYLE_NONE as int);
                assert!(BORDER_STYLE_HIDDEN as int == OUTLINE_STYLE_HIDDEN as int);
                assert!(BORDER_STYLE_HIDDEN as int == COLUMN_RULE_STYLE_HIDDEN as int);
                assert!(BORDER_STYLE_DOTTED as int == OUTLINE_STYLE_DOTTED as int);
                assert!(BORDER_STYLE_DOTTED as int == COLUMN_RULE_STYLE_DOTTED as int);
                assert!(BORDER_STYLE_DASHED as int == OUTLINE_STYLE_DASHED as int);
                assert!(BORDER_STYLE_DASHED as int == COLUMN_RULE_STYLE_DASHED as int);
                assert!(BORDER_STYLE_SOLID as int == OUTLINE_STYLE_SOLID as int);
                assert!(BORDER_STYLE_SOLID as int == COLUMN_RULE_STYLE_SOLID as int);
                assert!(BORDER_STYLE_DOUBLE as int == OUTLINE_STYLE_DOUBLE as int);
                assert!(BORDER_STYLE_DOUBLE as int == COLUMN_RULE_STYLE_DOUBLE as int);
                assert!(BORDER_STYLE_GROOVE as int == OUTLINE_STYLE_GROOVE as int);
                assert!(BORDER_STYLE_GROOVE as int == COLUMN_RULE_STYLE_GROOVE as int);
                assert!(BORDER_STYLE_RIDGE as int == OUTLINE_STYLE_RIDGE as int);
                assert!(BORDER_STYLE_RIDGE as int == COLUMN_RULE_STYLE_RIDGE as int);
                assert!(BORDER_STYLE_INSET as int == OUTLINE_STYLE_INSET as int);
                assert!(BORDER_STYLE_INSET as int == COLUMN_RULE_STYLE_INSET as int);
                assert!(BORDER_STYLE_OUTSET as int == OUTLINE_STYLE_OUTSET as int);
                assert!(BORDER_STYLE_OUTSET as int == COLUMN_RULE_STYLE_OUTSET as int);

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

            else if op as int == CSS_PROP_BORDER_TOP_WIDTH as int || op as int == CSS_PROP_BORDER_RIGHT_WIDTH as int || 
            op as int == CSS_PROP_BORDER_BOTTOM_WIDTH as int || op as int == CSS_PROP_BORDER_LEFT_WIDTH as int || 
            op as int == CSS_PROP_COLUMN_RULE_WIDTH as int || op as int == CSS_PROP_OUTLINE_WIDTH as int {

                assert!(BORDER_WIDTH_SET as int == OUTLINE_WIDTH_SET as int);
                assert!(BORDER_WIDTH_THIN as int == OUTLINE_WIDTH_THIN as int);
                assert!(BORDER_WIDTH_MEDIUM as int == OUTLINE_WIDTH_MEDIUM as int);
                assert!(BORDER_WIDTH_THICK as int == OUTLINE_WIDTH_THICK as int);

                if value as int == BORDER_WIDTH_SET as int {
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val as i32 , unit , ptr);
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

            else if op as int == CSS_PROP_MARGIN_TOP as int || op as int == CSS_PROP_MARGIN_RIGHT as int || 
            op as int == CSS_PROP_MARGIN_BOTTOM as int || op as int == CSS_PROP_MARGIN_LEFT as int || 
            op as int == CSS_PROP_BOTTOM as int || op as int == CSS_PROP_LEFT as int || op as int == CSS_PROP_RIGHT as int || 
            op as int == CSS_PROP_TOP as int || op as int == CSS_PROP_HEIGHT as int || op as int == CSS_PROP_WIDTH as int || 
            op as int == CSS_PROP_COLUMN_WIDTH as int {

                assert!(BOTTOM_SET as int == LEFT_SET as int);
                assert!(BOTTOM_AUTO as int == LEFT_AUTO as int);
                assert!(BOTTOM_SET as int == RIGHT_SET as int);
                assert!(BOTTOM_AUTO as int == RIGHT_AUTO as int);
                assert!(BOTTOM_SET as int == TOP_SET as int);
                assert!(BOTTOM_AUTO as int == TOP_AUTO as int);
                assert!(BOTTOM_SET as int == HEIGHT_SET as int);
                assert!(BOTTOM_AUTO as int == HEIGHT_AUTO as int);
                assert!(BOTTOM_SET as int == MARGIN_SET as int);
                assert!(BOTTOM_AUTO as int == MARGIN_AUTO as int);
                assert!(BOTTOM_SET as int == WIDTH_SET as int);
                assert!(BOTTOM_AUTO as int == WIDTH_AUTO as int);
                assert!(BOTTOM_SET as int == COLUMN_WIDTH_SET as int);
                assert!(BOTTOM_AUTO as int == COLUMN_WIDTH_AUTO as int);

                if value as int == BOTTOM_SET as int {
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val as i32 , unit , ptr);
                }

                else if value as int == BOTTOM_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
            }

            else if op as int == CSS_PROP_BREAK_AFTER as int || op as int == CSS_PROP_BREAK_BEFORE as int {

                assert!(BREAK_AFTER_AUTO as int == BREAK_BEFORE_AUTO as int);
                assert!(BREAK_AFTER_ALWAYS as int == BREAK_BEFORE_ALWAYS as int);
                assert!(BREAK_AFTER_AVOID as int == BREAK_BEFORE_AVOID as int);
                assert!(BREAK_AFTER_LEFT as int == BREAK_BEFORE_LEFT as int);
                assert!(BREAK_AFTER_RIGHT as int == BREAK_BEFORE_RIGHT as int);
                assert!(BREAK_AFTER_PAGE as int == BREAK_BEFORE_PAGE as int);
                assert!(BREAK_AFTER_COLUMN as int == BREAK_BEFORE_COLUMN as int);
                assert!(BREAK_AFTER_AVOID_PAGE as int == BREAK_BEFORE_AVOID_PAGE as int);
                assert!(BREAK_AFTER_AVOID_COLUMN as int == BREAK_BEFORE_AVOID_COLUMN as int);

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
                
                if (value as int & CLIP_SHAPE_MASK as int) == CLIP_SHAPE_RECT as int {
                    str::push_str(ptr , &"rect(");

                    if (value as int & CLIP_RECT_TOP_AUTO as int) > 0 {

                        str::push_str(ptr , &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    str::push_str(ptr , &", ");

                    if (value as int & CLIP_RECT_RIGHT_AUTO as int) > 0 {

                        str::push_str(ptr , &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    str::push_str(ptr , &", ");

                    if (value as int & CLIP_RECT_BOTTOM_AUTO as int) > 0 {

                        str::push_str(ptr , &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    str::push_str(ptr , &", ");

                    if (value as int & CLIP_RECT_LEFT_AUTO as int) > 0 {

                        str::push_str(ptr , &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    ptr.push_char(')');
                }
                str::push_str(ptr , &"auto");
            }

            else if op as int == CSS_PROP_COLOR as int {
                
                if value as int == COLOR_TRANSPARENT as int {
                    str::push_str(ptr , &"transparent");
                }

                else if value as int == COLOR_CURRENT_COLOR as int {
                    str::push_str(ptr , &"currentColor");
                }

                else if value as int == COLOR_SET as int {
                    
                    let colour: u32 = bytecode[iterator];
                    iterator += 1;
                    let string = fmt!("#%08x" , colour as uint);
                    str::push_str(ptr , string);
                }
            }

            else if op as int == CSS_PROP_COLUMN_COUNT as int {
                
                if value as int == COLUMN_COUNT_SET as int {
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    dump_number(some_val as i32 , ptr);
                }

                else if value as int == COLUMN_COUNT_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
            }

            else if op as int == CSS_PROP_COLUMN_FILL as int {
                
                if value as int == COLUMN_FILL_BALANCE as int {
                    str::push_str(ptr , &"balance");
                }

                else if value as int == COLUMN_FILL_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
            }

            else if op as int == CSS_PROP_COLUMN_GAP as int {
                
                if value as int == COLUMN_GAP_SET as int {
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val as i32 , unit , ptr);
                }

                else if value as int == COLUMN_GAP_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
            }

            else if op as int == CSS_PROP_COLUMN_SPAN as int {
                
                if value as int == COLUMN_SPAN_NONE as int {
                    str::push_str(ptr , &"none");
                }

                else if value as int == COLUMN_SPAN_ALL as int {
                    str::push_str(ptr , &"all");
                }
            }

            else if op as int == CSS_PROP_CONTENT as int {
                
                while value as int != CONTENT_NORMAL as int {

                    let snum = bytecode[iterator];

                    if (value as int & 0xff) == CONTENT_COUNTER as int {

                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);
                        iterator += 1;

                        if option_string.is_some() {
                            dump_counter(option_string.unwrap() , value , ptr);
                        }
                    }
                    else if (value as int & 0xff) == CONTENT_COUNTER as int {

                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);
                        iterator += 1;
                        let sep = bytecode[iterator];
                        iterator += 1;

                        if option_string.is_some() {
                            dump_counters(option_string.unwrap() , fmt!("%?" , sep) , value , ptr);
                        }
                    }
                    else if (value as int & 0xff) == CONTENT_URI as int || (value as int & 0xff) == CONTENT_ATTR as int || (value as int & 0xff) == CONTENT_STRING as int {

                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                        if value as int == CONTENT_URI as int {
                            str::push_str(ptr , &"url(");
                        }
                        if value as int == CONTENT_ATTR as int {
                            str::push_str(ptr , &"attr(");
                        }
                        if value as int == CONTENT_STRING as int {
                            str::push_str(ptr , &")");
                        }

                        iterator += 1;

                        if option_string.is_some() {
                            ptr.push_char('\'');
                            str::push_str(ptr , option_string.unwrap());
                        }
                    }
                    else if (value as int & 0xff) == CONTENT_OPEN_QUOTE as int {
                        str::push_str(ptr , "open-quote");
                    }
                    else if (value as int & 0xff) == CONTENT_CLOSE_QUOTE as int {
                        str::push_str(ptr , "close-quote");
                    }
                    else if (value as int & 0xff) == CONTENT_NO_OPEN_QUOTE as int {
                        str::push_str(ptr , "no-open-quote");
                    }
                    else if (value as int & 0xff) == CONTENT_NO_CLOSE_QUOTE as int {
                        str::push_str(ptr , "no-close-quote");
                    }
                    value = bytecode[iterator];
                    iterator += 1;

                    if value as int != CONTENT_NORMAL as int {
                        ptr.push_char(' ');
                    }
                } // end while
            }

            else if op as int == CSS_PROP_COUNTER_INCREMENT as int || op as int == CSS_PROP_COUNTER_RESET as int {
                
                assert!(COUNTER_INCREMENT_NONE as int == COUNTER_RESET_NONE as int);
                assert!(COUNTER_INCREMENT_NAMED as int == COUNTER_RESET_NAMED as int);

                if value as int == COUNTER_INCREMENT_NAMED as int {

                    while value as int != COUNTER_INCREMENT_NONE as int {
                        let snum = bytecode[iterator];
                        
                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                        iterator += 1;
                        
                        if option_string.is_some() {
                            str::push_str(ptr , option_string.unwrap());
                        }
                        ptr.push_char(' ');
                        let val = bytecode[iterator] as i32;
                        iterator += 1;
                        dump_number(val , ptr);

                        value = bytecode[iterator] as u32;
                        iterator += 1;
                        
                        if value as int != COUNTER_INCREMENT_NONE as int {
                            ptr.push_char(' ');
                        }
                    }
                }

                else if value as int == COUNTER_INCREMENT_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_CURSOR as int {
                
                while value as int == CURSOR_URI as int {
                    let snum = bytecode[iterator];
                    iterator += 1;
                    let(_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                    if option_string.is_some() {
                        str::push_str(ptr , &"url('");
                        str::push_str(ptr , option_string.unwrap());
                        str::push_str(ptr , &"'), ");
                    }

                    value = bytecode[iterator];
                    iterator += 1;
                }

                if value as int == CURSOR_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
                else if value as int == CURSOR_CROSSHAIR as int {
                    str::push_str(ptr , &"crosshair");
                }
                else if value as int == CURSOR_DEFAULT as int {
                    str::push_str(ptr , &"default");
                }
                else if value as int == CURSOR_POINTER as int {
                    str::push_str(ptr , &"pointer");
                }
                else if value as int == CURSOR_MOVE as int {
                    str::push_str(ptr , &"move");
                }
                else if value as int == CURSOR_E_RESIZE as int {
                    str::push_str(ptr , &"e-resize");
                }
                else if value as int == CURSOR_NE_RESIZE as int {
                    str::push_str(ptr , &"ne-resize");
                }
                else if value as int == CURSOR_NW_RESIZE as int {
                    str::push_str(ptr , &"nw-resize");
                }
                else if value as int == CURSOR_N_RESIZE as int {
                    str::push_str(ptr , &"n-resize");
                }
                else if value as int == CURSOR_SE_RESIZE as int {
                    str::push_str(ptr , &"se-resize");
                }
                else if value as int == CURSOR_SW_RESIZE as int {
                    str::push_str(ptr , &"sw-resize");
                }
                else if value as int == CURSOR_S_RESIZE as int {
                    str::push_str(ptr , &"s-resize");
                }
                else if value as int == CURSOR_W_RESIZE as int {
                    str::push_str(ptr , &"w-resize");
                }
                else if value as int == CURSOR_TEXT as int {
                    str::push_str(ptr , &"text");
                }
                else if value as int == CURSOR_WAIT as int {
                    str::push_str(ptr , &"wait");
                }
                else if value as int == CURSOR_HELP as int {
                    str::push_str(ptr , &"help");
                }
                else if value as int == CURSOR_PROGRESS as int {
                    str::push_str(ptr , &"progress");
                }
            }

            else if op as int == CSS_PROP_DIRECTION as int {

                if value as int == DIRECTION_LTR as int {
                    str::push_str(ptr , &"ltr");
                }
                else if value as int == DIRECTION_RTL as int {
                    str::push_str(ptr , &"rtl");
                }
            }

            else if op as int == CSS_PROP_DISPLAY as int {

                if value as int == DISPLAY_INLINE as int {
                    str::push_str(ptr , &"inline");
                }
                else if value as int == DISPLAY_BLOCK as int {
                    str::push_str(ptr , &"block");
                }
                else if value as int == DISPLAY_LIST_ITEM as int {
                    str::push_str(ptr , &"list-item");
                }
                else if value as int == DISPLAY_RUN_IN as int {
                    str::push_str(ptr , &"run-in");
                }
                else if value as int == DISPLAY_INLINE_BLOCK as int {
                    str::push_str(ptr , &"inline-block");
                }
                else if value as int == DISPLAY_TABLE as int {
                    str::push_str(ptr , &"table");
                }
                else if value as int == DISPLAY_INLINE_TABLE as int {
                    str::push_str(ptr , &"inline-table");
                }
                else if value as int == DISPLAY_TABLE_ROW_GROUP as int {
                    str::push_str(ptr , &"table-row-group");
                }
                else if value as int == DISPLAY_TABLE_HEADER_GROUP as int {
                    str::push_str(ptr , &"table-header-group");
                }
                else if value as int == DISPLAY_TABLE_FOOTER_GROUP as int {
                    str::push_str(ptr , &"table-footer-group");
                }
                else if value as int == DISPLAY_TABLE_ROW as int {
                    str::push_str(ptr , &"table-row");
                }
                else if value as int == DISPLAY_TABLE_COLUMN_GROUP as int {
                    str::push_str(ptr , &"table-column-group");
                }
                else if value as int == DISPLAY_TABLE_COLUMN as int {
                    str::push_str(ptr , &"table-column");
                }
                else if value as int == DISPLAY_TABLE_CELL as int {
                    str::push_str(ptr , &"table-cell");
                }
                else if value as int == DISPLAY_TABLE_CAPTION as int {
                    str::push_str(ptr , &"table-caption");
                }
                else if value as int == DISPLAY_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_ELEVATION as int {

                if value as int == ELEVATION_ANGLE as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == ELEVATION_BELOW as int {
                    str::push_str(ptr , &"below");
                }
                else if value as int == ELEVATION_LEVEL as int {
                    str::push_str(ptr , &"level");
                }
                else if value as int == ELEVATION_ABOVE as int {
                    str::push_str(ptr , &"above");
                }
                else if value as int == ELEVATION_HIGHER as int {
                    str::push_str(ptr , &"higher");
                }
                else if value as int == ELEVATION_LOWER as int {
                    str::push_str(ptr , &"lower");
                }
            }

            else if op as int == CSS_PROP_EMPTY_CELLS as int {

                if value as int == EMPTY_CELLS_SHOW as int {
                    str::push_str(ptr , &"show");
                }
                else if value as int == EMPTY_CELLS_HIDE as int {
                    str::push_str(ptr , &"hide");
                }
            }

            else if op as int == CSS_PROP_FLOAT as int {

                if value as int == FLOAT_LEFT as int {
                    str::push_str(ptr , &"left");
                }
                else if value as int == FLOAT_RIGHT as int {
                    str::push_str(ptr , &"right");
                }
                else if value as int == FLOAT_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_FONT_FAMILY as int {
                
                while value as int != FONT_FAMILY_END as int {

                    if value as int == FONT_FAMILY_STRING as int || value as int == FONT_FAMILY_IDENT_LIST as int {
                        let snum = bytecode[iterator];
                        iterator += 1;
                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);
                        
                        if option_string.is_some() {
                            ptr.push_char('\'');
                            str::push_str(ptr , option_string.unwrap());
                            ptr.push_char('\'');
                        }
                    }
                    else if value as int == FONT_FAMILY_SERIF as int {
                        str::push_str(ptr , &"serif");
                    }
                    else if value as int == FONT_FAMILY_SANS_SERIF as int {
                        str::push_str(ptr , &"sans-serif");
                    }
                    else if value as int == FONT_FAMILY_CURSIVE as int {
                        str::push_str(ptr , &"cursive");
                    }
                    else if value as int == FONT_FAMILY_FANTASY as int {
                        str::push_str(ptr , &"fantasy");
                    }
                    else if value as int == FONT_FAMILY_MONOSPACE as int {
                        str::push_str(ptr , &"monospace");
                    }

                    value = bytecode[iterator];
                    iterator += 1;

                    if value as int != FONT_FAMILY_END as int {
                        str::push_str(ptr , &", ");
                    }
                }
            }

            else if op as int == CSS_PROP_FONT_SIZE as int {

                if value as int == FONT_SIZE_DIMENSION as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == FONT_SIZE_XX_SMALL as int {
                    str::push_str(ptr , &"right");
                }
                else if value as int == FONT_SIZE_X_SMALL as int {
                    str::push_str(ptr , &"none");
                }
                else if value as int == FONT_SIZE_SMALL as int {
                    str::push_str(ptr , &"small");
                }
                else if value as int == FONT_SIZE_MEDIUM as int {
                    str::push_str(ptr , &"medium");
                }
                else if value as int == FONT_SIZE_LARGE as int {
                    str::push_str(ptr , &"large");
                }
                else if value as int == FONT_SIZE_X_LARGE as int {
                    str::push_str(ptr , &"x-large");
                }
                else if value as int == FONT_SIZE_XX_LARGE as int {
                    str::push_str(ptr , &"xx-large");
                }
                else if value as int == FONT_SIZE_LARGER as int {
                    str::push_str(ptr , &"larger");
                }
                else if value as int == FONT_SIZE_SMALLER as int {
                    str::push_str(ptr , &"smaller");
                }
            }

            else if op as int == CSS_PROP_FONT_STYLE as int {

                if value as int == FONT_STYLE_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
                else if value as int == FONT_STYLE_ITALIC as int {
                    str::push_str(ptr , &"italic");
                }
                else if value as int == FONT_STYLE_OBLIQUE as int {
                    str::push_str(ptr , &"oblique");
                }
            }

            else if op as int == CSS_PROP_FONT_VARIANT as int {

                if value as int == FONT_VARIANT_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
                else if value as int == FONT_VARIANT_SMALL_CAPS as int {
                    str::push_str(ptr , &"small-caps");
                }
            }

            else if op as int == CSS_PROP_FONT_WEIGHT as int {

                if value as int == FONT_WEIGHT_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
                else if value as int == FONT_WEIGHT_BOLD as int {
                    str::push_str(ptr , &"bold");
                }
                else if value as int == FONT_WEIGHT_BOLDER as int {
                    str::push_str(ptr , &"bolder");
                }
                else if value as int == FONT_WEIGHT_LIGHTER as int {
                    str::push_str(ptr , &"lighter");
                }
                else if value as int == FONT_WEIGHT_100 as int {
                    str::push_str(ptr , &"100");
                }
                else if value as int == FONT_WEIGHT_200 as int {
                    str::push_str(ptr , &"200");
                }
                else if value as int == FONT_WEIGHT_300 as int {
                    str::push_str(ptr , &"300");
                }
                else if value as int == FONT_WEIGHT_400 as int {
                    str::push_str(ptr , &"400");
                }
                else if value as int == FONT_WEIGHT_500 as int {
                    str::push_str(ptr , &"500");
                }
                else if value as int == FONT_WEIGHT_600 as int {
                    str::push_str(ptr , &"600");
                }
                else if value as int == FONT_WEIGHT_700 as int {
                    str::push_str(ptr , &"700");
                }
                else if value as int == FONT_WEIGHT_800 as int {
                    str::push_str(ptr , &"800");
                }
                else if value as int == FONT_WEIGHT_900 as int {
                    str::push_str(ptr , &"900");
                }
            }

            else if op as int == CSS_PROP_LETTER_SPACING as int || op as int == CSS_PROP_WORD_SPACING as int{

                assert!(LETTER_SPACING_SET as int == WORD_SPACING_SET as int);
                assert!(LETTER_SPACING_NORMAL as int == WORD_SPACING_NORMAL as int);

                if value as int == LETTER_SPACING_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == LETTER_SPACING_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
            }

            else if op as int == CSS_PROP_LINE_HEIGHT as int{

                if value as int == LINE_HEIGHT_NUMBER as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;

                    dump_number(some_val , ptr);
                }
                else if value as int == LINE_HEIGHT_DIMENSION as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == LINE_HEIGHT_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
            }

            else if op as int == CSS_PROP_LIST_STYLE_POSITION as int{

                if value as int == LIST_STYLE_POSITION_INSIDE as int {
                    str::push_str(ptr , &"inside");
                }
                else if value as int == LIST_STYLE_POSITION_OUTSIDE as int {
                    str::push_str(ptr , &"outside");
                }
            }

            else if op as int == CSS_PROP_LIST_STYLE_TYPE as int{

                if value as int == LIST_STYLE_TYPE_DISC as int {
                    str::push_str(ptr , &"disc");
                }
                else if value as int == LIST_STYLE_TYPE_CIRCLE as int {
                    str::push_str(ptr , &"circle");
                }
                else if value as int == LIST_STYLE_TYPE_SQUARE as int {
                    str::push_str(ptr , &"square");
                }
                else if value as int == LIST_STYLE_TYPE_DECIMAL as int {
                    str::push_str(ptr , &"decimal");
                }
                else if value as int == LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as int {
                    str::push_str(ptr , &"decimal-leading-zero");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_ROMAN as int {
                    str::push_str(ptr , &"lower-roman");
                }
                else if value as int == LIST_STYLE_TYPE_UPPER_ROMAN as int {
                    str::push_str(ptr , &"upper-roman");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_GREEK as int {
                    str::push_str(ptr , &"lower-greek");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_LATIN as int {
                    str::push_str(ptr , &"lower-latin");
                }
                else if value as int == LIST_STYLE_TYPE_UPPER_LATIN as int {
                    str::push_str(ptr , &"upper-latin");
                }
                else if value as int == LIST_STYLE_TYPE_ARMENIAN as int {
                    str::push_str(ptr , &"armenian");
                }
                else if value as int == LIST_STYLE_TYPE_GEORGIAN as int {
                    str::push_str(ptr , &"georgian");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_ALPHA as int {
                    str::push_str(ptr , &"lower-alpha");
                }
                else if value as int == LIST_STYLE_TYPE_UPPER_ALPHA as int {
                    str::push_str(ptr , &"upper-alpha");
                }
                else if value as int == LIST_STYLE_TYPE_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_MAX_HEIGHT as int || op as int == CSS_PROP_MAX_WIDTH as int{

                assert!(MAX_HEIGHT_SET as int == MAX_WIDTH_SET as int);
                assert!(MAX_HEIGHT_NONE as int == MAX_WIDTH_NONE as int);
                
                if value as int == MAX_HEIGHT_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == MAX_HEIGHT_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_OPACITY as int{

                if value as int == OPACITY_SET as int {
                   
                   let some_val = bytecode[iterator] as i32;
                   iterator += 1;

                   dump_number(some_val , ptr);
                }
            }

            else if op as int == CSS_PROP_PADDING_TOP as int || op as int == CSS_PROP_PADDING_RIGHT as int
            || op as int == CSS_PROP_PADDING_BOTTOM as int || op as int == CSS_PROP_PADDING_LEFT as int || op as int == CSS_PROP_MIN_HEIGHT as int || op as int == CSS_PROP_MIN_WIDTH as int 
            || op as int == CSS_PROP_PAUSE_AFTER as int || op as int == CSS_PROP_PAUSE_BEFORE as int || op as int == CSS_PROP_TEXT_INDENT as int{

                assert!(MIN_HEIGHT_SET as int == MIN_WIDTH_SET as int);
                assert!(MIN_HEIGHT_SET as int == PADDING_SET as int);
                assert!(MIN_HEIGHT_SET as int == PAUSE_AFTER_SET as int);
                assert!(MIN_HEIGHT_SET as int == PAUSE_BEFORE_SET as int);
                assert!(MIN_HEIGHT_SET as int == TEXT_INDENT_SET as int);

                if value as int == MIN_HEIGHT_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
            }

            else if op as int == CSS_PROP_ORPHANS as int || op as int == CSS_PROP_PITCH_RANGE as int
            || op as int == CSS_PROP_RICHNESS as int || op as int == CSS_PROP_STRESS as int || op as int == CSS_PROP_WIDOWS as int {

                assert!(ORPHANS_SET as int == PITCH_RANGE_SET as int);
                assert!(ORPHANS_SET as int == RICHNESS_SET as int);
                assert!(ORPHANS_SET as int == STRESS_SET as int);
                assert!(ORPHANS_SET as int == WIDOWS_SET as int);

                if value as int == ORPHANS_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;

                    dump_number(some_val , ptr);
                }
            }

            else if op as int == CSS_PROP_OUTLINE_COLOR as int {

                if value as int == OUTLINE_COLOR_TRANSPARENT as int {
                    str::push_str(ptr , &"transparent");
                }
                else if value as int == OUTLINE_COLOR_CURRENT_COLOR as int {
                    str::push_str(ptr , &"currentColor");
                }
                else if value as int == OUTLINE_COLOR_SET as int {
                    
                    let colour = bytecode[iterator];
                    iterator += 1;

                    let string = fmt!("#%08x" , colour as uint);
                    str::push_str(ptr , string);
                }
            }

            else if op as int == CSS_PROP_OVERFLOW as int {

                if value as int == OVERFLOW_VISIBLE as int {
                    str::push_str(ptr , &"visible");
                }
                else if value as int == OVERFLOW_HIDDEN as int {
                    str::push_str(ptr , &"hidden");
                }
                else if value as int == OVERFLOW_SCROLL as int {
                    str::push_str(ptr , &"scroll");
                }
                else if value as int == OVERFLOW_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
            }

            else if op as int == CSS_PROP_PAGE_BREAK_AFTER as int || op as int == CSS_PROP_PAGE_BREAK_BEFORE as int {
                
                assert!(PAGE_BREAK_AFTER_AUTO as int == PAGE_BREAK_BEFORE_AUTO as int);
                assert!(PAGE_BREAK_AFTER_ALWAYS as int == PAGE_BREAK_BEFORE_ALWAYS as int);
                assert!(PAGE_BREAK_AFTER_AVOID as int == PAGE_BREAK_BEFORE_AVOID as int);
                assert!(PAGE_BREAK_AFTER_LEFT as int == PAGE_BREAK_BEFORE_LEFT as int);
                assert!(PAGE_BREAK_AFTER_RIGHT as int == PAGE_BREAK_BEFORE_RIGHT as int);

                if value as int == PAGE_BREAK_AFTER_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
                else if value as int == PAGE_BREAK_AFTER_ALWAYS as int {
                    str::push_str(ptr , &"always");
                }
                else if value as int == PAGE_BREAK_AFTER_AVOID as int {
                    str::push_str(ptr , &"avoid");
                }
                else if value as int == PAGE_BREAK_AFTER_LEFT as int {
                    str::push_str(ptr , &"left");
                }
                else if value as int == PAGE_BREAK_AFTER_RIGHT as int {
                    str::push_str(ptr , &"right");
                }
            }

            else if op as int == CSS_PROP_PAGE_BREAK_INSIDE as int{

                if value as int == PAGE_BREAK_INSIDE_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
                else if value as int == PAGE_BREAK_INSIDE_AVOID as int {
                    str::push_str(ptr , &"avoid");
                }
            }

            else if op as int == CSS_PROP_PITCH as int{

                if value as int == PITCH_FREQUENCY as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;

                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == PITCH_X_LOW as int {
                    str::push_str(ptr , &"x-low");
                }
                else if value as int == PITCH_LOW as int {
                    str::push_str(ptr , &"low");
                }
                else if value as int == PITCH_MEDIUM as int {
                    str::push_str(ptr , &"medium");
                }
                else if value as int == PITCH_HIGH as int {
                    str::push_str(ptr , &"high");
                }
                else if value as int == PITCH_X_HIGH as int {
                    str::push_str(ptr , &"x-high");
                }
            }

            else if op as int == CSS_PROP_PLAY_DURING as int{

                if value as int == PLAY_DURING_URI as int {
                    
                    let snum = bytecode[iterator];
                    iterator += 1;
                    let(_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                    if option_string.is_some() {
                        ptr.push_char('\'');
                        str::push_str(ptr , option_string.unwrap());
                        ptr.push_char('\'');
                    }
                }
                else if value as int == PLAY_DURING_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
                else if value as int == PLAY_DURING_NONE as int {
                    str::push_str(ptr , &"none");
                }
                
                if (value as int & PLAY_DURING_MIX as int > 0) {
                    str::push_str(ptr , &"mix");
                }

                if (value as int & PLAY_DURING_REPEAT as int > 0) {
                    str::push_str(ptr , &"repeat");
                }
            }

            else if op as int == CSS_PROP_POSITION as int{

                if value as int == POSITION_STATIC as int {
                    str::push_str(ptr , &"static");
                }
                else if value as int == POSITION_RELATIVE as int {
                    str::push_str(ptr , &"relative");
                }
                else if value as int == POSITION_ABSOLUTE as int {
                    str::push_str(ptr , &"absolute");
                }
                else if value as int == POSITION_FIXED as int {
                    str::push_str(ptr , &"fixed");
                }
            }

            // TODO review
            else if op as int == CSS_PROP_QUOTES as int{

                if value as int == QUOTES_STRING as int {
                    
                    while value as int != QUOTES_NONE as int {
                        
                        let snum = bytecode[iterator];
                        iterator += 1;
                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                        if option_string.is_some() {
                            str::push_str(ptr , &" '");
                            str::push_str(ptr , option_string.unwrap());
                            str::push_str(ptr , &"' ");
                        }

                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                        if option_string.is_some() {
                            str::push_str(ptr , &" '");
                            str::push_str(ptr , option_string.unwrap());
                            str::push_str(ptr , &"' ");
                        }
                        iterator += 1;
                        value = bytecode[iterator];
                        iterator += 1;
                    }
                }
                else if value as int == QUOTES_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_SPEAK_HEADER as int{

                if value as int == SPEAK_HEADER_ONCE as int {
                    str::push_str(ptr , &"once");
                }
                else if value as int == SPEAK_HEADER_ALWAYS as int {
                    str::push_str(ptr , &"always");
                }
            }

            else if op as int == CSS_PROP_SPEAK_NUMERAL as int{

                if value as int == SPEAK_NUMERAL_DIGITS as int {
                    str::push_str(ptr , &"digits");
                }
                else if value as int == SPEAK_NUMERAL_CONTINUOUS as int {
                    str::push_str(ptr , &"continuous");
                }
            }

            else if op as int == CSS_PROP_SPEAK_PUNCTUATION as int{

                if value as int == SPEAK_PUNCTUATION_CODE as int {
                    str::push_str(ptr , &"code");
                }
                else if value as int == SPEAK_PUNCTUATION_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_SPEAK as int{

                if value as int == SPEAK_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
                else if value as int == SPEAK_NONE as int {
                    str::push_str(ptr , &"none");
                }
                else if value as int == SPEAK_SPELL_OUT as int {
                    str::push_str(ptr , &"spell-out");
                }
            }

            else if op as int == CSS_PROP_SPEECH_RATE as int{

                if value as int == SPEECH_RATE_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    dump_number(some_val , ptr);
                }
                else if value as int == SPEECH_RATE_X_SLOW as int {
                    str::push_str(ptr , &"x-slow");
                }
                else if value as int == SPEECH_RATE_SLOW as int {
                    str::push_str(ptr , &"slow");
                }
                else if value as int == SPEECH_RATE_MEDIUM as int {
                    str::push_str(ptr , &"medium");
                }
                else if value as int == SPEECH_RATE_FAST as int {
                    str::push_str(ptr , &"fast");
                }
                else if value as int == SPEECH_RATE_X_FAST as int {
                    str::push_str(ptr , &"x-fast");
                }
                else if value as int == SPEECH_RATE_FASTER as int {
                    str::push_str(ptr , &"faster");
                }
                else if value as int == SPEECH_RATE_SLOWER as int {
                    str::push_str(ptr , &"slower");
                }
            }

            else if op as int == CSS_PROP_TABLE_LAYOUT as int{

                if value as int == TABLE_LAYOUT_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
                else if value as int == TABLE_LAYOUT_FIXED as int {
                    str::push_str(ptr , &"fixed");
                }
            }

            else if op as int == CSS_PROP_TEXT_ALIGN as int{

                if value as int == TEXT_ALIGN_LEFT as int {
                    str::push_str(ptr , &"left");
                }
                else if value as int == TEXT_ALIGN_RIGHT as int {
                    str::push_str(ptr , &"right");
                }
                else if value as int == TEXT_ALIGN_CENTER as int {
                    str::push_str(ptr , &"center");
                }
                else if value as int == TEXT_ALIGN_JUSTIFY as int {
                    str::push_str(ptr , &"justify");
                }
                else if value as int == TEXT_ALIGN_LIBCSS_LEFT as int {
                    str::push_str(ptr , &"-libcss-left");
                }
                else if value as int == TEXT_ALIGN_LIBCSS_CENTER as int {
                    str::push_str(ptr , &"-libcss-center");
                }
                else if value as int == TEXT_ALIGN_LIBCSS_RIGHT as int {
                    str::push_str(ptr , &"-libcss-right");
                }
            }

            else if op as int == CSS_PROP_TEXT_DECORATION as int{

                if value as int == TEXT_DECORATION_NONE as int {
                    str::push_str(ptr , &"none");
                }
                if value as int == TEXT_DECORATION_UNDERLINE as int {
                    str::push_str(ptr , &" underline");
                }
                if value as int == TEXT_DECORATION_OVERLINE as int {
                    str::push_str(ptr , &" overline");
                }
                if value as int == TEXT_DECORATION_LINE_THROUGH as int {
                    str::push_str(ptr , &" line-through");
                }
                if value as int == TEXT_DECORATION_BLINK as int {
                    str::push_str(ptr , &"blink");
                }
            }

            else if op as int == CSS_PROP_TEXT_TRANSFORM as int{

                if value as int == TEXT_TRANSFORM_CAPITALIZE as int {
                    str::push_str(ptr , &"capitalize");
                }
                else if value as int == TEXT_TRANSFORM_UPPERCASE as int {
                    str::push_str(ptr , &"uppercase");
                }
                else if value as int == TEXT_TRANSFORM_LOWERCASE as int {
                    str::push_str(ptr , &"lowercase");
                }
                else if value as int == TEXT_TRANSFORM_NONE as int {
                    str::push_str(ptr , &"none");
                }
            }

            else if op as int == CSS_PROP_UNICODE_BIDI as int{

                if value as int == UNICODE_BIDI_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
                else if value as int == UNICODE_BIDI_EMBED as int {
                    str::push_str(ptr , &"embed");
                }
                else if value as int == UNICODE_BIDI_BIDI_OVERRIDE as int {
                    str::push_str(ptr , &"bidi-override");
                }
            }

            else if op as int == CSS_PROP_VERTICAL_ALIGN as int{

                if value as int == VERTICAL_ALIGN_SET as int {
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val as i32 , unit , ptr);
                }
                else if value as int == VERTICAL_ALIGN_BASELINE as int {
                    str::push_str(ptr , &"baseline");
                }
                else if value as int == VERTICAL_ALIGN_SUB as int {
                    str::push_str(ptr , &"sub");
                }
                else if value as int == VERTICAL_ALIGN_SUPER as int {
                    str::push_str(ptr , &"super");
                }
                else if value as int == VERTICAL_ALIGN_TOP as int {
                    str::push_str(ptr , &"top");
                }
                else if value as int == VERTICAL_ALIGN_TEXT_TOP as int {
                    str::push_str(ptr , &"text-top");
                }
                else if value as int == VERTICAL_ALIGN_MIDDLE as int {
                    str::push_str(ptr , &"middle");
                }
                else if value as int == VERTICAL_ALIGN_BOTTOM as int {
                    str::push_str(ptr , &"bottom");
                }
                else if value as int == VERTICAL_ALIGN_TEXT_BOTTOM as int {
                    str::push_str(ptr , &"text-bottom");
                }
            }

            else if op as int == CSS_PROP_VOICE_FAMILY as int {
                
                while value as int != VOICE_FAMILY_END as int {

                    if value as int == VOICE_FAMILY_STRING as int || value as int == VOICE_FAMILY_IDENT_LIST as int {
                        let snum = bytecode[iterator];
                        iterator += 1;

                        let (_ , option_string) = style.sheet.unwrap().css__stylesheet_string_get(snum as uint);

                        if option_string.is_some() {
                            ptr.push_char('\'');
                            str::push_str(ptr , option_string.unwrap());
                            ptr.push_char('\'');
                        }
                    }
                    else if value as int == VOICE_FAMILY_MALE as int {
                        str::push_str(ptr , &"male");
                    }
                    else if value as int == VOICE_FAMILY_FEMALE as int {
                        str::push_str(ptr , &"female");
                    }
                    else if value as int == VOICE_FAMILY_CHILD as int {
                        str::push_str(ptr , &"child");
                    }

                    value = bytecode[iterator];
                    iterator += 1;

                    if value as int != VOICE_FAMILY_END as int {
                        str::push_str(ptr , &", ");
                    }
                }
            }

            else if op as int == CSS_PROP_VOLUME as int {
                
                if value as int == VOLUME_NUMBER as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    dump_number(some_val , ptr);
                }
                else if value as int == VOLUME_DIMENSION as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    let unit = bytecode[iterator];
                    iterator += 1;
                    dump_unit(some_val , unit , ptr);
                }
                else if value as int == VOLUME_SILENT as int {
                    str::push_str(ptr , &"silent");
                }
                else if value as int == VOLUME_X_SOFT as int {
                    str::push_str(ptr , &"x-soft");
                }
                else if value as int == VOLUME_SOFT as int {
                    str::push_str(ptr , &"soft");
                }
                else if value as int == VOLUME_MEDIUM as int {
                    str::push_str(ptr , &"medium");
                }
                else if value as int == VOLUME_LOUD as int {
                    str::push_str(ptr , &"loud");
                }
                else if value as int == VOLUME_X_LOUD as int {
                    str::push_str(ptr , &"loud");
                }
            }

            else if op as int == CSS_PROP_VOLUME as int {

                if value as int == WHITE_SPACE_NORMAL as int {
                    str::push_str(ptr , &"normal");
                }
                else if value as int == WHITE_SPACE_PRE as int {
                    str::push_str(ptr , &"pre");
                }
                else if value as int == WHITE_SPACE_NOWRAP as int {
                    str::push_str(ptr , &"nowrap");
                }
                else if value as int == WHITE_SPACE_PRE_WRAP as int {
                    str::push_str(ptr , &"pre-wrap");
                }
                else if value as int == WHITE_SPACE_PRE_LINE as int {
                    str::push_str(ptr , &"pre-line");
                }
            }

            else if op as int == CSS_PROP_Z_INDEX as int {

                if value as int == Z_INDEX_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    dump_number(some_val , ptr);
                }
                else if value as int == Z_INDEX_AUTO as int {
                    str::push_str(ptr , &"auto");
                }
            }

            else {
                let string = fmt!("Unknown opcode %x" , op as uint);
                str::push_str(ptr , string);
            }

            if (isImportant(opv)) {
                str::push_str(ptr , &" !important")
            }
            ptr.push_char('\n');

        }
    }

    debug!(fmt!("ptr == %?" , ptr));

}

fn dump_number(val: i32 , ptr: &mut ~str){
    debug!("Entering: dump_number");
    if css_int_to_fixed((val >> 10) as int) == val {
        str::push_str(ptr , fmt!("%?" , val >> 10));
    }
    else {
        dump_css_fixed(val , ptr);
    }

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_css_fixed(a: i32 , ptr: &mut ~str){
    debug!("Entering: dump_css_fixed");
    let b: u32;
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
    
    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_unit(val: i32 , unit: u32 , ptr: &mut ~str) {
    debug!("Entering: dump_unit");
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

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_font_face(font_face: @mut css_font_face, ptr: &mut ~str){
    debug!("Entering: dump_font_face");
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

    debug!(fmt!("ptr == %?" , ptr));

}

fn dump_counter(name: ~str , value: u32 , ptr: &mut ~str) {
    debug!("Entering: dump_counter");
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

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_counters(name: ~str , separator: ~str , value: u32 , ptr: &mut ~str) {

    debug!("Entering: dump_counters");
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

    debug!(fmt!("ptr == %?" , ptr));
}
