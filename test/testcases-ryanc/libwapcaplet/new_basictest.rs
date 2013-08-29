

extern mod std;
extern mod extra;
extern mod wapcaplet;

use extra::time::*;
use std::io;
use wapcaplet::*;

fn main()
{
	println("wapcaplet");
}


#[test]
fn lwc_test() {
    let a = wapcaplet::lwc();

    let mut i =0 ;
    let j = 10000 ;

    let start_time = precise_time_ns();
    while(i<j) {
        let s1= a.lwc_intern_string(~"ABC");
        let s2= a.lwc_intern_string(~"abc");
        let s3= a.lwc_intern_string(~"aBc");
        let s4= a.lwc_intern_caseless_string(s3);
        assert!(!a.lwc_string_isequal(s1,s2));
        assert!(a.lwc_string_caseless_isequal(s1,s3));
        assert!(a.lwc_string_caseless_isequal(s4,s3));

        assert!(a.lwc_string_length(s4) == 3);

        assert!(a.lwc_string_data(s4) == ~"abc");

        let intern_one = a.lwc_intern_string(~"onedfjdjjjjjjjjjjjjjjjjjjjjfffffffffffffkkkkkkkkkkkkkkkkkkkkkkkssssssssssssss");
        let intern_two = a.lwc_intern_string(~"onedfjdjjjjjjjddddddddjjjjjjjjjjjjjfffffffffffffkkkkkkkkkkkkkkkkkkkkkkkssssssssssssss");

        assert!(!a.lwc_string_isequal(intern_one,intern_two));
        assert!(a.lwc_string_caseless_isequal(intern_one,intern_one));
        i += 1;
    }

    let end_time = precise_time_ns();
    io::println(fmt!("Time for lwc in seprate thread %?", (end_time as float - start_time as float)/1000f));


    a.lwc_thread_terminate();

}
