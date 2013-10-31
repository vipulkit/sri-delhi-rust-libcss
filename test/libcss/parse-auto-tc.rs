extern mod extra;
extern mod css;
extern mod wapcaplet;

use std::uint;
use std::int;
use std::io;
use std::str;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use libwapcaplet::wapcaplet::*;

pub fn find_char_between(s: &str, c: char, start: uint, end: uint) -> Option<uint>{
	let length = s.len();
	let mut i : uint = start;
	while i < length && i < end {
		if(s[i] as char == c){
			return Some(i);
		}
		
		i = i + 1;
	}
	
	return None;
}

pub fn resolve_url(_:@str, rel:@mut wapcaplet::lwc_string) -> (css_error,Option<@mut wapcaplet::lwc_string>) {
    return (CSS_OK,Some(rel));
}

enum expected_value {
    bytecode(u32),
    string(@str)
}


pub struct exp_entry{
    ftype:int,
    name: ~str,
    expected: ~[expected_value]
}

pub struct line_ctx {
    buf:~[u8],

    indata:bool,
    inerrors:bool,

    inrule:bool,
    lwc_instance:@mut lwc
}


pub fn is_string_caseless_equal(a : &str , b : &str ) -> bool {

    //debug!(fmt!("Strtol : strings are %? ====== %? ",a,b));
    if ( a.len() != b.len() ) {
        return false ;
    }
    
    let i :uint = a.len() ;
    for uint::iterate(0,i) |e| {
        if a[e] == b[e] {
            loop;
        }

        if (a[e] >= 'A' as u8  && a[e] <= 'Z'  as u8) {
            if (a[e]+32) == b[e] {
                loop;
            }
            else {
                return false ;
            }
        }

        if (b[e] >= 'A'  as u8 && b[e] <= 'Z'  as u8) {
            if (b[e]+32) == a[e] {
                loop;
            }
            else {
                return false ;
            }
        }
        return false ;
    }
    return true ;
}

pub fn strtol(data:~str , data_used: &mut uint) -> Option<int> {

    let mut res : i64  = 0 ;
    let mut negative : bool = false ;

    if *data_used >= data.len()  {
        return None ;
    }

    if data[*data_used] == ('-' as u8) {
        negative = true ;
        *data_used += 1;
    }
    else if data[*data_used] == ('+' as u8) {
        negative = false ;
        *data_used += 1;
    }
    
    while ( *data_used < data.len() ) {
        if (data[*data_used] > 47 && data[*data_used]<58 ) {
            res = res*10 + ( ( (data[*data_used] as u8) - 48 ) as i64);
            *data_used += 1;

            if( res >= (int::max_value as i64) ) {
                fail!(~"\n Excedded maximum value of an integer") ;
            }

            loop ;
        }   
        else {
            if negative {
                res = -res ;
            }
            return Some(res as int) ;
        }
    }

    if negative {
        res = -res ;
    }
    Some(res as int)
}

pub fn strtoul(data:~str , data_used: &mut uint) -> Option<uint> {

    let mut res : u64  = 0 ;
    let orig = *data_used ;
    if *data_used >= data.len()  {
        return None ;
    }

    if (data[*data_used] == ('-' as u8) ) || 
            (data[*data_used] == ('+' as u8) ) {
        *data_used += 1; // skip character
    }
    if (data[*data_used] == ('0' as u8) )  {
        *data_used += 1; // skip character
    }
    if (data[*data_used] == ('x' as u8) ) || 
            (data[*data_used] == ('X' as u8) ) {
        *data_used += 1; // skip character
    }
    else {
        *data_used = orig ;
        return None ;
    }
    
    while ( *data_used < data.len() ) {
        if (data[*data_used] > 47 && data[*data_used]<58 ) {
            res = res*16 + ( ( (data[*data_used] as u8) - 48 ) as u64);
            *data_used += 1;

            if( res >= (uint::max_value as u64) ) {
                fail!(~"\n Excedded maximum value of an integer") ;
            }

            loop ;
        }
        else if  (data[*data_used] > 64 && data[*data_used] < 71 ) {
            res = res*16 + ( ( (data[*data_used] as u8) - 55 ) as u64);
            *data_used += 1;

            if( res >= (uint::max_value as u64) ) {
                fail!(~"\n Excedded maximum value of an integer") ;
            }

            loop ;
        }
        else if (data[*data_used] > 96 && data[*data_used]<103 ) {
            res = res*16 + ( ( (data[*data_used] as u8) - 87 ) as u64);
            *data_used += 1;

            if( res >= (uint::max_value as u64) ) {
                fail!(~"\n Excedded maximum value of an integer") ;
            }

            loop ;
        }   
        else {
            return Some(res as uint) ;
        }
    }

    Some(res as uint)
}

fn main() {
    debug!("parse-auto");
    // corresponding code now in parse_auto function, entry from test-cases
}

fn parse_auto(file: ~str, file_Write:~str) {

    let file_content_result = io::read_whole_file_str(&Path(file)) ;
    let mut file_content : ~str ;
    let w:@Writer = io::file_writer(&Path(file_Write),[io::Create, io::Truncate]).unwrap();
    match file_content_result {
        Ok(x) => {
            file_content = x ;
        },
        Err(y) => {
            file_content = ~"" ;
            debug!(fmt!("\n Error opening file :%?",y));
            assert!(false) ;
        }
    }

    let ctx : @mut line_ctx = @mut line_ctx{
        buf:~[],
        indata:false,
        inerrors:false,
        inrule:false,
        lwc_instance:lwc()
    };

    for file_content.any_line_iter().advance |line| {
        let mut line_string : ~str = line.to_str() ;
        line_string.push_char('\n') ;
        debug!(fmt!("Entering:v data is=%?=",line_string));
        handle_line(line_string,ctx,w);
    }
    debug!(fmt!("Ctx ====================================\n%?\n==============================",ctx));
    /* and run final test */
    if ( ctx.buf.len()>0  ) {
        run_test(ctx, w);
    }
}

pub fn handle_line(data:~str,ctx:@mut line_ctx, w:@Writer) -> bool {

    let len : uint = 0 ;
    if (data[len] == ('#' as u8) ) {

        if (ctx.indata && data.len()>=7 && 
                (is_string_caseless_equal( data.slice(1,7), "errors")) ) {

            w.write_line("#expected");
            run_test(ctx,w);
            w.write_line("#reset");
            ctx.buf = ~[] ;

            ctx.indata = false;
            ctx.inerrors = true;
        } 
        else if (ctx.inerrors && data.len()>=9 && 
                (is_string_caseless_equal( data.slice(1,9), "expected"))) {

            ctx.indata = false;
            ctx.inerrors = false;
            ctx.inrule = false;
            //debug!(fmt!("Entering:v ctx in rule false 1=%?=",data));
        } 
        else if (ctx.indata) {
            //ctx.buf = ~[] ;
            for data.iter().advance |ch| {
                ctx.buf.push(ch as u8);
            }
            debug!(fmt!("Buffer is 1= %?",ctx.buf.clone()));
        } 
        else {
            ctx.indata = ( data.len()>=5 && is_string_caseless_equal( data.slice(1,5), "data") );
            ctx.inerrors = ( data.len()>=7 && is_string_caseless_equal( data.slice(1,7), "errors"));
        }
    } 
    else {
        if ctx.indata {
            //ctx.buf = ~[] ;
            for data.iter().advance |ch| {
                ctx.buf.push(ch as u8);
            }
        }
    }
    true 
}

pub fn report_fail(data:~[u8] , e:@mut exp_entry) {

    debug!(fmt!("Data == %? ", str::from_bytes(data)));
    debug!(fmt!("Expected entry type == %d, name == %s", e.ftype, e.name.clone()) );
    io::print(fmt!("Expected bytecode == ") );
    for e.expected.mut_iter().advance |&expected| {
        io::print(fmt!("%? ", expected ));
    }
    debug!("\n")
}

pub fn run_test(ctx:@mut line_ctx, w:@Writer) {
    debug!("Entering: run_test");

    let mut error : css_error ;
    let mut params = css_params {
        /* ABI version of this structure */
        params_version : CSS_PARAMS_VERSION_1,

        /* The language level of the stylesheet */
        level: CSS_LEVEL_21,

        /* The charset of the stylesheet data, or NULL to detect */
        charset : Some(~"UTF-8"),
        /* URL of stylesheet */
        url : @"foo",
        /* Title of stylesheet */
        title : @"",

        /* Permit quirky parsing of stylesheet */
        allow_quirks : false,
        /* This stylesheet is an inline style */
        inline_style : false,

        /* URL resolution function */
        resolve : @resolve_url,

        /* Import notification function */
        import :None,

        /* Colour resolution function */
        color : None,

        /* Font resolution function */
        font : None,
        lwc_instance: Some(ctx.lwc_instance),
        propstrings_instance: None
    };

    //let lwc_instance = lwc() ;

    let css_instance = css::css_create( &params) ;


    error = css_instance.css_stylesheet_append_data(ctx.buf.clone());
    match error {
        CSS_OK=>{},
        CSS_NEEDDATA=>{},
        _=> { 
            debug!( fmt!("\n Failed appending data : %?",error) );
        }
    }

    error = css_instance.css_stylesheet_data_done();
    let mut pending_imports = false ;
    assert!( match error {
                CSS_OK=>{
                    true
                },
                CSS_IMPORTS_PENDING=>{
                    pending_imports=true; 
                    true 
                },
                _=>{false}
            } );

    while  pending_imports {
        
        let (error,o_str,_) = css_instance.css_stylesheet_next_pending_import() ;
        assert!( match error {
            CSS_OK=>{
                true
            },
            CSS_INVALID=>{
                true 
            },
            _=>{false}
        } );

        let url = o_str.get_or_default(@"") ;

        match error {
            CSS_OK=> {
                params.url = url.clone();

                let import = css::css_create(&params) ;
                
                assert!(    match css_instance.css_stylesheet_register_import(
                                                        Some(import.stylesheet)) {
                                CSS_OK=>{true},
                                _=>{false}
                            });

                pending_imports = true ;
            }
            _=>{ pending_imports = false ;} 
        }
    }

    let mut ptr = css_instance.stylesheet.rule_list ;
        
    loop {
        match ptr {
            None=>{ 
                return ;
            },
            Some(crule) => {
                match crule {
                    RULE_SELECTOR(rule) => {
                        dump_rule_selector(rule,w);
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;
                    },
                    RULE_CHARSET(rule) => {
                        dump_rule_charset(rule, w);
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;
                    },
                    RULE_IMPORT(rule) => {
                        dump_rule_import(rule, w);
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;
                    },
                    RULE_UNKNOWN(_)=>{
                        debug!(fmt!("Unhandled rule type %?", CSS_RULE_UNKNOWN)) ;
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;

                    },
                    RULE_MEDIA(_)=>{
                        debug!(fmt!("Unhandled rule type %?", CSS_RULE_MEDIA)) ;
                        let mut s = ~"| ";
                        s = s + ((CSS_RULE_MEDIA as int).to_str());
                        s = s + " ";
                        w.write_line(s);
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;

                    },
                    RULE_FONT_FACE(_)=>{
                        debug!(fmt!("Unhandled rule type %?", CSS_RULE_FONT_FACE)) ;
                        let mut s = ~"| ";
                        s = s + ((CSS_RULE_FONT_FACE as int).to_str());
                        s = s + " ";
                        w.write_line(s);
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;

                    },
                    RULE_PAGE(_)=>{
                        debug!(fmt!("Unhandled rule type %?", CSS_RULE_PAGE) ) ;
                        let mut s = ~"| ";
                        s =  s + ((CSS_RULE_PAGE as int).to_str());
                        s = s + " ";
                        w.write_line(s);
                        ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
                        loop ;

                    }
                }
            }   
        }
    }
}

pub fn dump_rule_selector(s:@mut css_rule_selector,w:@Writer )  {

    debug!("Entering: validate_rule_selector");
    let mut name : ~str = ~"" ;
    let mut ptr : ~str = ~"" ;

    // Build selector string
     debug!("Entering: validate_rule_selector: unsafe");
     debug!(fmt!("Parsed Rule List:%?",s.selectors.len().clone()));
	 let mut i : uint = 0;
	 let length = s.selectors.len();
     while i < length {
        dump_selector_list(s.selectors[i],&mut ptr) ;
        if ( i != (s.selectors.len()-1) ) {
            name = name + ptr + ", ";
            debug!(fmt!("if name == %?" , name));
        }
        else {
            name = name + ptr ;
            debug!(fmt!("else name == %?" , name));
        }
        ptr = ~"" ;
        i = i + 1;
    }
    let mut res = ~"| "; 
    res =  res + ((CSS_RULE_SELECTOR as int).to_str());
    res = res + " ";
    res = res + name.clone(); 
    w.write_line(res);

    if (s.style.is_some())
    {
        let style = s.style.get() ;

        let mut i = 0 ;
        let mut out: ~str = ~"";
        out.reserve_at_least(style.bytecode.len());

        out = ~"|  ";       
        while i < style.bytecode.len() {
            out = out + " ";
            out = out + "0x";
            out = out + (fmt!("%08x",(style.bytecode[i].clone() as uint ))); 
                    i += 1;                 
        }
        w.write_line(out);
    }
                
}

pub fn dump_rule_charset(s:@mut css_rule_charset, w:@Writer)  {

    debug!(fmt!("Parsed Rule List:%?", s.encoding.clone()));

    let mut res = ~"| ";
    res = res + ((CSS_RULE_CHARSET as int).to_str());
    res = res + " ";
    res = res + s.encoding.clone(); 

    w.write_line(res);
}

pub fn dump_rule_import(s:@mut css_rule_import, w:@Writer)  {
    debug!(fmt!("Parsed Rule List:%?", s.url.clone()));
    let mut res = ~"| ";
    res = res + ((CSS_RULE_IMPORT as int).to_str());
    res = res + " ";
    res = res + s.url.clone(); 
    w.write_line(res);
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
            ptr.push_char('~');
            ptr.push_char(' ');
        }

    }
    dump_selector(list, ptr);
}

fn dump_selector(selector:@mut css_selector, ptr:&mut ~str){
    let d:~[@mut css_selector_detail] = selector.data.clone();
    debug!(fmt!("Selector Data:%?",d));
  	let mut iter:uint = 0;
    while iter < d.len() {
		debug!(fmt!("Selector Data len:%?, Iter:%?",d.len(), iter));
        dump_selector_detail(d[iter], ptr, (iter != d.len()-1));
        iter += 1;
    }   
}

fn dump_selector_detail(detail:@mut css_selector_detail, ptr: &mut ~str, detail_next:bool) {
	debug!(fmt!("Detail == %?",detail));
    if detail.negate {
        str::push_str(ptr,&":not(");
    }
    match detail.selector_type {
        CSS_SELECTOR_ELEMENT=>{
            if lwc_string_length(detail.qname.name) == 1 && 
                    lwc_string_data(detail.qname.name)[0] == ('*' as u8) && 
                    !detail_next {
              
                str::push_str(ptr,lwc_string_data(detail.qname.name));
            }
            else if lwc_string_length(detail.qname.name) != 1 ||
                lwc_string_data(detail.qname.name)[0] != ('*' as u8) { 
                str::push_str(ptr,lwc_string_data(detail.qname.name));
            }
        },

        CSS_SELECTOR_CLASS=> {

            ptr.push_char('.');
            str::push_str(ptr,lwc_string_data( detail.qname.name));
        },

        CSS_SELECTOR_ID =>{
            
            ptr.push_char('#');
            str::push_str(ptr,lwc_string_data( detail.qname.name));
        },

        CSS_SELECTOR_PSEUDO_CLASS | CSS_SELECTOR_PSEUDO_ELEMENT =>{
            ptr.push_char(':' );
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            match detail.value_type {
                CSS_SELECTOR_DETAIL_VALUE_STRING=> {
                    if detail.string.is_some() {
                        ptr.push_char('(' );
                        //let String = copy detail.string;
                        str::push_str(ptr, (lwc_string_data( detail.string.unwrap() )));
                        ptr.push_char(')' );
                    }
                } ,
                _=>{
                    ptr.push_char('(' );
                    str::push_str(ptr,fmt!("%?n+%?", detail.a.clone(), detail.b.clone()));
                    ptr.push_char(')' );
                }
            }
        },

        CSS_SELECTOR_ATTRIBUTE=>{
            ptr.push_char('[');
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_EQUAL =>{
            ptr.push_char('[');
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char('=');
            ptr.push_char('"');
            str::push_str(ptr,(lwc_string_data( detail.string.unwrap() )));
            ptr.push_char('"');
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_DASHMATCH=>{
            ptr.push_char('[');
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char('|');
            ptr.push_char('=');
            ptr.push_char('"');
            str::push_str(ptr,(lwc_string_data( detail.string.unwrap() )));
            ptr.push_char('"');
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_INCLUDES=>{
            ptr.push_char('[');
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char('~');
            ptr.push_char('=');
            ptr.push_char('"');
            str::push_str(ptr,(lwc_string_data( detail.string.unwrap() )));
            ptr.push_char('"');
            ptr.push_char(']');
        },
        CSS_SELECTOR_ATTRIBUTE_PREFIX=>{
            ptr.push_char('[' );
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char('^' );
            ptr.push_char('=' );
            ptr.push_char('"' );
            str::push_str(ptr,(lwc_string_data( detail.string.unwrap() )));
            ptr.push_char('"' );
            ptr.push_char(']' );
        },
        CSS_SELECTOR_ATTRIBUTE_SUFFIX=>{
            ptr.push_char('[' );
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char('$' );
            ptr.push_char('=' );
            ptr.push_char('"' );
            str::push_str(ptr,(lwc_string_data( detail.string.unwrap() )));
            ptr.push_char('"' );
            ptr.push_char(']' );
        },
        CSS_SELECTOR_ATTRIBUTE_SUBSTRING=>{
            ptr.push_char('[' );
            str::push_str(ptr,lwc_string_data( detail.qname.name));
            ptr.push_char('*' );
            ptr.push_char('=' );
            ptr.push_char('"' );
            str::push_str(ptr,(lwc_string_data( detail.string.unwrap() )));
            ptr.push_char('"' );
            ptr.push_char(']' );
        }
    }
    if detail.negate {
        ptr.push_char(')');
    }
}

#[test]
fn tests1() {
    parse_auto(~"data/parse/tests1.dat",~"data/parse/test1_result.dat");
}

#[test]
fn atrules() {
    parse_auto(~"data/parse/atrules.dat",~"data/parse/atrules_result.dat");
}

#[test]
fn colours() {
    parse_auto(~"data/parse/colours.dat",~"data/parse/colours_result.dat");
}

#[test]
fn colours_hsl() {
    parse_auto(~"data/parse/colours-hsl.dat",~"data/parse/colours-hs1_result.dat");
}

#[test]
fn nth() {
    parse_auto(~"data/parse/nth.dat",~"data/parse/nth_result.dat");
}

#[test]
fn properties() {
    parse_auto(~"data/parse/properties.dat",~"data/parse/properties_result.dat");
}

#[test]
fn selectors() {
    parse_auto(~"data/parse/selectors.dat",~"data/parse/selectors_result.dat");
}
