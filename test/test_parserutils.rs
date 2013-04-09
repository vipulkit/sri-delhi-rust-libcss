//sandy
extern mod std;

extern mod parserutils ; 
use parserutils::* ;
// use libc::c_uint;
//use libc::size_t  ;
fn sss(data: ~[u8], smibenum:~u16, source:~u32) -> parserutils_result
{
	return PARSERUTILS_GENERAL_OK;
}

fn main() {
	let mut parser : @mut lpu = lpu();
	//let mut asc: parserutils_inputstream_private_ptr;
	let pRslt= parser.parserutils_inputstream_create(~"ASCII",1);
	let mut stream:parserutils_inputstream;
	match(pRslt)
	{
		PARSERUTILS_INPUTSTREAM_CREATE_OK(temp)=>
		{ 
			let mut data:~[u8]= ~[10,10,10,10,10,10/*,10,10,10,10,10*/];
			let mut dta2:~[u8]= ~[10,10,10,10,10,10,10];
			stream = temp;
			parser.print_inputstream(& mut stream);
			lpu::parserutils_inputstream_insert(&mut stream,dta2);
			parser.print_inputstream(&mut stream);
			lpu::parserutils_inputstream_append(&mut stream,data);
			parser.print_inputstream(&mut stream);
			let source:@ mut u32= @mut 0;
			let opt= parser.parserutils_inputstream_read_charset(&mut stream,source);
			io::println(~"opt ="+opt.get());
			io::println(fmt!("1%?",source));
			parser.parserutils_inputstream_change_charset(&mut stream, ~"ISO-10646-UTF-1", 0);
			let source:@ mut u32= @mut 0;
			let opt= parser.parserutils_inputstream_read_charset(&mut stream,source);
			io::println(~"opt ="+opt.get());
			io::println(fmt!("2%?",source));
			let length:@mut uint= @mut 0;
			let ptr:@mut ~[u8] =@mut ~[];
			let mut (ptr,length):(~[u8],uint)= (~[],0) ;
			match(parser.parserutils_inputstream_peek(&mut stream,5))
			{
				PARSERUTILS_PEEK_OK(x,y)=>{
					ptr=x;
					length=y;
				},
				_=>{}
			}
			io::println(fmt!("%?,%?",ptr,length));
		},
		_=>{}
	}


	// io::println(fmt!("\n == Start testing Lib ParserUtils library functionality == ")) ;

	
	// let mut canon_opt : Option<parserutils_charset_aliases_canon>  = parser.parserutils__charset_alias_canonicalise(&~"US-ASCII");
	// if canon_opt.is_some() {
	// 	io::println(fmt!("\n Cannon value is %? , %s ",canon_opt.get().mib_enum , canon_opt.get().name ));
	// }
    
	// let mut mib_enum  = parser.parserutils_charset_mibenum_from_name(~"US-ASCII");
	// io::println(fmt!("\n Mib Enum value is %?  ",mib_enum ));

	// let mut name_opt   = parser.parserutils_charset_mibenum_to_name(mib_enum);
	// if !name_opt.is_none() {
	// 	io::println(fmt!("\n Name value for given mib is %s ",name_opt.get() ));
	// }

	// let presult : parserutils_result = parser.parserutils__filter_create(~"CP1256",~[]);
	// match presult {
	// 	PARSERUTILS_FILTER_CREATE_OK(x) =>  { 
	// 		let presult2 = parser.parserutils__filter_setopt(x,~"ASCII") ;
	// 	},
	// 	y => io::println(fmt!("\n Parser utils create filter failed = %? ",y)) 
	// }
	
	// io::println(fmt!("\n == End testing Lib ParserUtils library functionality   == ")) ;
}
