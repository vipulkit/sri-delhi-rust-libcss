extern mod std;
extern mod css;
extern mod wapcaplet;
extern mod dump;

use std::arc;
use css::css::*;
use css::css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use wapcaplet::*;
use dump::*;

pub struct line_ctx {
    buf:~[u8],

    exp:~[~[u8]],
    expused: uint,

    indata:bool,
    inexp:bool,
    inerrors:bool,
    inrule: bool
}

pub fn resolve_url(_:@str, rel:arc::RWARC<~lwc_string>) -> (css_error,Option<arc::RWARC<~lwc_string>>) {
    return (CSS_OK,Some(rel.clone()));
}

pub fn check_newline(x: &u8) -> bool { *x == ('\n' as u8) }

fn match_vec_u8(expected_data: &[u8] , found_string: &str) -> bool {

    let mut found_string_vector = str::to_bytes(found_string);
    if found_string_vector.len() != expected_data.len() {
        // debug!("lenghts don't match");
        return false;
    }

    for vec::each2(expected_data , found_string_vector) |&e , &f| {
        if e != f {
            return false;
        }
    } 
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
        font : None
    };
    return css_param;
}

fn main() {
    debug!("parse");
}

fn create_css() -> @mut css{
    debug!("Entering: create_css");
    let mut lwc = wapcaplet::lwc();
    let css = css_create( &(css_create_params()) , Some(lwc));
    css
}

pub fn handle_line(args: ~[u8],  ctx:@mut line_ctx)->bool {
    debug!("Entering: handle_line");
    let mut data : ~[u8] = args ;
    // unsafe{debug!(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    if  (data.len() == 0) {
        // debug!("error");
        return true;
    }

    if (data[0] == '#' as u8) {
        if (ctx.inexp) {
            /* This marks end of testcase, so run it */

            run_test(copy ctx.buf , copy ctx.exp);
            ctx.exp= ~[];
            ctx.buf=~[];
        }
        if (ctx.indata  && match_vec_u8(data , &"#errors")) {
            ctx.indata = false;
            ctx.inerrors = false;
            ctx.inexp = false;
        }
        else if (ctx.indata && match_vec_u8(data, &"#expected")) {
            ctx.indata = false;
            ctx.inexp = true;
            ctx.inerrors = false;
            ctx.inrule = false;
        }
        else if (ctx.inexp && match_vec_u8(data , &"#data")) {
            ctx.indata = true;
            ctx.inerrors = false;
            ctx.inexp = false;
        }
        else if (ctx.indata) {
            ctx.buf += copy data;
            ctx.buf.push('\n' as u8);
        } 
        else {
            ctx.indata = match_vec_u8(data , &"#data");
            ctx.inerrors = match_vec_u8(data , &"#errors");
            ctx.inexp = match_vec_u8(data , &"#expected");
        }
    }
    else {
        if ctx.indata {
            ctx.buf += copy data;
            ctx.buf.push('\n' as u8);
        }
        if (ctx.inexp) {
            ctx.exp.push(data);
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
    let mut vec_lines = vec::split(file_content, check_newline) ;

    for vec_lines.each |&each_line| {
        handle_line(each_line,ctx);
    }
    
    if unsafe {copy ctx.buf.len()} > 0 {
        run_test(copy ctx.buf,copy ctx.exp);
    }
}

pub fn run_test(data:~[u8], exp:~[~[u8]]) {
    debug!("Entering :: run_test");
    // debug!("\n == data == %?" , str::from_bytes(data));
    
    let mut css = create_css();
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
    //debug!(fmt!("\n == sheet ==%?=" , buf));
    let mut dvec = ~[];
    for str::each_line(buf) |s| {
        dvec.push(s.to_owned().to_bytes());
    }
    let mut a = vec::concat(dvec) ;
    let mut b = vec::concat(exp) ;
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

    for vec::each2(a,b) |&s,&e| {
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
