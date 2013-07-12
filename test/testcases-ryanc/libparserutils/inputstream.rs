//////////////////////////////////////////////////////////////////////
//
// Filename         : inputstream.rs
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////


extern mod std;
extern mod parserutils;
extern mod wapcaplet;
extern mod css;
//extern mod css;

use std::{io,vec};
use parserutils::input::*;
use parserutils::utils::errors::*;
use css::charset::csdetect::*;

fn main() {
    io::println("inputstream");
}

#[test]
fn utf8Test() {
    inputstream(~"data/input/UTF-8-test.txt");
}

fn inputstream(filename: ~str) {
    let (streamOption, PARSERUTILS_STATUS) = inputstream::inputstream(Some(~"UTF-8"), Some(CSS_CHARSET_DEFAULT as int), None);
    //let (streamOption, PARSERUTILS_STATUS) = inputstream::inputstream(~"UTF-8", None);
    match(PARSERUTILS_STATUS) {
        PARSERUTILS_OK=>{}
        _=>{assert!(false);}
    }

    let CHUNK_SIZE = 4096;
    //let mut buf: ~[u8] = ~[];
    let r: @Reader = io::file_reader(&Path(filename)).get();
    let mut stream = streamOption.unwrap();

    r.seek(0, io::SeekEnd);
    let mut len = r.tell();
    r.seek(0, io::SeekSet);


    while len >= CHUNK_SIZE {
        let buf = r.read_bytes(CHUNK_SIZE);

        match(stream.parserutils_inputstream_append(buf)) {
            PARSERUTILS_OK => {}
            _ => {assert!(false);}
        }

        len -= CHUNK_SIZE;

        loop {
            let (cOption, STATUS2) = stream.parserutils_inputstream_peek(0);
            match(STATUS2) {
                PARSERUTILS_NEEDDATA => {break;}
                PARSERUTILS_EOF      => {break;}
                PARSERUTILS_OK       => {
                    let (c, clen) = cOption.get();
                    stream.parserutils_inputstream_advance(clen);

                    if c[0].to_str() == ~"a" {
                        let insertdata : ~[u8] = ~['h' as u8,'e' as u8,'l' as u8,'l' as u8,'o' as u8,'!' as u8,'!' as u8,'!' as u8];
                        let STATUS3 = stream.parserutils_inputstream_insert(insertdata);
                        match(STATUS3) {
                            PARSERUTILS_OK => {}
                            _ => {assert!(false);}
                        }
                    }
                }
                _ => {assert!(false);}
            }
        }
    }

    if len > 0 {
        let mut buf: ~[u8] = vec::from_elem(len, 0);
        let read_size = r.read(buf, len);
        assert!(read_size == len);
        match(stream.parserutils_inputstream_append(buf)) {
            PARSERUTILS_OK => {}
            _ => {assert!(false);}
        }

        len = 0;
        assert!(len == 0); // to remove the warning;
    }

    let insertdata : ~[u8] = ~['h' as u8,'e' as u8,'l' as u8,'l' as u8,'o' as u8,'!' as u8,'!' as u8,'!' as u8];
    match(stream.parserutils_inputstream_insert(insertdata)) {
        PARSERUTILS_OK => {}
        _ => {assert!(false);}
    }

    let empty_buf : ~[u8] = ~[];
    match(stream.parserutils_inputstream_append(empty_buf)) {
        PARSERUTILS_OK => {}
        _ => {assert!(false);}
    }

    loop {
        let (cOption, STATUS) = stream.parserutils_inputstream_peek(0);
        match(STATUS) {
            PARSERUTILS_EOF => {break;}
            PARSERUTILS_OK  => {
                let (_c, clen) = cOption.get();
                stream.parserutils_inputstream_advance(clen);
            }
            _ => {assert!(false);}
        }
    }

    stream.parserutils_inputstream_destroy();
}
