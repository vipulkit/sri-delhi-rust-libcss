extern mod extra;
extern mod std;
extern mod css;
extern mod wapcaplet;
extern mod dumpcomputed;
// extern mod dump2;

use css::css::*;
use wapcaplet::*;
use std::cast;
use std::libc;
use std::ptr;
use std::str;

use css::include::types::*;
use css::utils::errors::*;
use css::select::common::*;
use css::stylesheet::*;
use css::select::select::*;
use dumpcomputed::*;
use css::parse::propstrings::*;
// use dump2::dump_sheet;

use css::include::properties::*;
use css::include::fpmath::*;

use extra::time;
use std::io;

pub struct attribute {
    name:uint,
    value:uint
}

pub struct node {
    name:Option<uint>,
    attrs:~[attribute],

    parent:Option<@mut node>,
    next:Option<@mut node>,
    prev:Option<@mut node>,
    children:Option<@mut node>,
    last_child:Option<@mut node>
}

pub struct sheet_ctx {
    sheet:~css,
    origin:css_origin,
    media:u64
}

pub struct ctx_pw {
    attr_class:uint,
    attr_id:uint
}   

pub struct line_ctx {
    //explen:uint,
    //expused:uint,
    exp:~str,

    intree:bool,
    insheet:bool,
    inerrors:bool,
    inexp:bool,

    tree:Option<@mut node>,
    current:Option<@mut node>,
    depth:u32,

    sheets:~[@mut sheet_ctx],

    media:u64,
    pseudo_element:u32,
    target:Option<@mut node>,
    
    attr_class:uint,
    attr_id:uint,

    // propstrings_instance: Option<~css_propstrings>
} 

pub fn select_test(file:~str) {
    
    let mut lwc_ref = lwc();
    let mut lwc_attr_class : Option<uint>;
    let mut lwc_attr_id : Option<uint>;

    lwc_attr_class = Some(lwc_ref.lwc_intern_string(&"class"));
    lwc_attr_id = Some(lwc_ref.lwc_intern_string(&"id"));
    let propstring = css_propstrings::css_propstrings(&mut lwc_ref);
    let ctx : @mut line_ctx = @mut line_ctx{
        //explen:0,
        //expused:0,
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
        
        attr_class:lwc_attr_class.take_unwrap(),
        attr_id:lwc_attr_id.take_unwrap(),
        
    };

    let file_content_result = io::read_whole_file_str(&Path(file)) ;
    let mut file_content : ~str ;
    match file_content_result {
        Ok(x) => {
            file_content = x ;
        },
        Err(_) => {
            file_content = ~"" ;
            // debug!("\n Error opening file ===============:%?",y);
            assert!(false) ;
        }
    }

    let css_stylesheet_create_time = @mut 0;
    let css_stylesheet_append_data_time = @mut 0;
    let css_select_style_time = @mut 0;
    let css_stylesheet_data_done_time= @mut 0f;

    for file_content.any_line_iter().advance |line| { 
        let mut line_string: ~str = line.to_str(); 
        line_string.push_char('\n');
        // debug!("Handling line =%?=",copy line_string);
        handle_line(&mut line_string, &mut lwc_ref, &propstring , ctx, css_stylesheet_create_time, 
                css_stylesheet_append_data_time, 
                css_select_style_time, 
                css_stylesheet_data_done_time  );

    }   

    if (ctx.tree.is_some() ) {
        run_test(ctx, lwc_ref, css_select_style_time);

    }

    io::println(fmt!("#css_stylesheet_create_time:%?",(*css_stylesheet_create_time as float /1000f))) ;
    io::println(fmt!("#css_stylesheet_append_data_time:%?",(*css_stylesheet_append_data_time as float/1000f))) ;
    io::println(fmt!("#css_stylesheet_data_done_time:%?",(*css_stylesheet_data_done_time/1000f))) ;
    io::println(fmt!("#css_select_style_time:%?",(*css_select_style_time as float /1000f))) ;
}

pub fn resolve_url(_:&str, rel:uint) -> (css_error,Option<uint>){

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
        font : None,
    };
     return css_param;
}

pub fn handle_line(data:&mut ~str, lwc_ref:&mut ~lwc, propstring_ref: &css_propstrings , ctx:@mut line_ctx, css_stylesheet_create_time:@mut u64, 
    css_stylesheet_append_data_time:@mut u64, 
    css_select_style_time:@mut u64, 
    css_stylesheet_data_done_time:@mut float
    ) -> bool 
{
    let mut error : css_error ;
    let mut len : uint ; 
    if ( data[0] == ('#' as u8) ) {
        // debug!("# encountered ");
        if( ctx.intree ) {
            // debug!("ctx intree");
            if( data.len() >= 7 && is_string_caseless_equal(*data,1, "errors",6 )){
                ctx.intree = false;
                ctx.insheet = false;
                ctx.inerrors = true ;
                ctx.inexp = false;
            }
            else {
                /* Assume start of stylesheet */
                css__parse_sheet(ctx,  data,1, css_stylesheet_create_time );
                // debug!("Sheet parsed 1");
                ctx.intree = false;
                ctx.insheet = true;
                ctx.inerrors = false;
                ctx.inexp = false;
            }
        }
        else if (ctx.insheet) {
            // debug!("ctx insheet");
            if(data.len() >= 7 && is_string_caseless_equal(*data,1, "errors",6)){
                len = ctx.sheets.len() -1;
                let start_time = time::precise_time_ns();
                assert!( 
                        match ctx.sheets[len].sheet.css_stylesheet_data_done(lwc_ref , propstring_ref) {
                                CSS_OK=>{true},
                                _=>{false}
                        });
                let end_time = time::precise_time_ns();
            let css_style_diff_time = (end_time as float - start_time as float);
            *css_stylesheet_data_done_time += css_style_diff_time;
                ctx.intree = false;
                ctx.insheet = false;
                ctx.inerrors = true ;
                ctx.inexp = false;
            }
            else if data.len() >= 3 && is_string_caseless_equal(*data,1, "ua", 2) ||
                        data.len() >= 5 && is_string_caseless_equal(*data,1, "user", 4) ||
                        data.len() >= 7 && is_string_caseless_equal(*data,1, "author", 6) {
                
                len = ctx.sheets.len() -1;
                let start_time = time::precise_time_ns();
                assert!( 
                        match ctx.sheets[len].sheet.css_stylesheet_data_done(lwc_ref , propstring_ref) {
                            CSS_OK=>{true},
                            _=>{false}
                        });
                let end_time = time::precise_time_ns();
            let css_style_diff_time = (end_time as float - start_time as float);
            *css_stylesheet_data_done_time += css_style_diff_time;
                css__parse_sheet(ctx,  data,1, css_stylesheet_create_time);
                // debug!("Sheet parsed 2");
            }
            else {
                len = ctx.sheets.len() -1;
                let start_time = time::precise_time_ns();
                let error = ctx.sheets[len].sheet.css_stylesheet_append_data(lwc_ref , propstring_ref , data.as_bytes().to_owned());
                let end_time = time::precise_time_ns();
                *css_stylesheet_append_data_time += (end_time - start_time);

                assert!( match error {
                            CSS_OK=>{true},
                            CSS_NEEDDATA=>{true},
                            _=>{false}
                         });
            }
        }
        else if (ctx.inerrors) {
            // debug!("in ctx errors");
            ctx.intree = false;
            ctx.insheet = false;
            ctx.inerrors = false;
            ctx.inexp = true;
        }
        else if (ctx.inexp) {
            // debug!("in ctx inexp");
            /* This marks end of testcase, so run it */
            run_test(ctx, lwc_ref.clone(), css_select_style_time);
            //ctx.expused = 0;

            ctx.intree = false;
            ctx.insheet = false;
            ctx.inerrors = false;
            ctx.inexp = false;
        }
        else {
            /* Start state */
            // debug!("in ctx tree ==== ");
            if(data.len()>=5 && is_string_caseless_equal(*data,1, "tree", 4)) {
                // debug!("entering for parse tree");
                css__parse_tree(ctx, data, 5 );
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
            css__parse_tree_data(ctx, lwc_ref, data.slice(1,data.len()-1) );
        }
        else if ( ctx.insheet ) {
            len = ctx.sheets.len() -1;
            let start_time = time::precise_time_ns();
            error = ctx.sheets[len].sheet.css_stylesheet_append_data(lwc_ref , propstring_ref , data.as_bytes().to_owned());
            let end_time = time::precise_time_ns();
            *css_stylesheet_append_data_time += (end_time - start_time);
            assert!( match error {
                        CSS_OK=>{true},
                        CSS_NEEDDATA=>{true},
                        _=>{false}
                    });
        }
        else if (ctx.inexp) {
            css__parse_expected(ctx, data.slice(0,data.len()) );
        }
    }
    true 
}

fn css__parse_expected(ctx: @mut line_ctx , data: &str) {

    ctx.exp = ctx.exp + data;
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

pub fn css__parse_tree(ctx:@mut line_ctx, data:&mut ~str, index:uint) {

    // debug!("\n Entering css__parse_tree ") ;
    let mut p = index;
    let end = data.len() ;
    //size_t left;

    /* [ <media_list> <pseudo>? ] ? */

    ctx.media = CSS_MEDIA_ALL as u64;
    ctx.pseudo_element = CSS_PSEUDO_ELEMENT_NONE as u32;

    /* Consume any leading whitespace */
    while ( (data[p]==0x20) || (data[p]==0x09) || (data[p]==0x0a) || 
         (data[p]==0x0b) || (data[p]==0x0c) || (data[p]==0x0d) ) && (p<end) {
        //debug!("Entering: while {...} 1");
        p += 1;
    }

    if (p < end) {
        //left = end - p;

        p = css__parse_media_list(data,p, &mut (ctx.media));

        //end = p + left;
    }

    if (p < end) {
        //left = end - p;

        css__parse_pseudo_list(data , p ,ctx);
    }
}

pub fn css__parse_tree_data(ctx:@mut line_ctx, lwc_ref:&mut ~lwc, data:&str) {
    
    // debug!("\n Entering css__parse_tree_data ") ;
    let mut p = 0;
    let end = data.len();

    let mut value = None;
    let mut namelen = 0;
    let mut valuelen = 0;
    let mut depth:u32 = 0;
    let mut target = false;

    /* ' '{depth+1} [ <element> '*'? | <attr> ]
     * 
     * <element> ::= [^=*[:space:]]+
     * <attr>    ::= [^=*[:space:]]+ '=' [^[:space:]]*
     */
     //debug!("\n Before while  ") ;
    while (p < end && isspace(data[p])) {
        depth += 1;
        p += 1;
    }
    depth -= 1;

    //debug!("\n Before attribute name  ") ;
    /* Get element/attribute name */
    let name_begin = p;
    while ( (p < end) && (data[p] != '=' as u8) && (data[p] != '*' as u8)  && (isspace(data[p]) == false) ){
        namelen += 1;
        p += 1;
    }

    let name = lwc_ref.lwc_intern_string(data.slice(name_begin,name_begin+namelen));

    //debug!("\n Before while  2") ;
    /* Skip whitespace */
    while (p < end && isspace(data[p])){
        p += 1;
    }
    
    let mut value_begin = 0;

    //debug!("\n Before attribute value  ") ;
    if (p < end && (data[p] == ('=' as u8)) ) {
        /* Attribute value */
        p += 1;

        value_begin = p;

        while (p < end && isspace(data[p]) == false) {
            valuelen += 1;
            p += 1;
        }
    } else if (p < end && (data[p] == ('*' as u8)) ) {
        /* Element is target node */
        target = true;
    }

    //debug!("\n Before 3  ") ;
    if valuelen > 0 {
        value = Some(lwc_ref.lwc_intern_string(data.slice(value_begin, value_begin+valuelen)));
    }

    //debug!("\n Before 4  ") ;
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
            
        n.name = Some(name);
            

        /* Insert it into tree */
        if ctx.tree.is_none() {
            ctx.tree = Some(n);
        } 
        else {
            assert!(depth > 0);
            assert!(depth <= ctx.depth + 1);

            //debug!("\n Before while  3") ;
            /* Find node to insert into */
            while (depth <= ctx.depth) {
                ctx.depth -= 1;
                ctx.current = ctx.current.get().parent;
            }
            //let ctx_current = ctx.current.get();  
            //debug!("\n Before insert into current node  ") ;
            /* Insert into current node */
            if (ctx.current.get().children.is_none()) {
                //debug!("\n Before insert into current node == if statement ") ;
                ctx.current.get().children = Some(n);
                ctx.current.get().last_child = Some(n);
            } else {
                //debug!("\n Before insert into current node == else statement ");
                ctx.current.get().last_child.get().next = Some(n);
                //debug!("\n Before insert into current node == else statement 2") ;
                n.prev = ctx.current.get().last_child;
                //debug!("\n Before insert into current node == else statement 3") ;
                ctx.current.get().last_child = Some(n);
            }
            //debug!("\n Before final updation  ") ;
            ctx.current = Some(ctx.current.get());  
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
        // debug!("\n Before else  ");
        let attr: attribute = attribute{
            name:name,
            value:value.unwrap()
        };

        ctx.current.unwrap().attrs.push(attr);

    }

}

pub fn css__parse_sheet(ctx:@mut line_ctx , data:&mut ~str,index:uint, css_stylesheet_create_time:@mut u64){
    
    // debug!("\n Entering css__parse_sheet ") ;
    let mut origin : css_origin = CSS_ORIGIN_AUTHOR;
    let mut media = CSS_MEDIA_ALL as u64; 
    let mut p : uint = index;
    let end : uint = data.len();
    /* Find end of origin */
    while p < end && !isspace(data[p]) {
        p += 1;
    }
    
    if p-index == 6 && is_string_caseless_equal(*data,index, "author", 6){
        origin = CSS_ORIGIN_AUTHOR;
    }
    else if p-index == 4 && is_string_caseless_equal(*data,index, "user", 4){
        origin = CSS_ORIGIN_USER;
    }
    else if p-index == 2 && is_string_caseless_equal(*data,index, "ua", 2){
        origin = CSS_ORIGIN_UA;
    }
    else {
        // debug!("Unknown stylesheet origin");
        assert!(false);
    }
    
    /* Skip any whitespace */
    while p < end && isspace(data[p]) {
        p += 1;
    }
    
    if p < end {
       css__parse_media_list(data,p,&mut media);
    }
    let params = css_create_params();
    // let lwc_ins = ctx.lwc_instance;
    
    let start_time = time::precise_time_ns();
    let sheet: ~css = css::css_create(&params);
    let end_time = time::precise_time_ns();
    *css_stylesheet_create_time += (end_time - start_time);

 

    // debug!("Sheet created in select-auto ");
    let sheet_ctx_ins = @mut sheet_ctx {
        sheet: sheet,
        origin: origin,
        media: media
    };
    // debug!("Before pushing Sheet ");
    ctx.sheets.push(sheet_ctx_ins) ;
    // debug!("Sheet pushed in select-auto ");
}

#[inline]
pub fn css__parse_media_list(data:&mut ~str ,index:uint, media : &mut u64) -> uint {
    // debug!("\n Entering css__parse_media_list =%?=%?=",data,index) ;
    // ' '  (0x20)  space (SPC)
    // '\t' (0x09)  horizontal tab (TAB)
    // '\n' (0x0a)  newline (LF)
    // '\v' (0x0b)  vertical tab (VT)
    // '\f' (0x0c)  feed (FF)
    // '\r' (0x0d)  carriage return (CR)
    let mut len : uint = index ;
    let mut result : u64 = 0 ;
    while len < data.len() {
        let start = len ;
        /* consume a medium */
        while data.len()>len && !((data[len]==0x20) || (data[len]==0x09) ||  (data[len]==0x0a) || 
             (data[len]==0x0b) ||  (data[len]==0x0c) ||  (data[len]==0x0d))   {

            if( data[len]!= (',' as u8)) {
                len += 1;
                loop ;
            }
            else {
                break ;
            }
        }

        // debug!("\n slice left is =%?=%?=%?=%?=",copy data.slice(start,data.len()),len,start,data.len() ) ;

        if ( (len-start)==10 && is_string_caseless_equal(*data,start, "projection", 10) ) {
            result = result | (CSS_MEDIA_PROJECTION as u64) ;
        }
        else if ( (len-start)==8 && is_string_caseless_equal(*data,start, "handheld", 8) ) {
            result = result | (CSS_MEDIA_HANDHELD as u64) ;
        }
        else if ( (len-start)==8 && is_string_caseless_equal(*data,start, "embossed", 8) ) {
            result = result | (CSS_MEDIA_EMBOSSED as u64) ;
        }
        else if ( (len-start)==7 && is_string_caseless_equal(*data,start, "braille", 7) ) {
            result = result | (CSS_MEDIA_BRAILLE as u64) ;
        }
        else if ( (len-start)==6 && is_string_caseless_equal(*data,start, "speech", 6) ) {
            result = result | (CSS_MEDIA_SPEECH as u64) ;
        }
        else if ( (len-start)==6 && is_string_caseless_equal(*data,start, "screen", 6) ) {
            result = result | (CSS_MEDIA_SCREEN as u64) ;
        }
        else if ( (len-start)==5 && is_string_caseless_equal(*data,start, "print", 5) ) {
            result = result | (CSS_MEDIA_PRINT as u64) ;
        }
        else if ( (len-start)==5 && is_string_caseless_equal(*data,start, "aural", 5) ) {
            result = result | (CSS_MEDIA_AURAL as u64) ;
        }
        else if ( (len-start)==3 && is_string_caseless_equal(*data,start, "tty", 3) ) {
            result = result | (CSS_MEDIA_TTY as u64) ;
        }
        else if ( (len-start)==3 && is_string_caseless_equal(*data,start, "all", 3) ) {
            result = result | (CSS_MEDIA_ALL as u64) ;
        }
        else if ( (len-start)==2 && is_string_caseless_equal(*data,start, "tv", 2) ) {
            result = result | (CSS_MEDIA_TV as u64) ;
        }
        else {
            // unknown media type
            // debug!("\n Unknown Media type encountered");
            assert!(false);
        }

        /* Consume whitespace */
        while data.len()>len && ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
        (data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) )  {
        
            len += 1;
        }

        /* Stop if we've reached the end */
        if ( data.len() <= len ) ||  (data[len] != (',' as u8) ) {
            break;
        }

        /* Consume comma */
        len += 1;

        /* Consume whitespace */
        while data.len()>len && ( (data[len]==0x20) || (data[len]==0x09) || (data[len]==0x0a) || 
        (data[len]==0x0b) || (data[len]==0x0c) || (data[len]==0x0d) )  {
            len += 1;
        }   
    }
    
    *media = result ;
    len
}

pub fn css__parse_pseudo_list(data:&mut ~str, index:uint,ctx:@mut line_ctx) -> uint {
    
    // debug!("\n Entering css__parse_pseudo_list ") ;
    let string = data.slice(index, data.len()).to_owned();
    *data = data.slice(0,index).to_owned();

    let mut p:uint = 0;
    let end:uint = string.len();

    /* <pseudo> [ ',' <pseudo> ]* */

    ctx.pseudo_element = CSS_PSEUDO_ELEMENT_NONE as u32;


    while p < end {
        let start:uint = p;

        /* consume a pseudo */
        while string[p] != ' ' as u8  && string[p] != ',' as u8 {
            p += 1;
        }

        /* Pseudo elements */
        if p - start == 12 && is_string_caseless_equal(string,start,&"first-letter", 12) {
            ctx.pseudo_element = CSS_PSEUDO_ELEMENT_FIRST_LETTER as u32;
        }
        else if p - start == 10 && is_string_caseless_equal(string,start,&"first-line", 10) {
            ctx.pseudo_element = CSS_PSEUDO_ELEMENT_FIRST_LINE as u32;
        }
        else if p - start == 6 && is_string_caseless_equal(string,start,&"before", 6) {
            ctx.pseudo_element = CSS_PSEUDO_ELEMENT_BEFORE as u32;
        }
        else if p - start == 5 && is_string_caseless_equal(string,start,&"after", 5) {
            ctx.pseudo_element = CSS_PSEUDO_ELEMENT_AFTER as u32;
        }
        else {
            fail!(~"Unknown pseudo");
        }

        /* Consume whitespace */
        while p < end || string[p] == ' ' as u8 {
            p += 1;
        }

        /* Stop if we've reached the end */
        if p == end || string[p] != ',' as u8 {
            break;
        }

        /* Consume comma */
        p += 1;

        /* Consume whitespace */
        while p < end && string[p] == ' ' as u8 {
            p += 1;
        }

    }
    *data = *data + string.slice(p,/*string.len()*/end).to_owned();
    
    0
}

fn to_lower(string:&str) -> ~str {
    let mut lower : ~[u8] = ~[];
    for string.bytes_iter().advance |c| {
        lower.push(lwc::dolower(c));
    }
    lower.push(0);
    str::from_bytes(lower)
}

pub fn run_test( ctx:@mut line_ctx, lwc_ref:~lwc, css_select_style_time:@mut u64) {
    //debug!("\n Entering run test =%?=",ctx) ;
    let mut select: ~css_select_ctx;
    let mut results: ~css_select_results;

    let mut i:u32=0;
    let mut buf:~str= ~"";
 
    select = css_select_ctx::css_select_ctx_create(lwc_ref);

    while i < (ctx.sheets.len() as u32) {
        // let ds_sheet = dump_sheet(ctx.sheets[i].sheet.stylesheet);
        // debug!("\n=================================================");
        // debug!("Dumpping Stylesheet before appending to selector");
        // debug!("%?",ds_sheet);
        // debug!("=================================================\n");

        match select.css_select_ctx_append_sheet(ctx.sheets[i].sheet.stylesheet,ctx.sheets[i].origin,ctx.sheets[i].media) {
            CSS_OK => {},
            _ => fail!()
        }
        i += 1;
    }
    let select_handler:  @mut css_select_handler = @mut css_select_handler {
        node_name: node_name,

        node_classes: node_classes,

        node_id: node_id,

        named_ancestor_node: named_ancestor_node,
       
        named_parent_node: named_parent_node,
        
        named_sibling_node: named_sibling_node,

        named_generic_sibling_node: named_generic_sibling_node,
        
        parent_node: parent_node,

        sibling_node: sibling_node,

        node_has_name: node_has_name,


        node_has_class: node_has_class,

        node_has_id: node_has_id,

        node_has_attribute: node_has_attribute,
        
        node_has_attribute_equal: node_has_attribute_equal,
       
        node_has_attribute_dashmatch: node_has_attribute_dashmatch,

        node_has_attribute_includes: node_has_attribute_includes,

        node_has_attribute_prefix: node_has_attribute_prefix,

        node_has_attribute_suffix: node_has_attribute_suffix,

        node_has_attribute_substring: node_has_attribute_substring,

        node_is_root: node_is_root,
       
        node_count_siblings: node_count_siblings,
        
        node_is_empty: node_is_empty,
        
        node_is_link: node_is_link,

        node_is_visited: node_is_visited,

        node_is_hover: node_is_hover,

        node_is_active: node_is_active,

        node_is_focus: node_is_focus,

        node_is_enabled: node_is_enabled,

        node_is_disabled: node_is_disabled,

        node_is_checked: node_is_checked,
     
        node_is_target: node_is_target,

        node_is_lang: node_is_lang,

        node_presentational_hint: node_presentational_hint,

        compute_font_size: compute_font_size,
       
        ua_default_for_property: ua_default_for_property,
        handler_version:1
    };
        
    unsafe {
        let pw = @mut ctx_pw{attr_class:ctx.attr_class, attr_id:ctx.attr_id};
        // debug!(fmt!("pw=%?",pw));
        let target = cast::transmute(ctx.target.unwrap());
        let pw_ptr = ::cast::transmute(pw);

        let start_time = time::precise_time_ns();
        let result = select.css_select_style(target,ctx.media as u64,None, select_handler,pw_ptr);
        let end_time = time::precise_time_ns();

        *css_select_style_time += (end_time - start_time);

        match result {
            (CSS_OK,Some(x)) => results = x,
               _=> fail!(~"During css_select_style in select-auto")
        }
    }

    assert!(results.styles[ctx.pseudo_element].is_some());
    dump_computed_style(results.styles[ctx.pseudo_element].unwrap(),  &mut select.lwc_ref, &mut buf);


    // debug!(fmt!(" CSS Selection result is =%?",results));
    let string:~str = ctx.exp.clone();
    // debug!(fmt!("Expected : %s ",string));
    // debug!(fmt!("Result: %s",buf));

    if !str::eq(&to_lower(buf), &to_lower(string)) {
        fail!(~"Select result mismatched with expected");
    }
    // else {
    //     // debug!("Result: Test case passed"); 
    // }
    ctx.exp = ~"";
    ctx.tree = None;
    ctx.current = None;
    ctx.depth = 0;
    ctx.sheets= ~[];
    ctx.target = None;
 }

 #[inline] 
fn node_name(n:*libc::c_void, qname : &mut css_qname) -> css_error {

    let node : @mut node;
    unsafe {
        node = ::cast::transmute(n);
        cast::forget(node);
        qname.name = node.name.get();
    }

    CSS_OK
}

#[inline] 
fn node_classes(lwc_ref:&mut ~lwc, pw:*libc::c_void, n:*libc::c_void, classes: &mut ~[uint] ) -> css_error{
    // debug!("node_classes");
    let mut node : @mut node;
    let mut lc : @mut ctx_pw;
    //println("I am in node_classes");
	unsafe {
        node = ::cast::transmute(n);
        cast::forget(node);
        lc = ::cast::transmute(pw);
        cast::forget(lc);
		classes.clear();
        
		let mut i = 0;
        let n_attrs = node.attrs.len();
        while i < n_attrs {
			let mut matched;
            matched = lwc_ref.lwc_string_caseless_isequal(node.attrs[i].name,lc.attr_class); 
           
            if matched {
				classes.push(node.attrs[i].value);
				break;
			}
			i += 1;
        }
    }
    CSS_OK
}

#[inline] 
fn node_id(lwc_ref:&mut ~lwc, pw:*libc::c_void, n:*libc::c_void, id: &mut uint ) -> css_error{
    // debug!("node_id");
    let mut node : @mut node;
    let mut lc : @mut ctx_pw;
    //println("I am in node_id");
	unsafe {
        node = ::cast::transmute(n);
        cast::forget(node);
        lc = ::cast::transmute(pw);
        cast::forget(lc);
        *id =lwc_ref.lwc_intern_string("");
		
        let mut i = 0;
        let n_attrs = node.attrs.len();
        while i < n_attrs {
			let mut matched;
            matched = lwc_ref.lwc_string_caseless_isequal(node.attrs[i].name,lc.attr_id); 

            if matched {
				*id = node.attrs[i].value;
				break;
			}
			i += 1;
        }
    }

    CSS_OK
}

#[inline] 
fn named_ancestor_node(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&mut css_qname, ancestor:*mut *libc::c_void) -> css_error {
    // debug!("named_ancestor_node");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
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
        matched = lwc_ref.lwc_string_caseless_isequal(node1.name.get(),qname.name);
        if matched {
            break;
        }
    }
    unsafe {
        *ancestor =  ::cast::transmute(node1);
    }
    CSS_OK
}

#[inline]   
fn named_parent_node(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&mut css_qname, parent:*mut*libc::c_void) -> css_error {
    // debug!("named_parent_node");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
        *parent = ptr::null();
    }   
    if node1.parent.is_some() {
        let matched: bool ;
        let parent_node : @mut node;
        parent_node = node1.parent.unwrap();
        matched = lwc_ref.lwc_string_caseless_isequal(qname.name,parent_node.name.get());
        if matched {
            unsafe {
                *parent = ::cast::transmute(parent_node);
            }
        }       
    }   
    CSS_OK
}

#[inline]    
fn named_sibling_node(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&mut css_qname, sibling:*mut* libc::c_void) -> css_error {
    // debug!("named_sibling_node");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
        *sibling = ptr::null();
    }   
    if node1.prev.is_some() {
        let matched: bool ;
        let prev_node: @mut node;
        prev_node = node1.prev.get();
        matched = lwc_ref.lwc_string_caseless_isequal(qname.name,prev_node.name.get());
        if matched {
            unsafe {
                *sibling = ::cast::transmute(prev_node);
            }
        }       
    }   
    CSS_OK
}

#[inline] 
fn named_generic_sibling_node(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&mut css_qname, sibling:*mut*libc::c_void) -> css_error {
    // debug!("named_generic_sibling_node");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
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
        matched = lwc_ref.lwc_string_caseless_isequal(node1.name.get(),qname.name);
        if matched {
            break;
        }
    }
    unsafe {
        *sibling =  ::cast::transmute(node1);
    }
    CSS_OK
}

#[inline]    
fn parent_node(n:*libc::c_void, parent:*mut*libc::c_void) -> css_error {
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n); 
        cast::forget(node1);
        
        if node1.parent.is_some() {
            *parent = ::cast::transmute(node1.parent.unwrap());
        }
        else {
            *parent = ptr::null();
        }
    }
    CSS_OK
}

#[inline] 
fn sibling_node(n:*libc::c_void, sibling:*mut*libc::c_void) -> css_error {
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
        
        if node1.prev.is_some() {
            *sibling = ::cast::transmute(node1.prev.unwrap());
        }
        else {
            *sibling = ptr::null();
        }
    }
    CSS_OK
}

#[inline] 
fn node_has_name(lwc_ref:&mut ~lwc, _:*libc::c_void, n:*libc::c_void, qname:&css_qname, matched:&mut bool) -> css_error {
    // debug!("node_has_name");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    if lwc_ref.lwc_string_length(qname.name) == 1 && lwc_ref.lwc_string_data(qname.name)[0] == '*' as u8 {
        *matched = true;
    }
    else {
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.name.get(),qname.name);
    }
    CSS_OK
}

#[inline] 
fn node_has_class(lwc_ref:&mut ~lwc, pw:*libc::c_void ,n:*libc::c_void, name:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_class");
    let mut node1:@mut node;
    let mut ctx: @mut  ctx_pw;
    let mut i:uint = 0 ;
    let len:uint;
	let mut condition_match : bool = false;
	
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
        ctx = ::cast::transmute(pw);
        cast::forget(ctx);
	}	
		
	// debug!(fmt!("node1.attrs.len=%?",node1.attrs.len()));
	// debug!(fmt!("node1.attrs[i].name=%?",copy node1.attrs[i].name));
	len = node1.attrs.len();
	
	while i < len {
		let mut amatched: bool;
		amatched = lwc_ref.lwc_string_caseless_isequal(ctx.attr_class,node1.attrs[i].name); 
		
		if amatched {
			/* Classes are case-sensitive in HTML */
			condition_match = lwc_ref.lwc_string_caseless_isequal(name, node1.attrs[i].value);
			break;
		}
		i += 1;
	}
        
    if condition_match {
        *matched = true;
    }
    else {
        *matched = false;
    }

    CSS_OK
}
#[inline] 
fn node_has_id(lwc_ref:&mut ~lwc, pw:*libc::c_void, n:*libc::c_void, name:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_id");
    let mut node1:@mut node;
    let mut ctx: @mut  ctx_pw;
    let mut i:uint = 0 ;
    let len:uint;
    
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
        ctx = ::cast::transmute(pw);
        cast::forget(ctx);
        len = node1.attrs.len();
        
        while i  < len {
            let mut amatched: bool;
            amatched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,ctx.attr_id); 
            if amatched {
                break;
            }
            i += 1;
        }
    }
    
    /* IDs are case-sensitive in HTML */
    let mut condition_match : bool = false;
    if (i != len) {
        //io::println(fmt!("name=%?",lwc_string_data(name.clone())));
        //io::println(fmt!("node1.attrs[i].name=%?",copy node1.attrs[i].value));
        condition_match = lwc_ref.lwc_string_caseless_isequal(name, node1.attrs[i].value);
    }
    
    if condition_match {
        *matched = true;
    }
    else {
        *matched = false;
    }

    //io::println(fmt!("node_has_id match=%?",*matched));
        
    CSS_OK
}

#[inline] 
fn node_has_attribute(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    let mut i:uint = 0 ;
    *matched = false;
    let attr_len = node1.attrs.len();
    while i < attr_len {
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }
    CSS_OK
}
    
#[inline] 
fn  node_has_attribute_equal(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute_equal");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    let mut i:uint = 0 ;
    
    *matched = false;
    let attr_len = node1.attrs.len();
    while i < attr_len { 
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }

    if *matched {
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,value);
    }
    CSS_OK
}


#[inline] 
fn node_has_attribute_includes(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute_includes");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    
    let mut i:uint = 0 ;
    let vlen = lwc_ref.lwc_string_length(value);
    
    *matched = false;
    
    let attr_len = node1.attrs.len();
    while i < attr_len { 
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }

    if *matched {
        let start = lwc_ref.lwc_string_data(node1.attrs[i].value);
        let mut start_len :uint = 0;
        let mut p:uint = 0;
        let end:uint = start.len();
        *matched =false;
		let val = lwc_ref.lwc_string_data(value);

        while p < end {
            if start[p] == ' ' as u8 {
                if (p - start_len == vlen) && 
                is_string_caseless_equal(start,start_len,val, vlen) {
                    *matched = true;
                    break;
                }
                start_len = p + 1;  
            }
            p +=1;
        }
    }
    CSS_OK
}

#[inline] 
fn node_has_attribute_dashmatch(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute_dashmatch");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    let mut i:uint = 0 ;
    let vlen = lwc_ref.lwc_string_length(value);
    *matched = false;
    let attr_len = node1.attrs.len();
    while i < attr_len { 
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }

    if *matched {
        let start = lwc_ref.lwc_string_data(node1.attrs[i].value);
        let mut start_len :uint = 0;
        let mut p:uint = 0;
        let end:uint = start.len();
        *matched =false;
		let val = lwc_ref.lwc_string_data(value);
		
        while p < end {
            if start[p] == '-' as u8 {
                if (p - start_len == vlen) && 
                is_string_caseless_equal(start,start_len,val, vlen) {
                    *matched = true;
                    break;
                }
                start_len = p + 1;  
            }
            p +=1;
        }
    }
    CSS_OK
}

#[inline] 
fn node_has_attribute_prefix(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute_prefix");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    let mut i:uint = 0 ;
    *matched = false;
    let attr_len = node1.attrs.len();
    while i < attr_len { 
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }

    if *matched {
        let len = lwc_ref.lwc_string_length(node1.attrs[i].value);
        let data = lwc_ref.lwc_string_data(node1.attrs[i].value);
        let vlen = lwc_ref.lwc_string_length(value);
        if len < vlen {
            *matched = false;
        }
        else {
            *matched = is_string_caseless_equal(data,0,lwc_ref.lwc_string_data(value), vlen);
        }
    }
    CSS_OK
}

#[inline] 
fn node_has_attribute_suffix(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute_suffix");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    let mut i:uint = 0 ;
    *matched = false;
    let attr_len = node1.attrs.len();
    while i < attr_len { 
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }

    if *matched {
        let len = lwc_ref.lwc_string_length(node1.attrs[i].value);
        let data = lwc_ref.lwc_string_data(node1.attrs[i].value);
        let vlen = lwc_ref.lwc_string_length(value);
        let suffix_start = len - vlen;
        if len < vlen {
            *matched = false;
        }
        else {
            *matched = is_string_caseless_equal(data,suffix_start,lwc_ref.lwc_string_data(value), vlen);
            
        }
    }


    CSS_OK
}

#[inline] 
fn node_has_attribute_substring(lwc_ref:&mut ~lwc, n:*libc::c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error {
    // debug!("node_has_attribute_substring");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    let mut i:uint = 0 ;
    *matched = false;
    let attr_len = node1.attrs.len();
    while i < attr_len { 
        *matched = lwc_ref.lwc_string_caseless_isequal(node1.attrs[i].name,qname.name);
        if *matched {
            break;
        }
        i += 1;
    }
    if *matched {
        let len = lwc_ref.lwc_string_length(node1.attrs[i].value);
        let data = lwc_ref.lwc_string_data(node1.attrs[i].value);
        let vlen = lwc_ref.lwc_string_length(value);
        let last_start_len = len -vlen;
        if len < vlen {
            *matched = false;
        }
        else {
            let mut iter:uint = 0;
			let val = lwc_ref.lwc_string_data(node1.attrs[i].value);
			
            while iter < last_start_len {
                if is_string_caseless_equal(data,iter,val, vlen) {
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
    CSS_OK
}

#[inline] 
fn node_is_root(n:*libc::c_void, matched:&mut bool) -> css_error {
    debug!("node_is_root");
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    *matched = node1.parent.is_none();
    CSS_OK
}

#[inline]    
fn node_count_siblings(lwc_ref:&mut ~lwc, n:*libc::c_void, same_name:bool, after:bool, count:&mut i32) -> css_error {
    // debug!("node_count_siblings");
    let mut cnt : i32 = 0;
    let mut matched;
    let mut node1:@mut node;
    let mut name: uint ;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
        name = node1.name.get();
    }
    
    if after {
        while node1.next.is_some() {
            if same_name {
                let mut next_name: uint ;
                let temp_node = (node1.next).unwrap();
                next_name = temp_node.name.get();
                
                matched = lwc_ref.lwc_string_caseless_isequal(name, next_name); 
                
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
                let mut prev_name:  uint;
                let temp_node = (node1.prev).unwrap();
                prev_name = temp_node.name.get();
                
                matched = lwc_ref.lwc_string_caseless_isequal(name,prev_name); 
                
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

#[inline]    
fn node_is_empty(n:*libc::c_void, matched:&mut bool) -> css_error {
    let mut node1:@mut node;
    unsafe {
        node1 = ::cast::transmute(n);
        cast::forget(node1);
    }
    *matched = node1.children.is_none();
    CSS_OK
}

#[inline]     
fn node_is_link(_:*libc::c_void, matched:&mut bool) -> css_error {
    
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_visited(_:*libc::c_void, matched:&mut bool) -> css_error {
    
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_hover(_:*libc::c_void, matched:&mut bool) -> css_error {
    
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_active(_:*libc::c_void, matched:&mut bool) -> css_error {
    
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_focus(_:*libc::c_void, matched:&mut bool) -> css_error {
    
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_enabled(_:*libc::c_void, matched:&mut bool) -> css_error {
    
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_disabled(_:*libc::c_void, matched:&mut bool) -> css_error {
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_checked(_:*libc::c_void, matched:&mut bool) -> css_error {
    *matched = false;
    CSS_OK
}

#[inline]  
fn node_is_target(_:*libc::c_void, matched:&mut bool) -> css_error {
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_is_lang(_:*libc::c_void, _:uint, matched:&mut bool) -> css_error {
    *matched = false;
    CSS_OK
}

#[inline] 
fn node_presentational_hint(_:*libc::c_void, _:u32) -> (css_error,Option<@mut css_hint>) {
    (CSS_PROPERTY_NOT_SET,None)
}

#[inline] 
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
        hint.status = CSS_QUOTES_STRING_OR_NONE as u8;
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

#[inline]
fn compute_font_size(parent: Option<@mut css_hint>, size: Option<@mut css_hint>) -> css_error {
    // debug!("\n Entering compute ") ;
    let mut parent_value:@mut css_hint;
    let mut size_val : @mut css_hint;
    let sizes:~[@mut css_hint_length] =
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

        // debug!(fmt!("size_val.status == %? , CSS_FONT_SIZE_INHERIT as u8 == %u" , size_val.status , CSS_FONT_SIZE_INHERIT as uint));
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

#[inline]
pub fn is_string_caseless_equal(a : &str , a_idx:uint, b : &str, cmplen:uint ) -> bool {
	
    
    let mut e :uint = 0 ;
	let mut diff:u8;
	
    while (e < cmplen) {
        if a[a_idx+e] == b[e] {
			e = e + 1;
            loop;
        }
		diff = a[a_idx+e] - b[e];
		if diff == 32 {
			if (a[a_idx+e] >= 'A' as u8  && a[a_idx+e] <= 'Z'  as u8) {
				if (a[a_idx+e]+32) == b[e] {
					e = e + 1;
					loop;
				}
				else {
					return false ;
				}
			}

			if (b[e] >= 'A'  as u8 && b[e] <= 'Z'  as u8) {
				if (b[e]+32) == a[a_idx+e] {
					e = e + 1;
					loop;
				}
				else {
					return false ;
				}
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





