/*
 * This file generates parts of LibCSS.
 */
use std::io::*;
use std::clone::Clone;


/**
* #Description:
*  Descriptors are space separated key:value pairs brackets () are
*  used to quote in values.
* #Examples:
*  list_style_image:CSS_PROP_LIST_STYLE_IMAGE IDENT:( INHERIT: NONE:0,LIST_STYLE_IMAGE_NONE IDENT:) URI:LIST_STYLE_IMAGE_URI
*
*  list_style_position:CSS_PROP_LIST_STYLE_POSITION IDENT:( INHERIT: INSIDE:0,LIST_STYLE_POSITION_INSIDE OUTSIDE:0,LIST_STYLE_POSITION_OUTSIDE IDENT:)
*/

struct keyval {
    key:~str,
    val:~str
}

impl Clone for keyval {
    #[inline]
    fn clone(&self) -> keyval {
        keyval {
            key:self.key.clone(),
            val:self.val.clone()
        }
    }
}


pub fn get_keyval(pos:~str) ->Option<~[keyval]> {
    
    let mut strKVPairs = ~[];
    for subs in pos.split_iter(' ') { 
       if !subs.is_empty() {
		 strKVPairs.push(subs); 
	   }
	}
    

    let mut nkeyval:~[keyval]=~[];
    for &kv in strKVPairs.mut_iter() {
        
        let mut tempKVPair=~[]; 
        for subs in kv.split_iter(':') { 
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
    fp.write_line("/**");
    fp.write_line(" * #Generated from:");
    fp.write_line(" *");
    fp.write_line(fmt!(" *     %s", descriptor));
    fp.write_line(" * ");
    fp.write_line(" */");
    fp.write_line("");
    fp.write_line("/**");
    fp.write_line("* #Arguments:");
    fp.write_line("*  'sheet' -  Stylesheet.");
    fp.write_line("*  'strings' -  css_propstrings.");
    fp.write_line("*  'vector' -  Vector of tokens to process.");
    fp.write_line("*  'ctx' -  Pointer to vector iteration context.");
    fp.write_line("*  'result' - resulting style.");
    fp.write_str(fmt!("%s",if is_generic {" * 'op' - Bytecode OpCode for CSS property to encode"} else { ""}));
    fp.write_line("* #Return Value:");
    fp.write_line("*  'css_error' - CSS_OK on success,");
    fp.write_line("      CSS_INVALID if the input is not valid");
    fp.write_line("* #Post Condition:");
    fp.write_line("*   ctx is updated with the next token to process");
    fp.write_line("*     If the input is invalid, then ctx remains unchanged.");
    fp.write_line("*/");
    fp.write_line(fmt!("pub fn css__parse_%s(_stylesheet_vector:&mut ~[css_stylesheet], _sheet:uint, lwc_ref:&mut ~lwc, strings:&css_propstrings,",parser_id.key));
    fp.write_str("      vector:&~[~css_token], ctx:@mut uint,");
    fp.write_line(fmt!(" result:&mut ~css_style%s) -> css_error", if is_generic {", op:css_properties_e" } else {""}    ));
    fp.write_line("{");
    fp.write_line(fmt!("//debug!(\"Entering: css__parse_%s\");", parser_id.key));
}


pub fn output_token_type_check(fp:@Writer, do_token_check:bool, IDENT:~[keyval], URI:~[keyval], NUMBER:~[keyval]) {
    let mut output : ~str = ~"\tlet orig_ctx = *ctx;\n";
    output.push_str( "\tlet mut error:css_error=CSS_OK;\n");
    output.push_str( "\tlet mut token: &~css_token;\n\n");
    
    output.push_str( "\tif *ctx >= vector.len() {\n");
    output.push_str( "\t\treturn CSS_INVALID\n");
    output.push_str( "\t}\n");
    output.push_str( "\ttoken = &vector[*ctx];\n");
    output.push_str( "\t*ctx += 1;\n\n");
    
    

    if do_token_check {
        let mut prev = false; /* there was a previous check - add && */
                  
        if !IDENT.is_empty() {
            output.push_str( "\tif ");
            output.push_str("(match token.token_type { CSS_TOKEN_IDENT => false, _ => true})");
            prev = true;
        }
        if !URI.is_empty() {
            if prev {  output.push_str(" && \n\t") }
            else {
                output.push_str( "\tif ");
            }
            output.push_str("(match token.token_type { CSS_TOKEN_URI => false, _ => true})");
            prev = true;
        }
        if !NUMBER.is_empty() {
            if prev {  output.push_str(" && \n\t") }
            else {
                output.push_str( "\tif ");
            }
            output.push_str("(match token.token_type { CSS_TOKEN_NUMBER => false, _ => true}) ");
            prev = true;
        }
        if prev {
            output.push_str( " {\n");
            output.push_str( "\t\t*ctx = orig_ctx;\n");
            output.push_str( "\t\treturn CSS_INVALID\n");
            output.push_str( "\t}\n\n");    
        }
    
    }
  
    fp.write_str(output);
}

pub fn output_ident(fp:@Writer, only_ident:bool, parseid:&keyval, IDENT:~[keyval]) {
    let mut output : ~str = ~"";
    for i in IDENT.iter() {
        
        output.push_str("\tif ");
        if !only_ident {
            output.push_str("(match token.token_type { CSS_TOKEN_IDENT => true, _ => false}) && \n");
        }
        output.push_str( fmt!("\tstrings.lwc_string_caseless_isequal(lwc_ref, token.idata.get_ref().clone(), %s as uint) {\n",i.clone().key));
        if i.key.clone() == ~"INHERIT" {
            output.push_str( fmt!("\t\tcss_stylesheet::css_stylesheet_style_inherit(result, %s)\n", parseid.val));

        } 
        else {
            output.push_str( fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, %s)\n",  parseid.val, i.clone().val));
        }
        output.push_str("\t} \n\telse ");
    }
    fp.write_str(output);
}

pub fn output_uri(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    
        let mut output : ~str = ~" if match token.token_type { CSS_TOKEN_URI => true, _ => false} {\n";
        
        output.push_str("\n");
        output.push_str("\t\tmatch (*_stylesheet_vector[_sheet].resolve)(_stylesheet_vector[_sheet].url, token.idata.get_ref().clone()) {\n");
        output.push_str("\t\t\t(CSS_OK, Some(uri)) => {\n");
        output.push_str("\t\t\t\tlet uri_snumber = _stylesheet_vector[_sheet].css__stylesheet_string_add(uri);\n");
        output.push_str(fmt!("\t\t\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s );\n",parseid.val,kvlist[0].val));
        output.push_str("\t\t\t\tcss_stylesheet::css__stylesheet_style_append(result, uri_snumber as u32)\n");
        output.push_str("\n");
        output.push_str("\t\t\t},\n");
        output.push_str("\t\t\t(error, _ ) => {\n");
        output.push_str("\t\t\t\t*ctx = orig_ctx;\n");
        output.push_str("\t\t\t\treturn error\n");
        output.push_str("\t\t\t},\n");
        output.push_str("\t\t}\n");
        output.push_str("\n");
        output.push_str("\t} \n\t else ");
        fp.write_str(output);
}

pub fn output_number(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    
    let mut output : ~str = ~"\tif match token.token_type { CSS_TOKEN_NUMBER => true, _ => false} {\n";
  
    output.push_str(fmt!("\t\tlet (num,consumed): (i32,uint)=  css__number_from_lwc_string(lwc_ref,token.idata.get_ref().clone(), %s);\n",kvlist[0].key));
    output.push_str("\t\t/* Invalid if there are trailing characters */\n");
    output.push_str("\t\tif consumed != lwc_ref.lwc_string_length(token.idata.get_ref().clone()) {\n");
    output.push_str("\t\t\t*ctx = orig_ctx;\n");
    output.push_str("\t\t\treturn CSS_INVALID\n");
    output.push_str("\t\t}\n");
    let mut i = 1;    
	let range = kvlist.len();
    while i < range {
        if kvlist[i].key == ~"RANGE" {
            output.push_str(fmt!("\t\tif %s {\n",kvlist[i].val));
            output.push_str("\t\t\t*ctx = orig_ctx;\n");
            output.push_str("\t\t\treturn CSS_INVALID\n");
            output.push_str("\t\t}\n\n")
        }
		i = i+1;
    }

    output.push_str(fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s);\n",parseid.val,kvlist[0].val));
    output.push_str("\t\tcss_stylesheet::css__stylesheet_style_append(result, num as u32)\n");
    output.push_str("\t} \n\t else ");
    fp.write_str(output);
}

pub fn output_color(fp:@Writer, parseid:&keyval) {
    let mut output : ~str = ~"\t{\n";
    output.push_str("\t\t*ctx = orig_ctx;\n\n");
    output.push_str("\t\tlet mut value:u16;\n\n");
    output.push_str("\t\tlet mut color:u32;\n\n");
    output.push_str("\t\tlet (value_option, color_option, res)= css__parse_color_specifier(_stylesheet_vector, _sheet, lwc_ref, strings, vector, ctx);\n");
    output.push_str("\t\terror = res;\n");
    output.push_str("\t//debug!(\"error == %? (1)\" , error)\n");
    output.push_str("\t\tmatch res {\n");
    output.push_str("\t\t\tCSS_OK => {\n");
    output.push_str("\t\t\t\tvalue = value_option.unwrap();\n");
    output.push_str("\t\t\t\tcolor = color_option.unwrap() },\n");
    //output.push_str("\t\t\t\tcolor = color_option.unwrap() },\n");
    output.push_str("\t\t\t_ => {\n");
    output.push_str("\t\t\t\t*ctx = orig_ctx;\n");
    output.push_str("\t\t\t\treturn res\n");
    output.push_str("\t\t\t}\n");
    output.push_str("\t\t}\n\n");
    output.push_str(fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, value);\n",parseid.val));
    output.push_str("\n");
    output.push_str("\t\tif value == COLOR_SET {\n");
    output.push_str("\t\t\tcss_stylesheet::css__stylesheet_style_append(result, color as u32)\n");
    output.push_str("\t\t}\n");
    output.push_str("\t}\n\n");
    fp.write_str(output);   
}

pub fn output_length_unit(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    let mut output : ~str = ~"\t{\n";
    output.push_str("\t\tlet reason = \"Function css__parse_letter_spacing\";\n");
    output.push_str("\t\tlet length:u32;\n");
    output.push_str("\t\t*ctx = orig_ctx;\n\n");
    output.push_str("\t\tlet mut unit:u32;\n\n");
    output.push_str(fmt!("\t\tlet (length_option, unit_option, res) =css__parse_unit_specifier(_stylesheet_vector, _sheet, lwc_ref, vector, ctx, %s as u32);\n",kvlist[0].key));
    output.push_str("\t\terror = res;\n");
    output.push_str("\t//debug!(\"error == %?(1)\" , error)\n");
    output.push_str("\t\tmatch res {\n");
    output.push_str("\t\t\tCSS_OK => {\n");
    output.push_str("\t\t\t\tunit = unit_option.unwrap();\n");
    output.push_str("\t\t\t\tlength = length_option.expect(reason) as u32;\n");
    output.push_str("\t\t\t},\n");
    output.push_str("\t\t\t_ => {\n");
    output.push_str("\t\t\t\t*ctx = orig_ctx;\n");
    output.push_str("\t\t\t\treturn res\n");
    output.push_str("\t\t\t}\n");
    output.push_str("\t\t}\n\n");
    output.push_str("\t\t\tlet _length_fixed = length_option.expect(reason);\n");
        

    let mut i = 1;    
	let range = kvlist.len();
    while i < range { 
        if kvlist[i].key == ~"ALLOW" {
            output.push_str(fmt!("\t\tif !(%s) {\n",kvlist[i].val));
            output.push_str("\t\t\t*ctx = orig_ctx;\n");
            output.push_str("\t\t\treturn CSS_INVALID\n");
            output.push_str("\t\t}\n\n")
        } else if kvlist[i].key == ~"DISALLOW" {
            output.push_str(fmt!("\t\tif %s {\n",kvlist[i].val));
            output.push_str("\t\t\t*ctx = orig_ctx;\n");
            output.push_str("\t\t\treturn CSS_INVALID\n");
            output.push_str("\t\t}\n\n")
        } else if kvlist[i].key == ~"RANGE" {
            output.push_str(fmt!("\t\tif _length_fixed %s {\n",kvlist[i].val));
            output.push_str("\t\t\t*ctx = orig_ctx;\n");
            output.push_str("\t\t\treturn CSS_INVALID\n");
            output.push_str("\t\t}\n\n")
        }
		i = i+1;
    }

    
    output.push_str(fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s);\n",parseid.val,kvlist[0].val));
    output.push_str("\n");
    output.push_str("\t\tcss_stylesheet::css__stylesheet_style_vappend(result , [length, unit])\n");
    output.push_str("\t}\n\n");
    fp.write_str(output);
}

pub fn output_ident_list(fp:@Writer, parseid:&keyval, kvlist:~[keyval]) {
    
    if kvlist[0].key == ~"STRING_OPTNUM" {
        /* list of IDENT and optional numbers */
        
        let mut output : ~str = ~"{\n"; 
            
            output.push_str(fmt!("\t\tcss_stylesheet::css__stylesheet_style_appendOPV(result, %s, 0, %s);\n", parseid.val,kvlist[0].val));
            output.push_str("\t\t\tlet mut token_null=false;\n\n");
            output.push_str("\t\twhile !token_null && (match token.token_type {CSS_TOKEN_IDENT => true, _ => false}) {\n");
            output.push_str("\t\t\tlet mut num:css_fixed;\n");
            output.push_str("\t\t\tlet mut pctx:uint;\n\n");
            output.push_str("\t\t\tlet snumber = _stylesheet_vector[_sheet].css__stylesheet_string_add(token.idata.unwrap());\n");
            output.push_str("\t\t\tcss_stylesheet::css__stylesheet_style_append(result, snumber as u32);\n"); 
            output.push_str("\t\t\tconsumeWhitespace(vector, ctx);\n\n");
            output.push_str("\t\t\tpctx = *ctx;\n");
            output.push_str("\t\t\tif *ctx >= vector.len() {\n");
            output.push_str("\t\t\t\ttoken_null = true\n");
            output.push_str("\t\t\t}\n");
            output.push_str("\t\t\telse { \n");
            output.push_str("\t\t\t\ttoken = &vector[*ctx];\n");
            output.push_str("\t\t\t\t*ctx += 1; //Iterate\n");
            output.push_str("\t\t\t}\n");
            output.push_str("\t\t\tif !token_null && (match token.token_type { CSS_TOKEN_NUMBER => true, _ => false}) {\n");
            output.push_str("\t\t\t\tlet (_num, consumed) = css__number_from_lwc_string(lwc_ref,token.idata.get_ref().clone(), true);\n");
            output.push_str("\t\t\t\tnum = _num;\n");
            output.push_str("\t\t\t\tif consumed != lwc_ref.lwc_string_length(token.idata.get_ref().clone()) {\n");
            output.push_str("\t\t\t\t\t*ctx = orig_ctx;\n");
            output.push_str("\t\t\t\t\treturn CSS_INVALID\n");
            output.push_str("\t\t\t\t}\n");
            output.push_str("\t\t\t\tconsumeWhitespace(vector, ctx);\n\n");
            output.push_str("\t\t\t\tpctx = *ctx;\n");
            output.push_str("\t\t\t\tif *ctx >= vector.len() {\n");
            output.push_str("\t\t\t\t\ttoken_null = true\n");
            output.push_str("\t\t\t\t}\n");
            output.push_str("\t\t\t\telse { \n");
            output.push_str("\t\t\t\t\ttoken = &vector[*ctx];\n");
            output.push_str("\t\t\t\t\t*ctx += 1; //Iterate\n");
            output.push_str("\t\t\t\t}\n");
            output.push_str("\t\t\t}\n");
            output.push_str("\t\t\telse {\n");
            output.push_str(fmt!("\t\t\t\tnum = css_int_to_fixed(%s);\n",kvlist[1].key));
            output.push_str("\t\t\t}\n\n");
            output.push_str("\t\t\tcss_stylesheet::css__stylesheet_style_append(result, num as u32);\n");
            output.push_str("\t\t\tif token_null {\n");
            output.push_str("\t\t\t\tbreak;\n}\n");
            output.push_str("\t\t\tif match token.token_type { CSS_TOKEN_IDENT => true, _ => false} {\n");
            output.push_str(fmt!("\t\t\t\tcss_stylesheet::css__stylesheet_style_append(result, %s as u32);\n",kvlist[0].val));
            output.push_str("\t\t\t}\n");
            output.push_str("\t\t\telse {\n");
            output.push_str("\t\t\t\t*ctx = pctx // rewind one token back */\n");
            output.push_str("\t\t\t}\n");
            output.push_str("\t\t}\n\n");
            output.push_str(fmt!("\t\tcss_stylesheet::css__stylesheet_style_append(result, %s as u32)\n",kvlist[1].val));
            output.push_str("\t}\n\n");
            fp.write_str(output);
    } 
    else {
        println(fmt!("unknown IDENT list type %s\n",kvlist[0].key));
        fail!();
    }
}

pub fn output_invalidcss(fp:@Writer) {
    fp.write_str("{\n\t\terror = CSS_INVALID }\n\n");
}

pub fn output_footer(fp:@Writer) {
    let mut output : ~str = ~"\tif match error {CSS_OK => false, _ => true} {\n";
    output.push_str("\t\t*ctx = orig_ctx;\n\t}\n");
    output.push_str(" \n");
    output.push_str("\t//debug!(\"error == %? (2)\" , error)\n");
    output.push_str("\treturn error\n");
    output.push_str("}\n\n");
    fp.write_str(output);
}

pub fn output_wrap(fp:@Writer, parseid:&keyval, WRAP:~[keyval]) {
    
    fp.write_str(fmt!(" return %s(_stylesheet_vector, _sheet, lwc_ref, strings, vector, ctx, result, %s)\n}\n",WRAP[0].val,parseid.val));
}


fn main() {

    //let args = os::args();

    let str_INHERIT = ~"INHERIT";

    let ident_inherit = keyval{ key:str_INHERIT, val:~""};
    
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
    //     println(fmt!("Usage: %s [-o <filename>] <descriptor>\n", args[0]));
    //     fail!(~"Invalid Usage");
    // }

    // if args[1] == ~"--o" {
        
    //     if (args.len() != 4) {
    //         println(fmt!("Usage: %s [-o <filename>] <descriptor>\n", args[0]));
    //         fail!();
    //     }

    //     let path = Path("args[2]");
    //     writer = file_writer(&path,[Create, io::Append]).get();
    //     descriptor = copy args[3]
    // } 
    // else {
    //     writer = io::stdout();
    //     descriptor = copy args[1]
    // }
       

    let input_path = Path("properties.gen");
    let reader = &file_reader(&input_path).unwrap();
    let output_path = Path("autogenerated.rs");
    writer = file_writer(&output_path,[Create]).unwrap();
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

        match get_keyval(descriptor.clone()) {
            None => {
                println(fmt!("Token error at offset %u\n", curpos));
                fail!();
            }

            Some(rkv_list)  =>  
                for rkv in rkv_list.iter() {
                    if rkv.clone().key == ~"WRAP" {
                        wrap.push(rkv.clone());
                        only_ident = false;
                    } 
                    else if rkv.clone().key == ~"NUMBER" {
                        if rkv.clone().val.char_at( 0) == '(' {
                            curlist = NUMBER;
                        } 
                        else if rkv.clone().val.char_at( 0) == ')' {
                            curlist = BASE;
                        } 
                        else {
                            number.push(rkv.clone());
                        }
                        only_ident = false;
                    } 
                    else if rkv.clone().key == ~"IDENT" {
                        if rkv.clone().val.char_at( 0) == '(' {
                            curlist = IDENT;
                        } 
                        else if rkv.clone().val.char_at( 0) == ')' {
                            curlist = BASE;
                        } 
                        else if rkv.clone().val ==  ~"INHERIT" {
                            ident.push(ident_inherit.clone());
                        }
                    } 
                    else if rkv.clone().key == ~"IDENT_LIST" {
                        if rkv.clone().val.char_at( 0) == '(' {
                            curlist = IDENT_LIST;
                        } 
                        else if rkv.clone().val.char_at( 0) == ')' {
                            curlist = BASE;
                        } 
                    } 
                    else if rkv.clone().key == ~"LENGTH_UNIT" {
                        if rkv.clone().val.char_at( 0) == '(' {
                            curlist = LENGTH_UNIT;
                        } 
                        else if rkv.clone().val.char_at( 0) == ')' {
                            curlist = BASE;
                        } 
                        only_ident = false;
                        do_token_check = false;
                    } 
                    else if rkv.clone().key == ~"COLOR" {
                        color.push(rkv.clone());
                        do_token_check = false;
                        only_ident = false;
                    } 
                    else if rkv.clone().key == ~"URI" {
                        uri.push(rkv.clone());
                        only_ident = false;
                    } 
                    else if rkv.clone().key == ~"GENERIC" {
                        is_generic = true;
                    } 
                    else {
                        /* just append to current list */
                        match curlist {
                            NUMBER => number.push(rkv.clone()), 
                            BASE => base.push(rkv.clone()),
                            IDENT => ident.push(rkv.clone()),
                            IDENT_LIST => ident_list.push(rkv.clone()),
                            LENGTH_UNIT => length_unit.push(rkv.clone())
                        }           
                    }
                }
        }

        if base.len() != 1 {
            println(fmt!("Incorrect base element count (got %u expected 1)\n", base.len()));
            fail!();
        }


        /* header */
        function_header(writer, descriptor, &base[0], is_generic);

        if !wrap.is_empty() {
            output_wrap(writer, &base[0], wrap);
        }
        else {
            /* check token type is correct */
            output_token_type_check(writer, do_token_check,  ident.clone(), uri.clone(), number.clone());

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

