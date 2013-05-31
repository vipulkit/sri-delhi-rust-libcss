//////////////////////////////////////////////////////////////////////
//
// Filename         : lex-auto.rs
// Author           : Ryan Choi
// Created on       : Friday, 31 May 2013
// Last Modified on : Friday, 31 May 2013
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
extern mod testutils;

use parserutils::input::inputstream::*;
use parserutils::utils::errors::*;

use css::charset::csdetect::*;
use css::lex::lexer::*;

use testutils::*;

fn main() {
    io::println("lex");
}

#[test]
fn tests1() {
    lexAuto(~"../data/lex/tests1.dat");
}

#[test]
fn tests2() {
    lexAuto(~"../data/lex/tests2.dat");
}

#[test]
fn regression() {
    lexAuto(~"../data/lex/regression.dat");
}


fn lexAuto(filename: ~str) {
    let len = css__parse_filesize(filename);
    if len == 0 {
        return;
    }

    let ctx = @mut line_ctx_lex {
        buf: ~[],
        exp: ~[],
        indata: false,
        inexp: false
    };

    assert!(css__parse_testfile(filename, ~handle_line, Lex(ctx)));

    /* and run final test */
    // ryanc: the last testcase
    run_test(copy ctx.buf, copy ctx.exp);
}

fn handle_line(data: ~str, pw: line_ctx) -> bool {
    let ctx: @mut line_ctx_lex;
    match pw {
        Lex(x) => {ctx = x},
        _ => {fail!(~"Type mismatch")}
    };

    if data.char_at(0) == '#' {
        if ctx.inexp {
            run_test(copy ctx.buf, copy ctx.exp);
            ctx.buf = ~[];
            ctx.exp = ~[];
        }

        let line = data.slice(1, data.len()).to_owned().to_lower();
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
            css__parse_expected(ctx, data);
        }
    }

    return true;
}

fn css__parse_expected(ctx: @mut line_ctx_lex, data: ~str) {
    let mut token: ~str;
    let mut text = ~"";
    let mut hasText = false;

    match str::find_char(data, ':') {
        Some(x) => {
            token = data.slice(0, x).to_owned();
            text = data.slice(x+1, data.len()).to_owned();
            hasText=true;
        }
        None => {
            token = data;
        }
    }

    let entry = ~exp_entry {
        token_type: string_to_type(token),
        text: text,
        hasText: hasText
    };

    ctx.exp.push(entry);
}

fn run_test(data: ~[u8], exp: ~[~exp_entry]) {
    let (inputStreamOption, status) =
        inputstream(Some(~"UTF-8"), Some(CSS_CHARSET_DEFAULT as int),
                    Some(~css__charset_extract));

    match(status) {
        PARSERUTILS_OK => {},
        _ => {assert!(false);}
    }

    let mut inputStream = inputStreamOption.unwrap();
    let mut lexer = css_lexer::css__lexer_create(inputStream);

    lexer.lexer_append_data(data);

    let mut i=0;
    loop {
        let mut (tokOption, status) = lexer.get_token();
        match status {
            LEXER_OK => {
                let tok = tokOption.unwrap();

                if !str::eq(&string_from_type(copy tok), &string_from_type(copy exp[i].token_type)) {
                    io::println(fmt!("Got token %?, Expected %?",
                                    tok, exp[i].token_type));
                }

                if exp[i].hasText {
                    if !str::eq(&text_from_type(copy tok), &exp[i].text) {
                        io::println(fmt!("Got data %?, Expected %?",
                                        tok, exp[i].text));
                    }
                }
                i+=1;
            }
            // FIXME: how do I check eof?
            // CSS_TOKEN_EOF => {
                // break;
            // }
            // FIXME: "unreachable pattern" error msg appears
            _ => {
                break;
            }
        }
    }
    lexer.data_done();
}

// FIXME: comented are either
// 1. not implemented
// 2. has different data type.
fn string_to_type(data: ~str) -> css_token_type {
    match data {
        ~"IDENT"         => {return CSS_TOKEN_IDENT(data)},
        ~"ATKEYWORD"     => {return CSS_TOKEN_ATKEYWORD(data)},
        ~"STRING"        => {return CSS_TOKEN_STRING(data)},
        ~"INVALID"       => {return CSS_TOKEN_INVALID_STRING},
        ~"HASH"          => {return CSS_TOKEN_HASH(data)},
        //~"NUMBER"=> {return CSS_TOKEN_NUMBER(data)},
        //~"PERCENTAGE"    => {return CSS_TOKEN_PERCENTAGE(data)},
        //~"DIMENSION"     => {return CSS_TOKEN_DIMENSION(data)},
        ~"URI"           => {return CSS_TOKEN_URI(data)},
        //~"UNICODE-RANGE" => {return CSS_TOKEN_UNICODE_RANGE(data)},
        ~"CDO"           => {return CSS_TOKEN_CDO},
        ~"CDC"           => {return CSS_TOKEN_CDC},
        ~"S"             => {return CSS_TOKEN_S},
        //~"COMMENT"=> {return CSS_TOKEN_COMMENT(data)},
        ~"FUNCTION"      => {return CSS_TOKEN_FUNCTION(data)},
        //~"INCLUDES"      => {return CSS_TOKEN_INCLUDES(data)},
        //~"DASHMATCH"=> {return CSS_TOKEN_DASHMATCH(data)},
        //~"PREFIXMATCH"=> {return CSS_TOKEN_PREFIXMATCH(data)},
        //~"SUFFIXMATCH"=> {return CSS_TOKEN_SUFFIXMATCH(data)},
        //~"SUBSTRINGMATCH"=> {return CSS_TOKEN_SUBSTRINGMATCH(data)},
        //~"CHAR"=> {return CSS_TOKEN_CHAR(data)},
        ~"EOF"           => {return CSS_TOKEN_EOF},
        _                => {fail!(~"Type mismatch");}
    }
}


fn string_from_type(token_type: css_token_type) -> ~str {
    match token_type {
        CSS_TOKEN_IDENT(_x)       => {return ~"IDENT";},
        CSS_TOKEN_ATKEYWORD(_x)   => {return ~"ATKEYWORD";},
        CSS_TOKEN_HASH(_x)        => {return ~"HASH";},
        CSS_TOKEN_FUNCTION(_x)    => {return ~"FUNCTION";},
        CSS_TOKEN_STRING(_x)      => {return ~"STRING";},
        CSS_TOKEN_INVALID_STRING  => {return ~"INVALID_STRING";},
        CSS_TOKEN_URI(_x)         => {return ~"URI";},
        //CSS_TOKEN_UNICODE_RANGE(ch1 , ch2)=>{return ~"UNICODE_RANGE";},
        CSS_TOKEN_CHAR(_x)        => {return ~"CHAR";},
        //CSS_TOKEN_NUMBER(x)=>{return ~"NUMBER";},
        //CSS_TOKEN_PERCENTAGE(x)=>{return ~"PERCENTAGE";},
        //CSS_TOKEN_DIMENSION(x)   => {return ~"DIMENSION";},
        CSS_TOKEN_CDO             => {return ~"CDO";},
        CSS_TOKEN_CDC             => {return ~"CDC";},
        CSS_TOKEN_S               => {return ~"S";},
        // CSS_TOKEN_COMMENT=>{return ~"COMMENT";},
        // CSS_TOKEN_INCLUDES=>{return ~"INCLUDES";},
        //CSS_TOKEN_DASHMATCH=>{return ~"DASHMATCH";},
        //CSS_TOKEN_PREFIXMATCH=>{return ~"PREFIXMATCH";},
        // CSS_TOKEN_SUFFIXMATCH=>{return ~"SUFFIXMATCH";},
        //CSS_TOKEN_SUBSTRINGMATCH=>{return ~"SUBSTRINGMATCH";},
        CSS_TOKEN_EOF             => {return~"EOF";},
        _                         => {fail!(~"Type mismatch")}
    }
}

fn text_from_type(token_type: css_token_type) -> ~str {
    match token_type {
        CSS_TOKEN_IDENT(x)       => {return x},
        CSS_TOKEN_ATKEYWORD(x)   => {return x;},
        CSS_TOKEN_HASH(x)        => {return x;},
        CSS_TOKEN_FUNCTION(x)    => {return x;},
        CSS_TOKEN_STRING(x)      => {return x;},
        CSS_TOKEN_INVALID_STRING  => {return ~""},
        CSS_TOKEN_URI(x)         => {return x;},
        //CSS_TOKEN_UNICODE_RANGE(ch1 , ch2)=>{return ~"UNICODE_RANGE";},
        CSS_TOKEN_CHAR(x)        => {return str::from_char(x);},
        //CSS_TOKEN_NUMBER(x)=>{return ~"NUMBER";},
        //CSS_TOKEN_PERCENTAGE(x)=>{return ~"PERCENTAGE";},
        //CSS_TOKEN_DIMENSION(x)   => {return ~"DIMENSION";},
        CSS_TOKEN_CDO             => {return ~"";},
        CSS_TOKEN_CDC             => {return ~"";},
        CSS_TOKEN_S               => {return ~"";},
        // CSS_TOKEN_COMMENT=>{return ~"COMMENT";},
        // CSS_TOKEN_INCLUDES=>{return ~"INCLUDES";},
        //CSS_TOKEN_DASHMATCH=>{return ~"DASHMATCH";},
        //CSS_TOKEN_PREFIXMATCH=>{return ~"PREFIXMATCH";},
        // CSS_TOKEN_SUFFIXMATCH=>{return ~"SUFFIXMATCH";},
        //CSS_TOKEN_SUBSTRINGMATCH=>{return ~"SUBSTRINGMATCH";},
        CSS_TOKEN_EOF             => {return ~"";},
        _                         => {fail!(~"Type mismatch")}
    }
}
