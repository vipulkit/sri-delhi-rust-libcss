//////////////////////////////////////////////////////////////////////
//
// Filename         : testutils.rs
// Author           : Ryan Choi
// Created on       : Monday, 28 May 2013
// Last Modified on : Monday, 28 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////
#[link(name="testutils", vers="0.1", author="ryanc")];
#[crate_type="lib"];

extern mod core;
extern mod css;

use core::io::*;
use css::lex::lexer::*;


pub type line_func = ~extern fn(data: ~str, pw: line_ctx) -> bool;

pub enum line_ctx {
    Csdetect(@mut line_ctx_csdetect),
    Number(@mut line_ctx_number),
    Lex(@mut line_ctx_lex)
}

pub struct line_ctx_csdetect {
    buflen: uint,  // FIXME: not needed
    bufused: uint, // FIXME: not needed
    buf: ~[u8],
    enc: ~str,
    indata: bool,
    inenc: bool
}

pub struct line_ctx_number{
    buf: ~[u8],
    exp: ~[u8],
    indata: bool,
    inexp: bool
}

pub struct line_ctx_lex {
    buf: ~[u8],
    exp: ~[~exp_entry],
    indata: bool,
    inexp: bool
}

pub struct exp_entry {
    token_type: css_token_type,
    text: ~str,
    hasText: bool
}


pub fn css__parse_filesize(filename: &str) -> uint {
    let r: @Reader = io::file_reader(&Path(filename)).get();
    r.seek(0, SeekEnd);
    let len = r.tell();
    return len;
}

pub fn css__parse_testfile(filename: &str, callback: line_func, pw: line_ctx) -> bool {
    let reader = result::get(&io::file_reader(&Path(filename)));
    while !reader.eof() {
        let line = reader.read_line();

        if !line.is_empty() {
            if !(*callback)(line, pw) {
                return false;
            }
        }
    }

    return true;
}
