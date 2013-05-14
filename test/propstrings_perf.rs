extern mod css;
//extern mod css_propstrings_parallel;
extern mod wapcaplet;

extern mod std;

use css::parse::propstrings::*;
use css::parse::propstrings_parallel::*;
use wapcaplet::*;

fn test_sequential() {
	io::println("testing sequential version");
	
    let mut delta = 0f;

	for uint::range(0 , 100) |_| {
		let lwc_instance = lwc();
		let start_time = std::time::precise_time_ns();
		css_propstrings::css_propstrings(lwc_instance);
		let end_time = std::time::precise_time_ns();
		delta += (end_time - start_time) as float;
	}

	//let delta = (end_time - start_time) as float;

	io::println(fmt!("Elapsed Time %f nsec", delta));
    io::println(fmt!("Elapsed Time %.3f usec", delta / 1000f));
    io::println(fmt!("Elapsed Time %.6f msec", delta / 1000000f));
    io::println(fmt!("Elapsed Time %.9f sec", delta / 1000000000f));
}

fn test_parallel() {
	io::println("testing parallel version");
	
    let mut delta = 0f;
	for uint::range(0 , 100) |_| {

		let lwc_instance = lwc();
		let start_time = std::time::precise_time_ns();
		css_propstrings_parallel::css_propstrings_parallel(lwc_instance);
		let end_time = std::time::precise_time_ns();
		delta += (end_time - start_time) as float;
	}

	io::println(fmt!("Elapsed Time %f nsec", delta));
    io::println(fmt!("Elapsed Time %.3f usec", delta / 1000f));
    io::println(fmt!("Elapsed Time %.6f msec", delta / 1000000f));
    io::println(fmt!("Elapsed Time %.9f sec", delta / 1000000000f));

}

fn main() {
	test_sequential();
	test_parallel();
}