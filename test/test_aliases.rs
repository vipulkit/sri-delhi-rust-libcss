
extern mod std;
extern mod parserutils;
extern mod test;

use std::arc;
use parserutils::*;
use test::*;
use core::str::*;

fn main() {

	let mut parser : arc::ARC<~lpu> = lpu();
	let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));
	let mut external_argument : ~str = ~"";
	let aliasData = ~[~"moose", ~"csinvariant", ~"csinvariant\"", ~"nats-sefi-add", ~"u.t.f.8"];
	 
	let mut index : uint = 0;

	while index < aliasData.len() {
		let mut retVal = arc::get(&parser).parserutils__charset_alias_canonicalise(copy aliasData[index]);
		
		match(retVal){

			Some(x) => {
						if !eq(&aliasData[index], &~"moose") {							
							if !eq(&aliasData[index], &~"u.t.f.8") {		
								let data = arc::get(&parser).parserutils_charset_mibenum_from_name(copy x.name);								
								if data == 0 as u16 {
									test_logger.fail( ~"test_aliases.rs", copy external_argument, ~"parserutils", ~"parserutils.rs", ~"mibenum_from_name", copy x.name, ~"Non Zero value", data.to_str(), ~"");
								}	
								else {
									test_logger.pass(  ~"test_aliases.rs", copy external_argument, ~"parserutils", ~"parserutils.rs", ~"mibenum_from_name", copy x.name, ~"Non Zero value", data.to_str(), ~"");
								}							
													
								let data = arc::get(&parser).parserutils_charset_mibenum_to_name(copy x.mib_enum);
								match(copy data) {
									Some(val)	=> {
														test_logger.pass(  ~"test_aliases.rs", copy external_argument, ~"parserutils", ~"parserutils.rs", ~"mibenum_to_name", copy x.name, ~"Non Zero value", data.unwrap(), ~"")
													},
									None		=>{
													test_logger.fail(  ~"test_aliases.rs", copy external_argument, ~"parserutils", ~"parserutils.rs", ~"mibenum_to_name", copy x.name, ~"Non Zero value", data.unwrap(), ~"")
													}				 
								}
							}													
						}
					},
			None => {	
						if eq(&aliasData[index], &~"moose") {
							test_logger.pass(  ~"test_aliases.rs", copy external_argument, ~"parserutils", ~"parserutils.rs", ~"parserutils__charset_alias_canonicalise", copy aliasData[index], ~"No value", ~"No value", ~"")
						}
						else { test_logger.fail(  ~"test_aliases.rs", copy external_argument, ~"parserutils", ~"parserutils.rs", ~"parserutils__charset_alias_canonicalise", copy aliasData[index], ~"Some value", ~"No value", ~"")
						}
					}
			}														
			index += 1;
		}		
	}					

