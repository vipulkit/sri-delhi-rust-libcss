
use include::properties::*;
use include::fpmath::*;

use utils::errors::*;

use stylesheet::*;
use select::common::*;
use select::computed::*;
use select::propset::*;
use select::properties::properties::*;
use include::types::*;

pub enum prop_group {
	GROUP_NORMAL	= 0x0,
	GROUP_UNCOMMON	= 0x1,
	GROUP_PAGE		= 0x2,
	GROUP_AURAL		= 0x3
}

pub struct prop_table {
	cascade : &'static fn (opv:u32, style:@mut css_style,
								state:@mut css_select_state)-> css_error ,
	set_from_hint :  &'static fn (hint:@mut css_hint,
								style: @mut css_computed_style) -> css_error ,
	initial :  &'static fn (state:@mut css_select_state) -> css_error ,
	compose :  &'static fn (parent:@mut css_computed_style,
								child:@mut css_computed_style,
								result:@mut css_computed_style) -> css_error ,

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

pub struct dispatch_table ;
	//table : ~[prop_table]

// dispatch table impl , to access dispatch table static functions from other modules
// otherwise static variables are not visible outside
impl dispatch_table {

	pub fn check_index(index:uint) {
		if( index >= CSS_N_PROPERTIES as uint) {
			fail!(~"In Check Index : index should be less that CSS_N_PROPERTIES value") ;
		}
	}

	pub fn get_cascade_ptr(index:uint) -> (&fn (opv:u32, style:@mut css_style,state:@mut css_select_state)-> css_error) {

		dispatch_table::check_index(index);
		let mut dispatch_cascade = prop_dispatch[index].cascade;
		dispatch_cascade
	}

	pub fn get_set_from_hint_ptr(index:uint) -> (&fn (hint:@mut css_hint, style: @mut css_computed_style) -> css_error) {

		dispatch_table::check_index(index);
		let mut dispatch_set_from_hint = prop_dispatch[index].set_from_hint;
		dispatch_set_from_hint
	}

	pub fn get_initial_ptr(index:uint) -> (&fn (state:@mut css_select_state) -> css_error ) {

		dispatch_table::check_index(index);
		let mut dispatch_initial = prop_dispatch[index].initial;
		dispatch_initial
	}

	pub fn get_compose_ptr(index:uint) -> (&fn (parent:@mut css_computed_style, child:@mut css_computed_style, 
											result:@mut css_computed_style) -> css_error) {

		dispatch_table::check_index(index);
		let mut dispatch_compose = prop_dispatch[index].compose;
		dispatch_compose
	}

	pub fn get_inherited(index:uint) -> uint {

		dispatch_table::check_index(index);
		let mut dispatch_inherited = prop_dispatch[index].inherited;
		dispatch_inherited
	}

	pub fn get_group(index:uint) -> prop_group {

		dispatch_table::check_index(index);
		let mut dispatch_group = prop_dispatch[index].group;
		dispatch_group
	}
}

///////////////////////////////////////////////////////////////////////
// Function pointers used in the "functions containing absolute in thier name"
// as written below in this file

// function pointer : used in "css__compute_absolute_values" function 
///////////////////////////////////////////////////////////////////////
pub type css_fnptr_compute_font_size =  @extern fn(parent:Option<@mut css_hint>,
                                                size:Option<@mut css_hint> ) 
                                                    -> css_error ;

pub type  compute_absolute_length_pair_get =  
    ~extern fn(style:@mut css_computed_style) -> (rect_result);

pub type  compute_absolute_length_pair_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            hlength:i32, 
            hunit:css_unit,
            vlength:i32,
            vunit:css_unit) ;

pub type  compute_absolute_length_normal_get =  
    ~extern fn(style:@mut css_computed_style) -> (u8,Option<i32>,Option<css_unit>);

pub type  compute_absolute_length_normal_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            length:i32, 
            unit:css_unit) ;

pub type  compute_absolute_length_none_get =  
    ~extern fn(style:@mut css_computed_style) -> (u8,Option<i32>,Option<css_unit>);

pub type  compute_absolute_length_none_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            length:i32, 
            unit:css_unit) ;

pub type  compute_absolute_length_auto_get =  
    ~extern fn(style:@mut css_computed_style) -> (u8,Option<i32>,Option<css_unit>);

pub type  compute_absolute_length_auto_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            length:i32, 
            unit:css_unit) ;

pub type  compute_absolute_length_get =  
    ~extern fn(style:@mut css_computed_style) -> (u8,Option<i32>,Option<css_unit>);

pub type  compute_absolute_length_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            length:i32, 
            unit:css_unit) ;

pub type  compute_absolute_border_side_width_get =  
    ~extern fn(style:@mut css_computed_style) -> (u8,Option<i32>,Option<css_unit>);

pub type  compute_absolute_border_side_width_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            length:i32, 
            unit:css_unit) ;

pub type  compute_absolute_color_get =  
    ~extern fn(style:@mut css_computed_style) -> (u8,Option<u32>);

pub type  compute_absolute_color_set =
    ~extern fn(style:@mut css_computed_style,
            ftype:u8, 
            color:u32) ;


//////////////////////////////////////////////////////////////////////

// Functionality containing creation / destruction / compose 
// of the computed functionality of the select/libcss 

//////////////////////////////////////////////////////////////////////

/**
* #Description:
* 	Create a computed style.
* #Return Value:
* 	'css_computed_style' - Pointer to box containing css_computed_style object.
*/
pub fn css_computed_style_create() -> @mut css_computed_style {
	let mut result = @mut css_computed_style {
	    bits:~[],
	    unused:~[],

	    background_color:0,

	    background_image:~"",

	    background_position:~[],

	    border_color:~[],
	    border_width:~[],

	    top:0,
	    right:0,
	    bottom:0,
	    left:0,

	    color:0,

	    font_size:0,

	    height:0,

	    line_height:0,

	    list_style_image:~"",

	    margin:~[],

	    max_height:0,
	    max_width:0,

	    min_height:0,
	    min_width:0,

	    opacity:0,

	    padding:~[],

	    text_indent:0,

	    vertical_align:0,

	    width:0,

	    z_index:0,

	    font_family:~[],

	    quotes:~[],

	    uncommon:None, 
	    aural:None,         
	    page:None 
	};
	for uint::range(0,34) |_| {
		result.bits.push(0) ;
	}
	for uint::range(0,2) |_| {
		result.unused.push(0) ;
	}
	for uint::range(0,2) |_| {
		result.background_position.push(0) ;
	}
	for uint::range(0,4) |_| {
		result.border_color.push(0) ;
	}
	for uint::range(0,4) |_| {
		result.border_width.push(0) ;
	}
	for uint::range(0,4) |_| {
		result.margin.push(0) ;
	}	
	for uint::range(0,4) |_| {
		result.padding.push(0) ;
	}

	result
}

/**
* #Description:
*  Initialize a computed style.
* #Arguments:
*  'style'  - Computed style to populate. 
*  'fn_handler' - pointer to box containing handler function.
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn css_computed_style_initialise(style: @mut css_computed_style ,
                                    fn_handler:@mut css_select_handler) -> css_error {

    let mut state: @mut css_select_state = @mut css_select_state {
        node:ptr::null(),
        media:(CSS_MEDIA_ALL as u64),       
        results:css_select_results{ 
        	styles:~[] 
        },    
        current_pseudo:CSS_PSEUDO_ELEMENT_NONE,  
        computed:style,   
        handler:Some(fn_handler),    
        pw:ptr::null(),
        sheet:None,   
        current_origin:CSS_ORIGIN_UA,  
        current_specificity:0,   
        element:css_qname{ 
            name:~"" , 
            ns:~"" 
        },
        id:~"",
        classes:~[],
        n_classes:0,             
        reject_cache: ~[],       
        next_reject:128-1,             
        props: ~[~[]] 
    };
    for uint::range(0,CSS_N_PROPERTIES as uint) |outer| {
        let mut prop_vec : ~[@mut prop_state] = ~[] ;
        for uint::range(0,CSS_PSEUDO_ELEMENT_COUNT as uint) |inner| {
            let mut pstate = @mut prop_state{
                specificity:0,
                set:false,
                origin:0,
                important:false,
                inherit:false    
            };
            prop_vec.push(pstate);
        }
        state.props.push(prop_vec);
    }

    let mut i: uint = 0 ;
    let mut error: css_error;

    if( prop_dispatch.len() < (CSS_N_PROPERTIES as uint) ) {
        return CSS_BADPARM ;
    }

    while i < (CSS_N_PROPERTIES as uint) {

        /* No need to initialise anything other than the normal
         * properties -- the others are handled by the accessors */
        match prop_dispatch[i].group {
            GROUP_NORMAL => {
                if ( prop_dispatch[i].inherited == 0 ) {
                    let mut dispatch_initial = prop_dispatch[i].initial;
                    error =  dispatch_initial(state);
                    match error {
                        CSS_OK=>{},
                        x =>  {
                            return x ;
                        }
                    }
                }
            }
            _ => { }
        }
        i += 1;
    }
    CSS_OK
}

/**
* #Description:
*  Compose two computed styles.
* #Arguments:
*  'parent'  - Parent style. 
*  'child' - Child style.
*  'compute_font_size' - Function to compute an absolute font size.
*  'result' - Pointer to style to compose into.
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn css_computed_style_compose(parent: @mut css_computed_style, 
                                child: @mut css_computed_style, 
                                compute_font_size_ptr: css_fnptr_compute_font_size , 
                                result: @mut css_computed_style
                                ) -> css_error {

    let mut error: css_error;
    let mut i : uint = 0;
    while i < (CSS_N_PROPERTIES as uint) {
        
        /* Skip any in extension blocks if the block does not exist */
        match prop_dispatch[i].group {
            GROUP_UNCOMMON => {
            	if ( parent.uncommon.is_none() &&
            		 child.uncommon.is_none()  ) {
					loop ;
				}
            },
            GROUP_PAGE => {
            	if ( parent.page.is_none() &&
            		 child.page.is_none()  ) {
					loop ;
				}
            },
            GROUP_AURAL => {
            	if ( parent.aural.is_none() &&
            		 child.aural.is_none()  ) {
					loop ;
				}
            },
            GROUP_NORMAL => {}
        }

		/* Compose the property */
		let mut dispatch_compose = prop_dispatch[i].compose;
		error =  dispatch_compose(parent, child, result);
        match error {
            CSS_OK=>{},
            _ =>  {
                break ;
            }
        }

        i += 1;
    }

    css__compute_absolute_values(Some(parent),result,compute_font_size_ptr)
}

//////////////////////////////////////////////////////////////////////

// Common functionality of propget and computed module ended here , 
// start of the absolute functions in the computed source of the 
// select component of the libcss

//////////////////////////////////////////////////////////////////////
/**
* #Description:
*  Compose two computed styles.
* #Arguments:
*  'parent'  - Parent style, or None for tree root. 
*  'style' - Computed style to process.
*  'compute_font_size' - Function to compute an absolute font size.
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn css__compute_absolute_values(parent: Option<@mut css_computed_style>,
                                    style: @mut css_computed_style,
                                    compute_font_size_ptr:css_fnptr_compute_font_size) 
                                    -> css_error {

    let mut psize = @mut css_hint{
        hint_type:HINT_LENGTH,
        status:0,
        clip:None,
        content:None,
        counters:None,
        length:None,
        position:None,
        color:None,
        fixed:None,
        integer:None,
        string:None,
        strings:None
    };
    let mut size = @mut css_hint{
        hint_type:HINT_LENGTH,
        status:0,
        clip:None,
        content:None,
        counters:None,
        length:None,
        position:None,
        color:None,
        fixed:None,
        integer:None,
        string:None,
        strings:None
    };
    let mut ex_size = @mut css_hint{
        hint_type:HINT_LENGTH,
        status:0,
        clip:None,
        content:None,
        counters:None,
        length:None,
        position:None,
        color:None,
        fixed:None,
        integer:None,
        string:None,
        strings:None
    };
    let mut error : css_error ;

    match parent {
        Some(parent_style)=>{
            let (a,b,c) = css_computed_font_size(parent_style);
            psize.status = a;
            let length = @mut css_hint_length { 
                value:b.get_or_default(0) , 
                unit:c.get_or_default(CSS_UNIT_PX) 
            };
            psize.length = Some(length);
            error = (*compute_font_size_ptr)(Some(psize),Some(size));
        },
        None=>{
            let (a,b,c) = css_computed_font_size(style);
            psize.status = a;
            let length = @mut css_hint_length { 
                value:b.get_or_default(0) , 
                unit:c.get_or_default(CSS_UNIT_PX) 
            };
            psize.length = Some(length);
            error = (*compute_font_size_ptr)(None,Some(size));
        }
    }
    match error {
        CSS_OK=>{},
        _=> return error
    }

    match size.hint_type {
        HINT_LENGTH=>{
            match size.length {
                None=>{
                    set_font_size(style,size.status,0,CSS_UNIT_PX);
                }
                Some(length)=>{
                    set_font_size(style,size.status,length.value,length.unit);
                }
            }
        },
        _=> return CSS_BADPARM
    }

    ex_size.status = CSS_FONT_SIZE_DIMENSION as u8;
    let length = @mut css_hint_length { 
        value:css_int_to_fixed(1) , 
        unit:CSS_UNIT_EX 
    };
    ex_size.length = Some(length);
    error = (*compute_font_size_ptr)(Some(size),Some(ex_size));
    match error {
        CSS_OK=>{},
        _=> return error
    }

    match size.length {
        None=>{
            ex_size.length.get().value = 0 ;
        },
        Some(length)=>{
            ex_size.length.get().value = css_divide_fixed(ex_size.length.get().value,length.value);
        }
    }

    // All functions called below uses the ex_size.length variable , and this 
    // variable is option in our case , so for all and once check , if it is none
    // in case it is none , all operations below go invalid.

    if ex_size.length.is_none() {
        return CSS_BADPARM ;
    }

    ex_size.length.get().unit = CSS_UNIT_EM ;
    /* Fix up background-position */
    error = compute_absolute_length_pair(style, 
            ex_size.length.get(), 
            ~css_computed_background_position,
            ~set_background_position);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up background-color */
    error = compute_absolute_color(style,
            ~css_computed_background_color,
            ~set_background_color);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up border-{top,right,bottom,left}-color */
    error = compute_border_colors(style);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up border-{top,right,bottom,left}-width */
    error = compute_absolute_border_width(style, ex_size.length.get());
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up sides */
    error = compute_absolute_sides(style, ex_size.length.get());
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up height */
    error = compute_absolute_length_auto(style, ex_size.length.get(), 
            ~css_computed_height,
            ~set_height);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up line-height (must be before vertical-align) */
    error = compute_absolute_line_height(style, ex_size.length.get());
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up margins */
    error = compute_absolute_margins(style, ex_size.length.get());
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up max-height */
    error = compute_absolute_length_none(style, 
            ex_size.length.get(), 
            ~css_computed_max_height, 
            ~set_max_height);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up max-width */
    error = compute_absolute_length_none(style, 
            ex_size.length.get(), 
            ~css_computed_max_width, 
            ~set_max_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up min-height */
    error = compute_absolute_length(style, 
            ex_size.length.get(), 
            ~css_computed_min_height, 
            ~set_min_height);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up min-width */
    error = compute_absolute_length(style, 
            ex_size.length.get(), 
            ~css_computed_min_width, 
            ~set_min_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up padding */
    error = compute_absolute_padding(style, ex_size.length.get());
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up text-indent */
    error = compute_absolute_length(style, 
            ex_size.length.get(), 
            ~css_computed_text_indent, 
            ~set_text_indent);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up vertical-align */
    error = compute_absolute_vertical_align(style, ex_size.length.get());
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Fix up width */
    error = compute_absolute_length_auto(style, 
            ex_size.length.get(), 
            ~css_computed_width, 
            ~set_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    /* Uncommon properties */
    match style.uncommon {
        Some(_)=> {
            /* Fix up border-spacing */
            error = compute_absolute_length_pair(style,
                    ex_size.length.get(),
                    ~css_computed_border_spacing,
                    ~set_border_spacing);
            match error {
                CSS_OK=>{},
                _=> return error
            }

            /* Fix up clip */
            error = compute_absolute_clip(style, ex_size.length.get());
            match error {
                CSS_OK=>{},
                _=> return error
            }

            /* Fix up letter-spacing */
            error = compute_absolute_length_normal(style,
                    ex_size.length.get(),
                    ~css_computed_letter_spacing, 
                    ~set_letter_spacing);
            match error {
                CSS_OK=>{},
                _=> return error
            }

            /* Fix up outline-color */
            error = compute_absolute_color(style,
                    ~css_computed_outline_color,
                    ~set_outline_color);
            match error {
                CSS_OK=>{},
                _=> return error
            }

            /* Fix up outline-width */
            error = compute_absolute_border_side_width(style, 
                    ex_size.length.get(), 
                    ~css_computed_outline_width, 
                    ~set_outline_width);
            match error {
                CSS_OK=>{},
                _=> return error
            }

            /* Fix up word spacing */
            error = compute_absolute_length_normal(style,
                    ex_size.length.get(),
                    ~css_computed_word_spacing, 
                    ~set_word_spacing);
            match error {
                CSS_OK=>{},
                _=> return error
            }
        }
        None=>{}
    }
    CSS_OK
}
///////////////////////////////////////////////////////////////////////

/**
* #Description:
*  Compute colour values, replacing any set to currentColor with the computed value of colour.
* #Arguments:
*  'style'  - The style to process. 
*  'getfn' - Accessors for colour value.
*  'setfn' - Mutator for colour value.
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn  compute_absolute_color(style: @mut css_computed_style,
                                getfn : compute_absolute_color_get,
                                setfn : compute_absolute_color_set
                                ) -> css_error {

    let mut (result,_) =  (*getfn)(style);

    if ( result == (CSS_BACKGROUND_COLOR_CURRENT_COLOR as u8) ) {

        let mut (_,ocomputed_color) = css_computed_color(style);
        let mut computed_color = ocomputed_color.get_or_default(0);

        (*setfn)(style, (CSS_BACKGROUND_COLOR_COLOR as u8), computed_color);
    }
    CSS_OK
}

/**
* #Description:
*  Compute border colours, replacing any set to currentColor with the computed value of colour.
* #Arguments:
*  'style'  - The style to process. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_border_colors(style: @mut css_computed_style) -> css_error {

    let mut (_,ocomputed_color) = css_computed_color(style);
    let mut computed_color = ocomputed_color.get_or_default(0);

    //let mut (result,border_color) : (u8,u32) = (0,0);
    let mut (result,_) = css_computed_border_top_color(style) ;
    if ( result == (CSS_BORDER_COLOR_CURRENT_COLOR as u8) ) {
        set_border_top_color(style, 
                (CSS_BORDER_COLOR_COLOR as u8) ,computed_color );
    }

    let mut (result,_) = css_computed_border_right_color(style) ;
    if ( result == (CSS_BORDER_COLOR_CURRENT_COLOR as u8) ) {
        set_border_right_color(style, 
                (CSS_BORDER_COLOR_COLOR as u8) ,computed_color );
    }

    let mut (result,_) = css_computed_border_bottom_color(style);
    if ( result  == (CSS_BORDER_COLOR_CURRENT_COLOR as u8) ) {
        set_border_bottom_color(style, 
                (CSS_BORDER_COLOR_COLOR as u8) ,computed_color );
    }

    let mut (result,_) = css_computed_border_left_color(style) ;
    if ( result == (CSS_BORDER_COLOR_CURRENT_COLOR as u8) ) {
        set_border_left_color(style, 
                (CSS_BORDER_COLOR_COLOR as u8) ,computed_color );
    }

    CSS_OK
}

/**
* #Description:
*  Compute absolute border widths.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn  compute_absolute_border_width(style: @mut css_computed_style,
                    ex_size: @mut css_hint_length) -> css_error {

    let mut error : css_error ;

    error = compute_absolute_border_side_width(style, ex_size,
            ~css_computed_border_top_width, 
            ~set_border_top_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_border_side_width(style, ex_size,
            ~css_computed_border_right_width, 
            ~set_border_right_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_border_side_width(style, ex_size,
            ~css_computed_border_bottom_width, 
            ~set_border_bottom_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_border_side_width(style, ex_size,
            ~css_computed_border_left_width, 
            ~set_border_left_width);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    CSS_OK
}

/**
* #Description:
*  Compute absolute border side widths.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
*  'getfn'  - Function to read length. 
*  'setfn'  - Function to write length. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn  compute_absolute_border_side_width(style: @mut css_computed_style,
                                    ex_size: @mut css_hint_length,
                                    getfn : compute_absolute_border_side_width_get,
                                    setfn : compute_absolute_border_side_width_set
                                    ) -> css_error {

    let mut (result,olength,ounit) =  (*getfn)(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    if (result == (CSS_BORDER_WIDTH_THIN as u8) ) {
        length = css_int_to_fixed(1);
        unit = CSS_UNIT_PX;
    } else if (result == (CSS_BORDER_WIDTH_MEDIUM as u8) ) {
        length = css_int_to_fixed(2);
        unit = CSS_UNIT_PX;
    } else if (result == (CSS_BORDER_WIDTH_THICK as u8) ) {
        length = css_int_to_fixed(4);
        unit = CSS_UNIT_PX;
    }

    match unit {
        CSS_UNIT_EX=> {
            length = css_multiply_fixed(length, ex_size.value);
            unit = ex_size.unit;
        }
        _=>{}
    }

    (*setfn)(style, (CSS_BORDER_WIDTH_WIDTH as u8), length, unit);
    CSS_OK
}

/**
* #Description:
*  Compute absolute border side widths.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_clip(style: @mut css_computed_style,
                    ex_size: @mut css_hint_length) -> css_error {

    let mut (result, orect) = css_computed_clip(style) ;

    match orect {
        None=> { 
            return CSS_BADPARM ;
        },
        Some(x)=> {
            let mut rect = x ;

            if ( result == (CSS_CLIP_RECT as u8) ) {
                if (rect.top_auto == false) {
                    match rect.tunit {
                        CSS_UNIT_EX=> {
                            rect.top = css_multiply_fixed(rect.top, ex_size.value);
                            rect.tunit = ex_size.unit;
                        }
                        _=>{}
                    }
                }

                if (rect.right_auto == false) {
                    match rect.runit {
                        CSS_UNIT_EX=> {
                            rect.right = css_multiply_fixed(rect.right, ex_size.value);
                            rect.runit = ex_size.unit;
                        }
                        _=>{}
                    }
                }

                if (rect.bottom_auto == false) {
                    match rect.bunit {
                        CSS_UNIT_EX=> {
                            rect.bottom = css_multiply_fixed(rect.bottom, ex_size.value);
                            rect.bunit = ex_size.unit;
                        }
                        _=>{}
                    }
                }

                if (rect.left_auto == false) {
                    match rect.lunit {
                        CSS_UNIT_EX=> {
                            rect.left = css_multiply_fixed(rect.left, ex_size.value);
                            rect.lunit = ex_size.unit;
                        }
                        _=>{}
                    }
                }

                set_clip(style, (CSS_CLIP_RECT as u8), rect);
            }
        }
    }

    CSS_OK
}

/**
* #Description:
*  Compute absolute line-height.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_line_height(style: @mut css_computed_style,
                    ex_size: @mut css_hint_length) -> css_error {

    let mut (result,olength,ounit) = css_computed_line_height(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    if ( result == (CSS_LINE_HEIGHT_DIMENSION as u8) ) {
        match unit {
            CSS_UNIT_EX=> {
            length = css_multiply_fixed(length, ex_size.value);
            unit = ex_size.unit;
            }
            _=>{}
        }

        set_line_height(style, result, length, unit);
    }

    CSS_OK
}

/**
* #Description:
*  Compute the absolute values of {top,right,bottom,left}.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_sides(style: @mut css_computed_style,
                    ex_size: @mut css_hint_length) -> css_error {

    let mut error : css_error ;

    /* Calculate absolute lengths for sides */
    error = compute_absolute_length_auto(style, ex_size, 
            ~css_computed_top, ~set_top);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_right, ~set_right);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_bottom, ~set_bottom);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_left, ~set_left);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    CSS_OK
}

/**
* #Description:
*  Compute absolute margins.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_margins(style: @mut css_computed_style,
                    ex_size: @mut css_hint_length) -> css_error {

    let mut error : css_error ;

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_margin_top, ~set_margin_top);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_margin_right, ~set_margin_right);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_margin_bottom, ~set_margin_bottom);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length_auto(style, ex_size,
            ~css_computed_margin_left, ~set_margin_left);
    match error {
        CSS_OK=>{},
        _=> return error
    }
    
    CSS_OK
}

/**
* #Description:
*  Compute absolute padding.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_padding(style: @mut css_computed_style,
                            ex_size: @mut css_hint_length) -> css_error {

    let mut error : css_error ;

    error = compute_absolute_length(style, ex_size,
            ~css_computed_padding_top, ~set_padding_top);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length(style, ex_size,
            ~css_computed_padding_right, ~set_padding_right);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length(style, ex_size,
            ~css_computed_padding_bottom, ~set_padding_bottom);
    match error {
        CSS_OK=>{},
        _=> return error
    }

    error = compute_absolute_length(style, ex_size,
            ~css_computed_padding_left, ~set_padding_left);
    match error {
        CSS_OK=>{},
        _=> return error
    }
    
    CSS_OK
}

/**
* #Description:
*  Compute absolute vertical-align.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_vertical_align(style: @mut css_computed_style,
                            ex_size: @mut css_hint_length) -> css_error {

    let mut (result,olength,ounit) = css_computed_vertical_align(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    if (result == (CSS_VERTICAL_ALIGN_SET as u8) ) {
        match unit {
            CSS_UNIT_EX=> {
                length = css_multiply_fixed(length, ex_size.value);
                unit = ex_size.unit;
            },
            _=>{}
        }

        set_vertical_align(style, result, length, unit);
    }
    CSS_OK
}

/**
* #Description:
*  Compute the absolute value of length.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
*  'getfn'  - Function to read length. 
*  'setfn'  - Function to write length. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_length(style: @mut css_computed_style,
                                    ex_size: @mut css_hint_length,
                                    getfn : compute_absolute_length_get,
                                    setfn : compute_absolute_length_set
                                    ) -> css_error {

    let mut (result,olength,ounit) =  (*getfn)(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    match unit {
        CSS_UNIT_EX=>{
            length = css_multiply_fixed(length, ex_size.value);
            unit = ex_size.unit;
        },
        _=>{}
    }

    (*setfn)(style, result, length, unit);
    CSS_OK
}

/**
* #Description:
*  Compute the absolute value of length or auto.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
*  'getfn'  - Function to read length. 
*  'setfn'  - Function to write length. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_length_auto(style: @mut css_computed_style,
                                    ex_size: @mut css_hint_length,
                                    getfn : compute_absolute_length_auto_get,
                                    setfn : compute_absolute_length_auto_set
                                    ) -> css_error {

    let mut (result,olength,ounit) =  (*getfn)(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    if (result != (CSS_BOTTOM_AUTO as u8) ) {
        match unit {
            CSS_UNIT_EX=> {
                length = css_multiply_fixed(length, ex_size.value);
                unit = ex_size.unit;
            },
            _=>{}
        }

        (*setfn)(style, (CSS_BOTTOM_SET as u8), length, unit);
        CSS_OK
    }
    else {
        (*setfn)(style, (CSS_BOTTOM_AUTO as u8), 0, CSS_UNIT_PX);
        CSS_OK
    }
}

/**
* #Description:
*  Compute the absolute value of length or none.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
*  'getfn'  - Function to read length. 
*  'setfn'  - Function to write length. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_length_none(style: @mut css_computed_style,
                                    ex_size: @mut css_hint_length,
                                    getfn : compute_absolute_length_none_get,
                                    setfn : compute_absolute_length_none_set
                                    ) -> css_error {

    let mut (result,olength,ounit) =  (*getfn)(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    if (result != (CSS_MAX_HEIGHT_NONE as u8) ) {
        match unit {
            CSS_UNIT_EX=> {
                length = css_multiply_fixed(length, ex_size.value);
                unit = ex_size.unit;
            },
            _=>{}
        }

        (*setfn)(style, (CSS_MAX_HEIGHT_SET as u8), length, unit);
        CSS_OK
    }
    else {
        (*setfn)(style, (CSS_MAX_HEIGHT_NONE as u8), 0, CSS_UNIT_PX);
        CSS_OK
    }
}

/**
* #Description:
*  Compute the absolute value of length or normal.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
*  'getfn'  - Function to read length. 
*  'setfn'  - Function to write length. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn  compute_absolute_length_normal(style: @mut css_computed_style,
                                    ex_size: @mut css_hint_length,
                                    getfn : compute_absolute_length_normal_get,
                                    setfn : compute_absolute_length_normal_set
                                    ) -> css_error {

    let mut (result,olength,ounit) =  (*getfn)(style);
    let mut length = olength.get_or_default(0);
    let mut unit = ounit.get_or_default(CSS_UNIT_PX);

    if (result != (CSS_LETTER_SPACING_NORMAL as u8) ) {
        match unit {
            CSS_UNIT_EX=> {
                length = css_multiply_fixed(length, ex_size.value);
                unit = ex_size.unit;
            },
            _=>{}
        }

        (*setfn)(style, (CSS_LETTER_SPACING_SET as u8), length, unit);
        CSS_OK 
    }
    else {
        (*setfn)(style, (CSS_LETTER_SPACING_NORMAL as u8), 0, CSS_UNIT_PX);
        CSS_OK
    }
}

/**
* #Description:
*  Compute the absolute value of length pair.
* #Arguments:
*  'style'  - The style to process. 
*  'ex_size'  - Ex size, in ems. 
*  'getfn'  - Function to read length. 
*  'setfn'  - Function to write length. 
* #Return Value:
*  'css_error' - CSS_OK on success, appropriate error otherwise.
*/
pub fn compute_absolute_length_pair(style: @mut css_computed_style,
                                    ex_size: @mut css_hint_length,
                                    getfn : compute_absolute_length_pair_get,
                                    setfn : compute_absolute_length_pair_set
                                    ) -> css_error {

    let mut result = (*getfn)(style) ;

    match result.hunit {
        CSS_UNIT_EX=>{
            result.hlength = css_multiply_fixed(result.hlength, ex_size.value);
            result.hunit = ex_size.unit;
        },
        _=>{}
    }

    match result.hunit {
        CSS_UNIT_EX=>{
            result.vlength = css_multiply_fixed(result.vlength, ex_size.value);
            result.vunit = ex_size.unit;
        },
        _=>{}
    }

    (*setfn)(style, result.result , result.hlength, result.hunit, result.vlength, result.vunit);
    CSS_OK
}

//////////////////////////////////////////////////////////////////////