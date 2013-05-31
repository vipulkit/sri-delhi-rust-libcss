//////////////////////////////////////////////////////////////////////
//
// Filename         : css21.rs
// Author           : Ryan Choi
// Created on       : Thursday, 30 May 2013
// Last Modified on : Thursday, 30 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

extern mod std;
extern mod css;
extern mod wapcaplet;

use core::io::*;
use std::arc;
use css::css::*;
use css::css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use wapcaplet::*;

fn main() {
    io::println("css21");
}

#[test]
fn allzengarden() {
    css21(~"data/css/allzengarden.css");
}

#[test]
fn badcomment() {
    css21(~"data/css/adcomment.css");
}

#[test]
fn blocks() {
    css21(~"data/css/blocks.css");
}

#[test]
fn color() {
    css21(~"data/css/color.css");
}

#[test]
fn fontface() {
    css21(~"data/css/fontface.css");
}

#[test]
fn malformed() {
    css21(~"data/css/malformed.css");
}

#[test]
fn simple() {
    css21(~"data/css/simple.css");
}

fn resolve_url(_:~str, rel:arc::RWARC<~lwc_string>) ->
    (css_error,Option<arc::RWARC<~lwc_string>>) {
    return (CSS_OK,Some(rel.clone()));
}

fn css21(filename: ~str) {
    let mut ITERATIONS = 1;

    // FIXME: rename it to css_stylesheet_params
    let mut params = css_params {
        params_version: CSS_PARAMS_VERSION_1,
        level: CSS_LEVEL_21,
        charset: Some(~"UTF-8"),
        url: copy filename,
        //title: None, // FIXME: need to support None?
        title: ~"",
        allow_quirks: false,
        inline_style: false,
        resolve: @resolve_url,
        import: None,
        color: None,
        font: None
    };

    let CHUNK_SIZE = 4096;

    for uint::range(0, ITERATIONS) |_i| {
        let mut sheet = css_create(copy params); // FIXME: rename it to css_stylesheet_create()

        let r: @Reader = io::file_reader(&Path(filename)).get();

        r.seek(0 , SeekEnd);
        let mut len = r.tell();
        let mut _origlen = len;
        r.seek(0 , SeekSet);

        while len>CHUNK_SIZE {
            let buf = r.read_bytes(CHUNK_SIZE);
            assert!(buf.len() == CHUNK_SIZE);

            let error = sheet.css_stylesheet_append_data(buf);
            match error {
                CSS_OK | CSS_NEEDDATA => {},
                _ => {assert!(false);}
            }
            len -= CHUNK_SIZE;
        }

        if len > 0 {
            let buf = r.read_bytes(len);
            let error = sheet.css_stylesheet_append_data(buf);
            match error {
                CSS_OK | CSS_NEEDDATA => {},
                _ => {assert!(false);}
            }
        }

        let (error , _) = sheet.css_stylesheet_data_done();
        let mut importPending=false;
        match error {
            CSS_OK => {},
            CSS_IMPORTS_PENDING => {
                importPending=true;
            },
            _ => {assert!(false);}
        }

        while importPending {
            // continue;
        }

        // sheet.css_stylesheet_destroy()

    }
}
