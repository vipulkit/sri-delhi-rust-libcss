use core::vec::*;
use std::arc;

use charset::aliases::*;
use input::parserutils_filter::*;
use utils::error::*;

pub type  parserutils_charset_detect_func =  
    ~extern fn(data: &~[u8], mibenum:u16, source:int, arc:arc::ARC<~alias>) -> (Option<u16>, Option<int>, parserutils_error);

pub struct inputstream {
    utf8: ~[u8],        // Buffer containing UTF-8 data 
    cursor: uint,       // Byte offset of current position 
    had_eof: bool,      // Whether EOF has been reached 
    raw: ~[u8],         // Buffer containing raw data 
    done_first_chunk: bool,     // Whether the first chunk has been processed 
    mibenum: u16,       // MIB enum for charset, or 0
    encsrc: int,     // Charset source
    input: ~filter, // Charset conversion filter
    csdetect: Option<parserutils_charset_detect_func>
}

pub fn inputstream(encoding: Option<~str>, charset_src: Option<int>, csdetect_instance: Option<parserutils_charset_detect_func>) ->  (Option<~inputstream> , parserutils_error) {

    let mut stream: ~inputstream;
    let mut stream_encoding : ~str;
    let mut stream_charset_src : int;

    if (encoding.is_some() && charset_src.is_some()) {
        stream_encoding = encoding.unwrap();
        stream_charset_src = charset_src.unwrap();
    }
    else {
        stream_encoding = ~"UTF-8";
        stream_charset_src = 0;
    }
    match parserutils_filter(alias() , copy stream_encoding) {
        (x,PARSERUTILS_OK) =>{
            let mut filter_instance = x.unwrap(); 
            stream = ~inputstream {
                utf8: ~[],
                cursor: 0,
                had_eof: false,
                raw: ~[],
                done_first_chunk: false,
                mibenum: arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(copy stream_encoding),
                encsrc: stream_charset_src,
                input: filter_instance,
                csdetect: csdetect_instance
            };
        },
        
        (_ , y) => return (None , y)
    }
    // functionality of inputstream_create
    if stream.mibenum==0 {
        return(None , PARSERUTILS_BADENCODING);
    }
    (Some(stream) , PARSERUTILS_OK)
}

impl inputstream {

    pub fn parserutils_inputstream_destroy(&mut self)-> parserutils_error {
        self.input.parserutils__filter_destroy();
        self.utf8 = ~[] ;
        self.raw = ~[] ;
        self.cursor = 0 ;
        self.had_eof = false ;
        self.done_first_chunk = false ;
        self.mibenum = 0 ;
        self.encsrc =0 ;
        self.csdetect = None;
        PARSERUTILS_OK
    }

    pub fn parserutils_inputstream_append(&mut self, data: ~[u8]) -> parserutils_error {
        if data.len()==0 {
            self.had_eof = true;
            return PARSERUTILS_OK;
        }
        self.raw += data;
        PARSERUTILS_OK
    }

    pub fn parserutils_inputstream_insert(&mut self, data: ~[u8])-> parserutils_error {
        
        if data.len()==0 && (self.utf8.len() < self.cursor) {
            return PARSERUTILS_BADPARAM;
        }

        let temp = self.utf8.slice(self.cursor,self.utf8.len()).to_owned();
        self.utf8.truncate(self.cursor);
        self.utf8 += data ;
        self.utf8 += temp ;
        PARSERUTILS_OK
    }

    pub fn parserutils_inputstream_read_charset(&mut self)-> (Option<~str>,int) {
        
        (arc::get(&self.input.instance).parserutils_charset_mibenum_to_name(self.mibenum),self.encsrc)
    }

    pub fn parserutils_inputstream_change_charset(&mut self, enc:~str, source:int)-> parserutils_error {

        if enc.len() == 0 {
            return PARSERUTILS_BADPARAM;
        }

        if self.done_first_chunk {
            return PARSERUTILS_INVALID;
        }
        
        self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(copy enc);
        if self.mibenum==0 {
            return PARSERUTILS_BADPARAM;
        }
        
        // Ensure filter is using the correct encoding 
        let filter_set_encoding_result = self.input.filter_set_encoding(enc);

        match(filter_set_encoding_result) {
            PARSERUTILS_OK => { 
                self.encsrc = source;
            },
            _ => {}
        }

        filter_set_encoding_result
    }

    pub fn parserutils_inputstream_strip_bom(&mut self)-> parserutils_error {

        let UTF32_BOM_LEN =4;
        let UTF16_BOM_LEN =2;
        let UTF8_BOM_LEN  =3;

        let totype : Option<~str> = arc::get(&self.input.instance).parserutils_charset_mibenum_to_name(self.mibenum);

        if totype.is_none() {
            return PARSERUTILS_BADPARAM;
        }

        let result: ~str= totype.unwrap();
        io::println(result);
        match(result) {
            ~"UTF-8" => {
                if (self.raw.len() >= UTF8_BOM_LEN) && self.raw[0] == 0xEF && self.raw[1] == 0xBB && self.raw[2] == 0xBF {
                    self.raw= slice(self.raw,UTF8_BOM_LEN,self.raw.len()).to_owned();
                    return PARSERUTILS_OK;
                } 
            },
            ~"UTF-32" => {
                self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-32BE");
                if self.mibenum==0 {
                    return PARSERUTILS_BADPARAM;
                }

                if self.raw.len() >= UTF32_BOM_LEN {
                    if self.raw[0] == 0x00 && self.raw[1] == 0x00 && self.raw[2] == 0xFE && self.raw[3] == 0xFF {
                        self.raw= slice(self.raw,UTF32_BOM_LEN,self.raw.len()).to_owned();
                        return PARSERUTILS_OK;
                    }
                    else if self.raw[0] == 0xFF && self.raw[1] == 0xFE && self.raw[2] == 0x00 && self.raw[3] == 0x00 {
                        self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-32LE");
                        if self.mibenum==0 {
                            return PARSERUTILS_BADPARAM;
                        }
                        
                        self.raw= slice(self.raw,UTF32_BOM_LEN,self.raw.len()).to_owned();
                        return PARSERUTILS_OK;
                    }
                }
            },
            ~"UTF-16" => {
                self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-16BE");
                
                if self.mibenum==0 {
                    return PARSERUTILS_BADPARAM;
                }

                if self.raw.len() >= UTF16_BOM_LEN {
                    if self.raw[0] == 0xFE && self.raw[1] == 0xFF {
                        self.raw= slice(self.raw,UTF16_BOM_LEN,self.raw.len()).to_owned();
                        return PARSERUTILS_OK;
                    }
                    else if self.raw[0] == 0xFF && self.raw[1] == 0xFE {
                        self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-16LE");
                        if self.mibenum == 0 {
                            return PARSERUTILS_BADPARAM;
                        }

                        self.raw= slice(self.raw,UTF16_BOM_LEN,self.raw.len()).to_owned();
                        return PARSERUTILS_OK;
                    }
                }
            },
            ~"UTF-16BE" => {
                if self.raw.len() >= UTF16_BOM_LEN && self.raw[0] == 0xFE && self.raw[1] == 0xFF {
                    self.raw= slice(self.raw,UTF16_BOM_LEN,self.raw.len()).to_owned();
                    return PARSERUTILS_OK;
                }
            },
            ~"UTF-16LE" => {
                if self.raw.len() >= UTF16_BOM_LEN && self.raw[0] == 0xFF && self.raw[1] == 0xFE {
                    

                    self.raw= slice(self.raw,UTF16_BOM_LEN,self.raw.len()).to_owned();
                    return PARSERUTILS_OK;
                }
            },
            
            ~"UTF-32BE" => {
                if self.raw.len() >= UTF32_BOM_LEN && self.raw[0] == 0x00 && self.raw[1] == 0x00 && self.raw[2] == 0xFE && self.raw[3] == 0xFF {
                    self.raw= slice(self.raw,UTF32_BOM_LEN,self.raw.len()).to_owned();
                    return PARSERUTILS_OK;
                }
            },
            ~"UTF-32LE" => {
                if self.raw.len() >= UTF32_BOM_LEN && self.raw[0] == 0xFF && self.raw[1] == 0xFE && self.raw[2] == 0x00 && self.raw[3] == 0x00 {
                    self.raw= slice(self.raw,UTF32_BOM_LEN,self.raw.len()).to_owned();
                    return PARSERUTILS_OK;
                }
            },
            _=>{
                
            }
        }
        return PARSERUTILS_OK;
    }

    pub fn IS_ASCII(&mut self , data:u8) -> bool {
        ((data & 0x80) == 0)
    }

    pub fn parserutils_inputstream_advance(&mut self, bytes:uint) -> parserutils_error {
        if bytes > (self.utf8.len() - self.cursor) {
                return PARSERUTILS_BADPARAM;
        }

        if (self.cursor == self.utf8.len()) {
                return PARSERUTILS_OK;
        }

        self.cursor += bytes;
        PARSERUTILS_OK
    }

    pub fn parserutils_inputstream_refill_buffer(&mut self) -> parserutils_error {
        
        if (self.done_first_chunk == false) {

            match(self.csdetect) {
                Some(copy f) => {
                    let (charsetOption,srcOption,error)= (*f)(&self.raw, self.mibenum, self.encsrc, self.input.instance.clone());

                    match error {
                        PARSERUTILS_OK => {
                            self.mibenum= charsetOption.unwrap();
                            self.encsrc = srcOption.unwrap();
                        },
                        x => match x {
                                PARSERUTILS_NEEDDATA => {   
                                                            if self.had_eof == false {
                                                                return x;
                                                            }
                                                        },
                                                    _ => return x   
                        }
                    }
                },
                None => {}
            }   
            if (self.mibenum == 0) {
                self.mibenum = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-8");
                if self.mibenum == 0 {
                    return PARSERUTILS_BADPARAM;
                }
                self.encsrc = 0;
            }

            match(self.parserutils_inputstream_strip_bom()) {
                PARSERUTILS_OK => {
                    //self.done_first_chunk = true;
                },
                _ => {
                    return PARSERUTILS_BADPARAM;
                } 
            }

            match arc::get(&self.input.instance).parserutils_charset_mibenum_to_name(self.mibenum) {
                None => { 
                    return PARSERUTILS_BADENCODING
                    },
                Some(x) => {
                    match self.input.filter_set_encoding(x) {
                        PARSERUTILS_OK => {
                            //self.done_first_chunk = true; 
                        },
                        _ => {
                         return PARSERUTILS_BADENCODING;
                        }
                    }
                }
            }
        }
        
        // Discard the data in the UTF-8 buffer before the cursor location
        if self.cursor == self.utf8.len() {
            self.utf8 = ~[] ;
        } 
        else {
            self.utf8=slice(self.utf8,self.cursor,self.utf8.len()).to_owned();
        }
        self.cursor = 0;
       
      
         // Try to fill utf8 buffer from the raw data
        let mut processedLen:uint;
        match(self.input.parserutils__filter_process_chunk(copy self.raw)) { //TODO :: remove copy
            (processed_chunk , PARSERUTILS_OK) => {
                 
                    if (!self.done_first_chunk) {
                        self.done_first_chunk = true;
                        if processed_chunk.outbuf[0]== 0xFF && processed_chunk.outbuf[1]== 0xFE && processed_chunk.outbuf[2]== 0x00 && processed_chunk.outbuf[3]== 0x00{
                            self.utf8 += slice(processed_chunk.outbuf,4,processed_chunk.outbuf.len()).to_owned();
                        }
                        else if processed_chunk.outbuf[0]== 0xFF && processed_chunk.outbuf[1]== 0xFE {
                            self.utf8 += slice(processed_chunk.outbuf,2,processed_chunk.outbuf.len()).to_owned();
                        }
                        else {
                            self.utf8 += processed_chunk.outbuf;
                        }
                }
                else {
                    self.utf8 += processed_chunk.outbuf;
                }
                //self.utf8 += processed_chunk.outbuf;
                processedLen = processed_chunk.len_processed as uint
            },
            (_ , y) => {
                return y
            }
        }

        self.raw= slice(self.raw,processedLen,self.raw.len()).to_owned();
        return PARSERUTILS_OK;
    }


    pub fn parserutils_inputstream_peek_slow(&mut self , offset: uint)-> (Option<(~[u8],uint)>,parserutils_error) {
            
        let len: uint;

        if self.raw.len() == 0 {
            if self.had_eof {
                return (None,PARSERUTILS_EOF);
            }
            else {
                return (None,PARSERUTILS_NEEDDATA);
            }
        }

         // Refill utf8 buffer from raw buffer 
        match(self.parserutils_inputstream_refill_buffer()) {
            PARSERUTILS_BADPARAM => {return (None, PARSERUTILS_BADPARAM);},
            PARSERUTILS_BADENCODING => {return (None, PARSERUTILS_BADENCODING);},
            PARSERUTILS_NEEDDATA => {return (None, PARSERUTILS_NEEDDATA);},
            _ => {}
        }

         // Refill may have succeeded, but not actually produced any new data 
        if self.cursor + offset == self.utf8.len() {                    
            return (None,PARSERUTILS_NEEDDATA);
        }
        
        let requested_data = slice(self.utf8, self.cursor + offset, self.utf8.len()).to_owned();
         // Now try the read 
        if self.IS_ASCII(self.utf8[self.cursor + offset]) {
            len = 1;
        }

        else {
            
            match(filter::parserutils_charset_utf8_char_byte_length(requested_data)) {
                None=>{
                    return (None, PARSERUTILS_BADPARAM);
                     
                },
                Some(l)=> {
                    len=l as uint;
                }
            }
        }

        return (Some((requested_data,len)),PARSERUTILS_OK);
    }

 
    pub fn parserutils_inputstream_peek(&mut self, offset: uint)-> (Option<(~[u8],uint)>,parserutils_error) {
        
        let mut ptr:~[u8];
        let mut len :uint;
        
        if self.cursor + offset < self.utf8.len() {

            if self.IS_ASCII(self.utf8[self.cursor + offset]) {
                ptr = slice(self.utf8, self.cursor + offset, self.utf8.len()).to_owned();
                // ascii char length is 1
                return (Some((ptr ,1)) , PARSERUTILS_OK);
            }
            else {
                ptr = slice(self.utf8, self.cursor + offset, self.utf8.len()).to_owned();
                
                match(filter::parserutils_charset_utf8_char_byte_length(ptr)) {
                    None=>{
                        return (None, PARSERUTILS_BADPARAM);
                    },
                    Some(l)=> {
                        len=l as uint;
                    }
                }
                return(Some((ptr , len)) , PARSERUTILS_OK);
            }
        }
        return self.parserutils_inputstream_peek_slow(offset);
    }
}



