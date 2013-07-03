
fn main() {
	let rust_file_content_result = io::read_whole_file_str(&Path(~"r_output.txt"));

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
    let mut rust_select_time = 0f;

    for str::each_line(rust_file_content) |line| {
    	if (str::starts_with(line, ~"#css_stylesheet_create_time")) {
    		debug!("found creation_time");
    		let val = line.slice(28, line.len());
    		debug!("val == %s", val);
    		rust_creation_time = float::from_str(val).unwrap();
    	}

		if (str::starts_with(line, ~"#css_stylesheet_append_data_time")) {
			debug!("found append_time");
			let val = line.slice(33, line.len());
    		debug!("val == %s", val);
    		rust_append_time = float::from_str(val).unwrap();
    	}

    	if (str::starts_with(line, ~"#css_select_style_time")) {
    		debug!("found select_time");
    		let val = line.slice(23, line.len());
    		debug!("val == %s", val);
    		rust_select_time = float::from_str(val).unwrap();
    	}      	
    }


    io::println(fmt!("rust:: creation_time == %.3f, append_time == %.3f, select_time == %.3f", rust_creation_time, rust_append_time, rust_select_time));

	let c_file_content_result = io::read_whole_file_str(&Path(~"c_output.txt"));

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
    let mut c_select_time = 0f;

    for str::each_line(c_file_content) |line| {
    	if (str::starts_with(line, ~"#css_stylesheet_create_time")) {
    		debug!("found creation_time");
    		let val = line.slice(28, line.len());
    		debug!("val == %s", val);
    		c_creation_time = float::from_str(val).unwrap();
    	}

		if (str::starts_with(line, ~"#css_stylesheet_append_data_time")) {
			debug!("found append_time");
			let val = line.slice(33, line.len());
    		debug!("val == %s", val);
    		c_append_time = float::from_str(val).unwrap();
    	}

    	if (str::starts_with(line, ~"#css_select_style_time")) {
    		debug!("found select_time");
    		let val = line.slice(23, line.len());
    		debug!("val == %s", val);
    		c_select_time = float::from_str(val).unwrap();
    	}      	
    }

    io::println(fmt!("c::    creation_time == %.3f, append_time == %.3f, select_time == %.3f", c_creation_time, c_append_time, c_select_time));

    let creation_perf:float = (rust_creation_time / c_creation_time);
    let append_perf:float = (rust_append_time / c_append_time);
    let select_perf:float = (rust_select_time / c_select_time);

    io::println(fmt!("perf:: creation: %.3fx, append: %.3fx, select: %.3fx", creation_perf, append_perf, select_perf));
}