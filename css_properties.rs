#[link(name = "css_properties", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod css_stylesheet;
extern mod css_language;

use css_stylesheet::*;
use css_language::*;

pub type handle =  @extern fn(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet) -> ();

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

	fn css__parse_azimuth(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_background(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_background_attachment(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_background_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_background_image(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_background_position(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_background_repeat(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_bottom(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_bottom_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_bottom_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_bottom_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_collapse(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_left(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_left_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_left_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_left_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_right(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_right_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_right_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_right_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_spacing(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_top(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_top_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_top_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_top_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_border_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_bottom(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_break_after(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_break_before(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_break_inside(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_caption_side(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_clear(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_clip(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_columns(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_count(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_fill(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_gap(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_rule(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_rule_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_rule_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_rule_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_span(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_column_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_content(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_counter_increment(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_counter_reset(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_cue(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_cue_after(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_cue_before(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_cursor(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_direction(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_display(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_elevation(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_empty_cells(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_float(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_font(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_font_family(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_font_size(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_font_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_font_variant(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_font_weight(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_height(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_left(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_letter_spacing(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_line_height(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_list_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_list_style_image(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_list_style_position(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_list_style_type(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_margin(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_margin_bottom(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_margin_left(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_margin_right(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_margin_top(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_max_height(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_max_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_min_height(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_min_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_opacity(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_orphans(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_outline(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_outline_color(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_outline_style(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_outline_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_overflow(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_padding(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_padding_bottom(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_padding_left(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_padding_right(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_padding_top(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_page_break_after(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_page_break_before(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_page_break_inside(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_pause(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_pause_after(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_pause_before(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_pitch_range(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_pitch(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_play_during(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_position(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_quotes(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_richness(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_right(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_speak_header(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_speak_numeral(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_speak_punctuation(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_speak(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_speech_rate(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_stress(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_table_layout(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_text_align(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_text_decoration(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_text_indent(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_text_transform(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_top(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_unicode_bidi(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_vertical_align(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_visibility(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_voice_family(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_volume(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_white_space(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_widows(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_width(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_word_spacing(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	fn css__parse_z_index(vector:&~[~css_token], context: @mut uint, style: @mut css_stylesheet)->() {
	}

	

}