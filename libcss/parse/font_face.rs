// ===========================================================================================================
// CSS-FONT-FACE implementation/data-structs starts here 
// ===========================================================================================================

use std::arc;
use wapcaplet::*;

use include::font_face::*;
use include::properties::*;

use lex::lexer::*;

use parse::propstrings::*;
use parse::properties::common::*;
use parse::properties::properties::*;

use stylesheet::*;

use utils::errors::*;

pub fn font_rule_font_family_reserved(strings:&mut ~css_propstrings, ident:&@css_token) -> bool {
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), SERIF as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(),SANS_SERIF as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), CURSIVE as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), FANTASY as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), MONOSPACE as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), INHERIT as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), INITIAL as uint) ||
    strings.lwc_string_caseless_isequal(ident.idata.get_ref().clone(), DEFAULT as uint)
}

pub fn font_face_parse_font_family(sheet:@mut css_stylesheet, strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint, 
    font_face:@mut css_font_face, lwc_instance:arc::RWARC<~lwc>) -> css_error {
    
    match css__ident_list_or_string_to_string(sheet , strings , vector, ctx, Some(@font_rule_font_family_reserved)) {
        (CSS_OK,Some(string)) => { 
            css__font_face_set_font_family(font_face, string, lwc_instance);
            return CSS_OK
        },
        (error,_) => return error
    }
    
}


/**
* #Description:
*   Initialise a selector detail.
* #Arguments:
*  'descriptor' - Token for this descriptor.
*  'strings' - css propstrings.
*  'vector' - Vector of tokens to process.
*  'ctx' - Pointer to vector iteration context.
*  'curRule' - Rule to process descriptor into.
*  'lwc_instance' - .
* #Return Value:
*   'css_error' - CSS_OK on success,
          CSS_BADPARM on bad parameters,
          CSS_INVALID on invalid syntax,
          appropriate error otherwise..
*/
pub fn css__parse_font_descriptor(sheet:@mut css_stylesheet, descriptor:&@css_token, strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint, 
    curRule:@mut css_rule_font_face, lwc_instance:arc::RWARC<~lwc>) -> css_error {
    
    let mut font_face:Option<@mut css_font_face> = curRule.font_face;

    if !font_face.is_some() {
        curRule.font_face = Some(@mut css_font_face {
            font_family: None,
            srcs:~[],
            bits:~[((CSS_FONT_WEIGHT_NORMAL as int << 2 ) as int| (CSS_FONT_STYLE_NORMAL) as int) as u8]
        });
    }

    font_face = curRule.font_face;

    if strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(), FONT_FAMILY as uint) {
        return font_face_parse_font_family(sheet, strings, vector, ctx, *font_face.get_ref(), lwc_instance)
    }
    else if strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(),SRC as uint) {
        return font_face_parse_src(sheet, strings, vector, ctx, *font_face.get_ref(), lwc_instance)
    }
    else if strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(),FONT_STYLE as uint) {
        return font_face_parse_font_style(strings, vector, ctx, *font_face.get_ref())
    }
    else if strings.lwc_string_caseless_isequal(descriptor.idata.get_ref().clone(),FONT_WEIGHT as uint) {
        return font_face_parse_font_weight(strings, vector, ctx, *font_face.get_ref())
    }
    
    CSS_INVALID
}   



pub fn font_face_parse_src(sheet:@mut css_stylesheet, strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint,
    font_face:@mut css_font_face, lwc_instance:arc::RWARC<~lwc>) -> css_error {

    let orig_ctx = *ctx;
    let mut srcs:~[~css_font_face_src]=~[];
            
    /* src             ::= spec-or-name [ ',' spec-or-name ]*
     * spec-or-name    ::= font-face-spec | font-face-name
     * font-face-spec  ::= URI [ 'format(' STRING [ ',' STRING ]* ')' ]?
     * font-face-name  ::= 'local(' ident-list-or-string ')'
     * ident-list-or-string ::= IDENT IDENT* | STRING
     */

    /* Create one css_font_face_src for each consecutive location and
     * [potentially] type pair in the comma-separated list
     */
    loop {  
        
        let location_type:@mut css_font_face_location_type =@mut CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED;
        let format:@mut css_font_face_format =@mut CSS_FONT_FACE_FORMAT_UNSPECIFIED;
        let mut new_src:~css_font_face_src =~css_font_face_src{location:None, bits:~[]};            

        match font_face_src_parse_spec_or_name(sheet, strings, vector, ctx, location_type, format, lwc_instance.clone()) {
            (CSS_OK,location) => {
                new_src.location = location;
                new_src.bits.push(*format as u8 << 2 | *location_type as u8 );
                srcs.push(new_src);
            },
            (error,_) => {
                *ctx = orig_ctx;
                return error
            }   
        }
        
                            
        consumeWhitespace(vector, ctx);
        if *ctx < vector.len() && tokenIsChar(&vector[*ctx], ',')
        {
            *ctx += 1 //Iterate
        } 
        else
        {
            *ctx += 1; //Iterate
            break
        }
    
    }

    match css__font_face_set_srcs(font_face, srcs) {
        CSS_OK => return CSS_OK, 
        error => {
            *ctx = orig_ctx;
            return error
        } 
    }
    
}

pub fn font_face_parse_font_style(strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint,
    font_face:@mut css_font_face) -> css_error {

    let orig_ctx = *ctx;
    let style:css_font_style_e;
    

    /* IDENT(normal, italic, oblique) */

    if *ctx > vector.len() || match vector[*ctx].token_type {CSS_TOKEN_IDENT => false, _ => true} {
        *ctx = orig_ctx;
        return CSS_INVALID
    }   
    
    let token = &vector[*ctx];
    *ctx += 1;

    if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), NORMAL as uint) {
        style = CSS_FONT_STYLE_NORMAL;
    } 
    else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), ITALIC as uint) {
        style = CSS_FONT_STYLE_ITALIC
    }
    else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), OBLIQUE as uint) {
        style = CSS_FONT_STYLE_OBLIQUE
    } else {
        *ctx = orig_ctx;
         return CSS_INVALID;
    }

    
    font_face.bits[0] = (font_face.bits[0] & 0xfc) | style as u8;
    return CSS_OK;
}

pub fn font_face_parse_font_weight(strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint,
    font_face:@mut css_font_face) -> css_error {

    let orig_ctx = *ctx;
    let token:&@css_token;
    let weight:css_font_weight_e;
    
    /* NUMBER (100, 200, 300, 400, 500, 600, 700, 800, 900) | 
     * IDENT (normal, bold) */
    
    if *ctx >= vector.len() || match vector[*ctx].token_type { CSS_TOKEN_IDENT | CSS_TOKEN_NUMBER => false, _ => true } {
        return CSS_INVALID;
    }

    token = &vector[*ctx];
    *ctx += 1;


    if match token.token_type { CSS_TOKEN_NUMBER => true, _ => false }  {
        let (num, consumed) = css__number_from_lwc_string(token.idata.get_ref().clone(), true);
        /* Invalid if there are trailing characters */
        if consumed != lwc_string_length(token.idata.get_ref().clone()) {
            *ctx = orig_ctx;
            return CSS_INVALID;
        }

        match num >> 10 {
            100 => weight = CSS_FONT_WEIGHT_100,
            200 => weight = CSS_FONT_WEIGHT_200,
            300 => weight = CSS_FONT_WEIGHT_300,
            400 => weight = CSS_FONT_WEIGHT_400,
            500 => weight = CSS_FONT_WEIGHT_500,
            600 => weight = CSS_FONT_WEIGHT_600,
            700 => weight = CSS_FONT_WEIGHT_700,
            800 => weight = CSS_FONT_WEIGHT_800,
            900 => weight = CSS_FONT_WEIGHT_900,
            _ =>  {
                *ctx = orig_ctx;
                return CSS_INVALID
            }
        }   
    } 
    else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),NORMAL as uint) {
        weight = CSS_FONT_WEIGHT_NORMAL
    } 
    else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),BOLD as uint) {
        weight = CSS_FONT_WEIGHT_BOLD
    } else {
        *ctx = orig_ctx;
        return CSS_INVALID
    }

    font_face.bits[0] = (font_face.bits[0] & 0xc3) | (weight as u8 << 2);
    
    return CSS_OK;
}

pub fn font_face_src_parse_spec_or_name(sheet:@mut css_stylesheet, strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint, 
    location_type:@mut css_font_face_location_type, format:@mut css_font_face_format, lwc_instance:arc::RWARC<~lwc>) -> (css_error, Option<arc::RWARC<~lwc_string>>) {

    let mut token: &@css_token;
    let mut location:Option<arc::RWARC<~lwc_string>> = None;
    /* spec-or-name    ::= font-face-spec | font-face-name
     * font-face-spec  ::= URI [ 'format(' STRING [ ',' STRING ]* ')' ]?
     * font-face-name  ::= 'local(' ident-list-or-string ')'
     * ident-list-or-string ::= IDENT IDENT* | STRING
     */

    consumeWhitespace(vector, ctx);

    if *ctx >= vector.len() {
        return (CSS_INVALID,None);  
    } 
    token = &vector[*ctx];
    *ctx += 1;  //Iterate

    match token.token_type {
        CSS_TOKEN_URI => {
            match (*sheet.resolve)(copy sheet.url, token.idata.get_ref().clone())
            { 
                (CSS_OK,loc) => location =loc,
                (error,_) => return (error,None)
            }   

            *location_type = CSS_FONT_FACE_LOCATION_TYPE_URI;

            consumeWhitespace(vector, ctx);

            if *ctx < vector.len() {
                
                token = &vector[*ctx];
                if match token.token_type { CSS_TOKEN_FUNCTION => true, _ => false}  &&
                    strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),FORMAT as uint) {
                
                    *ctx += 1;

                    match font_face_src_parse_format(strings, vector, ctx, format) {
                        CSS_OK => {},
                        error => {
                            return (error,None);
                        }   
                    }
                }
            }       
        },
        CSS_TOKEN_FUNCTION => {
            if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LOCAL as uint) {
                consumeWhitespace(vector, ctx);

                match css__ident_list_or_string_to_string(sheet , strings , vector, ctx, None) {
                    (CSS_OK,Some(loc)) => {
                        do lwc_instance.clone().write |lwc| {
                            location = Some(lwc.lwc_intern_string(copy loc));
                        }
                    },
                    (error,_) => return (error,None)
                }
                consumeWhitespace(vector, ctx);

                if *ctx >= vector.len() || !tokenIsChar(&vector[*ctx], ')') {
                    *ctx +=1; //Iterate
                    return (CSS_INVALID, None)
                }

                *ctx +=1; //Iterate
                *location_type = CSS_FONT_FACE_LOCATION_TYPE_LOCAL
            } 
            else {
                return (CSS_INVALID, None)
            }
        },
        _ => return (CSS_INVALID, None)     
    }

    return (CSS_OK, location)
}

/**
* #Description:
*   Set a font-faces array of srcs.

* #Arguments:
*  'font_face' - The font-face .

*  'srcs' - The vector of css_font_face_srcs.

* #Return Value:
*   'css_error' - CSS_OK .
*/
pub fn css__font_face_set_srcs(font_face:@mut css_font_face, srcs:~[~css_font_face_src]) -> css_error {
    font_face.srcs = srcs;
            
    CSS_OK
}

/**
* #Description:
*   Set a font-face's font-family name.

* #Arguments:
*  'font_face' - The font-face .

*  'font_family' - Font-family name.

* #Return Value:
*   'css_error' - CSS_OK .
*/
pub fn css__font_face_set_font_family(font_face: @mut css_font_face, font_family:~str, lwc_instance:arc::RWARC<~lwc>) -> css_error {
    // if (font_face.font_family != NULL)
    do lwc_instance.write |l|{
        font_face.font_family = Some(l.lwc_intern_string(copy font_family))
    }    
    // font_face.font_family = Some(font_family.clone());

    return CSS_OK
}



pub fn font_face_src_parse_format(strings:&mut ~css_propstrings, vector:&~[@css_token], ctx:@mut uint, format:@mut  css_font_face_format) -> css_error {
    
    let mut token:&@css_token;

    *format = CSS_FONT_FACE_FORMAT_UNSPECIFIED;

    /* 'format(' STRING [ ',' STRING ]* ')' 
     * 
     * 'format(' already consumed
     */

    loop {
        consumeWhitespace(vector, ctx);

        if  *ctx > vector.len() || match vector[*ctx].token_type { CSS_TOKEN_STRING => false, _ => true} {
            return CSS_INVALID
        } 
            
        token =&vector[*ctx];
        *ctx +=1;   //Iterate

        if strings.lwc_string_isequal(token.idata.get_ref().clone(), WOFF as uint) {
            *format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_WOFF as uint) }
        } 
        else if strings.lwc_string_isequal(token.idata.get_ref().clone(),TRUETYPE as uint) ||
            strings.lwc_string_isequal(token.idata.get_ref().clone(),OPENTYPE as uint) {
            *format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_OPENTYPE as uint) }
        } 
        else if strings.lwc_string_isequal(token.idata.get_ref().clone(), EMBEDDED_OPENTYPE as uint) {
            *format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE as uint) }
        }
        else if strings.lwc_string_isequal(token.idata.get_ref().clone(),SVG as uint) {
            *format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_SVG as uint) }    
        } 
        else {
            /* The spec gives a list of possible strings, which 
             * hints that unknown strings should be parse errors,
             * but it also talks about "unknown font formats",
             * so we treat any string we don't know not as a parse
             * error, but as indicating an "unknown font format".
             */
            *format = unsafe { cast::transmute(*format as uint | CSS_FONT_FACE_FORMAT_UNKNOWN as uint) }        
            
        }

        consumeWhitespace(vector, ctx);
        if *ctx >= vector.len() {
            *ctx += 1;
            return CSS_INVALID
        }
        else{
            token = &vector[*ctx];
            *ctx += 1;
            if !tokenIsChar(token, ',') {
                break;
            }
        }       
    } 
    
    if !tokenIsChar(token, ')') {
        return CSS_INVALID
    }   

    return CSS_OK
}

pub fn css_font_face_get_font_family(font_face: @mut css_font_face) -> Option< arc::RWARC<~lwc_string> > {
    if (font_face.font_family.is_some()) {
        let ff = font_face.font_family.swap_unwrap();

        font_face.font_family = Some(ff.clone());
        Some(ff)
    }
    else {
        None
    }
}

pub fn css_font_face_font_style(font_face: @mut css_font_face) -> css_font_style_e {
    unsafe {
        return cast::transmute((font_face.bits[0] & 0x3) as uint);
    }
}

pub fn css_font_face_font_weight(font_face: @mut css_font_face) -> css_font_weight_e {
    unsafe {
        return cast::transmute(((font_face.bits[0] >> 2) & 0xf) as uint);
    }
}

pub fn css_font_face_count_srcs(font_face: @mut css_font_face) -> uint {
    unsafe {
        return (&mut font_face.srcs).len();
    }
}

pub fn css_font_face_get_src(font_face: @mut css_font_face, index: uint) -> Option<~css_font_face_src> {
    unsafe {
        if (index > css_font_face_count_srcs(font_face)) {
            return None;
        }
    }

    let src = &mut font_face.srcs[index];
    
    let new_src = ~css_font_face_src {
        location: match src.location {
            None => None,
            Some(_) => {
                unsafe {
                    let new_location = src.location.get_ref();
                    Some(new_location.clone())
                }
            }
        },

        bits: copy src.bits
    };

    Some(new_src)
}

pub fn css_font_face_src_get_location(src: & ~css_font_face_src) -> Option<arc::RWARC<~lwc_string>> {
    match src.location {
        None => None,
        Some(_) => {
            unsafe {
                let new_location = src.location.get_ref();
                Some(new_location.clone())
            }
        }
    }
}

pub fn css_font_face_src_location_type(src: &~ css_font_face_src) -> css_font_face_location_type {
    unsafe {
        return cast::transmute((src.bits[0] & 0x3) as uint);
    }
}

pub fn css_font_face_src_format(src: &~ css_font_face_src) -> css_font_face_location_type {
    unsafe {
        return cast::transmute((src.bits[0] & 0x1f) as uint);
    }
}