#[link(name = "css_fontface", vers = "0.1")];
#[crate_type = "lib"];


extern mod css_enum;
extern mod parserutils_inputstream;
extern mod wapcaplet;
extern mod std;
extern mod css_parse_properties;


use std::arc;
use css_enum::* ;
use wapcaplet::*;
use parserutils_inputstream::*;
use css_parse_properties::*;

// pub struct css_font_face_src {
// 	location:arc::RWARC<~lwc_string>,
// 	/*
// 	 * Bit allocations:
// 	 *
// 	 *    76543210
// 	 *  1 _fffffll	format | location type
// 	 */
// 	bits:[u8, ..1]
// }

// pub struct css_font_face {
// 	font_family: arc::RWARC<~lwc_string>,
// 	src:~css_font_face_src, 
// 	n_srcs:uint,
	
// 	/*
// 	 * Bit allocations:
// 	 *
// 	 *    76543210
// 	 *  1 __wwwwss	font-weight | font-style
// 	 */
// 	bits:[u8, ..1]
// }

// pub struct css_rule_font_face {
// 	//base :css_rule,

// 	font_face:@css_font_face 
// }

fn font_rule_font_family_reserved(mut lwc_instance : lwc,
									strings : &mut ~[arc::RWARC<~lwc_string>], 
									idata :arc::RWARC<~lwc_string> ) -> bool  {

	return 
	    (lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[SERIF].clone())      == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[SANS_SERIF].clone()) == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[CURSIVE].clone())    == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[FANTASY].clone())    == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[MONOSPACE].clone())  == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[INHERIT].clone())    == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[INITIAL].clone())    == true  ) ||
		(lwc_instance.lwc_string_caseless_isequal( idata.clone(), strings[DEFAULT].clone())    == true  );
}

fn css__font_face_set_font_family(lwc_instance : &mut ~lwc, font_face : &mut ~css_font_face ,
		font_family : arc::RWARC<~lwc_string> ) -> css_result {
	
	if ( lwc::lwc_string_length((font_face.font_family).clone()) != 0 ) {
		lwc_instance.lwc_string_unref((font_face.font_family).clone());
	}
		
	font_face.font_family = lwc_instance.lwc_string_ref(font_family);

	return CSS_OK;
}


/*
fn css__font_face_set_font_family(font_face : &~css_font_face ,
		font_family:arc::RWARC<~lwc_string> , lwc_instance:arc::ARC<~lwc>) -> css_result
{
	if ( lwc::lwc_string_length(font_face.font_family) == 0 || lwc::lwc_string_length(font_family) == 0) {
		return CSS_BADPARM;
	}
	
	let lwc_ins = arc::get(&lwc_instance);	
	if (lwc::lwc_string_length(font_face.font_family) != 0) {
		lwc_ins.lwc_string_unref(font_face.font_family);
	}
		
	font_face.font_family = lwc_ins.lwc_string_ref(font_family);

	return CSS_OK;
}


fn font_face_parse_font_family(css_language *c, 
		const parserutils_vector *vector, int *ctx,
		css_font_face *font_face) -> css_result {
	
	css_result error;
	lwc_string @string;

	error = css__ident_list_or_string_to_string(c, vector, ctx,
				Some(font_rule_font_family_reserved), &string);
	if (error != CSS_OK)
		return error;

	css__font_face_set_font_family(font_face, string);

	lwc_string_unref(string);

	return CSS_OK;
}
	

pub fn css__parse_font_descriptor(css_language *c,
		const css_token *descriptor, const parserutils_vector *vector,
		int *ctx, struct css_rule_font_face *rule) -> css_result {

}
*/