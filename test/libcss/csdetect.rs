extern mod std;	
//extern mod test_utils;
extern mod parserutils ; 
use parserutils::charset::aliases::*;
use parserutils::input::parserutils_filter::* ;
use parserutils::charset::csdetect::*;

//use test_utils::*;
use core::str::*;
use std::arc;
use core::io::*;

struct line_ctx {
	buflen:uint,
	bufused:uint,
	buf:~[u8],
	enc:~str,
	indata:bool,
	inenc:bool
}
pub type  line_func =  
    ~extern fn(data:~str , pw:&mut line_ctx) -> bool;

fn main() {
	io::println("csdetect");
}

pub fn css__parse_filesize( fileName:~str)->uint {
	let r:@Reader = io::file_reader(&Path(fileName)).get(); 
	r.seek(0,SeekEnd);
	 r.tell()
}
pub fn css__parse_testfile(filename:~str,  callback:line_func, pw:&mut line_ctx)->bool {
	let r:@Reader = io::file_reader(&Path(filename)).get();
	let mut data:~str;
	let mut string:~str;
	while(!r.eof()) {				
       data= r.read_line();
       io::println(data);
       let numOfbuffers= data.len()/300 + 1 ;
       //let mut v =~[];
       let mut iter = 0;
       while iter < (numOfbuffers-1) {
       		string = data.slice(iter * 300 ,(iter +1) * 300).to_owned();
       		if string.len() == 0 {
       			loop;
       		}
       		if !(*callback)(string, pw) {
       	 		return false;
       		}
       		iter += 1;
       }
       string = data.slice(iter * 300, data.len()).to_owned();
       if string.len() > 0 {
       		if !(*callback)(string,pw) {
       	 		return false;
       		}	
       }
       
	}
	true
}
fn testMain(fileName: ~str) {
	io::println(~"testMain : "+ fileName);
	let len = css__parse_filesize(copy fileName);
	if len ==0 {
		return;
	}
	let mut ctx: line_ctx = line_ctx
	{
		mut buflen:len,
		mut bufused:0,
		mut buf:~[],
		mut enc:~"",
		mut indata:false,
		mut inenc:false
	};
	ctx.buf.push(0);//why?
	//ctx.enc += ~"\0";
	//ctx.enc.push_char(0);
	assert!(css__parse_testfile(copy fileName, ~handle_line, &mut ctx) == true);
	if (ctx.bufused > 0 && ctx.buf[ctx.bufused - 1] == '\n' as u8)
		{ctx.bufused -= 1;}

	run_test(&ctx.buf, ctx.bufused, copy ctx.enc);
}

pub fn handle_line(data:~str,  /*datalen:uint,*/ pw:&mut line_ctx)-> bool {
	let mut ctx:&mut line_ctx =  pw;
	if data.len() <= 0 {
		io::println("error");
		return true;
	}
	if data[0] == '#' as u8 {
		if ctx.inenc {
			if (ctx.buf[ctx.bufused - 1] == '\n' as u8) {
				ctx.bufused -= 1;	
			}
			run_test(&ctx.buf, ctx.bufused,copy  ctx.enc);	
            ctx.buf[0]=0;
            ctx.enc=~"";//[0]=0;
            ctx.bufused =0;
		}
		ctx.indata = str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"data");
		ctx.inenc = str::eq(&data.slice(1,data.len()).to_owned().to_lower(),&~"encoding");
		
	}
 	else {
		if (ctx.indata) {
			ctx.buf = ctx.buf.slice(0,ctx.bufused).to_owned();
			ctx.buf += data.to_bytes();
			ctx.bufused += data.len();
		}
		if (ctx.inenc) {
			ctx.enc = (data);
			 if (ctx.enc[ctx.enc.len() - 1] == '\n' as u8) {
			 	//pop_char( ctx.enc);
			 }
			 	
		}
	}

	return true;
}

pub fn run_test(data:&~[u8],  _:uint, expectedEncoding:~str) {
	io::println("inside csdetect run_test");
    io::println(~"data = "+ from_bytes(*data));
    io::println(~"expectedEncoding = "+expectedEncoding);
	
	let mut mibenum:u16 = 0;

	match parserutils_filter(alias() ,copy expectedEncoding) {
        (x,PARSERUTILS_OK) =>{
            let mut filter_instance = x.unwrap();
            let (charsetOption,srcOption,error)= css__charset_extract(data, mibenum, CSS_CHARSET_DEFAULT, filter_instance.instance.clone());
            assert!(match error {
            	PARSERUTILS_OK=>true,
            	_=>false
            }==true);
            mibenum = charsetOption.unwrap();
            //io::println(arc::get(&filter_instance.instance).parserutils_charset_mibenum_to_name(mibenum).unwrap());
			assert!(mibenum != 0);
			io::println(fmt!(" Detected mibenum %?   Expected %? ",mibenum,arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(to_upper(copy expectedEncoding))));
			//assert!(mibenum == arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(to_upper(copy expectedEncoding)));    
			io::println(fmt!(" Detected charset=( %?) mibenum=(%?) Source %? Expected charset=(%?) mibenum=(%?)",arc::get(&filter_instance.instance).parserutils_charset_mibenum_to_name(mibenum).unwrap(),mibenum,srcOption.unwrap(),expectedEncoding,arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(to_upper(copy expectedEncoding))));
           
        },
        
        (_ , y) => fail!() 
    }
}

/*#[test]
fn bom() {
	testMain(~"data/csdetect/bom.dat");
}*/

#[test]
fn bom_charset() {
	testMain(~"data/csdetect/bom-charset.dat");
}