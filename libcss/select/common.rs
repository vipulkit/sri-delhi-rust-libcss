use include::types::*;
use include::font_face::*;
use stylesheet::*;
use utils::errors::*;
//use extra::arc;
use wapcaplet::*;
use std::libc::*;
use std::clone::Clone;

pub enum css_computed_content_item_type {
    CSS_COMPUTED_CONTENT_NONE       = 0,
    CSS_COMPUTED_CONTENT_STRING     = 1,
    CSS_COMPUTED_CONTENT_URI        = 2,
    CSS_COMPUTED_CONTENT_COUNTER    = 3,
    CSS_COMPUTED_CONTENT_COUNTERS   = 4,
    CSS_COMPUTED_CONTENT_ATTR       = 5,
    CSS_COMPUTED_CONTENT_OPEN_QUOTE     = 6,
    CSS_COMPUTED_CONTENT_CLOSE_QUOTE    = 7,
    CSS_COMPUTED_CONTENT_NO_OPEN_QUOTE  = 8,
    CSS_COMPUTED_CONTENT_NO_CLOSE_QUOTE = 9
}

pub enum css_hint_data_type{
    CLIP_RECT,
    COLOR,
    CONTENT_ITEM,
    COUNTER,
    FIXED,
    INTEGER_TYPE,
    HINT_LENGTH,
    HINT_LENGTH_H_V,
    STRING,
    STRINGS_VECTOR
}

pub static CSS_LETTER_SPACING_INDEX: int =  0;
pub static CSS_LETTER_SPACING_SHIFT: int =  2;
pub static CSS_LETTER_SPACING_MASK : int =   0xfc;
pub static CSS_OUTLINE_COLOR_INDEX: int =  0;
pub static CSS_OUTLINE_COLOR_SHIFT: int =  0;
pub static CSS_OUTLINE_COLOR_MASK : int =   0x3;
pub static CSS_OUTLINE_WIDTH_INDEX: int =  1;
pub static CSS_OUTLINE_WIDTH_SHIFT: int =  1;
pub static CSS_OUTLINE_WIDTH_MASK : int =   0xfe;
pub static CSS_BORDER_SPACING_INDEX: int =  1;
pub static CSS_BORDER_SPACING_SHIFT : int = 0;
pub static CSS_BORDER_SPACING_MASK : int =  0x1;
pub static CSS_BORDER_SPACING_INDEX1: int =  2;
pub static CSS_BORDER_SPACING_SHIFT1: int =  0;
pub static CSS_BORDER_SPACING_MASK1: u8 =  0xff;
pub static CSS_WORD_SPACING_INDEX : int = 3;
pub static CSS_WORD_SPACING_SHIFT: int =  2;
pub static CSS_WORD_SPACING_MASK : int =  0xfc;
pub static CSS_COUNTER_INCREMENT_INDEX: int =  3;
pub static CSS_COUNTER_INCREMENT_SHIFT: int =  1;
pub static CSS_COUNTER_INCREMENT_MASK : int =   0x2;
pub static CSS_COUNTER_RESET_INDEX: int =  3;
pub static CSS_COUNTER_RESET_SHIFT: int =  0;
pub static CSS_COUNTER_RESET_MASK : int =  0x1;
pub static CSS_CURSOR_INDEX : int = 4;
pub static CSS_CURSOR_SHIFT: int =  3;
pub static CSS_CURSOR_MASK : int =  0xf8;
pub static CSS_CLIP_INDEX: int =  7;
pub static CSS_CLIP_SHIFT : int = 2;
pub static CSS_CLIP_MASK : int =  0xfc;
pub static CSS_CLIP_INDEX1: int =  5;
pub static CSS_CLIP_SHIFT1 : int = 0;
pub static CSS_CLIP_MASK1: u8 =  0xff;
pub static CSS_CLIP_INDEX2 : int = 6;
pub static CSS_CLIP_SHIFT2: int =  0;
pub static CSS_CLIP_MASK2: u8 =  0xff;
pub static CSS_CONTENT_INDEX : int = 7;
pub static CSS_CONTENT_SHIFT: int =  0;
pub static CSS_CONTENT_MASK : int =  0x3;
pub static CSS_VERTICAL_ALIGN_INDEX : int = 0;
pub static CSS_VERTICAL_ALIGN_SHIFT: int =  0;
pub static CSS_VERTICAL_ALIGN_MASK : int =  0xff;
pub static CSS_FONT_SIZE_INDEX: int =  1;
pub static CSS_FONT_SIZE_SHIFT : int = 0;
pub static CSS_FONT_SIZE_MASK : int =  0xff;
pub static CSS_BORDER_TOP_WIDTH_INDEX: int =  2;
pub static CSS_BORDER_TOP_WIDTH_SHIFT: int =  1;
pub static CSS_BORDER_TOP_WIDTH_MASK : int =  0xfe;
pub static CSS_BORDER_RIGHT_WIDTH_INDEX: int =  3;
pub static CSS_BORDER_RIGHT_WIDTH_SHIFT : int = 1;
pub static CSS_BORDER_RIGHT_WIDTH_MASK : int =  0xfe;
pub static CSS_BORDER_BOTTOM_WIDTH_INDEX : int = 4;
pub static CSS_BORDER_BOTTOM_WIDTH_SHIFT : int = 1;
pub static CSS_BORDER_BOTTOM_WIDTH_MASK : int = 0xfe;
pub static CSS_BORDER_LEFT_WIDTH_INDEX : int = 5;
pub static CSS_BORDER_LEFT_WIDTH_SHIFT : int = 1;
pub static CSS_BORDER_LEFT_WIDTH_MASK : int =  0xfe;
pub static CSS_BACKGROUND_IMAGE_INDEX : int = 2;
pub static CSS_BACKGROUND_IMAGE_SHIFT : int = 0;
pub static CSS_BACKGROUND_IMAGE_MASK : int =  0x1;
pub static CSS_COLOR_INDEX : int = 3;
pub static CSS_COLOR_SHIFT : int = 0;
pub static CSS_COLOR_MASK : int = 0x1;
pub static CSS_LIST_STYLE_IMAGE_INDEX: int =  4;
pub static CSS_LIST_STYLE_IMAGE_SHIFT: int =  0;
pub static CSS_LIST_STYLE_IMAGE_MASK : int =  0x1;
pub static CSS_QUOTES_INDEX : int = 5;
pub static CSS_QUOTES_SHIFT: int =  0;
pub static CSS_QUOTES_MASK : int = 0x1;
pub static CSS_TOP_INDEX : int =  6;
pub static CSS_TOP_SHIFT  : int = 2;
pub static CSS_TOP_MASK : int =  0xfc;
pub static CSS_RIGHT_INDEX : int =  7;
pub static CSS_RIGHT_SHIFT  : int = 2;
pub static CSS_RIGHT_MASK : int =  0xfc;
pub static CSS_BOTTOM_INDEX : int =  8;
pub static CSS_BOTTOM_SHIFT : int =  2;
pub static CSS_BOTTOM_MASK : int =  0xfc;
pub static CSS_LEFT_INDEX : int =  9;
pub static CSS_LEFT_SHIFT : int =  2;
pub static CSS_LEFT_MASK : int =  0xfc;
pub static CSS_BORDER_TOP_COLOR_INDEX  : int = 6;
pub static CSS_BORDER_TOP_COLOR_SHIFT : int =  0;
pub static CSS_BORDER_TOP_COLOR_MASK : int = 0x3;
pub static CSS_BORDER_RIGHT_COLOR_INDEX  : int = 7;
pub static CSS_BORDER_RIGHT_COLOR_SHIFT : int =  0;
pub static CSS_BORDER_RIGHT_COLOR_MASK : int =  0x3;
pub static CSS_BORDER_BOTTOM_COLOR_INDEX  : int = 8;
pub static CSS_BORDER_BOTTOM_COLOR_SHIFT : int =  0;
pub static CSS_BORDER_BOTTOM_COLOR_MASK : int =  0x3;
pub static CSS_BORDER_LEFT_COLOR_INDEX  : int = 9;
pub static CSS_BORDER_LEFT_COLOR_SHIFT : int =  0;
pub static CSS_BORDER_LEFT_COLOR_MASK : int =  0x3;
pub static CSS_HEIGHT_INDEX  : int = 10;
pub static CSS_HEIGHT_SHIFT  : int = 2;
pub static CSS_HEIGHT_MASK : int =  0xfc;
pub static CSS_LINE_HEIGHT_INDEX  : int = 11;
pub static CSS_LINE_HEIGHT_SHIFT : int =  2;
pub static CSS_LINE_HEIGHT_MASK : int = 0xfc;
pub static CSS_BACKGROUND_COLOR_INDEX : int =  10;
pub static CSS_BACKGROUND_COLOR_SHIFT  : int = 0;
pub static CSS_BACKGROUND_COLOR_MASK : int =  0x3;
pub static CSS_Z_INDEX_INDEX  : int = 11;
pub static CSS_Z_INDEX_SHIFT  : int = 0;
pub static CSS_Z_INDEX_MASK : int = 0x3;
pub static CSS_MARGIN_TOP_INDEX  : int = 12;
pub static CSS_MARGIN_TOP_SHIFT  : int = 2;
pub static CSS_MARGIN_TOP_MASK : int = 0xfc;
pub static CSS_MARGIN_RIGHT_INDEX  : int = 13;
pub static CSS_MARGIN_RIGHT_SHIFT  : int = 2;
pub static CSS_MARGIN_RIGHT_MASK : int = 0xfc;
pub static CSS_MARGIN_BOTTOM_INDEX  : int = 14;
pub static CSS_MARGIN_BOTTOM_SHIFT : int =  2;
pub static CSS_MARGIN_BOTTOM_MASK : int =  0xfc;
pub static CSS_MARGIN_LEFT_INDEX  : int = 15;
pub static CSS_MARGIN_LEFT_SHIFT : int =  2;
pub static CSS_MARGIN_LEFT_MASK : int =  0xfc;
pub static CSS_BACKGROUND_ATTACHMENT_INDEX : int =  12;
pub static CSS_BACKGROUND_ATTACHMENT_SHIFT : int =  0;
pub static CSS_BACKGROUND_ATTACHMENT_MASK : int =   0x3;
pub static CSS_BORDER_COLLAPSE_INDEX : int =  13;
pub static CSS_BORDER_COLLAPSE_SHIFT  : int = 0;
pub static CSS_BORDER_COLLAPSE_MASK : int =  0x3;
pub static CSS_CAPTION_SIDE_INDEX  : int = 14;
pub static CSS_CAPTION_SIDE_SHIFT : int =  0;
pub static CSS_CAPTION_SIDE_MASK : int =  0x3;
pub static CSS_DIRECTION_INDEX : int =  15;
pub static CSS_DIRECTION_SHIFT : int =  0;
pub static CSS_DIRECTION_MASK : int =  0x3;
pub static CSS_MAX_HEIGHT_INDEX  : int = 16;
pub static CSS_MAX_HEIGHT_SHIFT : int =  2;
pub static CSS_MAX_HEIGHT_MASK : int =  0xfc;
pub static CSS_MAX_WIDTH_INDEX  : int = 17;
pub static CSS_MAX_WIDTH_SHIFT  : int = 2;
pub static CSS_MAX_WIDTH_MASK : int =  0xfc;
pub static CSS_WIDTH_INDEX  : int = 18;
pub static CSS_WIDTH_SHIFT : int =  2;
pub static CSS_WIDTH_MASK : int =  0xfc;
pub static CSS_EMPTY_CELLS_INDEX : int =  16;
pub static CSS_EMPTY_CELLS_SHIFT : int =  0;
pub static CSS_EMPTY_CELLS_MASK : int =   0x3;
pub static CSS_FLOAT_INDEX  : int = 17;
pub static CSS_FLOAT_SHIFT : int =  0;
pub static CSS_FLOAT_MASK : int =  0x3;
pub static CSS_FONT_STYLE_INDEX  : int = 18;
pub static CSS_FONT_STYLE_SHIFT  : int = 0;
pub static CSS_FONT_STYLE_MASK : int =  0x3;
pub static CSS_MIN_HEIGHT_INDEX  : int = 19;
pub static CSS_MIN_HEIGHT_SHIFT : int =  3;
pub static CSS_MIN_HEIGHT_MASK : int =  0xf8;
pub static CSS_MIN_WIDTH_INDEX  : int = 20;
pub static CSS_MIN_WIDTH_SHIFT : int =  3;
pub static CSS_MIN_WIDTH_MASK : int =   0xf8;
pub static CSS_BACKGROUND_REPEAT_INDEX : int = 19;
pub static CSS_BACKGROUND_REPEAT_SHIFT  : int = 0;
pub static CSS_BACKGROUND_REPEAT_MASK : int =  0x7;
pub static CSS_CLEAR_INDEX : int =  20;
pub static CSS_CLEAR_SHIFT : int =  0;
pub static CSS_CLEAR_MASK : int =  0x7;
pub static CSS_PADDING_TOP_INDEX : int =  21;
pub static CSS_PADDING_TOP_SHIFT : int =  3;
pub static CSS_PADDING_TOP_MASK : int = 0xf8;
pub static CSS_PADDING_RIGHT_INDEX  : int = 22;
pub static CSS_PADDING_RIGHT_SHIFT : int =  3;
pub static CSS_PADDING_RIGHT_MASK : int =  0xf8;
pub static CSS_PADDING_BOTTOM_INDEX  : int = 23;
pub static CSS_PADDING_BOTTOM_SHIFT : int =  3;
pub static CSS_PADDING_BOTTOM_MASK : int =   0xf8;
pub static CSS_PADDING_LEFT_INDEX  : int = 24;
pub static CSS_PADDING_LEFT_SHIFT  : int = 3;
pub static CSS_PADDING_LEFT_MASK : int =  0xf8;
pub static CSS_OVERFLOW_INDEX  : int = 21;
pub static CSS_OVERFLOW_SHIFT  : int = 0;
pub static CSS_OVERFLOW_MASK : int =  0x7;
pub static CSS_POSITION_INDEX  : int = 22;
pub static CSS_POSITION_SHIFT : int =  0;
pub static CSS_POSITION_MASK : int =  0x7;
pub static CSS_OPACITY_INDEX  : int = 23;
pub static CSS_OPACITY_SHIFT : int =  2;
pub static CSS_OPACITY_MASK : int = 0x04;
pub static CSS_TEXT_TRANSFORM_INDEX  : int = 24;
pub static CSS_TEXT_TRANSFORM_SHIFT : int =  0;
pub static CSS_TEXT_TRANSFORM_MASK : int =  0x7;
pub static CSS_TEXT_INDENT_INDEX  : int = 25;
pub static CSS_TEXT_INDENT_SHIFT : int =  3;
pub static CSS_TEXT_INDENT_MASK : int =  0xf8;
pub static CSS_WHITE_SPACE_INDEX  : int = 25;
pub static CSS_WHITE_SPACE_SHIFT  : int = 0;
pub static CSS_WHITE_SPACE_MASK : int = 0x7;
pub static CSS_BACKGROUND_POSITION_INDEX : int =  27;
pub static CSS_BACKGROUND_POSITION_SHIFT : int =  7;
pub static CSS_BACKGROUND_POSITION_MASK : int =  0x80;
pub static CSS_BACKGROUND_POSITION_INDEX1 : int =  26;
pub static CSS_BACKGROUND_POSITION_SHIFT1 : int =  0;
pub static CSS_BACKGROUND_POSITION_MASK1 : u8 =  0xff;
pub static CSS_DISPLAY_INDEX  : int = 27;
pub static CSS_DISPLAY_SHIFT  : int = 2;
pub static CSS_DISPLAY_MASK : int =  0x7c;
pub static CSS_FONT_VARIANT_INDEX : int = 27;
pub static CSS_FONT_VARIANT_SHIFT : int =  0;
pub static CSS_FONT_VARIANT_MASK : int =  0x3;
pub static CSS_TEXT_DECORATION_INDEX : int =  28;
pub static CSS_TEXT_DECORATION_SHIFT : int =  3;
pub static CSS_TEXT_DECORATION_MASK : int =  0xf8;
pub static CSS_FONT_FAMILY_INDEX  : int = 28;
pub static CSS_FONT_FAMILY_SHIFT : int =  0;
pub static CSS_FONT_FAMILY_MASK : int =  0x7;
pub static CSS_BORDER_TOP_STYLE_INDEX : int =  29;
pub static CSS_BORDER_TOP_STYLE_SHIFT : int =  4;
pub static CSS_BORDER_TOP_STYLE_MASK : int =  0xf0;
pub static CSS_BORDER_RIGHT_STYLE_INDEX  : int = 29;
pub static CSS_BORDER_RIGHT_STYLE_SHIFT : int =  0;
pub static CSS_BORDER_RIGHT_STYLE_MASK : int =  0xf;
pub static CSS_BORDER_BOTTOM_STYLE_INDEX  : int = 30;
pub static CSS_BORDER_BOTTOM_STYLE_SHIFT : int =  4;
pub static CSS_BORDER_BOTTOM_STYLE_MASK : int =  0xf0;
pub static CSS_BORDER_LEFT_STYLE_INDEX  : int = 30;
pub static CSS_BORDER_LEFT_STYLE_SHIFT : int =  0;
pub static CSS_BORDER_LEFT_STYLE_MASK : int =  0xf;
pub static CSS_FONT_WEIGHT_INDEX  : int = 31;
pub static CSS_FONT_WEIGHT_SHIFT : int =  4;
pub static CSS_FONT_WEIGHT_MASK : int =  0xf0;
pub static CSS_LIST_STYLE_TYPE_INDEX : int =  31;
pub static CSS_LIST_STYLE_TYPE_SHIFT : int =  0;
pub static CSS_LIST_STYLE_TYPE_MASK : int =  0xf;
pub static CSS_OUTLINE_STYLE_INDEX : int =  32;
pub static CSS_OUTLINE_STYLE_SHIFT : int = 4;
pub static CSS_OUTLINE_STYLE_MASK : int =  0xf0;
pub static CSS_TABLE_LAYOUT_INDEX  : int = 32;
pub static CSS_TABLE_LAYOUT_SHIFT : int =  2;
pub static CSS_TABLE_LAYOUT_MASK : int = 0xc;
pub static CSS_UNICODE_BIDI_INDEX  : int = 32;
pub static CSS_UNICODE_BIDI_SHIFT  : int = 0;
pub static CSS_UNICODE_BIDI_MASK : int =  0x3;
pub static CSS_VISIBILITY_INDEX : int =  33;
pub static CSS_VISIBILITY_SHIFT : int =  6;
pub static CSS_VISIBILITY_MASK : int =  0xc0;
pub static CSS_LIST_STYLE_POSITION_INDEX : int =  33;
pub static CSS_LIST_STYLE_POSITION_SHIFT  : int = 4;
pub static CSS_LIST_STYLE_POSITION_MASK : int =  0x30;
pub static CSS_TEXT_ALIGN_INDEX  : int = 33;
pub static CSS_TEXT_ALIGN_SHIFT : int =  0;
pub static CSS_TEXT_ALIGN_MASK : int =  0xf;
pub static CSS_PAGE_BREAK_AFTER_INDEX : int =  0;
pub static CSS_PAGE_BREAK_AFTER_SHIFT : int =  0;
pub static CSS_PAGE_BREAK_AFTER_MASK : int = 0x7;
pub static CSS_PAGE_BREAK_BEFORE_INDEX  : int = 0;
pub static CSS_PAGE_BREAK_BEFORE_SHIFT : int =  3;
pub static CSS_PAGE_BREAK_BEFORE_MASK : int =  0x38;
pub static CSS_PAGE_BREAK_INSIDE_INDEX  : int = 0;
pub static CSS_PAGE_BREAK_INSIDE_SHIFT : int =  6;
pub static CSS_PAGE_BREAK_INSIDE_MASK : int =  0xc0;

/////////////////////////////////////////////


pub struct css_computed_counter {
    name:uint ,
    value:i32
}

impl Clone for css_computed_counter {  
    fn clone(&self) -> css_computed_counter {     
        css_computed_counter{  
            name: self.name,  
            value: self.value 
        }  
    }  
}

pub struct css_computed_content_item_counter {
    name:uint,
    sep:Option<uint>,
    style:u8
}

pub struct css_computed_content_item {
  
    item_type:css_computed_content_item_type,

    data:Option<uint>,
    counters_data:Option<css_computed_content_item_counter>
}

impl Clone for css_computed_content_item {  
    fn clone(&self) -> css_computed_content_item {     
        css_computed_content_item{  
            item_type: self.item_type,
            data: self.data,
            counters_data: self.counters_data 
        }  
    }  
}

pub struct css_computed_uncommon {
/*
 * border_spacing         1 + 2(4)    2(4)
 * clip               2 + 4(4) + 4    4(4)
 * letter_spacing         2 + 4       4
 * outline_color          2       4
 * outline_width          3 + 4       4
 * word_spacing           2 + 4       4
 *              ---     ---
 *               52 bits     40 bytes
 *
 * Encode counter_increment and _reset as an array of name, value pairs,
 * terminated with a blank entry.
 *
 * counter_increment          1       sizeof(ptr)
 * counter_reset          1       sizeof(ptr)
 *              ---     ---
 *                2 bits      2sizeof(ptr) bytes
 *
 * Encode cursor uri(s) as an array of string objects, terminated with a
 * blank entry.
 *
 * cursor             5       sizeof(ptr)
 *              ---     ---
 *                5 bits      sizeof(ptr) bytes
 *
 * Encode content as an array of content items, terminated with a blank entry.
 *
 * content            2       sizeof(ptr)
 *              ---     ---
 *                2 bits      sizeof(ptr)
 *
 *              ___     ___
 *               61 bits     40 + 4sizeof(ptr) bytes
 *
 *                8 bytes    40 + 4sizeof(ptr) bytes
 *              ===================
 *               48 + 4sizeof(ptr) bytes
 *
 * Bit allocations:
 *
 *    76543210
 *  1 llllllcc  letter-spacing | outline-color
 *  2 ooooooob  outline-width  | border-spacing
 *  3 bbbbbbbb  border-spacing
 *  4 wwwwwwir  word-spacing   | counter-increment | counter-reset
 *  5 uuuuu...  cursor         | <unused>
 *  6 cccccccc  clip
 *  7 cccccccc  clip
 *  8 ccccccoo  clip           | content
 */
    bits:~[u8],//~[u8, ..8],

    border_spacing:~[i32],//~[i32, ..2],

    clip:~[i32],//~[i32, ..4],

    letter_spacing:i32,

    outline_color:u32,
    outline_width:i32,

    word_spacing:i32,

    counter_increment:~[~css_computed_counter],
    counter_reset:~[~css_computed_counter],

    cursor:~[uint],

    content:~[~css_computed_content_item],
}


pub struct css_computed_page {
/*
 * page_break_after       3
 * page_break_before          3
 * page_break_inside          2
 *              ---
 *                8 bits
 */
    bits:~[u8],//~[u8, ..1]
} 
    
pub struct css_aural ;

pub struct css_computed_style {
/*
 * background_attachment      2
 * background_repeat          3
 * border_collapse        2
 * border_top_style       4
 * border_right_style         4
 * border_bottom_style        4
 * border_left_style          4
 * caption_side           2
 * clear              3
 * direction              2
 * display            5
 * empty_cells            2
 * float              2
 * font_style             2
 * font_variant           2
 * font_weight            4
 * list_style_position        2
 * list_style_type        4
 * overflow           3
 * outline_style          4
 * position           3
 * table_layout           2
 * text_align             4
 * text_decoration        5
 * text_transform         3
 * unicode_bidi           2
 * visibility             2
 * white_space            3
 *              ---
 *               84 bits
 *
 * Colours are 32bits of AARRGGBB
 * Dimensions are encoded as a fixed point value + 4 bits of unit data
 *
 * background_color       2       4
 * background_image       1       sizeof(ptr)
 * background_position        1 + 2(4)    2(4)
 * border_top_color       2       4
 * border_right_color         2       4
 * border_bottom_color        2       4
 * border_left_color          2       4
 * border_top_width       3 + 4       4
 * border_right_width         3 + 4       4
 * border_bottom_width        3 + 4       4
 * border_left_width          3 + 4       4
 * top                2 + 4       4
 * right              2 + 4       4
 * bottom             2 + 4       4
 * left               2 + 4       4
 * color              1       4
 * font_size              4 + 4       4
 * height             2 + 4       4
 * line_height            2 + 4       4
 * list_style_image       1       sizeof(ptr)
 * margin_top             2 + 4       4
 * margin_right           2 + 4       4
 * margin_bottom          2 + 4       4
 * margin_left            2 + 4       4
 * max_height             2 + 4       4
 * max_width              2 + 4       4
 * min_height             1 + 4       4
 * min_width              1 + 4       4
 * padding_top            1 + 4       4
 * padding_right          1 + 4       4
 * padding_bottom         1 + 4       4
 * padding_left           1 + 4       4
 * text_indent            1 + 4       4
 * vertical_align         4 + 4       4
 * width              2 + 4       4
 * z_index            2       4
 *              ---     ---
 *              181 bits    140 + 2sizeof(ptr) bytes
 *
 * Encode font family as an array of string objects, terminated with a 
 * blank entry.
 *
 * font_family            3       sizeof(ptr)
 *              ---     ---
 *                3 bits      sizeof(ptr)
 *
 * Encode quotes as an array of string objects, terminated with a blank entry.
 *
 * quotes             1       sizeof(ptr)
 *              ---     ---
 *                1 bit       sizeof(ptr) bytes
 *
 *              ___     ___
 *              269 bits    140 + 4sizeof(ptr) bytes
 *
 *               34 bytes   140 + 4sizeof(ptr) bytes
 *              ===================
 *              174 + 4sizeof(ptr) bytes
 *
 * Bit allocations:
 *
 *    76543210
 *  1 vvvvvvvv  vertical-align
 *  2 ffffffff  font-size
 *  3 ttttttti  border-top-width    | background-image
 *  4 rrrrrrrc  border-right-width  | color
 *  5 bbbbbbbl  border-bottom-width | list-style-image
 *  6 lllllllq  border-left-width   | quotes
 *  7 ttttttcc  top                 | border-top-color
 *  8 rrrrrrcc  right               | border-right-color
 *  9 bbbbbbcc  bottom              | border-bottom-color
 * 10 llllllcc  left                | border-left-color
 * 11 hhhhhhbb  height              | background-color
 * 12 llllllzz  line-height         | z-index
 * 13 ttttttbb  margin-top          | background-attachment
 * 14 rrrrrrbb  margin-right        | border-collapse
 * 15 bbbbbbcc  margin-bottom       | caption-side
 * 16 lllllldd  margin-left         | direction
 * 17 mmmmmmee  max-height          | empty-cells
 * 18 mmmmmmff  max-width           | float
 * 19 wwwwwwff  width               | font-style
 * 20 mmmmmbbb  min-height          | background-repeat
 * 21 mmmmmccc  min-width           | clear
 * 22 tttttooo  padding-top         | overflow
 * 23 rrrrrppp  padding-right       | position
 * 24 bbbbbo..  padding-bottom      | opacity               | <unused>
 * 25 lllllttt  padding-left        | text-transform
 * 26 tttttwww  text-indent         | white-space
 * 27 bbbbbbbb  background-position
 * 28 bdddddff  background-position | display               | font-variant
 * 29 tttttfff  text-decoration     | font-family
 * 30 ttttrrrr  border-top-style    | border-right-style
 * 31 bbbbllll  border-bottom-style | border-left-style
 * 32 ffffllll  font-weight         | list-style-type
 * 33 oooottuu  outline-style       | table-layout          | unicode-bidi
 * 34 vvlltttt  visibility          | list-style-position   | text-align
 */
    bits:~[u8],

    background_color:u32,

    background_image:Option<uint>,

    background_position:~[i32],

    border_color:~[u32],
    border_width:~[i32],

    top:i32,
    right:i32,
    bottom:i32,
    left:i32,

    color:u32,

    font_size:i32,

    height:i32,

    line_height:i32,

    list_style_image:Option<uint>,

    margin:~[i32],

    max_height:i32,
    max_width:i32,

    min_height:i32,
    min_width:i32,

    opacity:i32,

    padding:~[i32],

    text_indent:i32,

    vertical_align:i32,

    width:i32,

    z_index:i32,

    font_family:~[uint],

    //quotes chaned from wapcaplet-strings to strings
    quotes:~[uint],

    uncommon:Option<~css_computed_uncommon>, /**< Uncommon properties */
    aural:Option<~css_aural>,         /*< Aural properties */
    page:Option<~css_computed_page> /* *< Page properties */

}


pub struct css_computed_clip_rect {
    top:i32,
    right:i32,
    bottom:i32,
    left:i32,

    tunit:css_unit,
    runit:css_unit,
    bunit:css_unit,
    lunit:css_unit,

    top_auto:bool,
    right_auto:bool,
    bottom_auto:bool,
    left_auto:bool
}


pub struct css_hint_length {
    value:i32,
    unit:css_unit
}

pub struct css_hint_length_hv {
    h:css_hint_length,
    v:css_hint_length
}

pub struct css_hint {
    hint_type:css_hint_data_type,
    status:u8,

    // types specifies , which data type is used from 10 types defined below
    clip:Option<~css_computed_clip_rect>,
    content:Option<~css_computed_content_item>,
    counters:Option<~[~css_computed_counter]>,
    length:Option<~css_hint_length>,
    position:Option<~css_hint_length_hv>,
    color:u32,
    fixed:i32,
    integer:i32,
    string:Option<uint>,
    strings:Option<~[uint]>
}

pub struct rect_result {
    hlength:i32,
    vlength:i32,
    hunit:css_unit,
    vunit:css_unit,
    result:u8
}

pub enum css_pseudo_element {
    CSS_PSEUDO_ELEMENT_NONE         = 0,
    CSS_PSEUDO_ELEMENT_FIRST_LINE   = 1,
    CSS_PSEUDO_ELEMENT_FIRST_LETTER = 2,
    CSS_PSEUDO_ELEMENT_BEFORE       = 3,
    CSS_PSEUDO_ELEMENT_AFTER        = 4,
    CSS_PSEUDO_ELEMENT_COUNT    = 5 
}

pub struct css_select_results {
    /*
     * Array of pointers to computed styles, 
     * indexed by css_pseudo_element. If there
     * was no styling for a given pseudo element, 
     * then no computed style will be created and
     * the corresponding pointer will be set to NULL
     */

     // for corresponding pointer to be null , then that index
     // item must be option , for setting that index location as null

     // taking style as "@mut" type everywhere , because we need to pass
     // pointer everywhere, and modification will occour every-where.
     // size of this array to be preallocated is CSS_PSEUDO_ELEMENT_COUNT
    styles:~[Option<~css_computed_style>]
}

pub struct reject_item {
    value:uint,
    sel_type:css_selector_type 
} 

// impl Clone for reject_item {
//     #[inline]
//     fn clone(&self) -> reject_item {
//         reject_item {
//             value:self.value.clone(),
//             sel_type:self.sel_type
//         }
//     }
// }

pub struct prop_state {
    specificity:uint,       /* Specificity of property in result */
    set       : bool,         /* Whether property is set in result */
    origin    : u8,         /* Origin of property in result */
    important : bool,         /* Importance of property in result */
    inherit   : bool         /* Property is set to inherit */
}

impl Clone for prop_state {
    #[inline]
    fn clone(&self) -> prop_state {
        prop_state {
            specificity: self.specificity,       /* Specificity of property in result */
            set        : self.set,          /* Whether property is set in result */
            origin     : self.origin,         /* Origin of property in result */
            important  : self.important,         /* Importance of property in result */
            inherit    : self.inherit
        }
    }
}

pub enum css_select_handler_version {
    CSS_SELECT_HANDLER_VERSION_1 = 1
}

pub struct css_select_handler {

    node_name: extern fn( node:*c_void, qname: &mut css_qname ) -> css_error,

    node_classes: extern fn(lwc_ref:&mut ~lwc, pw:*c_void, n:*c_void, classes: &mut ~[uint] ) -> css_error,

    node_id: extern fn(lwc_ref:&mut ~lwc, pw:*c_void, node:*c_void, id:&mut uint ) -> css_error,

    named_ancestor_node: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&mut css_qname, ancestor:*mut*c_void) -> css_error,
   
    named_parent_node: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&mut css_qname, parent:*mut*c_void) -> css_error,
    
    named_sibling_node: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&mut css_qname, sibling:*mut*c_void) -> css_error,

    named_generic_sibling_node: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&mut css_qname, sibling:*mut*c_void) -> css_error,
    
    parent_node: extern fn(node:*c_void, parent:*mut*c_void) -> css_error,

    sibling_node: extern fn(node:*c_void, sibling:*mut*c_void) -> css_error,

    node_has_name: extern fn(lwc_ref:&mut ~lwc, pw:*c_void,node:*c_void, qname:&css_qname, matched:&mut bool) -> css_error,

    node_has_class: extern fn(lwc_ref:&mut ~lwc, pw:*c_void, node:*c_void, name:uint, matched:&mut bool) -> css_error,

    node_has_id: extern fn(lwc_ref:&mut ~lwc, pw:*c_void, node:*c_void, name:uint, matched:&mut bool) -> css_error,

    node_has_attribute: extern fn(lwc_ref:&mut ~lwc, node:*c_void, name:&css_qname, matched:&mut bool) -> css_error,
    
    node_has_attribute_equal: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error,
   
    node_has_attribute_dashmatch: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error,

    node_has_attribute_includes: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error,

    node_has_attribute_prefix: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error,

    node_has_attribute_suffix: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error,

    node_has_attribute_substring: extern fn(lwc_ref:&mut ~lwc, node:*c_void, qname:&css_qname,value:uint, matched:&mut bool) -> css_error,

    node_is_root: extern fn(node:*c_void, matched:&mut bool) -> css_error,
   
    node_count_siblings: extern fn(lwc_ref:&mut ~lwc, node:*c_void, same_name:bool, after:bool, count:&mut i32) -> css_error,
    
    node_is_empty: extern fn(node:*c_void, matched:&mut bool) -> css_error,
    
    node_is_link: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_visited: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_hover: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_active: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_focus: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_enabled: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_disabled: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_checked: extern fn(node:*c_void, matched:&mut bool) -> css_error,
 
    node_is_target: extern fn(node:*c_void, matched:&mut bool) -> css_error,

    node_is_lang: extern fn(node:*c_void, lang:uint, matched:&mut bool) -> css_error,

    node_presentational_hint: extern fn(node:*c_void, property:u32) -> 
        (css_error,Option<~css_hint>),

    compute_font_size: extern fn(parent: Option<&mut ~css_hint>, size: Option<&mut ~css_hint>) -> css_error,
   
    ua_default_for_property: extern fn(property:u32, hint:&mut ~css_hint ) -> css_error,
    handler_version:uint
}

pub struct css_select_state {
    node:*c_void,
    media:u64,         
    results:~css_select_results,
    //result_styles: ~[~css_computed_style],
    current_pseudo:css_pseudo_element,  
    computed:uint,  

    handler:Option<~css_select_handler>,    
    pw:*c_void,
    sheet:Option<uint>,   

    current_origin:css_origin, 
    current_specificity:uint,  

    element:css_qname,       
     //changed id from wapcaplet-string to string
    id: uint,
     //changes classes from wapcaplet  to string
    classes:~[uint],
    n_classes:u32,           

    reject_cache: ~[Option<reject_item>],     /* Reject cache (filled from end) */  
    next_reject:int,                        /* Next free slot in reject cache */

    props: ~[Option<~[prop_state]>] 
} 

/*
 * Font face selection result set
 */
pub struct css_select_font_faces_results {
    
    /*
     * Array of pointers to computed font faces. 
     */
    font_faces:~[~[~css_font_face]],
}

#[inline]
pub fn advance_bytecode(style: &mut ~css_style) {
    
	// if (style.bytecode.len() - style.used > 0) {
		style.used += 1 
	// }
	// else {
	// 	fail!(~"Advancing Bytecode vector after end index")
	// }
    
}   

#[inline]
pub fn peek_bytecode(style: &mut ~css_style) -> u32 {
    
	// if style.bytecode.len() - style.used > 0 {
		//debug!(fmt!("bytecode=%?",style.bytecode)); 
		style.bytecode[style.used] 
	// }
	// else {
	// 	fail!(~"Advancing Bytecode vector after end index")
	// }
    
}

/////////////////////////////////////////////////////////////////
