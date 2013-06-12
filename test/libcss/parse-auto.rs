extern mod std;
extern mod css;
extern mod wapcaplet;

use std::arc;
use css::css::*;
use css::stylesheet::*;
use css::utils::errors::*;
use wapcaplet::*;

pub fn resolve_url(_:~str, rel:arc::RWARC<~lwc_string>) -> (css_error,Option<arc::RWARC<~lwc_string>>) {
	return (CSS_OK,Some(rel.clone()));
}

pub struct stentry {
		off:uint,
		string:~str
}

pub struct exp_entry{
	ftype:int,
	name: ~str,
	bytecode:~[u32],
	stringtab:~[stentry]
}

pub struct line_ctx {
    buf:~[u8],

    exp:~[@mut exp_entry],

    indata:bool,
    inerrors:bool,
    inexp:bool,

    inrule:bool
}

pub fn is_string_caseless_equal(a : &str , b : &str ) -> bool {

	//io::println(fmt!("Strtol : strings are %? ====== %? ",a,b));
    if ( a.len() != b.len() ) {
        return false ;
    }
    
    let mut i :uint = a.len() ;
    for uint::range(0,i) |e| {
        if a[e] == b[e] {
            loop;
        }

        if (a[e] >= 'A' as u8  && a[e] <= 'Z'  as u8) {
            if (a[e]+32) == b[e] {
                loop;
            }
            else {
                return false ;
            }
        }

        if (b[e] >= 'A'  as u8 && b[e] <= 'Z'  as u8) {
            if (b[e]+32) == a[e] {
                loop;
            }
            else {
                return false ;
            }
        }
        return false ;
    }
    return true ;
}

pub fn strtol(data:~str , data_used: &mut uint) -> Option<int> {

	let mut res : i64  = 0 ;
	let mut negative : bool = false ;

	if *data_used >= data.len()  {
		return None ;
	}

	if data[*data_used] == ('-' as u8) {
		negative = true ;
		*data_used += 1;
	}
	else if data[*data_used] == ('+' as u8) {
		negative = false ;
		*data_used += 1;
	}
	
	while ( *data_used < data.len() ) {
		if (data[*data_used] > 47 && data[*data_used]<58 ) {
			res = res*10 + ( ( (data[*data_used] as u8) - 48 ) as i64);
			*data_used += 1;

			if( res >= (int::max_value as i64) ) {
				fail!(~"\n Excedded maximum value of an integer") ;
			}

			loop ;
		}	
		else {
			if negative {
				res = -res ;
			}
			return Some(res as int) ;
		}
	}

	if negative {
		res = -res ;
	}
	Some(res as int)
}

pub fn strtoul(data:~str , data_used: &mut uint) -> Option<uint> {

	let mut res : u64  = 0 ;

	if *data_used >= data.len()  {
		return None ;
	}

	if (data[*data_used] == ('-' as u8) ) || 
			(data[*data_used] == ('+' as u8) ) {
		*data_used += 1; // skip character
	}
	
	while ( *data_used < data.len() ) {
		if (data[*data_used] > 47 && data[*data_used]<58 ) {
			res = res*10 + ( ( (data[*data_used] as u8) - 48 ) as u64);
			*data_used += 1;

			if( res >= (uint::max_value as u64) ) {
				fail!(~"\n Excedded maximum value of an integer") ;
			}

			loop ;
		}	
		else {
			return Some(res as uint) ;
		}
	}

	Some(res as uint)
}

fn main() {
    io::println("parse-auto");
    // corresponding code now in parse_auto function, entry from test-cases
}

fn parse_auto(file: ~str) {

	let file_content_result = io::read_whole_file_str(&Path(file)) ;
	let mut file_content : ~str ;
	match file_content_result {
		Ok(x) => {
			file_content = x ;
		},
		Err(y) => {
			file_content = ~"" ;
			io::println(fmt!("\n Error opening file :%?",y));
			assert!(false) ;
		}
	}

	let mut ctx : @mut line_ctx = @mut line_ctx{
		buf:~[],
    	exp:~[],

	    indata:false,
	    inerrors:false,
	    inexp:false,
	    inrule:false
	};

	for str::each_line_any(file_content) |line| {
		let mut line_string : ~str = line.to_str() ;
		str::push_char(&mut line_string,'\n') ;
		
		handle_line(line_string,ctx);
	}
	io::println(fmt!("Ctx ====================================\n%?\n==============================",ctx));
	/* and run final test */
	if ( unsafe { ctx.buf.len()>0 } ) {
		run_test(ctx);
	}
}

pub fn handle_line(mut data:~str,ctx:@mut line_ctx) -> bool {

	let mut len : uint = 0 ;
	if (data[len] == ('#' as u8) ) {
		if (ctx.inexp) {
			/* This marks end of testcase, so run it */
			io::println(fmt!("Ctx ====================================\n%?\n==============================",ctx));
			run_test(ctx);

			ctx.buf = ~[];
		}

		if (ctx.indata && data.len()>=7 && 
				(is_string_caseless_equal( data.slice(1,7), "errors")) ) {

			ctx.indata = false;
			ctx.inerrors = true;
			ctx.inexp = false;
		} 
		else if (ctx.inerrors && data.len()>=9 && 
				(is_string_caseless_equal( data.slice(1,9), "expected"))) {

			ctx.indata = false;
			ctx.inerrors = false;
			ctx.inexp = true;
			ctx.inrule = false;
		} 
		else if (ctx.inexp && data.len()>=5 && 
				(is_string_caseless_equal( data.slice(1,5), "data"))) {

			ctx.indata = true;
			ctx.inerrors = false;
			ctx.inexp = false;
		} 
		else if (ctx.indata) {
			//ctx.buf = ~[] ;
			for str::each_char(data) |ch| {
				ctx.buf.push(ch as u8);
			}
			io::println(fmt!("Buffer is 1= %?",unsafe {copy ctx.buf}));
		} 
		else {
			ctx.indata = ( data.len()>=5 && is_string_caseless_equal( data.slice(1,5), "data") );
			ctx.inerrors = ( data.len()>=7 && is_string_caseless_equal( data.slice(1,7), "errors"));
			ctx.inexp = ( data.len()>=9 && is_string_caseless_equal( data.slice(1,9), "expected"));
		}
	} 
	else {
		if ctx.indata {
			//ctx.buf = ~[] ;
			for str::each_char(data) |ch| {
				ctx.buf.push(ch as u8);
			}
			io::println(fmt!("Buffer is 2= %?",unsafe {copy ctx.buf}));
		}
		if (ctx.inexp) {
			len = data.len() ;
			if (data[len - 1] == ('\n' as u8) ) {
				str::pop_char(&mut data);
			}

			css__parse_expected(ctx, data);
		}
	}

	true 
}

pub fn css__parse_expected(ctx:@mut line_ctx, data:~str) {

	let mut len : uint = 0 ;
	let mut _goto_start_rule : bool = true  ;
	if data.len()==0 || data[len] != ('|' as u8){
		return;
	}

	while _goto_start_rule {
		_goto_start_rule = false ;

		if( ctx.inrule==false) {
			len += 1;

			while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
				 (data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) ) && (data.len()>len) {
				len += 1;
			}

			let mut num = strtol (copy data,&mut len);

			while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
				 (data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) ) && (data.len()>len) {
				len += 1;
			}

			/* Append to list of expected rules */
			let min = if (data.len()-len) <= 128 { (data.len()-len) } else { 128 } ;

			let mut entry = @mut exp_entry{
				ftype: if num.is_some() { num.get() } 
					else {0} ,
				name: data.slice(len,len+min).to_str() ,
				bytecode:~[],
				stringtab:~[]
			};
			len += min ;

			ctx.exp.push(entry);
			ctx.inrule = true;
		}
		else {
			let mut explen = unsafe { ctx.exp.len()-1 };
			if explen < 0 {
				fail!(~"No exp entry found");
			}
			let mut rule = ctx.exp[explen] ;

			if( data[2] != (' ' as u8) ) {
				ctx.inrule = false ;
				_goto_start_rule = true ;
				loop ;
			}

			len += 1;
			while (len < data.len()) {

				/* Skip whitespace */
				while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
					 (data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) )&& (data.len()>len) {
					len += 1;
				}

				if len == data.len() {
					break ;
				}

				if data[len] == ('P' as u8) {

					let mut start = str::find_char( data.slice(len,data.len()) , '(') ;

					if start.is_none() {
						break ;
					}

					let mut end = str::find_char( data.slice(start.get(),data.len()) , '(') ;
					if end.is_none() {
						break ;
					}
						
					let mut stentry = stentry{
						off: 	(if unsafe{rule.bytecode.len()>0} {
									rule.bytecode[ unsafe{rule.bytecode.len()-1}]
								}
								else { 
									0
						}) as uint,
						string: data.slice( start.get()+1,end.get() ).to_str()
					} ;

					assert!( stentry.string.len()!=0 );
					rule.stringtab.push(stentry) ;
				}
				else {
					/* Assume hexnum */
					let mut val = strtoul(copy data,&mut len) ;
					/* Append to bytecode */
					rule.bytecode.push(val.get_or_default(0) as u32) ;
				}
			}
		}
	}
}

pub fn report_fail(data:~[u8] , e:@mut exp_entry) {

	io::println(fmt!("    Data: %? ", data) );

	io::println(fmt!("    Expected entry:") );
	io::println(fmt!("	entry type:%d name:%s\n", e.ftype, copy e.name) );
	io::println(fmt!("	bytecode ") );
	for e.bytecode.each_mut |code| {
		io::println(fmt!("%? ", code ));
	}
}

pub fn run_test(ctx:@mut line_ctx) {

	let mut error : css_error ;
	let mut params = css_params {
		/* ABI version of this structure */
		params_version : CSS_PARAMS_VERSION_1,

		/* The language level of the stylesheet */
		level: CSS_LEVEL_21,

		/* The charset of the stylesheet data, or NULL to detect */
		charset : Some(~"UTF-8"),
		/* URL of stylesheet */
		url : ~"foo",
		/* Title of stylesheet */
		title : ~"",

		/* Permit quirky parsing of stylesheet */
		allow_quirks : false,
		/* This stylesheet is an inline style */
		inline_style : false,

		/* URL resolution function */
		resolve : @resolve_url,

		/* Import notification function */
		import :None,

		/* Colour resolution function */
		color : None,

		/* Font resolution function */
		font : None,
	};

	let mut lwc_instance = lwc() ;

	let mut css_instance = css::css_create(copy params,Some(lwc_instance.clone())) ;

	error = css_instance.css_stylesheet_append_data(copy (ctx.buf));
	match error {
		CSS_OK=>{},
		CSS_NEEDDATA=>{},
		_=> { 
			io::println( fmt!("\n Failed appending data : %?",error) );
		}
	}

	error = css_instance.css_stylesheet_data_done();
	let mut pending_imports = false ;
	assert!( match error {
				CSS_OK=>{
					true
				},
				CSS_IMPORTS_PENDING=>{
					pending_imports=true; 
					true 
				},
				_=>{false}
			} );

	while  pending_imports {
		
		let mut (error,o_str,_) = css_instance.css_stylesheet_next_pending_import() ;
		assert!( match error {
			CSS_OK=>{
				true
			},
			CSS_INVALID=>{
				true 
			},
			_=>{false}
		} );

		let mut url = o_str.get_or_default(~"") ;

		match error {
			CSS_OK=> {
				params.url = copy url;

				let mut import = css::css_create(copy params,Some(lwc_instance.clone())) ;
				
				assert!( 	match css_instance.css_stylesheet_register_import(
														Some(import.stylesheet)) {
								CSS_OK=>{true},
								_=>{false}
							});

				pending_imports = true ;
			}
			_=>{}	
		}
	}
	let mut e : uint = 0;

	if (css_instance.stylesheet.rule_count != unsafe {ctx.exp.len()} ) {
		io::println(fmt!("Got %u rules. Expected %u\n",
				css_instance.stylesheet.rule_count , unsafe{ctx.exp.len()}) );
		fail!(~"Unexpected number of rules ") ;
	}

	let mut ptr = css_instance.stylesheet.rule_list ;
	loop {
		match ptr {
			None=>{ break },
			Some(crule) => {
				match crule {
				    RULE_SELECTOR(rule) => {
				    	if ( ctx.exp[e].ftype != (CSS_RULE_SELECTOR as int) ) {
				    		io::println(fmt!("Got type %d , Expected %d ",
				    			ctx.exp[e].ftype , (CSS_RULE_SELECTOR as int)  )) ;
				    		fail!(~"Expected type differs") ;
				    	}
				    	if validate_rule_selector(rule,ctx.exp[e]) {
		    				report_fail(copy ctx.buf,copy ctx.exp[e]);
		    				fail!(~"Validation of rule selector failed");
		    			}
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;
				    },
				    RULE_CHARSET(rule) => {
				    	if ( ctx.exp[e].ftype != (CSS_RULE_CHARSET as int) ) {
				    		io::println( fmt!("Got type %d , Expected %d ",
				    			ctx.exp[e].ftype , (CSS_RULE_SELECTOR as int) )) ;
				    		fail!(~"Expected type differs") ;
				    	}
				    	if validate_rule_charset(rule,ctx.exp[e]) {
		    				report_fail(copy ctx.buf,copy ctx.exp[e]);
		    			}
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;
				    },
				    RULE_IMPORT(rule) => {
				    	if ( ctx.exp[e].ftype != (CSS_RULE_IMPORT as int) ) {
				    		io::println( fmt!("Got type %d , Expected %d ",
				    			ctx.exp[e].ftype ,(CSS_RULE_SELECTOR as int)  ) );
				    		fail!(~"Expected type differs") ;
				    	}
				    	if validate_rule_import(rule,ctx.exp[e])  {
		    				report_fail(copy ctx.buf,copy ctx.exp[e]);
		    			}
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;
				    },
				   	RULE_UNKNOWN(_)=>{
				    	io::println(fmt!("Unhandled rule type %?", CSS_RULE_UNKNOWN)) ;
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;

				   	},
		    		RULE_MEDIA(_)=>{
				    	io::println(fmt!("Unhandled rule type %?", CSS_RULE_MEDIA)) ;
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;

		    		},
		    		RULE_FONT_FACE(_)=>{
				    	io::println(fmt!("Unhandled rule type %?", CSS_RULE_FONT_FACE)) ;
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;

		    		},
		    		RULE_PAGE(_)=>{
				    	io::println(fmt!("Unhandled rule type %?", CSS_RULE_PAGE) ) ;
				    	ptr = css_stylesheet::css__stylesheet_get_base_rule(crule).next; 
				    	e += 1 ;
				    	loop ;

		    		}
		    	}
		    }	
	    }
	}

	assert!( e== unsafe { (ctx.exp.len()-1) } );
	
	io::println("PASS\n");
}

pub fn validate_rule_selector(s:@mut css_rule_selector, e:@mut exp_entry ) -> bool {

	let mut name : ~str = ~"" ;
	let mut ptr : ~str = ~"" ;

  	// Build selector string
  	for s.selectors.each_mut |&sel| {
  		dump_selector_list(sel,&mut ptr) ;
  		name = name + ptr + ", ";
  		ptr = ~"" ;
  	}

  	/* Compare with expected selector */
	if name != e.name {
		io::println(fmt!("FAIL Mismatched names\n
						Got name '%s'. Expected '%s'\n",name,copy e.name) );
		return true ;
	}

	// Now compare bytecode
	if s.style.is_none() {
		if unsafe {e.bytecode.len()} == 0 {
			io::println(fmt!("FAIL: no bytecode created and no style created "));
			return false ;
		}
		else {
			io::println(fmt!("FAIL: no style found : style created,
								but bytecode not found"));
			return true ;
		}
	}
	else {
		if unsafe { e.bytecode.len()} == 0 {
			io::println(fmt!("FAIL: no bytecode found , but style created "));
			return true ;
		}
		else {
			let mut style = s.style.get() ;

			if unsafe { style.bytecode.len() != e.bytecode.len() } {
				io::println(fmt!("FAIL: bytecode length differs "));
				return true ;
			}

			let mut i = 0 ;
			let mut j = 0 ;

			while i < unsafe {e.bytecode.len()} {

				while j < unsafe { e.stringtab.len() } {
					if (e.stringtab[j].off == i) {
						break;
					}
					j += 1 ;
				}

				if (j != unsafe {e.stringtab.len()} ) {
					/* String */
					if( style.sheet.is_none() ) {
						io::println("\n Stylsheet not found in sheet , need sheet ");
		              	return false ;
		            }

		            let mut (res,op) = style.sheet.get().
		            			css__stylesheet_string_get(style.bytecode[i] as uint);

		            assert!( 	match res { 
		            				CSS_OK=>{true} , 
		            				_=>{false}
		            		});
		            let mut p = if op.is_some() { 
		            				op.unwrap() 
		            			}  
		            			else { 
		            				~"" 
		            			} ;   

		            if p != e.stringtab[j].string {
		            	io::println(fmt!("FAIL: string differs got %?, expected %? ",
		            			p , copy e.stringtab[j].string ) );
		            }

					i += 1;
				} 
				else if style.bytecode[i] != e.bytecode[i] {
					io::println(fmt!("FAIL Bytecode differs 
									Bytecode differs at %?", i) );
					while (i < unsafe {e.bytecode.len()} ) {
						io::println(fmt!("%? ", copy style.bytecode[i]));
						i += 1;
					}
					return true;
				}
			}
		}
	}
	false
}

pub fn validate_rule_charset(s:@mut css_rule_charset, e:@mut exp_entry) -> bool {

	unsafe {
		if( e.name.len() != s.encoding.len() ) {
			return false ;
		}
		let mut i =0 ;
		while ( i<s.encoding.len() ) {
			if ( s.encoding[i] != e.name[i] ) {
		    	return false ;
		  	}
		  	i += 1;
		}
		return true ;
	}
}

pub fn validate_rule_import(s:@mut css_rule_import, e:@mut exp_entry) -> bool {

  	unsafe {
		if( e.name.len() < s.url.len() ) {
			return false ;
		}
		let mut i =0 ;
		while ( i<s.url.len() ) {
			if ( s.url[i] != e.name[i] ) {
		    	return false ;
		    }
		    i += 1;
		}
		true
  	}
} 

fn dump_selector_list(list:@mut css_selector, ptr:&mut ~str){
	if list.combinator.is_some() {
		dump_selector_list(list.combinator.unwrap(),ptr);
	}
	match list.data[0].combinator_type {
		CSS_COMBINATOR_NONE=> {
			
		},
    	CSS_COMBINATOR_ANCESTOR=>{
    		ptr.push_char(' ');
    		
    	},
    	CSS_COMBINATOR_PARENT=>{
    		ptr.push_char(' ');
    		ptr.push_char('>');
    		ptr.push_char(' ');
			

    	},
    	CSS_COMBINATOR_SIBLING=>{
    		ptr.push_char(' ');
    		ptr.push_char('+');
    		ptr.push_char(' ');
    	},
   		CSS_COMBINATOR_GENERIC_SIBLING=>{
   			ptr.push_char(' ');
    		ptr.push_char('+');
    		ptr.push_char(' ');
   		}

	}
	dump_selector(list, ptr);
}

fn dump_selector(selector:@mut css_selector, ptr:&mut ~str){
	let mut d:~[@mut css_selector_detail] = copy selector.data;
	let mut iter:uint = 0;
	while iter < d.len() {
		dump_selector_detail(d[iter], ptr, (iter != d.len()-1) );
		iter += 1;
	}	
}

fn dump_selector_detail(detail:@mut css_selector_detail, ptr: &mut ~str, detail_next:bool ) {

	if detail.negate {
		str::push_str(ptr,&":not(");
	}
	match detail.selector_type {
		CSS_SELECTOR_ELEMENT=>{
			unsafe{
				if detail.qname.name.len() == 1 && 
						detail.qname.name[0] == ('*' as u8) && 
						!detail_next {
			   	
			   		str::push_str(ptr,copy detail.qname.name);
			   	}
			   	else if detail.qname.name.len() != 1 ||
		           detail.qname.name[0] != ('*' as u8) { 
		           str::push_str(ptr,copy detail.qname.name)
			   	}
			}
		},

    	CSS_SELECTOR_CLASS=> {

    		ptr.push_char('.');
			str::push_str(ptr,copy detail.qname.name);
    	},

    	CSS_SELECTOR_ID =>{
    		
    		ptr.push_char('#');
			str::push_str(ptr,copy detail.qname.name);
    	},

    	CSS_SELECTOR_PSEUDO_CLASS | CSS_SELECTOR_PSEUDO_ELEMENT =>{
    		ptr.push_char(':' );
			str::push_str(ptr,copy detail.qname.name);
			match detail.value_type {
				CSS_SELECTOR_DETAIL_VALUE_STRING=> {
					if detail.string.is_some() {
						ptr.push_char('(' );
						//let String = copy detail.string;
						str::push_str(ptr, (copy detail.string).unwrap());
						ptr.push_char(')' );
					}
				} ,
				_=>{
					str::push_str(ptr,fmt!("%?n+%?",copy detail.a,copy detail.b));
				}
			}
    	},

    	CSS_SELECTOR_ATTRIBUTE=>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_EQUAL =>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('=');
			ptr.push_char('"');
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"');
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_DASHMATCH=>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('|');
			ptr.push_char('=');
			ptr.push_char('"');
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"');
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_INCLUDES=>{
    		ptr.push_char('[');
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('~');
			ptr.push_char('=');
			ptr.push_char('"');
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"');
			ptr.push_char(']');
    	},
    	CSS_SELECTOR_ATTRIBUTE_PREFIX=>{
    		ptr.push_char('[' );
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('^' );
			ptr.push_char('=' );
			ptr.push_char('"' );
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"' );
			ptr.push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_SUFFIX=>{
    		ptr.push_char('[' );
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('$' );
			ptr.push_char('=' );
			ptr.push_char('"' );
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"' );
			ptr.push_char(']' );
    	},
    	CSS_SELECTOR_ATTRIBUTE_SUBSTRING=>{
    		ptr.push_char('[' );
			str::push_str(ptr,copy detail.qname.name);
			ptr.push_char('*' );
			ptr.push_char('=' );
			ptr.push_char('"' );
			str::push_str(ptr,(copy detail.string).unwrap());
			ptr.push_char('"' );
			ptr.push_char(']' );
    	}
	}
	if detail.negate {
		ptr.push_char(')');
	}
}

#[test]
fn parse_tests1() {
	parse_auto(~"data/parse/tests1.dat");
}

#[test]
fn parse_atrules() {
	parse_auto(~"data/parse/atrules.dat");
}

#[test]
fn parse_colours() {
	parse_auto(~"data/parse/colours.dat");
}

#[test]
fn parse_colours_hsl() {
	parse_auto(~"data/parse/colours-hsl.dat");
}

#[test]
fn parse_nth() {
	parse_auto(~"data/parse/nth.dat");
}

#[test]
fn parse_properties() {
	parse_auto(~"data/parse/properties.dat");
}

#[test]
fn parse_selectors() {
	parse_auto(~"data/parse/selectors.dat");
}

/////////////////////////////////////////