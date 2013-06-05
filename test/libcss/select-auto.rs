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
//use css::select::common::*;
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
	let mut len = unsafe { ctx.sheets.len() -1 } ;
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
				
				assert!( 
						match ctx.sheets[len].sheet.css_stylesheet_data_done() {
							CSS_OK=>{true},
							_=>{false}
						});
				css__parse_sheet(ctx, data.slice(1,data.len()-1) );
			}
			else {
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

pub fn css__parse_tree(ctx:@mut line_ctx, data:&str) {

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


