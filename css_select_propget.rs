#[link(name = "css_select_propget", vers = "0.1")];
#[crate_type = "lib"];


extern mod css_stylesheet;
extern mod css_bytecode;
extern mod css_enum;
extern mod css_select_computed;
extern mod std ;


use css_enum::* ;
use css_bytecode::*;
use css_stylesheet::*;
use css_select_computed::*;

fn get_letter_spacing(
		style : @mut css_computed_style) 
		-> (u8,Option<u8>,Option<css_unit>) {

    let mut length :Option<u8> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            (CSS_LETTER_SPACING_NORMAL as u8,length,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[0];
            bits = bits & 0xfc;
            bits = bits >> 2;

            if (bits&3) == (CSS_LETTER_SPACING_SET as u8) { 
                length = Some(uncommon_struct.letter_spacing);
                unit = Some(unsafe { cast::transmute((bits >> 2)as u64) }) ;
            }

            ((bits&3),length,unit)
        }
    }
}

fn get_outline_color(
                    style: @mut css_computed_style) 
                    -> (u8,Option<u32>) {

    let mut color : Option<u32> = None;
    match style.uncommon {
        None=>{
            (CSS_OUTLINE_COLOR_INVERT as u8,color)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[0];
            bits = bits & 0x3 ;
            //bits = bits >> 0;

            if (bits&3) == (CSS_OUTLINE_COLOR_COLOR as u8) { 
                color = Some(uncommon_struct.outline_color) ;
            }

            ((bits&3),color)
        }
    }
}


fn get_outline_width(
        style : @mut css_computed_style) 
        -> (u8,Option<u8>,Option<css_unit>) {

    let mut length :Option<u8> = None;
    let mut unit : Option<css_unit>  = None;
    match style.uncommon {
        None=>{
            length = Some(2);
            unit = Some(CSS_UNIT_PX);
            (CSS_OUTLINE_WIDTH_WIDTH as u8,length,unit)
        },
        Some(uncommon_struct)=>{
            let mut bits:u8= uncommon_struct.bits[1];
            bits = bits & 0xfe;
            bits = bits >> 1;

            if (bits&7) == (CSS_OUTLINE_WIDTH_WIDTH as u8) { 
                length = Some(uncommon_struct.outline_width);
                unit = Some(unsafe { cast::transmute((bits >> 3)as u64) }) ;
            }

            ((bits&3),length,unit)
        }
    }
}

pub struct border_spacing_result {
    hlength:u8,
    vlength:u8,
    hunit:css_unit,
    vunit:css_unit,
    result:u8
}

fn get_border_spacing(
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
            let mut bits = uncommon_struct.bits[1];
            bits = bits & 0x1 ;
            //bits = bits >> 0 ;

            if bits == (CSS_BORDER_SPACING_SET as u8) { 
                let mut bits1 = uncommon_struct.bits[2];
                bits1 = bits1 & 0xff ;
                //bits1 = bits1 >> 0 ;

                result.hlength = uncommon_struct.border_spacing[0];
                result.hunit = unsafe { cast::transmute((bits1 >> 4)as u64) } ;

                result.vlength = uncommon_struct.border_spacing[1];
                result.vunit = unsafe { cast::transmute((bits1 & 0xf)as u64) } ;
            }

            result.result = bits ;
            result
        }
    }
}


