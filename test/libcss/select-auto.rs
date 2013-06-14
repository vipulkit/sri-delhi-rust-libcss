extern mod std;
extern mod css;
extern mod wapcaplet;
extern mod dumpcomputed;

use std::arc;
use css::css::*;
use wapcaplet::*;

use css::include::types::*;
use css::utils::errors::*;
use css::select::common::*;
use css::stylesheet::*;
use css::select::select::*;
use dumpcomputed::*;

use css::include::properties::*;
use css::include::fpmath::*;


pub struct attribute {
	name:arc::RWARC<~lwc_string>,
	value:arc::RWARC<~lwc_string>
}

pub struct node {
	name:Option<arc::RWARC<~lwc_string> >,
	attrs:~[attribute],

	parent:Option<@mut node>,
	next:Option<@mut node>,
	prev:Option<@mut node>,
	children:Option<@mut node>,
	last_child:Option<@mut node>
}

pub struct sheet_ctx {
	sheet:@mut css,
	origin:css_origin,
	media:u64
}

pub struct line_ctx {
	explen:uint,
	expused:uint,
	exp:~str,

	intree:bool,
	insheet:bool,
	inerrors:bool,
	inexp:bool,

	tree:Option<@mut node>,
	current:Option<@mut node>,
	depth:u32,

	sheets:~[@mut sheet_ctx],

	media:u32,
	pseudo_element:u32,
	target:Option<@mut node>,
	
	attr_class:arc::RWARC<~lwc_string>,
	attr_id:arc::RWARC<~lwc_string>,

	lwc_instance:arc::RWARC<~lwc>
} 

pub fn select_test(file:~str) {
	let mut lwc_ins = lwc() ;
	let mut lwc_attr_class : Option<arc::RWARC<~lwc_string>> = None;
	let mut lwc_attr_id : Option<arc::RWARC<~lwc_string>> = None ;

	do lwc_ins.write |l| {
        lwc_attr_class = Some(l.lwc_intern_string(~"class"));
        lwc_attr_id = Some(l.lwc_intern_string(~"id"));
    }

	let mut ctx : @mut line_ctx = @mut line_ctx{
		explen:0,
		expused:0,
		exp:~"",

		intree:false,
		insheet:false,
		inerrors:false,
		inexp:false,

		tree:None,
		current:None,
		depth:0,

		sheets:~[],

		media:0,
		pseudo_element:0,
		target:None,
		
		attr_class:lwc_attr_class.swap_unwrap(),
		attr_id:lwc_attr_id.swap_unwrap(),

		lwc_instance:lwc_ins.clone()
	};

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

	for str::each_line_any(file_content) |line| { 
        	let mut line_string: ~str = line.to_str(); 
		line_string.push_char('\n');
		io::println(fmt!("%?",line_string)); 
	        handle_line(line_string,ctx);
    	}	

	if( ctx.tree.is_some() ) {
		run_test(ctx);
	}
}

pub fn resolve_url(base:~str, rel:arc::RWARC<~lwc_string>) -> (css_error,Option<arc::RWARC<~lwc_string>>){

	(CSS_OK, Some(rel.clone()))
}

pub fn css_create_params() -> css_params {
    let css_param = css_params {
        params_version : CSS_PARAMS_VERSION_1,
        level: CSS_LEVEL_21,
        charset : Some(~"UTF-8"),
        url : ~"foo",
        title : ~"foo",
        allow_quirks : false,
        inline_style : false,
        resolve : @resolve_url,
        import : None,
        color : None,
        font : None
    };
     return css_param;
}

pub fn main() {
	io::println(fmt!("\n Starting select-auto test cases "));
}

pub fn handle_line(data:&str , ctx:@mut line_ctx) -> bool {
	let mut error : css_error ;
	let mut len : uint ; 
	if ( data[0] == ('#' as u8) ) {
	    if( ctx.intree ) {

            if( data.len() >= 7 && is_string_caseless_equal(data.slice(1,7), "errors") ){
                ctx.intree = false;
                ctx.insheet = false;
                ctx.inerrors = true ;
                ctx.inexp = false;
            }
            else {
                /* Assume start of stylesheet */
                css__parse_sheet(ctx, data.slice(1,data.len()-1) );

                ctx.intree = false;
                ctx.insheet = true;
                ctx.inerrors = false;
                ctx.inexp = false;
            }
        }
        else if (ctx.insheet) {

            if(data.len() >= 6 && is_string_caseless_equal(data.slice(1,6), "errors")){
                len = unsafe { ctx.sheets.len() -1 } ;
                assert!( 
                        match ctx.sheets[len].sheet.css_stylesheet_data_done() {
                                CSS_OK=>{true},
                                _=>{false}
                        });
                ctx.intree = false;
                ctx.insheet = false;
                ctx.inerrors = true ;
                ctx.inexp = false;
            }
            else if data.len() >= 2 && is_string_caseless_equal(data.slice(1,2), "ua") ||
                        data.len() >= 4 && is_string_caseless_equal(data.slice(1,4), "user") ||
                        data.len() >= 6 && is_string_caseless_equal(data.slice(1,6), "author") {
                
                len = unsafe { ctx.sheets.len() -1 } ;
                assert!( 
                        match ctx.sheets[len].sheet.css_stylesheet_data_done() {
                            CSS_OK=>{true},
                            _=>{false}
                        });
                css__parse_sheet(ctx, data.slice(1,data.len()-1) );
            }
            else {
                len = unsafe { ctx.sheets.len() -1 } ;
                let mut error = ctx.sheets[len].sheet.css_stylesheet_append_data(data.to_bytes());
                assert!( match error {
                            CSS_OK=>{true},
                            CSS_NEEDDATA=>{true},
                            _=>{false}
                         });
            }
        }
        else if (ctx.inerrors) {
            ctx.intree = false;
            ctx.insheet = false;
            ctx.inerrors = false;
            ctx.inexp = true;
        }
        else if (ctx.inexp) {
            /* This marks end of testcase, so run it */
            run_test(ctx);
	
	    ctx.expused = 0;

            ctx.intree = false;
            ctx.insheet = false;
            ctx.inerrors = false;
            ctx.inexp = false;
        }
        else {
            /* Start state */
            if(data.len()>=4 && is_string_caseless_equal(data.slice(1,4), "tree")) {

                css__parse_tree(ctx, data.slice(5, data.len()-1) );
                ctx.intree = true;
                ctx.insheet = false;
                ctx.inerrors = false ;
                ctx.inexp = false;
            }
        }
    }
    else {
        if ( ctx.intree ){
            /* Not interested in the '|' */
            css__parse_tree_data(ctx, data.slice(1,data.len()-1) );
        }
        else if ( ctx.insheet ) {
            len = unsafe { ctx.sheets.len() -1 } ;
            error = ctx.sheets[len].sheet.css_stylesheet_append_data(data.to_bytes());
            assert!( match error {
                        CSS_OK=>{true},
                        CSS_NEEDDATA=>{true},
                        _=>{false}
                     });
        }
        // Not Needed
        //else if ( ctx.inexp ) {
        //  css__parse_expected(ctx, data );
        //}
    }
    true 
}


pub fn isspace (ch:u8)-> bool {
	if ( (ch==0x20 ) || (ch==0x09) || (ch==0x0a) || 
			 (ch==0x0b) || (ch==0x0c) || (ch==0x0d) ){
		true
	}
	else {
		false
	} 
}

pub fn css__parse_tree(ctx:@mut line_ctx, data:&str) {
	/* [ <media_list> <pseudo>? ] ? */
	ctx.media = CSS_MEDIA_ALL as u32;
	ctx.pseudo_element = CSS_PSEUDO_ELEMENT_NONE as u32;

	/* Consume any leading whitespace */
	let mut data2 = data.trim_left();

	let mut length_processed : uint = 0 ;
	if (data2.len()>0) {
		length_processed = css__parse_media_list(data2,ctx);
	}

	if length_processed < data2.len() {
		css__parse_pseudo_list(data2.slice(length_processed,data2.len()-1),ctx);
	}
}

pub fn css__parse_tree_data(ctx:@mut line_ctx, data:&str) {
	
	let mut p = 0;
	let end = data.len();

	let mut value = None;
	let mut namelen = 0;
	let mut valuelen = 0;
	let mut depth:u32 = 0;
	let mut target = false;
	let mut lwc_ins = lwc();

	/* ' '{depth+1} [ <element> '*'? | <attr> ]
	 * 
	 * <element> ::= [^=*[:space:]]+
	 * <attr>    ::= [^=*[:space:]]+ '=' [^[:space:]]*
	 */

	while (p < end && isspace(data[p])) {
		depth += 1;
		p += 1;
	}
	depth -= 1;

	/* Get element/attribute name */
	let name_begin = p;
	while (p < end && data[p] != '=' as u8 && data[p] != '*' as u8  && isspace(data[p]) == false) {
		namelen += 1;
		p += 1;
	}

	let mut name = data.slice(name_begin,name_begin+namelen);

	/* Skip whitespace */
	while (p < end && isspace(data[p])){
		p += 1;
	}
	
	let mut value_begin = 0;

	if (p < end && data[p] == '=' as u8 ) {
		/* Attribute value */
		p += 1;

		value_begin = p;

		while (p < end && isspace(data[p]) == false) {
			valuelen += 1;
			p += 1;
		}
	} else if (p < end && data[p] == '*' as u8 ) {
		/* Element is target node */
		target = true;
	}

	if valuelen > 0 {
		value = Some(data.slice(value_begin, value_begin+valuelen));
	}

	if (value.is_none() ) {
		/* We have an element, so create it */
		let n : @mut node = @mut node {
			name:None,
			attrs:~[],
			parent:None,
			next:None,
			prev:None,
			children:None,
			last_child:None
		};					
		do lwc_ins.write |l| {
			n.name = Some(l.lwc_intern_string(name.to_owned()));
		}

		/* Insert it into tree */
		if ctx.tree.is_none() {
			ctx.tree = Some(n);
		} 
		else {
			assert!(depth > 0);
			assert!(depth <= ctx.depth + 1);

			/* Find node to insert into */
			while (depth <= ctx.depth) {
				ctx.depth -= 1;
				ctx.current = ctx.current.unwrap().parent;
			}
			let ctx_current = ctx.current.unwrap();	
			/* Insert into current node */
			if (ctx_current.children.is_none()) {
				ctx_current.children = Some(n);
				ctx_current.last_child = Some(n);
			} else {
				ctx_current.last_child.get_mut_ref().next = Some(n);
				n.prev = ctx_current.last_child;

				ctx_current.last_child = Some(n);
			}
		 	ctx.current = Some(ctx_current);	
			n.parent = ctx.current;
		}

		ctx.current = Some(n);
		ctx.depth = depth;

		/* Mark the target, if it's us */
		if (target) {
			ctx.target = Some(n);
		}

	} 
	else {
		/* New attribute */

		let mut lwc_name:Option<arc::RWARC<~lwc_string> > = None;
		let mut lwc_value:Option<arc::RWARC<~lwc_string> > = None;

		do lwc_ins.write |l| {
			lwc_name = Some(l.lwc_intern_string(name.to_owned()));
			lwc_value = Some(l.lwc_intern_string(value.get_ref().to_owned()));
		}
		
		let mut attr: attribute = attribute{
			name:lwc_name.unwrap(),
			value:lwc_value.unwrap()
		};

		ctx.current.unwrap().attrs.push(attr);

	}

}

pub fn css__parse_sheet(ctx:@mut line_ctx, data:&str) {

    let mut origin : css_origin = CSS_ORIGIN_AUTHOR;
    let mut p : uint = 0;
    let end : uint = data.len();
    /* Find end of origin */
    while p < end && !isspace(data[p]) {
        p += 1;
    }
    
    if p == 6 && data.len() >= 6 && is_string_caseless_equal(data.slice(0,6), "author"){
        origin = CSS_ORIGIN_AUTHOR;
    }
    else if p == 4 && data.len() >= 4 && is_string_caseless_equal(data.slice(0,4), "user"){
        origin = CSS_ORIGIN_USER;
    }
    else if p == 2 && data.len() >= 2 && is_string_caseless_equal(data.slice(0,2), "ua"){
        origin = CSS_ORIGIN_UA;
    }
    else {
			println("Unknown stylesheet origin");
            assert!(false);
    }
    
    /* Skip any whitespace */
    while p < end && isspace(data[p]) {
        p += 1;
    }
    
    if p < end {
       css__parse_media_list(data.slice(p, end), ctx);
    }
    let params = css_create_params();
    let sheet:@mut css = css::css_create(params, None);
    let sheet_ctx = @mut sheet_ctx {
        sheet: sheet,
        origin: origin,
        media: ctx.media as u64
    };
    
    ctx.sheets.push(sheet_ctx);
}


pub fn css__parse_media_list(data:&str , ctx:@mut line_ctx) -> uint {

	// ' '	(0x20)	space (SPC)
	// '\t'	(0x09)	horizontal tab (TAB)
	// '\n'	(0x0a)	newline (LF)
	// '\v'	(0x0b)	vertical tab (VT)
	// '\f'	(0x0c)	feed (FF)
	// '\r'	(0x0d)	carriage return (CR)
	let mut len : uint = 0 ;
	let mut result : u64 = 0 ;
	while len < data.len() {

		/* consume a medium */
		if ( (data[len]!=0x20) && (data[len]!=0x09) && (data[len]!=0x0a) && 
			 (data[len]!=0x0b) && (data[len]!=0x0c) && (data[len]!=0x0d)  && data.len()>len) {
			if( data[len]!= (',' as u8)) {
				len += 1;
				loop ;
			}
		}

        if ( (data.len()>(10+len)) && data.len() >= len+10 && is_string_caseless_equal(data.slice(len,len+10), "projection") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(8+len)) && data.len() >= len+8 && is_string_caseless_equal(data.slice(len,len+8), "handheld") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(8+len)) && data.len() >= len+8 && is_string_caseless_equal(data.slice(len,len+8), "embossed") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(7+len)) && data.len() >= len+7 && is_string_caseless_equal(data.slice(len,len+7), "braille") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(6+len)) && data.len() >= len+6 && is_string_caseless_equal(data.slice(len,len+6), "speech") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(6+len)) && data.len() >= len+6 && is_string_caseless_equal(data.slice(len,len+6), "screen") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(5+len)) && data.len() >= len+5 && is_string_caseless_equal(data.slice(len,len+5), "print") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(5+len)) && data.len() >= len+5 && is_string_caseless_equal(data.slice(len,len+5), "aural") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(3+len)) && data.len() >= len+3 && is_string_caseless_equal(data.slice(len,len+3), "tty") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(3+len)) && data.len() >= len+3 && is_string_caseless_equal(data.slice(len,len+3), "all") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (data.len()>(2+len)) && data.len() >= len+2 && is_string_caseless_equal(data.slice(len,len+2), "tv") ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else {
            // unknown media type
            io::println("\n Unknown Media type encountered");
            assert!(false);
        }

		/* Consume whitespace */
		while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
			 	(data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) ) && data.len()>len {
				len += 1;
		}

		/* Stop if we've reached the end */
		if ( data.len() <= len ) {
			break;
		}

		if data[len] == (',' as u8) {
			len += 1;
		}

		/* Consume whitespace */
		while ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
			 	(data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) ) && data.len()>len {
				len += 1;
		}	
	}
	
	ctx.media = result as u32 ;
	len
}

pub fn css__parse_pseudo_list(data:&str,ctx:@mut line_ctx) -> uint {
	

	/*
	const char *p = *data;
	const char *end = p + *len;

	// <pseudo> [ ',' <pseudo> ]* 

	*element = CSS_PSEUDO_ELEMENT_NONE;

	while (p < end) {
		const char *start = p;

		// consume a pseudo 
		while (isspace(*p) == false && *p != ',')
			p++;

		// Pseudo elements 
		if (p - start == 12 &&
				strncasecmp(start, "first-letter", 12) == 0)
			*element = CSS_PSEUDO_ELEMENT_FIRST_LETTER;
		else if (p - start == 10 &&
				strncasecmp(start, "first-line", 10) == 0)
			*element = CSS_PSEUDO_ELEMENT_FIRST_LINE;
		else if (p - start == 6 &&
				strncasecmp(start, "before", 6) == 0)
			*element = CSS_PSEUDO_ELEMENT_BEFORE;
		else if (p - start == 5 &&
				strncasecmp(start, "after", 5) == 0)
			*element = CSS_PSEUDO_ELEMENT_AFTER;
		else
			assert(0 && "Unknown pseudo");

		// Consume whitespace 
		while (p < end && isspace(*p))
			p++;

		// Stop if we've reached the end 
		if (p == end || *p != ',')
			break;

		// Consume comma
		p++;

		// Consume whitespace 
		while (p < end && isspace(*p))
			p++;
	}

	*data = p;
	*len = end - p; */
	0
}

//Not Needed
//pub fn css__parse_expected(ctx:@mut line_ctx, data:&str) {
//
//}

pub fn run_test( ctx:@mut line_ctx) {
    let mut select: ~css_select_ctx;
    let mut results: css_select_results;

    let mut i:u32=0;
    let mut buf:~str= ~"";
    //let mut bufLen:uint;
    let mut testnum: int;//TODO static

    select = css_select_ctx::css_select_ctx_create();
    unsafe {
        while i < (ctx.sheets.len() as u32) {
            match select.css_select_ctx_append_sheet(ctx.sheets[i].sheet.stylesheet,ctx.sheets[i].origin,ctx.sheets[i].media) {
                CSS_OK => {},
                _ => fail!()
            }
            i += 1;
        }
    }
    let select_handler: @mut css_select_handler = @mut css_select_handler {
    node_name: @node_name,

    node_classes: @node_classes,

    node_id: @node_id,

    named_ancestor_node: @named_ancestor_node,
   
    named_parent_node: @named_parent_node,
    
    named_sibling_node: @named_sibling_node,

    named_generic_sibling_node: @named_generic_sibling_node,
    
    parent_node: @parent_node,

    sibling_node: @sibling_node,

    node_has_name: @node_has_name,

    node_has_class: @node_has_class,

    node_has_id: @node_has_id,

    node_has_attribute: @node_has_attribute,
    
    node_has_attribute_equal: @node_has_attribute_equal,
   
    node_has_attribute_dashmatch: @node_has_attribute_dashmatch,

    node_has_attribute_includes: @node_has_attribute_includes,

    node_has_attribute_prefix: @node_has_attribute_prefix,

    node_has_attribute_suffix: @node_has_attribute_suffix,

    node_has_attribute_substring: @node_has_attribute_substring,

    node_is_root: @node_is_root,
   
    node_count_siblings: @node_count_siblings,
    
    node_is_empty: @node_is_empty,
    
    node_is_link: @node_is_link,

    node_is_visited: @node_is_visited,

    node_is_hover: @node_is_hover,

    node_is_active: @node_is_active,

    node_is_focus: @node_is_focus,

    node_is_enabled: @node_is_enabled,

    node_is_disabled: @node_is_disabled,

    node_is_checked: @node_is_checked,
 
    node_is_target: @node_is_target,

    node_is_lang: @node_is_lang,

    node_presentational_hint: @node_presentational_hint,

    compute_font_size: @compute_font_size,
   
    ua_default_for_property: @ua_default_for_property,
    handler_version:1
};//TODO
    //testnum += 1;
    unsafe {
        let mut result = select.css_select_style(::cast::transmute(ctx.target.unwrap()),ctx.media as u64,None, select_handler,::cast::transmute(ctx));
        match result {
            (CSS_OK,Some(x)) => results = x,
            _=> fail!()
        }
    }

    assert!(results.styles[ctx.pseudo_element].is_some());
    dump_computed_style(results.styles[ctx.pseudo_element].unwrap(), &mut buf);
    let mut string:~str = copy ctx.exp;
    string = string.slice(0,ctx.explen).to_owned().to_lower();
    if 8192 - buf.len() !=  ctx.explen || str::eq(&buf.slice(0,ctx.explen).to_owned().to_lower(),&string) {
        io::println(fmt!("Expected : %?, %?",copy ctx.explen,string));
        io::println(fmt!("Result: %?,%?",8192-buf.len(),buf));
    }
    //css_select_ctx::css_select_results_destroy(&results);
    ctx.tree = None;
    ctx.current = None;
    ctx.depth = 0;
    //ctx->n_sheets = 0;
    ctx.sheets= ~[];
    ctx.target = None;

    
 }


 fn node_name(n:*libc::c_void, qname : &mut css_qname) -> css_error {

	let node : @mut node;
	unsafe {
		node = ::cast::transmute(n);
		qname.name = lwc_string_data((node.name).get_ref().clone());
	}

	CSS_OK
}

fn node_classes(pw:*libc::c_void, n:*libc::c_void, classes: &mut ~[~str] ) -> css_error{
	let mut node : @mut node;
	let mut lc : @mut line_ctx;
	unsafe {
		node = ::cast::transmute(n);
		lc = ::cast::transmute(pw);

		let mut i = 0;
		let n_attrs = node.attrs.len();
		while i < n_attrs {
			let mut matched = false;
			do lwc().write |l| {
				matched = l.lwc_string_caseless_isequal(node.attrs[i].name.clone(),lc.attr_class.clone()); 
			}
			
			if matched {break;}
			i += 1;
		}
		
		if i != n_attrs {
			classes.clear(); // as the next pushed val will be 1st elem.
			classes.push(lwc_string_data(node.attrs[i].name.clone()));
		}
		else {
			classes.clear();
		}
	}

	CSS_OK
}


fn node_id(pw:*libc::c_void, n:*libc::c_void, id:&mut ~str ) -> css_error{
	let mut node : @mut node;
	let mut lc : @mut line_ctx;
	unsafe {
		node = ::cast::transmute(n);
		lc = ::cast::transmute(pw);

		let mut i = 0;
		let n_attrs = node.attrs.len();
		while i < n_attrs {
			let mut matched = false;
			do lwc().write |l| {
				matched = l.lwc_string_caseless_isequal(node.attrs[i].name.clone(),lc.attr_class.clone()); 
			}

			if matched {break;}
			i += 1;
		}
		
		if i != n_attrs {
			*id = lwc_string_data(node.attrs[i].name.clone());
		}
		else {
			*id = ~"";
		}
	}

	CSS_OK
}

fn named_ancestor_node(n:*libc::c_void, qname:&mut css_qname, ancestor:*mut *libc::c_void) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	
	if node1.parent.is_none() {
		unsafe {
			*ancestor = ptr::null();
		}
		return CSS_OK;
	}
	while node1.parent.is_some() {
		node1 = node1.parent.unwrap();
		let matched:bool;
		unsafe {
			//matched = str::eq(&lwc_string_data(node1.name.get_ref().clone()).to_lower(),&qname.name.to_lower());
			matched = is_string_caseless_equal(lwc_string_data(node1.name.get_ref().clone()),qname.name);
		}
		if matched {
			break;
		}
	}
	unsafe {
		*ancestor =  ::cast::transmute(node1);
	}
	CSS_OK
}
   
fn named_parent_node(n:*libc::c_void, qname:&mut css_qname, parent:*mut*libc::c_void) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
		*parent = ptr::null();
	}	
	if node1.parent.is_some() {
		let matched: bool ;
		let parent_node : @mut node;
		unsafe {
			parent_node = node1.parent.unwrap();
			//matched = str::eq(&qname.name.to_lower(),&lwc_string_data(parent_node.name.get_ref().clone()).to_lower());
			matched = is_string_caseless_equal(qname.name,lwc_string_data(parent_node.name.get_ref().clone()));
		}
		if matched {
			unsafe {
				*parent = ::cast::transmute(parent_node);
			}
		}		
	}	
	CSS_OK
}
    
fn named_sibling_node(n:*libc::c_void, qname:&mut css_qname, sibling:*mut* libc::c_void) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
		*sibling = ptr::null();
	}	
	if node1.prev.is_some() {
		let matched: bool ;
		let prev_node: @mut node;
		unsafe {
			prev_node = *node1.prev.get_ref();
			//matched = str::eq(&qname.name.to_lower(),&lwc_string_data(prev_node.name.get_ref().clone()).to_lower());
			matched = is_string_caseless_equal(qname.name,lwc_string_data(prev_node.name.get_ref().clone()));
		}
		if matched {
			unsafe {
				*sibling = ::cast::transmute(prev_node);
			}
		}		
	}	
	CSS_OK
}

fn named_generic_sibling_node(n:*libc::c_void, qname:&mut css_qname, sibling:*mut*libc::c_void) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	if node1.prev.is_none() {
		unsafe {
			*sibling = ptr::null();
		}
		return CSS_OK;
	}
	
	while node1.prev.is_some() {
		node1 = node1.prev.unwrap();
		let matched:bool;
		unsafe {
			matched = is_string_caseless_equal(lwc_string_data(node1.name.get_ref().clone()),qname.name);
			//matched = str::eq(&lwc_string_data(node1.name.get_ref().clone()).to_lower(),&qname.name.to_lower());
		}
		if matched {
			break;
		}
	}
	unsafe {
		*sibling =  ::cast::transmute(node1);
	}
	CSS_OK
}
    
fn parent_node(n:*libc::c_void, parent:*mut*libc::c_void) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n); 
		
		*parent = ::cast::transmute(node1.parent.unwrap());
				
	}
	CSS_OK
}

fn sibling_node(n:*libc::c_void, sibling:*mut*libc::c_void) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
		
		*sibling = ::cast::transmute(node1.prev.unwrap());
			
	}
	CSS_OK
}

fn node_has_name(pw:*libc::c_void, n:*libc::c_void, qname:css_qname, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	if qname.name.len() == 1 && qname.name[0] == '*' as u8 {
		*matched = true;
	}
	else {
		unsafe {
			*matched = is_string_caseless_equal(lwc_string_data(node1.name.get_ref().clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.name.get_ref().clone()).to_lower(),&qname.name.to_lower());
		}		
	}
	CSS_OK
}

fn node_has_class(pw:*libc::c_void ,n:*libc::c_void, name:arc::RWARC<~lwc_string>, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	let mut ctx: @mut  line_ctx;
	let mut i:u32 = 0 ;
	unsafe {
		node1 = ::cast::transmute(n);
		ctx = ::cast::transmute(pw);
	}
	unsafe {
		
		while (i as uint) < node1.attrs.len() {
			let mut amatched: bool = false;
			do lwc().write |l| {
					amatched = l.lwc_string_caseless_isequal(node1.attrs[i].name.clone(),ctx.attr_class.clone()); 
				}
			if amatched {
				break;
			}
			i += 1;
		}

		/* Classes are case-sensitive in HTML */
		let mut condition_match : bool = false;
		do lwc().write |l| {
			condition_match = l.lwc_string_caseless_isequal(name.clone(), node1.attrs[i].value.clone());
		}
		
		if (i != node1.attrs.len() as u32) && condition_match {
			*matched = true;
		}
		else {
			*matched = false;
		}
	}
	CSS_OK
}

fn node_has_id(pw:*libc::c_void, n:*libc::c_void, name:arc::RWARC<~lwc_string>, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	let mut ctx: @mut  line_ctx;
	let mut i:u32 = 0 ;
	unsafe {
		node1 = ::cast::transmute(n);
		ctx = ::cast::transmute(pw);
	}
	unsafe {
		
		while (i as uint) < node1.attrs.len() {
			let mut amatched: bool = false;
			do lwc().write |l| {
					amatched = l.lwc_string_caseless_isequal(node1.attrs[i].name.clone(),ctx.attr_id.clone()); 
				}
			if amatched {
				break;
			}
			i += 1;
		}

		/* IDs are case-sensitive in HTML */
		let mut condition_match : bool = false;
		do lwc().write |l| {
			condition_match = l.lwc_string_caseless_isequal(name.clone(), node1.attrs[i].value.clone());
		}
		
		if i != (node1.attrs.len()as u32 ) && condition_match {
			*matched = true;
		}
		else {
			*matched = false;
		}
	}
	CSS_OK
}


fn node_has_attribute(n:*libc::c_void, qname:css_qname, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	let mut i:u32 = 0 ;
	//let mut vlen = value.len();
	*matched = false;
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()).to_lower(),&qname.name.to_lower());
			if *matched {
				break;
			}
			i += 1;
		}
	}
	CSS_OK
}
    

fn  node_has_attribute_equal(n:*libc::c_void, qname:css_qname,value:~str, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	let mut i:u32 = 0 ;
	
	*matched = false;
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()).to_lower(),&qname.name.to_lower());
			if *matched {
				break;
			}
			i += 1;
		}
	
		if *matched {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),value);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()).to_lower(),&value.to_lower());
		}
	}
	CSS_OK
}



fn node_has_attribute_includes(n:*libc::c_void, qname:css_qname,value:~str, matched:@mut bool) -> css_error {
	
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	
	let mut i:u32 = 0 ;
	let mut vlen = value.len();
	
	*matched = false;
	
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()),&qname.name);
			if *matched {
				break;
			}
			i += 1;
		}
	
		if *matched {
			let mut start = lwc_string_data(node1.attrs[i].value.clone());
			let mut start_len :uint = 0;
			let mut p:uint = 0;
			let end:uint = start.len();
			*matched =false;

			while p < end {
				if start[p] == ' ' as u8 {
					if (p - start_len == vlen) && 
					is_string_caseless_equal(start.slice(start_len,start_len + vlen).to_owned(),value) {
					//(str::eq(&start.slice(start_len,start_len + vlen).to_owned().to_lower(),&value.to_lower())) {
						*matched = true;
						break;
					}
					start_len = p + 1;	
				}
				p +=1;
			}
		}
	}
	CSS_OK
}


fn node_has_attribute_dashmatch(n:*libc::c_void, qname:css_qname,value:~str, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	let mut i:u32 = 0 ;
	let mut vlen = value.len();
	*matched = false;
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()),&qname.name);
			if *matched {
				break;
			}
			i += 1;
		}
	
		if *matched {
			let mut start = lwc_string_data(node1.attrs[i].value.clone());
			let mut start_len :uint = 0;
			let mut p:uint = 0;
			let end:uint = start.len();
			*matched =false;

			while p < end {
				if start[p] == '-' as u8 {
					if (p - start_len == vlen) && 
					is_string_caseless_equal(start.slice(start_len,start_len + vlen).to_owned(),value) {
					//(str::eq(&start.slice(start_len,start_len + vlen).to_owned().to_lower(),&value.to_lower())) {
						*matched = true;
						break;
					}
					start_len = p + 1;	
				}
				p +=1;
			}
		}
	}
	CSS_OK
}


fn node_has_attribute_prefix(n:*libc::c_void, qname:css_qname,value:~str, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	let mut i:u32 = 0 ;
	*matched = false;
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()),&qname.name);
			if *matched {
				break;
			}
			i += 1;
		}
	
		if *matched {
			let mut len = lwc_string_length(node1.attrs[i].value.clone());
			let mut data = lwc_string_data(node1.attrs[i].value.clone());
			let vlen = value.len();
			if len < vlen {
				*matched = false;
			}
			else {
				*matched = is_string_caseless_equal(data.slice(0, vlen).to_owned(),value);
				//*matched = str::eq(&data.slice(0, vlen).to_owned().to_lower(),&value.to_lower());
			}
		}
	}
	CSS_OK
}

fn node_has_attribute_suffix(n:*libc::c_void, qname:css_qname,value:~str, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	let mut i:u32 = 0 ;
	*matched = false;
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()),&qname.name);
			if *matched {
				break;
			}
			i += 1;
		}
	
		if *matched {
			let mut len = lwc_string_length(node1.attrs[i].value.clone());
			let mut data = lwc_string_data(node1.attrs[i].value.clone());
			let vlen = value.len();
			let suffix_start = len - vlen;
			if len < vlen {
				*matched = false;
			}
			else {
				*matched = is_string_caseless_equal(data.slice(suffix_start,suffix_start + vlen).to_owned(),value);
				//*matched = str::eq(&data.slice(suffix_start,suffix_start + vlen).to_owned().to_lower(),&value/*.slice(0,vlen).to_owned()*/.to_lower());
			}
		}
	}
	

	CSS_OK
}

fn node_has_attribute_substring(n:*libc::c_void, qname:css_qname,value:~str, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	let mut i:u32 = 0 ;
	*matched = false;
	unsafe {
		while (i as uint) < node1.attrs.len() {
			*matched = is_string_caseless_equal(lwc_string_data(node1.attrs[i].name.clone()),qname.name);
			//*matched = str::eq(&lwc_string_data(node1.attrs[i].name.clone()),&qname.name);
			if *matched {
				break;
			}
			i += 1;
		}
		if *matched {
			let mut len = lwc_string_length(node1.attrs[i].value.clone());
			let mut data = lwc_string_data(node1.attrs[i].value.clone());
			let vlen = value.len();
			let last_start_len = len -vlen;
			let last_start = data.slice(last_start_len,data.len()).to_owned();
			if len < vlen {
				*matched = false;
			}
			else {
				let mut iter:uint = 0;
				while iter < last_start_len {
					if is_string_caseless_equal(data.slice(iter,iter + vlen).to_owned(),value) {
					//if str::eq(&data.slice(iter,iter + vlen).to_owned().to_lower(),& value/*.slice(0,vlen).to_owned()*/.to_lower()) {
						*matched =true;
						break;
					}
					iter += 1;
				}
				if iter > last_start_len {
					*matched = false;
				}
			}
		}
	}
	CSS_OK
}

fn node_is_root(n:*libc::c_void, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	*matched = node1.parent.is_none();
	CSS_OK
}
   
fn node_count_siblings(n:*libc::c_void, same_name:bool, after:bool, count:@mut i32) -> css_error {
	let mut cnt : i32 = 0;
	let mut matched=  false;
	//*matched =false;
	let mut node1:@mut node;
	let mut name: arc::RWARC<~lwc_string> ;
	unsafe {
		node1 = ::cast::transmute(n);
		name = (node1.name).get_ref().clone();
	}
	
	if after {
		while node1.next.is_some() {
			if same_name {
				let mut next_name: arc::RWARC<~lwc_string> ;
				let mut temp_node = (copy node1.next).unwrap();
				unsafe {
					next_name = temp_node.name.get_ref().clone();
				}
				do lwc().write |l| {
					matched = l.lwc_string_caseless_isequal(name.clone(),next_name.clone()); 
				}
				if matched {
					cnt += 1;
				}
			}
			else {
				cnt += 1;
			}
			node1 = node1.next.unwrap();
		}
	}
	else {
		while node1.prev.is_some() {
			if same_name {
				let mut prev_name: arc::RWARC<~lwc_string> ;
				let mut temp_node = (copy node1.prev).unwrap();
				unsafe {
					prev_name = temp_node.name.get_ref().clone();
				}
				do lwc().write |l| {
					matched = l.lwc_string_caseless_isequal(name.clone(),prev_name.clone()); 
				}
				if matched {
					cnt += 1;
				}
			}
			else {
				cnt += 1;
			}
			node1 = node1.prev.unwrap();
		}
	}
	*count = cnt;
	CSS_OK
}
    
fn node_is_empty(n:*libc::c_void, matched:@mut bool) -> css_error {
	let mut node1:@mut node;
	unsafe {
		node1 = ::cast::transmute(n);
	}
	*matched = node1.children.is_none();
	CSS_OK
}
    
fn node_is_link(_:*libc::c_void, matched:@mut bool) -> css_error {
	
	*matched = false;
	CSS_OK
}

fn node_is_visited(_:*libc::c_void, matched:@mut bool) -> css_error {
	
	*matched = false;
	CSS_OK
}

fn node_is_hover(_:*libc::c_void, matched:@mut bool) -> css_error {
	
	*matched = false;
	CSS_OK
}

fn node_is_active(_:*libc::c_void, matched:@mut bool) -> css_error {
	
	*matched = false;
	CSS_OK
}

fn node_is_focus(_:*libc::c_void, matched:@mut bool) -> css_error {
	
	*matched = false;
	CSS_OK
}

fn node_is_enabled(_:*libc::c_void, matched:@mut bool) -> css_error {
	
	*matched = false;
	CSS_OK
}

fn node_is_disabled(_:*libc::c_void, matched:@mut bool) -> css_error {
	*matched = false;
	CSS_OK
}

fn node_is_checked(_:*libc::c_void, matched:@mut bool) -> css_error {
	*matched = false;
	CSS_OK
}
 
fn node_is_target(_:*libc::c_void, matched:@mut bool) -> css_error {
	*matched = false;
	CSS_OK
}

fn node_is_lang(_:*libc::c_void, _:~str, matched:@mut bool) -> css_error {
	*matched = false;
	CSS_OK
}

fn node_presentational_hint(_:*libc::c_void, _:u32) -> (css_error,Option<@mut css_hint>) {
	(CSS_PROPERTY_NOT_SET,None)
}

fn ua_default_for_property(property:u32, hint:@mut css_hint ) -> css_error {
	
	if property == CSS_PROP_COLOR as u32 {
		hint.color = Some(0xff000000);
		hint.status = CSS_COLOR_COLOR as u8;
	}
	else if property == CSS_PROP_FONT_FAMILY as u32 {
		hint.strings = None;
		hint.status = CSS_FONT_FAMILY_SANS_SERIF as u8;
	}
	else if property == CSS_PROP_QUOTES as u32 {
		/* Not exactly useful :) */
		hint.strings = None;
		hint.status = CSS_QUOTES_NONE as u8;
	}
	else if property == CSS_PROP_VOICE_FAMILY as u32 {
		// \todo Fix this when we have voice-family done 
		hint.strings = None;
		hint.status = 0;
	}
	else {
		return CSS_INVALID;
	}
	CSS_OK
}


fn compute_font_size(parent: Option<@mut css_hint>, size: Option<@mut css_hint>) -> css_error {
	let mut parent_value:@mut css_hint;
	let mut size_val : @mut css_hint;
	let mut sizes:~[@mut css_hint_length] =
	 	~[
			@mut css_hint_length{value:FLTTOFIX(6.75),unit:CSS_UNIT_PT},
	   		@mut css_hint_length{value:FLTTOFIX(7.50),unit:CSS_UNIT_PT},
	    	@mut css_hint_length{value:FLTTOFIX(9.75),unit:CSS_UNIT_PT},
	    	@mut css_hint_length{value:FLTTOFIX(12.0),unit:CSS_UNIT_PT},
	    	@mut css_hint_length{value:FLTTOFIX(13.5),unit:CSS_UNIT_PT},
	    	@mut css_hint_length{value:FLTTOFIX(18.0),unit:CSS_UNIT_PT},
	    	@mut css_hint_length{value:FLTTOFIX(24.0),unit:CSS_UNIT_PT}
		];
	  	let parent_size: @mut css_hint_length;
	  	
	  	/* Grab parent size, defaulting to medium if none */
	  	if parent.is_none() {
	  		parent_size = sizes[CSS_FONT_SIZE_MEDIUM as uint- 1];
	  	}
	  	else {
	  		parent_value = *parent.get_ref();
	  		assert!(parent_value.status == CSS_FONT_SIZE_DIMENSION as u8);
	  		assert!( match parent_value.length.unwrap().unit {
	  			CSS_UNIT_EM |
	  			CSS_UNIT_EX=> false,
	  			_=> true
	  		});	   
	  		parent_size = parent_value.length.unwrap();
	  	}

	  	if size.is_none() {
	  		fail!();
	  	}
	  	else {
	  		size_val = size.unwrap();
	  	}

	  	assert!(size_val.status != CSS_FONT_SIZE_INHERIT as u8);

	  	if size_val.status < CSS_FONT_SIZE_LARGER as u8 {
	  		/* Keyword -- simple */
	  		size_val.length = Some(sizes[size_val.status -1]);
	  	}
	  	else if size_val.status == CSS_FONT_SIZE_LARGER as u8 {
	  		// \todo Step within table, if appropriate 
	  		size_val.length.unwrap().value = css_multiply_fixed(parent_size.value, FLTTOFIX(1.2) );
	  		size_val.length.unwrap().unit = parent_size.unit;
	  	}
	  	else if size_val.status == CSS_FONT_SIZE_SMALLER as u8 {
	  		// \todo Step within table, if appropriate 
	  		size_val.length.unwrap().value = css_multiply_fixed(parent_size.value, FLTTOFIX(1.2) );
	  		size_val.length.unwrap().unit = parent_size.unit;
	  	}
	  	else if (
	  				match size_val.length.unwrap().unit {
	  					CSS_UNIT_EM |
	  					CSS_UNIT_EX => true,
	  					_=> false
	  				}
	  			) {
	  			
  			size_val.length.unwrap().value = css_multiply_fixed(size_val.length.unwrap().value,parent_size.value);
  			if (
  				match size_val.length.unwrap().unit {
  					CSS_UNIT_EX => true,
  					_=> false
  				}
  				) {
  					size_val.length.unwrap().value = css_multiply_fixed(size_val.length.unwrap().value,FLTTOFIX(0.6));
  			}
  			size_val.length.unwrap().unit = parent_size.unit;
	  	}
	  	else if (
	  				match size_val.length.unwrap().unit {
	  					CSS_UNIT_PCT => true,
	  					_=> false
	  				}
	  			) {
	  		
			size_val.length.unwrap().value = css_divide_fixed(css_multiply_fixed(size_val.length.unwrap().value,parent_size.value),FLTTOFIX(100.0));	
			size_val.length.unwrap().unit = parent_size.unit;
	  	}
	size_val.status = CSS_FONT_SIZE_DIMENSION as u8;
	CSS_OK
}


pub fn is_string_caseless_equal(a : &str , b : &str ) -> bool {

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


#[test]
fn selection_test() {

	select_test(~"data/select/tests1.dat");
}





