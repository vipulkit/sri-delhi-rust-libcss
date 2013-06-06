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

pub fn fill_params() -> css_params {
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

fn main() {
    io::println("parse");
}

fn css_create_fn() -> ~css{
    let mut lwc = wapcaplet::lwc();
    let css = css_create(fill_params() , Some(lwc));
    css
}

fn parse(file_name: ~str) {
    let mut css = css_create_fn();
    let r:@Reader = io::file_reader(&Path(file_name)).get();
    let mut dataFlag = false;
    // let mut expectedFlag = false;

    while !r.eof() {
        let buf = r.read_line();
        if buf == ~"#data" {
            dataFlag = true;
            // expectedFlag = false; 
        }
        else if buf == ~"#errors" {
            dataFlag = false;
            // expectedFlag = false;
        }
        else if buf == ~"#expected" {
            // expectedFlag = true;
            dataFlag = false;

        }
        else if buf == ~"#reset" {
            dataFlag = false;
            // expectedFlag = false;
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
            let error = css.css_stylesheet_data_done();

            match error {
                CSS_OK => {},
                _ => {assert!(false);}
            }
        }
    }
}


#[test]
fn parse2_au() {
    parse(~"data/parse2/au.dat");
}

#[test]
fn parse2_bg() {
    parse(~"data/parse2/bg.dat");
}

#[test]
fn parse2_bgpos() {
    parse(~"data/parse2/bgpos.dat");
}

#[test]
fn parse2_border() {
    parse(~"data/parse2/border.dat");
}

#[test]
fn parse2_comments() {
    parse(~"data/parse2/comments.dat");
}

#[test]
fn parse2_eof() {
    parse(~"data/parse2/eof.dat");
}

#[test]
fn parse2_font() {
    parse(~"data/parse2/font.dat");
}

#[test]
fn parse2_illegal_values() {
    parse(~"data/parse2/illegal-values.dat");
}

#[test]
fn parse2_list() {
    parse(~"data/parse2/list.dat");
}

#[test]
fn parse2_malformed_declarations() {
    parse(~"data/parse2/malformed-declarations.dat");
}

#[test]
fn parse2_margin() {
    parse(~"data/parse2/margin.dat");
}

#[test]
fn parse2_multicol() {
    parse(~"data/parse2/multicol.dat");
}

#[test]
fn parse2_outline() {
    parse(~"data/parse2/outline.dat");
}

#[test]
fn parse2_padding() {
    parse(~"data/parse2/padding.dat");
}

#[test]
fn parse2_selectors() {
    parse(~"data/parse2/selectors.dat");
}

#[test]
fn parse2_tests1() {
    parse(~"data/parse2/tests1.dat");
}

#[test]
fn parse2_unknown_properties() {
    parse(~"data/parse2/unknown-properties.dat");
}