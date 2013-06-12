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

	/* border-bottom-style */
	let val = css_computed_border_bottom_style(style);
	let val_enum: css_border_style_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_STYLE_INHERIT =>
			ptr.push_str("border-bottom-style: inherit\n"),
		CSS_BORDER_STYLE_NONE =>
			ptr.push_str("border-bottom-style: none\n"),
		CSS_BORDER_STYLE_HIDDEN =>
			ptr.push_str("border-bottom-style: hidden\n"),
		CSS_BORDER_STYLE_DOTTED =>
			ptr.push_str("border-bottom-style: dotted\n"),
		CSS_BORDER_STYLE_DASHED =>
			ptr.push_str("border-bottom-style: dashed\n"),
		CSS_BORDER_STYLE_SOLID =>
			ptr.push_str("border-bottom-style: solid\n"),
		CSS_BORDER_STYLE_DOUBLE =>
			ptr.push_str("border-bottom-style: double\n"),
		CSS_BORDER_STYLE_GROOVE =>
			ptr.push_str("border-bottom-style: groove\n"),
		CSS_BORDER_STYLE_RIDGE =>
			ptr.push_str("border-bottom-style: ridge\n"),
		CSS_BORDER_STYLE_INSET =>
			ptr.push_str("border-bottom-style: inset\n"),
		CSS_BORDER_STYLE_OUTSET =>
			ptr.push_str("border-bottom-style: outset\n"),
	}


	/* border-left-style */
	let val = css_computed_border_left_style(style);
	let val_enum: css_border_style_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_STYLE_INHERIT =>
			ptr.push_str("border-left-style: inherit\n"),
		CSS_BORDER_STYLE_NONE =>
			ptr.push_str("border-left-style: none\n"),
		CSS_BORDER_STYLE_HIDDEN =>
			ptr.push_str("border-left-style: hidden\n"),
		CSS_BORDER_STYLE_DOTTED =>
			ptr.push_str("border-left-style: dotted\n"),
		CSS_BORDER_STYLE_DASHED =>
			ptr.push_str("border-left-style: dashed\n"),
		CSS_BORDER_STYLE_SOLID =>
			ptr.push_str("border-left-style: solid\n"),
		CSS_BORDER_STYLE_DOUBLE =>
			ptr.push_str("border-left-style: double\n"),
		CSS_BORDER_STYLE_GROOVE =>
			ptr.push_str("border-left-style: groove\n"),
		CSS_BORDER_STYLE_RIDGE =>
			ptr.push_str("border-left-style: ridge\n"),
		CSS_BORDER_STYLE_INSET =>
			ptr.push_str("border-left-style: inset\n"),
		CSS_BORDER_STYLE_OUTSET =>
			ptr.push_str("border-left-style: outset\n"),
	}


	/* border-top-width */
	let (val,len1,unit1) = css_computed_border_top_width(style);
	let val_enum: css_border_width_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_WIDTH_INHERIT =>
			ptr.push_str("border-top-width: inherit\n"),
		CSS_BORDER_WIDTH_THIN =>
			ptr.push_str("border-top-width: thin\n"),
		CSS_BORDER_WIDTH_MEDIUM =>
			ptr.push_str("border-top-width: medium\n"),
		CSS_BORDER_WIDTH_THICK =>
			ptr.push_str("border-top-width: thick\n"),
		CSS_BORDER_WIDTH_WIDTH => {
			ptr.push_str("border-top-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}

	
	/* border-right-width */
	let (val, len1, unit1) = css_computed_border_right_width(style);
	let val_enum: css_border_width_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_WIDTH_INHERIT =>
			ptr.push_str("border-right-width: inherit\n"),
		CSS_BORDER_WIDTH_THIN =>
			ptr.push_str("border-right-width: thin\n"),
		CSS_BORDER_WIDTH_MEDIUM =>
			ptr.push_str("border-right-width: medium\n"),
		CSS_BORDER_WIDTH_THICK =>
			ptr.push_str("border-right-width: thick\n"),
		CSS_BORDER_WIDTH_WIDTH => {
			ptr.push_str("border-right-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* border-bottom-width */
	let (val, len1, unit1) = css_computed_border_bottom_width(style);
	let val_enum: css_border_width_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_WIDTH_INHERIT =>
			ptr.push_str("border-bottom-width: inherit\n"),
		CSS_BORDER_WIDTH_THIN =>
			ptr.push_str("border-bottom-width: thin\n"),
		CSS_BORDER_WIDTH_MEDIUM =>
			ptr.push_str("border-bottom-width: medium\n"),
		CSS_BORDER_WIDTH_THICK =>
			ptr.push_str("border-bottom-width: thick\n"),
		CSS_BORDER_WIDTH_WIDTH => {
			ptr.push_str("border-bottom-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* border-left-width */
	let (val, len1, unit1) = css_computed_border_left_width(style);
	let val_enum: css_border_width_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BORDER_WIDTH_INHERIT =>
			ptr.push_str("border-left-width: inherit\n"),
		CSS_BORDER_WIDTH_THIN =>
			ptr.push_str("border-left-width: thin\n"),
		CSS_BORDER_WIDTH_MEDIUM =>
			ptr.push_str("border-left-width: medium\n"),
		CSS_BORDER_WIDTH_THICK =>
			ptr.push_str("border-left-width: thick\n"),
		CSS_BORDER_WIDTH_WIDTH => {
			ptr.push_str("border-left-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}

	/* bottom */
	let (val, len1, unit1) = css_computed_bottom(style);
	let val_enum: css_bottom_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_BOTTOM_INHERIT =>
			ptr.push_str("bottom: inherit\n"),
		CSS_BOTTOM_AUTO =>
			ptr.push_str("bottom: auto\n"),
		CSS_BOTTOM_SET => {
			ptr.push_str("bottom: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		}
	}

	/* caption-side */
	let val = css_computed_caption_side(style);
	let val_enum: css_caption_side_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_CAPTION_SIDE_INHERIT =>
			ptr.push_str("caption-side: inherit\n"),
		CSS_CAPTION_SIDE_TOP =>
			ptr.push_str("caption-side: top\n"),
		CSS_CAPTION_SIDE_BOTTOM =>
			ptr.push_str("caption-side: bottom\n"),
	}

	/* clear */
	let val = css_computed_clear(style);
	let val_enum: css_clear_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_CLEAR_INHERIT =>
			ptr.push_str("clear: inherit\n"),
		CSS_CLEAR_NONE =>
			ptr.push_str("clear: none\n"),
		CSS_CLEAR_LEFT =>
			ptr.push_str("clear: left\n"),
		CSS_CLEAR_RIGHT =>
			ptr.push_str("clear: right\n"),
		CSS_CLEAR_BOTH =>
			ptr.push_str("clear: both\n"),
	}


	/* clip */
	let (val,rect_option) = css_computed_clip(style);
	let rect = rect_option.unwrap();
	let val_enum: css_clip_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_CLIP_INHERIT =>
			ptr.push_str("clip: inherit\n"),
		CSS_CLIP_AUTO =>
			ptr.push_str("clip: auto\n"),
		CSS_CLIP_RECT => {
			ptr.push_str("clip: rect( ");
			
			if (rect.top_auto) {
				ptr.push_str("auto");
			}	
			else {
				dump_css_unit(rect.top, rect.tunit, ptr);
			}			
			ptr.push_str(", ");
			
			if (rect.right_auto) {
				ptr.push_str("auto");
			}
			else {
				dump_css_unit(rect.right, rect.runit, ptr);
			}			
			ptr.push_str(", ");
			
			if (rect.bottom_auto) {
				ptr.push_str("auto");
			}	
			else {
				dump_css_unit(rect.bottom, rect.bunit, ptr);
			}			
			ptr.push_str(", ");
			
			if (rect.left_auto) {
				ptr.push_str("auto");
			}	
			else {
				dump_css_unit(rect.left, rect.lunit, ptr);
			}			
			ptr.push_str(")\n")
		},	
	}


	/* color */
	let (val,color) = css_computed_color(style);
        if (val == CSS_COLOR_INHERIT as u8) {
                ptr.push_str("color: inherit\n");
	} else if (val == CSS_COLOR_COLOR as u8) {
		ptr.push_str(fmt!("color: #%08x\n", color.unwrap() as uint));
	}

	/* content */
	let (val,content) = css_computed_content(style);
	let val_enum: css_content_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_CONTENT_INHERIT =>
			ptr.push_str("content: inherit\n"),
		CSS_CONTENT_NONE =>
			ptr.push_str("content: none\n"),
		CSS_CONTENT_NORMAL =>
			ptr.push_str("content: normal\n"),
		CSS_CONTENT_SET => {
			ptr.push_str("content:");
			let mut content_index = 0;

			while (content[content_index].item_type as uint != CSS_COMPUTED_CONTENT_NONE as uint) {
				ptr.push_str(" ");

			match (content[content_index].item_type) {
				CSS_COMPUTED_CONTENT_STRING =>
					ptr.push_str( fmt!(
						"\"%s\"",
						unsafe{copy *content[content_index].data.get_ref()})) ,
				CSS_COMPUTED_CONTENT_URI =>
					ptr.push_str( fmt!(
						"uri(\"%s\")",
						unsafe{copy *content[content_index].data.get_ref()})),
				CSS_COMPUTED_CONTENT_COUNTER =>
					ptr.push_str( fmt!(
						"counter(%s)",
						unsafe{copy  content[content_index].counters_data.get_ref().name})),
				CSS_COMPUTED_CONTENT_COUNTERS =>
					ptr.push_str( fmt!(
						"counters(%s, \"%s\")",
						unsafe{copy content[content_index].counters_data.get_ref().name},
						unsafe{copy *content[content_index].counters_data.get_ref().sep.get_ref()})),
				CSS_COMPUTED_CONTENT_ATTR =>
					ptr.push_str( fmt!(
						"attr(%s)",
						unsafe{copy *content[content_index].data.get_ref()})),
				CSS_COMPUTED_CONTENT_OPEN_QUOTE =>
					ptr.push_str(
						"open-quote"),
				CSS_COMPUTED_CONTENT_CLOSE_QUOTE =>
					ptr.push_str(
						"close-quote"),
				CSS_COMPUTED_CONTENT_NO_OPEN_QUOTE =>
					ptr.push_str(
						"no-open-quote"),
				CSS_COMPUTED_CONTENT_NO_CLOSE_QUOTE =>
					ptr.push_str(
						"no-close-quote"),
				_ => {}
			}

				content_index+=1;
			}

			ptr.push_str("\n")
		}
	}

	/* counter-increment */
	let (val,counter) = css_computed_counter_increment(style);
	let mut counter_index = 0;
	if (val == CSS_COUNTER_INCREMENT_INHERIT as u8) {
			ptr.push_str("counter-increment: inherit\n");
	}
	else if (counter.len() == 0) {
		ptr.push_str("counter-increment: none\n");
	} 
	else {
		ptr.push_str("counter-increment:");
	
		while (counter[counter_index].name != ~"") {
			ptr.push_str(fmt!(" %s ",
				copy counter[counter_index].name));
			
			dump_css_fixed(counter[counter_index].value, ptr);
			
			counter_index+=1;
		}

		ptr.push_str("\n");
	}


	/* cursor */
	let (val,string_list_option) = css_computed_cursor(style);
	ptr.push_str("cursor:");
	let mut string_list_index = 0;

	if (!string_list_option.is_none()) {
		let string_list = string_list_option.unwrap();

		while (string_list_index < string_list.len()) {
			ptr.push_str(fmt!(" url('%s')",
					string_list[string_list_index]));
			string_list_index+=1;
		}
	}

	let val_enum: css_cursor_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_CURSOR_INHERIT =>
			ptr.push_str(" inherit\n"),
		CSS_CURSOR_AUTO =>
			ptr.push_str(" auto\n"),
		CSS_CURSOR_CROSSHAIR =>
			ptr.push_str(" crosshair\n"),
		CSS_CURSOR_DEFAULT =>
			ptr.push_str(" default\n"),
		CSS_CURSOR_POINTER =>
			ptr.push_str(" pointer\n"),
		CSS_CURSOR_MOVE =>
			ptr.push_str(" move\n"),
		CSS_CURSOR_E_RESIZE =>
			ptr.push_str(" e-resize\n"),
		CSS_CURSOR_NE_RESIZE =>
			ptr.push_str(" ne-resize\n"),
		CSS_CURSOR_NW_RESIZE =>
			ptr.push_str(" nw-resize\n"),
		CSS_CURSOR_N_RESIZE =>
			ptr.push_str(" n-resize\n"),
		CSS_CURSOR_SE_RESIZE =>
			ptr.push_str(" se-resize\n"),
		CSS_CURSOR_SW_RESIZE =>
			ptr.push_str(" sw-resize\n"),
		CSS_CURSOR_S_RESIZE =>
			ptr.push_str(" s-resize\n"),
		CSS_CURSOR_W_RESIZE =>
			ptr.push_str(" w-resize\n"),
		CSS_CURSOR_TEXT =>
			ptr.push_str(" text\n"),
		CSS_CURSOR_WAIT =>
			ptr.push_str(" wait\n"),
		CSS_CURSOR_HELP =>
			ptr.push_str(" help\n"),
		CSS_CURSOR_PROGRESS =>
			ptr.push_str(" progress\n"),
	}

	/* direction */
	let val = css_computed_direction(style);
	let val_enum: css_direction_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_DIRECTION_INHERIT =>
			ptr.push_str("direction: inherit\n"),
		CSS_DIRECTION_LTR =>
			ptr.push_str("direction: ltr\n"),
		CSS_DIRECTION_RTL =>
			ptr.push_str("direction: rtl\n"),
	}


	/* display */
	let val = css_computed_display_static(style);
	let val_enum: css_display_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_DISPLAY_INHERIT =>
			ptr.push_str("display: inherit\n"),
		CSS_DISPLAY_INLINE =>
			ptr.push_str("display: inline\n"),
		CSS_DISPLAY_BLOCK =>
			ptr.push_str("display: block\n"),
		CSS_DISPLAY_LIST_ITEM =>
			ptr.push_str("display: list-item\n"),
		CSS_DISPLAY_RUN_IN =>
			ptr.push_str("display: run-in\n"),
		CSS_DISPLAY_INLINE_BLOCK =>
			ptr.push_str("display: inline-block\n"),
		CSS_DISPLAY_TABLE =>
			ptr.push_str("display: table\n"),
		CSS_DISPLAY_INLINE_TABLE =>
			ptr.push_str("display: inline-table\n"),
		CSS_DISPLAY_TABLE_ROW_GROUP =>
			ptr.push_str("display: table-row-group\n"),
		CSS_DISPLAY_TABLE_HEADER_GROUP =>
			ptr.push_str("display: table-header-group\n"),
		CSS_DISPLAY_TABLE_FOOTER_GROUP =>
			ptr.push_str("display: table-footer-group\n"),
		CSS_DISPLAY_TABLE_ROW =>
			ptr.push_str("display: table-row\n"),
		CSS_DISPLAY_TABLE_COLUMN_GROUP =>
			ptr.push_str("display: table-column-group\n"),
		CSS_DISPLAY_TABLE_COLUMN =>
			ptr.push_str("display: table-column\n"),
		CSS_DISPLAY_TABLE_CELL =>
			ptr.push_str("display: table-cell\n"),
		CSS_DISPLAY_TABLE_CAPTION =>
			ptr.push_str("display: table-caption\n"),
		CSS_DISPLAY_NONE =>
			ptr.push_str("display: none\n"),
	}


	/* empty-cells */
	let val = css_computed_empty_cells(style);
	let val_enum: css_empty_cells_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_EMPTY_CELLS_INHERIT =>
			ptr.push_str("empty-cells: inherit\n"),
		CSS_EMPTY_CELLS_SHOW =>
			ptr.push_str("empty-cells: show\n"),
		CSS_EMPTY_CELLS_HIDE =>
			ptr.push_str("empty-cells: hide\n"),
	}

	/* float */
	let val = css_computed_float(style);
	let val_enum: css_float_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_FLOAT_INHERIT =>
			ptr.push_str("float: inherit\n"),
		CSS_FLOAT_LEFT =>
			ptr.push_str("float: left\n"),
		CSS_FLOAT_RIGHT =>
			ptr.push_str("float: right\n"),
		CSS_FLOAT_NONE =>
			ptr.push_str("float: none\n")
	}

	/* font-family */
	let (val,string_list) = css_computed_font_family(style);
	let mut string_list_index = 0;

	if (val == CSS_FONT_FAMILY_INHERIT as u8) {
                ptr.push_str("font-family: inherit\n");
                
        } else {
		ptr.push_str("font-family:");
		
		if (string_list.len() != 0) {
			while (string_list_index <  string_list.len()) {
				ptr.push_str(fmt!(" \"%s\"",
					string_list[string_list_index]));

				string_list_index+=1;
			}
		}

		let val_enum: css_font_family_e =  unsafe {cast::transmute(val as uint)}; 
		match (val_enum) {
			CSS_FONT_FAMILY_SERIF =>
				ptr.push_str(" serif\n"),
			CSS_FONT_FAMILY_SANS_SERIF =>
				ptr.push_str(" sans-serif\n"),
			CSS_FONT_FAMILY_CURSIVE =>
				ptr.push_str(" cursive\n"),
			CSS_FONT_FAMILY_FANTASY =>
				ptr.push_str(" fantasy\n"),
			CSS_FONT_FAMILY_MONOSPACE =>
				ptr.push_str(" monospace\n"),
			_ =>
				{}
		}
		
	}

	/* font-size */
	let (val,len1,unit1) = css_computed_font_size(style);
	let val_enum: css_font_size_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_FONT_SIZE_INHERIT =>
			ptr.push_str("font-size: inherit\n"),
		CSS_FONT_SIZE_XX_SMALL =>
			ptr.push_str("font-size: xx-small\n"),
		CSS_FONT_SIZE_X_SMALL =>
			ptr.push_str("font-size: x-small\n"),
		CSS_FONT_SIZE_SMALL =>
			ptr.push_str("font-size: small\n"),
		CSS_FONT_SIZE_MEDIUM =>
			ptr.push_str("font-size: medium\n"),
		CSS_FONT_SIZE_LARGE =>
			ptr.push_str("font-size: large\n"),
		CSS_FONT_SIZE_X_LARGE =>
			ptr.push_str("font-size: x-large\n"),
		CSS_FONT_SIZE_XX_LARGE =>
			ptr.push_str("font-size: xx-large\n"),
		CSS_FONT_SIZE_LARGER =>
			ptr.push_str("font-size: larger\n"),
		CSS_FONT_SIZE_SMALLER =>
			ptr.push_str("font-size: smaller\n"),
		CSS_FONT_SIZE_DIMENSION => {
			ptr.push_str("font-size: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		}	
	}

	/* font-style */
	let val = css_computed_font_style(style);
	let val_enum: css_font_style_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_FONT_STYLE_INHERIT =>
			ptr.push_str("font-style: inherit\n"),
		CSS_FONT_STYLE_NORMAL =>
			ptr.push_str("font-style: normal\n"),
		CSS_FONT_STYLE_ITALIC =>
			ptr.push_str("font-style: italic\n"),
		CSS_FONT_STYLE_OBLIQUE =>
			ptr.push_str("font-style: oblique\n"),
	}


	/* font-variant */
	let val = css_computed_font_variant(style);
	let val_enum: css_font_variant_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_FONT_VARIANT_INHERIT =>
			ptr.push_str("font-variant: inherit\n"),
		CSS_FONT_VARIANT_NORMAL =>
			ptr.push_str("font-variant: normal\n"),
		CSS_FONT_VARIANT_SMALL_CAPS =>
			ptr.push_str("font-variant: small-caps\n"),
	}		

	/* font-weight */
	let val = css_computed_font_weight(style);
	let val_enum: css_font_weight_e =  unsafe {cast::transmute(val as uint)}; 
	match (val_enum) {
		CSS_FONT_WEIGHT_INHERIT =>
			ptr.push_str("font-weight: inherit\n"),
		CSS_FONT_WEIGHT_NORMAL =>
			ptr.push_str("font-weight: normal\n"),
		CSS_FONT_WEIGHT_BOLD =>
			ptr.push_str("font-weight: bold\n"),
		CSS_FONT_WEIGHT_BOLDER =>
			ptr.push_str("font-weight: bolder\n"),
		CSS_FONT_WEIGHT_LIGHTER =>
			ptr.push_str("font-weight: lighter\n"),
		CSS_FONT_WEIGHT_100 =>
			ptr.push_str("font-weight: 100\n"),
		CSS_FONT_WEIGHT_200 =>
			ptr.push_str("font-weight: 200\n"),
		CSS_FONT_WEIGHT_300 =>
			ptr.push_str("font-weight: 300\n"),
		CSS_FONT_WEIGHT_400 =>
			ptr.push_str("font-weight: 400\n"),
		CSS_FONT_WEIGHT_500 =>
			ptr.push_str("font-weight: 500\n"),
		CSS_FONT_WEIGHT_600 =>
			ptr.push_str("font-weight: 600\n"),
		CSS_FONT_WEIGHT_700 =>
			ptr.push_str("font-weight: 700\n"),
		CSS_FONT_WEIGHT_800 =>
			ptr.push_str("font-weight: 800\n"),
		CSS_FONT_WEIGHT_900 =>
			ptr.push_str("font-weight: 900\n")
	}

	/* height */
	let (val,len1,unit1) = css_computed_height(style);
	let val_enum: css_height_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_HEIGHT_INHERIT =>
			ptr.push_str("height: inherit\n"),
		CSS_HEIGHT_AUTO =>
			ptr.push_str("height: auto\n"),
		CSS_HEIGHT_SET => {
			ptr.push_str("height: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}

	/* left */
	let (val,len1,unit1) = css_computed_left(style);
	let val_enum: css_left_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_LEFT_INHERIT =>
			ptr.push_str("left: inherit\n"),
		CSS_LEFT_AUTO =>
			ptr.push_str("left: auto\n"),
		CSS_LEFT_SET => {
			ptr.push_str("left: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},	
	}
	
	/* letter-spacing */
	let (val,len1,unit1) = css_computed_letter_spacing(style);
	let val_enum: css_letter_spacing_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_LETTER_SPACING_INHERIT =>
			ptr.push_str("letter-spacing: inherit\n"),
		CSS_LETTER_SPACING_NORMAL =>
			ptr.push_str("letter-spacing: normal\n"),
		CSS_LETTER_SPACING_SET => {
			ptr.push_str("letter-spacing: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		}	
	}


	/* line-height */
	let (val,len1,unit1) = css_computed_line_height(style);
	let val_enum: css_line_height_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_LINE_HEIGHT_INHERIT =>
			ptr.push_str("line-height: inherit\n"),
		CSS_LINE_HEIGHT_NORMAL =>
			ptr.push_str("line-height: normal\n"),
		CSS_LINE_HEIGHT_NUMBER => {
			ptr.push_str("line-height: ");
			dump_css_fixed(len1.unwrap(), ptr);
			ptr.push_str("\n")
		},
		CSS_LINE_HEIGHT_DIMENSION => {
			ptr.push_str("line-height => ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}

	/* list-style-image */
	let (val,url) = css_computed_list_style_image(style);
    	
	if (val == CSS_LIST_STYLE_IMAGE_INHERIT as u8) {
        	ptr.push_str("list-style-image: inherit\n");
	} else if (url != ~"") {
		ptr.push_str(fmt!("list-style-image => url('%s')\n",url));
	} else if (val == CSS_LIST_STYLE_IMAGE_NONE as u8) {
		ptr.push_str("list-style-image: none\n");
	} 

	/* list-style-position */
	let val = css_computed_list_style_position(style);
	let val_enum: css_list_style_position_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_LIST_STYLE_POSITION_INHERIT =>
			ptr.push_str("list-style-position: inherit\n"),
		CSS_LIST_STYLE_POSITION_INSIDE =>
			ptr.push_str("list-style-position: inside\n"),
		CSS_LIST_STYLE_POSITION_OUTSIDE =>
			ptr.push_str("list-style-position: outside\n"),
	}


	/* list-style-type */
	let val = css_computed_list_style_type(style);
	let val_enum: css_list_style_type_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_LIST_STYLE_TYPE_INHERIT =>
			ptr.push_str("list-style-type: inherit\n"),
		CSS_LIST_STYLE_TYPE_DISC =>
			ptr.push_str("list-style-type: disc\n"),
		CSS_LIST_STYLE_TYPE_CIRCLE =>
			ptr.push_str("list-style-type: circle\n"),
		CSS_LIST_STYLE_TYPE_SQUARE =>
			ptr.push_str("list-style-type: square\n"),
		CSS_LIST_STYLE_TYPE_DECIMAL =>
			ptr.push_str("list-style-type: decimal\n"),
		CSS_LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO =>
			ptr.push_str(
					"list-style-type: decimal-leading-zero\n"),
		CSS_LIST_STYLE_TYPE_LOWER_ROMAN =>
			ptr.push_str("list-style-type: lower-roman\n"),
		CSS_LIST_STYLE_TYPE_UPPER_ROMAN =>
			ptr.push_str("list-style-type: upper-roman\n"),
		CSS_LIST_STYLE_TYPE_LOWER_GREEK =>
			ptr.push_str("list-style-type: lower-greek\n"),
		CSS_LIST_STYLE_TYPE_LOWER_LATIN =>
			ptr.push_str("list-style-type: lower-latin\n"),
		CSS_LIST_STYLE_TYPE_UPPER_LATIN =>
			ptr.push_str("list-style-type: upper-latin\n"),
		CSS_LIST_STYLE_TYPE_ARMENIAN =>
			ptr.push_str("list-style-type: armenian\n"),
		CSS_LIST_STYLE_TYPE_GEORGIAN =>
			ptr.push_str("list-style-type: georgian\n"),
		CSS_LIST_STYLE_TYPE_LOWER_ALPHA =>
			ptr.push_str("list-style-type: lower-alpha\n"),
		CSS_LIST_STYLE_TYPE_UPPER_ALPHA =>
			ptr.push_str("list-style-type: upper-alpha\n"),
		CSS_LIST_STYLE_TYPE_NONE =>
			ptr.push_str("list-style-type: none\n"),
	}

	/* margin-top */
	let (val,len1,unit1) = css_computed_margin_top(style);
	let val_enum: css_margin_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_MARGIN_INHERIT =>
			ptr.push_str("margin-top: inherit\n"),
		CSS_MARGIN_AUTO =>
			ptr.push_str("margin-top: auto\n"),
		CSS_MARGIN_SET => {
			ptr.push_str("margin-top: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},	
	}

	/* margin-right */
	let (val,len1,unit1) = css_computed_margin_right(style);
	let val_enum: css_margin_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_MARGIN_INHERIT =>
			ptr.push_str("margin-right: inherit\n"),
		CSS_MARGIN_AUTO =>
			ptr.push_str("margin-right: auto\n"),
		CSS_MARGIN_SET => {
			ptr.push_str("margin-right: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},	
	}

	/* margin-bottom */
	let (val,len1,unit1) = css_computed_margin_bottom(style);
	let val_enum: css_margin_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_MARGIN_INHERIT =>
			ptr.push_str("margin-bottom: inherit\n"),
		CSS_MARGIN_AUTO =>
			ptr.push_str("margin-bottom: auto\n"),
		CSS_MARGIN_SET => {
			ptr.push_str("margin-bottom: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		}	
	}

	/* margin-left */
	let (val,len1,unit1) = css_computed_margin_left(style);
	let val_enum: css_margin_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_MARGIN_INHERIT =>
			ptr.push_str("margin-left: inherit\n"),
		CSS_MARGIN_AUTO =>
			ptr.push_str("margin-left: auto\n"),
		CSS_MARGIN_SET => {
			ptr.push_str("margin-left: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}

	
	/* max-width */
	let (val, len1, unit1) = css_computed_max_width(style);
	let val_enum: css_max_width_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_MAX_WIDTH_INHERIT =>
			ptr.push_str("max-width: inherit\n"),
		CSS_MAX_WIDTH_NONE =>
			ptr.push_str("max-width: none\n"),
		CSS_MAX_WIDTH_SET => {
			ptr.push_str("max-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* min-height */
	let (val, len1, unit1) = css_computed_min_height(style);
	let val_enum: css_min_height_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_MIN_HEIGHT_INHERIT =>
			ptr.push_str("min-height: inherit\n"),
		CSS_MIN_HEIGHT_SET => {
			ptr.push_str("min-height: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* min-width */
	let (val, len1, unit1) = css_computed_min_width(style);
	let val_enum: css_min_width_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
        CSS_MIN_WIDTH_INHERIT =>
            ptr.push_str("min-width: inherit\n"),
		CSS_MIN_WIDTH_SET => {
			ptr.push_str("min-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* opacity */
	let (val, len1) = css_computed_opacity(style);
	let val_enum: css_opacity_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
        	CSS_OPACITY_INHERIT => 
			ptr.push_str("opacity: inherit\n"),
		CSS_OPACITY_SET => {
			ptr.push_str("opacity: ");
			dump_css_fixed(len1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* outline-color */
	let (val,color) = css_computed_outline_color(style);
	let val_enum: css_outline_color_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_OUTLINE_COLOR_INHERIT =>
			ptr.push_str("outline-color: inherit\n"),
		CSS_OUTLINE_COLOR_INVERT =>
			ptr.push_str("outline-color: invert\n"),
		CSS_OUTLINE_COLOR_COLOR =>
			ptr.push_str(fmt!("outline-color: #%08x\n", color.unwrap() as uint)),
		_ =>
			{}
	}


	/* outline-style */
	let val = css_computed_outline_style(style);
	let val_enum: css_outline_style_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_OUTLINE_STYLE_INHERIT =>
			ptr.push_str("outline-style: inherit\n"),
		CSS_OUTLINE_STYLE_NONE =>
			ptr.push_str("outline-style: none\n"),
		CSS_OUTLINE_STYLE_DOTTED =>
			ptr.push_str("outline-style: dotted\n"),
		CSS_OUTLINE_STYLE_DASHED =>
			ptr.push_str("outline-style: dashed\n"),
		CSS_OUTLINE_STYLE_SOLID =>
			ptr.push_str("outline-style: solid\n"),
		CSS_OUTLINE_STYLE_DOUBLE =>
			ptr.push_str("outline-style: double\n"),
		CSS_OUTLINE_STYLE_GROOVE =>
			ptr.push_str("outline-style: groove\n"),
		CSS_OUTLINE_STYLE_RIDGE =>
			ptr.push_str("outline-style: ridge\n"),
		CSS_OUTLINE_STYLE_INSET =>
			ptr.push_str("outline-style: inset\n"),
		CSS_OUTLINE_STYLE_OUTSET =>
			ptr.push_str("outline-style: outset\n"),
	}


	/* outline-width */
	let (val, len1, unit1) = css_computed_outline_width(style);
	let val_enum: css_outline_width_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_OUTLINE_WIDTH_INHERIT =>
			ptr.push_str("outline-width: inherit\n"),
		CSS_OUTLINE_WIDTH_THIN =>
			ptr.push_str("outline-width: thin\n"),
		CSS_OUTLINE_WIDTH_MEDIUM =>
			ptr.push_str("outline-width: medium\n"),
		CSS_OUTLINE_WIDTH_THICK =>
			ptr.push_str("outline-width: thick\n"),
		CSS_OUTLINE_WIDTH_WIDTH => {
			ptr.push_str("outline-width: ");
			dump_css_unit(len1.unwrap(), unit1.unwrap(), ptr);
			ptr.push_str("\n")
		},
	}


	/* overflow */
	let val = css_computed_overflow(style);
	let val_enum: css_overflow_e =  unsafe {cast::transmute(val as uint)}; 

	match (val_enum) {
		CSS_OVERFLOW_INHERIT =>
			ptr.push_str("overflow: inherit\n"),
		CSS_OVERFLOW_VISIBLE =>
			ptr.push_str("overflow: visible\n"),
		CSS_OVERFLOW_HIDDEN =>
			ptr.push_str("overflow: hidden\n"),
		CSS_OVERFLOW_SCROLL =>
			ptr.push_str("overflow: scroll\n"),
		CSS_OVERFLOW_AUTO =>
			ptr.push_str("overflow: auto\n"),
	}

}
