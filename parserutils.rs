#[link(name = "parserutils", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod riconv;
use core::io::Reader;
use core::io::ReaderUtil;
//use std::oldmap;
use core::hashmap::linear::LinearMap;
use core::vec::*;
use core::str::raw::* ;
use core::vec::raw::* ;
//use core::libc::size_t;

//pub type parserutils_charset_detect_func =  
//	~extern fn(data: ~[u8], mibenum:~u16, source:~u32) -> parserutils_result;

pub struct parserutils_inputstream 
{
	utf8: ~[u8],	/*< Buffer containing UTF-8 data */

	cursor: uint,		/*< Byte offset of current position */

	 had_eof: bool,			/*< Whether EOF has been reached */

	raw: ~[u8],	/*< Buffer containing raw data */

	done_first_chunk: bool ,		/*< Whether the first chunk has been processed */

	mibenum: u16,		/*< MIB enum for charset, or 0 */
	
	encsrc: u32,		/*< Charset source */

	input: @parserutils_filter ,	/*< Charset conversion filter */

	//mut csdetect: parserutils_charset_detect_func  /*< Charset detection func.*/

	//alloc: parserutils_alloc	/*< Memory (de)allocation function */
	//mut pw : ~[u8]			/*< Client private data */
} 

pub struct parserutils_charset_aliases_canon {
	mib_enum:u16,
	name_len:u16,
	name: @str
}

pub struct struct_settings{
	encoding: u16 		 /*< Input encoding */
}    

pub struct parserutils_filter {
	int_enc: u16,               /**< The internal encoding */
	settings : struct_settings ,
	iconv_h : u64 ,
	// mut pw : ~[u8]
}

pub enum parserutils_result
{
	PARSERUTILS_FILTER_CREATE_OK(@mut parserutils_filter),
	PARSERUTILS_FILTER_PROCESS_CHUNK_OK((u64,~[u8])),
	PARSERUTILS_INPUTSTREAM_CREATE_OK(@parserutils_inputstream),
	PARSERUTILS_CHARSET_EXT_OK((@u16,@u32)),
	PARSERUTILS_CHARSET_TRY_OK(@u16),
	PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(~[u8]),
	PARSERUTILS_GENERAL_OK,
    PARSERUTILS_BADPARAM,
    PARSERUTILS_NOMEM,
    PARSERUTILS_EOF,
    PARSERUTILS_BADENCODING,
    PARSERUTILS_DESTROY_SUCCESS,
    PARSERUTILS_NEEDDATA,
    PARSERUTILS_INVALID,
    PARSERUTILS_ICONV_ERROR,
    PARSERUTILS_SUCCESS,
}

pub struct lpu {
	
    // these two data structures together can be used for mibenum->canonical name conversion
	canonical_name_list: ~[~str],
	mibenum_map: ~LinearMap<u16,uint>,
	// this data structure can be used for name (canonnical/alias) ->mibenum conversion
	alias_map: ~LinearMap<~str,u16>
}

pub fn memcmp(str1 : &[u8] , str2 : &[u8] , len : uint ) -> int {
	let mut i : uint = 0 ;
	while ( i<len ) {
		if str1[i] != str2[i] {
			return ( (str1[i]-str2[i]) as int) ;
		}
		i = i+1 ; 
	}
	0
}

impl lpu {
	fn read_aliases(&mut self)
	{
		let aliases_file_reader: @Reader = (&io::file_reader(&Path(&"Aliases"))).get();

		let mut line_number=1;

		while !aliases_file_reader.eof()
		{
			let line = aliases_file_reader.read_line();
			if (!str::starts_with(line,"#") && line.len()>0) {
				let mut alias_entry_columns : ~[~str] = ~[];
				for str::each_split_str_nonempty(line,"\t") |column| {
					alias_entry_columns.push(column.to_owned());
				} 
				
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
					//let aliases=str::split_str_nonempty(alias_entry_columns[2]," ");
					let mut aliases : ~[~str] = ~[];
					for str::each_split_str_nonempty(alias_entry_columns[2]," ") |alias| {
						aliases.push(alias.to_owned());
					} 
					// insert <alias, mibenum> into alias_map
					for aliases.each |&alias| {
						self.alias_map.insert(alias, mibenum);
					}
				}
				line_number=line_number+1;
			}
		}
	}

	pub fn parserutils__charset_alias_canonicalise(&mut self, alias: &~str) -> Option<parserutils_charset_aliases_canon> { 
        match self.alias_map.find(alias) {
        	None => None,
        	Some(temp_mib_enum) => {
        		match self.mibenum_map.find(temp_mib_enum) {
        			None => None,
        			Some(canonical_name_list_index) => {
        				if (canonical_name_list_index < &self.canonical_name_list.len()) {
        					let temp_name = (self.canonical_name_list[*canonical_name_list_index]).to_managed();
        					Some( parserutils_charset_aliases_canon {
								        mib_enum : *temp_mib_enum,
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

	pub fn parserutils_charset_mibenum_from_name(&mut self, alias: ~str) -> u16 {
        match self.alias_map.find(&alias) {
        	None => 0 ,
        	Some(mib_enum) => *mib_enum
        }
	}

	pub fn parserutils_charset_mibenum_to_name(&mut self, mibenum: u16)-> Option<~str> {
		match self.mibenum_map.find(&(mibenum)) {
			None => None,
			Some (canonical_name_list_index) => {
				if canonical_name_list_index < &self.canonical_name_list.len() {
					Some(copy self.canonical_name_list[*canonical_name_list_index])
				}
				else {
					None
				}
			}
		}
	}

	pub fn filter_set_encoding(&mut self, input : @mut parserutils_filter ,enc : ~str) -> parserutils_result {

		if enc.len()==0 {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(enc);
		if mibenum_search_result==0 {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum = mibenum_search_result;

		// Exit early if we're already using this encoding 
		if input.settings.encoding==mibenum {
			return PARSERUTILS_FILTER_CREATE_OK(input);
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

		if (!riconv::riconv_initialized(input.iconv_h)) {

			return PARSERUTILS_BADENCODING;
		}

		input.settings.encoding = mibenum;
		PARSERUTILS_FILTER_CREATE_OK(input)
	}

	pub fn filter_set_defaults(&mut self, input : @mut parserutils_filter ) -> parserutils_result{
		input.settings.encoding = 0;
		self.filter_set_encoding(input, ~"UTF-8")
	}

	pub fn parserutils__filter_create(&mut self, int_enc: ~str ) -> parserutils_result {
		if int_enc.len() == 0 {
			return PARSERUTILS_BADPARAM;
		}

		let mut f : @mut parserutils_filter = @mut parserutils_filter {
			int_enc:0u16,
			settings : 	struct_settings {
							encoding:0u16
						},
			iconv_h : riconv::riconv_initialize(),
			//pw : ~[]
		};
		f.int_enc =self.parserutils_charset_mibenum_from_name(int_enc);
		if f.int_enc==0 {
			return PARSERUTILS_BADENCODING ;
		}

		//f.pw = copy pw;

		f.settings.encoding = 0;
		self.filter_set_encoding(f, ~"UTF-8")
	}
	
	pub fn parserutils_charset_utf8_char_byte_length(&mut self, s: ~[u8]) -> Option<u8> {
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

	pub fn parserutils__filter_destroy(&mut self, input : @mut parserutils_filter ) -> parserutils_result {
		if riconv::riconv_initialized(input.iconv_h)  {
			riconv::safe_riconv_close(input.iconv_h);
			input.iconv_h=riconv::riconv_initialize();
			PARSERUTILS_DESTROY_SUCCESS 
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

	pub fn parserutils__filter_setopt(&mut self, input : @mut parserutils_filter , encoding : ~str) -> parserutils_result {
		if encoding.len()==0 {
			io::println("__filter_setopt_bad param");//sandeep
			PARSERUTILS_BADPARAM
			
		}
		else {
			self.filter_set_encoding(input,encoding)
		}
	}

	pub fn parserutils__filter_reset(&mut self, input : @mut parserutils_filter ) -> parserutils_result {
		if riconv::riconv_initialized(input.iconv_h) {
			riconv::safe_riconv_close(input.iconv_h);
			input.iconv_h=riconv::riconv_initialize();
			PARSERUTILS_FILTER_CREATE_OK(input)
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

	pub fn parserutils__filter_process_chunk(&mut self, input : @mut parserutils_filter ,inbuf : ~[u8] ) -> parserutils_result {

		io::println(fmt!("\n Input arguments to the process chunk is =%?= ",inbuf));
		
		if ( input.int_enc==0 )||( inbuf.len()==0 ) {
			return PARSERUTILS_BADPARAM;
		}
		
		let iconv_result = riconv::safe_riconv(input.iconv_h, inbuf) ;

		if iconv_result.len_processed==0 {
			if iconv_result.err_state==1 {
				return PARSERUTILS_NOMEM ;
			}
			if iconv_result.err_state==2 {
				return PARSERUTILS_BADPARAM ;
			}
			else {
				return PARSERUTILS_ICONV_ERROR ;
			}
		}
		PARSERUTILS_FILTER_PROCESS_CHUNK_OK( (iconv_result.len_processed, copy iconv_result.outbuf) )
	}

	pub fn try_utf32_charset(&mut self, data : ~[u8]) -> parserutils_result {

		let mut charset: u16 = 0;
		let CHARSET_BE : &[u8] = ['0' as u8, '0' as u8, '0' as u8, '@' as u8, '0' as u8, '0' as u8, '0' as u8, 'c' as u8, '0' as u8, '0' as u8, '0' as u8, 'h' as u8, '0' as u8, '0' as u8, '0' as u8, 'a' as u8, '0' as u8, '0' as u8, '0' as u8, 'r' as u8, '0' as u8, '0' as u8, '0' as u8, 's' as u8, '0' as u8, '0' as u8, '0' as u8, 'e' as u8, '0' as u8, '0' as u8, '0' as u8, 't' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '"' as u8] ; 
		let CHARSET_LE : &[u8] = [ '@' as u8,'0' as u8,'0' as u8,'0' as u8,'c' as u8,'0' as u8,'0' as u8,'0' as u8,'h' as u8,'0' as u8,'0' as u8,'0' as u8,'a' as u8,'0' as u8,'0' as u8,'0' as u8,'r' as u8,'0' as u8,'0' as u8,'0' as u8,'s' as u8,'0' as u8,'0' as u8,'0' as u8,'e' as u8,'0' as u8,'0' as u8,'0' as u8,'t' as u8,'0' as u8,'0' as u8,' ' as u8,'0' as u8,'0' as u8,'0' as u8,'"' as u8,'0' as u8,'0' as u8,'0' as u8, ] ;

		io::println("\n Sushanta1: Inside CHARSET_LE 32 bit");
		io::println(fmt!("value of data.len() is %?", data.len()));
		io::println(fmt!("value of CHARSET_LE.len() is %?", CHARSET_LE.len()));

		// Here, when the data.len() is equals to CHARSET_LE.len() then it returns, then how would the next case would paas when again we re asking it to pass when length are equal ??
		if data.len() <= CHARSET_LE.len()
		{
			return PARSERUTILS_BADPARAM;
		}

		// if (memcmp(copy data, copy CHARSET_LE, CHARSET_LE.len()) == 0) 
		 //if (memcmp(data, CHARSET_LE, CHARSET_LE.len()) == 0) 

		let retVal : int = memcmp(data, CHARSET_LE, CHARSET_LE.len());
		io::println(fmt!("value of retVal is %?", retVal));
		if (retVal == 0) 
		{
			io::println("\n Inside CHARSET_LE 32 bit");

			io::println("\n 1 ");
			let startIndex : uint = data.len() + CHARSET_LE.len();
			let mut endIndex : uint = startIndex;

			io::println("\n 2 ");
			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;

			io::println("\n 3 ");

			//io::println(fmt!( ));	
			while endIndex < (CHARSET_LE.len() -1)
			{
				io::println("\n Sushanta1: while loop");
				let value1 : u8 = data[endIndex] | data[endIndex + 1] << 8 | data[endIndex + 2] << 16 | data[endIndex + 3] << 24 ;
		
				if value1 > 0x007f
				{
					break;
				}	

				if (value1 == '"' as u8) && (endIndex < data.len() + CHARSET_LE.len() - 8)		
				{
					let value2 = data[endIndex + 4] | data[endIndex + 5] << 8 | data[endIndex + 6] << 16 | data[endIndex + 7] << 24 ;

					if value2 == ';' as u8	
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u8 && value1 <= 'z' as u8
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				endIndex += 4;	
			} // while loop ends		
			
			// After while loop ends
			if (endIndex == data.len() - 4)
			{
				return PARSERUTILS_NEEDDATA;
			}


			// Convert to MIB enum 
			unsafe {
				charset = self.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_LE.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 

				charset == self.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16") ||

				charset == self.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{

				charset = 0;
			}
		}
		
		 let retVal2 : int = memcmp(data, CHARSET_BE, CHARSET_LE.len());
		 io::println(fmt!("value of retVal is %?", retVal2));
		 if (retVal2 == 0) 
		{
			io::println("\n 11: If condition passed ");

			let startIndex : uint = CHARSET_BE.len() - 1;
			let mut endIndex : uint = startIndex;

			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;
			
			// Is this condition right ?
			while (endIndex < (data.len() - 4))
			{
				let value1 : u8 = data[endIndex + 3] | data[endIndex + 2] << 8 | data[endIndex + 1] << 16 | data[endIndex] << 24 ;
				
				if value1 > 0x007f
				{
					break;
				}	

				// Is this condition right ?
				if (value1 == '"' as u8) && (endIndex < data.len() + CHARSET_BE.len() - 8)		
				{
					let value2 = data[endIndex + 7] | data[endIndex + 6] << 8 | data[endIndex + 5] << 16 | data[endIndex + 4] << 24 ;

					if value2 == ';' as u8	
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u8 && value1 <= 'z' as u8
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				// termination conditioning for while loop	
				endIndex += 4;	

			} // while loop ends

			if (endIndex == data.len() - 4)
			{
				return PARSERUTILS_NEEDDATA;
			}

			// Convert to MIB enum 
			unsafe {
				charset = self.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_BE.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 
			    charset == self.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16") ||
			    charset == self.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{
				charset = 0;
			}
		}
		else
		{
			io::println("\n Sushanta1: Inside NOWHERE, means ERROR ");
		}
		
		PARSERUTILS_CHARSET_TRY_OK(@4)
	}

	pub fn try_utf16_charset(&mut self, data : &[u8]) -> parserutils_result {
	//pub fn try_utf16_charset(data : &[u8]) -> parserutils_result {
		let mut charset: u16 = 0;
		let CHARSET_BE : &[u8] = ['0' as u8, '@' as u8, '0' as u8, 'c' as u8, '0' as u8, 'h' as u8, '0' as u8, 'a' as u8, '0' as u8, 'r' as u8, '0' as u8, 's' as u8, '0' as u8, 'e' as u8, '0' as u8, 't' as u8, '0' as u8, ' ' as u8,'0' as u8, '"' as u8] ; 
		
		let CHARSET_LE : &[u8] = ['@' as u8, '0' as u8, 'c' as u8, '0' as u8, 'h' as u8, '0' as u8, 'a' as u8, '0' as u8, 'r' as u8, '0' as u8, 's' as u8, '0' as u8, 'e' as u8, '0' as u8, 't' as u8, '0' as u8, ' ' as u8, '0' as u8, '"' as u8, '0' as u8] ; 

		io::println("\n Sushanta1: Inside CHARSET_LE 16 bit");
		io::println(fmt!("value of data.len() is %?", data.len()));
		io::println(fmt!("value of CHARSET_LE.len() is %?", CHARSET_LE.len()));
		io::println(fmt!("value of CHARSET_BE.len() is %?", CHARSET_BE.len()));

		io::println(fmt!("value of data is %?", data));
		io::println(fmt!("value of CHARSET_LE is %?", CHARSET_LE));
		io::println(fmt!("value of CHARSET_BE is %?", CHARSET_BE));
		
		if data.len() <= CHARSET_LE.len()
		{
			return PARSERUTILS_BADPARAM;
		}

		if (memcmp(data, CHARSET_LE, CHARSET_LE.len()) == 0) 
		{
			io::println("Sushanta1: Inside CHARSET_LE 16 bits ");

			let startIndex : uint = CHARSET_LE.len() - 1 ;
			let mut endIndex : uint = startIndex;

			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;

			while endIndex < (data.len()- 2)
			{
				io::println("Sushanta1: Inside while loop for CHARSET_LE 16 bits ");
				let value1 : u16 = (data[endIndex] | data[endIndex + 1] << 8) as u16 ;

					
				// if value1 > 0x007f
				// {
				// 	break;
				// }	
				

				if (value1 == '"' as u16) && (endIndex < data.len() + CHARSET_LE.len() - 4)		
				{
					let value2 : u16 = (data[endIndex + 2] | data[endIndex + 3] << 8) as u16 ;

					if value2 == ';' as u16	
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u16 && value1 <= 'z' as u16
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				// termination conditioning for while loop	
				endIndex += 2;	
			} // while loop ends		
			
			// After while loop ends
			if (endIndex == data.len() + CHARSET_LE.len() - 2)
			{
				return PARSERUTILS_NEEDDATA;
			}


			// Convert to MIB enum 
			unsafe {
				charset = self.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_LE.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 

				charset == self.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16") ||

				charset == self.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{

				charset = 0;
			}
		 }	
		else if (memcmp(data, CHARSET_BE, CHARSET_BE.len()) == 0) 
		{
			io::println("Sushanta1: Inside CHARSET_BE 16 bits ");

			let startIndex : uint = (CHARSET_BE.len() - 1);
			let mut endIndex : uint = startIndex;

			io::println(fmt!("value of startIndex is %?", startIndex));
			
			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;

			while endIndex < (data.len() - 2)
			{
				io::println("Sushanta1: Inside while loop for CHARSET_BE 16 bits ");

				io::println(fmt!("value of data[endIndex] is %?", data[endIndex]));
				io::println(fmt!("value of data[endIndex + 1] is %?", data[endIndex + 1]));
				
				io::println(fmt!("value of data[endIndex + 1]<<8 is %?", data[endIndex + 1]<<8));
				io::println(fmt!("value of data[endIndex]<<8 is %?", data[endIndex]<<8));
				io::println(fmt!("value of data[endIndex+1] | data[endIndex]<<8 is %?", data[endIndex] | data[endIndex]<<8));

				// Since it is Big-endian, data at MSB would be at lower address space
				
				// let value1 : u16 = (data[endIndex + 1] | data[endIndex] << 8) as u16 ;
				// io::println(fmt!("value of value1 is %?", value1));
						
				
				let mut value1 : u16 = data[endIndex] as u16;
				io::println(fmt!("value of value1 is %?", value1));
				value1 = value1 << 8;
				io::println(fmt!("value of value1 is %?", value1));
				value1 = value1 + data[endIndex+1] as u16;
				io::println(fmt!("value of value1 is %?", value1));
				io::println(fmt!("value of 0x007f is %?", 0x007f));

				// value1 is getting bigger val then 0x007f
				
				// if value1 > 0x007f
				// {
				// 	io::println("Sushanta1: value1 > 0x007f is satisfied, Going to break...");
				// 	break;
				// }	
				

				if (value1 == '"' as u16) && (endIndex < data.len() - 4)		
				{
					io::println(" CONDITION is passed...");
					let value2 = (data[endIndex + 3] | data[endIndex + 2] << 8) as u16;

					if value2 == ';' as u16
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u16 && value1 <= 'z' as u16
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				// termination conditioning for while loop	
				endIndex += 2;	
			} // while loop ends		
			
			if (endIndex == data.len()- 2)
			{
				return PARSERUTILS_NEEDDATA;
			}

			io::println(" Outside while loop ...");

			// Convert to MIB enum 
			unsafe {
				io::println(" B4 condn in UNSAFE...");
				charset = self.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_BE.len(), data.len()-1)) , data.len()-1 ) );
				io::println(" After condn in UNSAFE...");
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 
			    charset == self.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16") ||
			    charset == self.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{
				charset = 0;
			}
		}// else if terminates
		else
		{
			io::println("\n Sushanta1: Inside NOWHERE 16 BITS means ERROR ");
		}

		PARSERUTILS_CHARSET_TRY_OK(@4)
	}

	pub fn  try_ascii_compatible_charset(&mut self, data : ~[u8]) -> parserutils_result {

		let mut charset : u16 = 0;
		let CHARSET : ~[u8] = ~[ '@' as u8, 'c' as u8, 'h' as u8, 'a' as u8 , 'r' as u8, 's' as u8, 'e' as u8, 't' as u8, ' ' as u8 , '\"'  as u8] ;

		io::println("\n Sushanta1: Inside ASCII fun");
		io::println(fmt!("value of data.len() is %?", data.len()));
		io::println(fmt!("value of CHARSET.len() is %?", CHARSET.len()));
		
		io::println(fmt!("value of data is %?", data));
		io::println(fmt!("value of CHARSET is %?", CHARSET));

		if (data.len() <= CHARSET.len() ) {
			return PARSERUTILS_NEEDDATA;
		}

		// Look for @charset, assuming ASCII-compatible source data 
		//if ( memcmp(data, CHARSET, CHARSET.len() ) == 0) 
		 let retVal : int = memcmp(data, CHARSET, CHARSET.len());
		 io::println(fmt!("value of retVal is %?", retVal));
		 if (retVal == 0) 
		{
			io::println("INSIDE ASCII if condition ");

			let mut indexVal = CHARSET.len()-1;
			io::println(fmt!("value of indexVal at CHARSETlen() is %?", indexVal));

			// Looking for "; at the end of charset declaration
			while (indexVal < data.len()) 
			{
				io::println(fmt!("value of indexVal is %?", indexVal));
				io::println(fmt!("value of data[indexVal] is %?", data[indexVal]));

				//if data[indexVal] == ('"' as u8) && data[indexVal+1] == (';' as u8) && indexVal < (data.len()-1)  
				if data[indexVal] == ('"' as u8) && data[indexVal+1] == (';' as u8) && indexVal < (data.len())  
				{
					io::println(fmt!(" 3. Going to break with indexVal is %?", indexVal));
					break ;
				}
				indexVal = indexVal + 1 ;
			}

			// if this condition is true then, the input CSS file doesn't have anything except <charset>  string
			if indexVal == data.len() {
				io::println("INSIDE PARSERUTILS_NEEDDATA error block");
				return PARSERUTILS_NEEDDATA;
			}
			io::println("OUTSIDE PARSERUTILS_BADPARAM error block");

			// Convert to MIB enum 
			unsafe {
				charset = self.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 

				charset == self.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16") ||

				charset == self.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{

				charset = 0;
			}
		}
		else
		{
			io::println("INSIDE ASCII no where, means ERROR");
		}
		PARSERUTILS_CHARSET_TRY_OK(@4)
	}


	pub fn css_charset_read_bom_or_charset(&mut self, data : ~[u8], mibenum : ~u16 ) -> parserutils_result {

		let mut err : parserutils_result ;
		let mut charset : u16  = 0;
		//let mut parser : @lpu = lpu();


		if (data.len()<4) {
			return PARSERUTILS_BADPARAM;
		}


		// Look for BOM 
		if (data[0] == 0x00 && data[1] == 0x00 && 
				data[2] == 0xFE && data[3] == 0xFF) {
			charset = self.parserutils_charset_mibenum_from_name(~"UTF-32BE");
		} else if (data[0] == 0xFF && data[1] == 0xFE &&
				data[2] == 0x00 && data[3] == 0x00) {
			charset = self.parserutils_charset_mibenum_from_name(~"UTF-32LE");
		} else if (data[0] == 0xFE && data[1] == 0xFF) {
			charset = self.parserutils_charset_mibenum_from_name(~"UTF-16BE");
		} else if (data[0] == 0xFF && data[1] == 0xFE) {
			charset = self.parserutils_charset_mibenum_from_name(~"UTF-16LE");
		} else if (data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF) {
			charset = self.parserutils_charset_mibenum_from_name(~"UTF-8");
		}

		if (charset!=0) {
			return PARSERUTILS_CHARSET_TRY_OK(@charset);
		}
		
		err = self.try_utf32_charset(copy data);
		// Sushanta
		//err = try_utf32_charset(data, parser);
		match(err) {
			PARSERUTILS_CHARSET_TRY_OK(x) => return err ,
			_ => {}	
		}

		err = self.try_utf16_charset(copy data);
		//err = try_utf16_charset(data,parser);
		match(err) {
			PARSERUTILS_CHARSET_TRY_OK(x) => return err ,
			_ => {}	
		}
		
		return self.try_ascii_compatible_charset(copy data);
	}


	pub fn css__charset_extract(&mut self,  data : ~[u8] ,
			mibenum : ~u16 , source : ~u32) -> parserutils_result {

		let mut err : parserutils_result;
		let mut charset : @u16 = @0;
		let mut src : @u32 = @0 ;

		if (data.len()==(0 as uint))  || mibenum==~(0 as u16) || source==~(0) {
			return PARSERUTILS_BADPARAM ;
		}

		// If the charset was dictated by the client, we've nothing to detect 
		if source==~4  {//CSS_CHARSET_DICTATED
			charset=@*mibenum ;
			return PARSERUTILS_CHARSET_EXT_OK((charset,@4));
		}

		// Look for a BOM and/or @charset 
		err = self.css_charset_read_bom_or_charset(data, copy ~*charset);
		match(err) {
			PARSERUTILS_CHARSET_TRY_OK(x) => {} ,
			_ => return PARSERUTILS_BADPARAM	
		}

		if charset!=@0 {
			//mibenum = charset;
			src = @3 ; // CSS_CHARSET_DOCUMENT;
			return PARSERUTILS_CHARSET_EXT_OK((charset,src));
		}

		// If we've already got a charset from the linking mechanism or 
		//  referring document, then we've nothing further to do 
		if source!=~0 /* CSS_CHARSET_DEFAULT */ {
			src=@*source;
			return PARSERUTILS_CHARSET_EXT_OK((charset,src));
		}

		// We've not yet found a charset, so use the default fallback 
		charset = @self.parserutils_charset_mibenum_from_name(~"UTF-8");

		if charset==@0 {
			return PARSERUTILS_BADENCODING ;
		}

		//mibenum = charset ;
		src = @0 ; // CSS_CHARSET_DEFAULT;

		return PARSERUTILS_CHARSET_EXT_OK((charset,src));
	}
}
		
	/*pub fn parserutils_inputstream_create(&mut self,enc: ~str,
		encsrc: u32 )-> parserutils_result
	{
		let s: @parserutils_inputstream;
		let mut pRslt: parserutils_result;
        let inputTemp: @parserutils_filter;
		let mut mibenumTemp: u16 = 0; 
		 
		


          pRslt = self.parserutils__filter_create(~"UTF-8");
         match(pRslt)
         {
         	PARSERUTILS_FILTER_CREATE_OK(parserutils_filter) => 
         	{
         		inputTemp= parserutils_filter;
         	},
         	_=>
         	{
         	// 	parserutils_buffer_destroy(s->public.utf8);
		        // parserutils_buffer_destroy(s->raw);
		        // alloc(s, 0, pw);
              	return pRslt;    
         	} 
         }


        let x=self.parserutils_charset_mibenum_from_name(copy enc);
        match(x)
        {
        	0=>{},
        	_=> {mibenumTemp = x;}
        }

        s=  @parserutils_inputstream
        {
        
        		utf8: ~[],	

				cursor: 0,		

				had_eof: false,
        
			raw: ~[] ,	

			done_first_chunk: false ,		 
					 

			mibenum: mibenumTemp,		
			encsrc: encsrc,		

			input:inputTemp ,	

			//csdetect: csdetect  

			
			
        };
       
		 
		if (s.mibenum == 0)
			{return PARSERUTILS_BADENCODING;}
	
        pRslt = self.parserutils__filter_setopt(s.input,
				copy enc);
        match(pRslt)
        {
        	PARSERUTILS_FILTER_CREATE_OK(Temp)=>{},
        	_=>
        	{
        		/*parserutils__filter_destroy(s->input);
				parserutils_buffer_destroy(s->public.utf8);
				parserutils_buffer_destroy(s->raw);
				alloc(s, 0, pw);
				return err;
				self.parserutils__filter_destroy(s.input);
				return pRslt;
        	}
        }
	
	return PARSERUTILS_INPUTSTREAM_CREATE_OK(s);
	}*/
//}
	/*pub fn parserutils_inputstream_destroy(&mut self,
		stream:@parserutils_inputstream)-> parserutils_result
	{
		self.parserutils__filter_destroy(stream.input);
		PARSERUTILS_GENERAL_OK
	}	*/
/**
 * Append data to an input stream at the end of raw data
 *
 * \param stream  Input stream to append data to
 * \param data    Data to append (in document charset), or NULL to flag EOF
 * \param len     Length, in bytes, of data
 * \return PARSERUTILS_OK on success, appropriate error otherwise
 */
	/*pub fn parserutils_inputstream_append(stream:@parserutils_inputstream, 
		data: @[u8])-> parserutils_result{
		if (data.len()==0) {
			stream.had_eof = true;
			return PARSERUTILS_GENERAL_OK;
		}
		stream.raw = append(stream.raw,data);
		return PARSERUTILS_GENERAL_OK;
	
	}*/

	// pub fn print_inputstream(&mut self, stream:@parserutils_inputstream)
	// {
	// 	io::println("printing");
	// 	io::println(fmt!("%?",stream.public.cursor));
	// 	io::println(fmt!("%?",stream.public.utf8));
	// 	io::println(fmt!("%?",stream.raw));
	// }
// *
//  * Insert data into utf8 part of stream at current location. current location is
//  * pointed by cursor position
//  *
//  * \param stream  Input stream to insert into
//  * \param data    Data to insert (UTF-8 encoded)
//  * \return PARSERUTILS_GENERAL_OK on success, appropriate error otherwise
 
   /* pub fn parserutils_inputstream_insert(stream:@parserutils_inputstream, data: @[u8])-> parserutils_result{
                
        let dataLen= data.len();
        
        let offset= stream.cursor;
        
        
        let utf8Len= stream.utf8.len();
		
		if ( dataLen ==0) {
			return PARSERUTILS_BADPARAM;
		}
        
        if (offset > utf8Len){//running past the allocated buffer
        	return PARSERUTILS_BADPARAM;
        }
		

		if (offset == utf8Len)//start writing from end of buffer
		{
			
			stream.utf8 = append(stream.utf8,data);
			return	   PARSERUTILS_GENERAL_OK;
		}
        //offset is less than bufflen,i.e., points to middle of utf8 data
		let mut Iter= 0;
		
        while(Iter < dataLen)
        {
        	stream.utf8.insert(Iter  + offset,data[Iter]);
        	Iter += 1;
        }
        
       return PARSERUTILS_GENERAL_OK; 
	}
*/
	
	/*pub fn parserutils_inputstream_read_charset(&mut self,
		stream:@parserutils_inputstream,mut source:@mut u32)-> Option<~str>{
		*source = stream.encsrc;
		if ((*source) == 0)
		{
			return Some(~"UTF-8")
		}
		return self.parserutils_charset_mibenum_to_name(stream.mibenum);
	}*/


	/*pub fn parserutils_inputstream_change_charset(&mut self,stream:@parserutils_inputstream, 
		enc:~str, source:u32)-> parserutils_result{

		let pRslt: parserutils_result;
    	if (enc.len() == 0)
			{return PARSERUTILS_BADPARAM;}

		if (stream.done_first_chunk)
			{return PARSERUTILS_INVALID;}
        io::println(~"Hi"+ copy enc); 
		let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(copy enc);
		if mibenum_search_result==0 {
			io::println("bad param ##############");
			return PARSERUTILS_BADPARAM;
		}
        io::println("Hello"); 
		let mibenum = mibenum_search_result;

		// Ensure filter is using the correct encoding 
		//params.encoding.name = enc;
		pRslt = self.parserutils__filter_setopt(stream.input,enc);
		match(pRslt)
		{
			PARSERUTILS_FILTER_CREATE_OK(Temp) => {},
			_=>{io::println("return ##############");return pRslt;}
		}
		

		//Finally, replace the current settings 
		stream.mibenum = mibenum;
		stream.encsrc = source;

		return PARSERUTILS_GENERAL_OK;
	}*/


	/*pub fn parserutils_inputstream_strip_bom(&mut self,mibenum:@mut u16, mut buffer:~[u8])-> parserutils_result
	{

		let UTF32_BOM_LEN =(4);
		let UTF16_BOM_LEN =(2);
		let UTF8_BOM_LEN  =(3);
		let totype : Option<~str> = self.parserutils_charset_mibenum_to_name(*mibenum) ;
		if totype.is_none()  {
			return PARSERUTILS_BADPARAM;
		}	
		let Res:~str= totype.get();
		match(Res)
		{
			~"UTF-8"    =>{
				if (buffer.len() >= UTF8_BOM_LEN) &&
				buffer[0] == 0xEF &&
				buffer[1] == 0xBB && 
				buffer[2] == 0xBF{
					buffer= slice(buffer,UTF8_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				} 

			},
			~"UTF-16"   =>{
				
				let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(~"UTF-16BE");
				if mibenum_search_result==0 {
					return PARSERUTILS_BADPARAM;
				}

				*mibenum = mibenum_search_result;
				if (buffer.len() >= UTF16_BOM_LEN) {
					if (buffer[0] == 0xFE && 
							buffer[1] == 0xFF) {
					buffer= slice(buffer,UTF16_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
					
					} else if (buffer[0] == 0xFF && 
						buffer[1] == 0xFE) {
					
					let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(~"UTF-16LE");
					if mibenum_search_result==0 {
						return PARSERUTILS_BADPARAM;
					}

					*mibenum = mibenum_search_result;
					buffer= slice(buffer,UTF16_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
					
					}
				}
			},
			~"UTF-16BE" =>{
				if (buffer.len() >= UTF16_BOM_LEN &&
				buffer[0] == 0xFE &&
				buffer[1] == 0xFF) {
					buffer= slice(buffer,UTF16_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				}
			},
			~"UTF-16LE" =>{
				if (buffer.len() >= UTF16_BOM_LEN &&
				buffer[0] == 0xFF &&
				buffer[1] == 0xFE) {
					buffer= slice(buffer,UTF16_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				}
			},
			~"UTF-32"   =>{
				
				let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(~"UTF-32BE");
				if mibenum_search_result==0 {
					return PARSERUTILS_BADPARAM;
				}

				*mibenum = mibenum_search_result;
		if (buffer.len() >= UTF32_BOM_LEN) {
			if (buffer[0] == 0x00 && 
					buffer[1] == 0x00 &&
					buffer[2] == 0xFE &&
					buffer[3] == 0xFF) {
						buffer= slice(buffer,UTF32_BOM_LEN-1,buffer.len()-1).to_owned();
						return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				
					} 
					else if (buffer[0] == 0xFF && 
					buffer[1] == 0xFE &&
					buffer[2] == 0x00 &&
					buffer[3] == 0x00) {
				
						let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(~"UTF-32LE");
						if mibenum_search_result==0{
							return PARSERUTILS_BADPARAM;
						}

				*mibenum = mibenum_search_result;
				buffer= slice(buffer,UTF32_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				
			}
		}
			},
			~"UTF-32BE" =>{
				if (buffer.len() >= UTF32_BOM_LEN &&
				buffer[0] == 0x00 &&
				buffer[1] == 0x00 &&
				buffer[2] == 0xFE &&
				buffer[3] == 0xFF) {
                  	buffer= slice(buffer,UTF32_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				}
			},
			~"UTF-32LE" =>{
				if (buffer.len() >= UTF32_BOM_LEN &&
				buffer[0] == 0xFF &&
				buffer[1] == 0xFE &&
				buffer[2] == 0x00 &&
				buffer[3] == 0x00) {
					buffer= slice(buffer,UTF32_BOM_LEN-1,buffer.len()-1).to_owned();
					return PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer);
				}
			},
			_=>{
				io::println(~"RES="+Res);
			}
		}
		return PARSERUTILS_GENERAL_OK;
	}
*/

	/*pub fn parserutils_inputstream_refill_buffer(&mut self,
		stream:@parserutils_inputstream)-> parserutils_result{
		let mut pRslt:parserutils_result = PARSERUTILS_GENERAL_OK ;
		if (stream.done_first_chunk == false) {
			//parserutils_filter_optparams params;

			 // If there is a charset detection routine, give it an 
		 	// * opportunity to override any charset specified when the
		 	// * inputstream was created 

				//pRslt = (*stream.csdetect)(copy stream.raw, ~stream.mibenum, ~stream.encsrc);
				pRslt = self.css__charset_extract(copy stream.raw ,~stream.mibenum , ~stream.encsrc) ;
				match(pRslt)
				{
					PARSERUTILS_GENERAL_OK=>{
						io::println("ok result");//sandeep
					},
					PARSERUTILS_NEEDDATA=>{
						if stream.had_eof == false{
							return pRslt;
						}
					},
					_=> return pRslt
				}
				

					//  We don't have enough data to detect the 
					// * input encoding, but we're not going to get 
				 // 	* any more as we've been notified of EOF. 
				 // 	* Therefore, leave the encoding alone
					// * so that any charset specified when the
				 // 	* inputstream was created will be preserved.
				 // 	* If there was no charset specified, then
				 // 	* we'll default to UTF-8, below 
					
			

			 // Default to UTF-8 if there is still no encoding information 
		 	// * We'll do this if there was no encoding specified up-front
		 	// * and:
		 	// *    1) there was no charset detection routine
		 	// * or 2) there was insufficient data for the charset 
		 	// *       detection routine to detect an encoding
		 	
			if (stream.mibenum == 0) {
				
				let mibenum_search_result = self.parserutils_charset_mibenum_from_name(~"UTF-8");
				if mibenum_search_result==0 {
					io::println("bad Param"); //sandeep
					return PARSERUTILS_BADPARAM;
				}

				stream.mibenum = mibenum_search_result;
				stream.encsrc = 0;
			}

			if (stream.mibenum == 0)
				{
					fail!();
				}
			let mut temp:~[u8] = ~[];
			temp <-> stream.raw;	
			// Strip any BOM, and update encoding as appropriate 
			pRslt = self.parserutils_inputstream_strip_bom( @mut stream.mibenum, temp);
			match(copy pRslt)
			{
				PARSERUTILS_INPUTSTREAM_STRIP_BOM_OK(buffer) => {
					temp = buffer;
					temp <-> stream.raw;
				}
				_=>{return pRslt;}
			}
			

			// * Ensure filter is using the correct encoding 
			

            let totype : Option<~str> = self.parserutils_charset_mibenum_to_name(stream.mibenum) ;
		
			if totype.is_none() {
				io::println("bad param");//sandeep
				return PARSERUTILS_BADPARAM;
			}


			pRslt = self.parserutils__filter_setopt(stream.input,
					totype.get());
			io::println("__filter_setopt");//sandeep
			match(pRslt)
			{
				PARSERUTILS_FILTER_CREATE_OK(input)=>{},
				_=>{
					return pRslt;
				}
			}

			stream.done_first_chunk = true;
		}
		io::println("line______&");//sandeep
			 // Work out how to perform the buffer fill 
		if (stream.cursor == stream.utf8.len()) {
		 // Cursor's at the end, so simply reuse the entire buffer 
		
		stream.utf8= ~[];
		} else {
			 // Cursor's not at the end, so shift data after cursor to the
		 	// * bottom of the buffer. If the buffer's still over half full, 
		 	// 	* extend it. 
		 	stream.utf8=slice(stream.utf8,stream.cursor as uint,stream.utf8.len()).to_owned();
			
			
		}
		//self.print_inputstream(stream);
		io::println("line______&&");//sandeep	
		let utf8:~[u8]= ~[];
		let mut processedLen: uint;
		let raw:@mut ~[u8] = @mut ~[];
		*raw <-> stream.raw;
		let raw_length = raw.len();
        io::println("line______&&");
		 // Try to fill utf8 buffer from the raw data 
		pRslt = self.parserutils__filter_process_chunk(copy stream.input,copy *raw);
		*raw <-> stream.raw;
		io::println("line______&&**");
		match(copy pRslt)
		{
			PARSERUTILS_FILTER_PROCESS_CHUNK_OK((len,data)) =>{
				stream.utf8=append(copy stream.utf8,data);
				processedLen=len as uint;
			}
			_=>return pRslt
		}
io::println("line______&&&");//sandeep
		*raw <-> stream.raw;
		//let mut Iter = 0;
		//while(Iter < utf8.len())
		{
			//stream.utf8.append(utf8);
		}

		//  _NOMEM implies that there's more input to read than available space
	 // 	* in the utf8 buffer. That's fine, so we'll ignore that error. 
		

		// /* Remove the raw data we've processed from the raw buffer 
        stream.raw= slice(stream.raw,processedLen,stream.raw.len()-1).to_owned();
		
		 // Fix up the utf8 buffer information 
		

		 // Finally, fix up the cursor 
		io::println("line______&##");//sandeep
		stream.cursor = 0;

			return PARSERUTILS_GENERAL_OK;
		}*/
        
		/*pub fn IS_ASCII(&mut self,data:u8)-> bool
		{
			return (((data) & 0x80) == 0);
		}*/

		/*pub fn parserutils_inputstream_peek_slow(&mut self,	stream:@parserutils_inputstream, offset:uint , ptr:@mut~[u8], length:@mut uint )-> parserutils_result{
            io::println("line1_____");//sandeep
            let len:uint;
            let mut pRslt:parserutils_result;
			if (stream.raw.len() == 0) {
		 // No more data to be had 
				if(stream.had_eof){
					return PARSERUTILS_EOF;
				} else{
					return PARSERUTILS_NEEDDATA;
				}

			}

			 // Refill utf8 buffer from raw buffer 
			pRslt = self.parserutils_inputstream_refill_buffer(stream);
			//self.print_inputstream(stream);
			match(pRslt)
			{
				PARSERUTILS_GENERAL_OK => {},
				_=> return pRslt
			}

			 // Refill may have succeeded, but not actually produced any new data 
			if (stream.cursor + offset == stream.utf8.len())
				{
					io::println("No data produced");//sandeep
					return PARSERUTILS_NEEDDATA;
				}
             io::println("line______#");//sandeep
			 // Now try the read 
			if (self.IS_ASCII(stream.utf8[stream.cursor + offset])) {
				len = 1;
			} else {
				let Temp= self.parserutils_charset_utf8_char_byte_length(slice
					(stream.utf8,stream.cursor as uint+ offset as uint,stream.utf8.len()).to_owned());
				match(Temp)
				{
					None=>{
					 	pRslt = PARSERUTILS_BADPARAM;
					 	return pRslt;
					},
					Some(x)=> {
						len=(x as uint);
					}
				}
		
        		match(pRslt)
        		{
        			PARSERUTILS_GENERAL_OK => {},
        			PARSERUTILS_NEEDDATA   => {
        				if(stream.had_eof){
							return PARSERUTILS_EOF;
						} else{
							return PARSERUTILS_NEEDDATA;
						}
        			},
        			_=> return pRslt
        		}

		
		}
			io::println("line2_____");//sandeep
			(*length) = len;
			(*ptr) = slice(stream.utf8, stream.cursor + offset,stream.utf8.len()).to_owned();

			return PARSERUTILS_GENERAL_OK;
		}
*/
//}

	/*pub fn parserutils_inputstream_advance(&mut self,stream:@parserutils_inputstream, bytes:uint ){
		
		if (bytes as uint> stream.utf8.len() - stream.cursor as uint)
			{
				fail!();
			}

		if (stream.cursor as uint == stream.utf8.len())
			{
				return;
			}

		stream.cursor += bytes;
	}
*/
	/**
 * Look at the character in the stream that starts at 
 * offset bytes from the cursor
 *
 * \param stream  Stream to look in
 * \param offset  Byte offset of start of character
 * \param ptr     Pointer to location to receive pointer to character data
 * \param length  Pointer to location to receive character length (in bytes)
 * \return PARSERUTILS_OK on success, 
 *                    _NEEDDATA on reaching the end of available input,
 *                    _EOF on reaching the end of all input,
 *                    _BADENCODING if the input cannot be decoded,
 *                    _NOMEM on memory exhaustion,
 *                    _BADPARM if bad parameters are passed.
 *
 * Once the character pointed to by the result of this call has been advanced
 * past (i.e. parserutils_inputstream_advance has caused the stream cursor to 
 * pass over the character), then no guarantee is made as to the validity of 
 * the data pointed to. Thus, any attempt to dereference the pointer after 
 * advancing past the data it points to is a bug.
 */
 
    /*pub fn parserutils_inputstream_peek(&mut self,stream:@parserutils_inputstream, offset: uint, ptr:@mut ~[u8], length:@mut uint) 
    -> parserutils_result 
    {
		let mut pRslt: parserutils_result = PARSERUTILS_GENERAL_OK;
		//const parserutils_buffer *utf8;
		let utf8_data:@mut~[u8]=@mut copy stream.utf8;
		let mut len :uint;
		
		let mut utf8_len = stream.utf8.len();;
		let mut off = stream.cursor + offset;		

		if (off < utf8_len) {
			if (self.IS_ASCII(utf8_data[off])) {
				// Early exit for ASCII case 
				(*length) = 1;
				(*ptr) = slice(*utf8_data,off as uint,utf8_len).to_owned();
				return PARSERUTILS_GENERAL_OK;
			} else {
				match(self.parserutils_charset_utf8_char_byte_length(slice(*utf8_data, off as uint,utf8_len).to_owned()))
				{
					None => {
					 	return PARSERUTILS_BADPARAM;
					 	
					},
					Some(x) => {
						len=x as uint;
						(*length) = len;
					    (*ptr) = slice(*utf8_data,off as uint,utf8_len).to_owned();
					    return PARSERUTILS_GENERAL_OK;
					}
				}
				
				
			}
		}


		if (off != utf8_len)
		{
			match(pRslt)
			{
				PARSERUTILS_NEEDDATA => {

				},
				_=>{
					io::println(fmt!("%?", pRslt));
					fail!();
				}
			}
		}

		return self.parserutils_inputstream_peek_slow(stream, offset, ptr, length);
	}*/
//}



pub fn lpu() -> @mut lpu {
	let mut new_lpu = @mut lpu {
		canonical_name_list : ~[],
		mut mibenum_map : ~LinearMap::new(),
		mut alias_map : ~LinearMap::new()
	};

	new_lpu.read_aliases();
	new_lpu
}