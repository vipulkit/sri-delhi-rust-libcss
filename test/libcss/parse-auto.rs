extern mod std;
extern mod css;
extern mod wapcaplet;

use std::arc;
use css::css::*;
use css::css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use wapcaplet::*;

pub fn resolve_url(_:~str, rel:arc::RWARC<~lwc_string>) -> (css_error,Option<arc::RWARC<~lwc_string>>) {
	return (CSS_OK,Some(rel.clone()));
}

pub struct rule_s{
	rule_type:~str,
	name: ~str,
	bytecode:~[~str]
}

fn fill_params() -> css_params {
	let css_param = css_params {
		params_version : CSS_PARAMS_VERSION_1,
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
    parse(~"data/parse/selectors.dat");
}

fn parse(file_name: ~str) {
	let mut css = css_create_fn();
	let r:@Reader = io::file_reader(&Path(file_name)).get();
	let mut dataFlag = false;
	let mut expectedFlag = false;
	let mut resetFlag = false;
	let mut data_buf: ~[u8] = ~[];
	let mut expected_str: ~str = ~"";
	let mut vec_rule_s: rule_s = rule_s{
		rule_type: ~"",
		name: ~"",
		bytecode: ~[]
	};

	while !r.eof() {
		let buf = r.read_line();
		if buf == ~"#data" {
			dataFlag = true;
			expectedFlag = false; 
			resetFlag = false;
		}
		else if buf == ~"#errors" {
			dataFlag = false;
			expectedFlag = false;
			resetFlag = false;
		}
		else if buf == ~"#expected" {
			expectedFlag = true;
			dataFlag = false;
			resetFlag = false;

		}
		else if buf == ~"#reset" {
			dataFlag = false;
			expectedFlag = false;
			resetFlag = true;
		}
		else if buf == ~"" {
			dataFlag = false;
			expectedFlag = false;
			resetFlag = false;
		}
		else if dataFlag {
			for str::each_char(buf) |i| {
				data_buf.push(i as u8);
			}
			vec::reverse(data_buf);
			// io::println(fmt!("parse: data_buf is %?" , copy data_buf));
			let error = css.css_stylesheet_append_data(copy data_buf);
			data_buf = ~[];
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
			// io::println(fmt!("parse: css_stylesheet is %?" , copy css_stylesheet));
			let rule =  css_stylesheet.unwrap().rule_list;
			// io::println(fmt!("parse: rule is %?" , copy rule));
			loop {
				match rule {
					None => break,
					Some(x) => {
						match x {
							RULE_SELECTOR(x) => {
								// x.selectors[]
								// to be done after select module
								assert!(validate_rule_selector(x , copy vec_rule_s))
							},
							RULE_CHARSET(x) => {
								assert!(validate_rule_charset(x , copy vec_rule_s))
								//no fail condition
							},
							RULE_IMPORT(x) => {
								assert!(validate_rule_import(x , copy vec_rule_s))
								//no fail condition
							}
							_ => {}
						}
					}
				}
			}
		}
		else if expectedFlag {
			let (_ , new_buf) = str::slice_shift_char(buf);
			expected_str.push_str(new_buf);
			// io::println(fmt!("buf is %?" , new_buf));
		}

		if (resetFlag && !dataFlag && !expectedFlag) {
			// io::println(fmt!("expected_str is  = %? " , expected_str));
			vec_rule_s = parse_expected(expected_str);
			expected_str = ~"";
		}
		
	}
}

pub fn parse_expected(expected_str: ~str) -> rule_s{
	let mut expected_buf: ~[u8] = ~[];
	let mut string_from_buffer: ~str = ~"";
	for str::each_char(expected_str) |i| {
        expected_buf.push(i as u8);
    }

    let mut line_buffer: ~[~str] = ~[];
    // io::println(fmt!("%?" , expected_str));
    for str::each_line_any(expected_str) |k| {
    	line_buffer.push(k.to_owned());
    }
    if !line_buffer.is_empty() {
    	string_from_buffer = line_buffer.pop();
    }

    for str::each_split_str(string_from_buffer , "  ") |k| {
    	line_buffer.push(k.to_owned());
    }

    // io::println(fmt!("%?" , line_buffer));
    // vec::reverse(line_buffer);
    // io::println(fmt!("%?" , line_buffer));
    let mut vec_rule_s: rule_s = rule_s{
    	rule_type: ~"",
    	name: ~"",
    	bytecode: ~[]
    };

    if !line_buffer.is_empty() {
    	vec::reverse(line_buffer);
    	let first_string: ~str = line_buffer.pop();
    	vec::reverse(line_buffer);
    	// io::println(fmt!("first string%?" , first_string));
    	if first_string.len() > 1 {
    		vec_rule_s.rule_type = fmt!("%c" , first_string[1] as char);	
    	}
    	// vec_rule_s.rule_type = fmt!("%c" , first_string[1] as char);
    	if first_string.len() > 3 {
    		vec_rule_s.name = (first_string.substr(3 , (first_string.len() - 3))).to_owned();
    	}
    	// io::println(fmt!("rule type %?" , copy vec_rule_s.rule_type));
    	// io::println(fmt!("rule name%?" , copy vec_rule_s.name));
    }

    // io::println(fmt!("line buffer after if %?" , line_buffer));
    vec::reverse(line_buffer);
    vec_rule_s.bytecode = line_buffer;
    // io::println(fmt!("bytecode%?" , copy vec_rule_s.bytecode));
    return vec_rule_s
}

pub struct  stentry {
    off:uint,
    string:~str
}

pub fn validate_rule_selector(s:@mut css_rule_selector, rule: rule_s) -> bool {

	unsafe {
		match s.style {
			None => {
				if rule.bytecode.len() == 0 {
					//no style found & bytecode is also zero
					return true
				}
				else {
					return false
				}
			},
			Some(x) => {
				let len = x.bytecode.len();
				if (rule.bytecode.len() == len) {
						return true;
				}
				else {
					return false;
				}
			}
		}
	}
}

pub fn validate_rule_charset(s:@mut css_rule_charset, rule:rule_s ) -> bool {

	if rule.name != ~"*" {
		assert!(rule.name == s.encoding);	
	}
	true
}

pub fn validate_rule_import(s:@mut css_rule_import, rule:rule_s) -> bool {

	if rule.name != ~"*" {
		assert!(rule.name == s.url);
	}
	true
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