//////////////////////////////////////////////////////////////////////
//
// Filename         : parse.rs
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

/*
FIXME
* Do not understand: different function prototype for
   css__parser_create("UTF-8", CSS_CHARSET_DICTATED, myrealloc, NULL, &parser)
   pub fn css_parser_create(language: ~css_language, lexer: ~css_lexer, lwc: arc::RWARC<~lwc>);

*/


extern mod std;
extern mod css;

use core::io::*;
// use std::arc;
// use css::parse::*;

fn main() {
    io::println("parse");
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


fn parse(filename: ~str) {
    let ITERATIONS = 1;

    for int::range(0, ITERATIONS) |_i| {

        // HOWTO: using css__parser_create()

        let CHUNK_SIZE = 4096;
        let mut buf: ~[u8] = ~[];
        let r: @Reader = io::file_reader(&Path(filename)).get();
        r.seek(0, SeekEnd);
        let mut len = r.tell();
        r.seek(0, SeekSet);

        while len >= CHUNK_SIZE {
            let buf = r.read_bytes(CHUNK_SIZE);

            assert!(buf.len() == CHUNK_SIZE);


            //css__parser_parse_chunk();
        }

        if len > 0 {
            let read_size = r.read(buf, len);
            assert!(read_size == len);

            len = 0;
            assert!(len == 0); // to remove the warning
        }
    }
}
