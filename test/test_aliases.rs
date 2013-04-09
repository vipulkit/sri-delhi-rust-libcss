
extern mod std;
extern mod parserutils;

use parserutils::*;

fn main()
{
	let mut parser : ~lpu = lpu();

	let mut retVal = parser.parserutils__charset_alias_canonicalise(~"moose");

	match(copy retVal)
	{
		Some(x) => io::println("FAIL - found invalid encoding 'moose'\n"),		
		None => (io::println("PASS: moose "))
	}

	retVal = parser.parserutils__charset_alias_canonicalise(~"csinvariant");
    retVal = parser.parserutils__charset_alias_canonicalise(~"US-ASCII");
	match(copy retVal)
	{
		Some(x) => { //io::println(fmt!("name is %? and mib_enum is %?", x.name, x.mib_enum));
			if( x.mib_enum==3 ) {
            	io::print(fmt!("\n[test_aliases] : [file=parserutils.rs] : [function=lpu::parserutils__charset_alias_canonicalise] : PASSED"));
            }
            else {
            	io::print(fmt!("\n[test_aliases] : [file=parserutils.rs] : [function=lpu::parserutils__charset_alias_canonicalise] : FAIL"));
            }
            io::print(fmt!("\n Test with values US-ASCII result is %? ",x.mib_enum)); },		
		None	=>	io::println("FAIL - failed finding encoding 'csinvariant' ")
	}

	/*
	retVal = parser.parserutils__charset_alias_canonicalise(~"csinvariant\"");
	match(copy retVal)
	{
		Some(x) => io::println(fmt!("name is %? and mib_enum is %?", x.name, x.mib_enum)),
		None	=>	io::println("FAIL - failed finding encoding 'csinvariant' ")
	}
	
	retVal = parser.parserutils__charset_alias_canonicalise(~"nats-sefi-add");
	match(copy retVal)
	{
		Some(x) => io::println(fmt!("name is %? and mib_enum is %?", x.name, x.mib_enum)),
		None	=>	io::println("FAIL - failed finding encoding 'nats-sefi-add' ")
	}

	match(copy retVal)
	{
		Some(x)	=> {
					io::println(fmt!("%?", parser.parserutils_charset_mibenum_from_name(copy x.name)));
					io::println(fmt!("%?", parser.parserutils_charset_mibenum_to_name(copy x.mib_enum)));
					},
		None	=>	()
	}		

	let retVal = parser.parserutils__charset_alias_canonicalise(~"u.t.f.8");
	match(copy retVal)
	{
		Some(x) => io::println(fmt!("name is %? and mib_enum is %?", copy x.name, copy x.mib_enum)),
		None	=>	io::println("FAIL - failed finding encoding 'u.t.f.8' ")
	}
	*/
}	
