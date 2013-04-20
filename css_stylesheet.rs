#[link(name = "css_stylesheet", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod parserutils_inputstream;
extern mod wapcaplet;
extern mod std ;


use std::arc;
use css_enum::* ;
use wapcaplet::*;
use parserutils_inputstream::*;

pub struct css_rule {
	parent_rule:Option<@css_rule> ,
	parent_stylesheet:Option<@css_stylesheet>,
	prev:Option<@css_rule>,
	next:Option<@css_rule>,
	rule_type:uint,
	index:uint,
	items:uint
}

pub struct css_qname {
	name:~str,
	ns:~str
}

pub struct css_selector {
	combinator:Option<@css_selector>,
	rule:Option<@css_rule>,
	specificity:uint,

	//css_selector_detail - struct merged
	qname:css_qname,
	selector_type:uint,
	comb:uint,
	next:uint,
	value_type:uint,
	negate:uint,

	//css_selector_detail_value - struct merged
	string:~str,
	a:int,
	b:int
}

pub struct css_selector_hash {
	elements:@[@css_selector],
	classes:@[css_selector],
	ids:@[@css_selector],
	universal:@[@css_selector]
}

pub struct css_style {
	bytecode:~[u32],
	used:uint,
	allocated:uint
	//sheet:Option<@css_stylesheet>
}

pub struct css_stylesheet {
	selectors:css_selector_hash,
	rule_count:uint,
	rule_list:Option<@css_rule>,
	last_rule:Option<@css_rule>,
	disabled:bool,
	url:~str,
	title:~str,
	level:css_language_level,
	quirks_allowed:bool,
	quirks_used:bool,
	inline_style:bool,
	size:uint,
	cached_style:~[css_style]
}

impl css_stylesheet {

}

fn main () {
	io::println(fmt!("\n hellow world"));
}