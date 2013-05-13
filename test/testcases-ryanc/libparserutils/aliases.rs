//////////////////////////////////////////////////////////////////////
//
// Filename         : aliases.c
// Author           : Ryan Choi
// Created on       : Monday, 13 May 2013
// Last Modified on : Monday, 13 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

extern mod std;
extern mod parserutils;

use std::arc;
use parserutils::charset::*;


fn main() {
    io::println("aliases");
}

#[test]
fn canonicalise_moose() {
    let mut parser = alias::alias();

    let mut canon = arc::get(&parser).parserutils__charset_alias_canonicalise(~"moose");
    match(canon) {
        Some(_) => {
            assert!(false);
        }
        None => {
            assert!(true);
        }
    }
}

#[test]
fn canonicalise_csinvariant() {
    let mut parser = alias::alias();

    let mut canon = arc::get(&parser).parserutils__charset_alias_canonicalise(~"csinvariant");
    match(canon) {
        Some(c) => {
            io::println(fmt!("%s %?\n", c.name, c.mib_enum));
            assert!(true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn canonicalise_csinvariant_quote() {
    let mut parser = alias::alias();

    let mut canon = arc::get(&parser).parserutils__charset_alias_canonicalise(~"csinvariant\"");
    match(canon) {
        Some(c) => {
            io::println(fmt!("%s %?\n", c.name, c.mib_enum));
            assert!(true);
        }
        _ => {
            assert!(false);
        }
    }
}

#[test]
fn canonicalise_nats_sefi_add() {
    let mut parser = alias::alias();

    let mut canon = arc::get(&parser).parserutils__charset_alias_canonicalise(~"nats-sefi-add");
    match(canon) {
        Some(c) => {
            io::println(fmt!("%s %?\n", c.name, c.mib_enum));
            assert!(true);
            assert!(c.mib_enum == arc::get(&parser).parserutils_charset_mibenum_from_name(copy c.name));

            let name = arc::get(&parser).parserutils_charset_mibenum_to_name(c.mib_enum);
            match(name) {
                Some(n) => {
                    assert!(n == c.name);
                }
                _ => {assert!(false);}
            }
        }
        _ => {
            assert!(false);
        }
    }
}


#[test]
fn canonicalise_utf8() {
    let mut parser = alias::alias();

    let mut canon = arc::get(&parser).parserutils__charset_alias_canonicalise(~"u.t.f.8");
    match(canon) {
        Some(c) => {
            io::println(fmt!("%s %?\n", c.name, c.mib_enum));
            assert!(true);
        }
        _ => {
            assert!(false);
        }
    }
}

