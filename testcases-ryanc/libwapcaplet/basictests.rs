extern mod std;
extern mod wapcaplet;

use std::arc;
use wapcaplet::*;

fn main() {
    io::println(fmt!("libwapcaplet"));
}

fn init() -> lwc {
    let mut l = lwc{bucketVector: ~[]};
    for uint::range(0,4091) |_i| {
        let bucket: ~[arc::RWARC<~lwc_string>] = ~[];
        l.bucketVector.push(bucket);
    }

    return l;
}

fn lwc_string() -> arc::RWARC<~lwc_string> {
    let s = arc::RWARC(~lwc_string {
        string: ~"",
        length: 0,
        refcnt: 0,
        hash: 0,
        is_case_insensitive: false,
        case_insensitive: None
    });
    return s;
}


#[test]
fn ops_with_a_context() {
    let mut l = init();
    let empty = lwc_string();
    let str= l.lwc_intern_string(~"A");

    assert!(!lwc::lwc_string_isequal(empty.clone(), str.clone()));
}

#[test]
fn lwc_intern_string_twice_ok() {
    let mut l = init();
    let t1 = lwc_string();
    let t2 = lwc_string();
    let str1= l.lwc_intern_string(~"A");
    let str2= l.lwc_intern_string(~"B");

    assert!(!lwc::lwc_string_isequal(str1, t1));
    assert!(!lwc::lwc_string_isequal(str2, t2));
}

#[test]
fn lwc_intern_string_twice_same_ok() {
    let mut l = init();
    let t1 = lwc_string();
    let t2 = lwc_string();
    let s1= l.lwc_intern_string(~"A");
    let s2= l.lwc_intern_string(~"A");

    assert!(!lwc::lwc_string_isequal(s1, t1));
    assert!(!lwc::lwc_string_isequal(s2, t2));
}

#[test]
fn ops_with_a_filled_context() {
    let mut l = init();

    // with_filled_context_setup
    let intern_one = l.lwc_intern_string(~"one");
    let intern_two = l.lwc_intern_string(~"two");
    let intern_three = l.lwc_intern_string(~"three");
    let intern_YAY = l.lwc_intern_string(~"YAY");

    assert!(!lwc::lwc_string_isequal(intern_one.clone(), intern_two.clone()));
    assert!(!lwc::lwc_string_isequal(intern_one.clone(), intern_three.clone()));
    assert!(!lwc::lwc_string_isequal(intern_two.clone(), intern_three.clone()));


    // lwc_interning_works
    let new_one = l.lwc_intern_string(~"one");
    assert!(lwc::lwc_string_isequal(new_one.clone(), intern_one.clone()));


    // lwc_intern_substring
    let new_hre = l.lwc_intern_string(~"hre");
    let sub_hre = l.lwc_intern_substring(new_hre.clone(), 1, 3);
    assert!(lwc::lwc_string_isequal(new_hre.clone(), sub_hre.clone()));


    // lwc_string_ref_ok
    assert!(lwc::lwc_string_isequal(l.lwc_string_ref(intern_one.clone()), intern_one.clone()));

    // lwc_string_ref_unref_ok
    l.lwc_string_ref(intern_one.clone());
    l.lwc_string_unref(intern_one.clone());


    // lwc_string_unref_ok
    l.lwc_string_unref(intern_one.clone());

    // lwc_string_isequal_ok
    assert!(!lwc::lwc_string_isequal(intern_one.clone(), intern_two.clone()));

    // lwc_string_caseless_isequal_ok1
    let new_ONE = l.lwc_intern_string(~"ONE");
    assert!(!lwc::lwc_string_isequal(intern_one.clone(), new_ONE.clone()));
    assert!(l.lwc_string_caseless_isequal(intern_one.clone(), new_ONE.clone()));

    // lwc_string_caseless_isequal_ok2
    let new_yay = l.lwc_intern_string(~"yay");
    assert!(!lwc::lwc_string_isequal(intern_YAY.clone(), new_yay.clone()));
    assert!(l.lwc_string_caseless_isequal(intern_YAY.clone(), new_yay.clone()));

    // lwc_string_caseless_isequal_bad
    assert!(l.lwc_string_caseless_isequal(intern_YAY.clone(), intern_one.clone()));


    // lwc_extract_data_ok
    // NOTE: not implemented

    // lwc_string_hash_value_ok
    lwc::lwc_string_hash_value(intern_one.clone());

    // lwc_string_is_nul_terminated
    // NOTE: not applicable

    // lwc_substring_is_nul_terminated
    // NOTE: not applicable

    // lwc_intern_substring_bad_size
    let err_str = l.lwc_intern_substring(intern_three.clone(), 1, 100);

    // lwc_intern_substring_bad_offset
    let err_str = l.lwc_intern_substring(intern_three.clone(), 100, 1);


    // lwc_string_iteration
    // NOT Implemented
    // lwc_iterate_strings();







    // with_filled_context_teardown
    l.lwc_string_unref(intern_one.clone());
    l.lwc_string_unref(intern_two.clone());
    l.lwc_string_unref(intern_three.clone());
    l.lwc_string_unref(intern_YAY.clone());
}

