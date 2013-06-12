#[link(name ="Dump_computed", vers = "0.1")];
#[crate_type="lib"];

extern mod css;

use css::select::common::*;
use css::select::computed::*;
use css::stylesheet::*;
use css::include::types::*;
use css::include::properties::*;
use css::include::fpmath::*;

fn dump_css_fixed(f: css_fixed , ptr: &mut ~str){
    let abs_f: u32;
    if f < 0 {
        abs_f = -f as u32;
    }
    else {
        abs_f = f as u32;
    }
    let mut unitpart:u32 = abs_f >> 0;
    let mut fracpart:u32 = ((abs_f & 0x3ff)*1000 + 500)/(1 << 10);
    let mut flen: uint = 0;
    let mut tmp: ~[char] = ~[];

    if f < 0 {
        ptr.push_char('-');
    }
    let string_number = &"0123456789";

    loop {
        tmp.push(string_number[unitpart%10] as char);
        unitpart /= 10;
        if unitpart == 0 || tmp.len() >= 20 {
            break;    
        }
    }
    
    while (tmp.len() > 0) {
        ptr.push_char(tmp.pop());
    }
    ptr.push_char('.');
    loop {
        tmp.push(string_number[fracpart%10] as char);
        fracpart /= 10;
        if !(tmp.len() < 20 && fracpart != 0 ) {
            break;    
        }
    }

    while (tmp.len() > 0) {
        ptr.push_char(tmp.pop());
        flen += 1;
    }
    
    
    while flen < 3 {
        ptr.push_char('0');
        flen += 1;
    }
    
}

fn dump_css_number(val: css_fixed , ptr: &mut ~str){
    if css_int_to_fixed((val >> 10) as int) == val {
        str::push_str(ptr , fmt!("%?" , val >> 10));
    }
    else {
        dump_css_fixed(val , ptr);
    }
}

fn dump_css_unit(val: css_fixed , unit: css_unit , ptr: &mut ~str) {
    dump_css_number(val, ptr);

    match unit {
        CSS_UNIT_PX => {
            str::push_str(ptr , &"px");
        },
        CSS_UNIT_EX => {
            str::push_str(ptr , &"ex");
        },
        CSS_UNIT_EM => {
            str::push_str(ptr , &"em");
        },
        CSS_UNIT_IN => {
            str::push_str(ptr , &"in");
        },
        CSS_UNIT_CM => {
            str::push_str(ptr , &"cm");
        },
        CSS_UNIT_MM => {
            str::push_str(ptr , &"mm");
        },
        CSS_UNIT_PT => {
            str::push_str(ptr , &"pt");
        },
        CSS_UNIT_PC => {
            str::push_str(ptr , &"pc");
        },
        CSS_UNIT_PCT => {
            str::push_str(ptr , &"pct");
        },
        CSS_UNIT_DEG => {
            str::push_str(ptr , &"deg");
        },
        CSS_UNIT_GRAD => {
            str::push_str(ptr , &"grad");
        },
        CSS_UNIT_RAD => {
            str::push_str(ptr , &"rad");
        },
        CSS_UNIT_MS => {
            str::push_str(ptr , &"ms");
        },
        CSS_UNIT_S => {
            str::push_str(ptr , &"s");
        },
        CSS_UNIT_HZ => {
            str::push_str(ptr , &"Hz");
        },
        CSS_UNIT_KHZ => {
            str::push_str(ptr , &"kHz");
        }
    }
}


pub fn dump_computed_style(style:@mut css_computed_style, buf:&mut ~str) {
	let ptr = buf;
	let mut val:u8;
	let mut color_option:Option<css_color> = None;
	let mut url: ~str = ~"";
	let mut len1 = 0;
	let mut len2 = 0;
	let mut unit1:css_unit = CSS_UNIT_PX;
	let mut unit2:css_unit = CSS_UNIT_PX;
	let rect:@mut css_computed_clip_rect = @mut css_computed_clip_rect { 
					top:0, right:0, bottom:0, left:0, tunit:CSS_UNIT_PX, runit:CSS_UNIT_PX,
					bunit:CSS_UNIT_PX, lunit:CSS_UNIT_PX, top_auto:true, right_auto:true,
					bottom_auto:true, left_auto:true };
	let mut content:Option<@mut css_computed_content_item> = None;
	let mut counter:Option<@mut css_computed_counter> = None;
	//lwc_string **string_list = NULL;
	let mut zindex:i32 = 0;

	/* background-attachment */
	val = css_computed_background_attachment(style);

	let val_enum: css_background_attachment_e =  unsafe {cast::transmute(val as uint)}; 
	match val_enum {
		CSS_BACKGROUND_ATTACHMENT_INHERIT  =>
			ptr.push_str("background-attachment: inherit\n"),
		CSS_BACKGROUND_ATTACHMENT_FIXED =>
			ptr.push_str("background-attachment: fixed\n"),
		CSS_BACKGROUND_ATTACHMENT_SCROLL =>
			ptr.push_str("background-attachment: scroll\n"),
	}


	/* background-color */
	let (val,color_option) = css_computed_background_color(style);
	let val_enum: css_background_color_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BACKGROUND_COLOR_INHERIT =>
			ptr.push_str("background-color: inherit\n"),
		CSS_BACKGROUND_COLOR_COLOR =>
			ptr.push_str(fmt!("background-color: #%08x\n", color_option.unwrap() as uint)),
		_ =>
			{}
	}

	/* background-image */
	let (val,url) = css_computed_background_image(style);
    	if (val == CSS_BACKGROUND_IMAGE_INHERIT as u8) {
                ptr.push_str("background-image: inherit\n");
	}
	else if (val == CSS_BACKGROUND_IMAGE_IMAGE as u8 && url != ~"") {
		ptr.push_str(fmt!("background-image: url('%s')\n",
				url));
	}
	else if (val == CSS_BACKGROUND_IMAGE_NONE as u8) {
		ptr.push_str("background-image: none\n");
	}
	else {
		//wrote = 0; Do Nothing
	}


	/* background-position */
	let result:rect_result = css_computed_background_position(style);
	if (result.result == CSS_BACKGROUND_POSITION_INHERIT as u8) {
                ptr.push_str("background-position: inherit\n");
    	}
	else if (result.result == CSS_BACKGROUND_POSITION_SET as u8) {
		ptr.push_str("background-position: ");
		
		dump_css_unit(result.hlength, result.hunit, ptr);
		ptr.push_str(" ");
		
		dump_css_unit(result.vlength, result.vunit, ptr);
		ptr.push_str("\n");
		
	}

	/* background-repeat */
	let val = css_computed_background_repeat(style);
	let val_enum: css_background_repeat_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BACKGROUND_REPEAT_INHERIT =>
			ptr.push_str("background-repeat: inherit\n"),
		CSS_BACKGROUND_REPEAT_REPEAT_X =>
			ptr.push_str("background-repeat: repeat-x\n"),
		CSS_BACKGROUND_REPEAT_REPEAT_Y =>
			ptr.push_str("background-repeat: repeat-y\n"),
		CSS_BACKGROUND_REPEAT_REPEAT =>
			ptr.push_str("background-repeat: repeat\n"),
		CSS_BACKGROUND_REPEAT_NO_REPEAT =>
			ptr.push_str("background-repeat: no-repeat\n"),
	}


	/* border-collapse */
	let val = css_computed_border_collapse(style);
	let val_enum: css_border_collapse_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_COLLAPSE_INHERIT =>
			ptr.push_str("border-collapse: inherit\n"),
		CSS_BORDER_COLLAPSE_SEPARATE =>
			ptr.push_str("border-collapse: separate\n"),
		CSS_BORDER_COLLAPSE_COLLAPSE =>
			ptr.push_str("border-collapse: collapse\n"),
	}


	/* border-spacing */
	let result = css_computed_border_spacing(style);
	if (result.result == CSS_BORDER_SPACING_INHERIT as u8) {
        	ptr.push_str("border-spacing: inherit\n");
    	}	 
	else if (result.result == CSS_BORDER_SPACING_SET as u8) {
		ptr.push_str("border-spacing: ");
		dump_css_unit(result.hlength, result.hunit, ptr);
		ptr.push_str(" ");
		dump_css_unit(result.vlength, result.vunit, ptr);
		ptr.push_str("\n");
		
	}

	/* border-top-color */
	let (val,color) = css_computed_border_top_color(style);
	let val_enum: css_border_color_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_COLOR_INHERIT =>
			ptr.push_str("border-top-color: inherit\n"),
		CSS_BORDER_COLOR_CURRENT_COLOR =>
			ptr.push_str("border-top-color: currentColor\n"),
		CSS_BORDER_COLOR_COLOR =>
			ptr.push_str(fmt!("border-top-color: #%08x\n", color as uint)),
	}

	/* border-right-color */
	let (val,color) = css_computed_border_right_color(style);
	let val_enum: css_border_color_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_COLOR_INHERIT =>
			ptr.push_str("border-right-color: inherit\n"),
		CSS_BORDER_COLOR_CURRENT_COLOR =>
			ptr.push_str("border-right-color: currentColor\n"),
		CSS_BORDER_COLOR_COLOR =>
			ptr.push_str(fmt!("border-right-color: #%08x\n", color as uint)),
	}


	/* border-bottom-color */
	let (val,color) = css_computed_border_bottom_color(style);
	let val_enum: css_border_color_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_COLOR_INHERIT =>
			ptr.push_str("border-bottom-color:  inherit\n"),
		CSS_BORDER_COLOR_CURRENT_COLOR =>
			ptr.push_str("border-bottom-color: currentColor\n"),
		CSS_BORDER_COLOR_COLOR =>
			ptr.push_str(fmt!("border-bottom-color: #%08x\n", color as uint)),
	}


	/* border-left-color */
	let (val,color) = css_computed_border_left_color(style);
	let val_enum: css_border_color_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_COLOR_INHERIT =>
			ptr.push_str("border-left-color: inherit\n"),
		CSS_BORDER_COLOR_CURRENT_COLOR =>
			ptr.push_str("border-left-color: currentColor\n"),
		CSS_BORDER_COLOR_COLOR =>
			ptr.push_str(fmt!("border-left-color: #%08x\n", color as uint)),
	}


	/* border-top-style */
	let val = css_computed_border_top_style(style);
	let val_enum: css_border_style_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_STYLE_INHERIT =>
			ptr.push_str("border-top-style: inherit\n"),
		CSS_BORDER_STYLE_NONE =>
			ptr.push_str("border-top-style: none\n"),
		CSS_BORDER_STYLE_HIDDEN =>
			ptr.push_str("border-top-style: hidden\n"),
		CSS_BORDER_STYLE_DOTTED =>
			ptr.push_str("border-top-style: dotted\n"),
		CSS_BORDER_STYLE_DASHED =>
			ptr.push_str("border-top-style: dashed\n"),
		CSS_BORDER_STYLE_SOLID =>
			ptr.push_str("border-top-style: solid\n"),
		CSS_BORDER_STYLE_DOUBLE =>
			ptr.push_str("border-top-style: double\n"),
		CSS_BORDER_STYLE_GROOVE =>
			ptr.push_str("border-top-style: groove\n"),
		CSS_BORDER_STYLE_RIDGE =>
			ptr.push_str("border-top-style: ridge\n"),
		CSS_BORDER_STYLE_INSET =>
			ptr.push_str("border-top-style: inset\n"),
		CSS_BORDER_STYLE_OUTSET =>
			ptr.push_str("border-top-style: outset\n"),
	}


	/* border-right-style */
	let val = css_computed_border_right_style(style);
	let val_enum: css_border_style_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_STYLE_INHERIT =>
			ptr.push_str("border-right-style: inherit\n"),
		CSS_BORDER_STYLE_NONE =>
			ptr.push_str("border-right-style: none\n"),
		CSS_BORDER_STYLE_HIDDEN =>
			ptr.push_str("border-right-style: hidden\n"),
		CSS_BORDER_STYLE_DOTTED =>
			ptr.push_str("border-right-style: dotted\n"),
		CSS_BORDER_STYLE_DASHED =>
			ptr.push_str("border-right-style: dashed\n"),
		CSS_BORDER_STYLE_SOLID =>
			ptr.push_str("border-right-style: solid\n"),
		CSS_BORDER_STYLE_DOUBLE =>
			ptr.push_str("border-right-style: double\n"),
		CSS_BORDER_STYLE_GROOVE =>
			ptr.push_str("border-right-style: groove\n"),
		CSS_BORDER_STYLE_RIDGE =>
			ptr.push_str("border-right-style: ridge\n"),
		CSS_BORDER_STYLE_INSET =>
			ptr.push_str("border-right-style: inset\n"),
		CSS_BORDER_STYLE_OUTSET =>
			ptr.push_str("border-right-style: outset\n"),
	}

}
