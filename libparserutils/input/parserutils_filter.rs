use std::arc;

use charset::aliases::*;
use input::riconv;
use utils::errors::*;

pub struct filter {
    int_enc: u16,               // The internal encoding
    encoding : u16,
    iconv_h : u64,
    instance: arc::ARC<~alias>
}

impl filter {

    pub fn filter_set_encoding(&mut self, enc : ~str) -> parserutils_error {

        if enc.len()==0 {
            return PARSERUTILS_BADPARM;
        }

        let mibenum_search_result  = arc::get(&self.instance).parserutils_charset_mibenum_from_name(enc);
        if mibenum_search_result==0 {
            return PARSERUTILS_BADPARM;
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

        let totype: Option<~str> = arc::get(&self.instance).parserutils_charset_mibenum_to_name(self.int_enc) ;
        let fromtype: Option<~str> = arc::get(&self.instance).parserutils_charset_mibenum_to_name(mibenum) ;
        if totype.is_none() || fromtype.is_none() {
            return PARSERUTILS_BADPARM;
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
            PARSERUTILS_BADPARM
        }   
    }

    pub fn parserutils__filter_reset(&mut self ) -> parserutils_error {
        if riconv::riconv_initialized(self.iconv_h) {
            riconv::safe_riconv(self.iconv_h, ~[]);
            return PARSERUTILS_OK;
        }
        else {
            PARSERUTILS_BADPARM
        }   
    }

    pub fn parserutils__filter_process_chunk(&mut self, inbuf : &[u8] ) -> (parserutils_error, ~[u8], u64) {
                
        let (outbuf, len_processed, err_state) = riconv::safe_riconv(self.iconv_h, inbuf);
        
        let mut status : parserutils_error ;

        if len_processed==0 {
            if err_state==1 {
                status = PARSERUTILS_NOMEM;
            } 
            else if err_state==2 {
                status = PARSERUTILS_BADPARM;
            }
            else {
                status = PARSERUTILS_INVALID;
            }
        }
        else {
            status = PARSERUTILS_OK;
        }
        (status, outbuf, len_processed)
    }
}

pub fn parserutils_filter(mut existing_instance: arc::ARC<~alias> , int_enc: ~str) -> (Option<~filter> , parserutils_error) {

    let mut filter = ~filter{
        int_enc: arc::get(&existing_instance).parserutils_charset_mibenum_from_name(int_enc),               // The internal encoding
        encoding : 0,
        iconv_h : riconv::riconv_initialize(),
        instance : existing_instance.clone()
    };
    match filter.filter_set_encoding(~"UTF-8") {
        PARSERUTILS_OK => {
            return (Some(filter) , PARSERUTILS_OK );
        },
        x => {return (None , x);}
    }
}