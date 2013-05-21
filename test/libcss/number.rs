extern mod std;
extern mod css;
extern mod wapcaplet;

use css::parse::properties::common::*;
use wapcaplet::*;
use std::arc;

fn main() {
    io::println("number");
    number(~"data/number/number.dat");
}

fn number(file_name: ~str) {
    let r:@Reader = io::file_reader(&Path(file_name)).get();
    let mut lwc = wapcaplet::lwc();
    let mut dataFlag = false;
    let mut expectedFlag = false;
    let mut resetFlag = false;
    let mut data_string: ~str = ~"";
    let mut expected_str: ~str = ~"";

    while !r.eof() {
        let buf = r.read_line();
        // io::println(buf);
        if buf == ~"#data" {
            dataFlag = true;
            expectedFlag = false; 
            resetFlag = false;
        }
        else if buf == ~"#errors" {
            dataFlag = false;
            expectedFlag = false;
            resetFlag = false;
        }
        else if buf == ~"#expected" {
            expectedFlag = true;
            dataFlag = false;
            resetFlag = false;

        }
        else if buf == ~"#reset" {
            dataFlag = false;
            expectedFlag = false;
            resetFlag = true;
        }
        else if buf == ~"" {
            dataFlag = false;
            expectedFlag = false;
            resetFlag = false;
        }
        else if dataFlag {
            data_string = buf;
        }
        else if expectedFlag {
            expected_str = buf;
        }

        if (resetFlag && !dataFlag && !expectedFlag) {
            let lwc_string: Option<arc::RWARC<~lwc_string>> = None;
            // io::println(fmt!("data_string = %?" , data_string));
            // io::println(fmt!("expected_str = %?" , expected_str));
            do lwc.write |l| {
                let lwc_string= Some(l.lwc_intern_string(copy data_string));
                // io::println(fmt!("lwc string = %?" , lwc_string.get_ref().clone()));
                let (a , _) = css__number_from_lwc_string(lwc_string.unwrap() , false);
                io::println(fmt!("a = %?" , a));
                // io::println(fmt!("b = %?" , b));
                assert!(fmt!("%?" , a)==expected_str);
            }
            // io::println(fmt!("lwc = %?" , lwc));

            data_string = ~"";
            expected_str = ~"";
        }
    }
}

#[test]
fn test_number() {
    number(~"data/number/number.dat");
}