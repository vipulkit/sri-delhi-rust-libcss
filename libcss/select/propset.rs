use include::properties::*;
use include::types::*;

use select::common::*;
// use css_fpmath::*;
use wapcaplet::*;

#[inline]
pub fn ENSURE_UNCOMMON ( style:@mut css_computed_style){
    match style.uncommon {
        Some(_)=>{},
        None=>{
            let uncommon_struct = @mut css_computed_uncommon {
                bits:~[ 
                        ((CSS_LETTER_SPACING_INHERIT as u8) << 2) | (CSS_OUTLINE_COLOR_INVERT as u8),
                        ((CSS_OUTLINE_WIDTH_MEDIUM as u8) << 1) | (CSS_BORDER_SPACING_INHERIT as u8),
                        0,
                        ((CSS_WORD_SPACING_INHERIT as u8) << 2) | 
                        ((CSS_COUNTER_INCREMENT_NONE as u8) << 1) | (CSS_COUNTER_RESET_NONE as u8),
                        ((CSS_CURSOR_INHERIT as u8) << 3) | 0,
                        0,
                        0,
                        ((CSS_CLIP_AUTO as u8) << 2) | (CSS_CONTENT_NORMAL as u8)
                ],
                border_spacing:~[0,0],
                clip:~[0,0,0,0],
                letter_spacing:0,
                outline_color:0,
                outline_width:0,
                word_spacing:0,
                counter_increment:~[],
                counter_reset:~[],
                cursor:~[],
                content:~[]
            };
            style.uncommon = Some(uncommon_struct);
        }
    }
}

#[inline]
pub fn ENSURE_PAGE ( style:@mut css_computed_style){
    match style.uncommon {
        Some(_)=>{},
        None=>{
            let page_struct = @mut css_computed_page {
                bits:~[ 
                        ( ((CSS_PAGE_BREAK_INSIDE_AUTO as u8) <<  6) | 
                          ((CSS_PAGE_BREAK_BEFORE_AUTO as u8) << 3) |
                           (CSS_PAGE_BREAK_AFTER_AUTO as u8) ) ]
            };
            style.page = Some(page_struct);
        }
    }
}

#[inline]
pub fn set_letter_spacing(style:@mut css_computed_style,
                    ftype:u8, 
                    length:i32, 
                    unit:css_unit)  {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_LETTER_SPACING_INDEX];
        let mask_complement = (CSS_LETTER_SPACING_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3) | ( (unit as u8) << 2) ) << CSS_LETTER_SPACING_SHIFT);}

    style.uncommon.get().letter_spacing = length;
}

#[inline]
pub fn set_outline_color(style:@mut css_computed_style,
                    ftype:u8, 
                    color:u32)  {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_OUTLINE_COLOR_INDEX];
        let mask_complement = (CSS_OUTLINE_COLOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_OUTLINE_COLOR_SHIFT);}
    
    style.uncommon.get().outline_color = color;
}

#[inline]
pub fn set_outline_width(style:@mut css_computed_style,
                    ftype:u8, 
                    length:i32, 
                    unit:css_unit)  {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_OUTLINE_WIDTH_INDEX];
        let mask_complement = (CSS_OUTLINE_WIDTH_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ((ftype & 0x7)|((unit as u8)<<3))  << CSS_OUTLINE_WIDTH_SHIFT);}

    style.uncommon.get().outline_width = length;
}

#[inline]
pub fn set_border_spacing(style:@mut css_computed_style,
                    ftype:u8, 
                    hlength:i32, 
                    hunit:css_unit,
                    vlength:i32,
                    vunit:css_unit)  {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_BORDER_SPACING_INDEX];
        let mask_complement = (CSS_BORDER_SPACING_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_BORDER_SPACING_SHIFT);}

    {let bits = &mut style.uncommon.get().bits[CSS_BORDER_SPACING_INDEX1];
        let mask_complement = (CSS_BORDER_SPACING_MASK1 as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( ( (hunit as u8) << 4)|(vunit as u8) )  << CSS_BORDER_SPACING_SHIFT1);}

    style.uncommon.get().border_spacing[0] = hlength;
    style.uncommon.get().border_spacing[1] = vlength;
}

#[inline]
pub fn set_word_spacing(style:@mut css_computed_style,
                    ftype:u8, 
                    length:i32, 
                    unit:css_unit)  {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_WORD_SPACING_INDEX];
        let mask_complement = (CSS_WORD_SPACING_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ((ftype & 0x3)|((unit as u8)<<2))  << CSS_WORD_SPACING_SHIFT);}

    style.uncommon.get().word_spacing = length;
}

#[inline]
pub fn set_counter_increment(style: @mut css_computed_style,
                            ftype:u8,
                            counters:~[@mut css_computed_counter]) {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_COUNTER_INCREMENT_INDEX];
        let mask_complement = (CSS_COUNTER_INCREMENT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_COUNTER_INCREMENT_SHIFT);}

    style.uncommon.get().counter_increment = counters ;
}

#[inline]
pub fn set_counter_reset(style: @mut css_computed_style,
                        ftype:u8,
                        counters:~[@mut css_computed_counter]) {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_COUNTER_RESET_INDEX];
        let mask_complement = (CSS_COUNTER_RESET_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_COUNTER_RESET_SHIFT);}

    style.uncommon.get().counter_reset = counters ;
}

#[inline]
pub fn set_cursor(style:@mut css_computed_style,
                    ftype:u8, 
                    urls:~[@mut lwc_string])  {

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_CURSOR_INDEX];
        let mask_complement = (CSS_CURSOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1f)  << CSS_CURSOR_SHIFT);}

    style.uncommon.get().cursor = urls;
}

#[inline]
pub fn set_clip(style: @mut css_computed_style,
                ftype:u8, 
                rect:@mut css_computed_clip_rect){

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_CLIP_INDEX];
        let mask_complement = (CSS_CLIP_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_CLIP_SHIFT);}

    if ( ftype == (CSS_CLIP_RECT as u8) ){
        {let bits = &mut style.uncommon.get().bits[CSS_CLIP_INDEX];
                *bits = *bits | 
                        ((
                        ( if (rect.top_auto   ) { 0x20 } else { 0 } ) |
                        ( if (rect.right_auto ) { 0x10 } else { 0 } ) |
                        ( if (rect.bottom_auto) { 0x8  } else { 0 } ) |
                        ( if (rect.left_auto  ) { 0x4  } else { 0 } ) ) 
                        << CSS_CLIP_SHIFT );}

        {let bits = &mut style.uncommon.get().bits[CSS_CLIP_INDEX1];
                let mask_complement = (CSS_CLIP_MASK1 as u8) ^ 0xff ;
                *bits = ( *bits & mask_complement ) |
                    ( (((rect.tunit as u8) << 4) | (rect.runit as u8) )  
                    << CSS_CLIP_SHIFT1);}

        {let bits = &mut style.uncommon.get().bits[CSS_CLIP_INDEX2];
                let mask_complement = (CSS_CLIP_MASK2 as u8) ^ 0xff ;
                *bits = ( *bits & mask_complement ) |
                    ( (((rect.bunit as u8) << 4) | (rect.lunit as u8) )  
                    << CSS_CLIP_SHIFT2);}

        style.uncommon.get().clip[0] = rect.top;
        style.uncommon.get().clip[1] = rect.right;
        style.uncommon.get().clip[2] = rect.bottom;
        style.uncommon.get().clip[3] = rect.left;
    }
}

#[inline]
pub fn set_content(style: @mut css_computed_style,
                ftype:u8, 
                content:~[@mut css_computed_content_item]){

    ENSURE_UNCOMMON(style);

    {let bits = &mut style.uncommon.get().bits[CSS_CONTENT_INDEX];
        let mask_complement = (CSS_CONTENT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_CONTENT_SHIFT);}

    style.uncommon.get().content = content ;
}

#[inline]
pub fn set_vertical_align(style:@mut css_computed_style,
                    ftype:u8, 
                    length:i32, 
                    unit:css_unit)  {

    {let bits = &mut style.bits[CSS_VERTICAL_ALIGN_INDEX];
        let mask_complement = (CSS_VERTICAL_ALIGN_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0xf) | ( (unit as u8) << 4) ) << CSS_VERTICAL_ALIGN_SHIFT);}

    style.vertical_align = length;

}

#[inline]
pub fn set_font_size(style:@mut css_computed_style,
                    ftype:u8, 
                    length:i32, 
                    unit:css_unit)  {

    {let bits = &mut style.bits[CSS_FONT_SIZE_INDEX];
        let mask_complement = (CSS_FONT_SIZE_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0xf) | ( (unit as u8) << 4) ) << CSS_FONT_SIZE_SHIFT);}

    style.font_size = length;
}

#[inline]
pub fn set_border_top_width(style:@mut css_computed_style,
                        ftype:u8, 
                        length:i32, 
                        unit:css_unit)  {

    {let bits = &mut style.bits[CSS_BORDER_TOP_WIDTH_INDEX];
        let mask_complement = (CSS_BORDER_TOP_WIDTH_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_TOP_WIDTH_SHIFT);}

    style.border_width[0] = length;
}

#[inline]
pub fn set_border_right_width(style:@mut css_computed_style,
                        ftype:u8, 
                        length:i32, 
                        unit:css_unit)  {

    {let bits = &mut style.bits[CSS_BORDER_RIGHT_WIDTH_INDEX];
        let mask_complement = (CSS_BORDER_RIGHT_WIDTH_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_RIGHT_WIDTH_SHIFT);}

    style.border_width[1] = length;
}

#[inline]
pub fn set_border_bottom_width(style:@mut css_computed_style,
                        ftype:u8, 
                        length:i32, 
                        unit:css_unit)  {

    {let bits = &mut style.bits[CSS_BORDER_BOTTOM_WIDTH_INDEX];
        let mask_complement = (CSS_BORDER_BOTTOM_WIDTH_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_BOTTOM_WIDTH_SHIFT);}

    style.border_width[2] = length;
}

#[inline]
pub fn set_border_left_width(style:@mut css_computed_style,
                        ftype:u8, 
                        length:i32, 
                        unit:css_unit)  {

    {let bits = &mut style.bits[CSS_BORDER_LEFT_WIDTH_INDEX];
        let mask_complement = (CSS_BORDER_LEFT_WIDTH_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x7) | ( (unit as u8) << 3) ) << CSS_BORDER_LEFT_WIDTH_SHIFT);}

    style.border_width[3] = length;
}

#[inline]
pub fn set_background_image(style:@mut css_computed_style,
                            ftype:u8,
                            url:Option<@mut lwc_string>) {

    {let bits = &mut style.bits[CSS_BACKGROUND_IMAGE_INDEX];
        let mask_complement = (CSS_BACKGROUND_IMAGE_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_BACKGROUND_IMAGE_SHIFT);}

    style.background_image = url;
}

#[inline]
pub fn set_color(style:@mut css_computed_style,
                            ftype:u8,
                            color:u32) {

    {let bits = &mut style.bits[CSS_COLOR_INDEX];
        let mask_complement = (CSS_COLOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_COLOR_SHIFT);}

    style.color = color;
}

#[inline]
pub fn set_list_style_image(style:@mut css_computed_style,
                            ftype:u8,
                            url:Option<@mut lwc_string>) {

    {let bits = &mut style.bits[CSS_LIST_STYLE_IMAGE_INDEX];
        let mask_complement = (CSS_LIST_STYLE_IMAGE_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_LIST_STYLE_IMAGE_SHIFT);}

    style.list_style_image = url;
}

#[inline]
pub fn set_quotes(style:@mut css_computed_style,
                ftype:u8,
                quotes:~[@mut lwc_string]) {

    {let bits = &mut style.bits[CSS_QUOTES_INDEX];
        let mask_complement = (CSS_QUOTES_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x1)  << CSS_QUOTES_SHIFT);}

    style.quotes = quotes ;
}

#[inline]
pub fn set_top(style:@mut css_computed_style,
                ftype:u8,
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_TOP_INDEX];
        let mask_complement = (CSS_TOP_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_TOP_SHIFT);}

    style.top = length ;
}

#[inline]
pub fn set_right(style:@mut css_computed_style,
                ftype:u8,
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_RIGHT_INDEX];
        let mask_complement = (CSS_RIGHT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_RIGHT_SHIFT);}

    style.right = length ;
}

#[inline]
pub fn set_bottom(style:@mut css_computed_style,
                ftype:u8,
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_BOTTOM_INDEX];
        let mask_complement = (CSS_BOTTOM_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_BOTTOM_SHIFT);}

    style.bottom = length ;
}

#[inline]
pub fn set_left(style:@mut css_computed_style,
                ftype:u8,
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_LEFT_INDEX];
        let mask_complement = (CSS_LEFT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ((ftype & 0x3)|((unit as u8) << 2))  << CSS_LEFT_SHIFT);}

    style.left = length ;
}

#[inline]
pub fn set_border_top_color(style:@mut css_computed_style,
                        ftype:u8,
                        color:u32) {

    {let bits = &mut style.bits[CSS_BORDER_TOP_COLOR_INDEX];
        let mask_complement = (CSS_BORDER_TOP_COLOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_BORDER_TOP_COLOR_SHIFT);}

    style.border_color[0] = color ;
}

#[inline]
pub fn set_border_right_color(style:@mut css_computed_style,
                        ftype:u8,
                        color:u32) {

    {let bits = &mut style.bits[CSS_BORDER_RIGHT_COLOR_INDEX];
        let mask_complement = (CSS_BORDER_RIGHT_COLOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_BORDER_RIGHT_COLOR_SHIFT);}

    style.border_color[1] = color ;
}

#[inline]
pub fn set_border_bottom_color(style:@mut css_computed_style,
                        ftype:u8,
                        color:u32) {

    {let bits = &mut style.bits[CSS_BORDER_BOTTOM_COLOR_INDEX];
        let mask_complement = (CSS_BORDER_BOTTOM_COLOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_BORDER_BOTTOM_COLOR_SHIFT);}

    style.border_color[2] = color ;
}

#[inline]
pub fn set_border_left_color(style:@mut css_computed_style,
                        ftype:u8,
                        color:u32) {

    {let bits = &mut style.bits[CSS_BORDER_LEFT_COLOR_INDEX];
        let mask_complement = (CSS_BORDER_LEFT_COLOR_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x3)  << CSS_BORDER_LEFT_COLOR_SHIFT);}

    style.border_color[3] = color ;
}

#[inline]
pub fn set_height(style:@mut css_computed_style,
                ftype:u8,
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_HEIGHT_INDEX];
        let mask_complement = (CSS_HEIGHT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_HEIGHT_SHIFT);}

    style.height = length ;
}

#[inline]
pub fn set_line_height(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_LINE_HEIGHT_INDEX];
        let mask_complement = (CSS_LINE_HEIGHT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_LINE_HEIGHT_SHIFT);}

    style.line_height = length ;
}

#[inline]
pub fn set_background_color(style:@mut css_computed_style,
                    ftype:u8, 
                    color:u32) {

    {let bits = &mut style.bits[CSS_BACKGROUND_COLOR_INDEX];
        let mask_complement = (CSS_BACKGROUND_COLOR_MASK as u8) ^ 0xff ;
    
        *bits = (*bits & mask_complement) |
                ((ftype & 0x3) << CSS_BACKGROUND_COLOR_SHIFT);}

    style.background_color = color;
}

#[inline]
pub fn set_z_index(style:@mut css_computed_style,
                ftype:u8,
                z_index:i32) {

    {let bits = &mut style.bits[CSS_Z_INDEX_INDEX];
        let mask_complement = (CSS_Z_INDEX_MASK as u8) ^ 0xff ;
    
        *bits = (*bits & mask_complement) |
                ((ftype & 0x3) << CSS_Z_INDEX_SHIFT);}
    style.z_index = z_index;
}

#[inline]
pub fn set_margin_top(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_MARGIN_TOP_INDEX];
        let mask_complement = (CSS_MARGIN_TOP_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_TOP_SHIFT);}

    style.margin[0] = length ;
}

#[inline]
pub fn set_margin_right(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_MARGIN_RIGHT_INDEX];
        let mask_complement = (CSS_MARGIN_RIGHT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_RIGHT_SHIFT);}

    style.margin[1] = length ;
}

#[inline]
pub fn set_margin_bottom(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_MARGIN_BOTTOM_INDEX];
        let mask_complement = (CSS_MARGIN_BOTTOM_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_BOTTOM_SHIFT);}

    style.margin[2] = length ;
}

#[inline]
pub fn set_margin_left(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_MARGIN_LEFT_INDEX];
        let mask_complement = (CSS_MARGIN_LEFT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) )  << CSS_MARGIN_LEFT_SHIFT);}

    style.margin[3] = length ;
}

#[inline]
pub fn set_background_attachment(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_BACKGROUND_ATTACHMENT_INDEX];
    let mask_complement = (CSS_BACKGROUND_ATTACHMENT_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_BACKGROUND_ATTACHMENT_SHIFT);
}

#[inline]
pub fn set_border_collapse(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_BORDER_COLLAPSE_INDEX];
    let mask_complement = (CSS_BORDER_COLLAPSE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_BORDER_COLLAPSE_SHIFT);
}

#[inline]
pub fn set_caption_side(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_CAPTION_SIDE_INDEX];
    let mask_complement = (CSS_CAPTION_SIDE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_CAPTION_SIDE_SHIFT);
}

#[inline]
pub fn set_direction(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_DIRECTION_INDEX];
    let mask_complement = (CSS_DIRECTION_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_DIRECTION_SHIFT);
}

#[inline]
pub fn set_max_height( style: @mut css_computed_style,
                ftype:u8, 
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_MAX_HEIGHT_INDEX];
        let mask_complement = (CSS_MAX_HEIGHT_MASK as u8) ^ 0xff ;
    
        *bits = (*bits & mask_complement) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_MAX_HEIGHT_SHIFT);}

    style.max_height = length;
}

#[inline]
pub fn set_max_width( style: @mut css_computed_style,
                ftype:u8, 
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_MAX_WIDTH_INDEX];
        let mask_complement = (CSS_MAX_WIDTH_MASK as u8) ^ 0xff ;
    
        *bits = (*bits & mask_complement) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_MAX_WIDTH_SHIFT);}

    style.max_width = length;
}

#[inline]
pub fn set_width( style: @mut css_computed_style,
                ftype:u8, 
                length:i32,
                unit:css_unit) {

    {let bits = &mut style.bits[CSS_WIDTH_INDEX];
        let mask_complement = (CSS_WIDTH_MASK as u8) ^ 0xff ;
    
        *bits = (*bits & mask_complement) |
                ( ( (ftype & 0x3)|((unit as u8) <<2) ) << CSS_WIDTH_SHIFT);}

    style.width = length;
}

#[inline]
pub fn set_empty_cells(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_EMPTY_CELLS_INDEX];
    let mask_complement = (CSS_EMPTY_CELLS_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_EMPTY_CELLS_SHIFT);
}

#[inline]
pub fn set_float(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_FLOAT_INDEX];
    let mask_complement = (CSS_FLOAT_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_FLOAT_SHIFT);
}

#[inline]
pub fn set_font_style(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_FONT_STYLE_INDEX];
    let mask_complement = (CSS_FONT_STYLE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_FONT_STYLE_SHIFT);
}

#[inline]
pub fn set_min_height(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_MIN_HEIGHT_INDEX];
        let mask_complement = (CSS_MIN_HEIGHT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_MIN_HEIGHT_SHIFT);}

    style.min_height = length;
}

#[inline]
pub fn set_min_width(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_MIN_WIDTH_INDEX];
        let mask_complement = (CSS_MIN_WIDTH_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_MIN_WIDTH_SHIFT);}

    style.min_width = length;
}

#[inline]
pub fn set_background_repeat(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_BACKGROUND_REPEAT_INDEX];
    let mask_complement = (CSS_BACKGROUND_REPEAT_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_BACKGROUND_REPEAT_SHIFT);
}

#[inline]
pub fn set_clear(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_CLEAR_INDEX];
    let mask_complement = (CSS_CLEAR_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_CLEAR_SHIFT);
}

#[inline]
pub fn set_padding_top(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_PADDING_TOP_INDEX];
        let mask_complement = (CSS_PADDING_TOP_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_PADDING_TOP_SHIFT);}

    style.padding[0] = length;
}

#[inline]
pub fn set_padding_right(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_PADDING_RIGHT_INDEX];
        let mask_complement = (CSS_PADDING_RIGHT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_PADDING_RIGHT_SHIFT);}

    style.padding[1] = length;
}

#[inline]
pub fn set_padding_bottom(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_PADDING_BOTTOM_INDEX];
        let mask_complement = (CSS_PADDING_BOTTOM_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_PADDING_BOTTOM_SHIFT);}

    style.padding[2] = length;
}

#[inline]
pub fn set_padding_left(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_PADDING_LEFT_INDEX];
        let mask_complement = (CSS_PADDING_LEFT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_PADDING_LEFT_SHIFT);}

    style.padding[3] = length;
}

#[inline]
pub fn set_overflow(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_OVERFLOW_INDEX];
    let mask_complement = (CSS_OVERFLOW_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_OVERFLOW_SHIFT);
}

#[inline]
pub fn set_position(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_POSITION_INDEX];
    let mask_complement = (CSS_POSITION_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_POSITION_SHIFT);
}

#[inline]
pub fn set_opacity(style:@mut css_computed_style,
                            ftype:u8,
                            opacity:i32) {

    {let bits = &mut style.bits[CSS_OPACITY_INDEX];
        let mask_complement = (CSS_OPACITY_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x7)  << CSS_OPACITY_SHIFT);}

    style.opacity = opacity;
}

#[inline]
pub fn set_text_transform(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_TEXT_TRANSFORM_INDEX];
    let mask_complement = (CSS_TEXT_TRANSFORM_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_TEXT_TRANSFORM_SHIFT);
}

#[inline]
pub fn set_text_indent(style:@mut css_computed_style,
                    ftype:u8,
                    length:i32,
                    unit:css_unit) {

    {let bits = &mut style.bits[CSS_TEXT_INDENT_INDEX];
        let mask_complement = (CSS_TEXT_INDENT_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( ( (ftype & 0x1)|((unit as u8) << 1) )  << CSS_TEXT_INDENT_SHIFT);}

    style.text_indent = length;
}

#[inline]
pub fn set_white_space(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_WHITE_SPACE_INDEX];
    let mask_complement = (CSS_WHITE_SPACE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_WHITE_SPACE_SHIFT);
}

#[inline]
pub fn set_background_position(style : @mut css_computed_style,
                            ftype:u8,
                            hlength:i32,
                            hunit:css_unit,
                            vlength:i32,
                            vunit:css_unit) {

    {
        let bits = &mut style.bits[CSS_BACKGROUND_POSITION_INDEX];
        let mask_complement = (CSS_BACKGROUND_POSITION_MASK as u8) ^ 0xff ;

        *bits = (*bits & mask_complement) |
                ((ftype & 0x1) << CSS_BACKGROUND_POSITION_SHIFT);
    }
    
    {
        let bits = &mut style.bits[CSS_BACKGROUND_POSITION_INDEX1];
        let mask_complement = (CSS_BACKGROUND_POSITION_MASK1 as u8) ^ 0xff ;

        *bits = (*bits & mask_complement) |
                ( (( (hunit as u8) << 4) | (vunit as u8) ) << CSS_BACKGROUND_POSITION_SHIFT1);
    }

    style.background_position[0] = hlength;
    style.background_position[1] = vlength;
}

#[inline]
pub fn set_display(style:@mut css_computed_style,
                ftype:u8) {

    let bits = &mut style.bits[CSS_DISPLAY_INDEX];
    let mask_complement = (CSS_DISPLAY_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x1F)  << CSS_DISPLAY_SHIFT);
}

#[inline]
pub fn set_font_variant(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_FONT_VARIANT_INDEX];
    let mask_complement = (CSS_FONT_VARIANT_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_FONT_VARIANT_SHIFT);
}

#[inline]
pub fn set_text_decoration(style:@mut css_computed_style,
                        ftype:u8) {

    let bits = &mut style.bits[CSS_TEXT_DECORATION_INDEX];
    let mask_complement = (CSS_TEXT_DECORATION_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x1F)  << CSS_TEXT_DECORATION_SHIFT);
}

#[inline]
pub fn set_font_family(style:@mut css_computed_style,
                    ftype:u8,
                    names:~[@mut lwc_string]) {

    {let bits = &mut style.bits[CSS_FONT_FAMILY_INDEX];
        let mask_complement = (CSS_FONT_FAMILY_MASK as u8) ^ 0xff ;
        *bits = ( *bits & mask_complement ) |
                ( (ftype & 0x7)  << CSS_FONT_FAMILY_SHIFT);}

    style.font_family = names;
}

#[inline]
pub fn set_border_top_style(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_BORDER_TOP_STYLE_INDEX];
    let mask_complement = (CSS_BORDER_TOP_STYLE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_BORDER_TOP_STYLE_SHIFT);
}

#[inline]
pub fn set_border_right_style(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_BORDER_RIGHT_STYLE_INDEX];
    let mask_complement = (CSS_BORDER_RIGHT_STYLE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_BORDER_RIGHT_STYLE_SHIFT);
}

#[inline]
pub fn set_border_bottom_style(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_BORDER_BOTTOM_STYLE_INDEX];
    let mask_complement = (CSS_BORDER_BOTTOM_STYLE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_BORDER_BOTTOM_STYLE_SHIFT);
}

#[inline]
pub fn set_border_left_style(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_BORDER_LEFT_STYLE_INDEX];
    let mask_complement = (CSS_BORDER_LEFT_STYLE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_BORDER_LEFT_STYLE_SHIFT);
}

#[inline]
pub fn set_font_weight(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_FONT_WEIGHT_INDEX];
    let mask_complement = (CSS_FONT_WEIGHT_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_FONT_WEIGHT_SHIFT);
}

#[inline]
pub fn set_list_style_type(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_LIST_STYLE_TYPE_INDEX];
    let mask_complement = (CSS_LIST_STYLE_TYPE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_LIST_STYLE_TYPE_SHIFT);
}

#[inline]
pub fn set_outline_style(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_OUTLINE_STYLE_INDEX];
    let mask_complement = (CSS_OUTLINE_STYLE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_OUTLINE_STYLE_SHIFT);
}

#[inline]
pub fn set_table_layout(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_TABLE_LAYOUT_INDEX];
    let mask_complement = (CSS_TABLE_LAYOUT_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_TABLE_LAYOUT_SHIFT);
}

#[inline]
pub fn set_unicode_bidi(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_UNICODE_BIDI_INDEX];
    let mask_complement = (CSS_UNICODE_BIDI_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_UNICODE_BIDI_SHIFT);
}

#[inline]
pub fn set_visibility(style:@mut css_computed_style,
                    ftype:u8) {

    let bits = &mut style.bits[CSS_VISIBILITY_INDEX];
    let mask_complement = (CSS_VISIBILITY_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_VISIBILITY_SHIFT);
}

#[inline]
pub fn set_list_style_position(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_LIST_STYLE_POSITION_INDEX];
    let mask_complement = (CSS_LIST_STYLE_POSITION_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_LIST_STYLE_POSITION_SHIFT);
}

#[inline]
pub fn set_text_align(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_TEXT_ALIGN_INDEX];
    let mask_complement = (CSS_TEXT_ALIGN_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0xf)  << CSS_TEXT_ALIGN_SHIFT);
}

#[inline]
pub fn set_page_break_after(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_PAGE_BREAK_AFTER_INDEX];
    let mask_complement = (CSS_PAGE_BREAK_AFTER_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_PAGE_BREAK_AFTER_SHIFT);
}

#[inline]
pub fn set_page_break_before(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_PAGE_BREAK_BEFORE_INDEX];
    let mask_complement = (CSS_PAGE_BREAK_BEFORE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x7)  << CSS_PAGE_BREAK_BEFORE_SHIFT);
}

#[inline]
pub fn set_page_break_inside(style:@mut css_computed_style,
                            ftype:u8) {

    let bits = &mut style.bits[CSS_PAGE_BREAK_INSIDE_INDEX];
    let mask_complement = (CSS_PAGE_BREAK_INSIDE_MASK as u8) ^ 0xff ;
    *bits = ( *bits & mask_complement ) |
            ( (ftype & 0x3)  << CSS_PAGE_BREAK_INSIDE_SHIFT);
}


///////////////////////////////////////////////////////////////////////////
