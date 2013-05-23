//////////////////////////////////////////////////////////////////////
//
// Filename         : lex.rs
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

/*
FIXME
inconsistent names: lex, lexer, css_lexer.

get_token() => css__lexer_get_token()

not implemented
lexer.css__lexer_destroy();

lexer_create() takes ~stream. cannot use stream afterward

*/

extern mod std;
extern mod core;
extern mod parserutils;
extern mod css;

use core::io::*;
use parserutils::input::*;
use parserutils::charset::csdetect::*;
use css::lex::*;

fn main() {
    io::println("parse");
}

#[test]
fn lex() {
    let ITERATIONS = 1;
    let file=~"../data/lex/test1.dat";

    for int::range(0, ITERATIONS) |i| {
        let (streamOption, PARSERUTILS_STATUS) = inputstream::inputstream(Some(~"UTF-8"), Some(CSS_CHARSET_DEFAULT), None);
        match(PARSERUTILS_STATUS) {
            PARSERUTILS_OK=>{}
            //_ => {assert!(false);} // FIXME: unreachable pattern
        }

        let mut stream = streamOption.unwrap();

        // FIXME: lexer_create() takes ~stream. cannot use stream afterward
        let mut lexer = lexer::css_lexer::css__lexer_create(stream);
        // FIXME: need to check the status of lexer

        let CHUNK_SIZE = 4096;
        let mut buf: ~[u8] = ~[];
        let r: @Reader = io::file_reader(&Path(file)).get();

        r.seek(0, SeekEnd);
        let mut len = r.tell();
        r.seek(0, SeekSet);

        while len >= CHUNK_SIZE {
            let buf = r.read_bytes(CHUNK_SIZE);

             //match(stream.parserutils_inputstream_append(buf)) {
             //    PARSERUTILS_OK => {}
                 //_ => {assert!(false);}
             //}

            len -= CHUNK_SIZE;

            loop {
                let mut (tokOption, STATUS) = lexer.get_token();
                match(STATUS) {
                    CSS_OK => {
                        let tok = tokOption.unwrap();
                        io::println(fmt!("%?", tok));
                        match(tok) {
                            CSS_TOKEN_EOF => {break;}
                            // _ => {}  FIXME: check retrn value
                        }
                    }
                    //_ => {break;} FIXME: Check return value
                }
            }
        }

        if len > 0 {
            let read_size = r.read(buf, len);
            assert!(read_size == len);
            // let STATUS = stream.parserutils_inputstream_append(buf);
            // match(STATUS) {
                // // PARSERUTILS_OK => {}
                // //_ => {assert!(false);}
            // }

            len = 0;
            assert!(len == 0); // to remove the warning;
        }

        /*
        FIXME: use of stream
        let empty_buf : ~[u8] = ~[];
        match(stream.parserutils_inputstream_append(empty_buf)) {
            PARSERUTILS_OK => {}
            //_ => {assert!(false);}
        }
        */

        loop {
            let (tokOption, STATUS) = lexer.get_token();
            match(STATUS) {
                CSS_OK => {
                    let tok = tokOption.unwrap();
                    io::println(fmt!("%?", tok));
                    match(tok) {
                        CSS_TOKEN_EOF => {break;}
                        // _ => {} FIXME: check return value
                    }
                }
                // _ => {break;} FIXME: check return value
            }
        }

        //lexer.css__lexer_destroy(); FIXME: not implemented
        //stream.parserutils_inputstream_destroy();
    }
}

