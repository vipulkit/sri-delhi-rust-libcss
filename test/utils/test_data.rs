use std::io;
use std::float;

fn main() {
	let rust_file_content_result = io::read_whole_file_str(&Path("r_output.txt"));

	let mut rust_file_content:~str = ~"";

	match rust_file_content_result {
        Ok(x) => {
            rust_file_content = x ;
        },
        Err(_) => {
            debug!(fmt!("\n Error opening file"));
            assert!(false) ;
        }
    }

    let mut rust_creation_time = 0f;
    let mut rust_append_time = 0f;
    let mut rust_data_done_time = 0f;
    let mut rust_select_time = 0f;

    for rust_file_content.line_iter().advance |line| {
    	if (line.starts_with("#css_stylesheet_create_time")) {
    		debug!("found creation_time");
    		let val = line.slice(28, line.len());
    		debug!("val == %s", val);
    		rust_creation_time = float::from_str(val).unwrap();
    	}

		if (line.starts_with("#css_stylesheet_append_data_time")) {
			debug!("found append_time");
			let val = line.slice(33, line.len());
    		debug!("val == %s", val);
    		rust_append_time = float::from_str(val).unwrap();
    	}

        if (line.starts_with("#css_stylesheet_data_done_time")) {
            debug!("found data_done_time");
            let val = line.slice(31, line.len());
            debug!("val == %s", val);
            rust_data_done_time = float::from_str(val).unwrap();
        }

    	if (line.starts_with("#css_select_style_time")) {
    		debug!("found select_time");
    		let val = line.slice(23, line.len());
    		debug!("val == %s", val);
    		rust_select_time = float::from_str(val).unwrap();
    	}
           	
    }

    let overall_time_rust = rust_creation_time + rust_append_time + rust_data_done_time + rust_select_time ;
    io::println(fmt!("rust:: parsing time ==  %10.3f, selection time == %10.3f,   OVERALL TIME == %10.3f", rust_creation_time + rust_append_time + rust_data_done_time, rust_select_time,overall_time_rust));

	let c_file_content_result = io::read_whole_file_str(&Path("c_output.txt"));

	let mut c_file_content:~str = ~"";

	match c_file_content_result {
        Ok(x) => {
            c_file_content = x ;
        },
        Err(_) => {
            debug!(fmt!("\n Error opening file"));
            assert!(false) ;
        }
    }

    let mut c_creation_time = 0f;
    let mut c_append_time = 0f;
    let mut c_data_done_time = 0f;
    let mut c_select_time = 0f;

    for c_file_content.line_iter().advance |line| {
    	if (line.starts_with("#css_stylesheet_create_time")) {
    		debug!("found creation_time");
    		let val = line.slice(28, line.len());
    		debug!("val == %s", val);
    		c_creation_time = float::from_str(val).unwrap();
    	}

		if (line.starts_with("#css_stylesheet_append_data_time")) {
			debug!("found append_time");
			let val = line.slice(33, line.len());
    		debug!("val == %s", val);
    		c_append_time = float::from_str(val).unwrap();
    	}

        if (line.starts_with("#css_stylesheet_data_done_time")) {
            debug!("found data_done_time");
            let val = line.slice(31, line.len());
            debug!("val == %s", val);
            c_data_done_time = float::from_str(val).unwrap();
        }

    	if (line.starts_with("#css_select_style_time")) {
    		debug!("found select_time");
    		let val = line.slice(23, line.len());
    		debug!("val == %s", val);
    		c_select_time = float::from_str(val).unwrap();
    	}      	
    }

    let overall_time_c =  c_creation_time + c_append_time + c_data_done_time + c_select_time ;
    io::println(fmt!("c::    parsing time ==  %10.3f, selection time == %10.3f,   OVERALL TIME == %10.3f", c_creation_time + c_append_time + c_data_done_time, c_select_time,overall_time_c));

    let parsing_perf:float = (rust_creation_time + rust_append_time + rust_data_done_time) / (c_creation_time + c_append_time + c_data_done_time);
    let selection_perf:float = (rust_select_time / c_select_time);
    let overall_perf : float = (overall_time_rust/overall_time_c) ;

    io::println(fmt!("perf:: parsing:        %10.3fx, selection:       %10.3fx,   OVERALL      ==%10.3fx", parsing_perf, selection_perf,overall_perf));
}
