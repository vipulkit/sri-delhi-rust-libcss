
extern mod std;
extern mod parserutils;
extern mod test;

use std::arc;
use parserutils::*;
use test::*;
use core::str::*;

fn main(){

 	let mut parser : arc::ARC<~lpu> = lpu();
	let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));
	let aliasData = ~[~"moose", ~"csinvariant", ~"csinvariant\"", ~"nats-sefi-add", ~"u.t.f.8"];
	 
	let mut index : uint = 0;

	while index < aliasData.len() {
		let mut retVal = arc::get(&parser).parserutils__charset_alias_canonicalise(copy aliasData[index]);
		
		match(retVal){

			Some(x) => {
						if !eq(&aliasData[index], &~"moose")
						{
							test_logger.pass( ~"parserutils", ~"parserutils.rs", ~"parserutils__charset_alias_canonicalise", copy aliasData[index], ~"");

							if !eq(&aliasData[index], &~"u.t.f.8")
							{																
								if (arc::get(&parser).parserutils_charset_mibenum_from_name(copy x.name) == 0 as u16)
								{
									test_logger.fail( ~"parserutils", ~"parserutils.rs", ~"mibenum_from_name", copy x.name, ~"")
								}	
								else
								{
									test_logger.pass( ~"parserutils", ~"parserutils.rs", ~"mibenum_from_name", copy x.name, ~"")
								}							
					
								match(arc::get(&parser).parserutils_charset_mibenum_to_name(copy x.mib_enum))
								{
									Some(val)	=> {
														test_logger.pass( ~"parserutils", ~"parserutils.rs", ~"mibenum_to_name", copy x.name, ~"")
							 						},
									None		=>{
													test_logger.fail( ~"parserutils", ~"parserutils.rs", ~"mibenum_to_name", copy x.name, ~"")
							 						}				 
								}
							}

						else
						 {
						 	test_logger.fail( ~"parserutils", ~"parserutils.rs", ~"parserutils__charset_alias_canonicalise", copy aliasData[index], ~"")
						 }								
						}
					},
			None => {	
						if eq(&aliasData[index], &~"moose")
						{
							test_logger.pass( ~"parserutils", ~"parserutils.rs", ~"parserutils__charset_alias_canonicalise", copy aliasData[index], ~"")
						}
						else
						{
							test_logger.fail( ~"parserutils", ~"parserutils.rs", ~"parserutils__charset_alias_canonicalise", copy aliasData[index], ~"")
						}
					}
			}														
			index += 1;
		}		
	}					

