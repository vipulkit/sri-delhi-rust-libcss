extern mod std;
extern mod extra;
extern mod css;
extern mod wapcaplet;
extern mod dump;

use std::io;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use dump::*;

pub struct line_ctx {
    buf:~[u8],

    expused: uint,

    indata:bool,
    inerrors:bool,
    inrule: bool
}

pub fn resolve_url(_:@str, rel:@mut wapcaplet::lwc_string) -> (css_error,Option<@mut wapcaplet::lwc_string>) {
    return (CSS_OK,Some(rel.clone()));
}

pub fn check_newline(x: &u8) -> bool { *x == ('\n' as u8) }

fn match_vec_u8(vector: &[u8] , string: &str) -> bool {

    debug!("Entering: match_vec_u8 :: vector == %?, found_string == %?", vector, string);

    let string_vector = string.as_bytes();
    if string_vector.len() != vector.len() {
        debug!("Exiting: match_vec_u8 (1)");
        return false;
    }

	let mut iter_both = vector.iter().zip(string_vector.iter());
    for iter_both.advance() |(e , f)| {
        if e != f {
            debug!("Exiting: match_vec_u8 (2)");
            return false;
        }
    }
    debug!("Exiting: match_vec_u8 (3)");
    true
}

pub fn css_create_params() -> css_params {
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
        font : None,
        lwc_instance: None,
        propstrings_instance: None
    };
    return css_param;
}

fn main() {
    debug!("parse");
}

fn create_css() -> @mut css{
    debug!("Entering: create_css");
    let css = css::css_create( &(css_create_params()));
    css
}

pub fn handle_line(args: &[u8],  ctx:@mut line_ctx, w:@Writer)->bool {
    debug!("Entering: handle_line");
    let data : &[u8] = args ;
    // unsafe{debug!(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    if  (data.len() == 0) {
        debug!("error");
        return true;
    }

    if (data[0] == '#' as u8) {
        if (ctx.indata  && match_vec_u8(data , &"#errors")) {
            debug!("Entering: handle_line :: if (ctx.indata  && match_vec_u8(data , &\"#errors\"))");
            ctx.indata = false;
            ctx.inerrors = false;

            debug!("Entering: handle_line :: if (ctx.errors)");
            w.write_line("#expected");
            run_test(ctx.buf.clone(), w);
            w.write_line("#reset");
            ctx.buf=~[];
        }
        else if (ctx.indata) {
            debug!("Entering: handle_line :: if (ctx.indata) (1)");
            ctx.buf = ctx.buf + data.clone();
            ctx.buf.push('\n' as u8);
        } 
        else {
            debug!("Entering: handle_line :: else (1)");
            ctx.indata = match_vec_u8(data , &"#data");
            ctx.inerrors = match_vec_u8(data , &"#errors");
        }
    }
    else {
        debug!("Entering: handle_line :: else (2)");

        if ctx.indata {
            debug!("Entering: handle_line :: if (ctx.indata) (2)");
            ctx.buf = ctx.buf + data.clone();
            ctx.buf.push('\n' as u8);
        }
    }

    return true;
}

fn testMain(fileName: ~str, writeResultFile: ~str) {
    debug!(~"testMain : "+ fileName);
    let w:@Writer = io::file_writer(&Path(writeResultFile),[io::Create, io::Truncate]).unwrap();
    let ctx = @mut line_ctx
    {
        buf:~[],
        expused: 0,
        indata:false,
        inerrors: false,
        inrule: false
    };

    let file_content_result = io::read_whole_file(&Path(fileName)) ;
    let mut file_content : ~[u8] ;
    match file_content_result {
        Ok(x) => {
            file_content = x ;
        },
        Err(_) => {
            file_content = ~[] ;
            debug!(fmt!("\n Error opening file"));
            assert!(false) ;
        }
    }        
    let mut vec_lines = file_content.split_iter(check_newline) ;

    for vec_lines.advance |each_line| {
        handle_line(each_line,ctx, w);
    }
    
    if ctx.buf.len() > 0 {
        w.write_line("#expected");
        run_test(ctx.buf.clone(),w);
        w.write_line("#reset");
    }
}

pub fn run_test(data:~[u8], w:@Writer) {
    debug!("Entering :: run_test");
    // debug!("\n == data == %?" , str::from_bytes(data));
    
    let css = create_css();
    let mut buf: ~str;
    let mut error = css.css_stylesheet_append_data(data);
    match error {
        CSS_OK | CSS_NEEDDATA => {},
        _ => {assert!(false);}
    }

    error = css.css_stylesheet_data_done();
    //debug!(fmt!("error from css_stylesheet_data_done: %?" , error));
    match error {
        CSS_OK => {},
        _ => {assert!(false);}
    }

    buf = dump_sheet(css.stylesheet);
    if (buf.len() > 0)
    {
       w.write_line(buf.trim_right_chars(&'\n'));
    }
    //debug!(fmt!("\n == sheet ==%?=" , buf));
    let mut dvec : ~[~[u8]] = ~[];
    for buf.any_line_iter().advance |s| {
        dvec.push(s.as_bytes().to_owned());
    }
    // debug!("============================================================" );
    // debug!(fmt!(" == sheet ==%?=" , vec));
    // debug!("============================================================" );


}


#[test]
fn au() {
    testMain(~"data/parse2/au.dat", ~"data/parse2/au_result.dat");
}

#[test]
fn bg() {
    testMain(~"data/parse2/bg.dat", ~"data/parse2/bg_result.dat");
}

#[test]
fn bgpos() {
    testMain(~"data/parse2/bgpos.dat", ~"data/parse2/bgpos_result.dat");
}

#[test]
fn border() {
    testMain(~"data/parse2/border.dat", ~"data/parse2/border_result.dat");
}

#[test]
fn comments() {
    testMain(~"data/parse2/comments.dat", ~"data/parse2/comments_result.dat");
}

#[test]
fn eof() {
    testMain(~"data/parse2/eof.dat", ~"data/parse2/eof_result.dat");
}

#[test]
fn font() {
    testMain(~"data/parse2/font.dat", ~"data/parse2/font_result.dat");
}

#[test]
fn illegal_values() {
    testMain(~"data/parse2/illegal-values.dat", ~"data/parse2/illegal-values_result.dat");
}

#[test]
fn list() {
    testMain(~"data/parse2/list.dat", ~"data/parse2/list_result.dat");
}

#[test]
fn malformed_declarations() {
    testMain(~"data/parse2/malformed-declarations.dat", ~"data/parse2/malformed-declarations_result.dat");
}

#[test]
fn margin() {
    testMain(~"data/parse2/margin.dat", ~"data/parse2/margin_result.dat");
}

#[test]
fn multicol() {
    testMain(~"data/parse2/multicol.dat", ~"data/parse2/multicol_result.dat");
}

#[test]
fn outline() {
    testMain(~"data/parse2/outline.dat", ~"data/parse2/outline_result.dat");
}

#[test]
fn padding() {
    testMain(~"data/parse2/padding.dat", ~"data/parse2/padding_result.dat");
}

#[test]
fn selectors() {
    testMain(~"data/parse2/selectors.dat", ~"data/parse2/selectors_result.dat");
}

#[test]
fn tests1() {
    testMain(~"data/parse2/tests1.dat", ~"data/parse2/tests1_result.dat");
}
#[test]
fn unknown_properties() {
    testMain(~"data/parse2/unknown-properties.dat", ~"data/parse2/unknown-properties_result.dat");
}
