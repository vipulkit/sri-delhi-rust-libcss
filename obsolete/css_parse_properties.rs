#[link(name = "css_parse_properties", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod parserutils_inputstream;
extern mod wapcaplet;
extern mod std;


use std::arc;
use css_enum::* ;
use wapcaplet::*;
use parserutils_inputstream::*;

pub struct css_token {
	token_type:css_token_type,
    data:~[u8],
	idata:arc::RWARC<~lwc_string> 
}

pub fn copy_token(data : &css_token) -> css_token {
	let mut temp = css_token{
						token_type:copy data.token_type,
						data:copy data.data,
						idata:data.idata.clone()
					};
	temp
}

// function pointer name notation -> <modulename>_<fnptr>_actual_name_in_C
pub type css_fnptr_font_rule_font_family_reserved =  ~extern fn(strings : &mut ~[arc::RWARC<~lwc_string>], 
		idata :arc::RWARC<~lwc_string>) -> bool;


pub fn css__ident_list_or_string_to_string(mut strings : ~[arc::RWARC<~lwc_string>],
								vector : & ~[css_token], ctx : &mut uint,
								reserved : Option<css_fnptr_font_rule_font_family_reserved>,
								lwc_instance : arc::RWARC<~lwc> ) 
								-> ( css_result, Option<arc::RWARC<~lwc_string>> ) {

	if *ctx >= vector.len() { return (CSS_INVALID,None); }
	let token : css_token = copy_token(&vector[*ctx]);

	match token.token_type {
		CSS_TOKEN_STRING => { *ctx = *ctx+1;
				do lwc_instance.write |l| {
					( CSS_OK , Some(l.lwc_string_ref( (token.idata).clone() )) )
				}
		},
		CSS_TOKEN_IDENT => css__ident_list_to_string(strings,vector,ctx,reserved,lwc_instance) ,
		_ => (CSS_INVALID, None)
	}
}

pub fn css__ident_list_to_string(mut strings : ~[arc::RWARC<~lwc_string>],
								vector : & ~[css_token], ctx : &mut uint,
								reserved : Option<css_fnptr_font_rule_font_family_reserved>,
								lwc_instance : arc::RWARC<~lwc> ) 
								-> ( css_result, Option<arc::RWARC<~lwc_string>> ) {

	let mut orig_ctx : uint = *ctx;
	let mut token : css_token ;
	let mut buffer : ~str = ~"" ;

	while ( orig_ctx < vector.len() ) {
		token = copy_token(&vector[orig_ctx]);
		match token.token_type {
			CSS_TOKEN_IDENT => { 
					match reserved {
						None => return (CSS_INVALID,None) ,
						Some(copy x) => 	if( (*x)(&mut strings,(token.idata).clone()) ) {
										return (CSS_INVALID,None) ;
									}
					}
					buffer += lwc::lwc_string_data((token.idata).clone());
			},
			CSS_TOKEN_S => buffer += ~" " ,
			_ => loop 
		}
		orig_ctx += 1;
	}

	buffer.trim_right();
	do lwc_instance.write |l| {
		( CSS_OK , Some(l.lwc_intern_string(copy buffer)) )
	}
}
