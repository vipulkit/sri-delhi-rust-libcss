#[link(name = "css_select_computed", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_select_const;
extern mod css_fpmath;
extern mod css_select_propset;
extern mod std ;

use css_enum::* ;
use css_select_const::*;
use css_fpmath::*;
use css_select_propset::*;

// function pointer : used in "css__compute_absolute_values" function 
pub type css_fnptr_compute_font_size =  ~extern fn(parent:Option<@mut css_hint>,
                                                size:Option<@mut css_hint> ) 
                                                    -> css_result ;

////////////////////////////////////

pub fn css_computed_letter_spacing(
        style : @mut css_computed_style) 
        -> (u8,Option<i32>,Option<css_unit>) {

    let mut length :Option<i32> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            (CSS_LETTER_SPACING_NORMAL as u8,length,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_LETTER_SPACING_INDEX];
            bits = bits & (CSS_LETTER_SPACING_MASK as u8);
            bits = bits >> CSS_LETTER_SPACING_SHIFT;

            if (bits&3) == (CSS_LETTER_SPACING_SET as u8) { 
                length = Some(uncommon_struct.letter_spacing);
                unit = Some(unsafe { cast::transmute((bits >> 2)as int) }) ;
            }

            ((bits&3),length,unit)
        }
    }
}

pub fn css_computed_outline_color(
                    style: @mut css_computed_style) 
                    -> (u8,Option<u32>) {

    let mut color : Option<u32> = None;
    match style.uncommon {
        None=>{
            (CSS_OUTLINE_COLOR_INVERT as u8,color)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_OUTLINE_COLOR_INDEX];
            bits = bits & (CSS_OUTLINE_COLOR_MASK as u8) ;
            bits = bits >> CSS_OUTLINE_COLOR_SHIFT ;

            if (bits&3) == (CSS_OUTLINE_COLOR_COLOR as u8) { 
                color = Some(uncommon_struct.outline_color) ;
            }

            ((bits&3),color)
        }
    }
}


pub fn css_computed_outline_width(
        style : @mut css_computed_style) 
        -> (u8,Option<i32>,Option<css_unit>) {

    let mut width :Option<i32> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            width = Some(css_int_to_fixed(2));
            unit = Some(CSS_UNIT_PX);
            (CSS_OUTLINE_WIDTH_WIDTH as u8,width,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_OUTLINE_WIDTH_INDEX];
            bits = bits & (CSS_OUTLINE_WIDTH_MASK as u8);
            bits = bits >> CSS_OUTLINE_WIDTH_SHIFT;

            if (bits&7) == (CSS_OUTLINE_WIDTH_WIDTH as u8) { 
                width = Some(uncommon_struct.outline_width);
                unit = Some(unsafe { cast::transmute((bits >> 3)as int) }) ;
            }

            ((bits&3),width,unit)
        }
    }
}

pub struct rect_result {
    hlength:i32,
    vlength:i32,
    hunit:css_unit,
    vunit:css_unit,
    result:u8
}

pub fn css_computed_border_spacing(
                    style : @mut css_computed_style) 
                    -> rect_result {

    let mut result = 
            rect_result{
                hlength:0,
                vlength:0,
                hunit:CSS_UNIT_PX,
                vunit:CSS_UNIT_PX,
                result:CSS_BORDER_SPACING_SET as u8
            };

    match style.uncommon {
        None=>{
            result
        },
        Some(uncommon_struct)=>{
            let mut bits = uncommon_struct.bits[CSS_BORDER_SPACING_INDEX];
            bits = bits & (CSS_BORDER_SPACING_MASK as u8);
            bits = bits >> CSS_BORDER_SPACING_SHIFT ;

            if bits == (CSS_BORDER_SPACING_SET as u8) { 
                let mut bits1 = uncommon_struct.bits[2];
                bits1 = bits1 & (CSS_BORDER_SPACING_MASK1 as u8);
                bits1 = bits1 >> CSS_BORDER_SPACING_SHIFT1 ;

                result.hlength = uncommon_struct.border_spacing[CSS_BORDER_SPACING_INDEX1];
                result.hunit = unsafe { cast::transmute((bits1 >> 4)as int) } ;

                result.vlength = uncommon_struct.border_spacing[1];
                result.vunit = unsafe { cast::transmute((bits1 & 0xf)as int) } ;
            }

            result.result = bits ;
            result
        }
    }
}

pub fn css_computed_word_spacing(
                    style : @mut css_computed_style)
                    -> (u8,Option<i32>,Option<css_unit>) {

    let mut length :Option<i32> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            (CSS_WORD_SPACING_NORMAL as u8,length,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_WORD_SPACING_INDEX];
            bits = bits & (CSS_WORD_SPACING_MASK as u8);
            bits = bits >> CSS_WORD_SPACING_SHIFT;

            if (bits&3) == (CSS_WORD_SPACING_SET as u8) { 
                length = Some(uncommon_struct.word_spacing);
                unit = Some(unsafe { cast::transmute((bits >> 2)as int) }) ;
            }

            ((bits&3),length,unit)
        }
    }

}

pub fn css_computed_counter_increment(
                        style : @mut css_computed_style)
                        -> (u8,Option<@mut css_computed_counter>) {

    let mut counter :Option<@mut css_computed_counter> = None;
    match style.uncommon {
        None=>{
            (CSS_COUNTER_INCREMENT_NONE as u8,counter)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_COUNTER_INCREMENT_INDEX];
            bits = bits & (CSS_COUNTER_INCREMENT_MASK as u8);
            bits = bits >> CSS_COUNTER_INCREMENT_SHIFT;

            counter = uncommon_struct.counter_increment;

            (bits,counter)
        }
    }  
}

pub fn css_computed_counter_reset(
                        style : @mut css_computed_style)
                        -> (u8,Option<@mut css_computed_counter>) {

    let mut counter :Option<@mut css_computed_counter> = None;
    match style.uncommon {
        None=>{
            (CSS_COUNTER_RESET_NONE as u8,counter)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_COUNTER_RESET_INDEX];
            bits = bits & (CSS_COUNTER_RESET_MASK as u8);
            bits = bits >> CSS_COUNTER_RESET_SHIFT;

            counter = uncommon_struct.counter_reset;

            (bits,counter)
        }
    }  
}

pub fn css_computed_cursor(
                style : @mut css_computed_style)
                -> (u8,Option<~[~str]>) {

    let mut urls :Option<~[~str]> = None;
    match style.uncommon {
        None=>{
            (CSS_CURSOR_AUTO as u8,urls)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_CURSOR_INDEX];
            bits = bits & (CSS_CURSOR_MASK as u8);
            bits = bits >> CSS_CURSOR_SHIFT;

            urls = Some(copy uncommon_struct.cursor);

            (bits,urls)
        }
    }  
}

pub fn css_computed_clip(
            style : @mut css_computed_style) 
            -> (u8,Option<css_computed_clip_rect>) {

    let mut result : css_computed_clip_rect = 
        css_computed_clip_rect{
            top:0,
            right:0,
            bottom:0,
            left:0,
            tunit:CSS_UNIT_PX,
            runit:CSS_UNIT_PX,
            bunit:CSS_UNIT_PX,
            lunit:CSS_UNIT_PX,
            top_auto:false,
            right_auto:false,
            bottom_auto:false,
            left_auto:false
    } ;

    match style.uncommon {
        None=>{
            ((CSS_CLIP_AUTO as u8),None)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_CLIP_INDEX];
            bits = bits & (CSS_CLIP_MASK as u8);
            bits = bits >> CSS_CLIP_SHIFT;

            if (bits&0x3) == (CSS_CLIP_RECT as u8) {
                let mut bits1 : u8 ;

                result.left_auto = (bits & 0x4)!=0;
                result.bottom_auto = (bits & 0x8)!=0;
                result.right_auto = (bits & 0x10)!=0;
                result.top_auto = (bits & 0x20)!=0;

                if (result.top_auto == false ||
                        result.right_auto == false) {
                    /* 8bits: ttttrrrr : top | right */
                    bits1 = uncommon_struct.bits[CSS_CLIP_INDEX1];
                    bits1 &= (CSS_CLIP_MASK1 as u8);
                    bits1 >>= CSS_CLIP_SHIFT1;
                } 
                else {
                    bits1 = 0;
                }

                result.top = uncommon_struct.clip[0];
                result.tunit = unsafe { cast::transmute((bits1 >> 4)as int)};

                result.right = uncommon_struct.clip[1];
                result.runit = unsafe { cast::transmute((bits1 & 0xf)as int)};

                if (result.bottom_auto == false ||
                        result.left_auto == false) {
                    /* 8bits: bbbbllll : bottom | left */
                    bits1 = uncommon_struct.bits[CSS_CLIP_INDEX2];
                    bits1 &= (CSS_CLIP_MASK2 as u8);
                    bits1 >>= CSS_CLIP_SHIFT2;
                } 
                else {
                    bits1 = 0;
                }

                result.bottom = uncommon_struct.clip[2];
                result.bunit = unsafe { cast::transmute((bits1 >> 4)as int)};

                result.left = uncommon_struct.clip[3];
                result.lunit = unsafe { cast::transmute((bits1 & 0xf)as int)} ;
            }

            ((bits&0x3),Some(result))
        }
    }
}

pub fn css_computed_content(
                style : @mut css_computed_style)
                -> (u8,Option<@mut css_computed_content_item>) {

    match style.uncommon {
        None=>{
            (CSS_CONTENT_NORMAL as u8,None)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[CSS_CONTENT_INDEX];
            bits = bits & (CSS_CONTENT_MASK as u8);
            bits = bits >> CSS_CONTENT_SHIFT;

            (bits,uncommon_struct.content)
        }
    }  
}

pub fn css_computed_vertical_align(
                    style : @mut css_computed_style) 
                        ->(u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_VERTICAL_ALIGN_INDEX];
    bits = bits & (CSS_VERTICAL_ALIGN_MASK as u8);
    bits = bits >> CSS_VERTICAL_ALIGN_SHIFT;

    if ((bits&0xf)==(CSS_VERTICAL_ALIGN_SET as u8)) {
        length = Some(style.vertical_align);
        unit = Some(unsafe { cast::transmute((bits >> 4) as int)});
        return ((bits&0xf),length,unit);
    }

    ((bits&0xf),length,unit)
}

pub fn css_computed_font_size(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_FONT_SIZE_INDEX];
    bits = bits & (CSS_FONT_SIZE_MASK as u8);
    bits = bits >> CSS_FONT_SIZE_SHIFT;

    if ((bits&0xf)==(CSS_FONT_SIZE_DIMENSION as u8)) {
        length = Some(style.font_size);
        unit = Some(unsafe { cast::transmute((bits >> 4) as int)});
        return ((bits&0xf),length,unit);
    }

    ((bits&0xf),length,unit)
}

pub fn css_computed_border_top_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_TOP_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_TOP_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[0]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_border_right_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_RIGHT_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[1]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_border_bottom_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_BOTTOM_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[2]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_border_left_width(
                            style : @mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_LEFT_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_LEFT_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        length = Some(style.border_width[3]);
        unit = Some(unsafe { cast::transmute((bits >> 3) as int)});
        return ((bits&0x7),length,unit);
    }

    ((bits&0x7),length,unit)
}

pub fn css_computed_background_image(
                                    style:@mut css_computed_style)
                                    -> (u8,~str) {

    let mut url : ~str ;
    let mut bits = style.bits[CSS_BACKGROUND_IMAGE_INDEX];
    bits = bits & (CSS_BACKGROUND_IMAGE_MASK as u8);
    bits = bits >> CSS_BACKGROUND_IMAGE_SHIFT;

    url = copy style.background_image;

    (bits,url)
}

pub fn css_computed_color(
                        style : @mut css_computed_style)
                        ->(u8,u32) {

    let mut bits = style.bits[CSS_COLOR_INDEX];
    bits = bits & (CSS_COLOR_MASK as u8);
    bits = bits >> CSS_COLOR_SHIFT ;

    (bits,style.color)
}

pub fn css_computed_list_style_image(
                                    style : @mut css_computed_style)
                                    ->(u8,~str) {

    let mut url : ~str ;
    let mut bits = style.bits[CSS_LIST_STYLE_IMAGE_INDEX];
    bits = bits & (CSS_LIST_STYLE_IMAGE_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_IMAGE_SHIFT;

    url = copy style.list_style_image;

    (bits,url)
}

pub fn css_computed_quotes(
                        style:@mut css_computed_style)
                        -> (u8,~[~str]) {

    let mut result : ~[~str] ;
    let mut bits = style.bits[CSS_QUOTES_INDEX];
    bits = bits & (CSS_QUOTES_MASK as u8);
    bits = bits >> CSS_QUOTES_SHIFT;

    result = copy style.quotes;
    (bits,result)
}

pub fn css_computed_top(
                    style : @mut css_computed_style)
                    -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_TOP_INDEX];
    bits = bits & (CSS_TOP_MASK as u8);
    bits = bits >> CSS_TOP_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_TOP_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut bottom : u8 = style.bits[CSS_BOTTOM_INDEX];
        bottom = bottom & (CSS_BOTTOM_MASK as u8);
        bottom = bottom >> CSS_BOTTOM_SHIFT;  

        if( (bits&0x3)==(CSS_TOP_AUTO as u8) &&
            (bottom&0x3)==(CSS_BOTTOM_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_TOP_AUTO as u8) ) {
            length = Some(-style.bottom);
            unit = Some(unsafe { cast::transmute((bottom >> 2) as int)});
        }
        else {
            length = Some(style.top);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_TOP_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_TOP_SET as u8) ) {
        length = Some(style.top);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_right(
                        style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_RIGHT_INDEX];
    bits = bits & (CSS_RIGHT_MASK as u8);
    bits = bits >> CSS_RIGHT_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_RIGHT_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut left : u8 = style.bits[CSS_LEFT_INDEX];
        left = left & (CSS_LEFT_MASK as u8);
        left = left >> CSS_LEFT_SHIFT;  

        if( (bits&0x3)==(CSS_RIGHT_AUTO as u8) &&
            (left&0x3)==(CSS_LEFT_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_RIGHT_AUTO as u8) ) {
            length = Some(-style.left);
            unit = Some(unsafe { cast::transmute((left >> 2) as int)});
        }
        else {
            length = Some(style.right);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_RIGHT_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_RIGHT_SET as u8) ) {
        length = Some(style.right);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_bottom(
                        style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_BOTTOM_INDEX];
    bits = bits & (CSS_BOTTOM_MASK as u8);
    bits = bits >> CSS_BOTTOM_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_BOTTOM_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut top : u8 = style.bits[CSS_TOP_INDEX];
        top = top & (CSS_TOP_MASK as u8);
        top = top >> CSS_TOP_SHIFT;  

        if( (bits&0x3)==(CSS_BOTTOM_AUTO as u8) &&
            (top&0x3)==(CSS_TOP_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_BOTTOM_AUTO as u8) || 
                   (top&0x3)==(CSS_TOP_AUTO as u8) ) {
            length = Some(-style.top);
            unit = Some(unsafe { cast::transmute((top >> 2) as int)});
        }
        else {
            length = Some(style.bottom);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_BOTTOM_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_BOTTOM_SET as u8) ) {
        length = Some(style.bottom);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_left(
                        style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_LEFT_INDEX];
    bits = bits & (CSS_LEFT_MASK as u8);
    bits = bits >> CSS_LEFT_SHIFT;   
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_LEFT_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut right : u8 = style.bits[CSS_RIGHT_INDEX];
        right = right & (CSS_RIGHT_MASK as u8);
        right = right >> CSS_RIGHT_SHIFT;  

        if( (bits&0x3)==(CSS_LEFT_AUTO as u8) &&
            (right&0x3)==(CSS_RIGHT_AUTO as u8) ) {
            length = Some(0);
            unit = Some(CSS_UNIT_PX);
        }
        else if ( (bits&0x3)==(CSS_LEFT_AUTO as u8) ) {
            length = Some(-style.right);
            unit = Some(unsafe { cast::transmute((right >> 2) as int)});
        }
        else {
            length = Some(style.left);
            unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
        }

        bits = (CSS_LEFT_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_LEFT_SET as u8) ) {
        length = Some(style.left);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_border_top_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_TOP_COLOR_INDEX];
    bits = bits & (CSS_BORDER_TOP_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_COLOR_SHIFT; 

    (bits,style.border_color[0]) 
}

pub fn css_computed_border_right_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_RIGHT_COLOR_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_COLOR_SHIFT; 

    (bits,style.border_color[1]) 
}

pub fn css_computed_border_bottom_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_BOTTOM_COLOR_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_COLOR_SHIFT; 

    (bits,style.border_color[2]) 
}

pub fn css_computed_border_left_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BORDER_LEFT_COLOR_INDEX];
    bits = bits & (CSS_BORDER_LEFT_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_COLOR_SHIFT; 

    (bits,style.border_color[3]) 
}

pub fn css_computed_height(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_HEIGHT_INDEX];
    bits = bits & (CSS_HEIGHT_MASK as u8);
    bits = bits >> CSS_HEIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_HEIGHT_SET as u8) ) {
        length = Some(style.height);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_line_height(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_LINE_HEIGHT_INDEX];
    bits = bits & (CSS_LINE_HEIGHT_MASK as u8);
    bits = bits >> CSS_LINE_HEIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_LINE_HEIGHT_NUMBER as u8) || 
         (bits&0x3) == (CSS_LINE_HEIGHT_DIMENSION as u8)) {
        length = Some(style.line_height);
    }

    if ( (bits&0x3) == (CSS_LINE_HEIGHT_DIMENSION as u8) ) {
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_background_color(style: @mut css_computed_style)
                                    -> (u8,u32) {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_COLOR_INDEX];
    bits = bits & (CSS_BACKGROUND_COLOR_MASK as u8);
    bits = bits >> CSS_BACKGROUND_COLOR_SHIFT; 

    (bits,style.background_color) 
}

pub fn css_computed_z_index(style: @mut css_computed_style)
                            -> (u8,i32) {

    let mut bits : u8 = style.bits[CSS_Z_INDEX_INDEX];
    bits = bits & (CSS_Z_INDEX_MASK as u8);
    bits = bits >> CSS_Z_INDEX_SHIFT; 

    (bits,style.z_index) 
}

pub fn css_computed_margin_top(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_TOP_INDEX];
    bits = bits & (CSS_MARGIN_TOP_MASK as u8);
    bits = bits >> CSS_MARGIN_TOP_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[0]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_margin_right(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_RIGHT_INDEX];
    bits = bits & (CSS_MARGIN_RIGHT_MASK as u8);
    bits = bits >> CSS_MARGIN_RIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[1]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_margin_bottom(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_BOTTOM_INDEX];
    bits = bits & (CSS_MARGIN_BOTTOM_MASK as u8);
    bits = bits >> CSS_MARGIN_BOTTOM_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[2]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_margin_left(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MARGIN_LEFT_INDEX];
    bits = bits & (CSS_MARGIN_LEFT_MASK as u8);
    bits = bits >> CSS_MARGIN_LEFT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        length = Some(style.margin[3]);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_background_attachment(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_ATTACHMENT_INDEX];
    bits = bits & (CSS_BACKGROUND_ATTACHMENT_MASK as u8);
    bits = bits >> CSS_BACKGROUND_ATTACHMENT_SHIFT;      

    bits
}

pub fn css_computed_border_collapse(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_COLLAPSE_INDEX];
    bits = bits & (CSS_BORDER_COLLAPSE_MASK as u8);
    bits = bits >> CSS_BORDER_COLLAPSE_SHIFT;      

    bits
}

pub fn css_computed_caption_side(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_CAPTION_SIDE_INDEX];
    bits = bits & (CSS_CAPTION_SIDE_MASK as u8);
    bits = bits >> CSS_CAPTION_SIDE_SHIFT;      

    bits
}

pub fn css_computed_direction(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_DIRECTION_INDEX];
    bits = bits & (CSS_DIRECTION_MASK as u8);
    bits = bits >> CSS_DIRECTION_SHIFT;      

    bits
}

pub fn css_computed_max_height(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MAX_HEIGHT_INDEX];
    bits = bits & (CSS_MAX_HEIGHT_MASK as u8);
    bits = bits >> CSS_MAX_HEIGHT_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MAX_HEIGHT_SET as u8) ) {
        length = Some(style.max_height);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_max_width(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MAX_WIDTH_INDEX];
    bits = bits & (CSS_MAX_WIDTH_MASK as u8);
    bits = bits >> CSS_MAX_WIDTH_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_MAX_WIDTH_SET as u8) ) {
        length = Some(style.max_width);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_width(style: @mut css_computed_style)
                        -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_WIDTH_INDEX];
    bits = bits & (CSS_WIDTH_MASK as u8);
    bits = bits >> CSS_WIDTH_SHIFT;  
    let mut length : Option<i32> = None;
    let mut unit : Option<css_unit> = None;

    if ( (bits&0x3) == (CSS_WIDTH_SET as u8) ) {
        length = Some(style.width);
        unit = Some(unsafe { cast::transmute((bits >> 2) as int)});
    }

    ((bits&0x3),length,unit)
}

pub fn css_computed_empty_cells(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_EMPTY_CELLS_INDEX];
    bits = bits & (CSS_EMPTY_CELLS_MASK as u8);
    bits = bits >> CSS_EMPTY_CELLS_SHIFT;      

    bits
}

pub fn css_computed_float(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FLOAT_INDEX];
    bits = bits & (CSS_FLOAT_MASK as u8);
    bits = bits >> CSS_FLOAT_SHIFT;   

    if ( css_computed_position(style) == (CSS_POSITION_ABSOLUTE as u8) ||
            css_computed_position(style) == (CSS_POSITION_FIXED as u8) ) {
        return (CSS_FLOAT_NONE as u8);
    }

    bits
}

pub fn css_computed_font_style(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_STYLE_INDEX];
    bits = bits & (CSS_FONT_STYLE_MASK as u8);
    bits = bits >> CSS_FONT_STYLE_SHIFT;   

    bits
}

pub fn css_computed_min_height(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MIN_HEIGHT_INDEX];
    bits = bits & (CSS_MIN_HEIGHT_MASK as u8);
    bits = bits >> CSS_MIN_HEIGHT_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_MIN_HEIGHT_SET as u8) ) {
        length = Some(style.min_height);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_min_width(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_MIN_WIDTH_INDEX];
    bits = bits & (CSS_MIN_WIDTH_MASK as u8);
    bits = bits >> CSS_MIN_WIDTH_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_MIN_WIDTH_SET as u8) ) {
        length = Some(style.min_width);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_background_repeat(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_REPEAT_INDEX];
    bits = bits & (CSS_BACKGROUND_REPEAT_MASK as u8);
    bits = bits >> CSS_BACKGROUND_REPEAT_SHIFT;   

    bits
}

pub fn css_computed_clear(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_CLEAR_INDEX];
    bits = bits & (CSS_CLEAR_MASK as u8);
    bits = bits >> CSS_CLEAR_SHIFT;   

    bits
}

pub fn css_computed_padding_top(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_PADDING_TOP_INDEX];
    bits = bits & (CSS_PADDING_TOP_MASK as u8);
    bits = bits >> CSS_PADDING_TOP_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        length = Some(style.padding[0]);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_padding_right(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_PADDING_RIGHT_INDEX];
    bits = bits & (CSS_PADDING_RIGHT_MASK as u8);
    bits = bits >> CSS_PADDING_RIGHT_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        length = Some(style.padding[1]);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_padding_bottom(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_PADDING_BOTTOM_INDEX];
    bits = bits & (CSS_PADDING_BOTTOM_MASK as u8);
    bits = bits >> CSS_PADDING_BOTTOM_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        length = Some(style.padding[2]);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_padding_left(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_PADDING_LEFT_INDEX];
    bits = bits & (CSS_PADDING_LEFT_MASK as u8);
    bits = bits >> CSS_PADDING_LEFT_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        length = Some(style.padding[3]);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_overflow(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_OVERFLOW_INDEX];
    bits = bits & (CSS_OVERFLOW_MASK as u8);
    bits = bits >> CSS_OVERFLOW_SHIFT;   

    bits
}

pub fn css_computed_position(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_POSITION_INDEX];
    bits = bits & (CSS_POSITION_MASK as u8);
    bits = bits >> CSS_POSITION_SHIFT;   

    bits
}

pub fn css_computed_opacity(style:@mut css_computed_style)
                            -> (u8,Option<i32>) {

    let mut bits : u8 = style.bits[CSS_OPACITY_INDEX];
    bits = bits & (CSS_OPACITY_MASK as u8);
    bits = bits >> CSS_OPACITY_SHIFT;  
    let mut opacity : Option<i32> = None ;

    if ( (bits&0x1) == (CSS_OPACITY_SET as u8) ) {
        opacity = Some(style.opacity);
    }

    ((bits&0x1),opacity)
}

pub fn css_computed_text_transform(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TEXT_TRANSFORM_INDEX];
    bits = bits & (CSS_TEXT_TRANSFORM_MASK as u8);
    bits = bits >> CSS_TEXT_TRANSFORM_SHIFT;   

    bits
}

pub fn css_computed_text_indent(style:@mut css_computed_style)
                            -> (u8,Option<i32>,Option<css_unit>) {

    let mut bits : u8 = style.bits[CSS_TEXT_INDENT_INDEX];
    bits = bits & (CSS_TEXT_INDENT_MASK as u8);
    bits = bits >> CSS_TEXT_INDENT_SHIFT;  
    let mut length : Option<i32> = None ;
    let mut unit : Option<css_unit> = None ;

    if ( (bits&0x1) == (CSS_TEXT_INDENT_SET as u8) ) {
        length = Some(style.text_indent);
        unit = Some(unsafe { cast::transmute((bits >> 1) as int)});
    }

    ((bits&0x1),length,unit)
}

pub fn css_computed_white_space(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_WHITE_SPACE_INDEX];
    bits = bits & (CSS_WHITE_SPACE_MASK as u8);
    bits = bits >> CSS_WHITE_SPACE_SHIFT;   

    bits
}

pub fn css_computed_background_position(
                    style : @mut css_computed_style) 
                    -> rect_result {

    let mut result = 
            rect_result{
                hlength:0,
                vlength:0,
                hunit:CSS_UNIT_PX,
                vunit:CSS_UNIT_PX,
                result:0
            };

    let mut bits = style.bits[CSS_BACKGROUND_POSITION_INDEX];
    bits = bits & (CSS_BACKGROUND_POSITION_MASK as u8);
    bits = bits >> CSS_BACKGROUND_POSITION_SHIFT ;

    if ( bits == (CSS_BACKGROUND_POSITION_SET as u8) ) {

        let mut bits1 = style.bits[CSS_BACKGROUND_POSITION_INDEX1];
        bits1 = bits1 & (CSS_BACKGROUND_POSITION_MASK1 as u8);
        bits1 = bits1 >> CSS_BACKGROUND_POSITION_SHIFT1 ;

        result.hlength = style.background_position[0];
        result.hunit = unsafe { cast::transmute((bits1 >> 4)as int) } ;

        result.vlength = style.background_position[1];
        result.vunit = unsafe { cast::transmute((bits1 & 0xf)as int) } ;
        
        result.result = bits ; 
    }
    result
}

pub fn css_computed_display(style: @mut css_computed_style,
                        root: bool) -> u8 {

    let mut position: u8 ;
    let mut bits = style.bits[CSS_DISPLAY_INDEX];
    bits = bits & (CSS_DISPLAY_MASK as u8);
    bits = bits >> CSS_DISPLAY_SHIFT ;

    position = css_computed_position(style);

    if ( bits == (CSS_DISPLAY_NONE as u8) ) {
        return bits;
    }

    if ( (position == (CSS_POSITION_ABSOLUTE as u8)) || 
            (position == (CSS_POSITION_FIXED as u8))  ||
            (css_computed_float(style) != (CSS_FLOAT_NONE as u8)) ||
            (root == true) ) {

        if ( bits == (CSS_DISPLAY_INLINE_TABLE as u8) ) {
            return (CSS_DISPLAY_TABLE as u8);
        } 
        else if (bits == (CSS_DISPLAY_INLINE as u8) ||
                bits == (CSS_DISPLAY_RUN_IN as u8) ||
                bits == (CSS_DISPLAY_TABLE_ROW_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_COLUMN as u8) ||
                bits == (CSS_DISPLAY_TABLE_COLUMN_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_HEADER_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_FOOTER_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_ROW as u8) ||
                bits == (CSS_DISPLAY_TABLE_CELL as u8) ||
                bits == (CSS_DISPLAY_TABLE_CAPTION as u8) ||
                bits == (CSS_DISPLAY_INLINE_BLOCK as u8)) {
        
            return (CSS_DISPLAY_BLOCK as u8);
        }
    }

    return bits;
}

pub fn css_computed_display_static(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_DISPLAY_INDEX];
    bits = bits & (CSS_DISPLAY_MASK as u8);
    bits = bits >> CSS_DISPLAY_SHIFT;   

    bits
}

pub fn css_computed_font_variant(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_VARIANT_INDEX];
    bits = bits & (CSS_FONT_VARIANT_MASK as u8);
    bits = bits >> CSS_FONT_VARIANT_SHIFT;   

    bits
}

pub fn css_computed_text_decoration(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TEXT_DECORATION_INDEX];
    bits = bits & (CSS_TEXT_DECORATION_MASK as u8);
    bits = bits >> CSS_TEXT_DECORATION_SHIFT;   

    bits
}

pub fn css_computed_font_family(style:@mut css_computed_style)
                                        -> (u8,~[~str]) {

    let mut bits : u8 = style.bits[CSS_FONT_FAMILY_INDEX];
    bits = bits & (CSS_FONT_FAMILY_MASK as u8);
    bits = bits >> CSS_FONT_FAMILY_SHIFT;   

    (bits,copy style.font_family)
}

pub fn css_computed_border_top_style(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_TOP_STYLE_INDEX];
    bits = bits & (CSS_BORDER_TOP_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_STYLE_SHIFT;   

    bits
}

pub fn css_computed_border_right_style(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_RIGHT_STYLE_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_STYLE_SHIFT;   

    bits
}

pub fn css_computed_border_bottom_style(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_BOTTOM_STYLE_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_STYLE_SHIFT;   

    bits
}

pub fn css_computed_border_left_style(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_LEFT_STYLE_INDEX];
    bits = bits & (CSS_BORDER_LEFT_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_STYLE_SHIFT;   

    bits
}

pub fn css_computed_font_weight(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_WEIGHT_INDEX];
    bits = bits & (CSS_FONT_WEIGHT_MASK as u8);
    bits = bits >> CSS_FONT_WEIGHT_SHIFT;   

    bits
}

pub fn css_computed_list_style_type(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_LIST_STYLE_TYPE_INDEX];
    bits = bits & (CSS_LIST_STYLE_TYPE_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_TYPE_SHIFT;   

    bits
}

pub fn css_computed_outline_style(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_OUTLINE_STYLE_INDEX];
    bits = bits & (CSS_OUTLINE_STYLE_MASK as u8);
    bits = bits >> CSS_OUTLINE_STYLE_SHIFT;   

    bits
}

pub fn css_computed_table_layout(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TABLE_LAYOUT_INDEX];
    bits = bits & (CSS_TABLE_LAYOUT_MASK as u8);
    bits = bits >> CSS_TABLE_LAYOUT_SHIFT;   

    bits
}

pub fn css_computed_unicode_bidi(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_UNICODE_BIDI_INDEX];
    bits = bits & (CSS_UNICODE_BIDI_MASK as u8);
    bits = bits >> CSS_UNICODE_BIDI_SHIFT;   

    bits
}

pub fn css_computed_visibility(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_VISIBILITY_INDEX];
    bits = bits & (CSS_VISIBILITY_MASK as u8);
    bits = bits >> CSS_VISIBILITY_SHIFT;   

    bits
}

pub fn css_computed_list_style_position(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_LIST_STYLE_POSITION_INDEX];
    bits = bits & (CSS_LIST_STYLE_POSITION_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_POSITION_SHIFT;   

    bits
}


pub fn css_computed_text_align(style:@mut css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TEXT_ALIGN_INDEX];
    bits = bits & (CSS_TEXT_ALIGN_MASK as u8);
    bits = bits >> CSS_TEXT_ALIGN_SHIFT;   

    bits
}

pub fn css_computed_page_break_after(style:@mut css_computed_style)
                                        -> u8 {

    match  style.page {
        Some(computed_page)=>{
            let mut bits : u8 = computed_page.bits[CSS_PAGE_BREAK_AFTER_INDEX];
            bits = bits & (CSS_PAGE_BREAK_AFTER_MASK as u8);
            bits = bits >> CSS_PAGE_BREAK_AFTER_SHIFT;   
            bits
        },
        None=>{
            (CSS_PAGE_BREAK_AFTER_AUTO as u8)
        }
    }
}

pub fn css_computed_page_break_before(style:@mut css_computed_style)
                                        -> u8 {

    match  style.page {
        Some(computed_page)=>{
            let mut bits : u8 = computed_page.bits[CSS_PAGE_BREAK_BEFORE_INDEX];
            bits = bits & (CSS_PAGE_BREAK_BEFORE_MASK as u8);
            bits = bits >> CSS_PAGE_BREAK_BEFORE_SHIFT;   
            bits
        },
        None=>{
            (CSS_PAGE_BREAK_BEFORE_AUTO as u8)
        }
    }
}

pub fn css_computed_page_break_inside(style:@mut css_computed_style)
                                        -> u8 {

    match  style.page {
        Some(computed_page)=>{
            let mut bits : u8 = computed_page.bits[CSS_PAGE_BREAK_INSIDE_INDEX];
            bits = bits & (CSS_PAGE_BREAK_INSIDE_MASK as u8);
            bits = bits >> CSS_PAGE_BREAK_INSIDE_SHIFT;   
            bits
        },
        None=>{
            (CSS_PAGE_BREAK_INSIDE_AUTO as u8)
        }
    }
}

pub fn css__compute_absolute_values(parent: Option<@mut css_computed_style>,
                                    style: @mut css_computed_style,
                                    compute_font_size:css_fnptr_compute_font_size) 
                                    -> css_result {

    let mut psize = @mut css_hint{
        hint_type:HINT_LENGTH,
        status:0,
        clip:None,
        content:None,
        counter:None,
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
        counter:None,
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
        counter:None,
        length:None,
        position:None,
        color:None,
        fixed:None,
        integer:None,
        string:None,
        strings:None
    };
    let mut error : css_result ;

    match parent {
        Some(parent_style)=>{
            let (a,b,c) = css_computed_font_size(parent_style);
            psize.status = a;
            let length = @mut css_hint_length { 
                value:b.get_or_default(0) , 
                unit:c.get_or_default(CSS_UNIT_PX) 
            };
            psize.length = Some(length);
            error = (*compute_font_size)(Some(psize),Some(size));
        },
        None=>{
            let (a,b,c) = css_computed_font_size(style);
            psize.status = a;
            let length = @mut css_hint_length { 
                value:b.get_or_default(0) , 
                unit:c.get_or_default(CSS_UNIT_PX) 
            };
            psize.length = Some(length);
            error = (*compute_font_size)(None,Some(size));
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
        _=> return CSS_SHOULD_NEVER_OCCUR
    }

    ex_size.status = CSS_FONT_SIZE_DIMENSION as u8;
    let length = @mut css_hint_length { 
        value:css_int_to_fixed(1) , 
        unit:CSS_UNIT_EX 
    };
    ex_size.length = Some(length);
    error = (*compute_font_size)(Some(size),Some(ex_size));
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
    ex_size.length.get().unit = CSS_UNIT_EM ;
    // ...........
    // writing inside functions first 
    // .......
    CSS_OK
}


// pub type css_fnptr_getcompute_absolute_color =  ~extern fn(style: @mut css_computed_style) 
//                                                     -> (u8,u32) ;

// pub fn css_computed_background_color(style: @mut css_computed_style)
//                                     -> (u8,u32) {
// pub fn compute_absolute_color( style: @mut css_computed_style ) -> {

// }
    










