extern mod std;
extern mod css;
extern mod wapcaplet;

use css::parse::properties::common::*;
use wapcaplet::*;

fn main() {
    io::println("number");
    // number(~"data/number/number.dat");
    // let i = print_css_fixed(1);
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
        else if buf == ~"#errors" || buf == ~"" {
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
        else if dataFlag {
            data_string = buf;
        }
        else if expectedFlag {
            expected_str = buf;
        }

        if (resetFlag && !dataFlag && !expectedFlag) {
            // io::println(fmt!("data_string = %?" , data_string));
            // io::println(fmt!("expected_str = %?" , expected_str));
            do lwc.write |l| {
                let lwc_string= Some(l.lwc_intern_string(copy data_string));
                // io::println(fmt!("lwc string = %?" , lwc_string.get_ref().clone()));
                let (a , _) = css__number_from_lwc_string(lwc_string.unwrap() , false);
                // io::println(fmt!("a = %?" , a));
                
                let b = print_css_fixed(a);
                // io::println(fmt!("string expected is %?" , expected_str));
                // io::println(fmt!("string found is %?" , b));
                assert!(fmt!("%?" , b)==expected_str);
            }
            // io::println(fmt!("lwc = %?" , lwc));

            data_string = ~"";
            expected_str = ~"";
        }
    }
}

fn print_css_fixed(a: int) -> ~str {
    let b: int;
    let mut buf: ~str = ~"";
    if a < 0 {
        b = -a;
    }
    else {
        b = a;
    }
    let mut unitpart = b >> 10;
    let mut fracpart = ((b & 0x3ff)*1000 + 500)/(1 << 10);
    let mut flen: uint = 0;
    let mut tmp: ~[char] = ~[];
    
    if a < 0 {
        buf.push_char('-');
    }
    let string_number = ~"0123456789";

    loop {
        tmp.push(string_number[unitpart%10] as char);
        unitpart /= 10;
        if !(unitpart != 0 && tmp.len() < 20) {
            break;    
        }
    }
    
    for tmp.each_reverse |i| {
        buf.push_char(*i);
    }

    let mut len = buf.len();
    while len < 256 {
        buf.push_char('.');
        len += 1;
    }

    loop {
        tmp.push(string_number[fracpart%10] as char);
        fracpart /= 10;
        if !(fracpart != 0 && tmp.len() < 20) {
            break;    
        }
    }

    for tmp.each_reverse |i| {
        buf.push_char(*i);
        flen += 1;
    }
    
    len = 256 - buf.len();
    while len > 0 && flen < 3 {
        buf.push_char('0');
        len -= 1;
        flen += 1;
    }

    len = buf.len();
    while len < 256 {
        buf.push_char('0');
        len += 1;
    }

    return buf;
}

#[test]
fn test_number() {
    number(~"data/number/number.dat");
}