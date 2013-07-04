extern mod extra;
use std::vec::*;
use extra::arc;

use charset::aliases::*;
use input::parserutils_filter::*;
use utils::errors::*;

pub type  parserutils_charset_detect_func =  
    ~extern fn(data: &[u8], mibenum:u16, source:int, arc:arc::ARC<~alias>) -> (Option<u16>, Option<int>, parserutils_error);

pub struct inputstream {
    utf8: ~[u8],        // Buffer containing UTF-8 data 
    cursor: uint,       // Byte offset of current position 
    had_eof: bool,      // Whether EOF has been reached 
    priv raw: ~[u8],         // Buffer containing raw data 
    priv raw_cursor: uint,
    priv done_first_chunk: bool,     // Whether the first chunk has been processed 
    priv mibenum: u16,       // MIB enum for charset, or 0
    priv encsrc: int,     // Charset source
    priv input: ~filter, // Charset conversion filter
    priv csdetect: Option<parserutils_charset_detect_func>,
    inputstream_alias_create_time:float

}
 /**
    * Create an input stream
    * #Arguments:
    *  'encoding' - The encoding source of stream and filter.
    *  'charset_src'  -  Some of Document charset, or None to autodetect. 
    *  'csdetect_instance' - Charset detection function to auto-detect charset and encoding, or None
    * #Return Value:
    * '(Option<~inputstream> , parserutils_error)' - 
    *       Option<~inputstream>  => if input stream is created successfully Some(input stream) else None
    *       parserutils_error => PARSERUTILS_OK if no errors, otherwise appropriate error is returned
    */
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

    let mut start_time = extra::time::precise_time_ns();
    let alias_instance = alias();
    let mut end_time = extra::time::precise_time_ns();
    let alias_create_time = (end_time as float - start_time as float);

    match parserutils_filter(alias_instance , copy stream_encoding) {
        (x,PARSERUTILS_OK) =>{
            let mut filter_instance = x.unwrap(); 
            stream = ~inputstream {
                utf8: ~[],
                cursor: 0,
                had_eof: false,
                raw: ~[],
                raw_cursor: 0,
                done_first_chunk: false,
                mibenum: arc::get(&filter_instance.instance).parserutils_charset_mibenum_from_name(copy stream_encoding),
                encsrc: stream_charset_src,
                input: filter_instance,
                csdetect: csdetect_instance,
		inputstream_alias_create_time:alias_create_time
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
//////////////////////////////////////////////////////////////////
// Start of input stream internal functions
//////////////////////////////////////////////////////////////////
impl inputstream {

/**
    * Destroy an input stream
    * #Arguments:
    *  'self' - The input stream instance on which the function is called.
    * #Return Value:
    * 'parserutils_error' - return PARSERUTILS_OK on success, appropriate error otherwise
    */
    pub fn parserutils_inputstream_destroy(&mut self)-> parserutils_error {
        self.input.parserutils__filter_destroy();
        self.utf8 = ~[] ;
        self.raw = ~[] ;
        self.raw_cursor = 0;
        self.cursor = 0 ;
        self.had_eof = false ;
        self.done_first_chunk = false ;
        self.mibenum = 0 ;
        self.encsrc =0 ;
        self.csdetect = None;
        PARSERUTILS_OK
    }

 /**
    * Append data to an input stream
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, to which data is appended
    *  'data'  -  Data to append (in document charset), or ~[] to flag EOF 
    * #Return Value:
    * 'parserutils_error' - return PARSERUTILS_OK on success, appropriate error otherwise
    */
    pub fn parserutils_inputstream_append(&mut self, data: &[u8]) -> parserutils_error {
        // io::println("Entering: parserutils_inputstream_append");
        if data.len()==0 {
            self.had_eof = true;
            return PARSERUTILS_OK;
        }
        self.raw += data;
        PARSERUTILS_OK
    }

/**
    * Insert data into stream at current cursor location
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, to which data is appended after the current cursor position
    *  'data'  -  Data to insert (UTF-8 encoded) 
    * #Return Value:
    * 'parserutils_error' - return PARSERUTILS_OK on success, appropriate error otherwise
    */
    pub fn parserutils_inputstream_insert(&mut self, data: &[u8])-> parserutils_error {
        // io::println("Entering: parserutils_inputstream_insert");
        if data.len()==0 && (self.utf8.len() < self.cursor) {
            return PARSERUTILS_BADPARM;
        }

        let temp = self.utf8.slice(self.cursor,self.utf8.len()).to_owned();
        self.utf8.truncate(self.cursor);
        self.utf8 += data ;
        self.utf8 += temp ;
        PARSERUTILS_OK
    }

/**
    * Read the encoding scheme and source charset of the input stream
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, for which the source charset is to be determined
    * #Return Value:
    * '(Option<~str>,int)' - 
    *           Option<~str> => the encoding scheme read
    *           int => the source charset ..
    *                        CSS_CHARSET_DEFAULT=0,
    *                        CSS_CHARSET_REFERRED=1,
    *                        CSS_CHARSET_METADATA=2,
    *                        CSS_CHARSET_DOCUMENT=3,
    *                        CSS_CHARSET_DICTATED=4
    */
    pub fn parserutils_inputstream_read_charset(&mut self)-> (Option<~str>,int) {
        // io::println("Entering: parserutils_inputstream_read_charset");
        (arc::get(&self.input.instance).parserutils_charset_mibenum_to_name(self.mibenum),self.encsrc)
    }

/**
    *  Change the source charset of the input stream
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, for which the source charset is to be changed
    *  'enc'  - the encoding scheme to be updated
    *  'source' - the source charset to be updated
    * #Return Value:
    * return PARSERUTILS_OK on success,
    *         PARSERUTILS_BADPARM on invalid parameters,
    *         PARSERUTILS_INVALID if called after data has been read from stream,
    *         PARSERUTILS_BADENCODING if the encoding is unsupported,
    *         PARSERUTILS_NOMEM on memory exhaustion.
    */

    pub fn parserutils_inputstream_change_charset(&mut self, enc:~str, source:int)-> parserutils_error {
        // io::println("Entering: parserutils_inputstream_change_charset");
        if enc.len() == 0 {
            return PARSERUTILS_BADPARM;
        }

        if self.done_first_chunk {
            return PARSERUTILS_INVALID;
        }
        
        self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(copy enc);
        if self.mibenum==0 {
            return PARSERUTILS_BADPARM;
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

/**
    *  Strip a BOM from a buffer in the given encoding
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, from which BOM is to be removed
    * #Return Value:
    * return PARSERUTILS_OK on success, appropriate error otherwise
    */
    pub fn parserutils_inputstream_strip_bom(&mut self)-> parserutils_error {
        // io::println("Entering: parserutils_inputstream_strip_bom");
        let UTF32_BOM_LEN =4;
        let UTF16_BOM_LEN =2;
        let UTF8_BOM_LEN  =3;

        let totype : Option<~str> = arc::get(&self.input.instance).parserutils_charset_mibenum_to_name(self.mibenum);

        if totype.is_none() {
            return PARSERUTILS_BADPARM;
        }

        let result: ~str= totype.unwrap();
        // io::println(result);
        match(result) {
            ~"UTF-8" => {
                if (self.raw.len() >= UTF8_BOM_LEN) && self.raw[0] == 0xEF && self.raw[1] == 0xBB && self.raw[2] == 0xBF {
                    self.raw_cursor += UTF8_BOM_LEN;
                    return PARSERUTILS_OK;
                } 
            },
            ~"UTF-32" => {
                self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-32BE");
                if self.mibenum==0 {
                    return PARSERUTILS_BADPARM;
                }

                if self.raw.len() >= UTF32_BOM_LEN {
                    if self.raw[0] == 0x00 && self.raw[1] == 0x00 && self.raw[2] == 0xFE && self.raw[3] == 0xFF {
                        self.raw_cursor += UTF32_BOM_LEN;
                        return PARSERUTILS_OK;
                    }
                    else if self.raw[0] == 0xFF && self.raw[1] == 0xFE && self.raw[2] == 0x00 && self.raw[3] == 0x00 {
                        self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-32LE");
                        if self.mibenum==0 {
                            return PARSERUTILS_BADPARM;
                        }
                        
                        self.raw_cursor += UTF32_BOM_LEN;
                        return PARSERUTILS_OK;
                    }
                }
            },
            ~"UTF-16" => {
                self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-16BE");
                
                if self.mibenum==0 {
                    return PARSERUTILS_BADPARM;
                }

                if self.raw.len() >= UTF16_BOM_LEN {
                    if self.raw[0] == 0xFE && self.raw[1] == 0xFF {
                        self.raw_cursor += UTF16_BOM_LEN;
                        return PARSERUTILS_OK;
                    }
                    else if self.raw[0] == 0xFF && self.raw[1] == 0xFE {
                        self.mibenum  = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-16LE");
                        if self.mibenum == 0 {
                            return PARSERUTILS_BADPARM;
                        }

                        self.raw_cursor += UTF16_BOM_LEN;
                        return PARSERUTILS_OK;
                    }
                }
            },
            ~"UTF-16BE" => {
                if self.raw.len() >= UTF16_BOM_LEN && self.raw[0] == 0xFE && self.raw[1] == 0xFF {
                    self.raw_cursor += UTF16_BOM_LEN;
                    return PARSERUTILS_OK;
                }
            },
            ~"UTF-16LE" => {
                if self.raw.len() >= UTF16_BOM_LEN && self.raw[0] == 0xFF && self.raw[1] == 0xFE {
                    self.raw_cursor += UTF16_BOM_LEN;
                    return PARSERUTILS_OK;
                }
            },
            
            ~"UTF-32BE" => {
                if self.raw.len() >= UTF32_BOM_LEN && self.raw[0] == 0x00 && self.raw[1] == 0x00 && self.raw[2] == 0xFE && self.raw[3] == 0xFF {
                    self.raw_cursor += UTF32_BOM_LEN;
                    return PARSERUTILS_OK;
                }
            },
            ~"UTF-32LE" => {
                if self.raw.len() >= UTF32_BOM_LEN && self.raw[0] == 0xFF && self.raw[1] == 0xFE && self.raw[2] == 0x00 && self.raw[3] == 0x00 {
                    self.raw_cursor += UTF32_BOM_LEN;
                    return PARSERUTILS_OK;
                }
            },
            _=>{
                
            }
        }
        return PARSERUTILS_OK;
    }

/**
    *  determines if the data in the input stream passed is of valid ascii type
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, from which BOM is to be removed
    *   'data' - the array whose elements are to be verified
    * #Return Value:
    *   'bool' - true if data is valid ascii 
    */
    pub fn IS_ASCII(&mut self , data:u8) -> bool {
        //io::println(fmt!("Entering: IS_ASCII:: data == %?", data));
        ((data & 0x80) == 0)
    }


/**
    *  Advance the stream's current position
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, whose cursor position is to be updated
    *  'bytes' - the number of bytes to advance
    * #Return Value:
    * return PARSERUTILS_OK on success, appropriate error otherwise
    */
    pub fn parserutils_inputstream_advance(&mut self, bytes:uint) -> parserutils_error {
        // io::println("Entering: parserutils_inputstream_advance");
        if bytes > (self.utf8.len() - self.cursor) {
                return PARSERUTILS_BADPARM;
        }

        if (self.cursor == self.utf8.len()) {
                return PARSERUTILS_OK;
        }

        self.cursor += bytes;
        PARSERUTILS_OK
    }


/**
    *  Refill the UTF-8 buffer from the raw buffer of input stream
    * #Arguments:
    *  'self' - The input stream instance on which the function is called, whose utf8 buffer is refilled with raw buffer
    * #Return Value:
    *   return PARSERUTILS_OK on success, appropriate error otherwise
    */
    pub fn parserutils_inputstream_refill_buffer(&mut self) -> parserutils_error {
        // io::println("Entering: parserutils_inputstream_refill_buffer");
        if (self.done_first_chunk == false) {

            match(self.csdetect) {
                Some(f) => {
                    let (charsetOption,srcOption,error)= (*f)(self.raw.slice(self.raw_cursor, self.raw.len()), self.mibenum, self.encsrc, self.input.instance.clone());

                    match error {
                        PARSERUTILS_OK => {
                            self.mibenum= charsetOption.unwrap();
                            self.encsrc = srcOption.unwrap();
                        },
                        PARSERUTILS_NEEDDATA => {   
                            if self.had_eof == false {
                                return error;
                            }
                        },
                        _ => {
                            // io::println(fmt!("parserutils_inputstream_refill_buffer:: error == %?" , error));
                            return error 
                        }  
                    }
                },
                None => {}
            }   
            if (self.mibenum == 0) {
                self.mibenum = arc::get(&self.input.instance).parserutils_charset_mibenum_from_name(~"UTF-8");
                if self.mibenum == 0 {
                    // io::println("parserutils_inputstream_refill_buffer: self.mibenum == 0");
                    return PARSERUTILS_BADPARM;
                }
                self.encsrc = 0;
            }

            match(self.parserutils_inputstream_strip_bom()) {
                PARSERUTILS_OK => {
                },
                _ => {
                    // io::println("parserutils_inputstream_refill_buffer: match self.parserutils_inputstream_strip_bom => _");
                    return PARSERUTILS_BADPARM;
                } 
            }

            match arc::get(&self.input.instance).parserutils_charset_mibenum_to_name(self.mibenum) {
                None => { 
                    return PARSERUTILS_BADENCODING
                    },
                Some(x) => {
                    match self.input.filter_set_encoding(x) {
                        PARSERUTILS_OK => {
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
        let mut processed_length:uint;
        match(self.input.parserutils__filter_process_chunk(self.raw.slice(self.raw_cursor, self.raw.len()))) {
            (PARSERUTILS_OK, outbuf, len_processed) => {
                 
                    if (!self.done_first_chunk) {
                        self.done_first_chunk = true;
                        if outbuf[0]== 0xFF && outbuf[1]== 0xFE && outbuf[2]== 0x00 && outbuf[3]== 0x00{
                            self.utf8 += slice(outbuf,4,outbuf.len()).to_owned();
                        }
                        else if outbuf[0]== 0xFF && outbuf[1]== 0xFE {
                            self.utf8 += slice(outbuf,2,outbuf.len()).to_owned();
                        }
                        else {
                            self.utf8 += outbuf;
                        }
                }
                else {
                    self.utf8 += outbuf;
                }
                processed_length = len_processed as uint
            },
            (y, _, _ ) => {
                return y
            }
        }

        self.raw_cursor += processed_length;
        return PARSERUTILS_OK;
    }

/**
    *  Look at the character in the stream that starts at
    * offset bytes from the cursor (slow version) and 
    * return data from that offset and length of data
    * #Arguments:
    *  'self' - The input stream instance on which the function is called,
    *           whose utf8 buffer is refilled with raw buffer
    *  'offset' - Byte offset of start of character
    * #Return Value:
    *   '(Option<(~[u8],uint)>,parserutils_error)'
    *          ~[u8] => array of data to return
    *          uint  => length of character
    *          parserutils_error => 
    *               PARSERUTILS_OK on success,
    *               _NEEDDATA on reaching the end of available input,
    *               _EOF on reaching the end of all input,
    *               _BADENCODING if the input cannot be decoded,
    *               _BADPARM if bad parameters are passed.
    */
    pub fn parserutils_inputstream_peek_slow(&mut self , offset: uint)-> (Option<(~[u8],uint)>,parserutils_error) {
        //io::println("Entering: parserutils_inputstream_peek_slow");
        let len: uint;

        if (self.raw.len() <= self.raw_cursor) {
            //io::println("Entering: parserutils_inputstream_peek_slow:: self.raw.len() == 0");
            if self.had_eof {
                return (None,PARSERUTILS_EOF);
            }
            else {
                return (None,PARSERUTILS_NEEDDATA);
            }
        }

         // Refill utf8 buffer from raw buffer 
        match(self.parserutils_inputstream_refill_buffer()) {
            PARSERUTILS_BADPARM => {
                // io::println("parserutils_inputstream_peek_slow: Refill buffer badparam");
                return (None, PARSERUTILS_BADPARM);
            },
            PARSERUTILS_BADENCODING => {return (None, PARSERUTILS_BADENCODING);},
            PARSERUTILS_NEEDDATA => {return (None, PARSERUTILS_NEEDDATA);},
            _ => {}
        }

         // Refill may have succeeded, but not actually produced any new data 
        if self.cursor + offset == self.utf8.len() {  
            // io::println("parserutils_inputstream_peek_slow: cursor+ offset = = utf8.len() ");                  
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
                    // io::println("Entering: parserutils_inputstream_peek_slow: None");
                    return (None, PARSERUTILS_BADPARM);
                     
                },
                Some(l)=> {
                    len=l as uint;
                }
            }
        }

        //io::println("Exiting: parserutils_inputstream_peek_slow");
        return (Some((requested_data,len)),PARSERUTILS_OK);

    }



/**
    *  Look at the character in the stream that starts at
    * offset bytes from the cursor (slow version) and 
    * return data from that offset and length of data
    * #Arguments:
    *  'self' - The input stream instance on which the function is called,
    *           whose utf8 buffer is refilled with raw buffer
    *  'offset' - Byte offset of start of character
    * #Return Value:
    *   '(Option<(~[u8],uint)>,parserutils_error)'
    *          ~[u8] => array of data to return
    *          uint  => length of character
    *          parserutils_error => 
    *               PARSERUTILS_OK on success,
    *               _NEEDDATA on reaching the end of available input,
    *               _EOF on reaching the end of all input,
    *               _BADENCODING if the input cannot be decoded,
    *               _BADPARM if bad parameters are passed.
    */ 
    pub fn parserutils_inputstream_peek(&mut self, offset: uint)-> (Option<(~[u8],uint)>,parserutils_error) {
        //io::println("Entering: parserutils_inputstream_peek");
        let mut ptr:~[u8];
        let mut len :uint;
        
        //io::println(fmt!("parserutils_inputstream_peek:: self.cursor == %?, offset == %?, self.utf8.len() == %?", self.cursor, offset, self.utf8.len()));

        if self.cursor + offset < self.utf8.len() {
            //io::println("Entering: parserutils_inputstream_peek:: self.cursor + offset < self.utf8.len()");
            if self.IS_ASCII(self.utf8[self.cursor + offset]) {
                ptr = slice(self.utf8, self.cursor + offset, self.utf8.len()).to_owned();
                // ascii char length is 1
                return (Some((ptr ,1)) , PARSERUTILS_OK);
            }
            else {
                ptr = slice(self.utf8, self.cursor + offset, self.utf8.len()).to_owned();
                
                match(filter::parserutils_charset_utf8_char_byte_length(ptr)) {
                    None=>{
                        // io::println("parserutils_inputstream_peek: None");
                        return (None, PARSERUTILS_BADPARM);
                    },
                    Some(l)=> {
                        len=l as uint;
                    }
                }
                return(Some((ptr , len)) , PARSERUTILS_OK);
            }
        }
        //io::println("parserutils_inputstream_peek: before peek_slow");
        return self.parserutils_inputstream_peek_slow(offset);
    }
}



