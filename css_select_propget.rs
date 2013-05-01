#[link(name = "css_select_propget", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_enum;
extern mod css_select_computed;
extern mod css_select_const;
extern mod css_fpmath;
extern mod std ;


use css_enum::* ;
use css_select_computed::*;
use css_select_const::*;
use css_fpmath::*;

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

pub struct border_spacing_result {
    hlength:i32,
    vlength:i32,
    hunit:css_unit,
    vunit:css_unit,
    result:u8
}

pub fn css_computed_border_spacing(
                    style : @mut css_computed_style) 
                    -> border_spacing_result {

    let mut result = 
            border_spacing_result{
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

            counter = Some(uncommon_struct.counter_increment);

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

            counter = Some(uncommon_struct.counter_reset);

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

            (bits,Some(uncommon_struct.content))
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





