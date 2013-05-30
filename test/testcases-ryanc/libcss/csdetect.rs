//////////////////////////////////////////////////////////////////////
//
// Filename         : csdetect.rs
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
extern mod parserutils;
extern mod testutils;

use parserutils::charset::aliases::*;
use parserutils::utils::errors::*;
use css::charset::csdetect::*;
use testutils::*;
use std::arc;

fn main() {
    io::println("csdetect");
}

#[test]
fn bom_charset() {
    csdetect(~"data/csdetect/bom-charset.dat");
}

// #[test]
// fn bom() {
    // csdetect(~"data/csdetect/bom.dat");
// }

fn csdetect(filename: ~str) {
    let len = css__parse_filesize(filename);
    if len == 0 {
        return;
    }

    let ctx = @mut line_ctx_csdetect {
        buflen: len,
        bufused: 0,
        buf: ~[],
        enc: ~"",
        indata: false,
        inenc: false
    };

    assert!(css__parse_testfile(filename, ~handle_line, Csdetect(ctx)));

    /* and run final test */
    // ryanc: the last testcase
    run_test(copy ctx.buf, copy ctx.enc);
}

fn handle_line(data: ~str, pw: line_ctx) -> bool {
    let ctx: @mut line_ctx_csdetect;
    match pw {
        Csdetect(x) => {ctx = x},
        _ => {fail!(~"Type mismatch")}
    };

    if data.char_at(0) == '#' {
        if ctx.inenc {
            run_test(copy ctx.buf, copy ctx.enc);
            ctx.buf = ~[];
            ctx.enc = ~"";
            ctx.bufused=0;
        }

        let line = data.slice(1,data.len()).to_owned().to_lower();
        ctx.indata = str::eq(&line, &~"data");
        ctx.inenc = str::eq(&line, &~"encoding");
    }
    else {
        if ctx.indata {
            ctx.buf += data.to_bytes();
            ctx.bufused+=data.len();
        }
        if ctx.inenc {
            ctx.enc = data;
        }
    }

    return true;
}

fn run_test(data: ~[u8], expected: ~str) {
    let mut mibenum: u16 = 0;
    let source = CSS_CHARSET_DEFAULT;

    let alias = alias();
    let (charsetOption, srcOption, status) =
        css__charset_extract(&data, mibenum, source as int, alias.clone());
    match status {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }
    mibenum = charsetOption.unwrap();

    assert!(mibenum != 0);

    let detected_charset =
        arc::get(&alias).parserutils_charset_mibenum_to_name(mibenum).unwrap();
    let expected_mibenum =
        arc::get(&alias).parserutils_charset_mibenum_from_name(copy expected);

    io::println(fmt!("Detected charset %s (%?) Source %d Expected %s (%?)",
                     detected_charset, mibenum,
                     srcOption.unwrap(), expected, expected_mibenum));

    assert!(mibenum == expected_mibenum);
}
