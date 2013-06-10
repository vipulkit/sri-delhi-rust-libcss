extern mod std;
extern mod css;
extern mod wapcaplet;

use std::arc;
use css::css::*;
//use css::css::css::*;
use wapcaplet::*;

//use css::include::properties::*;
use css::include::types::*;
//use css::include::font_face::*;
//use css::bytecode::bytecode::*;
use css::utils::errors::*;
use css::select::common::*;
//use css::select::dispatch::*;
//use css::stylesheet::*;

pub struct attribute {
	name:arc::RWARC<~lwc_string>,
	value:arc::RWARC<~lwc_string>
}

pub struct node {
	name:arc::RWARC<~lwc_string>,
	attrs:~[attribute],

	parent:Option<@mut node>,
	next:Option<@mut node>,
	prev:Option<@mut node>,
	children:Option<@mut node>,
	last_child:Option<@mut node>
}

pub struct sheet_ctx {
	sheet:@mut css,
	origin:css_origin,
	media:u64
}

pub struct line_ctx {
	explen:uint,
	expused:uint,
	exp:~str,

	intree:bool,
	insheet:bool,
	inerrors:bool,
	inexp:bool,

	tree:Option<@mut node>,
	current:Option<@mut node>,
	depth:u32,

	sheets:~[@mut sheet_ctx],

	media:u32,
	pseudo_element:u32,
	target:Option<@mut node>,
	
	attr_class:arc::RWARC<~lwc_string>,
	attr_id:arc::RWARC<~lwc_string>,

	lwc_instance:arc::RWARC<~lwc>
} 

pub fn select_test(file:~str) {
	let mut lwc_ins = lwc() ;
	let mut lwc_attr_class : Option<arc::RWARC<~lwc_string>> = None;
	let mut lwc_attr_id : Option<arc::RWARC<~lwc_string>> = None ;

	do lwc_ins.write |l| {
        lwc_attr_class = Some(l.lwc_intern_string(~"class"));
        lwc_attr_id = Some(l.lwc_intern_string(~"id"));
    }

	let mut ctx : @mut line_ctx = @mut line_ctx{
		explen:0,
		expused:0,
		exp:~"",

		intree:false,
		insheet:false,
		inerrors:false,
		inexp:false,

		tree:None,
		current:None,
		depth:0,

		sheets:~[],

		media:0,
		pseudo_element:0,
		target:None,
		
		attr_class:lwc_attr_class.swap_unwrap(),
		attr_id:lwc_attr_id.swap_unwrap(),

		lwc_instance:lwc_ins.clone()
	};

	let file_content_result = io::read_whole_file_str(&Path(file)) ;
	let mut file_content : ~str ;
	match file_content_result {
		Ok(x) => {
			file_content = x ;
		},
		Err(y) => {
			file_content = ~"" ;
			io::println(fmt!("\n Error opening file :%?",y));
			assert!(false) ;
		}
	}

	for str::each_line_any(file_content) |line| { 
		//io::println(fmt!("%?",line)); 
		handle_line(line,ctx);
	}

	if( ctx.tree.is_some() ) {
		run_test(ctx);
	}
}

pub fn handle_line(data:&str , ctx:@mut line_ctx) -> bool {
	let mut error : css_error ;
	let mut len : uint ; 
	if ( (data.len()>0) && (data[0] == ('#' as u8)) ) {

		if( ctx.intree ) {

			if( (data.len()>8) && (is_string_caseless_equal( data.slice(1,7), "errors")) ){
				ctx.intree = false;
				ctx.insheet = false;
				ctx.inerrors = true ;
				ctx.inexp = false;
			}
			else {
				/* Assume start of stylesheet */
				css__parse_sheet(ctx, data.slice(1,data.len()-1) );

				ctx.intree = false;
				ctx.insheet = true;
				ctx.inerrors = false;
				ctx.inexp = false;
			}
		}
		else if (ctx.insheet) {

			if( (data.len()>6) && (is_string_caseless_equal( data.slice(1,6), "errors")) ){
				len = unsafe { ctx.sheets.len() -1 } ;
				assert!( 
						match ctx.sheets[len].sheet.css_stylesheet_data_done() {
								CSS_OK=>{true},
								_=>{false}
						});
				ctx.intree = false;
				ctx.insheet = false;
				ctx.inerrors = true ;
				ctx.inexp = false;
			}
			else if (	(data.len()>2) && (is_string_caseless_equal( data.slice(1,2), "ua")) ||
						(data.len()>4) && (is_string_caseless_equal( data.slice(1,4), "user")) ||
						(data.len()>6) && (is_string_caseless_equal( data.slice(1,6), "author")) ) {
				
				len = unsafe { ctx.sheets.len() -1 } ;
				assert!( 
						match ctx.sheets[len].sheet.css_stylesheet_data_done() {
							CSS_OK=>{true},
							_=>{false}
						});
				css__parse_sheet(ctx, data.slice(1,data.len()-1) );
			}
			else {
				len = unsafe { ctx.sheets.len() -1 } ;
				let mut error = ctx.sheets[len].sheet.css_stylesheet_append_data(data.to_bytes());
				assert!( match error {
							CSS_OK=>{true},
							CSS_NEEDDATA=>{true},
							_=>{false}
						 });
			}
		}
		else if (ctx.inerrors) {
			ctx.intree = false;
			ctx.insheet = false;
			ctx.inerrors = false;
			ctx.inexp = true;
		}
		else if (ctx.inexp) {
			/* This marks end of testcase, so run it */
			run_test(ctx);

			ctx.expused = 0;

			ctx.intree = false;
			ctx.insheet = false;
			ctx.inerrors = false;
			ctx.inexp = false;
		}
		else {
			/* Start state */
			if( (data.len()>4) && (is_string_caseless_equal( data.slice(1,4), "tree")) ){

				css__parse_tree(ctx, data.slice(5, data.len()-1) );
				ctx.intree = true;
				ctx.insheet = false;
				ctx.inerrors = false ;
				ctx.inexp = false;
			}
		}
	}
	else {
		if ( ctx.intree ){
			/* Not interested in the '|' */
			css__parse_tree_data(ctx, data.slice(1,data.len()-1) );
		}
		else if ( ctx.insheet ) {
			len = unsafe { ctx.sheets.len() -1 } ;
			error = ctx.sheets[len].sheet.css_stylesheet_append_data(data.to_bytes());
			assert!( match error {
						CSS_OK=>{true},
						CSS_NEEDDATA=>{true},
						_=>{false}
					 });
		}
		else if ( ctx.inexp ) {
			css__parse_expected(ctx, data );
		}
	}
	true 
}

pub fn css__parse_expected(ctx:@mut line_ctx, data:&str) {

}

pub fn css__parse_media_list(data:&str , ctx:@mut line_ctx) -> uint {

	// ' '	(0x20)	space (SPC)
	// '\t'	(0x09)	horizontal tab (TAB)
	// '\n'	(0x0a)	newline (LF)
	// '\v'	(0x0b)	vertical tab (VT)
	// '\f'	(0x0c)	feed (FF)
	// '\r'	(0x0d)	carriage return (CR)
	let mut len : uint = 0 ;
	let mut result : u64 = 0 ;
	while len < data.len() {

		/* consume a medium */
		if ( (data[len]!=0x20) && (data[len]!=0x09) && (data[len]!=0x0a) && 
			 (data[len]!=0x0b) && (data[len]!=0x0c) && (data[len]!=0x0d)  && data.len()>len) {
			if( data[len]!= (',' as u8)) {
				len += 1;
				loop ;
			}
		}

		if ( (data.len()>(10+len)) && is_string_caseless_equal( data.slice(len,len+10), "projection") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(8+len)) && is_string_caseless_equal( data.slice(len,len+8), "handheld") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(8+len)) && is_string_caseless_equal( data.slice(len,len+8), "embossed") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(7+len)) && is_string_caseless_equal( data.slice(len,len+7), "braille") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(6+len)) && is_string_caseless_equal( data.slice(len,len+6), "speech") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(6+len)) && is_string_caseless_equal( data.slice(len,len+6), "screen") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(5+len)) && is_string_caseless_equal( data.slice(len,len+5), "print") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(5+len)) && is_string_caseless_equal( data.slice(len,len+5), "aural") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(3+len)) && is_string_caseless_equal( data.slice(len,len+3), "tty") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(3+len)) && is_string_caseless_equal( data.slice(len,len+3), "all") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else if ( (data.len()>(2+len)) && is_string_caseless_equal( data.slice(len,len+2), "tv") ) {
			result = result | (CSS_MEDIA_PROJECTION as u64) ;
		}
		else {
			// unknown media type
			io::println("\n Unknown Media type encountered");
			assert!(false);
		}

		/* Consume whitespace */
		while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
			 	(data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) ) && data.len()>len {
				len += 1;
		}

		/* Stop if we've reached the end */
		if ( data.len() <= len ) {
			break;
		}

		if data[len] == (',' as u8) {
			len += 1;
		}

		/* Consume whitespace */
		while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
			 	(data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) ) && data.len()>len {
				len += 1;
		}	
	}
	
	ctx.media = result as u32 ;
	len
}

pub fn css__parse_pseudo_list(data:&str,ctx:@mut line_ctx) -> uint {
	/*
	const char *p = *data;
	const char *end = p + *len;

	// <pseudo> [ ',' <pseudo> ]* 

	*element = CSS_PSEUDO_ELEMENT_NONE;

	while (p < end) {
		const char *start = p;

		// consume a pseudo 
		while (isspace(*p) == false && *p != ',')
			p++;

		// Pseudo elements 
		if (p - start == 12 &&
				strncasecmp(start, "first-letter", 12) == 0)
			*element = CSS_PSEUDO_ELEMENT_FIRST_LETTER;
		else if (p - start == 10 &&
				strncasecmp(start, "first-line", 10) == 0)
			*element = CSS_PSEUDO_ELEMENT_FIRST_LINE;
		else if (p - start == 6 &&
				strncasecmp(start, "before", 6) == 0)
			*element = CSS_PSEUDO_ELEMENT_BEFORE;
		else if (p - start == 5 &&
				strncasecmp(start, "after", 5) == 0)
			*element = CSS_PSEUDO_ELEMENT_AFTER;
		else
			assert(0 && "Unknown pseudo");

		// Consume whitespace 
		while (p < end && isspace(*p))
			p++;

		// Stop if we've reached the end 
		if (p == end || *p != ',')
			break;

		// Consume comma
		p++;

		// Consume whitespace 
		while (p < end && isspace(*p))
			p++;
	}

	*data = p;
	*len = end - p; */
	0
}

pub fn css__parse_tree(ctx:@mut line_ctx, data:&str) {
	/* [ <media_list> <pseudo>? ] ? */
	ctx.media = CSS_MEDIA_ALL as u32;
	ctx.pseudo_element = CSS_PSEUDO_ELEMENT_NONE as u32;

	/* Consume any leading whitespace */
	let mut data2 = data.trim_left();

	let mut length_processed : uint = 0 ;
	if (data2.len()>0) {
		length_processed = css__parse_media_list(data2,ctx);
	}

	if length_processed < data2.len() {
		css__parse_pseudo_list(data2.slice(length_processed,data2.len()-1),ctx);
	}
}

pub fn css__parse_tree_data(ctx:@mut line_ctx, data:&str) {


}

pub fn css__parse_sheet(ctx:@mut line_ctx,data:&str) {

}

pub fn run_test(ctx:@mut line_ctx) {

}

pub fn main() {
	io::println(fmt!("\n Starting select-auto test cases "));
}

pub fn is_string_caseless_equal(a : &str , b : &str ) -> bool {

    if ( a.len() != b.len() ) {
        return false ;
    }
    
    let mut i :uint = a.len() ;
    for uint::range(0,i) |e| {
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


#[test]
fn selection_test() {
	select_test(~"data/select/tests1.dat");
}


