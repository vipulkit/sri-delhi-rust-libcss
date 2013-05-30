//////////////////////////////////////////////////////////////////////
//
// Filename         : number.rs
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

/*
data type of css_fixed?

*/


extern mod std;
extern mod css;
extern mod wapcaplet;
extern mod testutils;

use css::parse::properties::common::*;
use wapcaplet::*;
use testutils::*;

fn main() {
    io::println("number");
}


#[test]
fn numberTest() {
    number(~"data/number/number.dat");
}

fn number(filename: ~str) {
    let len = css__parse_filesize(filename);
    if len == 0 {
        return;
    }

    let ctx = @mut line_ctx_number {
        buf: ~[],
        exp: ~[],
        indata: false,
        inexp: false
    };

    assert!(css__parse_testfile(filename, ~handle_line, Number(ctx)));
    /* and run final test */
    // ryanc: the last testcase
    run_test(copy ctx.buf, copy ctx.exp);
}

fn handle_line(data: ~str, pw: line_ctx) -> bool {
    let ctx: @mut line_ctx_number;
    match pw {
        Number(x) => {ctx = x;},
        _ => {fail!(~"Type mismatch");}
    }

    if data.char_at(0) == '#' {
        if ctx.inexp {
            /* This marks end of testcase, so run it */
            run_test(copy ctx.buf, copy ctx.exp);
            ctx.buf = ~[];
            ctx.exp = ~[];
        }

        let line = data.slice(1,data.len()).to_owned().to_lower();
        if ctx.indata && str::eq(&line, &~"expected") {
            ctx.indata = false;
            ctx.inexp = true;
        }
        else if !ctx.indata {
            ctx.indata = str::eq(&line, &~"data");
            ctx.inexp = str::eq(&line, &~"expected");
        }
        else {
            ctx.buf += data.to_bytes();
        }
    }
    else {
        if ctx.indata {
            ctx.buf += data.to_bytes();
        }
        if ctx.inexp {
            ctx.exp += data.to_bytes();
        }
    }

    return true;
}

fn run_test(data: ~[u8], exp: ~[u8]) {
    let mut lwc = wapcaplet::lwc();

    do lwc.write|l| {
        let lwc_str = l.lwc_intern_string(str::from_bytes(data));
        let (num, _) = css__number_from_lwc_string(lwc_str, false);

        let result = print_css_fixed(num);
        assert_eq!(result.slice(0, exp.len()).to_owned(), str::from_bytes(exp));
    }
}

fn print_css_fixed(f: i32) -> ~str {
    let mut uintpart = fixToInt(abs(f));
    let mut fracpart = ((abs(f) & 0x3ff) * 1000 + 500) / (1 << 10);

    let mut buf = ~"";
    let mut tmp: ~[char] = ~[];
    let mut flen: uint = 0;

    if f < 0 {
        buf.push_char('-');
    }

    let nums = ~"0123456789";
    loop {
        tmp.push(nums.char_at((uintpart % 10) as uint));
        uintpart /= 10;
        if !(tmp.len() < 20 && uintpart != 0) {
            break;
        }
    }

    while !tmp.is_empty() {
        buf.push_char(tmp.pop());
    }

    buf.push_char('.');

    loop {
        tmp.push(nums.char_at((fracpart % 10) as uint));
        fracpart /= 10;
        if !(tmp.len() < 20 && fracpart != 0) {
            break;
        }
    }

    while !tmp.is_empty() {
        buf.push_char(tmp.pop());
        flen += 1;
    }

    while flen < 3 {
        buf.push_char('0');
        flen += 1;
    }

    return buf;
}

#[inline(always)]
fn abs(x: i32) -> u32 {
    if x < 0 {
        return -x as u32;
    }
    else {
        return x as u32;
    }
}

#[inline(always)]
fn fixToInt(x: u32) -> u32 {
    return (x >> 10) as u32;
}
