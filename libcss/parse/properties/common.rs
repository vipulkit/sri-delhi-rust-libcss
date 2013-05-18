use wapcaplet::*;
use std::arc;

use bytecode::bytecode::*;
use bytecode::opcodes::*;

use lex::lexer::*;
use stylesheet::*;

use include::fpmath::*;
use include::properties::*;

use parse::propstrings::*;

use utils::errors::*;

pub fn consumeWhitespace(vector:&~[@css_token], ctx:@mut uint) {
    loop {
        if *ctx < vector.len() {
            match vector[*ctx].token_type {
                CSS_TOKEN_S => {
                    *ctx = *ctx+1
                },
                _ => return
            }
        }
        else {
            break
        }
    }
}

pub fn css__parse_unit_specifier(sheet: @mut css_stylesheet, vector: &~[@css_token] , ctx: @mut uint , default_unit: u32) -> (Option<int> , Option<u32>, css_result) {

    consumeWhitespace(vector , ctx);
    let mut token:&@css_token;
    let mut unit_retVal:u32;
    let orig_ctx = *ctx;

    if *ctx >= vector.len() {
        return (None , None , CSS_INVALID)
    }
    token = &vector[*ctx];
    *ctx = *ctx + 1;

    match token.token_type {
        CSS_TOKEN_DIMENSION(_ , _ , _)|CSS_TOKEN_NUMBER(_ , _)|CSS_TOKEN_PERCENTAGE(_ , _) => {},
        _ => {
            *ctx = orig_ctx;
            return(None , None , CSS_INVALID);
        }
    }

    let (num , consumed_index) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);

    match token.token_type {
        CSS_TOKEN_DIMENSION(_ , _ , _) => {
            // let len = lwc_string_length(token.idata.get_ref().clone());
            let data = lwc_string_data(token.idata.get_ref().clone());

            let (unit , result) = css__parse_unit_keyword(data , consumed_index);
            match result {
                CSS_OK => {},
                _ => {
                    *ctx = orig_ctx;
                    return (None , None , result);
                }
            }
            unit_retVal = unit.unwrap() as u32;
        },
        CSS_TOKEN_NUMBER(_ , _) => {
            if num !=0 {
                if sheet.quirks_allowed {
                    sheet.quirks_used = true;
                }
                else {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
            }
            unit_retVal = default_unit;
            if sheet.quirks_allowed {
                let tmp_ctx = ctx;
                consumeWhitespace(vector , tmp_ctx);
                token = &vector[*tmp_ctx];
                *tmp_ctx = *tmp_ctx + 1;

                match token.token_type {
                    CSS_TOKEN_IDENT(_) => {
                        let (unit , result) = css__parse_unit_keyword(lwc_string_data(token.idata.get_ref().clone()) , 0);
                        match  result {
                            CSS_OK => {
                                sheet.quirks_used = true;
                                *ctx = *tmp_ctx;
                                unit_retVal = unit.unwrap() as u32;
                            },
                            _=> {}
                        };
                    },
                    _ => {}
                };
            }
        },
        //CSS_TOKEN_PERCENTAGE
        _ => {
            if lwc_string_length(token.idata.get_ref().clone()) != consumed_index {
                return (None , None , CSS_INVALID);
            }
            unit_retVal = UNIT_PCT as u32;
        }
    }
    return(Some(num) , Some(unit_retVal) , CSS_OK);
}

pub fn css__number_from_lwc_string(string: arc::RWARC<~lwc_string>, int_only: bool) -> (int , uint) {
    let mut ret_value = 0;
    let mut consumed_length = 0;

    if lwc_string_length(string.clone()) == 0 {
        return (ret_value , consumed_length);
    }
    css__number_from_string(lwc_string_data(string.clone()), 0, int_only)
}

pub fn css__parse_border_side(sheet: @mut css_stylesheet, strings: &mut ~css_propstrings , vector: &~[@css_token] , ctx: @mut uint , result_style: @mut css_style , side: border_side_e) -> css_result { 
    let orig_ctx = *ctx;
    let mut prev_ctx: uint;
    let color: bool = true;
    let style: bool = true;
    let width: bool = true;
    let color_style: @mut css_style;
    let style_style: @mut css_style;
    let width_style: @mut css_style;
    let mut token: &@css_token;

    if *ctx >= vector.len() {
        return CSS_INVALID;
    }

    token = &vector[*ctx];
    
    if (is_css_inherit(strings , token)) {
        css_stylesheet::css_stylesheet_style_inherit(result_style , unsafe{cast::transmute(CSS_PROP_BORDER_TOP_COLOR as uint + side as uint)});
        css_stylesheet::css_stylesheet_style_inherit(result_style, unsafe{cast::transmute(CSS_PROP_BORDER_TOP_STYLE as uint + side as uint)});
        css_stylesheet::css_stylesheet_style_inherit(result_style, unsafe{cast::transmute(CSS_PROP_BORDER_TOP_WIDTH as uint + side as uint)});
    }
    
    *ctx = *ctx + 1;
    color_style = sheet.css__stylesheet_style_create();
    style_style = sheet.css__stylesheet_style_create();
    width_style = sheet.css__stylesheet_style_create();

    prev_ctx = *ctx;
    while *ctx != prev_ctx {
        let mut error = CSS_OK;
        token = &vector[*ctx];
        if is_css_inherit(strings , token) {
            error = CSS_INVALID;
        }
    }
    CSS_OK
}

pub fn css__parse_unit_keyword(ptr:~str , index: uint)-> (Option<u32>,css_result) {
    let mut unit = UNIT_GRAD;
    let len:uint= ptr.len() - index;
    let ptr_lower = ptr.to_lower();
    match(len) {
        4=>if (ptr_lower == ~"grad") {
              unit= UNIT_GRAD;    
            },
        3=>{
            if (ptr_lower == ~"KHz") {
                unit= UNIT_KHZ;    
            }
            else if (ptr_lower == ~"deg") {
                unit= UNIT_DEG;    
            }
            else if (ptr_lower == ~"rad") {
                unit= UNIT_RAD;    
            }
            else {
                return (None,CSS_INVALID);
            }
        },
        2=>{
            if (ptr_lower == ~"Hz") {
                unit= UNIT_HZ;    
            }
            else if (ptr_lower == ~"ms") {
                unit= UNIT_MS;    
            }
            else if (ptr_lower == ~"px") {
                unit= UNIT_PX;    
            }
            else if (ptr_lower == ~"ex") {
                unit= UNIT_EX;    
            }
            else if (ptr_lower == ~"em") {
                unit= UNIT_EM;    
            }
            else if (ptr_lower == ~"in") {
                unit= UNIT_IN;    
            }
            else if (ptr_lower == ~"cm") {
                unit= UNIT_CM;    
            }
            else if (ptr_lower == ~"mm") {
                unit= UNIT_MM;    
            }
            else if (ptr_lower == ~"pt") {
                unit= UNIT_PT;    
            }
            else if (ptr_lower == ~"pc") {
                unit= UNIT_PC;    
            }
            else {
                return (None,CSS_INVALID);
            }
        },
        1=>{
            if (ptr_lower == ~"s") {
                unit= UNIT_S;    
            }
            else {
                return (None,CSS_INVALID);
            }
        },
        _=>{
            return (None,CSS_INVALID);
        }

    }
    (Some(unit) , CSS_OK)
}

pub fn css__number_from_string(data: ~str, data_index: uint, int_only: bool) -> (int , uint){

    let mut length = data.len();
    // let mut ptr = copy data;
    let mut sign = 1;
    let mut intpart: i32 = 0;
    let mut fracpart: i32 = 0;
    let mut pwr: i32 = 1;
    let mut ret_value = 0;
    let mut index = 0;
    let mut consumed_length = 0;
    

    if length - data_index ==0 {
        return (ret_value , consumed_length);
    }

    // number = [+-]? ([0-9]+ | [0-9]* '.' [0-9]+) 

    // Extract sign, if any 
    if data[0 + data_index] == '-' as u8 {
        sign = -1;
        length -= 1;
        index += 1;
    }
    else if data[0 + data_index] == '+' as u8 {
        length -=1;
        index += 1;
    }

    if length == 0 {
        return (ret_value , consumed_length);
    }
    else {
        if data[0 + data_index] == '.' as u8 {
            if length ==1 || (data[1 + data_index] < ('0' as u8)) || (('9' as u8) < data[1 + data_index]) {
                return (ret_value , consumed_length);
            }
        }
        else if (data[0 + data_index] < ('0' as u8)) || (('9' as u8) < data[0 + data_index]) {
            return (ret_value , consumed_length);
        }
    }

    while length>0 {
        if (data[0 + data_index] < ('0' as u8))||(('9' as u8) < data[0 + data_index]) {
            break
        }
        if intpart < (1<<22) {
            intpart *= 10;
            intpart += (data[0 + data_index] as i32) - ('0' as i32);
        }
        index += 1;
        length -= 1;
    }

    if int_only == false && length > 1 && (data[0 + data_index] == '.' as u8) && ('0' as u8 <= data[1 + data_index] && data[1 + data_index] <= '9' as u8) {
        index += 1; 
        length -= 1;

        while length >0 {
            if ((data[0 + data_index] < '0' as u8))|| (('9' as u8) < data[0 + data_index]) {
                break
            }

            if pwr < 1000000 {
                pwr *= 10;
                fracpart *= 10;
                fracpart += (data[0 + data_index] - '0' as u8) as i32;
            }
            index += 1;
            length -= 1;
        }
        fracpart = ((1 << 10) * fracpart + pwr/2) / pwr;
        if fracpart >= (1 << 10) {
            intpart += 1;
            fracpart &= (1 << 10) - 1;
        }
    }

    consumed_length = index;

    if sign > 0 {
        if intpart >= (1 << 21) {
            intpart = (1 << 21) - 1;
            fracpart = (1 << 10) - 1;
        }
    }
    else {
         // If the negated result is smaller than we can represent then clamp to the minimum value we can store. 
        if intpart >= (1 << 21) {
            intpart = -(1 << 21);
            fracpart = 0;
        }
        else {
            intpart = -intpart;
            if fracpart > 0 {
                fracpart = (1 << 10) - fracpart;
                intpart -= 1;
            }
        }
    }
    ret_value = ((intpart << 10) | fracpart )as int;
    (ret_value , consumed_length)

}

pub fn is_css_inherit(strings: &mut ~css_propstrings , token: &@css_token) ->bool {
    match token.token_type {
        CSS_TOKEN_IDENT(_) => {
             return strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , INHERIT as uint);
        }
        _ => false
    }
}

pub fn css__parse_color_specifier(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings , vector: &~[@css_token] , ctx: @mut uint) -> (Option<u16> , Option<u32> , css_result) {
    let mut token:&@css_token;
    let mut ret_value: u16 = 0;
    let mut ret_result: u32 = 0;
    let mut goto_flag = false;
    let orig_ctx = *ctx;

    consumeWhitespace(vector , ctx);
    if *ctx >= vector.len() {
        return (None , None , CSS_INVALID)
    }
    token = &vector[*ctx];
    *ctx = *ctx + 1;

    match token.token_type {
        CSS_TOKEN_IDENT(_) => {
            if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , TRANSPARENT as uint) {
                ret_value = COLOR_TRANSPARENT ;
                ret_result = 0;
                return (Some(ret_value) , Some(ret_result) , CSS_OK);
            }
            else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , CURRENTCOLOR as uint) {
                ret_value = COLOR_CURRENT_COLOR ;
                ret_result = 0;
                return (Some(ret_value) , Some(ret_result) , CSS_OK);
            }
            let (color_value , error) = css__parse_named_color(sheet , strings , token.idata.get_ref().clone());
            match error {
                CSS_OK => {},
                _ => {
                    if sheet.quirks_allowed {
                        let(hash_result , error_from_hash) = css__parse_hash_colour(token.idata.get_ref().clone());
                        match error_from_hash {
                            CSS_OK => sheet.quirks_used = true,
                            _ => {
                                goto_flag = true;
                            }
                        }
                    }
                    else {
                        goto_flag = true;
                    }
                }
            }
        },

        CSS_TOKEN_HASH(_) => {
            let(hash_result , error_from_hash) = css__parse_hash_colour(token.idata.get_ref().clone());
            match error_from_hash {
                CSS_OK => {},
                _ => {
                    goto_flag = true;
                }
            }
        },
        CSS_TOKEN_FUNCTION(_) => {
            let mut r: u8 = 0;
            let mut g: u8 = 0;
            let mut b: u8 = 0;
            let mut a: u8 = 0xff;
            let mut colour_channels: int = 0;
            if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RGB as uint) {
                colour_channels = 3;
            }
            else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RGBA as uint) {
                colour_channels = 4;
            }
            else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), HSL as uint) {
                colour_channels = 5;
            }
            else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), HSLA as uint) {
                colour_channels = 6;
            }

            if colour_channels ==3 || colour_channels == 4 {
                let mut i: int =0;
                let mut valid: Option<css_token_type> = None;
                let components: ~[u8] = ~[r , g , b , a];
                let mut component: u8;
                while i < colour_channels {
                    
                    let mut intval: i32;
                    let mut int_only: bool;

                    component = components[i];
                    consumeWhitespace(vector , ctx);

                    token = &vector[*ctx];
                    match token.token_type {
                        CSS_TOKEN_NUMBER(_ , _) => {},
                        CSS_TOKEN_PERCENTAGE(_ , _) => {},
                        _ => {
                            goto_flag = true;
                        }
                    }
                    if i==0 {

                        valid = Some(copy token.token_type);
                    }

                     else if (
                        i<3 &&
                        match token.token_type {
                            CSS_TOKEN_NUMBER(_,_) => false,
                            _=>true
                        }
                        ) {
                        goto_flag = true;
                     }

                   

                    if i<3 {
                        int_only = match valid {
                         Some(CSS_TOKEN_NUMBER(_ , _) )=> true,
                         _=> false
                        };
                    }
                    else {
                        int_only = false;
                    }
                    let (num , consumed_index) = css__number_from_lwc_string(token.idata.get_ref().clone() , int_only);

                    if consumed_index != lwc_string_length(token.idata.get_ref().clone()) {
                        goto_flag = true;
                    }
                     match valid {
                        Some(CSS_TOKEN_NUMBER(_,_))=>{
                            if (i==3) {
                                intval = css_multiply_fixed((num as i32), F_255 as i32)>> CSS_RADIX_POINT;
                                //FIXTOINT(FMUL(num, F_255));
                            }
                            else {
                                intval = num as i32 >> CSS_RADIX_POINT;
                                //FIXTOINT(num);
                            }
                        },
                        _=>{
                            intval = (css_divide_fixed(css_multiply_fixed((num as i32), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
                            //FIXTOINT(FDIV(FMUL(num, F_255), F_100));
                        }
                     }

                    if intval > 255 {
                        component = 255;
                    }
                    else if intval < 0 {
                        component = 0;
                    }
                    else {
                        component = intval as u8;
                    }

                    *ctx = *ctx + 1;
                    consumeWhitespace(vector , ctx);

                    token = &vector[*ctx];
                     if *ctx >= vector.len() {
                        goto_flag = true;
                     }
                    if (i != (colour_channels - 1) && tokenIsChar(token , ',')) {
                        *ctx = *ctx + 1;
                    }
                    else if (i == (colour_channels - 1) && tokenIsChar(token , ')')) {
                        *ctx = *ctx + 1;
                    }
                    else {
                        goto_flag = true;
                    }
                    i = i + 1;
                }
            }
            else if colour_channels == 5 || colour_channels == 6 {
                let mut hue: i32;
                let mut sat: i32;
                let mut lit: i32;
                let mut alpha: i32 = 255;

                consumeWhitespace(vector , ctx);

                token = &vector[*ctx];
                *ctx = *ctx + 1;
                if *ctx >= vector.len() {
                    goto_flag = true
                }
                match token.token_type {
                    CSS_TOKEN_NUMBER(_ , _) => {},
                    _ => goto_flag = true
                }
                let mut (hue_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
                hue = hue_res as i32;
                if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                    goto_flag = true;
                }
                while hue < 0 {
                    hue += F_360 as i32;
                }
                while hue >= F_360 as i32 {
                    hue -= F_360 as i32;
                }

                consumeWhitespace(vector , ctx);
                
                token = &vector[*ctx];
                *ctx = *ctx + 1;
                if *ctx >= vector.len() {
                    goto_flag = true
                }
                if !tokenIsChar(token , ',') {
                    goto_flag = true;
                }

                consumeWhitespace(vector , ctx);
                
                token = &vector[*ctx];
                *ctx = *ctx + 1;
                if *ctx >= vector.len() {
                    goto_flag = true
                }
                match token.token_type {
                    CSS_TOKEN_PERCENTAGE(_ , _) => {},
                    _ => {
                        goto_flag = true
                    }
                }
                let mut (sat_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
                sat = sat_res as i32;
                if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                    goto_flag = true;
                }

                if sat < css_int_to_fixed(0) {
                    sat = css_int_to_fixed(0);
                }
                else if sat > css_int_to_fixed(100) {
                    sat = css_int_to_fixed(100);
                }

                consumeWhitespace(vector, ctx);

                token = &vector[*ctx];
                *ctx = *ctx + 1;

                if !tokenIsChar(token , ',') {
                    goto_flag = true;
                }

                consumeWhitespace(vector , ctx);

                token = &vector[*ctx];
                *ctx = *ctx + 1;

                match token.token_type {
                    CSS_TOKEN_PERCENTAGE(_ , _) => {},
                    _ => {
                        goto_flag = true
                    }
                }
                let mut (lit_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
                lit = lit_res as i32;
                if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                    goto_flag = true;
                }

                if lit < css_int_to_fixed(0) {
                    lit = css_int_to_fixed(0);
                }
                else if lit > css_int_to_fixed(100) {
                    lit = css_int_to_fixed(100);
                }

                consumeWhitespace(vector , ctx);

                token = &vector[*ctx];
                *ctx = *ctx + 1;

                if colour_channels == 6 {
                    if !tokenIsChar(token , ',') {
                        goto_flag = true;
                    }
                    consumeWhitespace(vector , ctx);

                    token = &vector[*ctx];
                    *ctx = *ctx + 1;

                    match token.token_type {
                        CSS_TOKEN_NUMBER(_ , _) => {},
                        _ => {
                            goto_flag = true
                        }
                    }
                    let mut (alpha_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
                    alpha = alpha_res as i32;
                    if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                        goto_flag = true;
                    }

                    alpha = css_int_to_fixed(css_multiply_fixed(alpha as i32 , F_255 as i32) as int) as i32;
                    consumeWhitespace(vector , ctx);

                    token = &vector[*ctx];
                    *ctx = *ctx + 1;
                }
                if !tokenIsChar(token , ',') {
                    goto_flag = true;
                }
                let (ra , ga , ba) = HSL_to_RGB(hue as i32, sat as i32, lit as i32);
                r = ra;
                g = ga;
                b = ba;

                if alpha > 255 {
                    a = 255;
                }
                else if alpha < 0 {
                    a = 0;
                }
                else {
                    a = alpha as u8;
                }
            }
            else {
                goto_flag = true;
            }

            ret_result = (a << 24 | r << 16 | g << 8 | b) as u32;
            ret_value = COLOR_SET ;
        },
        _=>{
            if (sheet.quirks_allowed== false ||
                match token.token_type {
                    CSS_TOKEN_NUMBER(_,_)=> false,
                    CSS_TOKEN_DIMENSION(_ , _ , _) =>false,
                    _=> true,
                }) {
                goto_flag =true;
            } 
        }
    }
    if sheet.quirks_allowed {
        match token.token_type {
            CSS_TOKEN_NUMBER(_ , _) => {
                let(hash_result , error_from_hash) = css__parse_hash_colour(token.idata.get_ref().clone());
                match error_from_hash {
                    CSS_OK => {
                        sheet.quirks_used = true
                    },
                    _ => {
                        goto_flag = true;
                    }
                }
            },
            CSS_TOKEN_DIMENSION(_,_,_) => {
                let(hash_result , error_from_hash) = css__parse_hash_colour(token.idata.get_ref().clone());
                match error_from_hash {
                    CSS_OK => {
                        sheet.quirks_used = true
                    },
                    _ => {
                        goto_flag = true;
                    }
                }
            },
            _ => {
                goto_flag = true;
            }
        }
    }
    else {
        goto_flag = true;
    }

    if goto_flag {
        *ctx = orig_ctx;
        return (None , None , CSS_INVALID); 
    }
    (Some(ret_value) , Some(ret_result) , CSS_OK)
}

pub fn css__parse_hash_colour(data: arc::RWARC<~lwc_string>) -> (Option<u32> , css_result){
    let mut result_val: u32;
    let mut r: u8;
    let mut g: u8;
    let mut b: u8;
    let mut a: u8 = 0xff;
    let input_length = lwc_string_length(data.clone());
    let input_string = lwc_string_data(data.clone());

    if (input_length == 3 && isHex(input_string[0]) && isHex(input_string[1]) && isHex(input_string[2])) {
        r = charToHex(input_string[0]) as u8;
        g = charToHex(input_string[1]) as u8;
        b = charToHex(input_string[2]) as u8;

        r |= (r << 4);
        g |= (g << 4);
        b |= (b << 4);
    }
    else if (input_length == 6 && isHex(input_string[0]) && isHex(input_string[1]) &&   isHex(input_string[2]) && isHex(input_string[3]) && isHex(input_string[4]) && isHex(input_string[5])) {
        r = (charToHex(input_string[0]) << 4) as u8;
        r |= charToHex(input_string[1]) as u8;
        g = (charToHex(input_string[2]) << 4) as u8;
        g |= charToHex(input_string[3]) as u8;
        b = (charToHex(input_string[4]) << 4) as u8;
        b |= charToHex(input_string[5]) as u8;
    }
    else {
        return(None , CSS_INVALID)
    }

    result_val = ((a << 24) | (r << 16) | (g << 8) | b) as u32;

    return (Some(result_val) , CSS_OK);
}

/**
 * Determine if a token is a character
 *
 * \param token  The token to consider
 * \param c      The character to match (lowerASCII only)
 * \return True if the token matches, false otherwise
 */
pub fn tokenIsChar(token:&@css_token, c:char) -> bool {
    let result = false;

    match token.token_type {
        CSS_TOKEN_CHAR(_) => {   
                if lwc_string_length(token.idata.get_ref().clone()) == 1 {
                    let mut token_char = lwc_string_data(token.idata.get_ref().clone()).char_at(0);

                    // Ensure lowercomparison 
                    if 'A' <= token_char && token_char <= 'Z' {
                        token_char += 'a' - 'A'
                    }
                        
                    if token_char == c {
                        return true
                    }
                }                       
            },
        _ => return result
    }           
    
    return result
}


pub fn isDigit(c: u8) -> bool{
    return '0' <= (c as char) && (c as char) <= '9';
} 

pub fn isHex(c: u8) -> bool {
    return isDigit(c) || ('a' <= (c as char)&& (c as char) <= 'f') || ('A' <= (c as char) && (c as char) <= 'F');
}

pub fn charToHex(c: u8) -> u32 {
    let mut k = c;
    k -= '0' as u8;

    if (k > 9) {
        k -= ('A' as u8) - ('9' as u8) - 1;
    }

    if (k > 15) {
        k -= ('a' as u8) - ('A' as u8);
    }
    return k as u32;
}

pub fn HSL_to_RGB(hue: i32 , sat: i32 , lit: i32 ) -> (u8 , u8 , u8) {
    let min_rgb: i32;
    let max_rgb: i32;
    let chroma: i32;
    let relative_hue: i32;
    let scaled_hue: i32;
    let mid1: i32;
    let mid2: i32;
    let sextant: int;

    /* If saturation is zero there is no hue and r = g = b = lit */
    if (sat == css_int_to_fixed(0)) {
        let r = (css_divide_fixed(css_multiply_fixed((lit), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
        let g = (css_divide_fixed(css_multiply_fixed((lit), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
        let b = (css_divide_fixed(css_multiply_fixed((lit), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
        return (r as u8, g as u8, b as u8);
    }

    /* Compute max(r,g,b) */
    if (lit <= css_int_to_fixed(50)) {
        max_rgb = css_divide_fixed(css_multiply_fixed(lit, css_add_fixed(sat, F_100 as i32)), F_100 as i32);
    } 
    else {
        max_rgb = css_divide_fixed(css_subtract_fixed(css_multiply_fixed(css_add_fixed(lit, sat), F_100 as i32), css_multiply_fixed(lit, sat)), F_100 as i32);
    }

    /* Compute min(r,g,b) */
    min_rgb = css_subtract_fixed(css_multiply_fixed(lit, css_int_to_fixed(2)), max_rgb);

    /* Chroma is the difference between min and max */
    chroma = css_subtract_fixed(max_rgb, min_rgb);

    /* Compute which sextant the hue lies in (truncates result) */
    let hue_sextant = css_divide_fixed(css_multiply_fixed(hue, css_int_to_fixed(6)), F_360 as i32);
    sextant = (hue_sextant as int) >> CSS_RADIX_POINT;

    /* Compute offset of hue from start of sextant */
    relative_hue = css_subtract_fixed(hue, css_int_to_fixed(sextant));

    /* Scale offset by chroma */
    scaled_hue = css_multiply_fixed(relative_hue, chroma);

    /* Compute potential values of the third colour component */
    mid1 = css_add_fixed(min_rgb, scaled_hue);
    mid2 = css_subtract_fixed(max_rgb, scaled_hue);

    match sextant {
        0 => {
            let r = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let g = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
            let b = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            return (r as u8 , g as u8 , b as u8);
        },
        1 => {
            let r = (css_divide_fixed(css_multiply_fixed((mid2), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
            let g = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let b = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            return (r as u8 , g as u8 , b as u8);
        },
        2 => {
            let r = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let g = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let b = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
            return (r as u8 , g as u8 , b as u8);
        },
        3 => {
            let r = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let g = (css_divide_fixed(css_multiply_fixed((mid2), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
            let b = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            return (r as u8 , g as u8 , b as u8);
        },
        4 => {
            let r = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
            let g = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let b = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            return (r as u8 , g as u8 , b as u8);
        },
        5 => {
            let r = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let g = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            let b = (css_divide_fixed(css_multiply_fixed((mid2), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
            return (r as u8 , g as u8 , b as u8);
        }
        _ => { (0 , 0 , 0)}
    }
}

fn css__parse_named_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings , data: arc::RWARC<~lwc_string>) -> (Option<u32> , css_result){
    let mut result_val: u32;
    let colourmap: ~[u32] = ~[
        0xfff0f8ff, /* ALICEBLUE */
        0xfffaebd7, /* ANTIQUEWHITE */
        0xff00ffff, /* AQUA */
        0xff7fffd4, /* AQUAMARINE */
        0xfff0ffff, /* AZURE */
        0xfff5f5dc, /* BEIGE */
        0xffffe4c4, /* BISQUE */
        0xff000000, /* BLACK */
        0xffffebcd, /* BLANCHEDALMOND */
        0xff0000ff, /* BLUE */
        0xff8a2be2, /* BLUEVIOLET */
        0xffa52a2a, /* BROWN */
        0xffdeb887, /* BURLYWOOD */
        0xff5f9ea0, /* CADETBLUE */
        0xff7fff00, /* CHARTREUSE */
        0xffd2691e, /* CHOCOLATE */
        0xffff7f50, /* CORAL */
        0xff6495ed, /* CORNFLOWERBLUE */
        0xfffff8dc, /* CORNSILK */
        0xffdc143c, /* CRIMSON */
        0xff00ffff, /* CYAN */
        0xff00008b, /* DARKBLUE */
        0xff008b8b, /* DARKCYAN */
        0xffb8860b, /* DARKGOLDENROD */
        0xffa9a9a9, /* DARKGRAY */
        0xff006400, /* DARKGREEN */
        0xffa9a9a9, /* DARKGREY */
        0xffbdb76b, /* DARKKHAKI */
        0xff8b008b, /* DARKMAGENTA */
        0xff556b2f, /* DARKOLIVEGREEN */
        0xffff8c00, /* DARKORANGE */
        0xff9932cc, /* DARKORCHID */
        0xff8b0000, /* DARKRED */
        0xffe9967a, /* DARKSALMON */
        0xff8fbc8f, /* DARKSEAGREEN */
        0xff483d8b, /* DARKSLATEBLUE */
        0xff2f4f4f, /* DARKSLATEGRAY */
        0xff2f4f4f, /* DARKSLATEGREY */
        0xff00ced1, /* DARKTURQUOISE */
        0xff9400d3, /* DARKVIOLET */
        0xffff1493, /* DEEPPINK */
        0xff00bfff, /* DEEPSKYBLUE */
        0xff696969, /* DIMGRAY */
        0xff696969, /* DIMGREY */
        0xff1e90ff, /* DODGERBLUE */
        0xffd19275, /* FELDSPAR */
        0xffb22222, /* FIREBRICK */
        0xfffffaf0, /* FLORALWHITE */
        0xff228b22, /* FORESTGREEN */
        0xffff00ff, /* FUCHSIA */
        0xffdcdcdc, /* GAINSBORO */
        0xfff8f8ff, /* GHOSTWHITE */
        0xffffd700, /* GOLD */
        0xffdaa520, /* GOLDENROD */
        0xff808080, /* GRAY */
        0xff008000, /* GREEN */
        0xffadff2f, /* GREENYELLOW */
        0xff808080, /* GREY */
        0xfff0fff0, /* HONEYDEW */
        0xffff69b4, /* HOTPINK */
        0xffcd5c5c, /* INDIANRED */
        0xff4b0082, /* INDIGO */
        0xfffffff0, /* IVORY */
        0xfff0e68c, /* KHAKI */
        0xffe6e6fa, /* LAVENDER */
        0xfffff0f5, /* LAVENDERBLUSH */
        0xff7cfc00, /* LAWNGREEN */
        0xfffffacd, /* LEMONCHIFFON */
        0xffadd8e6, /* LIGHTBLUE */
        0xfff08080, /* LIGHTCORAL */
        0xffe0ffff, /* LIGHTCYAN */
        0xfffafad2, /* LIGHTGOLDENRODYELLOW */
        0xffd3d3d3, /* LIGHTGRAY */
        0xff90ee90, /* LIGHTGREEN */
        0xffd3d3d3, /* LIGHTGREY */
        0xffffb6c1, /* LIGHTPINK */
        0xffffa07a, /* LIGHTSALMON */
        0xff20b2aa, /* LIGHTSEAGREEN */
        0xff87cefa, /* LIGHTSKYBLUE */
        0xff8470ff, /* LIGHTSLATEBLUE */
        0xff778899, /* LIGHTSLATEGRAY */
        0xff778899, /* LIGHTSLATEGREY */
        0xffb0c4de, /* LIGHTSTEELBLUE */
        0xffffffe0, /* LIGHTYELLOW */
        0xff00ff00, /* LIME */
        0xff32cd32, /* LIMEGREEN */
        0xfffaf0e6, /* LINEN */
        0xffff00ff, /* MAGENTA */
        0xff800000, /* MAROON */
        0xff66cdaa, /* MEDIUMAQUAMARINE */
        0xff0000cd, /* MEDIUMBLUE */
        0xffba55d3, /* MEDIUMORCHID */
        0xff9370db, /* MEDIUMPURPLE */
        0xff3cb371, /* MEDIUMSEAGREEN */
        0xff7b68ee, /* MEDIUMSLATEBLUE */
        0xff00fa9a, /* MEDIUMSPRINGGREEN */
        0xff48d1cc, /* MEDIUMTURQUOISE */
        0xffc71585, /* MEDIUMVIOLETRED */
        0xff191970, /* MIDNIGHTBLUE */
        0xfff5fffa, /* MINTCREAM */
        0xffffe4e1, /* MISTYROSE */
        0xffffe4b5, /* MOCCASIN */
        0xffffdead, /* NAVAJOWHITE */
        0xff000080, /* NAVY */
        0xfffdf5e6, /* OLDLACE */
        0xff808000, /* OLIVE */
        0xff6b8e23, /* OLIVEDRAB */
        0xffffa500, /* ORANGE */
        0xffff4500, /* ORANGERED */
        0xffda70d6, /* ORCHID */
        0xffeee8aa, /* PALEGOLDENROD */
        0xff98fb98, /* PALEGREEN */
        0xffafeeee, /* PALETURQUOISE */
        0xffdb7093, /* PALEVIOLETRED */
        0xffffefd5, /* PAPAYAWHIP */
        0xffffdab9, /* PEACHPUFF */
        0xffcd853f, /* PERU */
        0xffffc0cb, /* PINK */
        0xffdda0dd, /* PLUM */
        0xffb0e0e6, /* POWDERBLUE */
        0xff800080, /* PURPLE */
        0xffff0000, /* RED */
        0xffbc8f8f, /* ROSYBROWN */
        0xff4169e1, /* ROYALBLUE */
        0xff8b4513, /* SADDLEBROWN */
        0xfffa8072, /* SALMON */
        0xfff4a460, /* SANDYBROWN */
        0xff2e8b57, /* SEAGREEN */
        0xfffff5ee, /* SEASHELL */
        0xffa0522d, /* SIENNA */
        0xffc0c0c0, /* SILVER */
        0xff87ceeb, /* SKYBLUE */
        0xff6a5acd, /* SLATEBLUE */
        0xff708090, /* SLATEGRAY */
        0xff708090, /* SLATEGREY */
        0xfffffafa, /* SNOW */
        0xff00ff7f, /* SPRINGGREEN */
        0xff4682b4, /* STEELBLUE */
        0xffd2b48c, /* TAN */
        0xff008080, /* TEAL */
        0xffd8bfd8, /* THISTLE */
        0xffff6347, /* TOMATO */
        0xff40e0d0, /* TURQUOISE */
        0xffee82ee, /* VIOLET */
        0xffd02090, /* VIOLETRED */
        0xfff5deb3, /* WHEAT */
        0xffffffff, /* WHITE */
        0xfff5f5f5, /* WHITESMOKE */
        0xffffff00, /* YELLOW */
        0xff9acd32  /* YELLOWGREEN */
    ];

    let mut index = ALICEBLUE as uint;

    while (index < YELLOWGREEN as uint) {
        if strings.lwc_string_caseless_isequal(data.clone() , index) {
            break
        }
        index +=1;
    }

    if index == YELLOWGREEN as uint + 1 {
        result_val = colourmap[(index - (ALICEBLUE as uint))];
        return (Some(result_val) , CSS_OK);
    }

    match sheet.color {
        None => {},
        Some(x) => {
            return (*x)(data.clone());
        }
    }
    return(None , CSS_INVALID);
}