extern mod std;
extern mod css;
extern mod wapcaplet;

use std::arc;
use css::css::*;
use css::css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use wapcaplet::*;

pub fn resolve_url(_:~str, rel:arc::RWARC<~lwc_string>) -> (css_result,Option<arc::RWARC<~lwc_string>>) {
	return (CSS_OK,Some(rel.clone()));
}

fn fill_params() -> css_params {
	let css_param = css_params {
		params_version : 1,
		level: CSS_LEVEL_21,
		charset : Some(~"UTF-8"),
		url : ~"foo",
		title : ~"",
		allow_quirks : false,
		inline_style : false,
		resolve : @resolve_url,
		import : None,
		color : None,
		font : None
	};
	return css_param;
}

fn css_create_fn() -> ~css{
	let css = css_create(fill_params());
	css
}

fn main() {
    io::println("parse");
    parse(~"../data/parse/atrules.dat");
}

fn parse(file_name: ~str) {
	let mut css = css_create_fn();
	let r:@Reader = io::file_reader(&Path(file_name)).get();
	let mut dataFlag = false;
	let mut expectedFlag = false;

	while !r.eof() {
		let buf = r.read_line();
		if buf == ~"#data" {
			dataFlag = true;
			expectedFlag = false; 
		}
		else if buf == ~"#errors" {
			dataFlag = false;
			expectedFlag = false;
		}
		else if buf == ~"#expected" {
			expectedFlag = true;
			dataFlag = false;

		}
		else if buf == ~"#reset" {
			dataFlag = false;
			expectedFlag = false;
		}
		else if dataFlag {
			let mut final_buf :~[u8] = ~[];
			for str::each_char(buf) |i| {
				final_buf.push(i as u8);
			}
			vec::reverse(final_buf);
			let error = css.css_stylesheet_append_data(final_buf);
			match error {
				CSS_OK => {},
				CSS_NEEDDATA => {},
				_ => {assert!(false);}
			}
			let (error , css_stylesheet) = css.css_stylesheet_data_done();

			match error {
				CSS_OK => {},
				_ => {assert!(false);}
			}
			let rule =  css_stylesheet.unwrap().rule_list;
			loop {
				match rule {
					None => break,
					Some(x) => {
						match x {
							RULE_SELECTOR(x) => {
								// x.selectors[]
								// to be done after select module
							},
							RULE_CHARSET(_) => {
								//no fail condition
							},
							RULE_IMPORT(_) => {
								//no fail condition
							}
							_ => {}
						}
					}
				}
			}
		}
	}
}





pub struct  stentry {
    off:uint,
    string:~str
}

pub struct exp_entry {
	name:~str,
	bytecode:~[u32],
	bcused:uint,
	stringtab:~[stentry],
	stused:uint
} 

pub fn validate_rule_selector(s:@mut css_rule_selector, e:@mut exp_entry ) -> bool {

	/*
	char name[MAX_RULE_NAME_LEN];
	char *ptr = name;
	uint32_t i;

	// Build selector string 
	for (i = 0; i < s->base.items; i++) {
		dump_selector_list(s->selectors[i], &ptr);
		if (i != (uint32_t) (s->base.items - 1)) {
			memcpy(ptr, ", ", 2);
			ptr += 2;
		}
	}
	*ptr = '\0';

	// Compare with expected selector 
	if (strcmp(e->name, name) != 0) {
		printf("FAIL Mismatched names\n"
		       "     Got name '%s'. Expected '%s'\n",
		       name, e->name);
		return true;
	}
	*/
	// Now compare bytecode 
	unsafe {
	match s.style {
		None=> {
			if (e.bytecode.len() == 0)  {
				// no style found & bytecode is also zero ;
				return true ;
			}
			else {
				return false ;
			}
		},
		Some(x)=>{
			if (e.bytecode.len() == 0) {
				if (x.bytecode.len() == 0) {
					return true ;
				}
				else{
					return false ;
				}
			}
			else {
				if (x.bytecode.len() == 0) {
					return false ;
				}
				else{
					if( e.bytecode.len() < e.bcused ) {
						// used variable overreaching available limit 
						return false ;
					}

					if( x.bytecode.len() < x.used ) {
						// used variable overreaching available limit 
						return false ;
					}
					let mut j : uint = 0 ;
					let mut i : uint = 0 ;

					while(i<e.bcused) {

						while (j<e.stused) {
						 	if (e.stringtab[j].off == i) {
								break;
							}
							j += 1; 
						 }

						if (j != e.stused) {

							if( x.sheet.is_none() ) {
								return false ;
							}
							let mut (res,op) = x.sheet.get().css__stylesheet_string_get(x.bytecode[i] as uint);
							let mut p = if op.is_some() { op.unwrap() }	else { ~"" } ;		

							if( p.len() != e.stringtab[j].string.len() ) {
								return false ;
							}
							let mut k =0 ;
							while ( k<p.len() ) {
								if ( p[i] != (e.stringtab[j].string)[i] ) {
									return false ;
								}
								k += 1;
							}

							i += 1;
						} else if (x.bytecode[i] != e.bytecode[i] ) {
							// printf("FAIL Bytecode differs\n"
							//        "    Bytecode differs at %u\n	",
							// 	(int) i);
							while (i < e.bcused) {
								// printf("%.2x ", 
								// 	((uint8_t *) s->style->bytecode)[i]);
								i += 1;
							}
							return true;
						}
						i = i+1;
					}
				}
			}
		}
	} 
	false
	}
}

pub fn validate_rule_charset(s:@mut css_rule_charset, e:@mut exp_entry) -> bool {

	unsafe {
	if( e.name.len() != s.encoding.len() ) {
		return false ;
	}
	let mut i =0 ;
	while ( i<s.encoding.len() ) {
		if ( s.encoding[i] != e.name[i] ) {
			return false ;
		}
		i += 1;
	}
	return true ;
	}
}

pub fn validate_rule_import(s:@mut css_rule_import, e:@mut exp_entry) -> bool {

	unsafe {
	if( e.name.len() < s.url.len() ) {
		return false ;
	}
	let mut i =0 ;
	while ( i<s.url.len() ) {
		if ( s.url[i] != e.name[i] ) {
			return false ;
		}
		i += 1;
	}
	true
	}
}

#[test]
fn parse_tests1() {
	parse(~"data/parse/tests1.dat");
}

#[test]
fn parse_atrules() {
	parse(~"data/parse/atrules.dat");
}

#[test]
fn parse_colours() {
	parse(~"data/parse/colours.dat");
}

#[test]
fn parse_colours_hsl() {
	parse(~"data/parse/colours-hsl.dat");
}

#[test]
fn parse_nth() {
	parse(~"data/parse/nth.dat");
}

#[test]
fn parse_properties() {
	parse(~"data/parse/properties.dat");
}

#[test]
fn parse_selectors() {
	parse(~"data/parse/selectors.dat");
}