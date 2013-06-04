extern mod std;
extern mod css;
extern mod wapcaplet;

use std::arc;
use css::css::*;
use css::css::css::*;
use wapcaplet::*;

use css::include::properties::*;
use css::include::types::*;
use css::include::font_face::*;
use css::bytecode::bytecode::*;
use css::utils::errors::*;
use css::select::common::*;
use css::select::dispatch::*;
use css::stylesheet::*;

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
	sheet:@mut css_stylesheet,
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

	n_sheets:u32,
	sheets:Option<@mut sheet_ctx>,

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

		n_sheets:0,
		sheets:None,

		media:0,
		pseudo_element:0,
		target:None,
		
		attr_class:lwc_attr_class.swap_unwrap(),
		attr_id:lwc_attr_id.swap_unwrap(),

		lwc_instance:lwc_ins.clone()
	};

	//css__parse_testfile(file,handle_line,ctx) ;
}

pub fn main() {
	io::println(fmt!("\n Starting select-auto test cases "));
}

#[test]
fn selection_test() {
	select_test(~"data/select/tests1.dat");
}


