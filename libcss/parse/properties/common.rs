use wapcaplet::*;
use extra::arc;

use bytecode::bytecode::*;
use bytecode::opcodes::*;

use lex::lexer::*;
use stylesheet::*;

use include::fpmath::*;

use parse::propstrings::*;

use utils::errors::*;


/**
#Arguments:
*  'vector' - Vector of tokens to process.
*  'ctx'    - Pointer to vector iteration ctx.
*/
pub fn consumeWhitespace(vector:&~[@css_token], ctx:@mut uint) {

    debug!("Entering: consumeWhitespace");
    loop {
        if *ctx < vector.len() {
            match vector[*ctx].token_type {
                CSS_TOKEN_S => {
                    debug!("Entering: consumeWhitespace:: CSS_TOKEN_S");
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

/**
* #Arguments:
*  'sheet'  - Stylesheet. 

*  'vector' - Vector of tokens to process.

*  'ctx'    - Pointer to current vector iteration context.

*  'default_unit'    - The default unit to use if none specified.

* #Return Value:
* 'length' - Option of i32(Some(x) if CSS_OK else None).

* 'unit' - Option of u32(Some(x) if CSS_OK else None).

* 'css_error' - CSS_OK on success,  
                CSS_INVALID if the input is not valid.
* #Post condition:
*   ctx is updated with the next token to process.
*   If the input is invalid, then ctx remains unchanged.
*/
pub fn css__parse_unit_specifier(sheet: @mut css_stylesheet, vector: &~[@css_token] , ctx: @mut uint , default_unit: u32) -> (Option<i32> , Option<u32>, css_error) {

    debug!("Entering: css__parse_unit_specifier");
    debug!("Entering: css__parse_unit_specifier :: ctx == %?  ,  vector == %? " , ctx ,vector);
    let mut token:&@css_token;
    let mut unit_retVal:u32;
    let orig_ctx = *ctx;

    consumeWhitespace(vector , ctx);

    if *ctx >= vector.len() {
        debug!("Exiting: css__parse_unit_specifier (1)");
        return (None , None , CSS_INVALID)
    }
    token = &vector[*ctx];
    *ctx = *ctx + 1;
    debug!("css__parse_unit_specifier :: token == %? , vector == %? " , token , vector);
    match token.token_type {
        CSS_TOKEN_DIMENSION|CSS_TOKEN_NUMBER|CSS_TOKEN_PERCENTAGE => {},
        _ => {
            *ctx = orig_ctx;
            debug!("Exiting: css__parse_unit_specifier (2)");
            return(None , None , CSS_INVALID);
        }
    }

    let (num , consumed_index) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
    debug!("css__parse_unit_specifier :: num == %?" , num);
    match token.token_type {
        CSS_TOKEN_DIMENSION => {
            let data = lwc_string_data(token.idata.get_ref().clone());

            let (unit , result) = css__parse_unit_keyword(data.slice(consumed_index, data.len()));
            match result {
                CSS_OK => {},
                _ => {
                    *ctx = orig_ctx;
                    debug!("Exiting: css__parse_unit_specifier (3)");
                    return (None , None , result);
                }
            }
            unit_retVal = unit.unwrap() as u32;
        },
        CSS_TOKEN_NUMBER => {
            if num !=0 {
                if sheet.quirks_allowed {
                    sheet.quirks_used = true;
                }
                else {
                    *ctx = orig_ctx;
                    debug!("Exiting: css__parse_unit_specifier (4)");
                    return (None , None , CSS_INVALID);
                }
            }
            unit_retVal = default_unit;
            if sheet.quirks_allowed {
                let tmp_ctx = ctx;
                consumeWhitespace(vector , tmp_ctx);
                if *ctx >= vector.len() {
                    debug!("Exiting: css__parse_unit_specifier (5)");
                    return (None , None , CSS_INVALID)
                }
                token = &vector[*tmp_ctx];
                *tmp_ctx = *tmp_ctx + 1;

                match token.token_type {
                    CSS_TOKEN_IDENT => {
                        let (unit , result) = css__parse_unit_keyword(lwc_string_data(token.idata.get_ref()));
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
                debug!("Exiting: css__parse_unit_specifier (6)");
                return (None , None , CSS_INVALID);
            }
            unit_retVal = UNIT_PCT as u32;
        }
    }
    debug!("Exiting: css__parse_unit_specifier (7)");
    debug!(fmt!("css__parse_unit_specifier:: num == %?, unit_retVal == %?", num, unit_retVal));
    return(Some(num) , Some(unit_retVal) , CSS_OK);
}

pub fn css__number_from_lwc_string(string: @mut lwc_string, int_only: bool) -> (i32 , uint) {
    
    debug!("Entering: css__number_from_lwc_string");
    let mut ret_value = 0;
    let mut consumed_length = 0;

    if lwc_string_length(string.clone()) == 0 {
        return (ret_value , consumed_length);
    }
    css__number_from_string(lwc_string_data(string.clone()), @mut 0, int_only)
}

/**
* #Arguments:
*  'ptr'  - keyword string. 

*  'index' - index of string from where we have to parse.
* #Return Value:
* 'unit' - Option of u32(Some(x) if CSS_OK else None).

* 'css_error' - CSS_OK on success,  
                CSS_INVALID if the input is not valid.
*/
pub fn css__parse_unit_keyword(ptr:&str)-> (Option<u32>,css_error) {
    
    debug!("Entering: css__parse_unit_keyword");
    debug!(fmt!("css__parse_unit_keyword:: ptr == %s", copy ptr));
    let mut unit = UNIT_GRAD;
    let ptr_lower = ptr.to_lower();
    match(ptr_lower.len()) {
        4=> if (ptr_lower == ~"grad") {
                unit= UNIT_GRAD;    
            },
        3=> {
            if (ptr_lower == ~"khz") {
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
        2=> {
            if (ptr_lower == ~"hz") {
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
        1=> {
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

pub fn css__number_from_string(data: ~str, data_index:@mut uint, int_only: bool) -> (i32 , uint){

    debug!("Entering: css__number_from_string");
    let mut length = data.len() - *data_index;
    let orig_data_index = *data_index;
    let mut sign = 1;
    let mut intpart: i32 = 0;
    let mut fracpart: i32 = 0;
    let mut pwr: i32 = 1;
    let mut ret_value :i32= 0;
    //let mut data_index = 0;
    let mut consumed_length = 0;
    
    if length <=0 {
        return (ret_value , consumed_length);
    }
		
    // number = [+-]? ([0-9]+ | [0-9]* '.' [0-9]+) 

    // Extract sign, if any 
    if data[0 + *data_index] == '-' as u8 {
        sign = -1;
        length -= 1;
        *data_index += 1;
    }
    else if data[0 + *data_index] == '+' as u8 {
        length -=1;
        *data_index += 1;
    }

	/* Ensure we have either a digit or a '.' followed by a digit */
    if length == 0 {
        return (ret_value , consumed_length);
    }
    else {
        if data[0 + *data_index] == '.' as u8 {
            if length ==1 || (data[1 + *data_index] < ('0' as u8)) || (('9' as u8) < data[1 + *data_index]) {
                return (ret_value , consumed_length);
            }
        }
        else if (data[0 + *data_index] < ('0' as u8)) || (('9' as u8) < data[0 + *data_index]) {
            return (ret_value , consumed_length);
        }
    }

	/* Now extract intpart, assuming base 10 */
    while length>0 {
		/* Stop on first non-digit */
        if (data[0 + *data_index] < ('0' as u8))||(('9' as u8) < data[0 + *data_index]) {
            break
        }
		
		/* Prevent overflow of 'intpart'; proper clamping below */
        if intpart < (1i32<<22) {
            intpart *= 10;
            intpart += (data[0 + *data_index] as i32) - ('0' as i32);
        }
        *data_index += 1;
        length -= 1;
    }

	/* And fracpart, again, assuming base 10 */
    if int_only == false && length > 1 && (data[0 + *data_index] == '.' as u8) && ('0' as u8 <= data[1 + *data_index] && data[1 + *data_index] <= '9' as u8) {
        *data_index += 1; 
        length -= 1;
		
        while length >0 {
            if ((data[0 + *data_index] < '0' as u8))|| (('9' as u8) < data[0 + *data_index]) {
                break
            }
			
            if pwr < 1000000 {
                pwr *= 10;
                fracpart *= 10;
                fracpart += (data[0 + *data_index] - '0' as u8) as i32;
            }
            *data_index += 1;
            length -= 1;
        }
        fracpart = ((1i32 << 10) * fracpart + pwr/2) / pwr;
        if fracpart >= (1i32 << 10) {
            intpart += 1;
            fracpart &= (1i32 << 10) - 1;
        }
    }

    consumed_length = *data_index - orig_data_index;

    if sign > 0 {
        /* If the result is larger than we can represent,
		 * then clamp to the maximum value we can store. */
		if intpart >= (1i32 << 21) {
            intpart = (1i32 << 21) - 1;
            fracpart = (1i32 << 10) - 1;
        }
    }
    else {
        /* If the negated result is smaller than we can represent
		 * then clamp to the minimum value we can store. */
        if intpart >= (1i32 << 21) {
            intpart = -(1i32 << 21);
            fracpart = 0;
        }
        else {
            intpart = -intpart;
            if fracpart != 0 {
                fracpart = (1 << 10) - fracpart;
                intpart -= 1;
            }
        }
    }

    ret_value = ((intpart << 10) | fracpart );
    (ret_value , consumed_length)
}

pub fn is_css_inherit(strings: &mut ~css_propstrings , token: &@css_token) ->bool {
    
    debug!("Entering: is_css_inherit");
    match token.token_type {
        CSS_TOKEN_IDENT => {
             return strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , INHERIT as uint);
        }
        _ => false
    }
}

/**
* #Arguments:
*  'sheet'  - Stylesheet. 

*  'vector' - Vector of tokens to process.

*  'ctx'    - Pointer to current vector iteration context.

* #Return Value:
* 'value' - Option of u16(Some(x) if CSS_OK else None).

* 'result' - Option of u16(Some(x) if CSS_OK else None) (AARRGGBB).

* 'css_error' - CSS_OK on success,  
                CSS_INVALID if the input is not valid.

				* #Post condition:
*   ctx is updated with the next token to process.
*   If the input is invalid, then ctx remains unchanged.
*/
pub fn css__parse_color_specifier(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings , vector: &~[@css_token] , ctx: @mut uint) -> (Option<u16> , Option<u32> , css_error) {
    
    debug!("Entering: css__parse_color_specifier");
    let mut token:&@css_token;
    let mut ret_value: u16;
    let mut ret_result: u32 = 0;
    let orig_ctx = *ctx;

    consumeWhitespace(vector , ctx);
    if *ctx >= vector.len() {
        return (None , None , CSS_INVALID)
    }
    
    /* IDENT(<colour name>) | 
     * HASH(rgb | rrggbb) |
     * FUNCTION(rgb) [ [ NUMBER | PERCENTAGE ] ',' ] {3} ')'
     * FUNCTION(rgba) [ [ NUMBER | PERCENTAGE ] ',' ] {4} ')'
     * FUNCTION(hsl) ANGLE ',' PERCENTAGE ',' PERCENTAGE  ')'
     * FUNCTION(hsla) ANGLE ',' PERCENTAGE ',' PERCENTAGE ',' NUMBER ')'
     *
     * For quirks, NUMBER | DIMENSION | IDENT, too
     * I.E. "123456" -> NUMBER, "1234f0" -> DIMENSION, "f00000" -> IDENT
     */

    token = &vector[*ctx];
    *ctx = *ctx + 1;

    if token.token_type as int != CSS_TOKEN_IDENT as int && token.token_type as int != CSS_TOKEN_HASH as int 
        && token.token_type as int != CSS_TOKEN_FUNCTION as int {

        if (sheet.quirks_allowed== false || (token.token_type as int != CSS_TOKEN_NUMBER as int 
            && token.token_type as int != CSS_TOKEN_DIMENSION as int ))
        {
            *ctx = orig_ctx;
            return (None , None , CSS_INVALID);
        } 
    }
	unsafe {
		debug!(fmt!("Token = %?", token));
		debug!(fmt!("sheet.quirks_allowed = %?", sheet.quirks_allowed));
	}
	
    if token.token_type as int == CSS_TOKEN_IDENT as int  {
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

        let (_ret_result , error) = css__parse_named_color(sheet , strings , token.idata.get_ref().clone());
        
        if _ret_result.is_some() {
            ret_result = _ret_result.unwrap();
        }

        if error as int != CSS_OK as int && sheet.quirks_allowed {
            let(_ret_result , error) = css__parse_hash_colour(token.idata.get_ref().clone());
            if _ret_result.is_some() {
                ret_result = _ret_result.unwrap();
            }
            
            if error as int == CSS_OK as int {
                sheet.quirks_used = true;
            }
        }

        if error as int != CSS_OK as int {
            *ctx = orig_ctx;
            return (None , None , CSS_INVALID);
        }
    }

    else if (token.token_type as int ==  CSS_TOKEN_HASH as int || (sheet.quirks_allowed && token.token_type as int ==  CSS_TOKEN_NUMBER as int) ||
        (sheet.quirks_allowed && token.token_type as int ==  CSS_TOKEN_DIMENSION as int))
    {
        let(_ret_result , error_from_hash) = css__parse_hash_colour(token.idata.get_ref().clone());

        match error_from_hash {
            CSS_OK => {
                if token.token_type as int != CSS_TOKEN_HASH as int {
                    sheet.quirks_used = true;
                }
            },
            _ => {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
        }
        if _ret_result.is_some() {
            ret_result = _ret_result.unwrap();
        }
    }
    else if (token.token_type as int ==  CSS_TOKEN_FUNCTION as int) {
        let r:@mut u8 = @mut 0;
        let g:@mut u8 = @mut 0;
        let b:@mut u8 = @mut 0;
        let a:@mut u8 = @mut 0xff;
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
            let mut valid = CSS_TOKEN_NUMBER;
            let components: ~[@mut u8] = ~[r, g, b, a];
            let mut component: @mut u8;
            while i < colour_channels {
                
                let mut intval: i32;
                let mut int_only: bool;

                component = components[i];
                consumeWhitespace(vector , ctx);
				
				if *ctx >= vector.len() {
					*ctx = orig_ctx;
					return (None , None , CSS_INVALID);
				}
				token = &vector[*ctx];
                
				match token.token_type {
                    CSS_TOKEN_NUMBER => {},
                    CSS_TOKEN_PERCENTAGE => {},
                    _ => {
                        *ctx = orig_ctx;
                        return (None , None , CSS_INVALID);
                    }
                }
                if i==0 {
                    valid = token.token_type;
                }
                else if ( i<3 ) && (token.token_type as uint != valid as uint) {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }

               
				/* The alpha channel may be a float */
                if i<3 {
                    int_only = match valid {
                    CSS_TOKEN_NUMBER => true,
                     _=> false
                    };
                }
                else {
                    int_only = false;
                }
                let (num , consumed_index) = css__number_from_lwc_string(token.idata.get_ref().clone() , int_only);

                if consumed_index != lwc_string_length(token.idata.get_ref().clone()) {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
                
				match valid {
                    CSS_TOKEN_NUMBER=>{
                        if (i==3) {
							/* alpha channel */
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
                    *component = 255;
                }
                else if intval < 0 {
                    *component = 0;
                }
                else {
                    *component = intval as u8;
                }

                *ctx = *ctx + 1; //Iterate
                
				consumeWhitespace(vector , ctx);

                if *ctx >= vector.len() {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);     
                }
				token = &vector[*ctx];
				
                if (i != (colour_channels - 1) && tokenIsChar(token , ',')) {
                    *ctx = *ctx + 1; //Iterate
                }
                else if (i == (colour_channels - 1) && tokenIsChar(token , ')')) {
                    *ctx = *ctx + 1; //Iterate
                }
                else {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
                i = i + 1;
            }
        }
        else if colour_channels == 5 || colour_channels == 6 {
            /* hue - saturation - lightness */
			
			let mut hue: i32;
            let mut sat: i32;
            let mut lit: i32;
            let mut alpha: i32 = 255;
			
			/* hue is a number without a unit representing an 
			 * angle (0-360) degrees  
			 */
            consumeWhitespace(vector , ctx);

            if *ctx >= vector.len() {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			token = &vector[*ctx];
            *ctx = *ctx + 1; //Iterate
            
            match token.token_type {
                CSS_TOKEN_NUMBER => {},
                _ => {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
            }
			
            let (hue_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
            hue = hue_res as i32;
            
			if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                /* failed to consume the whole string as a number */
				*ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
            
			/* Normalise hue to the range [0, 360) */
			while hue < 0 {
                hue += F_360 as i32;
            }
            while hue >= F_360 as i32 {
                hue -= F_360 as i32;
            }

            consumeWhitespace(vector , ctx);
            
            if *ctx >= vector.len() {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			token = &vector[*ctx];
            *ctx = *ctx + 1;
            
            if !tokenIsChar(token , ',') {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			/* saturation */
            consumeWhitespace(vector , ctx);
            
            if *ctx >= vector.len() {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			token = &vector[*ctx];
            *ctx = *ctx + 1;
            
            match token.token_type {
                CSS_TOKEN_PERCENTAGE => {},
                _ => {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
            }
            
			let (sat_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
            sat = sat_res as i32;
            if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
				/* failed to consume the whole string as a number */
				*ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			/* Normalise saturation to the range [0, 100] */
            if sat < css_int_to_fixed(0) {
                sat = css_int_to_fixed(0);
            }
            else if sat > css_int_to_fixed(100) {
                sat = css_int_to_fixed(100);
            }

            consumeWhitespace(vector, ctx);

            if *ctx >= vector.len() {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			token = &vector[*ctx];
            *ctx = *ctx + 1;

            if !tokenIsChar(token , ',') {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }

			/* lightness */	
            consumeWhitespace(vector , ctx);
			
			if *ctx >= vector.len() {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
            token = &vector[*ctx];
            *ctx = *ctx + 1;

            match token.token_type {
                CSS_TOKEN_PERCENTAGE => {},
                _ => {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
            }
            let (lit_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
            lit = lit_res as i32;
            if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                /* failed to consume the whole string as a number */
				*ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			/* Normalise lightness to the range [0, 100] */
            if lit < css_int_to_fixed(0) {
                lit = css_int_to_fixed(0);
            }
            else if lit > css_int_to_fixed(100) {
                lit = css_int_to_fixed(100);
            }

            consumeWhitespace(vector , ctx);

            if *ctx >= vector.len() {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			token = &vector[*ctx];
            *ctx = *ctx + 1;

            if colour_channels == 6 {
                /* alpha */
				
				if !tokenIsChar(token , ',') {
                    *ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }
                consumeWhitespace(vector , ctx);

                if *ctx >= vector.len() {
					*ctx = orig_ctx;
					return (None , None , CSS_INVALID);
				}
				token = &vector[*ctx];
                *ctx = *ctx + 1;

                match token.token_type {
                    CSS_TOKEN_NUMBER => {},
                    _ => {
                        *ctx = orig_ctx;
                        return (None , None , CSS_INVALID);
                    }
                }
                
				let (alpha_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
                alpha = alpha_res as i32;
                if consumed_length_from_lwc_string != lwc_string_length(token.idata.get_ref().clone()) {
                    /* failed to consume the whole string as a number */
					*ctx = orig_ctx;
                    return (None , None , CSS_INVALID);
                }

                alpha = ((css_multiply_fixed(alpha as i32 , F_255 as i32) as int) >> CSS_RADIX_POINT) as i32;
                
				consumeWhitespace(vector , ctx);

                if *ctx >= vector.len() {
					*ctx = orig_ctx;
					return (None , None , CSS_INVALID);
				}
				token = &vector[*ctx];
                *ctx = *ctx + 1;
            }
			
            if !tokenIsChar(token , ')') {
                *ctx = orig_ctx;
                return (None , None , CSS_INVALID);
            }
			
			debug!(fmt!("hue= %?, sat= %?, lit= %?", hue as i32, sat as i32, lit as i32)); //DEBUG
            
			/* have a valid HSV entry, convert to RGB */
			let (ra , ga , ba) = HSL_to_RGB(hue as i32, sat as i32, lit as i32);
            *r = ra;
            *g = ga;
            *b = ba;
			
			debug!(fmt!("ra= %?, ga= %?, ba= %?", ra, ga, ba)); //DEBUG
            
			if alpha > 255 {
                *a = 255;
            }
            else if alpha < 0 {
                *a = 0;
            }
            else {
                *a = alpha as u8;
            }
        }
        else {
            *ctx = orig_ctx;
            return (None , None , CSS_INVALID);
        }
		debug!(fmt!("a= %?, r= %?, g= %?, b= %?", *a, *r, *g, *b)); //DEBUG
		debug!(fmt!("a= %?, r= %?, g= %?, b= %?", *a as u32 << 24, *r as u32 << 16, *g as u32 << 8, *b as u32)); //DEBUG
		
        ret_result = (*a as u32 << 24 | *r as u32 << 16 | *g as u32 << 8 | *b as u32);
        
    }

    else {
        *ctx = orig_ctx;
        return (None , None , CSS_INVALID);
    }
	
	ret_value = COLOR_SET ;
	
	debug!(fmt!("css__parse_color_specifier :: Return value= %?, result= %?", ret_value, ret_result)); //DEBUG
    
	(Some(ret_value) , Some(ret_result) , CSS_OK)
}


/**
* #Arguments:
*  'data'  - arc of colour string(lwc_string). 

* #Return Value:
* 'result' - Option of u32 (AARRGGBB) (some(x) if CSS_OK else None).

* 'css_error' - CSS_OK on success,  
                CSS_INVALID if the input is not valid.

* #Post condition:
*   ctx is updated with the next token to process.
*   If the input is invalid, then ctx remains unchanged.
*/
pub fn css__parse_hash_colour(data: @mut lwc_string) -> (Option<u32> , css_error){

    debug!("Entering: css__parse_hash_colour");
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
		debug!(fmt!("r=%?, g=%?, b=%? ",r, g, b));

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
	debug!(fmt!("r=%?, g=%?, b=%? ",r, g, b));
	
    result_val = (a as u32 << 24) | (r as u32 << 16) | (g as u32 << 8) | b as u32;
	
	debug!(fmt!("result_val=%?",result_val));
    return (Some(result_val) , CSS_OK);
}

/**
* #Arguments:
*  'token'  - The token to consider. 

*  'c'  - The character to match (lowerASCII only). 

* #Return Value:
* 'bool' - True if the token matches, false otherwise.
*/
pub fn tokenIsChar(token:&@css_token, c:char) -> bool {
    
    debug!("Entering: tokenIsChar");
    let result = false;

    match token.token_type {
        CSS_TOKEN_CHAR => {   
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


/**
* #Arguments:
*  'hue'  - Hue in degrees 0..360. 

*  'sat'  - Saturation value in percent 0..100. 

*  'lit'  - Lightness value in percent 0..100. 

* #Return Value:
* 'r(u8)' - red component.

* 'g(u8)' - green component.

* 'b(u8)' - blue component.
*/
pub fn HSL_to_RGB(hue: i32 , sat: i32 , lit: i32 ) -> (u8 , u8 , u8) {

    debug!("Entering: HSL_to_RGB");
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
	debug!(fmt!("max_rgb= %?", max_rgb));
	
    /* Compute min(r,g,b) */
    min_rgb = css_subtract_fixed(css_multiply_fixed(lit, css_int_to_fixed(2)), max_rgb);
	debug!(fmt!("min_rgb= %?", min_rgb));
	
    /* We know that the value of at least one of the components is 
     * max(r,g,b) and that the value of at least one of the other
     * components is min(r,g,b).
     *
     * We can determine which components have these values by
     * considering which the sextant of the hexcone the hue lies
     * in:
     *
     * Sextant: max(r,g,b): min(r,g,b):
     *
     * 0        r       b
     * 1        g       b
     * 2        g       r
     * 3        b       r
     * 4        b       g
     * 5        r       g
     *
     * Thus, we need only compute the value of the third component
     */

    /* Chroma is the difference between min and max */
    chroma = css_subtract_fixed(max_rgb, min_rgb);
	debug!(fmt!("chroma= %?", chroma));
	
    /* Compute which sextant the hue lies in (truncates result) */
    let hue = css_divide_fixed(css_multiply_fixed(hue, css_int_to_fixed(6)), F_360 as i32);
    sextant = (hue as int) >> CSS_RADIX_POINT;
	debug!(fmt!("hue= %?", hue));
	debug!(fmt!("sextant= %?", sextant));

    /* Compute offset of hue from start of sextant */
    relative_hue = css_subtract_fixed(hue, css_int_to_fixed(sextant));
	debug!(fmt!("relative_hue= %?", relative_hue));
	
    /* Scale offset by chroma */
    scaled_hue = css_multiply_fixed(relative_hue, chroma);
	debug!(fmt!("scaled_hue= %?", scaled_hue));
	
    /* Compute potential values of the third colour component */
    mid1 = css_add_fixed(min_rgb, scaled_hue);
    mid2 = css_subtract_fixed(max_rgb, scaled_hue);
	debug!(fmt!("mid1= %?", mid1));
	debug!(fmt!("mid2= %?", mid2));
	
    match sextant {
        0 => {
            let r = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
            debug!(fmt!("r= %?", r));
			let g = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32    )) >> CSS_RADIX_POINT;
			debug!(fmt!("g= %?", g));
            let b = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32 )) >> CSS_RADIX_POINT;
			debug!(fmt!("b= %?", b));
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

/**
* #Arguments:
*  'data'  - arc of colour string(lwc_string). 

* #Return Value:
* 'result' - Option of u32 (AARRGGBB) (some(x) if CSS_OK else None).

* 'css_error' - CSS_OK on success,  
                CSS_INVALID if the input is not valid.
*/
fn css__parse_named_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings , data: @mut lwc_string) -> (Option<u32> , css_error){
    
    debug!("Entering: css__parse_named_color");
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

    while (index < YELLOWGREEN as uint + 1) {
        if strings.lwc_string_caseless_isequal(data.clone() , index) {
            break
        }
        index +=1;
    }

    if index <= YELLOWGREEN as uint {
        result_val = colourmap[(index - (ALICEBLUE as uint))];
        return (Some(result_val) , CSS_OK);
    }

    /* We don't know this colour name; ask the client */
    match sheet.color {
        None => {},
        Some(x) => {
            return (*x)(data.clone());
        }
    }
    return(None , CSS_INVALID);
}