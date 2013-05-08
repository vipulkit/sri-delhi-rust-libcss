#[link(name = "css_properties", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod css_stylesheet;
extern mod css_propstrings;
extern mod css_enum;
extern mod wapcaplet;
extern mod css_fpmath;

//extern mod css_propstrings_parallel;

use css_stylesheet::*;
use css_propstrings::*;
use css_enum::*;
use wapcaplet::*;
use std::arc;
use core::str::*;
use css_fpmath::*;

//use css_propstrings::*; 
pub struct css_token {
	token_type: css_token_type,
	idata: Option<arc::RWARC<~lwc_string>>,
	// col: u32,
	// line: u32
}

pub type handle =  @extern fn(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style) ->css_result;

pub struct css_properties {
	property_handlers: ~[handle],
	sheet: @mut css_stylesheet
}

impl css_properties {

	pub fn css_properties(sheet_instance: @mut css_stylesheet) -> ~css_properties {

		let mut vec = ~[
			@css_properties::css__parse_azimuth , //0
			@css_properties::css__parse_background , //1
			@css_properties::css__parse_background_attachment, //2
			@css_properties::css__parse_background_color, //3
			@css_properties::css__parse_background_image, //4
			@css_properties::css__parse_background_position, //5
			@css_properties::css__parse_background_repeat, //6
			@css_properties::css__parse_border, //7
			@css_properties::css__parse_border_bottom, //8
			@css_properties::css__parse_border_bottom_color, //9 
			@css_properties::css__parse_border_bottom_style, //10
			@css_properties::css__parse_border_bottom_width, //11
			@css_properties::css__parse_border_collapse, //12
			@css_properties::css__parse_border_color, //13
			@css_properties::css__parse_border_left, //14
			@css_properties::css__parse_border_left_color, //15
			@css_properties::css__parse_border_left_style, //16
			@css_properties::css__parse_border_left_width, //17
			@css_properties::css__parse_border_right, //18
			@css_properties::css__parse_border_right_color, //19
			@css_properties::css__parse_border_right_style, //20
			@css_properties::css__parse_border_right_width, //21		
			@css_properties::css__parse_border_spacing, //22
			@css_properties::css__parse_border_style, //23
			@css_properties::css__parse_border_top, //24
			@css_properties::css__parse_border_top_color, //25
			@css_properties::css__parse_border_top_style, //26
			@css_properties::css__parse_border_top_width, //27
			@css_properties::css__parse_border_width, //28
			@css_properties::css__parse_bottom, //29
			@css_properties::css__parse_break_after, //30
			@css_properties::css__parse_break_before, //31
			@css_properties::css__parse_break_inside, //32
			@css_properties::css__parse_caption_side, //33
			@css_properties::css__parse_clear, //34
			@css_properties::css__parse_clip, //35
			@css_properties::css__parse_color, //36
			@css_properties::css__parse_columns, //37
			@css_properties::css__parse_column_count, //38
			@css_properties::css__parse_column_fill, //39
			@css_properties::css__parse_column_gap, //40
			@css_properties::css__parse_column_rule, //41
			@css_properties::css__parse_column_rule_color, //42
			@css_properties::css__parse_column_rule_style, //43
			@css_properties::css__parse_column_rule_width, //44
			@css_properties::css__parse_column_span, //45
			@css_properties::css__parse_column_width, //46
			@css_properties::css__parse_content, //47
			@css_properties::css__parse_counter_increment, //48
			@css_properties::css__parse_counter_reset, //49
			@css_properties::css__parse_cue, //50
			@css_properties::css__parse_cue_after, //51
			@css_properties::css__parse_cue_before, //52
			@css_properties::css__parse_cursor, //53
			@css_properties::css__parse_direction, //54
			@css_properties::css__parse_display, //55
			@css_properties::css__parse_elevation, //56
			@css_properties::css__parse_empty_cells, //57
			@css_properties::css__parse_float, //58
			@css_properties::css__parse_font, //59
			@css_properties::css__parse_font_family, //60
			@css_properties::css__parse_font_size, //61
			@css_properties::css__parse_font_style, //62
			@css_properties::css__parse_font_variant, //63
			@css_properties::css__parse_font_weight, //64
			@css_properties::css__parse_height, //65
			@css_properties::css__parse_left, //66
			@css_properties::css__parse_letter_spacing, //67
			@css_properties::css__parse_line_height, //68
			@css_properties::css__parse_list_style, //69
			@css_properties::css__parse_list_style_image, //70
			@css_properties::css__parse_list_style_position, //71
			@css_properties::css__parse_list_style_type, //72
			@css_properties::css__parse_margin, //73
			@css_properties::css__parse_margin_bottom, //74
			@css_properties::css__parse_margin_left, //75
			@css_properties::css__parse_margin_right, //76
			@css_properties::css__parse_margin_top, //77
			@css_properties::css__parse_max_height, //78
			@css_properties::css__parse_max_width, //79
			@css_properties::css__parse_min_height, //80
			@css_properties::css__parse_min_width, //81
			@css_properties::css__parse_opacity, //82
			@css_properties::css__parse_orphans, //83
			@css_properties::css__parse_outline, //84
			@css_properties::css__parse_outline_color, //85
			@css_properties::css__parse_outline_style, //86
			@css_properties::css__parse_outline_width, //87
			@css_properties::css__parse_overflow, //88
			@css_properties::css__parse_padding, //89
			@css_properties::css__parse_padding_bottom, //90
			@css_properties::css__parse_padding_left, //91
			@css_properties::css__parse_padding_right, //92
			@css_properties::css__parse_padding_top, //93
			@css_properties::css__parse_page_break_after, //94
			@css_properties::css__parse_page_break_before, //95
			@css_properties::css__parse_page_break_inside, //96
			@css_properties::css__parse_pause, //97
			@css_properties::css__parse_pause_after, //98
			@css_properties::css__parse_pause_before, //99
			@css_properties::css__parse_pitch_range, //100
			@css_properties::css__parse_pitch, //101
			@css_properties::css__parse_play_during, //102
			@css_properties::css__parse_position, //103
			@css_properties::css__parse_quotes, //104
			@css_properties::css__parse_richness, //105
			@css_properties::css__parse_right, //106
			@css_properties::css__parse_speak_header, //107
			@css_properties::css__parse_speak_numeral, //108
			@css_properties::css__parse_speak_punctuation, //109
			@css_properties::css__parse_speak, //110
			@css_properties::css__parse_speech_rate, //111
			@css_properties::css__parse_stress, //112
			@css_properties::css__parse_table_layout, //113
			@css_properties::css__parse_text_align, //114
			@css_properties::css__parse_text_decoration, //115
			@css_properties::css__parse_text_indent, //116
			@css_properties::css__parse_text_transform, //117
			@css_properties::css__parse_top, //118
			@css_properties::css__parse_unicode_bidi, //119
			@css_properties::css__parse_vertical_align, //120
			@css_properties::css__parse_visibility, //121
			@css_properties::css__parse_voice_family, //122
			@css_properties::css__parse_volume, //123
			@css_properties::css__parse_white_space, //124
			@css_properties::css__parse_widows, //125
			@css_properties::css__parse_width, //126
			@css_properties::css__parse_word_spacing, //127
			@css_properties::css__parse_z_index, //128
		];
		
		~css_properties{
			property_handlers: vec,
			sheet: sheet_instance
		}
	}

	fn css__parse_azimuth(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], ctx: @mut uint, style: @mut css_style)->css_result {
	    let orig_context:uint = *ctx;
		let mut flags:u8 = 0;
		let mut  value:u16 = 0;
		let mut unit:u32 = 0;
		let mut return_length_val = None;
		let mut return_unit_val = None;

		if *ctx >= vector.len() {
			return CSS_INVALID;
		}
		
		let mut token=&vector[*ctx];
		if ( 
			match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			} && strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), INHERIT as uint) 
		) {
			*ctx += 1;
			flags = FLAG_INHERIT  ;

		}
		else if ( 
			match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			} && strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFTWARDS as uint)
		) {
			*ctx += 1;
			value = AZIMUTH_LEFTWARDS;
		}

		
		else if ( 
			match (token.token_type) {
				CSS_TOKEN_IDENT(_) => true,
				_=> false
			 } && strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHTWARDS as uint) 
			) {
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
				*ctx += 1;
				value = AZIMUTH_LEFT_SIDE;

			}

			else if ( 
			strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), FAR_LEFT as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_FAR_LEFT;

			}
			else if ( 
			strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LEFT as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_LEFT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER_LEFT as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_CENTER_LEFT;

			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), CENTER as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_CENTER;
			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(),CENTER_RIGHT as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_CENTER_RIGHT;
			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHT as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_RIGHT;
			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), FAR_RIGHT as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_FAR_RIGHT;
			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), RIGHT_SIDE as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_RIGHT_SIDE;
			}
			else if ( 
			 strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), BEHIND as uint) 
			) {
				*ctx += 1;
				value = AZIMUTH_BEHIND;
			}
			else {
				*ctx = orig_context;
				return CSS_INVALID;
			}
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
			} 
			else if (
				match (token.token_type) {
					CSS_TOKEN_IDENT(_) => false,
					_=> true
			 	}&& value == AZIMUTH_BEHIND
			){
				value |= AZIMUTH_CENTER;
			}
		} 
		else{
			let (length_val , unit_val , result) = css_properties::css__parse_unit_specifier(sheet , vector, ctx, UNIT_DEG as u32);
			return_length_val = length_val;
			return_unit_val = unit_val;
			match result {
				CSS_OK => {},
				_ => {
					*ctx = orig_context;
					return result;
				}
			}
			if (unit_val.unwrap() & UNIT_ANGLE as u32) == 0 {
				*ctx = orig_context;
				return CSS_INVALID;
			}
			if (unit_val.unwrap() == UNIT_DEG as u32) {
				if ((length_val.unwrap() < -F_400) || (length_val.unwrap() > F_360)) {
					*ctx = orig_context;
					return CSS_INVALID;
				}
			}
			else if (unit_val.unwrap() == UNIT_GRAD as u32) {
				if ((length_val.unwrap() < -F_400) || (length_val.unwrap() > F_400)) {
					*ctx = orig_context;
					return CSS_INVALID;
				}
			} 
			else if (unit_val.unwrap() == UNIT_RAD as u32) {
				if ((length_val.unwrap() < -F_2PI) || (length_val.unwrap() > F_2PI)) {
					*ctx = orig_context;
					return CSS_INVALID;
				}
			}
			value = AZIMUTH_ANGLE;
		}

		css_stylesheet::css__stylesheet_style_appendOPV(style,CSS_PROP_AZIMUTH, flags, value);

		if (((flags & FLAG_INHERIT) == 0) && (value == AZIMUTH_ANGLE)) {
			css_stylesheet::css__stylesheet_style_vappend(style, [return_length_val.unwrap() as u32 , return_unit_val.unwrap() as u32]);
		}
		CSS_OK
	}

	fn css__parse_background(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings, vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_attachment(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings, vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_image(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_position(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_background_repeat(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_bottom_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_collapse(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		let orig_ctx = *context;
		let mut prev_ctx: uint;
		let mut token: &~css_token;
		let mut side_count: u32	= 0;

		if *context >= vector.len() {
			return CSS_INVALID;
		}
		
		token=&vector[*context];

		if css_properties::is_css_inherit(strings , token) {
			css_stylesheet::css_stylesheet_style_inherit(style, CSS_PROP_BORDER_TOP_COLOR);
			css_stylesheet::css_stylesheet_style_inherit(style, CSS_PROP_BORDER_RIGHT_COLOR);
			css_stylesheet::css_stylesheet_style_inherit(style, CSS_PROP_BORDER_BOTTOM_COLOR);
			css_stylesheet::css_stylesheet_style_inherit(style, CSS_PROP_BORDER_LEFT_COLOR);
			*context = *context + 1;
		}

		prev_ctx = *context;
		let mut side_val_vec: ~[u16] = ~[]; 
		let mut side_color_vec: ~[u32] = ~[];
		while  ((*context != prev_ctx) && (side_count < 4)) {
			if css_properties::is_css_inherit(strings , token) {
				*context = orig_ctx;
				return CSS_INVALID;
			}
			let (side_val,side_color , result) = css_properties::css__parse_color_specifier(sheet , strings , vector , context);

			match result {
				CSS_OK => {
					side_count += 1;
					consumeWhitespace(vector , context);
					token=&vector[*context];
					side_val_vec.push(side_val.unwrap());
					side_color_vec.push(side_color.unwrap());
				},
				_ => {
					break
				}
			}
		}

		match side_count {
			1 => {
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_TOP_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_RIGHT_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_BOTTOM_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_LEFT_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
			},
			2 => {
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_TOP_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_RIGHT_COLOR , 0 , side_val_vec[1]);
				if side_val_vec[1] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[1] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_BOTTOM_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_LEFT_COLOR , 0 , side_val_vec[1]);
				if side_val_vec[1] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[1] as u32)
				}
			},
			3 => {
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_TOP_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_RIGHT_COLOR , 0 , side_val_vec[1]);
				if side_val_vec[1] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[1] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_BOTTOM_COLOR , 0 , side_val_vec[2]);
				if side_val_vec[2] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[2] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_LEFT_COLOR , 0 , side_val_vec[1]);
				if side_val_vec[1] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[1] as u32)
				}
			},
			4 => {
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_TOP_COLOR , 0 , side_val_vec[0]);
				if side_val_vec[0] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[0] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_RIGHT_COLOR , 0 , side_val_vec[1]);
				if side_val_vec[1] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[1] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_BOTTOM_COLOR , 0 , side_val_vec[2]);
				if side_val_vec[2] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[2] as u32)
				}
				css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_BORDER_LEFT_COLOR , 0 , side_val_vec[3]);
				if side_val_vec[3] == BORDER_COLOR_SET as u16 {
					css_stylesheet::css__stylesheet_style_append(style , side_val_vec[3] as u32)
				}
			},
			_ => {
				*context = orig_ctx;
				return CSS_INVALID;
			}
		}
		CSS_OK
	}

	fn css__parse_border_left(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_left_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_right_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_spacing(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_top_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_border_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_bottom(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_break_after(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_break_before(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_break_inside(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_caption_side(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_clear(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_clip(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		let orig_ctx = *context;
		let mut token: &~css_token;
		let mut num_lengths: int = 0;
		let mut length: ~[i32] = ~[];
		let mut unit: ~[u32] = ~[]; 

		if *context >= vector.len() {
			return CSS_INVALID;
		}
		
		token=&vector[*context];
		*context = *context + 1;

		match token.token_type {
			CSS_TOKEN_IDENT(_) => {
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , INHERIT as uint) {
					css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_CLIP , FLAG_INHERIT as u8 , 0);
				}
				else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , AUTO as uint) {
					css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_CLIP , 0 , CLIP_AUTO as u16);
				}
			},
			CSS_TOKEN_FUNCTION(_) => {
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , RECT as uint) {
					let mut i: int = 0;
					let mut value: u16 = CLIP_SHAPE_RECT as u16;

					while i < 4 {
						consumeWhitespace(vector , context);

						if *context >= vector.len() {
							*context = orig_ctx;
							return CSS_INVALID;
						}

						token=&vector[*context];

						match token.token_type {
							CSS_TOKEN_IDENT(_) => {
								if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , AUTO as uint) {
									value |= 1 << (i+3);
								}
								else {
									*context = orig_ctx;
									return CSS_INVALID;
								}
								*context = *context + 1;
							}
							_ => {
								let (length_val , unit_val , result) = css_properties::css__parse_unit_specifier(sheet , vector, context, UNIT_PX as u32);
								
								match result {
									CSS_OK => {},
									_ => {
										*context = orig_ctx;
										return result;
									}
								}
								length.push(length_val.unwrap() as i32);
								unit.push(unit_val.unwrap());
								if (unit[num_lengths] & (UNIT_ANGLE as u32)) > 0 {
									*context = orig_ctx;
									return CSS_INVALID;
								}
								if (unit[num_lengths] & (UNIT_TIME as u32)) > 0{
									*context = orig_ctx;
									return CSS_INVALID;
								}
								if (unit[num_lengths] & (UNIT_FREQ as u32)) > 0{
									*context = orig_ctx;
									return CSS_INVALID;
								}
								if (unit[num_lengths] & (UNIT_PCT as u32)) > 0{
									*context = orig_ctx;
									return CSS_INVALID;
								}
								num_lengths += 1;
							}
									
						}
						consumeWhitespace(vector , context);
						if i<3 {
							if *context >= vector.len() {
								*context = orig_ctx;
								return CSS_INVALID;
							}
							token=&vector[*context];
							if tokenIsChar(token , ',') {
								*context = *context + 1;
							}
						}
						i += 1;
					}
					consumeWhitespace(vector , context);
					if *context >= vector.len() {
						*context = orig_ctx;
						return CSS_INVALID;
					}
					token=&vector[*context];
					*context = *context + 1;

					if (tokenIsChar(token , ')') == false) {
						*context = orig_ctx;
						return CSS_INVALID;
					}
					css_stylesheet::css__stylesheet_style_appendOPV(style , CSS_PROP_CLIP , 0 , value);

					while i < num_lengths {
						css_stylesheet::css__stylesheet_style_vappend(style , unit);
						num_lengths += 1;
					}
				}
			},
			_ => {
				*context = orig_ctx;
				return CSS_INVALID;
			}
		}

		CSS_OK
	}

	fn css__parse_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_columns(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_count(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_fill(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_gap(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}
	fn css__parse_column_rule(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_rule_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_rule_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_rule_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_span(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_column_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_content(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_counter_increment(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_counter_reset(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cue(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cue_after(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cue_before(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_cursor(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_direction(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_display(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_elevation(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_empty_cells(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_float(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_family(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_size(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_variant(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_font_weight(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_height(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_left(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_letter_spacing(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_line_height(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style_image(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style_position(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_list_style_type(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}
	fn css__parse_margin_bottom(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin_left(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin_right(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_margin_top(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_max_height(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_max_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_min_height(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_min_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_opacity(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_orphans(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline_style(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_outline_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_overflow(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_bottom(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_left(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_right(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_padding_top(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_page_break_after(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_page_break_before(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_page_break_inside(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pause(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pause_after(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pause_before(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pitch_range(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_pitch(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_play_during(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_position(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_quotes(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_richness(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_right(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak_header(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak_numeral(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak_punctuation(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speak(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_speech_rate(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_stress(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_table_layout(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_align(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_decoration(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		let mut orig_context:uint= *context;
		let mut result:css_result= CSS_INVALID;
		let mut token: &~css_token;

		if *context >= vector.len() {
			return CSS_INVALID;
		}
		token=&vector[*context];
		*context += 1;

		match token.token_type {
			CSS_TOKEN_IDENT(_) => {
				if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), INHERIT as uint) {
					css_stylesheet::css_stylesheet_style_inherit(style, CSS_PROP_TEXT_DECORATION);
				}
				else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), NONE as uint)  {
					css_stylesheet::css__stylesheet_style_appendOPV(style,CSS_PROP_TEXT_DECORATION, 0, TEXT_DECORATION_NONE as u16);
				}
				else {
					let mut value: u16 = 0 ;
					while (*context < vector.len()) {
						if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), UNDERLINE as uint) {
							if ((value & TEXT_DECORATION_UNDERLINE as u16) == 0) {
								value |= TEXT_DECORATION_UNDERLINE as u16;
							}
							
							else {
								*context = orig_context;
								return CSS_INVALID;
							}
						}
						else if  strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), OVERLINE as uint) {
						    if ((value & TEXT_DECORATION_OVERLINE as u16) == 0) {
								value |= TEXT_DECORATION_OVERLINE as u16;
							}
							else {
								*context = orig_context;
								return CSS_INVALID;
							}
						}
						else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), LINE_THROUGH as uint) {
							if ((value & TEXT_DECORATION_LINE_THROUGH as u16) == 0) {
								value |= TEXT_DECORATION_LINE_THROUGH as u16;
							}
							else {
								*context = orig_context;
								return CSS_INVALID;

							}
						}
						else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone(), BLINK as uint) {
							if ((value & (TEXT_DECORATION_BLINK as u16)) == 0) {
								value |= TEXT_DECORATION_BLINK as u16;
							}
							else {
								*context = orig_context;
								return CSS_INVALID;
							}
						}
						else {
							*context = orig_context;
							return CSS_INVALID;
						}
						consumeWhitespace(vector, context);
						token=&vector[*context];
						*context += 1;
						match (token.token_type) {
							CSS_TOKEN_IDENT(_) => {

							},
							_=> {
								break;
							}
						}
						css_stylesheet::css__stylesheet_style_appendOPV(style,	CSS_PROP_TEXT_DECORATION, 0, value);
					}
				}
			},
			_=> {
		 		*context = orig_context;
				return CSS_INVALID;
			}
		}
		CSS_OK
	}

	fn css__parse_text_indent(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_text_transform(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_top(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_unicode_bidi(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_vertical_align(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_visibility(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_voice_family(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_volume(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_white_space(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_widows(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_width(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_word_spacing(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_z_index(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings ,vector:&~[~css_token], context: @mut uint, style: @mut css_style)->css_result {
		CSS_OK
	}

	fn css__parse_named_color(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings , data: arc::RWARC<~lwc_string>) -> (Option<index_property> , css_result){
		// static vector_length: uint = (index_property::YELLOWGREEN as uint) + 1 - (index_property::ALICEBLUE as uint);
		let mut result_val: index_property;
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
			result_val = unsafe {cast::transmute(index - (ALICEBLUE as uint))};
			return (Some(result_val) , CSS_OK);
		}

		//TODO----
		// if (sheet->color != NULL)
		// return sheet->color(c->sheet->color_pw, data, result);

		return(None , CSS_INVALID);
	}

	fn css__parse_hash_colour(data: arc::RWARC<~lwc_string>) -> (Option<u32> , css_result){
		let mut result_val: u32;
		let mut r: u8 = 0;
		let mut g: u8 = 0;
		let mut b: u8 = 0;
		let mut a: u8 = 0xff;
		let input_length = lwc::lwc_string_length(data.clone());
		let input_string = lwc::lwc_string_data(data.clone());

		if (input_length == 3 && isHex(input_string[0]) && isHex(input_string[1]) && isHex(input_string[2])) {
			r = charToHex(input_string[0]) as u8;
			g = charToHex(input_string[1]) as u8;
			b = charToHex(input_string[2]) as u8;

			r |= (r << 4);
			g |= (g << 4);
			b |= (b << 4);
		}
		else if (input_length == 6 && isHex(input_string[0]) && isHex(input_string[1]) && 	isHex(input_string[2]) && isHex(input_string[3]) && isHex(input_string[4]) && isHex(input_string[5])) {
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

	fn css__parse_color_specifier(sheet: @mut css_stylesheet , strings: &mut ~css_propstrings , vector: &~[~css_token] , ctx: @mut uint) -> (Option<u16> , Option<u32> , css_result) {
		// TODO's
		let mut token:&~css_token;
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
					ret_value = COLOR_TRANSPARENT as u16;
					ret_result = 0;
					return (Some(ret_value) , Some(ret_result) , CSS_OK);
				}
				else if strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , CURRENTCOLOR as uint) {
					ret_value = COLOR_CURRENT_COLOR as u16;
					ret_result = 0;
					return (Some(ret_value) , Some(ret_result) , CSS_OK);
				}
				let (color_value , error) = css_properties::css__parse_named_color(sheet , strings , token.idata.get_ref().clone());
				match error {
					CSS_OK => {},
					_ => {
						if sheet.quirks_allowed {
							let(hash_result , error_from_hash) = css_properties::css__parse_hash_colour(token.idata.get_ref().clone());
							match error_from_hash {
								CSS_OK => sheet.quirks_used = true,
								_ => {}
							}
						}
						goto_flag = true;
					}
				}
			},

			CSS_TOKEN_HASH(_) => {
				let(hash_result , error_from_hash) = css_properties::css__parse_hash_colour(token.idata.get_ref().clone());
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
					// TODO
					let mut valid: Option<css_token_type> = None;
					let components: ~[u8] = ~[
						r , g , b , a
					];

					while i < colour_channels {
						let mut component: u8;
						let mut consumed: uint = 0;
						let mut intval: i32 = 0;
						let mut int_only: bool = false;

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

							//TODO
							// valid = Some(copy token.token_type);
						}

						// TODO
						// else if i<3 &&{
						// 	int_only = false;
						// }

						if i<3 {
							// TODO
							// int_only = match valid.unwrap() {
							// 	CSS_TOKEN_NUMBER(_ , _) => true,
							// 	_=> false
							// };
						}
						else {
							int_only = false;
						}
						let (num , consumed_index) = css__number_from_lwc_string(token.idata.get_ref().clone() , int_only);

						if consumed_index != lwc::lwc_string_length(token.idata.get_ref().clone()) {
							goto_flag = true;
						}
						//TODO
						// match valid

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
					let consumed: uint = 0;
					let mut hue: i32;
					let mut sat: i32;
					let mut lit: i32;
					let mut alpha: i32 = 255;

					consumeWhitespace(vector , ctx);

					token = &vector[*ctx];
					*ctx = *ctx + 1;
					match token.token_type {
						CSS_TOKEN_NUMBER(_ , _) => {},
						_ => goto_flag = true
					}
					let mut (hue_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
					hue = hue_res as i32;
					if consumed_length_from_lwc_string != lwc::lwc_string_length(token.idata.get_ref().clone()) {
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
					let mut (sat_res , consumed_length_from_lwc_string) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);
					sat = sat_res as i32;
					if consumed_length_from_lwc_string != lwc::lwc_string_length(token.idata.get_ref().clone()) {
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
					if consumed_length_from_lwc_string != lwc::lwc_string_length(token.idata.get_ref().clone()) {
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
						if consumed_length_from_lwc_string != lwc::lwc_string_length(token.idata.get_ref().clone()) {
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
				ret_value = COLOR_SET as u16;
			},
			CSS_TOKEN_NUMBER(_ , _) => {
				if sheet.quirks_allowed {
					let(hash_result , error_from_hash) = css_properties::css__parse_hash_colour(token.idata.get_ref().clone());
					match error_from_hash {
						CSS_OK => {
							sheet.quirks_used = true
						},
						_ => {
							goto_flag = true;
						}
					}
				}
				else {
					goto_flag = true;
				}
			},
			CSS_TOKEN_DIMENSION(_ , _ , _) => {
				if sheet.quirks_allowed {
					let(hash_result , error_from_hash) = css_properties::css__parse_hash_colour(token.idata.get_ref().clone());
					match error_from_hash {
						CSS_OK => {
							sheet.quirks_used = true
						},
						_ => {
							goto_flag = true;
						}
					}
				}
				else {
					goto_flag = true;
				}
			},
			_ => {
				return (None , None , CSS_INVALID);
			}
		}

		if goto_flag {
			*ctx = orig_ctx;
			return (None , None , CSS_INVALID);	
		}
		(Some(ret_value) , Some(ret_result) , CSS_OK)
	}

	fn css__parse_unit_specifier(sheet: @mut css_stylesheet, vector: &~[~css_token] , ctx: @mut uint , default_unit: u32) -> (Option<int> , Option<u32>, css_result) {

		consumeWhitespace(vector , ctx);
		let mut token:&~css_token;
		let mut unit_retVal:u32 = 0;
		let orig_ctx = *ctx;

		if *ctx >= vector.len() {
			return (None , None , CSS_INVALID)
		}
		token = &vector[*ctx];
		*ctx = *ctx + 1;

		match token.token_type {
			CSS_TOKEN_DIMENSION(_ , _ , _)|CSS_TOKEN_NUMBER(_ , _)|CSS_TOKEN_PERCENTAGE(_ , _) => {
			},
			_ => return(None , None , CSS_INVALID)
		}

		let (num , consumed_index) = css__number_from_lwc_string(token.idata.get_ref().clone() , false);

		match token.token_type {
			CSS_TOKEN_DIMENSION(_ , _ , _) => {
				let len = lwc::lwc_string_length(token.idata.get_ref().clone());
				let data = lwc::lwc_string_data(token.idata.get_ref().clone());

				let (unit , result) = css__parse_unit_keyword(data , consumed_index);
				match result {
					CSS_OK => {},
					_ => return (None , None , result)
				}
				unit_retVal = unit.unwrap() as u32;
			},
			CSS_TOKEN_NUMBER(_ , _) => {
				if num !=0 {
					if sheet.quirks_allowed {
						sheet.quirks_used = true;
					}
					else {
						return (None , None , CSS_INVALID)
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
							let (unit , result) = css__parse_unit_keyword(lwc::lwc_string_data(token.idata.get_ref().clone()) , 0);
							match  result {
								CSS_OK => {
									sheet.quirks_used = true;
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
				if lwc::lwc_string_length(token.idata.get_ref().clone()) != consumed_index {
					return (None , None , CSS_INVALID);
				}
				unit_retVal = UNIT_PCT as u32;
			}
		}
		return(Some(num) , Some(unit_retVal) , CSS_OK);
	}

	fn is_css_inherit(strings: &mut ~css_propstrings , token: &~css_token) ->bool {
		match token.token_type {
			CSS_TOKEN_IDENT(_) => {
				 return strings.lwc_string_caseless_isequal(token.idata.get_ref().clone() , INHERIT as uint);
			}
			_ => false
		}
	}

	// fn css__parse_border_side(sheet: @mut css_stylesheet, strings: &mut ~css_propstrings , vector: &~[~css_token] , ctx: @mut uint , result_style: @mut css_style , side: border_side_e) -> css_result { 
	// 	let orig_ctx = *ctx;
	// 	let mut prev_ctx: int;
	// 	let color: bool = true;
	// 	let style: bool = true;
	// 	let width: bool = true;
	// 	let color_style: @mut css_style;
	// 	let style_style: @mut css_style;
	// 	let width_style: @mut css_style;
	// 	let mut token: &~css_token;

	// 	if *ctx >= vector.len() {
	// 		return CSS_INVALID;
	// 	}

	// 	token = &vector[*ctx];
		
	// 	if (css_properties::is_css_inherit(strings , token)) {
	// 		css_stylesheet::css_stylesheet_style_inherit(result_style , unsafe{cast::transmute(CSS_PROP_BORDER_TOP_COLOR as uint + side as uint)});
	// 		css_stylesheet::css_stylesheet_style_inherit(result_style, unsafe{cast::transmute(CSS_PROP_BORDER_TOP_STYLE as uint + side as uint)});
	// 		css_stylesheet::css_stylesheet_style_inherit(result_style, unsafe{cast::transmute(CSS_PROP_BORDER_TOP_WIDTH as uint + side as uint)});
	// 	}

	// 	*ctx = *ctx + 1;
	// 	color_style = sheet.css__stylesheet_create();
	// 	style_style = sheet.css__stylesheet_create();
	// 	width_style = sheet.css__stylesheet_create();

	// 	prev_ctx = *ctx;
	// 	while *ctx != prev_ctx {
	// 		let mut error = CSS_OK;
	// 		token = &vector[*ctx];
	// 		if css_properties::is_css_inherit(strings , token) {
	// 			error = CSS_INVALID;
	// 			*ctx = orig_ctx;
	// 			return error;
	// 		}
	// 		if color && 
	// 	}
	// }
}

pub fn css__parse_unit_keyword(ptr:~str , index: uint)-> (Option<unit>,css_result) {
	let mut unit: unit = UNIT_GRAD;
	let len:uint= ptr.len() - index;
	match(len) {
		4=>if eq(&(ptr.to_lower()),&~"grad") {
              unit= UNIT_GRAD;    
			},
		3=>{
			if eq(&(ptr.to_lower()),&~"KHz") {
            	unit= UNIT_KHZ;    
			}
			else if eq(&(ptr.to_lower()),&~"deg") {
            	unit= UNIT_DEG;    
			}
			else if eq(&(ptr.to_lower()),&~"rad") {
            	unit= UNIT_RAD;    
			}
			else {
				return (None,CSS_INVALID);
			}
		},
		2=>{
			if eq(&(ptr.to_lower()),&~"Hz") {
            	unit= UNIT_HZ;    
			}
			else if eq(&(ptr.to_lower()),&~"ms") {
            	unit= UNIT_MS;    
			}
			else if eq(&(ptr.to_lower()),&~"px") {
            	unit= UNIT_PX;    
			}
			else if eq(&(ptr.to_lower()),&~"ex") {
            	unit= UNIT_EX;    
			}
			else if eq(&(ptr.to_lower()),&~"em") {
            	unit= UNIT_EM;    
			}
			else if eq(&(ptr.to_lower()),&~"in") {
            	unit= UNIT_IN;    
			}
			else if eq(&(ptr.to_lower()),&~"cm") {
            	unit= UNIT_CM;    
			}
			else if eq(&(ptr.to_lower()),&~"mm") {
            	unit= UNIT_MM;    
			}
			else if eq(&(ptr.to_lower()),&~"pt") {
            	unit= UNIT_PT;    
			}
			else if eq(&(ptr.to_lower()),&~"pc") {
            	unit= UNIT_PC;    
			}
			else {
				return (None,CSS_INVALID);
			}
		},
		1=>{
			if eq(&(ptr.to_lower()),&~"s") {
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

/**
 * Determine if a token is a character
 *
 * \param token  The token to consider
 * \param c      The character to match (lowerASCII only)
 * \return True if the token matches, false otherwise
 */
pub fn tokenIsChar(token:&~css_token, c:char) -> bool {
	let result = false;

	match token.token_type {
		CSS_TOKEN_CHAR(_) => {   
				if lwc::lwc_string_length(token.idata.get_ref().clone()) == 1 {
					let mut token_char = lwc::lwc_string_data(token.idata.get_ref().clone()).char_at(0);

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

pub fn css__number_from_lwc_string(string: arc::RWARC<~lwc_string>, int_only: bool) -> (int , uint) {
	let mut ret_value = 0;
	let mut consumed_length = 0;

	if lwc::lwc_string_length(string.clone()) == 0 {
		return (ret_value , consumed_length);
	}
	css__number_from_string(lwc::lwc_string_data(string.clone()), 0, int_only)
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
		let r = (css_divide_fixed(css_multiply_fixed((lit), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
		let g = (css_divide_fixed(css_multiply_fixed((lit), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
		let b = (css_divide_fixed(css_multiply_fixed((lit), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
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
    		let r = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let g = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let b = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			return (r as u8 , g as u8 , b as u8);
    	},
    	1 => {
    		let r = (css_divide_fixed(css_multiply_fixed((mid2), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let g = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let b = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			return (r as u8 , g as u8 , b as u8);
    	},
    	2 => {
    		let r = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let g = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let b = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			return (r as u8 , g as u8 , b as u8);
    	},
    	3 => {
    		let r = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let g = (css_divide_fixed(css_multiply_fixed((mid2), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let b = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			return (r as u8 , g as u8 , b as u8);
    	},
    	4 => {
    		let r = (css_divide_fixed(css_multiply_fixed((mid1), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let g = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let b = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			return (r as u8 , g as u8 , b as u8);
    	},
    	5 => {
    		let r = (css_divide_fixed(css_multiply_fixed((max_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let g = (css_divide_fixed(css_multiply_fixed((min_rgb), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			let b = (css_divide_fixed(css_multiply_fixed((mid2), F_255 as i32), F_100 as i32	)) >> CSS_RADIX_POINT;
			return (r as u8 , g as u8 , b as u8);
    	}
    	_ => { (0 , 0 , 0)}
    }
}