#[link(name = "css_properties", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod css_stylesheet;
extern mod css_propstrings;
extern mod css_enum;
extern mod wapcaplet;

//extern mod css_propstrings_parallel;

use css_stylesheet::*;
use css_propstrings::*;
use css_enum::*;
use wapcaplet::*;
use std::arc;
use core::str::*;

//use css_propstrings::*; 
pub struct css_token {
	token_type: css_token_type,
	idata: Option<arc::RWARC<~lwc_string>>,
	// col: u32,
	// line: u32
}

pub type handle =  @extern fn(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style) ->css_result;

pub struct css_properties {
	property_handlers: ~[handle]
}

impl css_properties {

	pub fn css_properties() -> ~css_properties {

		let mut vec = ~[];
		vec.push(@css_properties::css__parse_azimuth); //0
		vec.push(@css_properties::css__parse_background); //1
		vec.push(@css_properties::css__parse_background_attachment); //2
		vec.push(@css_properties::css__parse_background_color); //3
		vec.push(@css_properties::css__parse_background_image); //4
		vec.push(@css_properties::css__parse_background_position); //5
		vec.push(@css_properties::css__parse_background_repeat); //6
		vec.push(@css_properties::css__parse_border); //7
		vec.push(@css_properties::css__parse_border_bottom); //8
		vec.push(@css_properties::css__parse_border_bottom_color); //9 
		vec.push(@css_properties::css__parse_border_bottom_style); //10
		vec.push(@css_properties::css__parse_border_bottom_width); //11
		vec.push(@css_properties::css__parse_border_collapse); //12
		vec.push(@css_properties::css__parse_border_color); //13
		vec.push(@css_properties::css__parse_border_left); //14
		vec.push(@css_properties::css__parse_border_left_color); //15
		vec.push(@css_properties::css__parse_border_left_style); //16
		vec.push(@css_properties::css__parse_border_left_width); //17
		vec.push(@css_properties::css__parse_border_right); //18
		vec.push(@css_properties::css__parse_border_right_color); //19
		vec.push(@css_properties::css__parse_border_right_style); //20
		vec.push(@css_properties::css__parse_border_right_width); //21		
		vec.push(@css_properties::css__parse_border_spacing); //22
		vec.push(@css_properties::css__parse_border_style); //23
		vec.push(@css_properties::css__parse_border_top); //24
		vec.push(@css_properties::css__parse_border_top_color); //25
		vec.push(@css_properties::css__parse_border_top_style); //26
		vec.push(@css_properties::css__parse_border_top_width); //27
		vec.push(@css_properties::css__parse_border_width); //28
		vec.push(@css_properties::css__parse_bottom); //29
		vec.push(@css_properties::css__parse_break_after); //30
		vec.push(@css_properties::css__parse_break_before); //31
		vec.push(@css_properties::css__parse_break_inside); //32
		vec.push(@css_properties::css__parse_caption_side); //33
		vec.push(@css_properties::css__parse_clear); //34
		vec.push(@css_properties::css__parse_clip); //35
		vec.push(@css_properties::css__parse_color); //36
		vec.push(@css_properties::css__parse_columns); //37
		vec.push(@css_properties::css__parse_column_count); //38
		vec.push(@css_properties::css__parse_column_fill); //39
		vec.push(@css_properties::css__parse_column_gap); //40
		vec.push(@css_properties::css__parse_column_rule); //41
		vec.push(@css_properties::css__parse_column_rule_color); //42
		vec.push(@css_properties::css__parse_column_rule_style); //43
		vec.push(@css_properties::css__parse_column_rule_width); //44
		vec.push(@css_properties::css__parse_column_span); //45
		vec.push(@css_properties::css__parse_column_width); //46
		vec.push(@css_properties::css__parse_content); //47
		vec.push(@css_properties::css__parse_counter_increment); //48
		vec.push(@css_properties::css__parse_counter_reset); //49
		vec.push(@css_properties::css__parse_cue); //50
		vec.push(@css_properties::css__parse_cue_after); //51
		vec.push(@css_properties::css__parse_cue_before); //52
		vec.push(@css_properties::css__parse_cursor); //53
		vec.push(@css_properties::css__parse_direction); //54
		vec.push(@css_properties::css__parse_display); //55
		vec.push(@css_properties::css__parse_elevation); //56
		vec.push(@css_properties::css__parse_empty_cells); //57
		vec.push(@css_properties::css__parse_float); //58
		vec.push(@css_properties::css__parse_font); //59
		vec.push(@css_properties::css__parse_font_family); //60
		vec.push(@css_properties::css__parse_font_size); //61
		vec.push(@css_properties::css__parse_font_style); //62
		vec.push(@css_properties::css__parse_font_variant); //63
		vec.push(@css_properties::css__parse_font_weight); //64
		vec.push(@css_properties::css__parse_height); //65
		vec.push(@css_properties::css__parse_left); //66
		vec.push(@css_properties::css__parse_letter_spacing); //67
		vec.push(@css_properties::css__parse_line_height); //68
		vec.push(@css_properties::css__parse_list_style); //69
		vec.push(@css_properties::css__parse_list_style_image); //70
		vec.push(@css_properties::css__parse_list_style_position); //71
		vec.push(@css_properties::css__parse_list_style_type); //72
		vec.push(@css_properties::css__parse_margin); //73
		vec.push(@css_properties::css__parse_margin_bottom); //74
		vec.push(@css_properties::css__parse_margin_left); //75
		vec.push(@css_properties::css__parse_margin_right); //76
		vec.push(@css_properties::css__parse_margin_top); //77
		vec.push(@css_properties::css__parse_max_height); //78
		vec.push(@css_properties::css__parse_max_width); //79
		vec.push(@css_properties::css__parse_min_height); //80
		vec.push(@css_properties::css__parse_min_width); //81
		vec.push(@css_properties::css__parse_opacity); //82
		vec.push(@css_properties::css__parse_orphans); //83
		vec.push(@css_properties::css__parse_outline); //84
		vec.push(@css_properties::css__parse_outline_color); //85
		vec.push(@css_properties::css__parse_outline_style); //86
		vec.push(@css_properties::css__parse_outline_width); //87
		vec.push(@css_properties::css__parse_overflow); //88
		vec.push(@css_properties::css__parse_padding); //89
		vec.push(@css_properties::css__parse_padding_bottom); //90
		vec.push(@css_properties::css__parse_padding_left); //91
		vec.push(@css_properties::css__parse_padding_right); //92
		vec.push(@css_properties::css__parse_padding_top); //93
		vec.push(@css_properties::css__parse_page_break_after); //94
		vec.push(@css_properties::css__parse_page_break_before); //95
		vec.push(@css_properties::css__parse_page_break_inside); //96
		vec.push(@css_properties::css__parse_pause); //97
		vec.push(@css_properties::css__parse_pause_after); //98
		vec.push(@css_properties::css__parse_pause_before); //99
		vec.push(@css_properties::css__parse_pitch_range); //100
		vec.push(@css_properties::css__parse_pitch); //101
		vec.push(@css_properties::css__parse_play_during); //102
		vec.push(@css_properties::css__parse_position); //103
		vec.push(@css_properties::css__parse_quotes); //104
		vec.push(@css_properties::css__parse_richness); //105
		vec.push(@css_properties::css__parse_right); //106
		vec.push(@css_properties::css__parse_speak_header); //107
		vec.push(@css_properties::css__parse_speak_numeral); //108
		vec.push(@css_properties::css__parse_speak_punctuation); //109
		vec.push(@css_properties::css__parse_speak); //110
		vec.push(@css_properties::css__parse_speech_rate); //111
		vec.push(@css_properties::css__parse_stress); //112
		vec.push(@css_properties::css__parse_table_layout); //113
		vec.push(@css_properties::css__parse_text_align); //114
		vec.push(@css_properties::css__parse_text_decoration); //115
		vec.push(@css_properties::css__parse_text_indent); //116
		vec.push(@css_properties::css__parse_text_transform); //117
		vec.push(@css_properties::css__parse_top); //118
		vec.push(@css_properties::css__parse_unicode_bidi); //119
		vec.push(@css_properties::css__parse_vertical_align); //120
		vec.push(@css_properties::css__parse_visibility); //121
		vec.push(@css_properties::css__parse_voice_family); //122
		vec.push(@css_properties::css__parse_volume); //123
		vec.push(@css_properties::css__parse_white_space); //124
		vec.push(@css_properties::css__parse_widows); //125
		vec.push(@css_properties::css__parse_width); //126
		vec.push(@css_properties::css__parse_word_spacing); //127
		vec.push(@css_properties::css__parse_z_index); //128
		
		~css_properties{
			property_handlers: vec
		}
	}

	fn css__parse_azimuth(strings: &mut ~css_propstrings ,vector:&~[~css_token], ctx: @mut uint, style: @mut css_style)->css_result {
	    let orig_context:uint = *ctx;
		let error:css_result = CSS_OK;
		
		
		let mut flags:u8 = 0;
		let mut  value:u16 = 0;
		//let length: css_fixed = 0;
		let mut unit:u32 = 0;
		//let mut  matches:bool;

		if *ctx >= vector.len()
		{
			return CSS_INVALID;
		}
		
		let mut token=&vector[*ctx];
		if ( 
			match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			 } && strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), INHERIT as uint) 
		) {
			//token=&vector[*ctx];
			*ctx += 1;
			flags = FLAG_INHERIT  ;

		}
		else if ( 
			match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			 } && strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFTWARDS as uint)
		) {
			

			//token=&vector[*ctx];
			*ctx += 1;
			value = AZIMUTH_LEFTWARDS;
		}

		
		else if ( 
			match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			 } && strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHTWARDS as uint) 
			) {
			//token=&vector[*ctx];
			*ctx += 1;
			value = AZIMUTH_RIGHTWARDS;

		}

		else if ( match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			 } 
		)
			{
			if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFT_SIDE as uint) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_LEFT_SIDE;

			}

			else if ( 
			strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), FAR_LEFT as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_FAR_LEFT;

			}
			else if ( 
			strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFT as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_LEFT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER_LEFT as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_CENTER_LEFT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_CENTER;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),CENTER_RIGHT as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_CENTER_RIGHT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHT as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_RIGHT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), FAR_RIGHT as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_FAR_RIGHT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHT_SIDE as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_RIGHT_SIDE;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), BEHIND as uint) 
			) {
				//token=&vector[*ctx];
				*ctx += 1;
				value = AZIMUTH_BEHIND;

			}
			else {
				*ctx = orig_context;
				return CSS_INVALID;
			}
			//consumeWhitespace(vector, ctx);
			token=&vector[*ctx];

			if (
				match (token.token_type) {
					CSS_TOKEN_IDENT(_) => true,
					_=> false
			 	}&& value == AZIMUTH_BEHIND
			) {
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFT_SIDE as uint) {
					value |= AZIMUTH_LEFT_SIDE;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), FAR_LEFT as uint) {
					value |= AZIMUTH_FAR_LEFT;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFT as uint) {
					value |= AZIMUTH_LEFT;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER_LEFT as uint) {
					value |= AZIMUTH_CENTER_LEFT;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER as uint) {
					value |=  AZIMUTH_CENTER;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER_RIGHT as uint) {
					value |= AZIMUTH_CENTER_RIGHT;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHT as uint) {
					value |= AZIMUTH_RIGHT;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), FAR_RIGHT as uint) {
					value |= AZIMUTH_FAR_RIGHT;
				}
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHT_SIDE as uint) {
					value |= AZIMUTH_RIGHT_SIDE;
				}
				else {
				*ctx = orig_context;
				return CSS_INVALID;
				}
			}
			else if  (
				match (token.token_type) {
					CSS_TOKEN_IDENT(_) => true,
					_=> false
			 	}&& value != AZIMUTH_BEHIND
			) {
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), BEHIND as uint) {
					value |= AZIMUTH_BEHIND;
				}
				else {
					*ctx = orig_context;
					return CSS_INVALID;
				}
			} else if (
				match (token.token_type) {
					CSS_TOKEN_IDENT(_) => false,
					_=> true
			 	}&& value == AZIMUTH_BEHIND
			){
				value |= AZIMUTH_CENTER;
			}
		} else{
			//css__parse_unit_specifier(c, vector, ctx, CSS_UNIT_DEG,	&length, &unit);

		}
		CSS_OK
	}

	fn css__parse_background(strings: &mut ~css_propstrings, vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_attachment(strings: &mut ~css_propstrings, vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_image(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_position(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_repeat(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_collapse(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_spacing(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_bottom(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_break_after(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_break_before(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_break_inside(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_caption_side(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_clear(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_clip(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_columns(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_count(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_fill(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_gap(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}
	fn css__parse_column_rule(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_rule_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_rule_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_rule_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_span(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_content(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_counter_increment(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_counter_reset(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cue(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cue_after(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cue_before(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cursor(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_direction(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_display(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_elevation(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_empty_cells(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_float(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_family(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_size(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_variant(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_weight(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_height(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_left(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_letter_spacing(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_line_height(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style_image(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style_position(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style_type(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}
	fn css__parse_margin_bottom(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin_left(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin_right(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin_top(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_max_height(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_max_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_min_height(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_min_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_opacity(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_orphans(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline_color(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline_style(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_overflow(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_bottom(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_left(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_right(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_top(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_page_break_after(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_page_break_before(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_page_break_inside(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pause(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pause_after(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pause_before(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pitch_range(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pitch(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_play_during(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_position(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_quotes(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_richness(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_right(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak_header(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak_numeral(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak_punctuation(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speech_rate(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_stress(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_table_layout(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_align(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_decoration(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_indent(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_transform(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_top(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_unicode_bidi(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_vertical_align(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_visibility(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_voice_family(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_volume(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_white_space(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_widows(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_width(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_word_spacing(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_z_index(strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	// fn css__parse_unit_specifier(&mut self , vector: &~[~css_token] , ctx: @mut int , default_unit: u32) -> (Option<int> , Option<uint>, css_result) {

	// 	consumeWhitespace(vector , ctx);
	// 	if *ctx >= vector.len() {
	// 		return (None , None , CSS_INVALID)
	// 	}	

	// 	match token.token_type {
	// 		CSS_TOKEN_DIMENSION(_)||CSS_TOKEN_NUMBER(_)||CSS_TOKEN_PERCENTAGE(_) => {
	// 			let num = 
	// 		},
	// 		_ => return(None , None , CSS_INVALID)
	// 	}
	// }

}

pub fn css__parse_unit_keyword(ptr:~str)-> (Option<css_unit>,css_result) {
	let mut unit: css_unit = CSS_UNIT_GRAD;
	let len:uint= ptr.len();
	match(len) {
		4=>if eq(&(ptr.to_lower()),&~"grad") {
              unit= CSS_UNIT_GRAD;    
			},
		3=>{
			if eq(&(ptr.to_lower()),&~"KHz") {
            	unit= CSS_UNIT_KHZ;    
			}
			else if eq(&(ptr.to_lower()),&~"deg") {
            	unit= CSS_UNIT_DEG;    
			}
			else if eq(&(ptr.to_lower()),&~"rad") {
            	unit= CSS_UNIT_RAD;    
			}
			else {
				return (None,CSS_INVALID);
			}
		},
		2=>{
			if eq(&(ptr.to_lower()),&~"Hz") {
            	unit= CSS_UNIT_HZ;    
			}
			else if eq(&(ptr.to_lower()),&~"ms") {
            	unit= CSS_UNIT_MS;    
			}
			else if eq(&(ptr.to_lower()),&~"px") {
            	unit= CSS_UNIT_PX;    
			}
			else if eq(&(ptr.to_lower()),&~"ex") {
            	unit= CSS_UNIT_EX;    
			}
			else if eq(&(ptr.to_lower()),&~"em") {
            	unit= CSS_UNIT_EM;    
			}
			else if eq(&(ptr.to_lower()),&~"in") {
            	unit= CSS_UNIT_IN;    
			}
			else if eq(&(ptr.to_lower()),&~"cm") {
            	unit= CSS_UNIT_CM;    
			}
			else if eq(&(ptr.to_lower()),&~"mm") {
            	unit= CSS_UNIT_MM;    
			}
			else if eq(&(ptr.to_lower()),&~"pt") {
            	unit= CSS_UNIT_PT;    
			}
			else if eq(&(ptr.to_lower()),&~"pc") {
            	unit= CSS_UNIT_PC;    
			}
			else {
				return (None,CSS_INVALID);
			}
		},
		1=>{
			if eq(&(ptr.to_lower()),&~"s") {
            	unit= CSS_UNIT_S;    
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

pub fn consumeWhitespace(vector:&~[~css_token], ctx:@mut uint) {
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

pub fn css__number_from_string(data: ~str, data_index:uint, int_only: bool) -> (int , uint){

	let mut length = data.len();
	let mut ptr = copy data;
	let mut sign = 1;
	let mut intpart = 0;
	let mut fracpart = 0;
	let mut pwr = 1;
	let mut ret_value = 0;
	let mut index = 0;
	let mut consumed_length = 0;
	

	if data.is_empty()||length == 0 {
		return (ret_value , consumed_length);
	}

	// number = [+-]? ([0-9]+ | [0-9]* '.' [0-9]+) 

	// Extract sign, if any 
	if ptr[0] == '-' as u8 {
		sign = -1;
		length -= 1;
		index += 1;
	}
	else if ptr[0] == '+' as u8 {
		length -=1;
		index += 1;
	}

	if length == 0 {
		return (ret_value , consumed_length);
	}
	else {
		if ptr[0] == '.' as u8 {
			if length ==1 || (ptr[1] < ('0' as u8)) || (('9' as u8) < ptr[1]) {
				return (ret_value , consumed_length);
			}
		}
		else if (ptr[0] < ('0' as u8)) || (('9' as u8) < ptr[0]) {
			return (ret_value , consumed_length);
		}
	}

	while length>0 {
		if (ptr[0] < ('0' as u8))||(('9' as u8) < ptr[0]) {
			break
		}
		if intpart < (1<<22) {
			intpart *= 10;
			intpart += (ptr[0] as u8) - ('0' as u8);
		}
		index += 1;
		length -= 1;
	}

	if int_only == false && length > 1 && (ptr[0] == '.' as u8) && ('0' as u8 <= ptr[1] && ptr[1] <= '9' as u8) {
		index += 1;	
		length -= 1;

		while length >0 {
			if ((ptr[0] < '0' as u8))|| (('9' as u8) < ptr[0]) {
				break
			}

			if pwr < 1000000 {
				pwr *= 10;
				fracpart *= 10;
				fracpart += (ptr[0] - '0' as u8);
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

pub fn css__number_from_lwc_string(string: arc::RWARC<~lwc_string>, int_only: bool) -> (int , uint) {
	let mut ret_value = 0;
	let mut consumed_length = 0;

	if lwc::lwc_string_length(string.clone()) == 0 {
		return (ret_value , consumed_length);
	}
	css__number_from_string(lwc::lwc_string_data(string.clone()), 0, int_only)
}