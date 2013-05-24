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
             io::println(fmt!("data = %?" , data_string));
            // io::println(fmt!("expected_str = %?" , expected_str));
            do lwc.write |l| {
                let lwc_string= Some(l.lwc_intern_string(copy data_string));
                //io::println(fmt!("lwc string = %?" , lwc_string.get_ref().clone()));
                let (a , _) = css__number_from_lwc_string(lwc_string.unwrap() , false);
                // io::println(fmt!("a = %?" , a));
                
                let b = print_css_fixed(256, a);
                //io::println(fmt!("got: %s expected: %.*s\n", b, expected_str.len(), expected_str));
                io::println(fmt!("expected is %?" , expected_str));
                io::println(fmt!("found is %? \n" , b));
                assert!(str::starts_with(b, expected_str));
            }
            // io::println(fmt!("lwc = %?" , lwc));

            data_string = ~"";
            expected_str = ~"";
        }
    }
}

fn print_css_fixed(mut len:uint, a: i32) -> ~str {
    let b: u32;
    let mut buf: ~str = ~"";
    if a < 0 {
        b = -a as u32;
    }
    else {
        b = a as u32;
    }
    //io::println(fmt!("Result %?", a));
    let mut unitpart:u32 = b >> 10;
    io::println(fmt!("Expected Unitpart %?", unitpart));
    //io::println(fmt!("b %?", b));
    let mut fracpart:u32 = ((b & 0x3ff)*1000 + 500)/(1 << 10);
    io::println(fmt!("Expected Fracpart %?", fracpart));
    let mut flen: uint = 0;
    let mut tmp: ~[char] = ~[];

    if a < 0 {
        buf.push_char('-');
        len -= 1;
    }
    let string_number = ~"0123456789";

    loop {
        tmp.push(string_number[unitpart%10] as char);
        unitpart /= 10;
        if unitpart == 0 || tmp.len() >= 20 {
            break;    
        }
    }
    
    while (len > 0 && tmp.len() > 0) {
        buf.push_char(tmp.pop());
        len -= 1;
    }
    //io::println(fmt!("Buffer Length %?", buf.len()));
    if len > 0 {
        buf.push_char('.');
        len -=1;
    }
    //io::println(fmt!("Fracpart %?", fracpart));
    loop {
        tmp.push(string_number[fracpart%10] as char);
        fracpart /= 10;
        if !(tmp.len() < 20 && fracpart != 0 ) {
            break;    
        }
    }
    //io::println(fmt!("Fracpart %?", fracpart));

    while (len > 0 && tmp.len() > 0) {
        buf.push_char(tmp.pop());
        flen += 1;
        len -= 1;
    }
    
    
    while len > 0 && flen < 3 {
        buf.push_char('0');
        len -= 1;
        flen += 1;
    }
    
    return buf;
}

#[test]
fn test_number() {
    number(~"data/number/number.dat");
}