/*
 * This file generates parts of LibCSS.
 */
extern mod std;
use core::io::WriterUtil;
//use std::getopts::*;



//use std::tempfile;

/* Descriptors are space separated key:value pairs brackets () are
 * used to quote in values.
 *
 * Examples:
 * list_style_image:CSS_PROP_LIST_STYLE_IMAGE IDENT:( INHERIT: NONE:0,LIST_STYLE_IMAGE_NONE IDENT:) URI:LIST_STYLE_IMAGE_URI
 *
 * list_style_position:CSS_PROP_LIST_STYLE_POSITION IDENT:( INHERIT: INSIDE:0,LIST_STYLE_POSITION_INSIDE OUTSIDE:0,LIST_STYLE_POSITION_OUTSIDE IDENT:)
*/

struct keyval {
    key:~str,
    val:~str
}


pub fn get_keyval(pos:~str) ->Option<~[keyval]> {
    
    let mut strKVPairs = ~[];
    for str::each_split_char_nonempty(pos,' ') |subs| { 
        strKVPairs.push(subs); 
    }
    

    let mut nkeyval:~[keyval]=~[];
    for  strKVPairs.each |&kv| {
        
        let mut tempKVPair=~[]; 
        for str::each_split_char_nonempty(kv,':') |subs| { 
            tempKVPair.push(subs); 
        }
        if tempKVPair.len() > 1 {
            nkeyval.push(keyval{key:tempKVPair[0].to_owned(),val:tempKVPair[1].to_owned()})
        }   
        else {
            nkeyval.push(keyval{key:tempKVPair[0].to_owned(),val:~""})
        }
    }

    return Some(nkeyval)
}

pub fn file_header(fp:@Writer) -> () {
    fp.write_line("/*");
    fp.write_line(" * This file was generated by LibCSS css_property_parser_gen ");
    fp.write_line(" * Mozilla Samsung Servo Browser Project.");
    fp.write_line(" */");
    fp.write_line("");
    fp.write_line("");
    fp.write_line("use stylesheet::*;");
    fp.write_line("use parse::propstrings::*;");
    fp.write_line("use include::properties::*;");
    fp.write_line("use parse::properties::common::*;");
    fp.write_line("use utils::errors::*;");
    fp.write_line("use lex::lexer::*;");
    fp.write_line("use wapcaplet::*;");
    fp.write_line("use bytecode::opcodes::*;");
    fp.write_line("use bytecode::bytecode::*;");
    fp.write_line("use include::fpmath::*;");
    fp.write_line("");
}

pub fn function_header(fp:@Writer, descriptor:~str, parser_id:&keyval, is_generic:bool) -> () {
    fp.write_line(" /* Generated from:");
    fp.write_line(" *");
    fp.write_line(fmt!(" * %s", descriptor));
    fp.write_line(" * ");
    fp.write_line(" */");
    fp.write_line("");
    fp.write_line("/**");
    fp.write_line(fmt!(" * Parse %s",parser_id.key));
    fp.write_line(" *");
    fp.write_line(" * \\param strings Propstrings");
    fp.write_line(" * \\param vector  Vector of tokens to process");
    fp.write_line(" * \\param ctx     Pointer to vector iteration context");
    fp.write_line(" * \\param result  resulting style");
    fp.write_str(fmt!("%s",if is_generic {" * \\param op     Bytecode OpCode for CSS property to encode"} else { ""}));
    fp.write_line(" * \\return CSS_OK on success,");
    fp.write_line(" *      CSS_INVALID if the input is not valid");
    fp.write_line(" *");
    fp.write_line(" * Post condition: \\a @ctx is updated with the next token to process");
    fp.write_line(" *          If the input is invalid, then \\a @ctx remains unchanged.");
    fp.write_line(" */");
    fp.write_line(fmt!("pub fn css__parse_%s(sheet:@mut css_stylesheet, strings:&mut ~css_propstrings,",parser_id.key));
    fp.write_str("      vector:&~[@css_token], ctx:@mut uint,");
    fp.write_line(fmt!(" result:@mut css_style%s) -> css_error", if is_generic {", op:css_properties_e" } else {""}    ));
    fp.write_line("{");
}


pub fn output_token_type_check(fp:@Writer, do_token_check:bool, IDENT:~[keyval], URI:~[keyval], NUMBER:~[keyval]) {
    let mut output : ~str = ~"\tlet orig_ctx = *ctx;\n";
    str::push_str(&mut output, "\tlet mut error:css_error=CSS_OK;\n");
    str::push_str(&mut output, "\tlet mut token:&@css_token;\n\n");
    
    str::push_str(&mut output, "\tif *ctx >= vector.len() {\n");
    str::push_str(&mut output, "\t\treturn CSS_INVALID\n");
    str::push_str(&mut output, "\t}\n");
    str::push_str(&mut output, "\ttoken = &vector[*ctx];\n");
    str::push_str(&mut output, "\t*ctx += 1;\n\n");
    
    

    if do_token_check {
        let mut prev = false; /* there was a previous check - add && */
                  
        if !vec::is_empty(IDENT) {
            str::push_str(&mut output, "\tif ");
            str::push_str(&mut output,"(match token.token_type { CSS_TOKEN_IDENT(_) => true, _ => false})");
            prev = true;
        }
        if !vec::is_empty(URI) {
            if prev {  str::push_str(&mut output," && \n\t") }
            else {
                str::push_str(&mut output, "\tif ");
            }
            str::push_str(&mut output,"(match token.token_type { CSS_TOKEN_URI(_) => false, _ => true})");
            prev = true;
        }
        if !vec::is_empty(NUMBER) {
            if prev {  str::push_str(&mut output," && \n\t") }
            else {
                str::push_str(&mut output, "\tif ");
            }
            str::push_str(&mut output,"(match token.token_type { CSS_TOKEN_NUMBER(_,_) => false, _ => true}) ");
            prev = true;
        }
        if prev {
            str::push_str(&mut output, " {\n");
            str::push_str(&mut output, "\t\t*ctx = orig_ctx;\n");
            str::push_str(&mut output, "\t\treturn CSS_INVALID\n");
            str::push_str(&mut output, "\t}\n\n");    
        }
    
    }
  
    fp.write_str(output);
}

pub fn output_ident(fp:@Writer, only_ident:bool, parseid:&keyval, IDENT:~[keyval]) {
    let mut output : ~str = ~"";
    for IDENT.each |&i| {
        
        str::push_str(&mut output,"\tif ");
        if !only_ident {
            str::push_str(&mut output,"(match token.token_type { CSS_TOKEN_IDENT(_) => true, _ => false}) && \n");
        }
        str::push_str(&mut output, fmt!("\tstrings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), %s as uint) {\n",i.key));
        if i.key == ~"INHERIT" {
            str::push_str(&mut output, fmt!("\t\tcss_stylesheet::css_stylesheet_style_inherit(result, %s)\n", parseid.val));
        } 
        else {
            str::push_str(&mut output, fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, %s)\n",  parseid.val, i.val));
        }
        str::push_str(&mut output,"\t} \n\telse ");
    }
    fp.write_str(output);
}

pub fn output_uri(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    
        let mut output : ~str = ~" if match token.token_type { CSS_TOKEN_URI(_) => true, _ => false} {\n";
        
        str::push_str(&mut output,"\n");
        str::push_str(&mut output,"\t\tmatch (*sheet.resolve)(copy sheet.url, token.idata.get_ref().clone()) {\n");
        str::push_str(&mut output,"\t\t\t(CSS_OK, Some(uri)) => {\n");
        str::push_str(&mut output,"\t\t\t\tlet uri_snumber = sheet.css__stylesheet_string_add(lwc_string_data(uri));\n");
        str::push_str(&mut output,fmt!("\t\t\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s );\n",parseid.val,kvlist[0].val));
        str::push_str(&mut output,"\t\t\t\tcss_stylesheet::css__stylesheet_style_append(result, uri_snumber as u32)\n");
        str::push_str(&mut output,"\n");
        str::push_str(&mut output,"\t\t\t},\n");
        str::push_str(&mut output,"\t\t\t(error, _ ) => {\n");
        str::push_str(&mut output,"\t\t\t\t*ctx = orig_ctx;\n");
        str::push_str(&mut output,"\t\t\t\treturn error\n");
        str::push_str(&mut output,"\t\t\t},\n");
        str::push_str(&mut output,"\t\t}\n");
        str::push_str(&mut output,"\n");
        str::push_str(&mut output,"\t} \n\t else ");
        fp.write_str(output);
}

pub fn output_number(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    
    let mut output : ~str = ~"\tif match token.token_type { CSS_TOKEN_NUMBER(_,_) => true, _ => false} {\n";
  
    str::push_str(&mut output,fmt!("\t\tlet mut (num,consumed): (i32,uint)=  css__number_from_lwc_string(token.idata.get_ref().clone(), %s);\n",kvlist[0].key));
    str::push_str(&mut output,"\t\t/* Invalid if there are trailing characters */\n");
    str::push_str(&mut output,"\t\tif consumed != lwc_string_length(token.idata.get_ref().clone()) {\n");
    str::push_str(&mut output,"\t\t\t*ctx = orig_ctx;\n");
    str::push_str(&mut output,"\t\t\treturn CSS_INVALID\n");
    str::push_str(&mut output,"\t\t}\n");
        

    for uint::range(1 , kvlist.len()-1) |i| {
        if kvlist[i].key == ~"RANGE" {
            str::push_str(&mut output,fmt!("\t\tif %s {\n",kvlist[i].val));
            str::push_str(&mut output,"\t\t\t*ctx = orig_ctx;\n");
            str::push_str(&mut output,"\t\t\treturn CSS_INVALID\n");
            str::push_str(&mut output,"\t\t}\n\n")
        }
    }

    str::push_str(&mut output,fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s);\n",parseid.val,kvlist[0].val));
    str::push_str(&mut output,"\t\tcss_stylesheet::css__stylesheet_style_append(result, num as u32)\n");
    str::push_str(&mut output,"\t} \n\t else ");
    fp.write_str(output);
}

pub fn output_color(fp:@Writer, parseid:&keyval) {
    let mut output : ~str = ~"\t{\n";
    str::push_str(&mut output,"\t\t*ctx = orig_ctx;\n\n");
    str::push_str(&mut output,"\t\tlet mut value:u16;\n\n");
    str::push_str(&mut output,"\t\tlet mut color:u32;\n\n");
    str::push_str(&mut output,"\t\tlet (value_option, color_option, res)= css__parse_color_specifier(sheet, strings, vector, ctx);\n");
    str::push_str(&mut output,"\t\tmatch res {\n");
    str::push_str(&mut output,"\t\t\tCSS_OK => {\n");
    str::push_str(&mut output,"\t\t\t\tvalue = value_option.unwrap();\n");
    str::push_str(&mut output,"\t\t\t\tcolor = color_option.unwrap() },\n");
    //str::push_str(&mut output,"\t\t\t\tcolor = color_option.unwrap() },\n");
    str::push_str(&mut output,"\t\t\t_ => {\n");
    str::push_str(&mut output,"\t\t\t\t*ctx = orig_ctx;\n");
    str::push_str(&mut output,"\t\t\t\treturn error\n");
    str::push_str(&mut output,"\t\t\t}\n");
    str::push_str(&mut output,"\t\t}\n\n");
    str::push_str(&mut output,fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, value);\n",parseid.val));
    str::push_str(&mut output,"\n");
    str::push_str(&mut output,"\t\tif value == COLOR_SET {\n");
    str::push_str(&mut output,"\t\t\tcss_stylesheet::css__stylesheet_style_append(result, color as u32)\n");
    str::push_str(&mut output,"\t\t}\n");
    str::push_str(&mut output,"\t}\n\n");
    fp.write_str(output);   
}

pub fn output_length_unit(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    let mut output : ~str = ~"\t{\n";
    str::push_str(&mut output,"\t\tlet length:u32;\n");
    str::push_str(&mut output,"\t\t*ctx = orig_ctx;\n\n");
    str::push_str(&mut output,"\t\tlet mut unit:u32;\n\n");
    str::push_str(&mut output,fmt!("\t\tlet (length_option, unit_option, res) =css__parse_unit_specifier(sheet, vector, ctx, %s as u32);\n",kvlist[0].key));
    str::push_str(&mut output,"\t\tmatch res {\n");
    str::push_str(&mut output,"\t\t\tCSS_OK => {\n");
    str::push_str(&mut output,"\t\t\t\tunit = unit_option.unwrap();\n");
    str::push_str(&mut output,"\t\t\t\tlength = length_option.unwrap() as u32;\n");
    str::push_str(&mut output,"\t\t\t},\n");
    str::push_str(&mut output,"\t\t\t_ => {\n");
    str::push_str(&mut output,"\t\t\t\t*ctx = orig_ctx;\n");
    str::push_str(&mut output,"\t\t\t\treturn error\n");
    str::push_str(&mut output,"\t\t\t}\n");
    str::push_str(&mut output,"\t\t}\n\n");
        

    for uint::range(1 , kvlist.len()-1) |i| { 
        
        if kvlist[i].key == ~"ALLOW" {
            str::push_str(&mut output,fmt!("\t\tif !(%s) {\n",kvlist[i].val));
            str::push_str(&mut output,"\t\t\t*ctx = orig_ctx;\n");
            str::push_str(&mut output,"\t\t\treturn CSS_INVALID\n");
            str::push_str(&mut output,"\t\t}\n\n")
        } else if kvlist[i].key == ~"DISALLOW" {
            str::push_str(&mut output,fmt!("\t\tif %s {\n",kvlist[i].val));
            str::push_str(&mut output,"\t\t\t*ctx = orig_ctx;\n");
            str::push_str(&mut output,"\t\t\treturn CSS_INVALID\n");
            str::push_str(&mut output,"\t\t}\n\n")
        } else if kvlist[i].key == ~"RANGE" {
            str::push_str(&mut output,fmt!("\t\tif length %s {\n",kvlist[i].val));
            str::push_str(&mut output,"\t\t\t*ctx = orig_ctx;\n");
            str::push_str(&mut output,"\t\t\treturn CSS_INVALID\n");
            str::push_str(&mut output,"\t\t}\n\n")
        }

    }

    
    str::push_str(&mut output,fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s);\n",parseid.val,kvlist[0].val));
    str::push_str(&mut output,"\n");
    str::push_str(&mut output,"\t\tcss_stylesheet::css__stylesheet_style_vappend(result , [length, unit])\n");
    str::push_str(&mut output,"\t}\n\n");
    fp.write_str(output);
}

pub fn output_ident_list(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    
    if kvlist[0].key == ~"STRING_OPTNUM" {
        /* list of IDENT and optional numbers */
        
        let mut output : ~str = ~"{\n"; 
            
            str::push_str(&mut output,fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s);\n", parseid.val,kvlist[0].val));
            str::push_str(&mut output,"\t\twhile (*ctx < vector.len()) && (match (&vector[*ctx]).token_type {CSS_TOKEN_IDENT(_) => true, _ => false}) {\n");
            str::push_str(&mut output,"\t\t\ttoken=&vector[*ctx];\n");
            str::push_str(&mut output,"\t\t\tlet mut num:css_fixed;\n");
            str::push_str(&mut output,"\t\t\tlet mut pctx:uint;\n\n");
            str::push_str(&mut output,"\t\t\tlet mut token_null=false;\n\n");
            str::push_str(&mut output,"\t\t\tlet snumber = sheet.css__stylesheet_string_add(lwc_string_data(token.idata.get_ref().clone()));\n");
            str::push_str(&mut output,"\t\t\tcss_stylesheet::css__stylesheet_style_append(result, snumber as u32);\n"); 
            str::push_str(&mut output,"\t\t\tconsumeWhitespace(vector, ctx);\n\n");
            str::push_str(&mut output,"\t\t\tpctx = *ctx;\n");
            str::push_str(&mut output,"\t\t\tif *ctx >= vector.len() {\n");
            str::push_str(&mut output,"\t\t\t\ttoken_null = true\n");
            str::push_str(&mut output,"\t\t\t}\n");
            str::push_str(&mut output,"\t\t\telse { \n");
            str::push_str(&mut output,"\t\t\t\ttoken = &vector[*ctx];\n");
            str::push_str(&mut output,"\t\t\t\t*ctx += 1 //Iterate\n");
            str::push_str(&mut output,"\t\t\t}\n");
            str::push_str(&mut output,"\t\t\tif !token_null && (match token.token_type { CSS_TOKEN_NUMBER(_,_) => true, _ => false}) {\n");
            str::push_str(&mut output,"\t\t\t\tlet (ret_num, consumed) = css__number_from_lwc_string(token.idata.get_ref().clone(), true);\n");
            str::push_str(&mut output,"\t\t\t\tif consumed != lwc_string_length(token.idata.get_ref().clone()) {\n");
            str::push_str(&mut output,"\t\t\t\t\t*ctx = orig_ctx;\n");
            str::push_str(&mut output,"\t\t\t\t\treturn CSS_INVALID\n");
            str::push_str(&mut output,"\t\t\t\t}\n");
            str::push_str(&mut output,"\t\t\t\tnum = css_int_to_fixed(ret_num as int);\n\n");
            str::push_str(&mut output,"\t\t\t\tconsumeWhitespace(vector, ctx);\n\n");
            str::push_str(&mut output,"\t\t\t\tpctx = *ctx;\n");
            str::push_str(&mut output,"\t\t\t\tif *ctx >= vector.len() {\n");
            str::push_str(&mut output,"\t\t\t\t\ttoken_null = true\n");
            str::push_str(&mut output,"\t\t\t\t}\n");
            str::push_str(&mut output,"\t\t\t\telse { \n");
            str::push_str(&mut output,"\t\t\t\t\ttoken = &vector[*ctx];\n");
            str::push_str(&mut output,"\t\t\t\t\t*ctx += 1 //Iterate\n");
            str::push_str(&mut output,"\t\t\t\t}\n");
            str::push_str(&mut output,"\t\t\t}\n");
            str::push_str(&mut output,"\t\t\telse {\n");
            str::push_str(&mut output,fmt!("\t\t\t\tnum = css_int_to_fixed(%s)\n",kvlist[1].key));
            str::push_str(&mut output,"\t\t\t}\n\n");
            str::push_str(&mut output,"\t\t\tcss_stylesheet::css__stylesheet_style_append(result, num as u32);\n");
            str::push_str(&mut output,"\t\t\tif token_null {\n");
            str::push_str(&mut output,"\t\t\t\tbreak;\n}\n");
            str::push_str(&mut output,"\t\t\tif match token.token_type { CSS_TOKEN_IDENT(_) => true, _ => false} {\n");
            str::push_str(&mut output,fmt!("\t\t\t\tcss_stylesheet::css__stylesheet_style_append(result, %s as u32);\n",kvlist[0].val));
            str::push_str(&mut output,"\t\t\t}\n");
            str::push_str(&mut output,"\t\t\telse {\n");
            str::push_str(&mut output,"\t\t\t\t*ctx = pctx // rewind one token back */\n");
            str::push_str(&mut output,"\t\t\t}\n");
            str::push_str(&mut output,"\t\t}\n\n");
            str::push_str(&mut output,fmt!("\t\tcss_stylesheet::css__stylesheet_style_append(result, %s as u32)\n",kvlist[1].val));
            str::push_str(&mut output,"\t}\n\n");
            fp.write_str(output);
    } 
    else {
        io::println(fmt!("unknown IDENT list type %s\n",kvlist[0].key));
        fail!();
    }
}

pub fn output_invalidcss(fp:@Writer) {
    fp.write_str("{\n\t\terror = CSS_INVALID }\n\n");
}

pub fn output_footer(fp:@Writer) {
    let mut output : ~str = ~"\tif match error {CSS_OK => false, _ => true} {\n";
    str::push_str(&mut output,"\t\t*ctx = orig_ctx;\n\t}\n");
    str::push_str(&mut output," \n");
    str::push_str(&mut output,"\treturn error\n");
    str::push_str(&mut output,"}\n\n");
    fp.write_str(output);
}

pub fn output_wrap(fp:@Writer, parseid:&keyval, WRAP:~[keyval]) {
    
    fp.write_str(fmt!(" return %s(sheet, strings, vector, ctx, result, %s)\n}\n",WRAP[0].val,parseid.val));
}


fn main() {

    //let args = os::args();

    let str_INHERIT = ~"INHERIT";

    let mut ident_inherit = keyval{ key:str_INHERIT, val:~""};
    
    let writer:@Writer;
    
    let curpos = 0; /* current position in input string */
    
    enum list_type {
        NUMBER, 
        BASE,
        IDENT,
        IDENT_LIST,
        LENGTH_UNIT 
    }

    
    // Below commented code takes command line arguments and generates the code in the given path
    // if args.len() < 2 {
    //     io::println(fmt!("Usage: %s [-o <filename>] <descriptor>\n", args[0]));
    //     fail!(~"Invalid Usage");
    // }

    // if args[1] == ~"--o" {
        
    //     if (args.len() != 4) {
    //         io::println(fmt!("Usage: %s [-o <filename>] <descriptor>\n", args[0]));
    //         fail!();
    //     }

    //     let path = Path("args[2]");
    //     writer = io::file_writer(&path,[io::Create, io::Append]).get();
    //     descriptor = copy args[3]
    // } 
    // else {
    //     writer = io::stdout();
    //     descriptor = copy args[1]
    // }
       

    let input_path = Path("properties.gen");
    let reader = result::get(&io::file_reader(&input_path));
    let output_path = Path("autogenerated.rs");
    writer = io::file_writer(&output_path,[io::Create]).get();
    file_header(writer);

    while !reader.eof() {
        let mut descriptor:~str;       
        let mut line = reader.read_line();
        let mut curlist:list_type; //:&~[keyval]=&~[];
        let mut do_token_check = true; /* if the check for valid tokens is done */
        let mut only_ident = true; /* if the only token type is ident */
        let mut is_generic = false;

        let mut base:~[keyval]=~[];
        let mut ident:~[keyval]=~[];
        let mut ident_list:~[keyval]=~[];
        let mut length_unit:~[keyval]=~[];
        let mut uri:~[keyval]=~[];
        let mut wrap:~[keyval]=~[];
        let mut number:~[keyval]=~[];
        let mut color:~[keyval]=~[];

        while (line.is_whitespace() || line[0] == '#' as u8) && !reader.eof() {
            line = reader.read_line()
        }
        
        if reader.eof() {
            break;
        }

        descriptor = ~"";
        descriptor.push_str(line);
        
        while !line.is_whitespace() && line[0] != '#' as u8 && !reader.eof() {
            line = reader.read_line();
            descriptor.push_str(line)
        }  
        

        curlist = BASE;

        match get_keyval(copy descriptor) {
            None =>     {
                            io::println(fmt!("Token error at offset %u\n", curpos));
                            fail!();
                        }

            Some(rkv_list)  =>  
                            for rkv_list.each |&rkv| {
                                if rkv.key == ~"WRAP" {
                                    wrap.push(rkv);
                                    only_ident = false;
                                } 
                                else if rkv.key == ~"NUMBER" {
                                    if str::char_at(rkv.val, 0) == '(' {
                                        curlist = NUMBER;
                                    } 
                                    else if str::char_at(rkv.val, 0) == ')' {
                                        curlist = BASE;
                                    } 
                                    else {
                                        number.push(rkv);
                                    }
                                    only_ident = false;
                                } 
                                else if rkv.key == ~"IDENT" {
                                    if str::char_at(rkv.val, 0) == '(' {
                                        curlist = IDENT;
                                    } 
                                    else if str::char_at(rkv.val, 0) == ')' {
                                        curlist = BASE;
                                    } 
                                    else if rkv.val ==  ~"INHERIT" {
                                        ident.push(copy ident_inherit);
                                    }
                                } 
                                else if rkv.key == ~"IDENT_LIST" {
                                    if str::char_at(rkv.val, 0) == '(' {
                                        curlist = IDENT_LIST;
                                    } 
                                    else if str::char_at(rkv.val, 0) == ')' {
                                        curlist = BASE;
                                    } 
                                } 
                                else if rkv.key == ~"LENGTH_UNIT" {
                                    if str::char_at(rkv.val, 0) == '(' {
                                        curlist = LENGTH_UNIT;
                                    } 
                                    else if str::char_at(rkv.val, 0) == ')' {
                                        curlist = BASE;
                                    } 
                                    only_ident = false;
                                    do_token_check = false;
                                } 
                                else if rkv.key == ~"COLOR" {
                                    color.push(rkv);
                                    do_token_check = false;
                                    only_ident = false;
                                } 
                                else if rkv.key == ~"URI" {
                                    uri.push(rkv);
                                    only_ident = false;
                                } 
                                else if rkv.key == ~"GENERIC" {
                                    is_generic = true;
                                } 
                                else {
                                    /* just append to current list */
                                    match curlist {
                                        NUMBER => number.push(rkv), 
                                        BASE => base.push(rkv),
                                        IDENT => ident.push(rkv),
                                        IDENT_LIST => ident_list.push(rkv),
                                        LENGTH_UNIT => length_unit.push(rkv)
                                    }           
                                }
                            }
        }

        if base.len() != 1 {
            io::println(fmt!("Incorrect base element count (got %u expected 1)\n", base.len()));
            fail!();
        }


        /* header */
        function_header(writer, descriptor, &base[0], is_generic);

        if !wrap.is_empty() {
            output_wrap(writer, &base[0], wrap);
        }
        else {
            /* check token type is correct */
            output_token_type_check(writer, do_token_check,  copy ident, copy uri, copy number);

            if !ident.is_empty() {
                output_ident(writer, only_ident, &base[0], ident);
            }
            if !uri.is_empty() {
                output_uri(writer,&base[0], uri);
            }
            if !number.is_empty() {
                output_number(writer, &base[0], number);
            }

            /* terminal blocks, these end the ladder ie no trailing else */
            if !color.is_empty() {
                output_color(writer, &base[0]);
            } 
            else if !length_unit.is_empty() {
                output_length_unit(writer, &base[0], length_unit);
            } 
            else if !ident_list.is_empty() {
                output_ident_list(writer, &base[0], ident_list);
            } 
            else {
                output_invalidcss(writer);
            }

            output_footer(writer);

        }
    }    
    
}

