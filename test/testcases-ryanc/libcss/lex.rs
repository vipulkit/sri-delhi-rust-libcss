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
extern mod parserutils;
extern mod css;

use core::io::*;

use parserutils::input::inputstream::*;
use parserutils::charset::aliases::*;
use parserutils::utils::errors::*;

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
            _ => {assert!(false);}
        }

        let mut stream = streamOption.unwrap();

        // FIXME: lexer_create() takes ~stream. cannot use stream afterward
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

            //match(stream.parserutils_inputstream_append(buf)) {
            //    PARSERUTILS_OK => {}
            //    _ => {assert!(false);}
            //}
            // FIXME: Is it how to append data?
            // FIXME: Need to check the status
            lexer.lexer_append_data(buf);

            len -= CHUNK_SIZE;

            loop {
                let mut (tokOption, STATUS) = lexer.get_token();
                match(STATUS) {
                    CSS_OK => {
                        let tok = tokOption.unwrap();
                        io::println(fmt!("%?", tok));
                        match(tok) {
                            CSS_TOKEN_EOF => {break;}
                            _ => {}
                        }
                    }
                    // FIXME: "unreachable pattern" error msg appears
                    //_ => {break;}
                }
            }
        }

        if len > 0 {
            let read_size = r.read(buf, len);
            assert!(read_size == len);

            //let STATUS = stream.parserutils_inputstream_append(buf);
            //match(STATUS) {
            //    PARSERUTILS_OK => {}
            //    _ => {assert!(false);}
            //}
            // FIXME: Need to check the status
            lexer.lexer_append_data(buf);

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
                        _ => {}
                    }
                }
                //_ => {break;}
            }
        }

        // FIXME: not implemented
        //lexer.css__lexer_destroy();
        //stream.parserutils_inputstream_destroy();
    }
}

// FIXME: change the name: css_token_type -> css_token
fn printToken(token: css_token_type) {
    let mut toPrint;

    // FIXME: token does not have line and col
    //io::println(fmt!("[%d, %d] : ", token.line, token.col));

    match token {
        CSS_TOKEN_IDENT(x) => {
            toPrint = ~"IDENT" + x;
        },
        CSS_TOKEN_ATKEYWORD(x) => {
            toPrint = ~"ATKEYWORD " + x;
        },
        CSS_TOKEN_STRING(x) => {
            toPrint = ~"STRING " + x;
        },
        // FIXME: invalid number of arg error msg
        // CSS_TOKEN_INVALID_STRING(x) => {
            // toPrint = ~"INVALID " + x;
        // },
        CSS_TOKEN_HASH(x) => {
            toPrint = ~"HASH " + x;
        },
        // FIXME: invalid number of arg error msg
        // CSS_TOKEN_NUMBER(x) => {
            // toPrint = ~"NUMBER " + x;
        // },
        // FIXME: invalid number of arg error msg
        // CSS_TOKEN_PERCENTAGE(x) => {
            // toPrint = ~"PERCENTAGE " + x;
        // },
        // FIXME: invalid number of arg error msg
        // CSS_TOKEN_DIMENSION(x) => {
            // toPrint = ~"DIMENSION "+ x;
        // },
        CSS_TOKEN_URI(x) => {
            toPrint = ~"URI " + x;
        },
        // FIXME: invalid number of arg error msg
        // CSS_TOKEN_UNICODE_RANGE(x) => {
            // toPrint = ~"UNICODE_RANGE " + x;
        // },
        CSS_TOKEN_CDO => {
            toPrint = ~"CDO";
        },
        CSS_TOKEN_CDC => {
            toPrint = ~"CDC";
        },
        CSS_TOKEN_S => {
            toPrint = ~"S";
        },
        // FIXME: not defined
        // CSS_TOKEN_COMMENT(x) => {
            // toPrint = ~"COMMENT" + x;
        // },
        CSS_TOKEN_FUNCTION(x) => {
            toPrint = ~"FUNCTION " + x;
        },
        CSS_TOKEN_INCLUDES => {
            toPrint = ~"INCLUDES";
        },
        // FIXME: unreachable pattern
        // CSS_TOKEN_DASHMATCH => {
            // toPrint = ~"DASHMATCH";
        // },
        // CSS_TOKEN_PREFIXMATCH => {
            // toPrint = ~"PREFIXMATCH";
        // },
        // CSS_TOKEN_SUFFIXMATCH => {
            // toPrint = ~"SUFFIXMATCH";
        // },
        // CSS_TOKEN_SUBSTRINGMATCH => {
            // toPrint = ~"SUBSTRINGMATCH";
        // },
        // FIXME: type mismatch
        // CSS_TOKEN_CHAR(x) => {
            // toPrint = ~"CHAR " + x
        // },

        // FIXME: unreachable pattern
        // CSS_TOKEN_EOF => {
            // toPrint = ~"EOF ";
        // }
        // CSS_TOKEN_LAST_INTERN_LOWER => {;}
        // CSS_TOKEN_LAST_INTERN => {;}
    }
    io::println(toPrint);

}

