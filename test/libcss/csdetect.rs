
extern mod std;	
extern mod testutils;
extern mod parserutils ; 
use parserutils::charset::aliases::*;
use parserutils::input::parserutils_filter::* ;
use parserutils::charset::csdetect::*;

use testutils::*;
use core::str::*;
use std::arc;
use core::io::*;

fn main() {
    io::println("csdetect");
}


fn testMain(fileName: ~str) {
	// io::println(~"testMain : "+ fileName);
	let len = css__parse_filesize(copy fileName);
	if len ==0 {
		return;
	}
	
	let ctx: @mut line_ctx_csdetect = @mut line_ctx_csdetect
		{
			buflen:len,
			bufused:0,
			buf:~[],
			enc:~"",
			indata:false,
			inenc:false
		};
	ctx.buf.push(0);//why?
	//ctx.enc += ~"\0";
	//ctx.enc.push_char(0);
	assert!(css__parse_testfile(copy fileName, ~handle_line, CSDETECT(ctx)) == true);
	if (ctx.bufused > 0 && ctx.buf[ctx.bufused - 1] == '\n' as u8)
		{ctx.bufused -= 1;}

	run_test(copy ctx.buf, ctx.bufused, copy ctx.enc);
}

pub fn handle_line(data:~str, pw:LINE_CTX_DATA_TYPE)-> bool {
	let ctx :@mut line_ctx_csdetect;

	match pw { CSDETECT(x) => ctx = x, LEX(_) => fail!(~"In File csdetect.rs, Function handle_line, argument LINE_CTX_DATA_TYPE contains incorrect struct line_ctx_lex") };

	if data.len() <= 0 {
		io::println("error");
		return true;
	}
	if data[0] == '#' as u8 {
		if ctx.inenc {
			if (ctx.buf[ctx.bufused - 1] == '\n' as u8) {
				ctx.bufused -= 1;	
			}
			run_test(copy ctx.buf, ctx.bufused,copy  ctx.enc);	
            ctx.buf[0]=0;
            ctx.enc=~"";//[0]=0;
            ctx.bufused =0;
		}
		ctx.indata = str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"data");
		ctx.inenc = str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"encoding");
		
	}
 	else {
		if (ctx.indata) {
			ctx.buf =  unsafe { ctx.buf.slice(0,ctx.bufused).to_owned() };
			ctx.buf += data.to_bytes();
			ctx.bufused += data.len();
		}
		if (ctx.inenc) {
			ctx.enc = (data);
			 unsafe {
			 	if (ctx.enc[ctx.enc.len() - 1] == '\n' as u8) {
			 		pop_char(&mut ctx.enc);
			 	}
			 }	
		}
	}

    return true;
}


pub fn run_test(data:~[u8],  _:uint, expectedEncoding:~str) {
	// io::println("inside csdetect run_test");
    // io::println(~"data = "+ from_bytes(*data));
    // io::println(~"expectedEncoding = "+expectedEncoding);
    
    let mut mibenum:u16 = 0;
    let mut failCount:uint = 0;

    match parserutils_filter(alias() ,copy expectedEncoding) {
        (x,PARSERUTILS_OK) =>{
            let mut filter_instance = x.unwrap();
            let (charsetOption,srcOption,error)= css__charset_extract(&data, mibenum, CSS_CHARSET_DEFAULT, filter_instance.instance.clone());
            assert!(match error {
                PARSERUTILS_OK=>true,
                _=>false
            }==true);
            mibenum = charsetOption.unwrap();
            //io::println(arc::get(&filter_instance.instance).parserutils_charset_mibenum_to_name(mibenum).unwrap());
            assert!(mibenum != 0);
            if !(mibenum == arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(to_upper(copy expectedEncoding))) {
                io::print("fail::");
                failCount += 1;
            }   
            io::println(fmt!(" Detected mibenum %?   Expected %? ",mibenum,arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(to_upper(copy expectedEncoding))));

            io::println(fmt!(" Detected charset=( %?) mibenum=(%?) Source %? Expected charset=(%?) mibenum=(%?)",arc::get(&filter_instance.instance).parserutils_charset_mibenum_to_name(mibenum).unwrap(),mibenum,srcOption.unwrap(),expectedEncoding,arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(to_upper(copy expectedEncoding))));
           
        },
        
        (_ , _) => fail!() 
    }
    assert!(failCount == 0);
}

/*#[test]
fn bom() {
    testMain(~"data/csdetect/bom.dat");
}*/

#[test]
fn bom_charset() {
    testMain(~"data/csdetect/bom-charset.dat");
}