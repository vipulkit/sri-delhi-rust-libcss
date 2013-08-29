extern mod std;
extern mod wapcaplet;

use wapcaplet::*;

fn main()
{
	println("wapcaplet");
}


#[test]
fn ops_with_a_filled_context() {
    let mut a = wapcaplet::lwc();

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
    let intern_three = a.lwc_intern_string(~"onedfjdjjjjjjjjjjjjjjjjjjjjfffffffffffffkkkkkkkkkkkkkkkkkkkkkkkssssssssssssss");
    let intern_YAY = a.lwc_intern_string(~"onedfjdjjjjjjjddddddddjjjjjjjjjjjjjfffffffffffffkkkkkkkkkkkkkkkkkkkkkkksssssssssssssszzz");

    assert!(!a.lwc_string_isequal(intern_one,intern_two));
    assert!(a.lwc_string_caseless_isequal(intern_one,intern_one));

}
