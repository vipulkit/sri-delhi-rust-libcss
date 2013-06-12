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

pub fn resolve_url(_:~str, rel:arc::RWARC<~lwc_string>) -> (css_error,Option<arc::RWARC<~lwc_string>>) {
    return (CSS_OK,Some(rel.clone()));
}

pub fn check_newline(x: &u8) -> bool { *x == ('\n' as u8) }

fn match_vec_u8(expected_data: &[u8] , found_string: &str) -> bool {

    let mut found_string_vector = str::to_bytes(found_string);
    if found_string_vector.len() != expected_data.len() {
        // io::println("lenghts don't match");
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

fn create_css() -> @mut css{
    let mut lwc = wapcaplet::lwc();
    let css = css_create(css_create_params() , Some(lwc));
    css
}

pub fn handle_line(args: ~[u8],  ctx:@mut line_ctx)->bool {
    let mut data : ~[u8] = args ;
    // unsafe{io::println(fmt!("ctx.indata == %?, ctx.inexp == %?", ctx.indata, ctx.inexp));}
    if  (data.len() == 0) {
        // io::println("error");
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
    // io::println(~"testMain : "+ fileName);
    let ctx: @mut line_ctx = @mut line_ctx
    {
        mut buf:~[],
        mut exp : ~[],
        mut expused: 0,
        mut indata:false,
        mut inexp:false,
        mut inerrors: false,
        mut inrule: false
    };

    let file_content_result = io::read_whole_file(&Path(fileName)) ;
    let mut file_content : ~[u8] ;
    match file_content_result {
        Ok(x) => {
            file_content = x ;
        },
        Err(_) => {
            file_content = ~[] ;
            io::println(fmt!("\n Error opening file"));
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
    io::println(fmt!("entering run_test"));
    io::println(fmt!("data == %?" , data));
    io::println(fmt!("exp == %?" , exp));
    let mut css = create_css();
    let mut buf: ~str = ~"";
    let mut error = css.css_stylesheet_append_data(data);
    match error {
        CSS_OK | CSS_NEEDDATA => {},
        _ => {assert!(false);}
    }

    error = css.css_stylesheet_data_done();
    io::println(fmt!("error from css_stylesheet_data_done: %?" , error));
    match error {
        CSS_OK => {},
        _ => {assert!(false);}
    }

    buf = dump_sheet(css.stylesheet);
    let mut vec_buf = ~[];
    for str::each_line(buf) |s| {
        vec_buf.push(s.to_owned().to_bytes());
    }

    let exp_len = exp.len();
    let buf_len = buf.len();



    assert!(vec_buf == exp);

    // let mut bool_value = false;
    // let mut i = 0;
    // let mut index = 0;
    // loop {
    //     while i < exp.len() {
    //         if exp[index][i] != data[i] {
    //             bool_value = false;
    //         }
    //         else {
    //             bool_value = true;
    //         }
    //     }

    //     if (((2*exp_len)-buf.len())!=0) || (!bool_value) {
    //         assert!(false);
    //     }
    //     index += 1;
    //     if (index == exp.len()) {break;}
    // }
    

}


#[test]
fn parse2_au() {
    testMain(~"data/parse2/au.dat");
}

#[test]
fn parse2_bg() {
    testMain(~"data/parse2/bg.dat");
}

#[test]
fn parse2_bgpos() {
    testMain(~"data/parse2/bgpos.dat");
}

#[test]
fn parse2_border() {
    testMain(~"data/parse2/border.dat");
}

#[test]
fn parse2_comments() {
    testMain(~"data/parse2/comments.dat");
}

#[test]
fn parse2_eof() {
    testMain(~"data/parse2/eof.dat");
}

#[test]
fn parse2_font() {
    testMain(~"data/parse2/font.dat");
}

#[test]
fn parse2_illegal_values() {
    testMain(~"data/parse2/illegal-values.dat");
}

#[test]
fn parse2_list() {
    testMain(~"data/parse2/list.dat");
}

#[test]
fn parse2_malformed_declarations() {
    testMain(~"data/parse2/malformed-declarations.dat");
}

#[test]
fn parse2_margin() {
    testMain(~"data/parse2/margin.dat");
}

#[test]
fn parse2_multicol() {
    testMain(~"data/parse2/multicol.dat");
}

#[test]
fn parse2_outline() {
    testMain(~"data/parse2/outline.dat");
}

#[test]
fn parse2_padding() {
    testMain(~"data/parse2/padding.dat");
}

#[test]
fn parse2_selectors() {
    testMain(~"data/parse2/selectors.dat");
}

#[test]
fn parse2_tests1() {
    testMain(~"data/parse2/tests1.dat");
}
#[test]
fn parse2_unknown_properties() {
    testMain(~"data/parse2/unknown-properties.dat");
}