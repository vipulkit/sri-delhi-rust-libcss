extern mod std;
extern mod extra;
extern mod css;
extern mod wapcaplet;
extern mod dump;

use std::io;
use std::vec;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use dump::*;
use wapcaplet::*;
use css::parse::propstrings::css_propstrings;

pub struct line_ctx {
    buf:~[u8],

    exp:~[~[u8]],
    expused: uint,

    indata:bool,
    inexp:bool,
    inerrors:bool,
    inrule: bool
}

pub fn resolve_url(_:&str, rel:uint) -> (css_error,Option<uint>) {
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
    for (e, f) in iter_both {
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
        url : ~"foo",
        title : ~"",
        allow_quirks : false,
        inline_style : false,
        resolve : @resolve_url,
        import : None,
        color : None,
        font : None,
    };
    return css_param;
}

fn main() {
    debug!("parse");
}

fn create_css() -> ~css{
    debug!("Entering: create_css");
    let css = css::css_create( &css_create_params());
    css
}

pub fn handle_line(args: &[u8],  ctx:@mut line_ctx)->bool {
    debug!("Entering: handle_line");
    let data : &[u8] = args ;
    // unsafe{debug!(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    if  (data.len() == 0) {
        debug!("error");
        return true;
    }

    if (data[0] == '#' as u8) {
        if (ctx.inexp) {
            /* This marks end of testcase, so run it */
            debug!("Entering: handle_line :: if (ctx.inexp)");

            run_test(ctx.buf.clone() , ctx.exp.clone());
            ctx.exp= ~[];
            ctx.buf=~[];
        }
        if (ctx.indata  && match_vec_u8(data , &"#errors")) {
            debug!("Entering: handle_line :: if (ctx.indata  && match_vec_u8(data , &\"#errors\"))");
            ctx.indata = false;
            ctx.inerrors = false;
            ctx.inexp = false;
        }
        else if (ctx.indata && match_vec_u8(data, &"#expected")) {
            debug!("Entering: handle_line :: if (ctx.indata  && match_vec_u8(data , &\"#expected\"))");
            ctx.indata = false;
            ctx.inexp = true;
            ctx.inerrors = false;
            ctx.inrule = false;
        }
        else if (ctx.inexp && match_vec_u8(data , &"#data")) {
            debug!("Entering: handle_line :: if (ctx.indata  && match_vec_u8(data , &\"#data\"))");
            ctx.indata = true;
            ctx.inerrors = false;
            ctx.inexp = false;
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
            ctx.inexp = match_vec_u8(data , &"#expected");
        }
    }
    else {
        debug!("Entering: handle_line :: else (2)");

        if ctx.indata {
            debug!("Entering: handle_line :: if (ctx.indata) (2)");
            ctx.buf = ctx.buf + data.clone();
            ctx.buf.push('\n' as u8);
        }
        if (ctx.inexp) {
            debug!("Entering: handle_line :: if (ctx.inexp)");
            ctx.exp.push(data.to_owned());
        }
    }

    return true;
}

fn testMain(fileName: ~str) {
    debug!(~"testMain : "+ fileName);
    let ctx = @mut line_ctx
    {
        buf:~[],
        exp : ~[],
        expused: 0,
        indata:false,
        inexp:false,
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

    for line in vec_lines {
        handle_line(line,ctx);
    }
    
    if ctx.buf.len() > 0 {
        run_test(ctx.buf.clone(),ctx.exp.clone());
    }
}

pub fn run_test(data:~[u8], exp:~[~[u8]]) {
    debug!("Entering :: run_test");
    // debug!("\n == data == %?" , str::from_bytes(data));
    let mut lwc_ref = lwc();
    let propstring = css_propstrings::css_propstrings(&mut lwc_ref);
    let mut css = create_css();
    let mut buf: ~str;
    let mut error = css.css_stylesheet_append_data(&mut lwc_ref , &propstring , data);
    match error {
        CSS_OK | CSS_NEEDDATA => {},
        _ => {assert!(false);}
    }

    error = css.css_stylesheet_data_done(&mut lwc_ref , &propstring);
    //debug!(fmt!("error from css_stylesheet_data_done: %?" , error));
    match error {
        CSS_OK => {},
        _ => {assert!(false);}
    }
    println(fmt!("BEFORE DUMP"));
    buf = dump_sheet(css.stylesheet_index, &mut lwc_ref);
    //debug!(fmt!("\n == sheet ==%?=" , buf));
    let mut dvec : ~[~[u8]] = ~[];
    for s in buf.any_line_iter() {
        dvec.push(s.as_bytes().to_owned());
   }
    let a = vec::concat(dvec) ;
    let b = vec::concat(exp) ;
    // debug!("============================================================" );
    // debug!(fmt!(" == sheet ==%?=" , vec));
    // debug!("============================================================" );
    // debug!(fmt!(" == exp ==%?=" , exp));
    // debug!("============================================================" );

    if a.len() != b.len() {
        debug!("============================================================" );
        debug!(" == sheet ==%?=" , (a));
        debug!("============================================================" );
        debug!(" == exp   ==%?=" , (b));
        debug!("============================================================" );
        fail!(~"Expected lines not equal to sheet dump lines");
    }

	let mut iter_both = a.iter().zip(b.iter());
    for (s, e) in iter_both {
        if s != e {
            debug!("============================================================" );
            debug!(" == sheet ==%?=" , (a));
            debug!("============================================================" );
            debug!(" == exp   ==%?=" , (b));
            debug!("============================================================" );
            fail!(~"character mismatch during result checking ");       
        }
    }
}


#[test]
fn au() {
    testMain(~"data/parse2/au.dat");
}

#[test]
fn bg() {
    testMain(~"data/parse2/bg.dat");
}

#[test]
fn bgpos() {
    testMain(~"data/parse2/bgpos.dat");
}

#[test]
fn border() {
    testMain(~"data/parse2/border.dat");
}

#[test]
fn comments() {
    testMain(~"data/parse2/comments.dat");
}

#[test]
fn eof() {
    testMain(~"data/parse2/eof.dat");
}

#[test]
fn font() {
    testMain(~"data/parse2/font.dat");
}

#[test]
fn illegal_values() {
    testMain(~"data/parse2/illegal-values.dat");
}

#[test]
fn list() {
    testMain(~"data/parse2/list.dat");
}

#[test]
fn malformed_declarations() {
    testMain(~"data/parse2/malformed-declarations.dat");
}

#[test]
fn margin() {
    testMain(~"data/parse2/margin.dat");
}

#[test]
fn multicol() {
    testMain(~"data/parse2/multicol.dat");
}

#[test]
fn outline() {
    testMain(~"data/parse2/outline.dat");
}

#[test]
fn padding() {
    testMain(~"data/parse2/padding.dat");
}

#[test]
fn selectors() {
    testMain(~"data/parse2/selectors.dat");
}

#[test]
fn tests1() {
    testMain(~"data/parse2/tests1.dat");
}
#[test]
fn unknown_properties() {
    testMain(~"data/parse2/unknown-properties.dat");
}
