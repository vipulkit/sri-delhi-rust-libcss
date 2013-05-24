
use include::properties::*;
use utils::errors::*;
use stylesheet::*;
use select::common::*;
use select::properties::properties::*;

pub enum prop_group {
	GROUP_NORMAL	= 0x0,
	GROUP_UNCOMMON	= 0x1,
	GROUP_PAGE		= 0x2,
	GROUP_AURAL		= 0x3
}

pub struct prop_table {
	cascade : &'static fn (opv:u32, style:@mut css_style,
								state:@mut css_select_state)-> css_result,
	set_from_hint :  &'static fn (hint:@mut css_hint,
								style: @mut css_computed_style) -> css_result,
	initial :  &'static fn (state:@mut css_select_state) -> css_result,
	compose :  &'static fn (parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style) -> css_result,

	inherited:uint,
	group:prop_group
} 

// vector of size -- CSS_N_PROPERTIES
static prop_dispatch : &'static[&'static prop_table] = &[
	&prop_table { 
		cascade:css__cascade_azimuth, 
		set_from_hint:css__set_azimuth_from_hint,
		initial:css__initial_azimuth,
		compose:css__compose_azimuth,
		inherited:1,
		group:GROUP_AURAL
	},


	&prop_table { 
		cascade:css__cascade_azimuth ,
		set_from_hint:css__set_azimuth_from_hint ,
		initial:css__initial_azimuth ,
		compose:css__compose_azimuth ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_background_attachment ,
		set_from_hint:css__set_background_attachment_from_hint ,
		initial:css__initial_background_attachment ,
		compose:css__compose_background_attachment ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_background_color ,
		set_from_hint:css__set_background_color_from_hint ,
		initial:css__initial_background_color ,
		compose:css__compose_background_color ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_background_image ,
		set_from_hint:css__set_background_image_from_hint ,
		initial:css__initial_background_image ,
		compose:css__compose_background_image ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_background_position ,
		set_from_hint:css__set_background_position_from_hint ,
		initial:css__initial_background_position ,
		compose:css__compose_background_position ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_background_repeat ,
		set_from_hint:css__set_background_repeat_from_hint ,
		initial:css__initial_background_repeat ,
		compose:css__compose_background_repeat ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_collapse ,
		set_from_hint:css__set_border_collapse_from_hint ,
		initial:css__initial_border_collapse ,
		compose:css__compose_border_collapse ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_spacing ,
		set_from_hint:css__set_border_spacing_from_hint ,
		initial:css__initial_border_spacing ,
		compose:css__compose_border_spacing ,
		inherited:1 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_top_color ,
		set_from_hint:css__set_border_top_color_from_hint ,
		initial:css__initial_border_top_color ,
		compose:css__compose_border_top_color ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_right_color ,
		set_from_hint:css__set_border_right_color_from_hint ,
		initial:css__initial_border_right_color ,
		compose:css__compose_border_right_color ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_bottom_color ,
		set_from_hint:css__set_border_bottom_color_from_hint ,
		initial:css__initial_border_bottom_color ,
		compose:css__compose_border_bottom_color ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_left_color ,
		set_from_hint:css__set_border_left_color_from_hint ,
		initial:css__initial_border_left_color ,
		compose:css__compose_border_left_color ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_top_style ,
		set_from_hint:css__set_border_top_style_from_hint ,
		initial:css__initial_border_top_style ,
		compose:css__compose_border_top_style ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_right_style ,
		set_from_hint:css__set_border_right_style_from_hint ,
		initial:css__initial_border_right_style ,
		compose:css__compose_border_right_style ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_bottom_style ,
		set_from_hint:css__set_border_bottom_style_from_hint ,
		initial:css__initial_border_bottom_style ,
		compose:css__compose_border_bottom_style ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_left_style ,
		set_from_hint:css__set_border_left_style_from_hint ,
		initial:css__initial_border_left_style ,
		compose:css__compose_border_left_style ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_top_width ,
		set_from_hint:css__set_border_top_width_from_hint ,
		initial:css__initial_border_top_width ,
		compose:css__compose_border_top_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_right_width ,
		set_from_hint:css__set_border_right_width_from_hint ,
		initial:css__initial_border_right_width ,
		compose:css__compose_border_right_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_bottom_width ,
		set_from_hint:css__set_border_bottom_width_from_hint ,
		initial:css__initial_border_bottom_width ,
		compose:css__compose_border_bottom_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_border_left_width ,
		set_from_hint:css__set_border_left_width_from_hint ,
		initial:css__initial_border_left_width ,
		compose:css__compose_border_left_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_bottom ,
		set_from_hint:css__set_bottom_from_hint ,
		initial:css__initial_bottom ,
		compose:css__compose_bottom ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_caption_side ,
		set_from_hint:css__set_caption_side_from_hint ,
		initial:css__initial_caption_side ,
		compose:css__compose_caption_side ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_clear ,
		set_from_hint:css__set_clear_from_hint ,
		initial:css__initial_clear ,
		compose:css__compose_clear ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table {
	 	cascade:css__cascade_clip ,
		set_from_hint:css__set_clip_from_hint ,
		initial:css__initial_clip ,
		compose:css__compose_clip ,
		inherited:0 ,  
		group:GROUP_UNCOMMON ,
	} ,
		
	&prop_table { 
		cascade:css__cascade_color ,
		set_from_hint:css__set_color_from_hint ,
		initial:css__initial_color ,
		compose:css__compose_color ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_content ,
		set_from_hint:css__set_content_from_hint ,
		initial:css__initial_content ,
		compose:css__compose_content ,
		inherited:0 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_counter_increment ,
		set_from_hint:css__set_counter_increment_from_hint ,
		initial:css__initial_counter_increment ,
		compose:css__compose_counter_increment ,
		inherited:0 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_counter_reset ,
		set_from_hint:css__set_counter_reset_from_hint ,
		initial:css__initial_counter_reset ,
		compose:css__compose_counter_reset ,
		inherited:0 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_cue_after ,
		set_from_hint:css__set_cue_after_from_hint ,
		initial:css__initial_cue_after ,
		compose:css__compose_cue_after ,
		inherited:0 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_cue_before ,
		set_from_hint:css__set_cue_before_from_hint ,
		initial:css__initial_cue_before ,
		compose:css__compose_cue_before ,
		inherited:0 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_cursor ,
		set_from_hint:css__set_cursor_from_hint ,
		initial:css__initial_cursor ,
		compose:css__compose_cursor ,
		inherited:1 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_direction ,
		set_from_hint:css__set_direction_from_hint ,
		initial:css__initial_direction ,
		compose:css__compose_direction ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_display ,
		set_from_hint:css__set_display_from_hint ,
		initial:css__initial_display ,
		compose:css__compose_display ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_elevation ,
		set_from_hint:css__set_elevation_from_hint ,
		initial:css__initial_elevation ,
		compose:css__compose_elevation ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_empty_cells ,
		set_from_hint:css__set_empty_cells_from_hint ,
		initial:css__initial_empty_cells ,
		compose:css__compose_empty_cells ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_float ,
		set_from_hint:css__set_float_from_hint ,
		initial:css__initial_float ,
		compose:css__compose_float ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_font_family ,
		set_from_hint:css__set_font_family_from_hint ,
		initial:css__initial_font_family ,
		compose:css__compose_font_family ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_font_size ,
		set_from_hint:css__set_font_size_from_hint ,
		initial:css__initial_font_size ,
		compose:css__compose_font_size ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_font_style ,
		set_from_hint:css__set_font_style_from_hint ,
		initial:css__initial_font_style ,
		compose:css__compose_font_style ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_font_variant ,
		set_from_hint:css__set_font_variant_from_hint ,
		initial:css__initial_font_variant ,
		compose:css__compose_font_variant ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_font_weight ,
		set_from_hint:css__set_font_weight_from_hint ,
		initial:css__initial_font_weight ,
		compose:css__compose_font_weight ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_height ,
		set_from_hint:css__set_height_from_hint ,
		initial:css__initial_height ,
		compose:css__compose_height ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	
	&prop_table {
	 	cascade:css__cascade_left ,
		set_from_hint:css__set_left_from_hint ,
		initial:css__initial_left ,
		compose:css__compose_left ,
		inherited:0 ,  
		group:GROUP_NORMAL ,
	} ,
		
	   
	&prop_table { 
		cascade:css__cascade_letter_spacing ,
		set_from_hint:css__set_letter_spacing_from_hint ,
		initial:css__initial_letter_spacing ,
		compose:css__compose_letter_spacing ,
		inherited:1 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_line_height ,
		set_from_hint:css__set_line_height_from_hint ,
		initial:css__initial_line_height ,
		compose:css__compose_line_height ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_list_style_image ,
		set_from_hint:css__set_list_style_image_from_hint ,
		initial:css__initial_list_style_image ,
		compose:css__compose_list_style_image ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_list_style_position ,
		set_from_hint:css__set_list_style_position_from_hint ,
		initial:css__initial_list_style_position ,
		compose:css__compose_list_style_position ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_list_style_type ,
		set_from_hint:css__set_list_style_type_from_hint ,
		initial:css__initial_list_style_type ,
		compose:css__compose_list_style_type ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_margin_top ,
		set_from_hint:css__set_margin_top_from_hint ,
		initial:css__initial_margin_top ,
		compose:css__compose_margin_top ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_margin_right ,
		set_from_hint:css__set_margin_right_from_hint ,
		initial:css__initial_margin_right ,
		compose:css__compose_margin_right ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_margin_bottom ,
		set_from_hint:css__set_margin_bottom_from_hint ,
		initial:css__initial_margin_bottom ,
		compose:css__compose_margin_bottom ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_margin_left ,
		set_from_hint:css__set_margin_left_from_hint ,
		initial:css__initial_margin_left ,
		compose:css__compose_margin_left ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_max_height ,
		set_from_hint:css__set_max_height_from_hint ,
		initial:css__initial_max_height ,
		compose:css__compose_max_height ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_max_width ,
		set_from_hint:css__set_max_width_from_hint ,
		initial:css__initial_max_width ,
		compose:css__compose_max_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_min_height ,
		set_from_hint:css__set_min_height_from_hint ,
		initial:css__initial_min_height ,
		compose:css__compose_min_height ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_min_width ,
		set_from_hint:css__set_min_width_from_hint ,
		initial:css__initial_min_width ,
		compose:css__compose_min_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_orphans ,
		set_from_hint:css__set_orphans_from_hint ,
		initial:css__initial_orphans ,
		compose:css__compose_orphans ,
		inherited:1 ,
		group:GROUP_PAGE ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_outline_color ,
		set_from_hint:css__set_outline_color_from_hint ,
		initial:css__initial_outline_color ,
		compose:css__compose_outline_color ,
		inherited:0 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_outline_style ,
		set_from_hint:css__set_outline_style_from_hint ,
		initial:css__initial_outline_style ,
		compose:css__compose_outline_style ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_outline_width ,
		set_from_hint:css__set_outline_width_from_hint ,
		initial:css__initial_outline_width ,
		compose:css__compose_outline_width ,
		inherited:0 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_overflow ,
		set_from_hint:css__set_overflow_from_hint ,
		initial:css__initial_overflow ,
		compose:css__compose_overflow ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_padding_top ,
		set_from_hint:css__set_padding_top_from_hint ,
		initial:css__initial_padding_top ,
		compose:css__compose_padding_top ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_padding_right ,
		set_from_hint:css__set_padding_right_from_hint ,
		initial:css__initial_padding_right ,
		compose:css__compose_padding_right ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_padding_bottom ,
		set_from_hint:css__set_padding_bottom_from_hint ,
		initial:css__initial_padding_bottom ,
		compose:css__compose_padding_bottom ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_padding_left ,
		set_from_hint:css__set_padding_left_from_hint ,
		initial:css__initial_padding_left ,
		compose:css__compose_padding_left ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_page_break_after ,
		set_from_hint:css__set_page_break_after_from_hint ,
		initial:css__initial_page_break_after ,
		compose:css__compose_page_break_after ,
		inherited:0 ,
		group:GROUP_PAGE ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_page_break_before ,
		set_from_hint:css__set_page_break_before_from_hint ,
		initial:css__initial_page_break_before ,
		compose:css__compose_page_break_before ,
		inherited:0 ,
		group:GROUP_PAGE ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_page_break_inside ,
		set_from_hint:css__set_page_break_inside_from_hint ,
		initial:css__initial_page_break_inside ,
		compose:css__compose_page_break_inside ,
		inherited:1 ,
		group:GROUP_PAGE ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_pause_after ,
		set_from_hint:css__set_pause_after_from_hint ,
		initial:css__initial_pause_after ,
		compose:css__compose_pause_after ,
		inherited:0 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_pause_before ,
		set_from_hint:css__set_pause_before_from_hint ,
		initial:css__initial_pause_before ,
		compose:css__compose_pause_before ,
		inherited:0 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_pitch_range ,
		set_from_hint:css__set_pitch_range_from_hint ,
		initial:css__initial_pitch_range ,
		compose:css__compose_pitch_range ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_pitch ,
		set_from_hint:css__set_pitch_from_hint ,
		initial:css__initial_pitch ,
		compose:css__compose_pitch ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_play_during ,
		set_from_hint:css__set_play_during_from_hint ,
		initial:css__initial_play_during ,
		compose:css__compose_play_during ,
		inherited:0 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_position ,
		set_from_hint:css__set_position_from_hint ,
		initial:css__initial_position ,
		compose:css__compose_position ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_quotes ,
		set_from_hint:css__set_quotes_from_hint ,
		initial:css__initial_quotes ,
		compose:css__compose_quotes ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_richness ,
		set_from_hint:css__set_richness_from_hint ,
		initial:css__initial_richness ,
		compose:css__compose_richness ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_right ,
		set_from_hint:css__set_right_from_hint ,
		initial:css__initial_right ,
		compose:css__compose_right ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_speak_header ,
		set_from_hint:css__set_speak_header_from_hint ,
		initial:css__initial_speak_header ,
		compose:css__compose_speak_header ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_speak_numeral ,
		set_from_hint:css__set_speak_numeral_from_hint ,
		initial:css__initial_speak_numeral ,
		compose:css__compose_speak_numeral ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_speak_punctuation ,
		set_from_hint:css__set_speak_punctuation_from_hint ,
		initial:css__initial_speak_punctuation ,
		compose:css__compose_speak_punctuation ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_speak ,
		set_from_hint:css__set_speak_from_hint ,
		initial:css__initial_speak ,
		compose:css__compose_speak ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_speech_rate ,
		set_from_hint:css__set_speech_rate_from_hint ,
		initial:css__initial_speech_rate ,
		compose:css__compose_speech_rate ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_stress ,
		set_from_hint:css__set_stress_from_hint ,
		initial:css__initial_stress ,
		compose:css__compose_stress ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_table_layout ,
		set_from_hint:css__set_table_layout_from_hint ,
		initial:css__initial_table_layout ,
		compose:css__compose_table_layout ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_text_align ,
		set_from_hint:css__set_text_align_from_hint ,
		initial:css__initial_text_align ,
		compose:css__compose_text_align ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_text_decoration ,
		set_from_hint:css__set_text_decoration_from_hint ,
		initial:css__initial_text_decoration ,
		compose:css__compose_text_decoration ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_text_indent ,
		set_from_hint:css__set_text_indent_from_hint ,
		initial:css__initial_text_indent ,
		compose:css__compose_text_indent ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_text_transform ,
		set_from_hint:css__set_text_transform_from_hint ,
		initial:css__initial_text_transform ,
		compose:css__compose_text_transform ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_top ,
		set_from_hint:css__set_top_from_hint ,
		initial:css__initial_top ,
		compose:css__compose_top ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_unicode_bidi ,
		set_from_hint:css__set_unicode_bidi_from_hint ,
		initial:css__initial_unicode_bidi ,
		compose:css__compose_unicode_bidi ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_vertical_align ,
		set_from_hint:css__set_vertical_align_from_hint ,
		initial:css__initial_vertical_align ,
		compose:css__compose_vertical_align ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_visibility ,
		set_from_hint:css__set_visibility_from_hint ,
		initial:css__initial_visibility ,
		compose:css__compose_visibility ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_voice_family ,
		set_from_hint:css__set_voice_family_from_hint ,
		initial:css__initial_voice_family ,
		compose:css__compose_voice_family ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_volume ,
		set_from_hint:css__set_volume_from_hint ,
		initial:css__initial_volume ,
		compose:css__compose_volume ,
		inherited:1 ,
		group:GROUP_AURAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_white_space ,
		set_from_hint:css__set_white_space_from_hint ,
		initial:css__initial_white_space ,
		compose:css__compose_white_space ,
		inherited:1 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_windows ,
		set_from_hint:css__set_windows_from_hint ,
		initial:css__initial_windows ,
		compose:css__compose_windows ,
		inherited:1 ,
		group:GROUP_PAGE ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_width ,
		set_from_hint:css__set_width_from_hint ,
		initial:css__initial_width ,
		compose:css__compose_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_word_spacing ,
		set_from_hint:css__set_word_spacing_from_hint ,
		initial:css__initial_word_spacing ,
		compose:css__compose_word_spacing ,
		inherited:1 ,
		group:GROUP_UNCOMMON ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_z_index ,
		set_from_hint:css__set_z_index_from_hint ,
		initial:css__initial_z_index ,
		compose:css__compose_z_index ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_opacity ,
		set_from_hint:css__set_opacity_from_hint ,
		initial:css__initial_opacity ,
		compose:css__compose_opacity ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_break_after ,
		set_from_hint:css__set_break_after_from_hint ,
		initial:css__initial_break_after ,
		compose:css__compose_break_after ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_break_before ,
		set_from_hint:css__set_break_before_from_hint ,
		initial:css__initial_break_before ,
		compose:css__compose_break_before ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_break_inside ,
		set_from_hint:css__set_break_inside_from_hint ,
		initial:css__initial_break_inside ,
		compose:css__compose_break_inside ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_count ,
		set_from_hint:css__set_column_count_from_hint ,
		initial:css__initial_column_count ,
		compose:css__compose_column_count ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_fill ,
		set_from_hint:css__set_column_fill_from_hint ,
		initial:css__initial_column_fill ,
		compose:css__compose_column_fill ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_gap ,
		set_from_hint:css__set_column_gap_from_hint ,
		initial:css__initial_column_gap ,
		compose:css__compose_column_gap ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_rule_color ,
		set_from_hint:css__set_column_rule_color_from_hint ,
		initial:css__initial_column_rule_color ,
		compose:css__compose_column_rule_color ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_rule_style ,
		set_from_hint:css__set_column_rule_style_from_hint ,
		initial:css__initial_column_rule_style ,
		compose:css__compose_column_rule_style ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_rule_width ,
		set_from_hint:css__set_column_rule_width_from_hint ,
		initial:css__initial_column_rule_width ,
		compose:css__compose_column_rule_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_span ,
		set_from_hint:css__set_column_span_from_hint ,
		initial:css__initial_column_span ,
		compose:css__compose_column_span ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} ,
	   
	&prop_table { 
		cascade:css__cascade_column_width ,
		set_from_hint:css__set_column_width_from_hint ,
		initial:css__initial_column_width ,
		compose:css__compose_column_width ,
		inherited:0 ,
		group:GROUP_NORMAL ,
	} 

] ;