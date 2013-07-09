
extern mod std;
extern mod extra;
extern mod testutils;
extern mod parserutils; 
extern mod css;

use parserutils::charset::aliases::*;
use parserutils::utils::errors::*;
use css::charset::csdetect::*;

use testutils::*;
use std::str::*;
use extra::arc;
use std::io;

fn testMain(fileName: ~str) {
    let len = css__parse_filesize(copy fileName);
    if len ==0 {
        return;
    }
    
    let ctx: @mut line_ctx_csdetect = @mut line_ctx_csdetect
        {
            buf:~[],
            enc:~"",
            indata:false,
            inenc:false
        };
    ctx.buf.push(0);//why?

    assert!(css__parse_testfile(copy fileName, ~handle_line, CSDETECT(ctx)));

}

pub fn handle_line(data:~str, pw:LINE_CTX_DATA_TYPE)-> bool {
    let mut result = true;

    let ctx :@mut line_ctx_csdetect;

    match pw { CSDETECT(x) => ctx = x, LEX(_) => fail!(~"In File csdetect.rs, Function handle_line, argument LINE_CTX_DATA_TYPE contains incorrect struct line_ctx_lex") };

    if data.len() <= 0 {
        io::println("error");
        return true;
    }
    if data[0] == '#' as u8 {
        if ctx.inenc {
            unsafe {
                if (ctx.buf.last() == &('\n' as u8)) {
                    ctx.buf.pop();
                }
            }
            result = run_test(copy ctx.buf, copy  ctx.enc); 
            ctx.buf = ~[];
            ctx.enc=~"";
        }
        ctx.indata = data.ends_with(&"data");
        ctx.inenc = data.ends_with(&"encoding");

    }
    else {
        if (ctx.indata) {
                
            ctx.buf = ctx.buf + data.to_str().as_bytes_with_null_consume();
        }
        if (ctx.inenc) {
            ctx.enc = (data);
             unsafe {
                if (ctx.enc[ctx.enc.len() - 1] == '\n' as u8) {
                    ctx.enc.pop_char();
                }
             }  
        }
    }

    return result;
}

pub fn run_test(data:~[u8],  expected_encoding:~str) -> bool {
    let alias_instance = alias();
    
    let expected_mibenum = 
        alias_instance.get().parserutils_charset_mibenum_from_name(copy expected_encoding);
    
    let mut mibenum:u16 = 0;
    let alias_instance = alias();

    let (charsetOption,srcOption,error)= css__charset_extract(data, mibenum, CSS_CHARSET_DEFAULT as int, alias_instance.clone());
    assert!(match error {
        PARSERUTILS_OK=>true,
        _=>false
    }==true);
    mibenum = charsetOption.unwrap();

    assert!(mibenum != 0);

    let detected_charset = 
        alias_instance.get().parserutils_charset_mibenum_to_name(mibenum).unwrap();

    io::println(fmt!(" Detected mibenum == %?, Expected mibenum == %? ", mibenum, expected_mibenum));
    io::println(fmt!(" Detected charset == %?, Source == %? Expected charset ==%?",detected_charset,srcOption.unwrap(), expected_encoding));    

    if !(mibenum == expected_mibenum) {
        return false;
    }

    true
}

/*#[test]
fn bom() {
    testMain(~"data/csdetect/bom.dat");
}*/

#[test]
fn bom_charset() {
    testMain(~"data/csdetect/bom-charset.dat");
}
