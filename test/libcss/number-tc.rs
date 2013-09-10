extern mod std;
extern mod css;
extern mod wapcaplet;

use css::parse::properties::common::*;
use wapcaplet::*;
use std::io;
use std::io::Create;

fn main() {
    debug!("number");
    // number(~"data/number/number.dat");
    // let i = print_css_fixed(1);
}

fn number(file_name: ~str, file_name_write: ~str) {
    let r:@Reader = io::file_reader(&Path(file_name)).unwrap();
    let w:@Writer = io::file_writer(&Path(file_name_write),[io::Create, io::Truncate]).unwrap();
    let lwc = wapcaplet::lwc();
    let mut dataFlag = false;
    let mut resetFlag = false;
    let mut data_string: ~str = ~"";

    while !r.eof() {
        let buf = r.read_line();
        // debug!(buf);
        if buf == ~"#data" {
            dataFlag = true;
            resetFlag = false;
        }
        else if buf == ~"#errors" || buf == ~"" {
            dataFlag = false;
            resetFlag = false;
        }
        else if buf == ~"#reset" {
            dataFlag = false;
            resetFlag = true;
        }
        else if dataFlag {
            data_string = buf;
        }

        if (resetFlag && !dataFlag ) {
             debug!(fmt!("data = %?" , data_string));
            // debug!(fmt!("expected_str = %?" , expected_str));
            let lwc_string= Some(lwc.lwc_intern_string(data_string));
            //debug!(fmt!("lwc string = %?" , lwc_string.get_ref().clone()));
            let (a , _) = css__number_from_lwc_string(lwc_string.unwrap() , false);
            // debug!(fmt!("a = %?" , a));
                
            let b = print_css_fixed(256, a);
            //debug!(fmt!("got: %s expected: %.*s\n", b, expected_str.len(), expected_str));
            //debug!(fmt!("expected is %?" , expected_str));
            debug!(fmt!("found is %? \n" , b));
            //assert!(b.starts_with(expected_str));
            // debug!(fmt!("lwc = %?" , lwc));
            w.write_line("#expected");
            w.write_line(b);	
            w.write_line("#reset");

            data_string = ~"";
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
    //debug!(fmt!("Result %?", a));
    let mut unitpart:u32 = b >> 10;
    debug!(fmt!("Expected Unitpart %?", unitpart));
    //debug!(fmt!("b %?", b));
    let mut fracpart:u32 = ((b & 0x3ff)*1000 + 500)/(1 << 10);
    debug!(fmt!("Expected Fracpart %?", fracpart));
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
    //debug!(fmt!("Buffer Length %?", buf.len()));
    if len > 0 {
        buf.push_char('.');
        len -=1;
    }
    //debug!(fmt!("Fracpart %?", fracpart));
    loop {
        tmp.push(string_number[fracpart%10] as char);
        fracpart /= 10;
        if !(tmp.len() < 20 && fracpart != 0 ) {
            break;    
        }
    }
    //debug!(fmt!("Fracpart %?", fracpart));

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
    number(~"data/number/number.dat", ~"data/number/number_result.dat");
}
