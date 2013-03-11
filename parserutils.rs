#[link(name = "parserutils", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod riconv;
use core::io::Reader;
use core::io::ReaderUtil;
use std::oldmap;
use libc::size_t;

pub struct parserutils_charset_aliases_canon {
	mib_enum:u16,
	name_len:u16,
	name: @str
}

pub struct struct_settings{
	encoding: u16 		 /*< Input encoding */
}    

pub struct parserutils_filter {
	mut int_enc: u16,               /**< The internal encoding */
	mut settings : struct_settings ,
	mut iconv_h : u64 ,
	mut pw : ~[u8]
}

pub enum parserutils_result
{
	PARSERUTILS_OK(@parserutils_filter),
    PARSERUTILS_BADPARAM,
    PARSERUTILS_NOMEM,
    PARSERUTILS_BADENCODING,
    PARSERUTILS_DESTROY_SUCCESS
}

pub struct lpu {
    // these two data structures together can be used for mibenum->canonical name conversion
	mut canonical_name_list: ~[~str],
	mut mibenum_map: @oldmap::HashMap<u16,uint>,
	// this data structure can be used for name (canonnical/alias) ->mibenum conversion
	mut alias_map: @oldmap::HashMap<~str,u16>
}


impl lpu {
	fn read_aliases(&self)
	{
		let aliases_file_reader: Reader = result::get(&io::file_reader(&Path(&"Aliases")));

		let mut line_number=1;

		while !aliases_file_reader.eof()
		{
			let line = aliases_file_reader.read_line();
			if (!str::starts_with(line,"#") && line.len()>0) {
				let alias_entry_columns = str::split_str_nonempty(line,"\t");
				
				// first column is canonical name
				let canonical_name = copy alias_entry_columns[0];
				// second column is mibenum
				let mibenum = u16::from_str(alias_entry_columns[1]).get();
				
				// add the canonical name to the list of canonical names
				self.canonical_name_list.push(copy canonical_name);
				// insert <mibenum, index of canonical name> into mibenum_map
				self.mibenum_map.insert(mibenum,line_number-1);
				// insert <canonical_name, mibenum> into alias_map
				self.alias_map.insert(canonical_name, mibenum);

				// optionally, the third column has other aliases
				if (alias_entry_columns.len() > 2) {
					let aliases=str::split_str_nonempty(alias_entry_columns[2]," ");
					// insert <alias, mibenum> into alias_map
					for aliases.each |&alias| {
						self.alias_map.insert(alias, mibenum);
					}
				}
				line_number=line_number+1;
			}
		}
	}

	pub fn parserutils__charset_alias_canonicalise(&self, alias: &~str) -> Option<parserutils_charset_aliases_canon> { 
        match self.alias_map.find(alias) {
        	None => None,
        	Some(temp_mib_enum) => {
        		match self.mibenum_map.find(&temp_mib_enum) {
        			None => None,
        			Some(canonical_name_list_index) => {
        				if (canonical_name_list_index < self.canonical_name_list.len()) {
        					let temp_name = (self.canonical_name_list[canonical_name_list_index]).to_managed();
        					Some( parserutils_charset_aliases_canon {
								        mib_enum : temp_mib_enum,
								        name : temp_name,
								        name_len : temp_name.len() as u16
			    					}
        						)
        				}
        				else {
        					None
        				}
        			}
        		}
        	}
        }
	}

	pub fn parserutils_charset_mibenum_from_name(&self, alias: ~str) -> Option<u16> {
        match self.alias_map.find(&alias) {
        	None => None ,
        	Some(mib_enum) => Some(mib_enum)
        }
	}

	pub fn parserutils_charset_mibenum_to_name(&self, mibenum: u16)-> Option<~str> {
		match self.mibenum_map.find(&(mibenum)) {
			None => None,
			Some (canonical_name_list_index) => {
				if canonical_name_list_index < self.canonical_name_list.len() {
					Some(copy self.canonical_name_list[canonical_name_list_index])
				}
				else {
					None
				}
			}
		}
	}

	pub fn filter_set_encoding(&self, input : @parserutils_filter ,enc : ~str) -> parserutils_result {

		if enc.len()==0 {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum_search_result : Option<u16> = self.parserutils_charset_mibenum_from_name(enc);
		if mibenum_search_result.is_none() {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum = mibenum_search_result.get();

		/* Exit early if we're already using this encoding */
		if input.settings.encoding==mibenum {
			return PARSERUTILS_OK(input);
		}

		if riconv::riconv_initialized(input.iconv_h) {
			riconv::safe_riconv_close(input.iconv_h);
			input.iconv_h=riconv::riconv_initialize();
		}

		let totype : Option<~str> = self.parserutils_charset_mibenum_to_name(input.int_enc) ;
		let fromtype : Option<~str> = self.parserutils_charset_mibenum_to_name(mibenum) ;

		if totype.is_none() || fromtype.is_none() {
			return PARSERUTILS_BADPARAM;
		}

		input.iconv_h = riconv::safe_riconv_open(totype.get(),fromtype.get());

		if riconv::riconv_initialized(input.iconv_h)==false {
			return PARSERUTILS_BADENCODING;
		}

		input.settings.encoding = mibenum;
		PARSERUTILS_OK(input)
	}

	pub fn filter_set_defaults(&self, input : @parserutils_filter ) -> parserutils_result{
		input.settings.encoding = 0;
		self.filter_set_encoding(input, ~"UTF-8")
	}

	pub fn parserutils__filter_create(&self, int_enc: ~str ,  pw : ~[u8]) -> parserutils_result {
		if int_enc.len() == 0 {
			return PARSERUTILS_BADPARAM;
		}

		let mut f : @parserutils_filter = @parserutils_filter {
			int_enc:0u16,
			settings : 	struct_settings {
							encoding:0u16
						},
			iconv_h : riconv::riconv_initialize(),
			pw : ~[]
		};
		
		match self.parserutils_charset_mibenum_from_name(int_enc) {
			None => return PARSERUTILS_BADENCODING,
			Some(x) => f.int_enc = x 
		}

		f.pw = copy pw;

		f.settings.encoding = 0;
		self.filter_set_encoding(f, ~"UTF-8")
	}
	
	pub fn parserutils_charset_utf8_char_byte_length(self, s: ~[u8]) -> Option<u8> {
		let  numContinuations : ~[u8] = ~[
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
		2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
		3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5,
		] ;

		if s.len()==0 {
			None
		}
		else {
			Some(numContinuations[s[0]] + 1)  
		}
	}

	pub fn parserutils__filter_destroy(&self, input : @parserutils_filter ) -> parserutils_result {
		if riconv::riconv_initialized(input.iconv_h)  {
			riconv::safe_riconv_close(input.iconv_h);
			input.iconv_h=riconv::riconv_initialize();
			PARSERUTILS_DESTROY_SUCCESS 
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

	pub fn parserutils__filter_setopt(&self, input : @parserutils_filter , encoding : ~str) -> parserutils_result {
		if encoding.len()==0 {
			PARSERUTILS_BADPARAM
		}
		else {
			self.filter_set_encoding(input,encoding)
		}
	}

	pub fn parserutils__filter_reset(&self, input : @parserutils_filter ) -> parserutils_result {
		if riconv::riconv_initialized(input.iconv_h) {
			riconv::safe_riconv_close(input.iconv_h);
			input.iconv_h=riconv::riconv_initialize();
			PARSERUTILS_OK(input)
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

}

pub fn lpu() -> @lpu {
	let new_lpu = @lpu {
		canonical_name_list : ~[],
		mibenum_map : @oldmap::HashMap::<u16,uint>(),
		alias_map : @oldmap::HashMap::<~str,u16>()
	};

	new_lpu.read_aliases();
	new_lpu
}