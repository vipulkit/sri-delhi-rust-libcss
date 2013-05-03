#[link(name = "parserutils_filter", vers = "0.1")];
#[crate_type = "lib"];

extern mod parserutils;
extern mod std;
extern mod riconv;

use parserutils::*;
use std::arc;

pub struct lpu_filter {
	int_enc: u16,               // The internal encoding
	encoding : u16,
	iconv_h : u64,
	lpu_instance: arc::ARC<~lpu>
}

impl lpu_filter {

	pub fn filter_set_encoding(&mut self, enc : ~str) -> parserutils_error {

		if enc.len()==0 {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum_search_result  = arc::get(&self.lpu_instance).parserutils_charset_mibenum_from_name(enc);
		if mibenum_search_result==0 {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum = mibenum_search_result;
		// Exit early if we're already using this encoding 
		if self.encoding==mibenum {
			return PARSERUTILS_OK;
		}

		if riconv::riconv_initialized(self.iconv_h) {
			riconv::safe_riconv_close(self.iconv_h);
			self.iconv_h=riconv::riconv_initialize();
		}

		let totype: Option<~str> = arc::get(&self.lpu_instance).parserutils_charset_mibenum_to_name(self.int_enc) ;
		let fromtype: Option<~str> = arc::get(&self.lpu_instance).parserutils_charset_mibenum_to_name(mibenum) ;
		if totype.is_none() || fromtype.is_none() {
			return PARSERUTILS_BADPARAM;
		}

		self.iconv_h = riconv::safe_riconv_open(totype.unwrap(),fromtype.unwrap());
		if (!riconv::riconv_initialized(self.iconv_h)) {

			return PARSERUTILS_BADENCODING;
		}

		self.encoding = mibenum;
		PARSERUTILS_OK
	}

	pub fn filter_set_defaults(&mut self ) -> parserutils_error {
		self.encoding = 0;
		self.filter_set_encoding(~"UTF-8")
	}

	pub fn parserutils_charset_utf8_char_byte_length(s: &[u8]) -> Option<u8> {
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

	pub fn parserutils__filter_destroy(&mut self) -> parserutils_error {
		if riconv::riconv_initialized(self.iconv_h) {
			riconv::safe_riconv_close(self.iconv_h);
			self.iconv_h=riconv::riconv_initialize();
			PARSERUTILS_OK
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

	pub fn parserutils__filter_reset(&mut self ) -> parserutils_error {
		if riconv::riconv_initialized(self.iconv_h) {
			riconv::safe_riconv(self.iconv_h, ~[]);
			return PARSERUTILS_OK;
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

	pub fn parserutils__filter_process_chunk(&mut self, inbuf : ~[u8] ) -> (~riconv::chunk_result, parserutils_error) {
				
		let iconv_result = riconv::safe_riconv(self.iconv_h, inbuf);
		let mut status : parserutils_error ;

		if iconv_result.len_processed==0 {
			if iconv_result.err_state==1 {
				status = PARSERUTILS_NOMEM;
			} 
			else if iconv_result.err_state==2 {
				status = PARSERUTILS_BADPARAM;
			}
			else {
				status = PARSERUTILS_ICONV_ERROR;
			}
		}
		else {
			status = PARSERUTILS_OK;
		}
		(iconv_result, status)
	}
}

pub fn lpu_filter(mut existing_lpu_instance: arc::ARC<~lpu> , int_enc: ~str) -> (Option<~lpu_filter> , parserutils_error) {

	let mut filter = ~lpu_filter{
		int_enc: arc::get(&existing_lpu_instance).parserutils_charset_mibenum_from_name(int_enc),               // The internal encoding
		encoding : 0,
		iconv_h : riconv::riconv_initialize(),
		lpu_instance : existing_lpu_instance.clone()
	};
	match filter.filter_set_encoding(~"UTF-8") {
		PARSERUTILS_OK => {
			return (Some(filter) , PARSERUTILS_OK );
		},
		x => {return (None , x);}
	}
}