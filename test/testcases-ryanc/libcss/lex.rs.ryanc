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

not implemented
lexer.css__lexer_destroy();

In rust we do not need
token.data.data
token.data.len


Internal compile error
*/

extern mod std;
extern mod parserutils;
extern mod css;

use core::io::*;

use parserutils::input::inputstream::*;
use css::utils::errors::*;

use css::charset::csdetect::*;
use css::lex::lexer::*;

fn main() {
    io::println("parse");
}

#[test]
fn tests1() {
    lex(~"../data/lex/tests1.dat");
}

#[test]
fn tests2() {
    lex(~"../data/lex/tests2.dat");
}

#[test]
fn regression() {
    lex(~"../data/lex/regression.dat");
}


fn lex(file: ~str) {
    let ITERATIONS = 1;

    for int::range(0, ITERATIONS) |_i| {
        let (streamOption, PARSERUTILS_STATUS) = inputstream(Some(~"UTF-8"), Some(CSS_CHARSET_DEFAULT as int), Some(~css__charset_extract));
        match(PARSERUTILS_STATUS) {
            PARSERUTILS_OK=>{}
            //_ => {assert!(false);}
        }

        let mut stream = streamOption.unwrap();
        let mut lexer = css_lexer::css__lexer_create(stream);
        // FIXME: need to check the status of lexer

        let CHUNK_SIZE = 4096;
        let mut buf: ~[u8] = ~[];
        let r: @Reader = io::file_reader(&Path(file)).get();

        r.seek(0, SeekEnd);
        let mut len = r.tell();
        r.seek(0, SeekSet);

        while len >= CHUNK_SIZE {
            let buf = r.read_bytes(CHUNK_SIZE);

            assert!(buf.len() == CHUNK_SIZE);
            // FIXME: Need to check the status
            lexer.css__lexer_append_data(buf);

            len -= CHUNK_SIZE;

            loop {
                let mut (status, tokOption) = lexer.css__lexer_get_token();
                match(status) {
                    CSS_OK => {
                        let tok = tokOption.unwrap();
                        //printToken(tok);
                        match(tok.token_type) { // FIXME: token_type => type
                            CSS_TOKEN_EOF => {break;}
                            _ => {}
                        }
                    }
                    _ => {break;}
                }
            }
        }

        if len > 0 {
            let read_size = r.read(buf, len);
            assert!(read_size == len);

            // FIXME: Need to check the status
            lexer.css__lexer_append_data(buf);

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
            let (status, tokOption) = lexer.css__lexer_get_token();
            match(status) {
                CSS_OK => {
                    let tok = tokOption.unwrap();
                    //printToken(tok);
                    match(tok.token_type) {
                        CSS_TOKEN_EOF => {break;}
                        _ => {}
                    }
                }
                _ => {break;}
            }
        }

        // FIXME: not implemented
        //lexer.css__lexer_destroy();
        //stream.parserutils_inputstream_destroy();
    }
}

// FIXME: change the name: css_token_type -> css_token
fn printToken(token: @mut css_token) {
    let mut toPrint;

    io::println(fmt!("[%?, %?] : ", token.line, token.col));

    match token.token_type {
        CSS_TOKEN_IDENT => {
            toPrint = fmt!("IDENT %?", token.data.data);
        },
        CSS_TOKEN_ATKEYWORD => {
            toPrint = fmt!("ATKEYWORD %?", token.data.data);
        },
        CSS_TOKEN_STRING => {
            toPrint = fmt!("STRING %?", token.data.data);
        },
        CSS_TOKEN_INVALID_STRING => {
            toPrint = fmt!("INVALID %?", token.data.data);
        },
        CSS_TOKEN_HASH => {
            toPrint = fmt!("HASH %?", token.data.data);
        },
        CSS_TOKEN_NUMBER => {
            toPrint = fmt!("NUMBER %?", token.data.data);
        },
        CSS_TOKEN_PERCENTAGE => {
            toPrint = fmt!("PERCENTAGE %?", token.data.data);
        },
        CSS_TOKEN_DIMENSION => {
            toPrint = fmt!("DIMENSION %?", token.data.data);
        },
        CSS_TOKEN_URI => {
            toPrint = fmt!("URI %?", token.data.data);
        },
        CSS_TOKEN_UNICODE_RANGE => {
            toPrint = fmt!("UNICODE_RANGE %?", token.data.data);
        },
        CSS_TOKEN_CDO => {
            toPrint = ~"CDO";
        },
        CSS_TOKEN_CDC => {
            toPrint = ~"CDC";
        },
        CSS_TOKEN_S => {
            toPrint = ~"S";
        },
        CSS_TOKEN_COMMENT => {
            toPrint = fmt!("COMMENT %?", token.data.data);
        },
        CSS_TOKEN_FUNCTION => {
            toPrint = fmt!("FUNCTION %?", token.data.data);
        },
        CSS_TOKEN_INCLUDES => {
           toPrint = fmt!("INCLUDES %?", token.data.data);
        },
        CSS_TOKEN_DASHMATCH => {
            toPrint = ~"DASHMATCH";
        },
        CSS_TOKEN_PREFIXMATCH => {
            toPrint = ~"PREFIXMATCH";
        },
        CSS_TOKEN_SUFFIXMATCH => {
            toPrint = ~"SUFFIXMATCH";
        },
        CSS_TOKEN_SUBSTRINGMATCH => {
            toPrint = ~"SUBSTRINGMATCH";
        },
        CSS_TOKEN_CHAR => {
            toPrint = fmt!("CHAR %?", token.data.data);
        },
        CSS_TOKEN_EOF => {
            toPrint = ~"EOF ";
        }
        // FIXME: unreachable pattern
        //CSS_TOKEN_LAST_INTERN_LOWER => {}
        CSS_TOKEN_LAST_INTERN => {}
        // _ => {fail!(~"Invalid type")}
    }
    io::println(toPrint);

}

