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

what is riconv? riconv::chunck_result? why needed??

interface difference?
parserutils_error parserutils__filter_process_chunk(parserutils_filter *input,
		const uint8_t **data, size_t *len,
		uint8_t **output, size_t *outlen)

pub fn parserutils__filter_process_chunk(&mut self, inbuf : ~[u8] ) -> (~riconv::chunk_result, parserutils_error)

*/


extern mod std;
extern mod core;
extern mod parserutils;

// use core::io::*;
use std::arc;
use parserutils::input::*;
use parserutils::charset::*;


fn main() {
    io::println("filter");
}


#[test]
fn filter() {
    let mut parser = aliases::alias();

    let (filterOption, STATUS) = parserutils_filter::parserutils_filter(parser, ~"UTF-8"); // FIXME: rename
    match(STATUS) {
        PARSERUTILS_OK => {}
        //_ => {assert!(false);} // FIXME: why is it unreachable?
    }

    let input = filterOption.unwrap();

    // filter_setopt() is missing

    // Simple case: valid input & output buffer large enough
    let mut in: ~[u8] = "hell\xc2\xa0o!".to_bytes();
    let mut out: ~[u8] = ~[];

    //let (, STATUS) = input.parserutils__filter_process_chunk(in);


}
