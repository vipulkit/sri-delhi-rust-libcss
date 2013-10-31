extern mod std;
extern mod extra;
extern mod css;
extern mod wapcaplet;
extern mod dump;

use std::io::*;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use dump::*;
use libwapcaplet::wapcaplet::*;
use css::parse::propstrings::*;

pub fn resolve_url(_:&str, rel:uint) -> (css_error,Option<uint>) {
    return (CSS_OK,Some(rel.clone()));
}

fn css_create_params() -> css_params {
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

fn create_css(stylesheet_vector:&mut ~[css_stylesheet]) -> ~css{
    
    let css = css::css_create(stylesheet_vector, &css_create_params());
    css
}

fn main() {
    // css(~"data/css/blocks.css");
    debug!("css21");   
}

fn css(file_name: ~str) {
    let mut stylesheet_vector:~[css_stylesheet]=~[];
    let mut css_rule_data_list:~[~css_rule_data_type]=~[];
    let mut lwc_ref = lwc();
    let propstring = css_propstrings::css_propstrings(&mut lwc_ref);
    let mut css = create_css(&mut stylesheet_vector);
    let CHUNK_SIZE = 4096;
    let mut buf: ~[u8];
    let r:@Reader = file_reader(&Path(file_name)).unwrap(); 
    r.seek(0 , SeekEnd);
    let mut len = r.tell();
    let origlen = len; 
    r.seek(0 , SeekSet);
    while len>CHUNK_SIZE {
        buf = r.read_bytes(CHUNK_SIZE as uint);
        len -= CHUNK_SIZE;
        let error = css.css_stylesheet_append_data(&mut stylesheet_vector, &mut css_rule_data_list, &mut lwc_ref , &propstring , buf);
        match error {
            CSS_OK | CSS_NEEDDATA => {},
            _ => {assert!(false);}
        }
    }
    buf = r.read_bytes(len as uint);
    let error = css.css_stylesheet_append_data(&mut stylesheet_vector, &mut css_rule_data_list, &mut lwc_ref ,&propstring , buf);
    match error {
        CSS_OK | CSS_NEEDDATA => {},
        _ => {assert!(false);}
    }

    let mut error = css.css_stylesheet_data_done(&mut stylesheet_vector, &mut css_rule_data_list, &mut lwc_ref , &propstring);


    match error {
        CSS_OK | CSS_IMPORTS_PENDING => {},
        _ => {assert!(false);}
    }

    loop {
        match error {
            CSS_IMPORTS_PENDING => {
                let (error1 , option_url , _) = css.css_stylesheet_next_pending_import(&mut stylesheet_vector, &mut css_rule_data_list);
                match error1 {
                    CSS_OK => {
                                                                      
                        let mut params: css_params = css_create_params();
                        params.url = option_url.unwrap();
                        let mut css_import = create_css(&mut stylesheet_vector);
                        let err = css_import.css_stylesheet_data_done(&mut stylesheet_vector, &mut css_rule_data_list, &mut lwc_ref , &propstring);
                        match err {
                            CSS_OK => {},
                            _ => {assert!(false);}
                        }
                        let err_register = css.css_stylesheet_register_import(&mut stylesheet_vector, &mut css_rule_data_list, Some(css_import.stylesheet));
                        match err_register {
                            CSS_OK => {},
                            _ => {assert!(false);}
                        }
                        error = CSS_IMPORTS_PENDING;
                    } 
                    CSS_INVALID =>{break;},
                    _ => {assert!(false);}
                }
            },
            _ =>{
                break;
            }
        }
    }

    let outsize = if 16384 > (origlen*8) {
        16384
    }
    else {
        origlen*8
    };

    let mut buf: ~str;

    buf = dump_sheet(&mut stylesheet_vector, css.stylesheet,&mut css_rule_data_list ,&mut lwc_ref );
    let outlen = buf.len();
    let written = outsize - outlen;
    // debug!(fmt!("written == %? , outsize - outlen == %?" , written , outsize-outlen));
    assert!(written == outsize-outlen);
    
} 


#[test]
fn allzengarden() {
    css(~"data/css/allzengarden.css");
}

#[test]
fn badcomment() {
    css(~"data/css/badcomment.css");
}

#[test]
fn blocks() {
    css(~"data/css/blocks.css");
}

#[test]
fn color() {
    css(~"data/css/color.css");
}

#[test]
fn fontface() {
    css(~"data/css/fontface.css");
}

#[test]
fn malformed() {
    css(~"data/css/malformed.css");
}

#[test]
fn simple() {
    css(~"data/css/simple.css");
}
