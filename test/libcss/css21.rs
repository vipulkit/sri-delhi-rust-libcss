extern mod std;
extern mod css;
extern mod wapcaplet;

use core::io::*;
use std::arc;
use css::css::*;
use css::css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use css::parse::properties::*;
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
	io::println("css21");	
	css(~"../data/css/Allzengarden.css");
}

fn css(file_name: ~str) {
	let mut css = css_create_fn();
	let CHUNK_SIZE = 4096;
    let mut buf: ~[u8];
    let r:@Reader = io::file_reader(&Path(file_name)).get(); 
    r.seek(0 , SeekEnd);
    let mut len = r.tell();
    r.seek(0 , SeekSet);
    while len>0 {
        buf = r.read_bytes(CHUNK_SIZE as uint);
        len -= buf.len();
        let error = css.css_stylesheet_append_data(buf);
        match error {
        	CSS_OK => {},
        	CSS_NEEDDATA => {},
        	_ => {assert!(false);}
        }
    }
    buf = r.read_bytes(len as uint);
    let error = css.css_stylesheet_append_data(buf);
    match error {
    	CSS_OK => {},
        CSS_NEEDDATA => {},
        _ => {assert!(false);}
    }

    let (error , css_stylesheet) = css.css_stylesheet_data_done();

    match error {
		CSS_OK | CSS_IMPORTS_PENDING => {},
		_ => {assert!(false);}
	}

	match error {
		CSS_IMPORTS_PENDING => {
			// check for next_pending_import
		},
		_ =>{}
	}
} 


#[test]
fn allzengarden() {
	css(~"../data/css/Allzengarden.css");
}

#[test]
fn badcomment() {
	css(~"../data/css/Badcomment.css");
}

#[test]
fn Blocks() {
	css(~"../data/css/Blocks.css");
}

#[test]
fn Color() {
	css(~"../data/css/Color.css");
}

#[test]
fn Fontface() {
	css(~"../data/css/Fontface.css");
}

#[test]
fn Malformed() {
	css(~"../data/css/Malformed.css");
}

#[test]
fn Simple() {
	css(~"../data/css/Simple.css");
}