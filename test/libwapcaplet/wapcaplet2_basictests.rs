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
l.lwc_string_isequal(new_hre.clone(), sub_hre.clone())

e.g.,
s1.lwc_string_hash_value()

than
wapcaplet::lwc_string_hash_value(intern_one.clone());
*/

extern mod std;
extern mod wapcaplet2;

use std::arc;
use wapcaplet2::*;

fn main() {
    io::println(fmt!("libwapcaplet"));
}


#[test]
fn ops_with_a_context() {
    let mut lwc = lwc_thread_safe();
    //let empty = lwc_string();

    do lwc.write |l| {
        let s1 = l.lwc_intern_string(&"A");
        let empty = l.lwc_intern_string(&"");
        assert!(!lwc_string_isequal(empty.clone(), s1.clone()));
    }
}

#[test]
fn lwc_intern_string_twice_ok() {
    let mut lwc = lwc_thread_safe();
    //let t1 = lwc_string();
    //let t2 = lwc_string();

    do lwc.write |l| {
        let t1 = l.lwc_intern_string(&"");
        let t2 = l.lwc_intern_string(&"");
        let s1= l.lwc_intern_string(&"A");
        let s2= l.lwc_intern_string(&"B");
        assert!(!lwc_string_isequal(s1.clone(), t1.clone()));
        assert!(!lwc_string_isequal(s2.clone(), t2.clone()));
    }
}

#[test]
fn lwc_intern_string_twice_same_ok() {
    let mut lwc = lwc_thread_safe();
    //let t1 = lwc_string();
    //let t2 = lwc_string();

    do lwc.write |l| {
        let t1 = l.lwc_intern_string(&"");
        let t2 = l.lwc_intern_string(&"");
        let s1= l.lwc_intern_string(&"A");
        let s2= l.lwc_intern_string(&"A");
        assert!(!lwc_string_isequal(s1.clone(), t1.clone()));
        assert!(!lwc_string_isequal(s2.clone(), t2.clone()));
    }
}

#[test]
fn ops_with_a_filled_context() {
    let mut lwc = lwc_thread_safe();

    do lwc.write |l| {
        // with_filled_context_setup
        let intern_one = l.lwc_intern_string(&"one");
        let intern_two = l.lwc_intern_string(&"two");
        let intern_three = l.lwc_intern_string(&"three");
        let intern_YAY = l.lwc_intern_string(&"YAY");

        assert!(!lwc_string_isequal(intern_one.clone(), intern_two.clone()));
        assert!(!lwc_string_isequal(intern_one.clone(), intern_three.clone()));
        assert!(!lwc_string_isequal(intern_two.clone(), intern_three.clone()));

        // lwc_interning_works
        let new_one = l.lwc_intern_string(&"one");
        assert!(lwc_string_isequal(new_one.clone(), intern_one.clone()));

        // lwc_intern_substring
        let new_hre = l.lwc_intern_string(&"hre");
        // let sub_hre_option = l.lwc_intern_substring(~"hre".slice(1,3));
        // match sub_hre_option {
        //     Some(sub_hre) => {
        //         assert!(lwc_string_isequal(new_hre.clone(), sub_hre.clone()));
        //     }
        //     None => {}
        // } 

        // lwc_string_ref_ok
        // FIXME: needed but not implemented!!!
        //assert!(l.lwc_string_isequal(l.lwc_string_ref(intern_one.clone()), intern_one.clone()));

        // lwc_string_ref_unref_ok
        // FIXME: needed but not implemented!!!
        //l.lwc_string_ref(intern_one.clone());
        //l.lwc_string_unref(intern_one.clone());


        // lwc_string_unref_ok
        // FIXME: needed but not implememted!!!
        // l.lwc_string_unref(intern_one.clone());

        // lwc_string_isequal_ok
        assert!(!lwc_string_isequal(intern_one.clone(), intern_two.clone()));

        // lwc_string_caseless_isequal_ok1
        let new_ONE = l.lwc_intern_string(&"ONE");
        assert!(!lwc_string_isequal(intern_one.clone(), new_ONE.clone()));
        assert!(l.lwc_string_caseless_isequal(intern_one.clone(), new_ONE.clone()));

        // lwc_string_caseless_isequal_ok2
        let new_yay = l.lwc_intern_string(&"yay");
        assert!(!lwc_string_isequal(intern_YAY.clone(), new_yay.clone()));
        assert!(l.lwc_string_caseless_isequal(intern_YAY.clone(), new_yay.clone()));

        // lwc_string_caseless_isequal_bad
        assert!(!l.lwc_string_caseless_isequal(intern_YAY.clone(), intern_one.clone()));

        // lwc_extract_data_ok
        // NOTE: not implemented

        // lwc_string_hash_value_ok
        // FIXME: this is a function.
        //--wapcaplet::lwc_string_hash_value(intern_one.clone());

        // lwc_string_is_nul_terminated
        // NOTE: not applicable

        // lwc_substring_is_nul_terminated
        // NOTE: not applicable

        // lwc_intern_substring_bad_size
        //--let _err_str = l.lwc_intern_substring(intern_three.clone(), 1, 100);

        // lwc_intern_substring_bad_offset
        //--let _err_str = l.lwc_intern_substring(intern_three.clone(), 100, 1);


        // lwc_string_iteration
        // NOT Implemented
        // lwc_iterate_strings();

        // with_filled_context_teardown
        // FIXME: needed but not implemented!!!
        // l.lwc_string_unref(intern_one.clone());
        // l.lwc_string_unref(intern_two.clone());
        // l.lwc_string_unref(intern_three.clone());
        // l.lwc_string_unref(intern_YAY.clone());
    }
}

