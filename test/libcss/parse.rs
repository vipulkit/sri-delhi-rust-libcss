extern mod std;
extern mod extra;
extern mod css;
extern mod wapcaplet;

use std::io::*;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use wapcaplet::*;

pub fn resolve_url(_:@str, rel:@mut wapcaplet::lwc_string) -> (css_error,Option<@mut wapcaplet::lwc_string>) {
    return (CSS_OK,Some(rel.clone()));
}

fn fill_params() -> css_params {
    let css_param = css_params {
        params_version : CSS_PARAMS_VERSION_1,
        level: CSS_LEVEL_21,
        charset : Some(~"UTF-8"),
        url : @"foo",
        title : @"",
        allow_quirks : false,
        inline_style : false,
        resolve : @resolve_url,
        import : None,
        color : None,
        font : None
    };
    return css_param;
}

fn css_create_fn() -> @mut css{
    let lwc = wapcaplet::lwc();
    let css = css::css_create( &fill_params() , Some(lwc));
    css
}

fn main() {
    println("parse");
    // parse(~"../data/parse/atrules.dat");
}

fn parse(file_name: ~str) {
    let css = css_create_fn();
    let r:@Reader = file_reader(&Path(file_name)).get();
    let mut dataFlag = false;
    // let mut expectedFlag: bool;

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
            for buf.iter().advance |i| {
                final_buf.push(i as u8);
            }
            final_buf.reverse();
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
