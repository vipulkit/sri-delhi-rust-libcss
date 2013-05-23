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


/*

FIXME
charset/alias.rs should be charset/aliases.rs

parserutils_error code is defined in charset/alias.rs
it should be defined in utils/error.rs


*/

extern mod std;
extern mod core;
extern mod parserutils;


use core::io::*;
use parserutils::input::*;
use parserutils::charset::aliases::*; // FIXME: to be removed
use parserutils::charset::csdetect::*; // FIXME: to be removed


fn main() {
    io::println("inputstream");
}

#[test]
fn inputstream() {
    let file=~"../data/input/UTF-8-test.txt";

    let (streamOption, PARSERUTILS_STATUS) = inputstream::inputstream(Some(~"UTF-8"), Some(CSS_CHARSET_DEFAULT), None);
    //let (streamOption, PARSERUTILS_STATUS) = inputstream::inputstream(~"UTF-8", None);
    match(PARSERUTILS_STATUS) {
        PARSERUTILS_OK=>{}
        _=>{assert!(false);}
    }

    let CHUNK_SIZE = 4096;
    let mut buf: ~[u8] = ~[];
    let r: @Reader = io::file_reader(&Path(file)).get();
    let mut stream = streamOption.unwrap();

    r.seek(0, SeekEnd);
    let mut len = r.tell();
    r.seek(0, SeekSet);


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
                    let mut (c, clen) = cOption.get();
                    stream.parserutils_inputstream_advance(clen);

                    if c[0].to_str() == ~"a" {
                        let STATUS3 = stream.parserutils_inputstream_insert("hello!!!".to_bytes());
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
        let read_size = r.read(buf, len);
        assert!(read_size == len);
        match(stream.parserutils_inputstream_append(buf)) {
            PARSERUTILS_OK => {}
            _ => {assert!(false);}
        }

        len = 0;
        assert!(len == 0); // to remove the warning;
    }

    match(stream.parserutils_inputstream_insert("hello!!!".to_bytes())) {
        PARSERUTILS_OK => {}
        _ => {assert!(false);}
    }

    let empty_buf : ~[u8] = ~[];
    match(stream.parserutils_inputstream_insert(empty_buf)) {
        PARSERUTILS_OK => {}
        _ => {assert!(false);}
    }

    loop {
        let (cOption, STATUS) = stream.parserutils_inputstream_peek(0);
        match(STATUS) {
            PARSERUTILS_EOF => {break;}
            PARSERUTILS_OK  => {
                let mut (_c, clen) = cOption.get();
                stream.parserutils_inputstream_advance(clen);
            }
            _ => {false;}
        }
    }

    stream.parserutils_inputstream_destroy();
}
