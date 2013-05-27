//////////////////////////////////////////////////////////////////////
//
// Filename         : filter.rs
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

/*
FIXME: not needed but not implemented
filter_setopt()

change the name
filter => parserutils_filter

interface difference?
parserutils_error parserutils__filter_process_chunk(parserutils_filter *input,
		const uint8_t **data, size_t *len,
		uint8_t **output, size_t *outlen)

pub fn parserutils__filter_process_chunk(&mut self, inbuf : ~[u8] ) -> (~riconv::chunk_result, parserutils_error)

*/


extern mod std;
extern mod core;
extern mod parserutils;

//use parserutils::input::*;
use parserutils::input::parserutils_filter::*;
//use parserutils::charset::*;
use parserutils::charset::aliases::*;
use parserutils::utils::errors::*;


fn main() {
    io::println("filter");
}


#[test]
fn filter() {
    let mut alias = alias();
    let mut (filterOption, status) = 
        parserutils_filter(alias, ~"UTF-8"); // FIXME: rename
 
    match(status) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);} 
    }

    let mut input = filterOption.unwrap();

    // filter_setopt() is missing
    // Convert filter to UTF-8 encoding
    match(input.filter_set_encoding(~"UTF-8")) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    // Simple case: valid input & output buffer large enough
    // ryanc: normal case. buffer size does not matter in Rust
    let mut in: ~[u8] = "hell\xc2\xa0o!".to_bytes();
    let mut out: ~[u8] = ~[];

    match(input.parserutils__filter_process_chunk(in)) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("%?", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }
    assert!(str::from_bytes(out) == ~"hell\xc2\xa0o!");

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    // Too small an output buffer; noencoding edge cases
    // ryanc: not applicable to Rust. In this case, it's the same
    // as normal case

    // Illegal input sequence; output buffer large enough
    // ryanc: only checking for illegal input sequence
    let mut in: ~[u8] = "hell\x96o!".to_bytes(); // need to read in ascii

    // Input does loose decoding, converting to U+FFFD if illegal
    // input is encountered
    match(input.parserutils__filter_process_chunk(in)) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("%?", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }
    //assert!(str::from_bytes(out) == ~"hell\xef\xbf\xbdo!");
    //assert!(vec::eq(out, "hell\xef\xbf\xbdo!".to_bytes()));

    // Input ends mid-sequence


    
}
