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
FIXME: needed but not implemented?
filter_setopt()

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
    /* Convert filter to UTF-8 encoding */
    match(input.filter_set_encoding(~"UTF-8")) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    /* Simple case: valid input & output buffer large enough
     * ryanc: normal case. buffer size does not matter in Rust */
    let mut in: ~[u8] = "hell\xc2\xa0o!".to_bytes();
    let mut out: ~[u8] = ~[];

    match(input.parserutils__filter_process_chunk(in)) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    assert_eq!(str::from_bytes(out), ~"hell\xc2\xa0o!");

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    /* Too small an output buffer; noencoding edge cases
    // ryanc: not applicable to Rust. In this case, it's the same
    // as normal case */

    /* Illegal input sequence; output buffer large enough
    // ryanc: only checking for illegal input sequence */
    let mut in: ~[u8] = "hell\x96o!".to_bytes();

    /* Input does loose decoding, converting to U+FFFD if illegal
       input is encountered */
    match(input.parserutils__filter_process_chunk(in)) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    // FIXME: fails
    assert_eq!(str::from_bytes(out), ~"hell\xef\xbf\xbdo!");

    /* Input ends mid-sequence */
    let mut in: ~[u8] = "hell\xc2\xa0o!".to_bytes();
    let mut inlen = in.len()-3;
    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    inlen+=3;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    assert_eq!(str::from_bytes(out), ~"hell\xc2\xa0o!");

    /* Input ends mid-sequence, but second attempt has too small a
     * buffer, but large enough to write out the incomplete character.
     * ryanc: Skipping as this is not applicable to Rust */


    /* Input ends mid-sequence, but second attempt has too small a
     * buffer, not large enough to write out the incomplete character.
     * ryanc: Skipping as this is not applicable to Rust */

    /* Input ends mid-sequence, but second attempt contains
     * invalid character */

    let mut in: ~[u8] = "hell\xc2\xc2o!".to_bytes();
    let mut inlen = in.len()-3;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    inlen+=3;

    /* Input does loose decoding, converting to U+FFFD if illegal
     * input is encountered */

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    // FIXME: failed
    assert_eq!(str::from_bytes(out), ~"hell\xef\xbf\xbd\xef\xbf\xbdo!");

    /* Input ends mid-sequence, but second attempt contains another
     * incomplete character */
    let mut in: ~[u8] = "hell\xc2\xa0\xc2\xa1o!".to_bytes();
    let mut inlen = in.len()-5;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    inlen+=2;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    inlen+=3;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    assert_eq!(str::from_bytes(out), ~"hell\xc2\xa0\xc2\xa1o!");


    /* Input ends mid-sequence, but second attempt contains insufficient
     * data to complete the incomplete character */

    let mut in: ~[u8] = "hell\xe2\x80\xa2o!".to_bytes();
    let mut inlen = in.len()-4;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    inlen+=1;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    inlen+=3;

    match(input.parserutils__filter_process_chunk(in.slice(0, inlen).to_owned())) {
        (chunk, PARSERUTILS_OK) => {
            out = copy chunk.outbuf;
            io::println(fmt!("'%s'", str::from_bytes(out)));
        },
        (_, _) => {assert!(false);}
    }

    match(input.parserutils__filter_reset()) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    assert_eq!(str::from_bytes(out), ~"hell\xe2\x80\xa2o!");

    /* Clean up */
    input.parserutils__filter_destroy();

}
