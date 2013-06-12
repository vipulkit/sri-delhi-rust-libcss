extern mod wapcaplet;
extern mod std;

use wapcaplet::*;
use std::arc;

fn main () {
	let strings = ~[
		~".jammer.",~".Jammer.",~".123456.",~".Jam4er.",~".Ja1mer.",
		~".Jamm8r.",~".J3mmer.",~".Ja43er.",~".Jam12r.",~".99mmer.",
		~".2amm8r.",~".23mmer.",~".2a43er.",~".2am12r.",~".29mmer.",
		~".2amm81.",~".23mme1.",~".2a43e1.",~".2am121.",~".29mme1.",
		~"Xymmer..",~"XXmme4..",~"XX3456..",~"XXm4er..",~"XX1mer..",
		~"XXmm8r..",~"xXmme3..",~"Xx43er..",~"Xxm12r..",~"XXmme3..",
		~"XXmm8r..",~"XXmme2..",~"xX43e2..",~"XXm122..",~"XXmme2..",
		~"XXmm81..",~"XXmme1..",~"XX43e1..",~"XXm121..",~"XXmme1..",
		~"^Xmm8r..",~"^Xmmer..",~"^X43er..",~"^Xm12r..",~"^Xmmer..",
		~"^Xmm81..",~"^Xmme1..",~"^X43e1..",~"^Xm121..",~"^Xmme1.."
	];
	
	let lcase_strings = ~[
		~".jammer.",~".jammer.",~".123456.",~".jam4er.",~".ja1mer.",
		~".jamm8r.",~".j3mmer.",~".ja43er.",~".jam12r.",~".99mmer.",
		~".2amm8r.",~".23mmer.",~".2a43er.",~".2am12r.",~".29mmer.",
		~".2amm81.",~".23mme1.",~".2a43e1.",~".2am121.",~".29mme1.",
		~"xymmer..",~"xxmme4..",~"xx3456..",~"xxm4er..",~"xx1mer..",
		~"xxmm8r..",~"xxmme3..",~"xx43er..",~"xxm12r..",~"xxmme3..",
		~"xxmm8r..",~"xxmme2..",~"xx43e2..",~"xxm122..",~"xxmme2..",
		~"xxmm81..",~"xxmme1..",~"xx43e1..",~"xxm121..",~"xxmme1..",
		~"^xmm8r..",~"^xmmer..",~"^x43er..",~"^xm12r..",~"^xmmer..",
		~"^xmm81..",~"^xmme1..",~"^x43e1..",~"^xm121..",~"^xmme1.."
	];

	/* Test 1: Intern Strings */
	
	let mut lwc = lwc();
	//let mut interned_strings: ~[arc::RWARC<~lwc_string>] = ~[];
	
	let start_time = std::time::precise_time_ns();

	for uint::range(0, 10000) |_| {		
		for uint::range(0, 50) |j| {
			do lwc.write |l| {
				l.lwc_intern_string(strings[j]);
			}
		}
	}
	
	let end_time = std::time::precise_time_ns();

	let delta = (end_time - start_time) as float;

	io::println(fmt!("Time for lwc_intern_string %.3f usec", delta / 1000f));

	/* Test 2: Is Equal */

}