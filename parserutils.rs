#[link(name = "parserutils", vers = "0.1")];
#[crate_type = "lib"];

extern mod std;
extern mod riconv;
use core::io::Reader;
use core::io::ReaderUtil;
use std::oldmap;
use core::vec::*;
use libc::size_t;

pub type  parserutils_charset_detect_func =  ~extern fn(
		data: ~[u8], mibenum:~u16, source:~u32) -> parserutils_result;
pub struct parserutils_inputstream 
{
	utf8: ~[u8],	/*< Buffer containing UTF-8 data */

	cursor: u32,		/*< Byte offset of current position */

	had_eof: bool			/*< Whether EOF has been reached */
} 


pub struct parserutils_inputstream_private {
	mut public:  parserutils_inputstream,	/*< Public part. Must be first */

	mut raw: ~[u8],	/*< Buffer containing raw data */

	mut done_first_chunk: bool ,		/*< Whether the first chunk has 
					  been processed */

	mut mibenum: u16,		/*< MIB enum for charset, or 0 */
	mut encsrc: u32,		/*< Charset source */

	mut input: @parserutils_filter ,	/*< Charset conversion filter */

	mut csdetect: parserutils_charset_detect_func  /*< Charset detection func.*/

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
	mut int_enc: u16,               /**< The internal encoding */
	mut settings : struct_settings ,
	mut iconv_h : u64 ,
	mut pw : ~[u8]
}

pub enum parserutils_result
{
	PARSERUTILS_FILTER_CREATE_OK(@parserutils_filter),
	PARSERUTILS_FILTER_PROCESS_CHUNK_OK((u64,~[u8])),
	PARSERUTILS_INPUTSTREAM_CREATE_OK(@parserutils_inputstream_private),
	PARSERUTILS_CHARSET_EXT_OK((@u16,@u32)),
	PARSERUTILS_CHARSET_TRY_OK(@u16),
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

	pub fn parserutils_charset_mibenum_from_name(&self, alias: ~str) -> u16 {
        match self.alias_map.find(&alias) {
        	None => 0 ,
        	Some(mib_enum) => mib_enum
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

		let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(enc);
		if mibenum_search_result==0 {
			return PARSERUTILS_BADPARAM;
		}

		let mibenum = mibenum_search_result;

		/* Exit early if we're already using this encoding */
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

			io::println("abcdefgh");
			return PARSERUTILS_BADENCODING;
		}
io::println("abcdefgh++++++");
		input.settings.encoding = mibenum;
		PARSERUTILS_FILTER_CREATE_OK(input)
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
		f.int_enc =self.parserutils_charset_mibenum_from_name(int_enc);
		if f.int_enc==0 {
			return PARSERUTILS_BADENCODING ;
		}

		f.pw = copy pw;

		f.settings.encoding = 0;
		self.filter_set_encoding(f, ~"UTF-8")
	}
	
	pub fn parserutils_charset_utf8_char_byte_length(&self, s: ~[u8]) -> Option<u8> {
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
			io::println("__filter_setopt_bad param");//sandeep
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
			PARSERUTILS_FILTER_CREATE_OK(input)
		}
		else {
			PARSERUTILS_BADPARAM
		}	
	}

	pub fn parserutils__filter_process_chunk(&self, input : @parserutils_filter ,inbuf : ~[u8] ) -> parserutils_result {

		io::println(fmt!("\n Input arguments to the process chunk is =%?= ",inbuf));
		
		if ( input.int_enc==0 )||( inbuf.len()==0 ) {
			return PARSERUTILS_BADPARAM;
		}
		
		let iconv_result : riconv::chunk_result = riconv::safe_riconv(input.iconv_h,move inbuf) ;

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
		PARSERUTILS_FILTER_PROCESS_CHUNK_OK( (iconv_result.len_processed,copy iconv_result.outbuf) )
	}
		
	pub fn parserutils_inputstream_create(&self,enc: ~str,
		encsrc: u32,  csdetect: parserutils_charset_detect_func, pw : ~[u8] )-> parserutils_result
	{
		let s: @parserutils_inputstream_private;
		let mut pRslt: parserutils_result;
        let inputTemp: @parserutils_filter;
		let mut mibenumTemp: u16 = 0; 
		 
		


          pRslt = self.parserutils__filter_create(~"UTF-8",copy  pw );
         match(pRslt)
         {
         	PARSERUTILS_FILTER_CREATE_OK(parserutils_filter) => 
         	{
         		inputTemp= parserutils_filter;
         	},
         	_=>
         	{
         		/*parserutils_buffer_destroy(s->public.utf8);
		        parserutils_buffer_destroy(s->raw);
		        alloc(s, 0, pw);*/
              	return pRslt;    
         	} 
         }


        let x=self.parserutils_charset_mibenum_from_name(copy enc);
        match(x)
        {
        	0=>{},
        	_=> {mibenumTemp = x;}
        }

        s=  @parserutils_inputstream_private
        {
        	public: parserutils_inputstream
        	{
        		utf8: ~[],	

				cursor: 0,		

				had_eof: false
        	},	

			raw: ~[] ,	

			done_first_chunk: false ,		 
					 

			mibenum: mibenumTemp,		
			encsrc: encsrc,		

			input:inputTemp ,	

			csdetect: csdetect  

			
			
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
				return err;*/
				self.parserutils__filter_destroy(s.input);
				return pRslt;
        	}
        }
	
	return PARSERUTILS_INPUTSTREAM_CREATE_OK(s);
	}

	pub fn parserutils_inputstream_destroy(&self,
		stream:@parserutils_inputstream_private)-> parserutils_result
	{
		self.parserutils__filter_destroy(stream.input);
		PARSERUTILS_GENERAL_OK
	}	
/**
 * Append data to an input stream at the end of raw data
 *
 * \param stream  Input stream to append data to
 * \param data    Data to append (in document charset), or NULL to flag EOF
 * \param len     Length, in bytes, of data
 * \return PARSERUTILS_OK on success, appropriate error otherwise
 */
	pub fn parserutils_inputstream_append(&self,stream:@parserutils_inputstream_private, 
		data: @[u8])-> parserutils_result{
		if (data.len()==0) {
			stream.public.had_eof = true;
			return PARSERUTILS_GENERAL_OK;
		}
		let mut rawTemp:~[u8] = ~[];
		stream.raw <-> rawTemp;
		rawTemp=append(rawTemp, data);
		stream.raw <-> rawTemp;
		return PARSERUTILS_GENERAL_OK;
	
	}

	pub fn print_inputstream(&self,stream:@parserutils_inputstream_private)
	{
		io::println("printing");
		io::println(fmt!("%?",stream.public.cursor));
		io::println(fmt!("%?",stream.public.utf8));
		io::println(fmt!("%?",stream.raw));
	}
/**
 * Insert data into utf8 part of stream at current location. current location is
 * pointed by cursor position
 *
 * \param stream  Input stream to insert into
 * \param data    Data to insert (UTF-8 encoded)
 * \return PARSERUTILS_GENERAL_OK on success, appropriate error otherwise
 */
    pub fn parserutils_inputstream_insert(&self,stream:@parserutils_inputstream_private,
	   data: @[u8])-> parserutils_result{
        let mut Iter=0;        
        let dataLen= data.len();
        let mut buffer=  ~[];
        let offset= stream.public.cursor;
        buffer <->stream.public.utf8;
        let bufLen= buffer.len();
        
		
		if ( dataLen ==0) {
			return PARSERUTILS_BADPARAM;
		}
        
        if (offset > bufLen as u32){//running past the allocated buffer
        	return PARSERUTILS_BADPARAM;
        }
		

		if (offset == bufLen as u32)//start writing from end of buffer
		{
			buffer  =  append(buffer, data);
			buffer <-> stream.public.utf8;
			return	   PARSERUTILS_GENERAL_OK;
		}
        //offset is less than bufflen,i.e., points to middle of utf8 data

        while(Iter < dataLen)
        {
        	buffer.insert(Iter,data[Iter]);
        	Iter += 1;
        }
		
		buffer <->stream.public.utf8;
       return PARSERUTILS_GENERAL_OK; 
	}

	
	pub fn parserutils_inputstream_read_charset(&self,
		stream:@parserutils_inputstream_private,mut source:@mut u32)-> Option<~str>{
		*source = stream.encsrc;
		if ((*source) == 0)
		{
			return Some(~"UTF-8")
		}
		return self.parserutils_charset_mibenum_to_name(stream.mibenum);
	}


	pub fn parserutils_inputstream_change_charset(&self,stream:@parserutils_inputstream_private, 
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

		/* Ensure filter is using the correct encoding */
		//params.encoding.name = enc;
		pRslt = self.parserutils__filter_setopt(stream.input,enc);
		match(pRslt)
		{
			PARSERUTILS_FILTER_CREATE_OK(Temp) => {},
			_=>{io::println("return ##############");return pRslt;}
		}
		

		/* Finally, replace the current settings */
		stream.mibenum = mibenum;
		stream.encsrc = source;

		return PARSERUTILS_GENERAL_OK;
	}


	pub fn parserutils_inputstream_strip_bom(&self,mibenum:@mut u16, 
		buffer:@mut~[u8])-> parserutils_result
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
					*buffer= slice(*buffer,UTF8_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
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
					*buffer= slice(*buffer,UTF16_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
					
					} else if (buffer[0] == 0xFF && 
						buffer[1] == 0xFE) {
					
					let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(~"UTF-16LE");
					if mibenum_search_result==0 {
						return PARSERUTILS_BADPARAM;
					}

					*mibenum = mibenum_search_result;
					*buffer= slice(*buffer,UTF16_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
					
					}
				}
			},
			~"UTF-16BE" =>{
				if (buffer.len() >= UTF16_BOM_LEN &&
				buffer[0] == 0xFE &&
				buffer[1] == 0xFF) {
					*buffer= slice(*buffer,UTF16_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
				}
			},
			~"UTF-16LE" =>{
				if (buffer.len() >= UTF16_BOM_LEN &&
				buffer[0] == 0xFF &&
				buffer[1] == 0xFE) {
					*buffer= slice(*buffer,UTF16_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
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
				*buffer= slice(*buffer,UTF32_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
				
			} else if (buffer[0] == 0xFF && 
					buffer[1] == 0xFE &&
					buffer[2] == 0x00 &&
					buffer[3] == 0x00) {
				
				let mibenum_search_result  = self.parserutils_charset_mibenum_from_name(~"UTF-32LE");
				if mibenum_search_result==0{
					return PARSERUTILS_BADPARAM;
				}

				*mibenum = mibenum_search_result;
				*buffer= slice(*buffer,UTF32_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
				
			}
		}
			},
			~"UTF-32BE" =>{
				if (buffer.len() >= UTF32_BOM_LEN &&
				buffer[0] == 0x00 &&
				buffer[1] == 0x00 &&
				buffer[2] == 0xFE &&
				buffer[3] == 0xFF) {
                  	*buffer= slice(*buffer,UTF32_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
				}
			},
			~"UTF-32LE" =>{
				if (buffer.len() >= UTF32_BOM_LEN &&
				buffer[0] == 0xFF &&
				buffer[1] == 0xFE &&
				buffer[2] == 0x00 &&
				buffer[3] == 0x00) {
					*buffer= slice(*buffer,UTF32_BOM_LEN-1,buffer.len()-1);
					return PARSERUTILS_GENERAL_OK;
				}
			},
			_=>{
				io::println(~"RES="+Res);
			}
		}
		return PARSERUTILS_GENERAL_OK;
	}


	pub fn parserutils_inputstream_refill_buffer(&self,
		stream:@parserutils_inputstream_private)-> parserutils_result{
		let mut pRslt:parserutils_result = PARSERUTILS_GENERAL_OK ;
		if (stream.done_first_chunk == false) {
			//parserutils_filter_optparams params;

			/* If there is a charset detection routine, give it an 
		 	* opportunity to override any charset specified when the
		 	* inputstream was created */
			
			
				

				pRslt = (*stream.csdetect)(copy stream.raw, ~stream.mibenum, ~stream.encsrc);
				match(pRslt)
				{
					PARSERUTILS_GENERAL_OK=>{
						io::println("ok result");//sandeep
					},
					PARSERUTILS_NEEDDATA=>{
						if stream.public.had_eof == false{
							return pRslt;
						}
					},
					_=> return pRslt
				}
				

					/* We don't have enough data to detect the 
					* input encoding, but we're not going to get 
				 	* any more as we've been notified of EOF. 
				 	* Therefore, leave the encoding alone
					* so that any charset specified when the
				 	* inputstream was created will be preserved.
				 	* If there was no charset specified, then
				 	* we'll default to UTF-8, below */
					
			

			/* Default to UTF-8 if there is still no encoding information 
		 	* We'll do this if there was no encoding specified up-front
		 	* and:
		 	*    1) there was no charset detection routine
		 	* or 2) there was insufficient data for the charset 
		 	*       detection routine to detect an encoding
		 	*/
		 	io::println("line______#");//sandeep
			if (stream.mibenum == 0) {
				
				let mibenum_search_result = self.parserutils_charset_mibenum_from_name(~"UTF-8");
				if mibenum_search_result==0 {
					io::println("bad Param"); //sandeep
					return PARSERUTILS_BADPARAM;
				}

				stream.mibenum = mibenum_search_result;
				stream.encsrc = 0;
			}
io::println("line______##");//sandeep
			if (stream.mibenum == 0)
				{
					fail!();
				}
			let mut temp:@mut ~[u8] =@mut ~[];
			*temp <-> stream.raw;	
			/* Strip any BOM, and update encoding as appropriate */
			pRslt = self.parserutils_inputstream_strip_bom( @mut stream.mibenum, 
				  temp);
			*temp <-> stream.raw;
			match(pRslt)
			{
				PARSERUTILS_GENERAL_OK=>{},
				_=>{return pRslt;}
			}
			
			io::println("line______###");//sandeep
			/* Ensure filter is using the correct encoding */
			

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
			/* Work out how to perform the buffer fill */
		if (stream.public.cursor == stream.public.utf8.len() as u32) {
		/* Cursor's at the end, so simply reuse the entire buffer */
		
		stream.public.utf8= ~[];
		} else {
			/* Cursor's not at the end, so shift data after cursor to the
		 	* bottom of the buffer. If the buffer's still over half full, 
		 		* extend it. */
		 	stream.public.utf8=slice(stream.public.utf8,stream.public.cursor as uint,stream.public.utf8.len());
			
			
		}
		self.print_inputstream(stream);
		io::println("line______&&");//sandeep	
		let utf8:&str= "";
		let raw:@mut ~[u8] = @mut ~[];
		*raw <-> stream.raw;
		let raw_length = raw.len();
        io::println("line______&&");
		/* Try to fill utf8 buffer from the raw data */
		//pRslt = self.parserutils__filter_process_chunk(stream.input, raw.to_str(), &(raw_length as u64), utf8, &100);
		io::println("line______&&**");
		match(pRslt)
		{
			PARSERUTILS_GENERAL_OK =>{}
			_=>return pRslt
		}
io::println("line______&&&");//sandeep
		*raw <-> stream.raw;
		let mut Iter = 0;
		while(Iter < utf8.len())
		{
			stream.public.utf8.insert(stream.public.utf8.len(),utf8[Iter]);
		}

		/* _NOMEM implies that there's more input to read than available space
	 	* in the utf8 buffer. That's fine, so we'll ignore that error. */
		

		/* Remove the raw data we've processed from the raw buffer */
        stream.raw= slice(stream.raw,stream.raw.len() - raw_length,stream.raw.len()-1);
		
		/* Fix up the utf8 buffer information */
		

		/* Finally, fix up the cursor */
		io::println("line______&##");//sandeep
		stream.public.cursor = 0;

			return PARSERUTILS_GENERAL_OK;
		}
        
		pub fn IS_ASCII(&self,data:u8)-> bool
		{
			return (((data) & 0x80) == 0);
		}

		pub fn parserutils_inputstream_peek_slow(&self,	stream:@parserutils_inputstream_private, 
			offset:size_t , ptr:@mut~[u8], length:@mut size_t )-> parserutils_result{
            io::println("line1_____");//sandeep
            let len:size_t;
            let mut pRslt:parserutils_result;
			if (stream.raw.len() == 0) {
		/* No more data to be had */
				if(stream.public.had_eof){
					return PARSERUTILS_EOF;
				} else{
					return PARSERUTILS_NEEDDATA;
				}

			}

			/* Refill utf8 buffer from raw buffer */
			pRslt = self.parserutils_inputstream_refill_buffer(stream);
			self.print_inputstream(stream);
			match(pRslt)
			{
				PARSERUTILS_GENERAL_OK => {},
				_=> return pRslt
			}

			/* Refill may have succeeded, but not actually produced any new data */
			if (stream.public.cursor + offset as u32 == stream.public.utf8.len() as u32)
				{
					io::println("No data produced");//sandeep
					return PARSERUTILS_NEEDDATA;
				}
             io::println("line______#");//sandeep
			/* Now try the read */
			if (self.IS_ASCII(stream.public.utf8[stream.public.cursor + offset as u32])) {
				len = 1;
			} else {
				let Temp= self.parserutils_charset_utf8_char_byte_length(slice
					(stream.public.utf8,stream.public.cursor as uint+ offset as uint,stream.public.utf8.len()));
				match(Temp)
				{
					None=>{
					 	pRslt = PARSERUTILS_BADPARAM;
					 	return pRslt;
					},
					Some(x)=> {
						len=x as size_t;
					}
				}
		
        		match(pRslt)
        		{
        			PARSERUTILS_GENERAL_OK => {},
        			PARSERUTILS_NEEDDATA   => {
        				if(stream.public.had_eof){
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
			(*ptr) = slice(stream.public.utf8, stream.public.cursor as uint+ offset as uint,stream.public.utf8.len());

			return PARSERUTILS_GENERAL_OK;
		}



	pub fn parserutils_inputstream_advance(&self,stream:@parserutils_inputstream_private, bytes:size_t ){
		
		if (bytes as uint> stream.public.utf8.len() - stream.public.cursor as uint)
			{
				fail!();
			}

		if (stream.public.cursor as uint == stream.public.utf8.len())
			{
				return;
			}

		stream.public.cursor += bytes as u32;
	}


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
    pub fn parserutils_inputstream_peek(&self,stream:@parserutils_inputstream_private, offset: size_t, ptr:@mut ~[u8], length:@mut size_t) 
    -> parserutils_result 
    {
		let mut pRslt: parserutils_result = PARSERUTILS_GENERAL_OK;
		//const parserutils_buffer *utf8;
		let utf8_data:@mut~[u8]=@mut copy stream.public.utf8;
		let mut len :size_t;
		
		let mut utf8_len = stream.public.utf8.len();;
		let mut off = stream.public.cursor + offset as u32;
		

		/*#ifdef RANDOMISE_INPUTSTREAM
			parserutils_buffer_randomise(stream->utf8);//left out
		#endif*/

		

		if (off < utf8_len as u32) {
			if (self.IS_ASCII(utf8_data[off])) {
				/* Early exit for ASCII case */
				(*length) = 1;
				(*ptr) = slice(*utf8_data,off as uint,utf8_len);
				return PARSERUTILS_GENERAL_OK;
			} else {
				match(self.parserutils_charset_utf8_char_byte_length(slice(*utf8_data, off as uint,utf8_len)))
				{
					None => {
					 	return PARSERUTILS_BADPARAM;
					 	
					},
					Some(x) => {
						len=x as size_t;
						(*length) = len;
					    (*ptr) = slice(*utf8_data,off as uint,utf8_len);
					    return PARSERUTILS_GENERAL_OK;
					}
				}
				
				
			}
		}


		if (off != utf8_len as u32)
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