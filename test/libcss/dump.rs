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


pub fn dump_sheet(stylesheet_vector:&mut ~[css_stylesheet], sheet:uint, css_rule_data_list:&mut ~[~css_rule_data_type], lwc_ref:&mut ~lwc) -> ~str {
    
    debug!("Entering: dump_sheet");
    let reason = "Function dump_sheet";

    
    // debug!("Entering: unsafe");
    // // debug!(fmt!("stylesheet_vector[sheet].selectors == %?" , stylesheet_vector[sheet].selectors));
    // debug!(fmt!("stylesheet_vector[sheet].rule_count == %?" , stylesheet_vector[sheet].rule_count));
    // // debug!(fmt!("stylesheet_vector[sheet].last_rule == %?" , stylesheet_vector[sheet].last_rule));
    // debug!(fmt!("stylesheet_vector[sheet].disabled == %?" , stylesheet_vector[sheet].disabled));
    // debug!(fmt!("stylesheet_vector[sheet].url == %?" , stylesheet_vector[sheet].url));
    // debug!(fmt!("stylesheet_vector[sheet].title == %?" , stylesheet_vector[sheet].title));
    // debug!(fmt!("stylesheet_vector[sheet].level == %?" , stylesheet_vector[sheet].level));
    // debug!(fmt!("stylesheet_vector[sheet].quirks_allowed == %?" , stylesheet_vector[sheet].quirks_allowed));
    // debug!(fmt!("stylesheet_vector[sheet].quirks_used == %?" , stylesheet_vector[sheet].quirks_used));
    // debug!(fmt!("stylesheet_vector[sheet].inline_style == %?" , stylesheet_vector[sheet].inline_style));
    // debug!(fmt!("stylesheet_vector[sheet].cached_style == %?" , stylesheet_vector[sheet].cached_style));
    // debug!(fmt!("stylesheet_vector[sheet].string_vector == %?" , stylesheet_vector[sheet].string_vector));
    // debug!(fmt!("stylesheet_vector[sheet].resolve == %?" , stylesheet_vector[sheet].resolve));
    // debug!(fmt!("stylesheet_vector[sheet].import == %?" , stylesheet_vector[sheet].import));
    // debug!(fmt!("stylesheet_vector[sheet].font == %?" , stylesheet_vector[sheet].font));
    // debug!(fmt!("stylesheet_vector[sheet].color == %?" , stylesheet_vector[sheet].color));
    
    // debug!(fmt!("stylesheet_vector[sheet].rule_list == %?" , stylesheet_vector[sheet].rule_list));

    let mut rule: Option<uint> = stylesheet_vector[sheet].rule_list ;
    let mut ptr: ~str = ~"";
    //debug!(fmt!("rule == %?" , rule));
    while rule.is_some() {
        //debug!(fmt!("rule == %?" , rule.unwrap()));
        match css_rule_data_list[rule.expect("")].rule_type {

            CSS_RULE_SELECTOR=>{
                dump_rule_selector(stylesheet_vector, sheet,css_rule_data_list ,rule.expect(reason), lwc_ref, &mut ptr, 1);
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_selector.get_mut_ref().base].next;
            },
            CSS_RULE_CHARSET=>{
                dump_rule_charset(stylesheet_vector, sheet , css_rule_data_list,rule.expect(reason) , &mut ptr);
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_charset.get_mut_ref().base].next;
            },
            CSS_RULE_IMPORT=>{
                dump_rule_import(stylesheet_vector, sheet , css_rule_data_list,rule.expect(reason) , &mut ptr);
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_import.get_mut_ref().base].next;
            },
            CSS_RULE_MEDIA=>{
                dump_rule_media(stylesheet_vector, sheet , css_rule_data_list,rule.expect(reason) , lwc_ref,&mut ptr);
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_media.get_mut_ref().base].next;
            },
            CSS_RULE_FONT_FACE=>{
                dump_rule_font_face(stylesheet_vector, sheet , css_rule_data_list,rule.expect(reason), lwc_ref, &mut ptr);
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_font_face.get_mut_ref().base].next;
            },
            CSS_RULE_PAGE=>{
                dump_rule_page(stylesheet_vector, sheet ,css_rule_data_list, rule.expect(reason), lwc_ref, &mut ptr);
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_page.get_mut_ref().base].next; 
            },
            CSS_RULE_UNKNOWN=>{
                ptr = ptr + &"Unhandled rule type ";
                // add rule.type
                ptr.push_char('\n');
                rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule.expect("")].rule_unknown].next;
            }
        }
    }
    
    debug!(fmt!("ptr == %?" , ptr));

    ptr
}
fn dump_rule_selector(stylesheet_vector:&mut ~[css_stylesheet], sheet : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, lwc_ref:&mut ~lwc, ptr:&mut ~str, depth:u32){
    debug!("Entering: dump_rule_selector");
    let mut i = 0;

    ptr.push_char('|');
    while i < depth as uint {
        ptr.push_char(' ');
        i += 1;
    }
    
    i = 0;
    while i < css_rule_data_list[rule].rule_selector.get_mut_ref().selectors.len(){
        dump_selector_list(stylesheet_vector, sheet, css_rule_data_list , rule ,i, lwc_ref, ptr,false);
        if !(i == css_rule_data_list[rule].rule_selector.get_mut_ref().selectors.len() - 1) {
            ptr.push_char(',');
            ptr.push_char(' ');
        }
        i += 1;
    } 

    ptr.push_char('\n');
    if css_rule_data_list[rule].rule_selector.get_mut_ref().style.is_some() {
        debug!("Entering: dump_rule_selector :: if s.style.is_some()");
        dump_bytecode(stylesheet_vector, css_rule_data_list , rule ,lwc_ref, ptr,  depth +1);
    }

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_charset(_:&mut ~[css_stylesheet], _ : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, ptr:&mut ~str) {
    debug!("Entering: dump_rule_charset");
    ptr.push_str(&"| @charset(");
    ptr.push_str( css_rule_data_list[rule].rule_charset.get_mut_ref().encoding.clone());
    ptr.push_char(')');
    ptr.push_char('\n');

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_import(_:&mut ~[css_stylesheet], _ : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, ptr:&mut ~str){
    debug!("Entering: dump_rule_import");
    ptr.push_str( &"| @import url(");
    ptr.push_str( css_rule_data_list[rule].rule_import.get_mut_ref().url.clone());
    ptr.push_char('\n');

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_media(stylesheet_vector:&mut ~[css_stylesheet], sheet : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, lwc_ref:&mut ~lwc, ptr: &mut ~str) {
    debug!("Entering: dump_rule_media");
    ptr.push_str( &"| @media ");
    ptr.push_char('\n');

    let mut rule = css_rule_data_list[rule].rule_media.get_mut_ref().first_child;
    
    while rule.is_some() {
        let rule_index = rule.unwrap();
        match css_rule_data_list[rule_index].rule_type {
            CSS_RULE_SELECTOR => {
                 dump_rule_selector(stylesheet_vector, sheet, css_rule_data_list , rule_index, lwc_ref, ptr, 2);
                 rule = stylesheet_vector[sheet].css_rule_list[css_rule_data_list[rule_index].rule_selector.get_mut_ref().base].next;
            },
            _ =>{
                fail!(~"Only selector type expected");
            }
        }
    }

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_page(stylesheet_vector:&mut ~[css_stylesheet], sheet : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, lwc_ref:&mut ~lwc, ptr:&mut ~str){
    debug!("Entering: dump_rule_page");
    ptr.push_str( &"| @page ");

    if css_rule_data_list[rule].rule_page.get_mut_ref().selector.is_some() {
        let i = css_rule_data_list[rule].rule_page.get_mut_ref().selector.unwrap();
        dump_selector_list(stylesheet_vector, sheet, css_rule_data_list , rule, i,lwc_ref, ptr , false);
    }

    ptr.push_char('\n');

    if css_rule_data_list[rule].rule_page.get_mut_ref().style.is_some() {
        dump_bytecode(stylesheet_vector ,css_rule_data_list ,rule,lwc_ref, ptr, 2);
    }   

    // debug!(fmt!("ptr == %?" , ptr));
}

fn dump_rule_font_face(_:&mut ~[css_stylesheet], _ : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, lwc_ref:&mut ~lwc, ptr:&mut ~str){
    debug!("Entering: dump_rule_font_face");
    ptr.push_str( &"| @font-face ");
    if css_rule_data_list[rule].rule_font_face.get_mut_ref().font_face.is_some() {
        dump_font_face(css_rule_data_list[rule].rule_font_face.get_mut_ref().font_face.get_ref(), lwc_ref, ptr);
    }
    ptr.push_char('\n');

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_selector_list(stylesheet_vector:&mut ~[css_stylesheet], sheet : uint, css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, index: uint ,lwc_ref:&mut ~lwc, ptr:&mut ~str , flag:bool){
    debug!("Entering: dump_selector_list");
    let mut list: uint = 0;
    if (!flag)
    {
        if css_rule_data_list[rule].rule_type as int == CSS_RULE_SELECTOR as int
        {
             list = css_rule_data_list[rule].rule_selector.get_mut_ref().selectors[index];
        }
        else if css_rule_data_list[rule].rule_type as int == CSS_RULE_PAGE as int
        {
            list = index;

        }
    }
    else
    {
        list = index;
    }
    if stylesheet_vector[sheet].css_selectors_list[list].combinator.is_some() {
        dump_selector_list(stylesheet_vector, sheet, css_rule_data_list , rule,stylesheet_vector[sheet].css_selectors_list[list].combinator.unwrap(), lwc_ref, ptr , true);
    }
    match stylesheet_vector[sheet].css_selectors_list[list].data[0].combinator_type {
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
    dump_selector(&mut stylesheet_vector[sheet].css_selectors_list[list], lwc_ref, ptr);

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_selector(selector:&mut ~css_selector, lwc_ref:&mut ~lwc, ptr:&mut ~str){
    debug!("Entering: dump_selector");
    let d:&~[~css_selector_detail] = &selector.data;
    let mut iter:uint = 0;
    while iter < d.len() {
        dump_selector_detail(&d[iter], lwc_ref, ptr, (iter != (d.len() - 1)));
        iter += 1;
    }   

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_selector_detail(detail:&~css_selector_detail, lwc_ref:&mut ~lwc, ptr: &mut ~str, detail_next:bool ) {
    debug!("Entering: dump_selector_detail");
    if detail.negate {
        ptr.push_str(&":not(");
    }

    debug!("dump_selector_detail :: detail.selector_type == %?" , detail.selector_type);

    match detail.selector_type {
        CSS_SELECTOR_ELEMENT=>{
            debug!("Entering: CSS_SELECTOR_ELEMENT");
            if lwc_ref.lwc_string_length(detail.qname.name) == 1 && lwc_ref.lwc_string_data(detail.qname.name)[0] == '*' as u8 && !detail_next {
            
                ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            }
            else if lwc_ref.lwc_string_length(detail.qname.name) != 1 ||
            
               lwc_ref.lwc_string_data(detail.qname.name)[0] != '*' as u8 { 
               ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name))
            }
        },

        CSS_SELECTOR_CLASS=> {

            ptr.push_char('.');
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
        },

        CSS_SELECTOR_ID =>{
            
            ptr.push_char('#');
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
        },

        CSS_SELECTOR_PSEUDO_CLASS | CSS_SELECTOR_PSEUDO_ELEMENT =>{
            ptr.push_char(':' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            match detail.value_type {
                CSS_SELECTOR_DETAIL_VALUE_STRING=> {
                    if detail.string.is_some() {
                        ptr.push_char('(' );
                        //let String = copy detail.string;
                        ptr.push_str( lwc_ref.lwc_string_data(detail.string.unwrap()));
                        ptr.push_char(')' );
                    }
                } ,
                _=>{
                    ptr.push_str(fmt!("%in+%i",detail.a.clone() as int, detail.b.clone() as int));
                }
            }
        },

        CSS_SELECTOR_ATTRIBUTE=>{
            ptr.push_char('[');
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_EQUAL =>{
            ptr.push_char('[');
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char('=');
            ptr.push_char('"');
            ptr.push_str(lwc_ref.lwc_string_data(detail.string.unwrap()));
            ptr.push_char('"');
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_DASHMATCH=>{
            ptr.push_char('[');
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char('|');
            ptr.push_char('=');
            ptr.push_char('"');
            ptr.push_str(lwc_ref.lwc_string_data(detail.string.unwrap()));
            ptr.push_char('"');
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_INCLUDES=>{
            ptr.push_char('[');
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char('~');
            ptr.push_char('=');
            ptr.push_char('"');
            ptr.push_str(lwc_ref.lwc_string_data(detail.string.unwrap()));
            ptr.push_char('"');
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_PREFIX=>{
            ptr.push_char('[' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char('^' );
            ptr.push_char('=' );
            ptr.push_char('"' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.string.unwrap()));
            ptr.push_char('"' );
            ptr.push_char(']' );
        },
        CSS_SELECTOR_ATTRIBUTE_SUFFIX=>{
            ptr.push_char('[' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char('$' );
            ptr.push_char('=' );
            ptr.push_char('"' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.string.unwrap()));
            ptr.push_char('"' );
            ptr.push_char(']' );
        },
        CSS_SELECTOR_ATTRIBUTE_SUBSTRING=>{
            ptr.push_char('[' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.qname.name));
            ptr.push_char('*' );
            ptr.push_char('=' );
            ptr.push_char('"' );
            ptr.push_str(lwc_ref.lwc_string_data(detail.string.unwrap()));
            ptr.push_char('"' );
            ptr.push_char(']' );
        }
    }
    if detail.negate {
        ptr.push_char(')');
    }

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_bytecode(stylesheet_vector:&mut ~[css_stylesheet],css_rule_data_list:&mut ~[~css_rule_data_type],rule:uint, lwc_ref:&mut ~lwc, ptr:&mut ~str, depth:u32 ){
    
    debug!("Entering: dump_bytecode");
    
    let bytecode =  match css_rule_data_list[rule].rule_type
            {  
                CSS_RULE_SELECTOR =>
                {       
                    css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().bytecode.clone()
                },
                _ =>
                {
                    // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                    css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().bytecode.clone()
                }
        };

    let mut op: css_properties_e;
    let mut value: u32;
    let opcode_names = opcode_names();
    let mut iterator = 0;
    
    // for bytecode.each|&opv| {
    while iterator < bytecode.len() {
    
        let opv = bytecode[iterator];
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
        
        ptr.push_str( opcode_names[op as int]);
        ptr.push_char(':');
        ptr.push_char(' ');
        
        if isInherit(opv) {

            ptr.push_str( &"inherit");
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
                    ptr.push_str( &"leftwards");
                }
                else if val as int == AZIMUTH_RIGHTWARDS as int {
                    ptr.push_str( &"rightwards");
                }
                else if val as int == AZIMUTH_LEFT_SIDE as int {
                    ptr.push_str( &"left-side");
                }
                else if val as int == AZIMUTH_FAR_LEFT as int {
                    ptr.push_str( &"far-left");
                }
                else if val as int == AZIMUTH_LEFT as int {
                    ptr.push_str( &"left");
                }
                else if val as int == AZIMUTH_CENTER_LEFT as int {
                    ptr.push_str( &"center-left");
                }
                else if val as int == AZIMUTH_CENTER as int {
                    ptr.push_str( &"center");
                }
                else if val as int == AZIMUTH_CENTER_RIGHT as int {
                    ptr.push_str( &"center-right");
                }
                else if val as int == AZIMUTH_RIGHT as int {
                    ptr.push_str( &"right");
                }
                else if val as int == AZIMUTH_FAR_RIGHT as int {
                    ptr.push_str( &"far-right");
                }
                else if val as int == AZIMUTH_RIGHT_SIDE as int {
                    ptr.push_str( &"right-side");
                }

                if (value & (AZIMUTH_BEHIND as u32) > 0) {
                    ptr.push_str( &" behind");
                }
            }

            else if op as int == CSS_PROP_BACKGROUND_ATTACHMENT as int {
                if value as int == BACKGROUND_ATTACHMENT_FIXED as int {
                    ptr.push_str( &"fixed");
                }
                else if value as int == BACKGROUND_ATTACHMENT_SCROLL as int {
                    ptr.push_str( &"scroll");
                }
            }

            else if (op as int == CSS_PROP_BORDER_TOP_COLOR as int || op as int == CSS_PROP_BORDER_RIGHT_COLOR as int 
            || op as int == CSS_PROP_BORDER_BOTTOM_COLOR as int || op as int == CSS_PROP_BORDER_LEFT_COLOR as int 
            || op as int == CSS_PROP_BACKGROUND_COLOR as int || op as int == CSS_PROP_COLUMN_RULE_COLOR as int) {

                assert!(BACKGROUND_COLOR_TRANSPARENT as int == BORDER_COLOR_TRANSPARENT as int);
                assert!(BACKGROUND_COLOR_CURRENT_COLOR as int == BORDER_COLOR_CURRENT_COLOR as int);
                assert!(BACKGROUND_COLOR_SET as int == BORDER_COLOR_SET as int);

                if value as int == BACKGROUND_COLOR_TRANSPARENT as int {
                    ptr.push_str( &"transparent");
                }

                else if value as int == BACKGROUND_COLOR_CURRENT_COLOR as int {
                    ptr.push_str( &"currentColor");   
                }
                else if value as int == BACKGROUND_COLOR_SET as int {
                    
                    let colour: u32 = bytecode[iterator];
                    iterator += 1;
                    let string = fmt!("#%08x" , colour as uint);
                    ptr.push_str( string);
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
                    ptr.push_str( &"none");
                }

                else if value as int == BACKGROUND_IMAGE_URI as int {
                    
                    let snum = bytecode[iterator];
                    let index = match css_rule_data_list[rule].rule_type
                                                   {  
                                                       CSS_RULE_SELECTOR =>
                                                       {       
                                                           css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       },
                                                       _ =>
                                                       {
                                                           // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                           css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       }
                                               };
                    let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);
                    iterator += 1;

                    if option_string.is_some() {
                        ptr.push_str( &"url('");
                        ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned() );
                        ptr.push_str( &"')");    
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
                    
                    ptr.push_str( &"center");
                }


                else if val as int == BACKGROUND_POSITION_HORZ_RIGHT as int {
                    
                    ptr.push_str( &"right");
                }


                else if val as int == BACKGROUND_POSITION_HORZ_LEFT as int {
                    
                    ptr.push_str( &"left");
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
                    
                    ptr.push_str( &"center");
                }


                else if val as int == BACKGROUND_POSITION_VERT_BOTTOM as int {
                    
                    ptr.push_str( &"bottom");
                }


                else if val as int == BACKGROUND_POSITION_VERT_TOP as int {
                    
                    ptr.push_str( &"top");
                }
            }

            else if op as int == CSS_PROP_BACKGROUND_REPEAT as int {

                if value as int == BACKGROUND_REPEAT_NO_REPEAT as int {
                    ptr.push_str( &"no-repeat");
                }

                else if value as int == BACKGROUND_REPEAT_REPEAT_X as int {
                    ptr.push_str( &"repeat-x");
                }

                else if value as int == BACKGROUND_REPEAT_REPEAT_Y as int {
                    ptr.push_str( &"repeat-y");
                }

                else if value as int == BACKGROUND_REPEAT_REPEAT as int {
                    ptr.push_str( &"repeat");
                }
            }

            else if op as int == CSS_PROP_BORDER_COLLAPSE as int {
                
                if value as int == BORDER_COLLAPSE_SEPARATE as int {
                    ptr.push_str( &"separate");
                }

                else if value as int == BORDER_COLLAPSE_COLLAPSE as int {
                    ptr.push_str( &"collapse");
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
                    ptr.push_str( &"none");
                }

                else if value as int == BORDER_STYLE_HIDDEN as int {
                    ptr.push_str( &"hidden");
                }

                else if value as int == BORDER_STYLE_DOTTED as int {
                    ptr.push_str( &"dotted");
                }

                else if value as int == BORDER_STYLE_DASHED as int {
                    ptr.push_str( &"dashed");
                }

                else if value as int == BORDER_STYLE_SOLID as int {
                    ptr.push_str( &"solid");
                }

                else if value as int == BORDER_STYLE_DOUBLE as int {
                    ptr.push_str( &"double");
                }

                else if value as int == BORDER_STYLE_GROOVE as int {
                    ptr.push_str( &"groove");
                }

                else if value as int == BORDER_STYLE_RIDGE as int {
                    ptr.push_str( &"ridge");
                }

                else if value as int == BORDER_STYLE_INSET as int {
                    ptr.push_str( &"inset");
                }

                else if value as int == BORDER_STYLE_OUTSET as int {
                    ptr.push_str( &"outset");
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
                    ptr.push_str( &"thin");
                }

                else if value as int == BORDER_WIDTH_MEDIUM as int {
                    ptr.push_str( &"medium");
                }

                else if value as int == BORDER_WIDTH_THICK as int {
                    ptr.push_str( &"thick");
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
                    ptr.push_str( &"auto");
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
                    ptr.push_str( &"auto");
                }

                else if value as int == BREAK_AFTER_ALWAYS as int {
                    ptr.push_str( &"always");
                }

                else if value as int == BREAK_AFTER_AVOID as int {
                    ptr.push_str( &"avoid");
                }

                else if value as int == BREAK_AFTER_LEFT as int {
                    ptr.push_str( &"left");
                }

                else if value as int == BREAK_AFTER_RIGHT as int {
                    ptr.push_str( &"right");
                }

                else if value as int == BREAK_AFTER_PAGE as int {
                    ptr.push_str( &"page");
                }

                else if value as int == BREAK_AFTER_COLUMN as int {
                    ptr.push_str( &"column");
                }

                else if value as int == BREAK_AFTER_AVOID_PAGE as int {
                    ptr.push_str( &"avoid-page");
                }

                else if value as int == BREAK_AFTER_AVOID_COLUMN as int {
                    ptr.push_str( &"avoid-column");
                }
            }

            else if op as int == CSS_PROP_BREAK_INSIDE as int {
                
                if value as int == BREAK_INSIDE_AUTO as int {
                    ptr.push_str( &"auto");
                }

                else if value as int == BREAK_INSIDE_AVOID as int {
                    ptr.push_str( &"avoid");
                }

                else if value as int == BREAK_INSIDE_AVOID_PAGE as int {
                    ptr.push_str( &"avoid-page");
                }

                else if value as int == BREAK_INSIDE_AVOID_COLUMN as int {
                    ptr.push_str( &"avoid-column");
                }
            }

            else if op as int == CSS_PROP_CAPTION_SIDE as int {
                
                if value as int == CAPTION_SIDE_TOP as int {
                    ptr.push_str( &"top");
                }

                else if value as int == CAPTION_SIDE_BOTTOM as int {
                    ptr.push_str( &"bottom");
                }

            }

            else if op as int == CSS_PROP_CLEAR as int {
                
                if value as int == CLEAR_NONE as int {
                    ptr.push_str( &"none");
                }

                else if value as int == CLEAR_LEFT as int {
                    ptr.push_str( &"left");
                }

                else if value as int == CLEAR_RIGHT as int {
                    ptr.push_str( &"right");
                }

                else if value as int == CLEAR_BOTH as int {
                    ptr.push_str( &"both");
                }
            }

            else if op as int == CSS_PROP_CLIP as int {
                
                if (value as int & CLIP_SHAPE_MASK as int) == CLIP_SHAPE_RECT as int {
                    ptr.push_str( &"rect(");

                    if (value as int & CLIP_RECT_TOP_AUTO as int) > 0 {

                        ptr.push_str( &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    ptr.push_str( &", ");

                    if (value as int & CLIP_RECT_RIGHT_AUTO as int) > 0 {

                        ptr.push_str( &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    ptr.push_str( &", ");

                    if (value as int & CLIP_RECT_BOTTOM_AUTO as int) > 0 {

                        ptr.push_str( &"auto");
                    }
                    else {

                        let some_val = bytecode[iterator];
                        iterator += 1;
                        let unit = bytecode[iterator];
                        iterator += 1;
                        dump_unit(some_val as i32 , unit , ptr);
                    }

                    ptr.push_str( &", ");

                    if (value as int & CLIP_RECT_LEFT_AUTO as int) > 0 {

                        ptr.push_str( &"auto");
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
                else {
                    ptr.push_str( &"auto");    
                }
            }

            else if op as int == CSS_PROP_COLOR as int {
                
                if value as int == COLOR_TRANSPARENT as int {
                    ptr.push_str( &"transparent");
                }

                else if value as int == COLOR_CURRENT_COLOR as int {
                    ptr.push_str( &"currentColor");
                }

                else if value as int == COLOR_SET as int {
                    
                    let colour: u32 = bytecode[iterator];
                    iterator += 1;
                    let string = fmt!("#%08x" , colour as uint);
                    ptr.push_str( string);
                }
            }

            else if op as int == CSS_PROP_COLUMN_COUNT as int {
                
                if value as int == COLUMN_COUNT_SET as int {
                    
                    let some_val = bytecode[iterator];
                    iterator += 1;
                    dump_number(some_val as i32 , ptr);
                }

                else if value as int == COLUMN_COUNT_AUTO as int {
                    ptr.push_str( &"auto");
                }
            }

            else if op as int == CSS_PROP_COLUMN_FILL as int {
                
                if value as int == COLUMN_FILL_BALANCE as int {
                    ptr.push_str( &"balance");
                }

                else if value as int == COLUMN_FILL_AUTO as int {
                    ptr.push_str( &"auto");
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
                    ptr.push_str( &"normal");
                }
            }

            else if op as int == CSS_PROP_COLUMN_SPAN as int {
                
                if value as int == COLUMN_SPAN_NONE as int {
                    ptr.push_str( &"none");
                }

                else if value as int == COLUMN_SPAN_ALL as int {
                    ptr.push_str( &"all");
                }
            }

            else if op as int == CSS_PROP_CONTENT as int {

                if value as int == CONTENT_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == CONTENT_NONE as int {
                    ptr.push_str( &"none");
                }
                
                while value as int != CONTENT_NORMAL as int {

                    let snum = bytecode[iterator];

                    if (value as int & 0xff) == CONTENT_COUNTER as int {
                        let index = match css_rule_data_list[rule].rule_type
                                                       {  
                                                           CSS_RULE_SELECTOR =>
                                                           {       
                                                               css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           },
                                                           _ =>
                                                           {
                                                               // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                               css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           }
                                                   };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);
                        iterator += 1;

                        if option_string.is_some() {
                            debug!("dump_bytecode :: CONTENT_COUNTER :: option_string == %?" , option_string.get_ref());
                            dump_counter( lwc_ref.lwc_string_data(option_string.unwrap()), value , ptr);
                        }
                    }
                    else if (value as int & 0xff) == CONTENT_COUNTERS as int {
                        let index = match css_rule_data_list[rule].rule_type
                                                       {  
                                                           CSS_RULE_SELECTOR =>
                                                           {       
                                                               css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           },
                                                           _ =>
                                                           {
                                                               // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                               css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           }
                                                   };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);
                        iterator += 1;
                        let sep = bytecode[iterator];
                        iterator += 1;

                        if option_string.is_some() {
                            debug!("dump_bytecode :: CONTENT_COUNTERS :: option_string == %?" , option_string.get_ref());
                            dump_counters( lwc_ref.lwc_string_data(option_string.unwrap()) , fmt!("%?" , sep) , value , ptr);
                        }
                    }
                    else if (value as int & 0xff) == CONTENT_URI as int || (value as int & 0xff) == CONTENT_ATTR as int || (value as int & 0xff) == CONTENT_STRING as int {
                        let index = match css_rule_data_list[rule].rule_type
                                                       {  
                                                           CSS_RULE_SELECTOR =>
                                                           {       
                                                               css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           },
                                                           _ =>
                                                           {
                                                               // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                               css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           }
                                                   };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                        if value as int == CONTENT_URI as int {
                            ptr.push_str( &"url(");
                        }
                        if value as int == CONTENT_ATTR as int {
                            ptr.push_str( &"attr(");
                        }
                        if value as int != CONTENT_STRING as int {
                            ptr.push_str( &")");
                        }

                        iterator += 1;

                        if option_string.is_some() {
                            debug!("dump_bytecode :: CONTENT_URI_ATTR_STRING :: option_string == %?" , option_string.get_ref());
                            ptr.push_str( &"'");
                            ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned());
                            ptr.push_str( &"'");
                        }
                    }
                    else if (value as int & 0xff) == CONTENT_OPEN_QUOTE as int {
                        ptr.push_str( "open-quote");
                    }
                    else if (value as int & 0xff) == CONTENT_CLOSE_QUOTE as int {
                        ptr.push_str( "close-quote");
                    }
                    else if (value as int & 0xff) == CONTENT_NO_OPEN_QUOTE as int {
                        ptr.push_str( "no-open-quote");
                    }
                    else if (value as int & 0xff) == CONTENT_NO_CLOSE_QUOTE as int {
                        ptr.push_str( "no-close-quote");
                    }
                    value = bytecode[iterator];
                    iterator += 1;

                    if value as int != CONTENT_NORMAL as int {
                        ptr.push_char(' ');
                    }
                    debug!("dump_bytecode :: while :: ptr == %?" , ptr);
                } // end while
            }

            else if op as int == CSS_PROP_COUNTER_INCREMENT as int || op as int == CSS_PROP_COUNTER_RESET as int {
                
                assert!(COUNTER_INCREMENT_NONE as int == COUNTER_RESET_NONE as int);
                assert!(COUNTER_INCREMENT_NAMED as int == COUNTER_RESET_NAMED as int);

                if value as int == COUNTER_INCREMENT_NAMED as int {

                    while value as int != COUNTER_INCREMENT_NONE as int {
                        let snum = bytecode[iterator];
                        let index = match css_rule_data_list[rule].rule_type
                                                                           {  
                                                                               CSS_RULE_SELECTOR =>
                                                                               {       
                                                                                   css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                                               },
                                                                               _ =>
                                                                               {
                                                                                   // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                                                   css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                                               }
                                                                       };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                        iterator += 1;
                        
                        if option_string.is_some() {
                            ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned());
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
                    ptr.push_str( &"none");
                }
            }

            else if op as int == CSS_PROP_CURSOR as int {
                
                while value as int == CURSOR_URI as int {
                    let snum = bytecode[iterator];
                    iterator += 1;
                    let index = match css_rule_data_list[rule].rule_type
                                                                       {  
                                                                           CSS_RULE_SELECTOR =>
                                                                           {       
                                                                               css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                                           },
                                                                           _ =>
                                                                           {
                                                                               // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                                               css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                                           }
                                                                   };
                    let(_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                    if option_string.is_some() {
                        ptr.push_str( &"url('");
                        ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned());
                        ptr.push_str( &"'), ");
                    }

                    value = bytecode[iterator];
                    iterator += 1;
                }

                if value as int == CURSOR_AUTO as int {
                    ptr.push_str( &"auto");
                }
                else if value as int == CURSOR_CROSSHAIR as int {
                    ptr.push_str( &"crosshair");
                }
                else if value as int == CURSOR_DEFAULT as int {
                    ptr.push_str( &"default");
                }
                else if value as int == CURSOR_POINTER as int {
                    ptr.push_str( &"pointer");
                }
                else if value as int == CURSOR_MOVE as int {
                    ptr.push_str( &"move");
                }
                else if value as int == CURSOR_E_RESIZE as int {
                    ptr.push_str( &"e-resize");
                }
                else if value as int == CURSOR_NE_RESIZE as int {
                    ptr.push_str( &"ne-resize");
                }
                else if value as int == CURSOR_NW_RESIZE as int {
                    ptr.push_str( &"nw-resize");
                }
                else if value as int == CURSOR_N_RESIZE as int {
                    ptr.push_str( &"n-resize");
                }
                else if value as int == CURSOR_SE_RESIZE as int {
                    ptr.push_str( &"se-resize");
                }
                else if value as int == CURSOR_SW_RESIZE as int {
                    ptr.push_str( &"sw-resize");
                }
                else if value as int == CURSOR_S_RESIZE as int {
                    ptr.push_str( &"s-resize");
                }
                else if value as int == CURSOR_W_RESIZE as int {
                    ptr.push_str( &"w-resize");
                }
                else if value as int == CURSOR_TEXT as int {
                    ptr.push_str( &"text");
                }
                else if value as int == CURSOR_WAIT as int {
                    ptr.push_str( &"wait");
                }
                else if value as int == CURSOR_HELP as int {
                    ptr.push_str( &"help");
                }
                else if value as int == CURSOR_PROGRESS as int {
                    ptr.push_str( &"progress");
                }
            }

            else if op as int == CSS_PROP_DIRECTION as int {

                if value as int == DIRECTION_LTR as int {
                    ptr.push_str( &"ltr");
                }
                else if value as int == DIRECTION_RTL as int {
                    ptr.push_str( &"rtl");
                }
            }

            else if op as int == CSS_PROP_DISPLAY as int {

                if value as int == DISPLAY_INLINE as int {
                    ptr.push_str( &"inline");
                }
                else if value as int == DISPLAY_BLOCK as int {
                    ptr.push_str( &"block");
                }
                else if value as int == DISPLAY_LIST_ITEM as int {
                    ptr.push_str( &"list-item");
                }
                else if value as int == DISPLAY_RUN_IN as int {
                    ptr.push_str( &"run-in");
                }
                else if value as int == DISPLAY_INLINE_BLOCK as int {
                    ptr.push_str( &"inline-block");
                }
                else if value as int == DISPLAY_TABLE as int {
                    ptr.push_str( &"table");
                }
                else if value as int == DISPLAY_INLINE_TABLE as int {
                    ptr.push_str( &"inline-table");
                }
                else if value as int == DISPLAY_TABLE_ROW_GROUP as int {
                    ptr.push_str( &"table-row-group");
                }
                else if value as int == DISPLAY_TABLE_HEADER_GROUP as int {
                    ptr.push_str( &"table-header-group");
                }
                else if value as int == DISPLAY_TABLE_FOOTER_GROUP as int {
                    ptr.push_str( &"table-footer-group");
                }
                else if value as int == DISPLAY_TABLE_ROW as int {
                    ptr.push_str( &"table-row");
                }
                else if value as int == DISPLAY_TABLE_COLUMN_GROUP as int {
                    ptr.push_str( &"table-column-group");
                }
                else if value as int == DISPLAY_TABLE_COLUMN as int {
                    ptr.push_str( &"table-column");
                }
                else if value as int == DISPLAY_TABLE_CELL as int {
                    ptr.push_str( &"table-cell");
                }
                else if value as int == DISPLAY_TABLE_CAPTION as int {
                    ptr.push_str( &"table-caption");
                }
                else if value as int == DISPLAY_NONE as int {
                    ptr.push_str( &"none");
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
                    ptr.push_str( &"below");
                }
                else if value as int == ELEVATION_LEVEL as int {
                    ptr.push_str( &"level");
                }
                else if value as int == ELEVATION_ABOVE as int {
                    ptr.push_str( &"above");
                }
                else if value as int == ELEVATION_HIGHER as int {
                    ptr.push_str( &"higher");
                }
                else if value as int == ELEVATION_LOWER as int {
                    ptr.push_str( &"lower");
                }
            }

            else if op as int == CSS_PROP_EMPTY_CELLS as int {

                if value as int == EMPTY_CELLS_SHOW as int {
                    ptr.push_str( &"show");
                }
                else if value as int == EMPTY_CELLS_HIDE as int {
                    ptr.push_str( &"hide");
                }
            }

            else if op as int == CSS_PROP_FLOAT as int {

                if value as int == FLOAT_LEFT as int {
                    ptr.push_str( &"left");
                }
                else if value as int == FLOAT_RIGHT as int {
                    ptr.push_str( &"right");
                }
                else if value as int == FLOAT_NONE as int {
                    ptr.push_str( &"none");
                }
            }

            else if op as int == CSS_PROP_FONT_FAMILY as int {
                
                while value as int != FONT_FAMILY_END as int {

                    if value as int == FONT_FAMILY_STRING as int || value as int == FONT_FAMILY_IDENT_LIST as int {
                        let snum = bytecode[iterator];
                        iterator += 1;
                        let index = match css_rule_data_list[rule].rule_type
                                                       {  
                                                           CSS_RULE_SELECTOR =>
                                                           {       
                                                               css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           },
                                                           _ =>
                                                           {
                                                               // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                               css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                           }
                                                   };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);
                        
                        if option_string.is_some() {
                            ptr.push_char('\'');
                            ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned() );
                            ptr.push_char('\'');
                        }
                    }
                    else if value as int == FONT_FAMILY_SERIF as int {
                        ptr.push_str( &"serif");
                    }
                    else if value as int == FONT_FAMILY_SANS_SERIF as int {
                        ptr.push_str( &"sans-serif");
                    }
                    else if value as int == FONT_FAMILY_CURSIVE as int {
                        ptr.push_str( &"cursive");
                    }
                    else if value as int == FONT_FAMILY_FANTASY as int {
                        ptr.push_str( &"fantasy");
                    }
                    else if value as int == FONT_FAMILY_MONOSPACE as int {
                        ptr.push_str( &"monospace");
                    }

                    value = bytecode[iterator];
                    iterator += 1;

                    if value as int != FONT_FAMILY_END as int {
                        ptr.push_str( &", ");
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
                    ptr.push_str( &"right");
                }
                else if value as int == FONT_SIZE_X_SMALL as int {
                    ptr.push_str( &"none");
                }
                else if value as int == FONT_SIZE_SMALL as int {
                    ptr.push_str( &"small");
                }
                else if value as int == FONT_SIZE_MEDIUM as int {
                    ptr.push_str( &"medium");
                }
                else if value as int == FONT_SIZE_LARGE as int {
                    ptr.push_str( &"large");
                }
                else if value as int == FONT_SIZE_X_LARGE as int {
                    ptr.push_str( &"x-large");
                }
                else if value as int == FONT_SIZE_XX_LARGE as int {
                    ptr.push_str( &"xx-large");
                }
                else if value as int == FONT_SIZE_LARGER as int {
                    ptr.push_str( &"larger");
                }
                else if value as int == FONT_SIZE_SMALLER as int {
                    ptr.push_str( &"smaller");
                }
            }

            else if op as int == CSS_PROP_FONT_STYLE as int {

                if value as int == FONT_STYLE_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == FONT_STYLE_ITALIC as int {
                    ptr.push_str( &"italic");
                }
                else if value as int == FONT_STYLE_OBLIQUE as int {
                    ptr.push_str( &"oblique");
                }
            }

            else if op as int == CSS_PROP_FONT_VARIANT as int {

                if value as int == FONT_VARIANT_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == FONT_VARIANT_SMALL_CAPS as int {
                    ptr.push_str( &"small-caps");
                }
            }

            else if op as int == CSS_PROP_FONT_WEIGHT as int {

                if value as int == FONT_WEIGHT_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == FONT_WEIGHT_BOLD as int {
                    ptr.push_str( &"bold");
                }
                else if value as int == FONT_WEIGHT_BOLDER as int {
                    ptr.push_str( &"bolder");
                }
                else if value as int == FONT_WEIGHT_LIGHTER as int {
                    ptr.push_str( &"lighter");
                }
                else if value as int == FONT_WEIGHT_100 as int {
                    ptr.push_str( &"100");
                }
                else if value as int == FONT_WEIGHT_200 as int {
                    ptr.push_str( &"200");
                }
                else if value as int == FONT_WEIGHT_300 as int {
                    ptr.push_str( &"300");
                }
                else if value as int == FONT_WEIGHT_400 as int {
                    ptr.push_str( &"400");
                }
                else if value as int == FONT_WEIGHT_500 as int {
                    ptr.push_str( &"500");
                }
                else if value as int == FONT_WEIGHT_600 as int {
                    ptr.push_str( &"600");
                }
                else if value as int == FONT_WEIGHT_700 as int {
                    ptr.push_str( &"700");
                }
                else if value as int == FONT_WEIGHT_800 as int {
                    ptr.push_str( &"800");
                }
                else if value as int == FONT_WEIGHT_900 as int {
                    ptr.push_str( &"900");
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
                    ptr.push_str( &"normal");
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
                    ptr.push_str( &"normal");
                }
            }

            else if op as int == CSS_PROP_LIST_STYLE_POSITION as int{

                if value as int == LIST_STYLE_POSITION_INSIDE as int {
                    ptr.push_str( &"inside");
                }
                else if value as int == LIST_STYLE_POSITION_OUTSIDE as int {
                    ptr.push_str( &"outside");
                }
            }

            else if op as int == CSS_PROP_LIST_STYLE_TYPE as int{

                if value as int == LIST_STYLE_TYPE_DISC as int {
                    ptr.push_str( &"disc");
                }
                else if value as int == LIST_STYLE_TYPE_CIRCLE as int {
                    ptr.push_str( &"circle");
                }
                else if value as int == LIST_STYLE_TYPE_SQUARE as int {
                    ptr.push_str( &"square");
                }
                else if value as int == LIST_STYLE_TYPE_DECIMAL as int {
                    ptr.push_str( &"decimal");
                }
                else if value as int == LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as int {
                    ptr.push_str( &"decimal-leading-zero");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_ROMAN as int {
                    ptr.push_str( &"lower-roman");
                }
                else if value as int == LIST_STYLE_TYPE_UPPER_ROMAN as int {
                    ptr.push_str( &"upper-roman");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_GREEK as int {
                    ptr.push_str( &"lower-greek");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_LATIN as int {
                    ptr.push_str( &"lower-latin");
                }
                else if value as int == LIST_STYLE_TYPE_UPPER_LATIN as int {
                    ptr.push_str( &"upper-latin");
                }
                else if value as int == LIST_STYLE_TYPE_ARMENIAN as int {
                    ptr.push_str( &"armenian");
                }
                else if value as int == LIST_STYLE_TYPE_GEORGIAN as int {
                    ptr.push_str( &"georgian");
                }
                else if value as int == LIST_STYLE_TYPE_LOWER_ALPHA as int {
                    ptr.push_str( &"lower-alpha");
                }
                else if value as int == LIST_STYLE_TYPE_UPPER_ALPHA as int {
                    ptr.push_str( &"upper-alpha");
                }
                else if value as int == LIST_STYLE_TYPE_NONE as int {
                    ptr.push_str( &"none");
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
                    ptr.push_str( &"none");
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
                    ptr.push_str( &"transparent");
                }
                else if value as int == OUTLINE_COLOR_CURRENT_COLOR as int {
                    ptr.push_str( &"currentColor");
                }
                else if value as int == OUTLINE_COLOR_SET as int {
                    
                    let colour = bytecode[iterator];
                    iterator += 1;

                    let string = fmt!("#%08x" , colour as uint);
                    ptr.push_str( string);
                }
                else if value as int == OUTLINE_COLOR_INVERT as int {
                    ptr.push_str( &"invert");
                }
            }

            else if op as int == CSS_PROP_OVERFLOW as int {

                if value as int == OVERFLOW_VISIBLE as int {
                    ptr.push_str( &"visible");
                }
                else if value as int == OVERFLOW_HIDDEN as int {
                    ptr.push_str( &"hidden");
                }
                else if value as int == OVERFLOW_SCROLL as int {
                    ptr.push_str( &"scroll");
                }
                else if value as int == OVERFLOW_AUTO as int {
                    ptr.push_str( &"auto");
                }
            }

            else if op as int == CSS_PROP_PAGE_BREAK_AFTER as int || op as int == CSS_PROP_PAGE_BREAK_BEFORE as int {
                
                assert!(PAGE_BREAK_AFTER_AUTO as int == PAGE_BREAK_BEFORE_AUTO as int);
                assert!(PAGE_BREAK_AFTER_ALWAYS as int == PAGE_BREAK_BEFORE_ALWAYS as int);
                assert!(PAGE_BREAK_AFTER_AVOID as int == PAGE_BREAK_BEFORE_AVOID as int);
                assert!(PAGE_BREAK_AFTER_LEFT as int == PAGE_BREAK_BEFORE_LEFT as int);
                assert!(PAGE_BREAK_AFTER_RIGHT as int == PAGE_BREAK_BEFORE_RIGHT as int);

                if value as int == PAGE_BREAK_AFTER_AUTO as int {
                    ptr.push_str( &"auto");
                }
                else if value as int == PAGE_BREAK_AFTER_ALWAYS as int {
                    ptr.push_str( &"always");
                }
                else if value as int == PAGE_BREAK_AFTER_AVOID as int {
                    ptr.push_str( &"avoid");
                }
                else if value as int == PAGE_BREAK_AFTER_LEFT as int {
                    ptr.push_str( &"left");
                }
                else if value as int == PAGE_BREAK_AFTER_RIGHT as int {
                    ptr.push_str( &"right");
                }
            }

            else if op as int == CSS_PROP_PAGE_BREAK_INSIDE as int{

                if value as int == PAGE_BREAK_INSIDE_AUTO as int {
                    ptr.push_str( &"auto");
                }
                else if value as int == PAGE_BREAK_INSIDE_AVOID as int {
                    ptr.push_str( &"avoid");
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
                    ptr.push_str( &"x-low");
                }
                else if value as int == PITCH_LOW as int {
                    ptr.push_str( &"low");
                }
                else if value as int == PITCH_MEDIUM as int {
                    ptr.push_str( &"medium");
                }
                else if value as int == PITCH_HIGH as int {
                    ptr.push_str( &"high");
                }
                else if value as int == PITCH_X_HIGH as int {
                    ptr.push_str( &"x-high");
                }
            }

            else if op as int == CSS_PROP_PLAY_DURING as int{

                if value as int == PLAY_DURING_URI as int {
                    
                    let snum = bytecode[iterator];
                    iterator += 1;
                    let index = match css_rule_data_list[rule].rule_type
                                                   {  
                                                       CSS_RULE_SELECTOR =>
                                                       {       
                                                           css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       },
                                                       _ =>
                                                       {
                                                           // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                           css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       }
                                               };
                    let(_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                    if option_string.is_some() {
                        ptr.push_char('\'');
                        ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned() );
                        ptr.push_char('\'');
                    }
                }
                else if value as int == PLAY_DURING_AUTO as int {
                    ptr.push_str( &"auto");
                }
                else if value as int == PLAY_DURING_NONE as int {
                    ptr.push_str( &"none");
                }
                
                if (value as int & PLAY_DURING_MIX as int > 0) {
                    ptr.push_str( &"mix");
                }

                if (value as int & PLAY_DURING_REPEAT as int > 0) {
                    ptr.push_str( &"repeat");
                }
            }

            else if op as int == CSS_PROP_POSITION as int{

                if value as int == POSITION_STATIC as int {
                    ptr.push_str( &"static");
                }
                else if value as int == POSITION_RELATIVE as int {
                    ptr.push_str( &"relative");
                }
                else if value as int == POSITION_ABSOLUTE as int {
                    ptr.push_str( &"absolute");
                }
                else if value as int == POSITION_FIXED as int {
                    ptr.push_str( &"fixed");
                }
            }

            // TODO review
            else if op as int == CSS_PROP_QUOTES as int{

                if value as int == QUOTES_STRING as int {
                    
                    while value as int != QUOTES_NONE as int {
                        
                        let snum = bytecode[iterator];
                        iterator += 1;
                        let index = match css_rule_data_list[rule].rule_type
                                                   {  
                                                       CSS_RULE_SELECTOR =>
                                                       {       
                                                           css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       },
                                                       _ =>
                                                       {
                                                           // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                           css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       }
                                               };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                        if option_string.is_some() {
                            ptr.push_str( &" '");
                            ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned() );
                            ptr.push_str( &"' ");
                        }
                        let index = match css_rule_data_list[rule].rule_type
                                                   {  
                                                       CSS_RULE_SELECTOR =>
                                                       {       
                                                           css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       },
                                                       _ =>
                                                       {
                                                           // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                           css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       }
                                               };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                        if option_string.is_some() {
                            ptr.push_str( &" '");
                            ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned() );
                            ptr.push_str( &"' ");
                        }
                        iterator += 1;
                        value = bytecode[iterator];
                        iterator += 1;
                    }
                }
                else if value as int == QUOTES_NONE as int {
                    ptr.push_str( &"none");
                }
            }

            else if op as int == CSS_PROP_SPEAK_HEADER as int{

                if value as int == SPEAK_HEADER_ONCE as int {
                    ptr.push_str( &"once");
                }
                else if value as int == SPEAK_HEADER_ALWAYS as int {
                    ptr.push_str( &"always");
                }
            }

            else if op as int == CSS_PROP_SPEAK_NUMERAL as int{

                if value as int == SPEAK_NUMERAL_DIGITS as int {
                    ptr.push_str( &"digits");
                }
                else if value as int == SPEAK_NUMERAL_CONTINUOUS as int {
                    ptr.push_str( &"continuous");
                }
            }

            else if op as int == CSS_PROP_SPEAK_PUNCTUATION as int{

                if value as int == SPEAK_PUNCTUATION_CODE as int {
                    ptr.push_str( &"code");
                }
                else if value as int == SPEAK_PUNCTUATION_NONE as int {
                    ptr.push_str( &"none");
                }
            }

            else if op as int == CSS_PROP_SPEAK as int{

                if value as int == SPEAK_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == SPEAK_NONE as int {
                    ptr.push_str( &"none");
                }
                else if value as int == SPEAK_SPELL_OUT as int {
                    ptr.push_str( &"spell-out");
                }
            }

            else if op as int == CSS_PROP_SPEECH_RATE as int{

                if value as int == SPEECH_RATE_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    dump_number(some_val , ptr);
                }
                else if value as int == SPEECH_RATE_X_SLOW as int {
                    ptr.push_str( &"x-slow");
                }
                else if value as int == SPEECH_RATE_SLOW as int {
                    ptr.push_str( &"slow");
                }
                else if value as int == SPEECH_RATE_MEDIUM as int {
                    ptr.push_str( &"medium");
                }
                else if value as int == SPEECH_RATE_FAST as int {
                    ptr.push_str( &"fast");
                }
                else if value as int == SPEECH_RATE_X_FAST as int {
                    ptr.push_str( &"x-fast");
                }
                else if value as int == SPEECH_RATE_FASTER as int {
                    ptr.push_str( &"faster");
                }
                else if value as int == SPEECH_RATE_SLOWER as int {
                    ptr.push_str( &"slower");
                }
            }

            else if op as int == CSS_PROP_TABLE_LAYOUT as int{

                if value as int == TABLE_LAYOUT_AUTO as int {
                    ptr.push_str( &"auto");
                }
                else if value as int == TABLE_LAYOUT_FIXED as int {
                    ptr.push_str( &"fixed");
                }
            }

            else if op as int == CSS_PROP_TEXT_ALIGN as int{

                if value as int == TEXT_ALIGN_LEFT as int {
                    ptr.push_str( &"left");
                }
                else if value as int == TEXT_ALIGN_RIGHT as int {
                    ptr.push_str( &"right");
                }
                else if value as int == TEXT_ALIGN_CENTER as int {
                    ptr.push_str( &"center");
                }
                else if value as int == TEXT_ALIGN_JUSTIFY as int {
                    ptr.push_str( &"justify");
                }
                else if value as int == TEXT_ALIGN_LIBCSS_LEFT as int {
                    ptr.push_str( &"-libcss-left");
                }
                else if value as int == TEXT_ALIGN_LIBCSS_CENTER as int {
                    ptr.push_str( &"-libcss-center");
                }
                else if value as int == TEXT_ALIGN_LIBCSS_RIGHT as int {
                    ptr.push_str( &"-libcss-right");
                }
            }

            else if op as int == CSS_PROP_TEXT_DECORATION as int{

                if value as int == TEXT_DECORATION_NONE as int {
                    ptr.push_str( &"none");
                }
                if value as int == TEXT_DECORATION_UNDERLINE as int {
                    ptr.push_str( &" underline");
                }
                if value as int == TEXT_DECORATION_OVERLINE as int {
                    ptr.push_str( &" overline");
                }
                if value as int == TEXT_DECORATION_LINE_THROUGH as int {
                    ptr.push_str( &" line-through");
                }
                if value as int == TEXT_DECORATION_BLINK as int {
                    ptr.push_str( &"blink");
                }
            }

            else if op as int == CSS_PROP_TEXT_TRANSFORM as int{

                if value as int == TEXT_TRANSFORM_CAPITALIZE as int {
                    ptr.push_str( &"capitalize");
                }
                else if value as int == TEXT_TRANSFORM_UPPERCASE as int {
                    ptr.push_str( &"uppercase");
                }
                else if value as int == TEXT_TRANSFORM_LOWERCASE as int {
                    ptr.push_str( &"lowercase");
                }
                else if value as int == TEXT_TRANSFORM_NONE as int {
                    ptr.push_str( &"none");
                }
            }

            else if op as int == CSS_PROP_UNICODE_BIDI as int{

                if value as int == UNICODE_BIDI_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == UNICODE_BIDI_EMBED as int {
                    ptr.push_str( &"embed");
                }
                else if value as int == UNICODE_BIDI_BIDI_OVERRIDE as int {
                    ptr.push_str( &"bidi-override");
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
                    ptr.push_str( &"baseline");
                }
                else if value as int == VERTICAL_ALIGN_SUB as int {
                    ptr.push_str( &"sub");
                }
                else if value as int == VERTICAL_ALIGN_SUPER as int {
                    ptr.push_str( &"super");
                }
                else if value as int == VERTICAL_ALIGN_TOP as int {
                    ptr.push_str( &"top");
                }
                else if value as int == VERTICAL_ALIGN_TEXT_TOP as int {
                    ptr.push_str( &"text-top");
                }
                else if value as int == VERTICAL_ALIGN_MIDDLE as int {
                    ptr.push_str( &"middle");
                }
                else if value as int == VERTICAL_ALIGN_BOTTOM as int {
                    ptr.push_str( &"bottom");
                }
                else if value as int == VERTICAL_ALIGN_TEXT_BOTTOM as int {
                    ptr.push_str( &"text-bottom");
                }
            }

            else if op as int == CSS_PROP_VISIBILITY as int {

                if value as int == VISIBILITY_VISIBLE as int {
                    ptr.push_str( &"visible");
                }
                else if value as int == VISIBILITY_HIDDEN as int {
                    ptr.push_str( &"hidden");
                }
                else if value as int == VISIBILITY_COLLAPSE as int {
                    ptr.push_str( &"collapse");
                }
            }

            else if op as int == CSS_PROP_VOICE_FAMILY as int {
                
                while value as int != VOICE_FAMILY_END as int {

                    if value as int == VOICE_FAMILY_STRING as int || value as int == VOICE_FAMILY_IDENT_LIST as int {
                        let snum = bytecode[iterator];
                        iterator += 1;
                        let index = match css_rule_data_list[rule].rule_type
                                                   {  
                                                       CSS_RULE_SELECTOR =>
                                                       {       
                                                           css_rule_data_list[rule].rule_selector.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       },
                                                       _ =>
                                                       {
                                                           // ONLY CALLED FOR TWO RULE PAGE AND RULE SELECTOR
                                                           css_rule_data_list[rule].rule_page.get_mut_ref().style.get_mut_ref().sheet.unwrap()
                                                       }
                                               };
                        let (_ , option_string) = stylesheet_vector[index].css__stylesheet_string_get(snum as uint);

                        if option_string.is_some() {
                            ptr.push_char('\'');
                            ptr.push_str( lwc_ref.lwc_string_data(option_string.unwrap()).to_owned() );
                            ptr.push_char('\'');
                        }
                    }
                    else if value as int == VOICE_FAMILY_MALE as int {
                        ptr.push_str( &"male");
                    }
                    else if value as int == VOICE_FAMILY_FEMALE as int {
                        ptr.push_str( &"female");
                    }
                    else if value as int == VOICE_FAMILY_CHILD as int {
                        ptr.push_str( &"child");
                    }

                    value = bytecode[iterator];
                    iterator += 1;

                    if value as int != VOICE_FAMILY_END as int {
                        ptr.push_str( &", ");
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
                    ptr.push_str( &"silent");
                }
                else if value as int == VOLUME_X_SOFT as int {
                    ptr.push_str( &"x-soft");
                }
                else if value as int == VOLUME_SOFT as int {
                    ptr.push_str( &"soft");
                }
                else if value as int == VOLUME_MEDIUM as int {
                    ptr.push_str( &"medium");
                }
                else if value as int == VOLUME_LOUD as int {
                    ptr.push_str( &"loud");
                }
                else if value as int == VOLUME_X_LOUD as int {
                    ptr.push_str( &"loud");
                }
            }

            else if op as int == CSS_PROP_VOLUME as int {

                if value as int == WHITE_SPACE_NORMAL as int {
                    ptr.push_str( &"normal");
                }
                else if value as int == WHITE_SPACE_PRE as int {
                    ptr.push_str( &"pre");
                }
                else if value as int == WHITE_SPACE_NOWRAP as int {
                    ptr.push_str( &"nowrap");
                }
                else if value as int == WHITE_SPACE_PRE_WRAP as int {
                    ptr.push_str( &"pre-wrap");
                }
                else if value as int == WHITE_SPACE_PRE_LINE as int {
                    ptr.push_str( &"pre-line");
                }
            }

            else if op as int == CSS_PROP_Z_INDEX as int {

                if value as int == Z_INDEX_SET as int {
                    
                    let some_val = bytecode[iterator] as i32;
                    iterator += 1;
                    dump_number(some_val , ptr);
                }
                else if value as int == Z_INDEX_AUTO as int {
                    ptr.push_str( &"auto");
                }
            }

            else {
                let string = fmt!("Unknown opcode %x" , op as uint);
                ptr.push_str( string);
            }
        }
        
        if (isImportant(opv)) {
            ptr.push_str( &" !important")
        }
        ptr.push_char('\n');

    }

    debug!(fmt!("ptr == %?" , ptr));

}

fn dump_number(val: i32 , ptr: &mut ~str){
    debug!("Entering: dump_number");
    if css_int_to_fixed((val >> 10) as int) == val {
        ptr.push_str( fmt!("%i" , val as int >> 10 as int));
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
            ptr.push_str( &"px");
        },
        UNIT_EX => {
            ptr.push_str( &"ex");
        },
        UNIT_EM => {
            ptr.push_str( &"em");
        },
        UNIT_IN => {
            ptr.push_str( &"in");
        },
        UNIT_CM => {
            ptr.push_str( &"cm");
        },
        UNIT_MM => {
            ptr.push_str( &"mm");
        },
        UNIT_PT => {
            ptr.push_str( &"pt");
        },
        UNIT_PC => {
            ptr.push_str( &"pc");
        },
        UNIT_PCT => {
            ptr.push_str( &"%");
        },
        UNIT_DEG => {
            ptr.push_str( &"deg");
        },
        UNIT_GRAD => {
            ptr.push_str( &"grad");
        },
        UNIT_RAD => {
            ptr.push_str( &"rad");
        },
        UNIT_MS => {
            ptr.push_str( &"ms");
        },
        UNIT_S => {
            ptr.push_str( &"s");
        },
        UNIT_HZ => {
            ptr.push_str( &"Hz");
        },
        UNIT_KHZ => {
            ptr.push_str( &"kHz");
        },
        _ => {}
    }

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_font_face(font_face: &~css_font_face, lwc_ref:&mut ~lwc, ptr: &mut ~str){
    debug!("Entering: dump_font_face");
    let mut style: u8;
    let mut weight: u8;
    let reason = "Function dump_font_face";

    if font_face.font_family.is_some() {
        ptr.push_char('\n');
        ptr.push_str( &"| font_family: ");
        ptr.push_str( lwc_ref.lwc_string_data(font_face.font_family.expect(reason)));
    }
    ptr.push_str( &"\n| font-style: ");

    style = css_font_face_font_style(font_face) as u8;

    if style as int == CSS_FONT_STYLE_INHERIT as int {
        ptr.push_str( &"unspecified");
    }
    else if style as int == CSS_FONT_STYLE_NORMAL as int {
        ptr.push_str( &"normal");
    }
    else if style as int == CSS_FONT_STYLE_ITALIC as int {
        ptr.push_str( &"italic");
    }
    else if style as int == CSS_FONT_STYLE_OBLIQUE as int {
        ptr.push_str( &"oblique");
    }

    ptr.push_str( &"\n| font-weight: ");

    weight = css_font_face_font_weight(font_face) as u8;

    if weight as int == CSS_FONT_WEIGHT_INHERIT as int {
        ptr.push_str( &"unspecified");
    }
    else if weight as int == CSS_FONT_WEIGHT_NORMAL as int {
        ptr.push_str( &"normal");
    }
    else if weight as int == CSS_FONT_WEIGHT_BOLD as int {
        ptr.push_str( &"normal");
    }
    else if weight as int == CSS_FONT_WEIGHT_100 as int {
        ptr.push_str( &"100");
    }
    else if weight as int == CSS_FONT_WEIGHT_200 as int {
        ptr.push_str( &"200");
    }
    else if weight as int == CSS_FONT_WEIGHT_300 as int {
        ptr.push_str( &"300");
    }
    else if weight as int == CSS_FONT_WEIGHT_400 as int {
        ptr.push_str( &"400");
    }
    else if weight as int == CSS_FONT_WEIGHT_500 as int {
        ptr.push_str( &"500");
    }
    else if weight as int == CSS_FONT_WEIGHT_600 as int {
        ptr.push_str( &"600");
    }
    else if weight as int == CSS_FONT_WEIGHT_700 as int {
        ptr.push_str( &"700");
    }
    else if weight as int == CSS_FONT_WEIGHT_800 as int {
        ptr.push_str( &"800");
    }
    else if weight as int == CSS_FONT_WEIGHT_900 as int {
        ptr.push_str( &"900");
    }
    else {
        ptr.push_str( &"Unhandled weight");
        ptr.push_str( fmt!("%d" , weight as int));
        ptr.push_char('\n');
    }

        for i in font_face.srcs.iter() {
            ptr.push_str( &"\n| src: ");
            let format = css_font_face_src_format(i);
            ptr.push_str( &"\n| format: ");

            if format as int == CSS_FONT_FACE_FORMAT_UNSPECIFIED as int {
                ptr.push_str( &"unspecified");
            }
            else if format as int == CSS_FONT_FACE_FORMAT_WOFF as int {
                ptr.push_str( &"WOFF");
            }
            else if format as int == CSS_FONT_FACE_FORMAT_OPENTYPE as int {
                ptr.push_str( &"OTF");
            }
            else if format as int == CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE as int {
                ptr.push_str( &"EOTF");
            }
            else if format as int == CSS_FONT_FACE_FORMAT_SVG as int {
                ptr.push_str( &"SVG");
            }
            else if format as int == CSS_FONT_FACE_FORMAT_UNKNOWN as int {
                ptr.push_str( &"unknown");
            }
            else {
                ptr.push_str( &"UNEXPECTED");
            }

            if i.location.is_some() {
                ptr.push_str( &"\n| location: ");

                let location = css_font_face_src_location_type(i);
                
                if location as int == CSS_FONT_FACE_LOCATION_TYPE_LOCAL as int {
                    ptr.push_str( &"local");
                }
                else if location as int == CSS_FONT_FACE_LOCATION_TYPE_URI as int {
                    ptr.push_str( &"url");
                }
                else {
                    ptr.push_str( &"UNKNOWN");
                }

                ptr.push_str( lwc_ref.lwc_string_data(i.location.expect(reason)));
            }

        }

    debug!(fmt!("ptr == %?" , ptr));

}

fn dump_counter(name: ~str , value: u32 , ptr: &mut ~str) {
    debug!("Entering: dump_counter");
    ptr.push_str( &"counter(");
    ptr.push_str( name);
    let val = value >> CONTENT_COUNTER_STYLE_SHIFT;

    if val as int == LIST_STYLE_TYPE_DISC as int {
        ptr.push_str( &", disc");
    }
    else if val as int == LIST_STYLE_TYPE_CIRCLE as int {
        ptr.push_str( &", circle");
    }
    else if val as int == LIST_STYLE_TYPE_SQUARE as int {
        ptr.push_str( &", square");
    }
    // else if (val as int == LIST_STYLE_TYPE_DECIMAL as int) {

    // }
    else if (val as int == LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as int) {
        ptr.push_str( &", decimal-leading-zero");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_ROMAN as int {
        ptr.push_str( &", lower-roman");
    }
    else if val as int == LIST_STYLE_TYPE_UPPER_ROMAN as int {
        ptr.push_str( &", upper-roman");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_GREEK as int {
        ptr.push_str( &", lower-greek");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_LATIN as int {
        ptr.push_str( &", lower-latin");
    }
    else if val as int == LIST_STYLE_TYPE_UPPER_LATIN as int {
        ptr.push_str( &", upper-latin");
    }
    else if val as int == LIST_STYLE_TYPE_ARMENIAN as int {
        ptr.push_str( &", armenian");
    }
    else if val as int == LIST_STYLE_TYPE_GEORGIAN as int {
        ptr.push_str( &", georgian");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_ALPHA as int {
        ptr.push_str( &", lower-alpha");
    }
    else if val as int == LIST_STYLE_TYPE_UPPER_ALPHA as int {
        ptr.push_str( &", upper-alpha");
    }
    else if val as int == LIST_STYLE_TYPE_NONE as int {
        ptr.push_str( &", none");
    }
    ptr.push_char(')');

    debug!(fmt!("ptr == %?" , ptr));
}

fn dump_counters(name: ~str , separator: ~str , value: u32 , ptr: &mut ~str) {

    debug!("Entering: dump_counters");
    ptr.push_str( &"counter(");
    ptr.push_str( name);
    ptr.push_str( separator);
    let val = value >> CONTENT_COUNTER_STYLE_SHIFT;

    if val as int == LIST_STYLE_TYPE_DISC as int {
        ptr.push_str( &", disc");
    }
    else if val as int == LIST_STYLE_TYPE_CIRCLE as int {
        ptr.push_str( &", circle");
    }
    else if val as int == LIST_STYLE_TYPE_SQUARE as int {
        ptr.push_str( &", square");
    }
    // else if (val as int == LIST_STYLE_TYPE_DECIMAL as int) {}
    else if (val as int == LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO as int) {
        ptr.push_str( &", decimal-leading-zero");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_ROMAN as int {
        ptr.push_str( &", lower-roman");
    }
    else if val as int == LIST_STYLE_TYPE_UPPER_ROMAN as int {
        ptr.push_str( &", upper-roman");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_GREEK as int {
        ptr.push_str( &", lower-greek");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_LATIN as int {
        ptr.push_str( &", lower-latin");
    }
    else if val as int == LIST_STYLE_TYPE_UPPER_LATIN as int {
        ptr.push_str( &", upper-latin");
    }
    else if val as int == LIST_STYLE_TYPE_ARMENIAN as int {
        ptr.push_str( &", armenian");
    }
    else if val as int == LIST_STYLE_TYPE_GEORGIAN as int {
        ptr.push_str( &", georgian");
    }
    else if val as int == LIST_STYLE_TYPE_LOWER_ALPHA as int {
        ptr.push_str( &", lower-alpha");
    }
    else if val as int == LIST_STYLE_TYPE_UPPER_ALPHA as int {
        ptr.push_str( &", upper-alpha");
    }
    else if val as int == LIST_STYLE_TYPE_NONE as int {
        ptr.push_str( &", none");
    }
    ptr.push_char(')');

    debug!(fmt!("ptr == %?" , ptr));
}
