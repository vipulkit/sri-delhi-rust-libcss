extern mod std;
extern mod extra;
extern mod css;
extern mod wapcaplet;

use std::io::*;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use css::parse::propstrings::css_propstrings;
use wapcaplet::*;

pub fn resolve_url(_:&str, rel:uint) -> (css_error,Option<uint>) {
    return (CSS_OK,Some(rel.clone()));
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

fn css_create_fn(stylesheet_vector:&mut ~[css_stylesheet]) -> ~css{
    
    let css = css::css_create(stylesheet_vector, &fill_params());
    css
}

fn main() {
    println("parse");
    // parse(~"data/parse/tests1.dat");
    // parse(~"data/parse/atrules.dat");
    // parse(~"data/parse/colours.dat");
    // parse(~"data/parse/colours-hsl.dat");
    // parse(~"data/parse/nth.dat");
    // parse(~"data/parse/properties.dat");
    // parse(~"data/parse/selectors.dat");

}

fn parse(file_name: ~str) {
    let mut stylesheet_vector:~[css_stylesheet]=~[];
    let mut css_rule_data_list:~[~css_rule_data_type]=~[];
    let mut lwc = lwc();
    let mut css = css_create_fn(&mut stylesheet_vector);
    let propstring = css_propstrings::css_propstrings(&mut lwc);
    let r:@Reader = file_reader(&Path(file_name)).unwrap();
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
            for  i in buf.iter() {
                final_buf.push(i as u8);
            }
            final_buf.reverse();
            let error = css.css_stylesheet_append_data(&mut stylesheet_vector, &mut css_rule_data_list, &mut lwc , &propstring , final_buf);
            match error {
                CSS_OK => {},
                CSS_NEEDDATA => {},
                _ => {assert!(false);}
            }
            let error = css.css_stylesheet_data_done(&mut stylesheet_vector, &mut css_rule_data_list, &mut lwc , &propstring);

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
