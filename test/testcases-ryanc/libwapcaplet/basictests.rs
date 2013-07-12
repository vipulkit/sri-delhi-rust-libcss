//////////////////////////////////////////////////////////////////////
//
// Filename         : basictests.c
// Author           : Ryan Choi
// Created on       : Monday, 6 May 2013
// Last Modified on : Monday, 6 May 2013
// Version          : 1.00
// Title            :
//
//////////////////////////////////////////////////////////////////////

/*
FIXME
NOT implemented:
lwc_string_ref()
lwc_string_unref()

Design
NOT entirely OO.
e.g.,

should be more like
l.lwc_string_isequal(s1)

than
l.lwc_string_isequal(new_hre, sub_hre)

e.g.,
s1.lwc_string_hash_value()

than
wapcaplet::lwc_string_hash_value(intern_one);
*/

extern mod std;
extern mod wapcaplet;

use wapcaplet::*;

fn main() {
    println(fmt!("libwapcaplet"));
}

fn lwc_string() -> @mut lwc_string {
    let s = @mut lwc_string {
        id:0,
        string: ~"",
        insensitive: None
    };
    return s;
}


#[test]
fn ops_with_a_context() {
    let lwc = wapcaplet::lwc();
    let empty = lwc_string();

    let s1 = lwc.lwc_intern_string("A");
    assert!(!lwc.lwc_string_isequal(empty, s1));
}

#[test]
fn lwc_intern_string_twice_ok() {
    let lwc = wapcaplet::lwc();
    let t1 = lwc_string();
    let t2 = lwc_string();

    let s1= lwc.lwc_intern_string("A");
    let s2= lwc.lwc_intern_string("B");
    assert!(!lwc.lwc_string_isequal(s1,t1));
    assert!(!lwc.lwc_string_isequal(s2,t2));
}

#[test]
fn lwc_intern_string_twice_same_ok() {
    let lwc = wapcaplet::lwc();
    let t1 = lwc_string();
    let t2 = lwc_string();

    let s1= lwc.lwc_intern_string("A");
    let s2= lwc.lwc_intern_string("A");
    assert!(!lwc.lwc_string_isequal(s1, t1));
    assert!(!lwc.lwc_string_isequal(s2, t2));
}

#[test]
fn ops_with_a_filled_context() {
    let lwc = wapcaplet::lwc();

        // with_filled_context_setup
        let intern_one = lwc.lwc_intern_string("one");
        let intern_two = lwc.lwc_intern_string("two");
        let intern_three = lwc.lwc_intern_string("three");
        let intern_YAY = lwc.lwc_intern_string("YAY");

        assert!(!lwc.lwc_string_isequal(intern_one, intern_two));
        assert!(!lwc.lwc_string_isequal(intern_one, intern_three));
        assert!(!lwc.lwc_string_isequal(intern_two, intern_three));

        // lwc_interning_works
        let new_one = lwc.lwc_intern_string("one");
        assert!(lwc.lwc_string_isequal(new_one, intern_one));

        // lwc_intern_substring
        let new_hre = lwc.lwc_intern_string("hre");
        let sub_hre_option = lwc.lwc_intern_substring(new_hre, 1, 3);
        match sub_hre_option {
            Some(sub_hre) => {
                assert!(lwc.lwc_string_isequal(new_hre, sub_hre));
            }
            None => {}
        } 

        // lwc_string_ref_ok
        // FIXME: needed but not implemented!!!
        //assert!(lwc.lwc_string_isequal(lwc.lwc_string_ref(intern_one), intern_one));

        // lwc_string_ref_unref_ok
        // FIXME: needed but not implemented!!!
        //lwc.lwc_string_ref(intern_one);
        //lwc.lwc_string_unref(intern_one);


        // lwc_string_unref_ok
        // FIXME: needed but not implememted!!!
        // lwc.lwc_string_unref(intern_one);

        // lwc_string_isequal_ok
        assert!(!lwc.lwc_string_isequal(intern_one, intern_two));

        // lwc_string_caseless_isequal_ok1
        let new_ONE = lwc.lwc_intern_string("ONE");
        assert!(!lwc.lwc_string_isequal(intern_one, new_ONE));
        assert!(lwc.lwc_string_caseless_isequal(intern_one, new_ONE));

        // lwc_string_caseless_isequal_ok2
        let new_yay = lwc.lwc_intern_string("yay");
        assert!(!lwc.lwc_string_isequal(intern_YAY, new_yay));
        assert!(lwc.lwc_string_caseless_isequal(intern_YAY, new_yay));

        // lwc_string_caseless_isequal_bad
        assert!(!lwc.lwc_string_caseless_isequal(intern_YAY, intern_one));

        // lwc_extract_data_ok
        // NOTE: not implemented

        // lwc_string_hash_value_ok
        // FIXME: this is a function.
        //wapcaplet::lwc_string_hash_value(intern_one);

        // lwc_string_is_nul_terminated
        // NOTE: not applicable

        // lwc_substring_is_nul_terminated
        // NOTE: not applicable

        // lwc_intern_substring_bad_size
        let _err_str = lwc.lwc_intern_substring(intern_three, 1, 100);

        // lwc_intern_substring_bad_offset
        let _err_str = lwc.lwc_intern_substring(intern_three, 100, 1);


        // lwc_string_iteration
        // NOT Implemented
        // lwc_iterate_strings();

        // with_filled_context_teardown
        // FIXME: needed but not implemented!!!
        // l.lwc_string_unref(intern_one);
        // l.lwc_string_unref(intern_two);
        // l.lwc_string_unref(intern_three);
        // l.lwc_string_unref(intern_YAY);
}

