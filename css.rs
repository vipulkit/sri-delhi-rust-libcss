#[link(name = "css", vers = "0.1")];
#[crate_type = "lib"];



extern mod std;
extern mod parserutils;
extern mod wapcaplet;
extern mod css_ds;
extern mod css_enum ;

use wapcaplet::*;
use core::cast;
use core::vec::* ; 
use core::str::* ;
use core::str::raw::* ;
use core::vec::raw::* ;
use parserutils::* ;
use css_ds::*;
use css_enum::* ;


// ===========================================================================================================
// CSS Public APIs implementation/data-structs Starts here 
// ===========================================================================================================

/*
 * All public APIs and data-structures as exposed by libcss ( please see external include folder of libcss)
 * comes here , APIs and data-structs comes in this section need only to be public interfaces , 
 * rest can be private to this module
 * 
 * Mirror of include folder of libcss
 */

// functypes.h 
// TODO : css_allocator_fn 

// libcss.h 
// Done : nothing remaining 


pub fn css_result_to_string(css_err : css_result ) -> ~str {

	let mut result : ~str = ~"" ;
	match css_err {
		CSS_IMPORTS_PENDING_OK(x,y)=>{result=~"import pending list created"}
		CSS_LANGUAGE_CREATED_OK(x)=> {result=~"language instance created successfully"},
		CSS_STYLESHEET_CREATE_OK(sheet)=>{result=~"stylesheet successfully created"},
		CSS_STRING_GET(x)=>{result = ~"get the string from vector part of stylesheet"},
		CSS_RULE_CREATED_OK(x) => {result=~"Css rule created successfully"},
		CSS_STRING_ADD_OK(x)  => {result = ~"string added to stylesheet successfully"}, 
		CSS_GET_LANGUAGE_LEVEL(x)=>{result= ~"get language level"},
		CSS_GET_URL(x)=>{result=~"get url"},
		CSS_GET_TITLE(x)=>{result=~"get title"},
		CSS_IS_QUIRK_ALLOWED(x)=>{result = ~"is Quirks allowed?"},
		CSS_IS_QUIRK_USED(x)=>{result= ~"IS_QUIRK_USED?"},
		CSS_GET_SHEET_DISABLED(x)=>{result=~"_GET_if-SHEET_DISABLED"},
		CSS_STYLECREATED_OK(x)=>{result=~"Style created successfully"},
		/*CSS_RULE_SELECTOR_CREATED(x) => {result=~"Css rule selector created successfully"},
		CSS_RULE_CHARSET_CREATED(x) => {result=~"Css rule charset created successfully"},
		CSS_RULE_IMPORT_CREATED(x) => {result=~"Css rule imported successfully"},
		CSS_RULE_MEDIA_CREATED(x) => {result=~"Css rule media created successfully"},
		CSS_RULE_FONT_FACE_CREATED(x) => {result=~"Css rule font-face created successfully"},
		CSS_RULE_PAGE_CREATED(x) => {result=~"Css rule page created successfully"},*/
		CSS_GENERAL_OK=> {result=~"Css Success "},
		CSS_LANGUAGE_CREATED(x) => {result=~"Css language created successfully"},
		CSS_PROPSTRINGS_OK(x) => {result=~"Css propstrings success "},
		CSS_NOMEM=> {result=~"Css error : No-memory"},
		CSS_BADPARM=> {result=~"Css error : bad-parameters "},
		CSS_INVALID=> {result=~"Css error : Invalid operation "},
		CSS_FILENOTFOUND=> {result=~"Css error : file not found"},
		CSS_NEEDDATA=> {result=~"Css error : need more data"},
		CSS_BADCHARSET=> {result=~"Css error : bad charset"},
		CSS_EOF=> {result=~"Css error : end of file "},
		CSS_IMPORTS_PENDING=> {result=~"Css imports pending "},
		CSS_PROPERTY_NOT_SET=> {result=~"Css property not set "},
		// _ => { result=~"Unknown error enumeration" },
	}
	result
}

// hint.h


// font-face.h

// fpmath.h





// ===========================================================================================================
// CSS Public APIs implementation/data-structs ends here 
// ===========================================================================================================


/*
 * This file is part of Rust-LibCSS.
 */




	
// ===========================================================================================================
// Lib CSS implementation/data-structs start here 
// ===========================================================================================================


pub fn lcss()->@mut lcss {
	let mut lwc_inst        = lwc();
	let mut lexer_inst      = lcss_lexer();
	let mut stylesheet_inst = lcss_stylesheet(&mut lwc_inst);
	let mut language_inst   = lcss_language(stylesheet_inst);
	let mut parser_inst     = lcss_parser(lexer_inst,language_inst);
	@mut lcss {
		lwc_instance:&lwc_inst,
		lpu_instance:lpu(),
		lcss_language:language_inst,
		lcss_stylesheet:stylesheet_inst,
		lcss_parser:parser_inst,
		lcss_lexer:lexer_inst,

		
	}
}


impl lcss {

	//static pub fn isDigit( c:char)-> bool
	pub fn isDigit( c:char)-> bool
	{
		return '0' <= c && c <= '9';
	}

	//static pub fn isHex(c:char)->bool 
	pub fn isHex(c:char)->bool 
	{
		return lcss::isDigit(c) || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F');
	}

	//static pub fn charToHex( mut c: char)-> u32
	pub fn charToHex( mut c: char)-> u32
	{
		c -= '0';

		if (c as u8> 9)
			{c -= 'A' - '9' - 1 as char;}

		if (c as u8 > 15)
			{c -= 'a' - 'A';}

		return c as u32;
	}

	

	/*pub fn lwc_string_length(string:~lwc_string)-> size_t
	{
	        assert(string);
	        
	        return string.len;
	}


	pub fn lwc_string_data(string:~lwc_string)-> ~str
	{
	        assert(string);
	        
	        return CSTR_OF(string);
	}*/
	pub fn css__number_from_lwc_string(string:@mut lwc_string,
			int_only:bool , consumed:@mut int)-> css_fixed
	{
		
			if(lwc::lwc_string_length(string)== 0)
			{
				return 0;
			} 
	    return  lcss::css__number_from_string(lwc::lwc_string_data(string),int_only,consumed);
		/*return css__number_from_string(
				(uint8_t *)lwc_string_data(string),
				lwc_string_length(string),
				int_only,
				consumed);*/
	}

	 pub fn css__number_from_string(data:@str/*, len:size_t*/ ,
			int_only:bool , consumed:@mut int )-> css_fixed
	{
	    let mut sign:int = 1;
	    let mut len = data.len();
	    let mut iter = 0;
		let mut intpart:i32 = 0;
		let mut fracpart:i32 = 0;
		let mut pwr:i32 = 1;

	    if len == 0
	    {
	    	return 0;
	    }

		if (data[iter] == '-' as u8) {
			sign = -1;
			len -= 1;
			iter += 1;
		} else if (data[iter] == '+' as u8) {
			len -= 1;
			iter += 1;
		}

		if (len == 0) {
			*consumed = 0;
			return 0;
		} else {
			if (data[iter] == '.' as u8) {
				if (len == 1 || data[iter+1] < '0' as u8 || data[iter+1] > '9' as u8) {
					*consumed = 0 ;
					return 0;
				}
			} else if (data[iter] < '0' as u8 || data[iter] > '9' as u8) {
				*consumed = 0;
				return 0;
			}
		}


	    /* Now extract intpart, assuming base 10 */
		while (len > 0) {
			/* Stop on first non-digit */
			if (data[iter] < '0' as u8 || data[iter] > '9' as u8)
				{break;}

			/* Prevent overflow of 'intpart'; proper clamping below */
			if (intpart < (1 << 22)) {
				intpart *= 10;
				intpart += (data[iter] - '0' as u8) as i32;
			}
			iter += 1;
			len -=1;
		}


	    /* And fracpart, again, assuming base 10 */
		if (int_only == false && len > 1 && data[iter] == '.' as u8 && 
				('0' as u8 <= data[iter + 1] && data[iter + 1] <= '9' as u8)) {
			iter += 1;
			len -= 1;

			while (len > 0) {
				if (data[iter] < '0' as u8 || ('9' as u8) < data[iter])
					{break;}

				if (pwr < 1000000) {
					pwr *= 10;
					fracpart *= 10;
					fracpart += (data[iter] - '0' as u8) as i32;
				}
				iter += 1;
				len -= 1;
			}//
			fracpart = ((1 << 10) * fracpart + pwr/2) / pwr;
			if (fracpart >= (1 << 10)) {
				intpart += 1;
				fracpart &= (1 << 10) - 1;
			}
		}

		*consumed = iter;


		if (sign > 0) {
			/* If the result is larger than we can represent,
			 * then clamp to the maximum value we can store. */
			if (intpart >= (1 << 21)) {
				intpart = (1 << 21) - 1;
				fracpart = (1 << 10) - 1;
			}
		}
		else {
			/* If the negated result is smaller than we can represent
			 * then clamp to the minimum value we can store. */
			if (intpart >= (1 << 21)) {
				intpart = -(1 << 21);
				fracpart = 0;
			}
			else {
				intpart = -intpart;
				if (fracpart != 0) {
					fracpart = (1 << 10) - fracpart;
					intpart -= 1;
				}
			}
		}

		return (intpart << 10) | fracpart;
	}

	
	 pub fn buildOPV(opcode : css_properties_e , flags : u8 , value : u16 ) -> css_code_t {

		(( (opcode as int)  & 0x3ff) | ((flags as int)<< 10) | (((value as int)& 0x3fff)  << 18) ) as u32
	}

	 pub fn getOpcode(OPV : css_code_t ) -> css_properties_e {

		 //((OPV & 0x3ff) as int) as opcode_t
		 let op_code : int = (OPV & 0x3ff) as int ;
		 unsafe { cast::transmute(&op_code) }
	}

	 pub fn getFlags(OPV : css_code_t ) -> u8 {

		((OPV >> 10) & 0xff) as u8
	}

	pub fn getValue(OPV : css_code_t ) -> u16 {

		 (OPV >> 18) as u16
	}

	 pub fn isImportant(OPV : css_code_t ) -> bool {

		if (lcss::getFlags(OPV) & 0x1)==0 {
		 	false
		 }
		 else {
		 	true
		 }
	}

	pub fn isInherit(OPV : css_code_t ) -> bool {

		if (lcss::getFlags(OPV) & 0x2)==0 {
			false 
		}
		else {
			true
		}
	}


}

 



// ===========================================================================================================
// Lib CSS- implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// CSS-PARSER implementation/data-structs Starts here 
// ===========================================================================================================



/*
 * Css parser event handler function pointer
 */

// null function for initializing
pub fn dummy_par_ev_hand(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:&css_language) -> css_result {
	CSS_GENERAL_OK
}


fn Stylesheet_event_handler(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:&css_language)-> css_result
{
	CSS_GENERAL_OK
}

pub type css_parser_event_handler =  @extern fn(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:&css_language) -> css_result;

pub fn css_parser_optparams_instance()->@css_parser_optparams
{   let sheet = lcss_stylesheet(lwc());
	let lang=lcss_language(sheet);
	@css_parser_optparams
	{
		quirks:false,
		event_handler: css_parser_event_handler_struct
		{
			handler:@Stylesheet_event_handler,
			pw:lang
		}	
	}
}
/*
 * Css parser constructor
 */

 pub fn css__parser_create(charset:~str,cs_source:css_charset_source ,lcss_language_inst:@css_language)->@lcss_parser {
	@lcss_parser{ event_pw:lcss_language_inst, 
		quirks:false,lcss_lexer_instance:lcss_lexer() , lparserutils_instance:lpu() }
}
pub fn css__parser_create_for_inline_style(charset:~str,cs_source:css_charset_source ,lcss_language_inst:@css_language)->@lcss_parser {
	@lcss_parser{ event_pw:lcss_language_inst, 
		quirks:false,lcss_lexer_instance:lcss_lexer() , lparserutils_instance:lpu() }
}
pub fn lcss_parser(lcss_lexer_inst:@lcss_lexer,lcss_language_inst:@css_language)->@lcss_parser {
	@lcss_parser{ event_pw:lcss_language_inst, 
		quirks:false,lcss_lexer_instance:lcss_lexer_inst , lparserutils_instance:lpu() }
}


/*
 * Css parser implementation
 */
impl lcss_parser {
	pub fn css__parser_completed(&self)->css_result {
	CSS_GENERAL_OK
	}
	pub fn css__parser_create(&self)  {

	}
	pub fn css__parser_parse_chunk(&self, data:~[u8]) -> css_result{
     CSS_GENERAL_OK
	}
	
	pub fn css__parser_create_internal(&self,charset:~str, 
			cs_source:css_charset_source ,pw :~[u8], initial:parser_state ) -> css_result
	{
		let mut err : css_result ;
		//css_parser *p;
		let mut perr : parserutils::parserutils_result ;

		//if (alloc == NULL || parser == NULL)
		//	return CSS_BADPARM;

		//p = alloc(NULL, sizeof(css_parser), pw);
		//if (p == NULL)
		//	return CSS_NOMEM;
		/*
		perror = self.lparserutils_instance.parserutils_inputstream_create(charset, cs_source as u32,
				css__charset_extract, (parserutils_alloc) alloc, pw,
				&p->stream);
		perror = parserutils_inputstream_create(charset,ccs)
		if (perror != PARSERUTILS_OK) {
			alloc(p, 0, pw);
			return css_error_from_parserutils_error(perror);
		}

		error = css__lexer_create(p->stream, alloc, pw, &p->lexer);
		if (error != CSS_OK) {
			parserutils_inputstream_destroy(p->stream);
			alloc(p, 0, pw);
			return error;
		}

		perror = parserutils_stack_create(sizeof(parser_state), 
				STACK_CHUNK, (parserutils_alloc) alloc, pw,
				&p->states);
		if (perror != PARSERUTILS_OK) {
			css__lexer_destroy(p->lexer);
			parserutils_inputstream_destroy(p->stream);
			alloc(p, 0, pw);
			return css_error_from_parserutils_error(perror);
		}

		perror = parserutils_vector_create(sizeof(css_token), 
				STACK_CHUNK, (parserutils_alloc) alloc, pw,
				&p->tokens);
		if (perror != PARSERUTILS_OK) {
			parserutils_stack_destroy(p->states);
			css__lexer_destroy(p->lexer);
			parserutils_inputstream_destroy(p->stream);
			alloc(p, 0, pw);
			return css_error_from_parserutils_error(perror);
		}

		perror = parserutils_stack_create(sizeof(char), 
				STACK_CHUNK, (parserutils_alloc) alloc, pw,
				&p->open_items);
		if (perror != PARSERUTILS_OK) {
			parserutils_vector_destroy(p->tokens);
			parserutils_stack_destroy(p->states);
			css__lexer_destroy(p->lexer);
			parserutils_inputstream_destroy(p->stream);
			alloc(p, 0, pw);
			return css_error_from_parserutils_error(perror);
		}

		perror = parserutils_stack_push(p->states, (void *) &initial);
		if (perror != PARSERUTILS_OK) {
			parserutils_stack_destroy(p->open_items);
			parserutils_vector_destroy(p->tokens);
			parserutils_stack_destroy(p->states);
			css__lexer_destroy(p->lexer);
			parserutils_inputstream_destroy(p->stream);
			alloc(p, 0, pw);
			return css_error_from_parserutils_error(perror);
		}

		p->quirks = false;
		p->pushback = NULL;
		p->parseError = false;
		p->match_char = 0;
		p->event = NULL;
		p->last_was_ws = false;
		p->event_pw = NULL;
		p->alloc = alloc;
		p->pw = pw;

		*parser = p;
		*/
		CSS_GENERAL_OK
	}
pub fn css__parser_setopt(/*css_parser *parser,*/&self,  opt_type:css_parser_opttype,
		params:@css_parser_optparams )-> css_result
{
	/*if (parser == NULL || params == NULL)
		return CSS_BADPARM;
*/
	match (opt_type) {
	 CSS_PARSER_QUIRKS=>{
			//self.quirks = params.quirks;
		},	
		
	 CSS_PARSER_EVENT_HANDLER=>	{
			//self.event = params.event_handler.handler;
			//self.event_pw = params.event_handler.pw;
		}
		
	}

	return CSS_GENERAL_OK;
}
}


// ===========================================================================================================
// CSS-PARSER implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// CSS-LEXER implementation/data-structs starts here 
// ===========================================================================================================



pub fn ascii_lower(string: &str) -> ~str {
    do str::map(string) |c| {
        match c {
            'A'..'Z' => c + ASCII_LOWER_OFFSET,
            _ => c,
        }
    }
}




fn preprocess(input: &str) -> ~str {
    // TODO: Is this faster if done in one pass?
    str::replace(str::replace(str::replace(input,
    "\r\n", "\n"),
    "\r", "\n"),
    "\x00", "\uFFFD")
}

macro_rules! push_char(
    ($string:ident, $c:expr) => (
        str::push_char(&mut $string, $c)
    );
)

macro_rules! is_match(
    ($value:expr, $pattern:pat) => (
        match $value { $pattern => true, _ => false }
    );
)


impl lcss_lexer {
     fn from_vec(input: ~[u8], transform_function_whitespace: bool)
            -> ~lcss_lexer {
                let string_from_input = str::from_bytes(input);
                let string_from_input = preprocess(string_from_input);
                let input = str::to_bytes(string_from_input);
        ~lcss_lexer {
            length: input.len(),
            input: input,
            position: 0,
            transform_function_whitespace: transform_function_whitespace
        }
    }

    pub fn css__lexer_get_token(&self) -> (Token, Option<ParseError>) {
        if self.is_eof() { 
            (EOF, None) 
        }
        else { 
            self.consume_token()
        }
    }

    fn handle_transform_function_whitespace(&self, string: ~str)
            -> (Token, Option<ParseError>) {
        while !self.is_eof() {
            match self.current_char() {
                '\t' | '\n' | '\x0C' | ' ' => self.position += 1,
                '(' => { self.position += 1; return (Function(string), None) }
                _ => break,
            }
        }
        // Go back for one whitespace character.
        self.position -= 1;
        (Ident(string), None)
    }

    fn is_eof(&self) -> bool {
        self.position >= self.length
    }

    pub fn consume_token(&self) -> (Token, Option<ParseError>) {
        // Comments are special because they do not even emit a token,
        // unless they reach EOF which is an error.
        match self.consume_comments() {
            Some(result) => return result,
            None => ()
        }
        if self.is_eof() { return (EOF, None) }
        let c = self.current_char();
        match c {
            '-' => {
                if self.match_here(~"-->") {
                    self.position += 3;
                    (CDC, None)
                }
                else if self.next_is_namestart_or_escape() {
                    self.consume_ident()
                } else {
                    self.consume_numeric()
                }
            },
            '<' => {
                if self.match_here(~"<!--") {
                    self.position += 4;
                    (CDO, None)
                } else {
                    self.position += 1;
                    (Delim('<'), None)
                }
            },
            '0'..'9' | '.' | '+' => self.consume_numeric(),
            'u' | 'U' => self.consume_unicode_range(),
            'a'..'z' | 'A'..'Z' | '_' | '\\' => self.consume_ident(),
            _ if c >= '\x80' => self.consume_ident(), // Non-ASCII
            _ => {
                match self.consume_char() {
                    '\t' | '\n' | '\x0C' | ' ' => {
                        while !self.is_eof() {
                            match self.current_char() {
                                '\t' | '\n' | '\x0C' | ' '
                                    => self.position += 1,
                                _ => break,
                            }
                        }
                        (WhiteSpace, None)
                    },
                    '"' => self.consume_quoted_string(false),
                    '#' => self.consume_hash(),
                    '\'' => self.consume_quoted_string(true),
                    '(' => (OpenParenthesis, None),
                    ')' => (CloseParenthesis, None),
                    ':' => (Colon, None),
                    ';' => (Semicolon, None),
                    '@' => self.consume_at_keyword(),
                    '[' => (OpenSquareBraket, None),
                    ']' => (CloseSquareBraket, None),
                    '{' => (OpenCurlyBraket, None),
                    '}' => (CloseCurlyBraket, None),
                    _ => (Delim(c), None)
                }
            }
        }
    }

    fn consume_quoted_string(&self, single_quote: bool)
            -> (Token, Option<ParseError>) {
        let mut string: ~str = ~"";
        while !self.is_eof() {
            match self.consume_char() {
                '"' if !single_quote => return (String(string), None),
                '\'' if single_quote => return (String(string), None),
                '\n' | '\x0C' => {
                    return self.error_token(BadString, ~"Newline in quoted string");
                },
                '\\' => {
                    match self.next_n_chars(1) {
                        // Quoted newline
                        ['\n'] | ['\x0C'] => self.position += 1,
                        [] =>
                            return self.error_token(BadString, ~"EOF in quoted string"),
                        _ => push_char!(string, self.consume_escape())
                    }
                }
                c => push_char!(string, c),
            }
        }
        self.error_token(String(string), ~"EOF in quoted string")
    }

    fn consume_hash(&self) -> (Token, Option<ParseError>) {
        let string = self.consume_ident_string_rest();
        (if string == ~"" { Delim('#') } else { Hash(string) }, None)
    }

    fn consume_char(&self) -> char {
        let range = str::char_range_at(str::from_bytes(self.input), self.position);
        self.position = range.next;
        range.ch
    }

    fn match_here(&self, needle: ~str) -> bool {
        let mut i = self.position;
        if i + needle.len() > self.length { return false }
        let haystack: &str = str::from_bytes(self.input);
        for needle.each |c| { if haystack[i] != c { return false; } i += 1u; }
        return true;
    }

    fn consume_comments(&self)
            -> Option<(Token, Option<ParseError>)> {
        let vec_to_string: ~str = str::from_bytes(self.input);
        while self.match_here(~"/*") {
            self.position += 2; // consume /*
            match str::find_str_from(vec_to_string, "*/", self.position) {
                Some(end_position) => self.position = end_position + 2,
                None => {
                    self.position = self.length;
                    return Some(self.error_token(EOF, ~"Unclosed comment"))
                }
            }
        }
        None
    }

    fn consume_at_keyword(&self) -> (Token, Option<ParseError>) {
        (match self.consume_ident_string() {
            Some(string) => AtKeyword(string),
            None => Delim('@')
        }, None)
    }

    fn current_char(&self) -> char {
        str::char_at(str::from_bytes(self.input) , self.position)
    }

    fn next_is_namestart_or_escape(&self) -> bool {
        self.position += 1;
        let result = !self.is_eof() && self.is_namestart_or_escape();
        self.position -= 1;
        result
    }

    fn next_n_chars(&self, n: uint) -> ~[char] {
        let mut chars: ~[char] = ~[];
        let mut position = self.position;
        for n.times {
            if position >= self.length { break }
            let range = str::char_range_at(str::from_bytes(self.input), position);
            position = range.next;
            chars.push(range.ch);
        }
        chars
    }

    fn is_invalid_escape(&self) -> bool {
        match self.next_n_chars(2) {
            ['\\', '\n'] | ['\\', '\x0C'] | ['\\'] => true,
            _ => false,
        }
    }

    fn is_namestart_or_escape(&self) -> bool {
        match self.current_char() {
            'a'..'z' | 'A'..'Z' | '_' => true,
            '\\' => !self.is_invalid_escape(),
            c => c >= '\x80', // Non-ASCII
        }
    }


    fn consume_ident(&self) -> (Token, Option<ParseError>) {
        match self.consume_ident_string() {
            Some(string) => {
                if self.is_eof() { return (Ident(string), None) }
                match self.current_char() {
                    '\t' | '\n' | '\x0C' | ' '
                            if self.transform_function_whitespace => {
                        self.position += 1;
                        self.handle_transform_function_whitespace(string)
                    }
                    '(' => {
                        self.position += 1;
                        if ascii_lower(string) == ~"url" { self.consume_url() }
                        else { (Function(string), None) }
                    },
                    _ => (Ident(string), None)
                }
            },
            None => match self.current_char() {
                '-' => {
                    self.position += 1;
                    (Delim('-'), None)
                },
                '\\' => {
                    self.position += 1;
                    self.error_token(Delim('\\'), ~"Invalid escape")
                },
                _ => fail!(), // Should not have called consume_ident() here.
            }
        }
    }

    fn consume_ident_string(&self) -> Option<~str> {
        match self.current_char() {
            '-' => if !self.next_is_namestart_or_escape() { None }
                   else { Some(self.consume_ident_string_rest()) },
            '\\' if self.is_invalid_escape() => return None,
            _ if !self.is_namestart_or_escape() => return None,
            _ => Some(self.consume_ident_string_rest())
        }
    }

    fn consume_ident_string_rest(&self) -> ~str {
        let mut string = ~"";
        while !self.is_eof() {
            let c = self.current_char();
            let next_char = match c {
                'a'..'z' | 'A'..'Z' | '0'..'9' | '_' | '-' => {
                    self.position += 1; c },
                _ if c >= '\x80' => self.consume_char(), // Non-ASCII
                '\\' => {
                    if self.is_invalid_escape() { break }
                    self.position += 1;
                    self.consume_escape()
                },
                _ => break
            };
            push_char!(string, next_char)
        }
        string
    }

    fn char_from_hex(&self ,hex: &[char]) -> char {
        uint::from_str_radix(str::from_chars(hex), 16).get() as char
    }

    fn consume_escape(&self) -> char {
        let c = self.consume_char();
        match c {
            '0'..'9' | 'A'..'F' | 'a'..'f' => {
                let mut hex = ~[c];
                while hex.len() < 6 && !self.is_eof() {
                    let c = self.current_char();
                    match c {
                        '0'..'9' | 'A'..'F' | 'a'..'f' => {
                            hex.push(c); self.position += 1 },
                        _ => break
                    }
                }
                if !self.is_eof() {
                    match self.current_char() {
                        '\t' | '\n' | '\x0C' | ' ' => self.position += 1,
                        _ => ()
                    }
                }
                let c = self.char_from_hex(hex);
                if '\x00' < c && c <= MAX_UNICODE { c }
                else { '\uFFFD' } // Replacement character
            },
            c => c
        }
    }

    fn consume_url(&self) -> (Token, Option<ParseError>) {
        while !self.is_eof() {
            match self.current_char() {
                '\t' | '\n' | '\x0C' | ' ' => self.position += 1,
                '"' => return self.consume_quoted_url(false),
                '\'' => return self.consume_quoted_url(true),
                ')' => { self.position += 1; return (URL(~""), None) },
                _ => return self.consume_unquoted_url(),
            }
        }
        self.error_token(BadURL, ~"EOF in URL")
    }

    fn consume_quoted_url(&self, single_quote: bool)
            -> (Token, Option<ParseError>) {
        self.position += 1; // The initial quote
        let (token, err) = self.consume_quoted_string(single_quote);
        match err {
            Some(_) => {
                let (token, _) = self.consume_bad_url();
                (token, err)
            },
            None => match token {
                String(string) => self.consume_url_end(string),
                // consume_quoted_string() never returns a non-String token
                // without error:
                _ => fail!(),
            }
        }
    }

    fn consume_unquoted_url(&self) -> (Token, Option<ParseError>) {
        let mut string = ~"";
        while !self.is_eof() {
            let next_char = match self.consume_char() {
                '\t' | '\n' | '\x0C' | ' '
                    => return self.consume_url_end(string),
                ')' => return (URL(string), None),
                '\x00'..'\x08' | '\x0E'..'\x1F' | '\x7F'..'\x9F' // non-printable
                    | '"' | '\'' | '(' => return self.consume_bad_url(),
                '\\' => match self.next_n_chars(1) {
                    ['\n'] | ['\x0C'] | [] => return self.consume_bad_url(),
                    _ => self.consume_escape()
                },
                c => c
            };
            push_char!(string, next_char)
        }
        self.error_token(BadURL, ~"EOF in URL")
    }

    fn consume_url_end(&self, string: ~str)
            -> (Token, Option<ParseError>) {
        while !self.is_eof() {
            match self.consume_char() {
                '\t' | '\n' | '\x0C' | ' ' => (),
                ')' => return (URL(string), None),
                _ => return self.consume_bad_url()
            }
        }
        self.error_token(BadURL, ~"EOF in URL")
    }

    fn consume_bad_url(&self) -> (Token, Option<ParseError>) {
        // Consume up to the closing )
        while !self.is_eof() {
            match self.consume_char() {
                ')' => break,
                '\\' => self.position += 1, // Skip an escaped ) or \
                _ => ()
            }
        }
        self.error_token(BadURL, ~"Invalid URL syntax")
    }

    fn consume_unicode_range(&self)
            -> (Token, Option<ParseError>) {
        let next_3 = self.next_n_chars(3);
        // We got here with U or u
        assert!((next_3[0] == 'u') || (next_3[0] == 'U'));
        // Check if this is indeed an unicode range. Fallback on ident.
        if next_3.len() == 3 && next_3[1] == '+' {
            match next_3[2] {
                '0'..'9' | 'a'..'f' | 'A'..'F' => self.position += 2,
                _ => { return self.consume_ident() }
            }
        } else { return self.consume_ident() }

        let mut hex = ~[];
        while hex.len() < 6 && !self.is_eof() {
            let c = self.current_char();
            match c {
                '0'..'9' | 'A'..'F' | 'a'..'f' => {
                    hex.push(c); self.position += 1 },
                _ => break
            }
        }
        assert !(hex.len() > 0);
        let max_question_marks = 6u - hex.len();
        let mut question_marks = 0u;
        while question_marks < max_question_marks && !self.is_eof()
                && self.current_char() == '?' {
            question_marks += 1;
            self.position += 1
        }
        let start: char, end: char;
        if question_marks > 0 {
            start = self.char_from_hex(hex + vec::from_elem(question_marks, '0'));
            end = self.char_from_hex(hex + vec::from_elem(question_marks, 'F'));
        } else {
            start = self.char_from_hex(hex);
            hex = ~[];
            if !self.is_eof() && self.current_char() == '-' {
                self.position += 1;
                while hex.len() < 6 && !self.is_eof() {
                    let c = self.current_char();
                    match c {
                        '0'..'9' | 'A'..'F' | 'a'..'f' => {
                            hex.push(c); self.position += 1 },
                        _ => break
                    }
                }
            }
            end = if hex.len() > 0 { self.char_from_hex(hex) } else { start }
        }
        (if start > MAX_UNICODE || end < start {
            EmptyUnicodeRange
        } else {
            let end = if end <= MAX_UNICODE { end } else { MAX_UNICODE };
            UnicodeRange(start, end)
        }, None)
    }

    fn consume_numeric(&self) -> (Token, Option<ParseError>) {
        let c = self.consume_char();
        match c {
            '-' | '+' => self.consume_numeric_sign(c),
           '.' => {
                if self.is_eof() { return (Delim('.'), None) }
                match self.current_char() {
                    '0'..'9' => self.consume_numeric_fraction(~"."),
                    _ => (Delim('.'), None),
                }
            },
            '0'..'9' => self.consume_numeric_rest(c),
            _ => fail!(), 
        }
    }

    fn consume_numeric_sign(&self, sign: char)
            -> (Token, Option<ParseError>) {
        if self.is_eof() { return (Delim(sign), None) }
        match self.current_char() {
            '.' => {
                self.position += 1;
                if !self.is_eof()
                        && is_match!(self.current_char(), '0'..'9') {
                    self.consume_numeric_fraction(str::from_char(sign) + ~".")
                } else {
                    self.position -= 1;
                    (Delim(sign), None)
                }
            },
            '0'..'9' => self.consume_numeric_rest(sign),
            _ => (Delim(sign), None)
        }
    }

    fn consume_numeric_rest(&self, initial_char: char)
            -> (Token, Option<ParseError>) {
        let mut string = str::from_char(initial_char);
        while !self.is_eof() {
            let c = self.current_char();
            match c {
                '0'..'9' => { push_char!(string, c); self.position += 1 },
                '.' => {
                    self.position += 1;
                    if !self.is_eof()
                            && is_match!(self.current_char(), '0'..'9') {
                        push_char!(string, '.');
                        return self.consume_numeric_fraction(string);
                    } else {
                        self.position -= 1; break
                    }
                },
                _ => match self.consume_scientific_number(string) {
                    Ok(token) => return (token, None),
                    Err(s) => { string = s; break }
                }
            }
        }
        let value = Integer(int::from_str(
            // Remove any + sign as int::from_str() does not parse them.
            if string[0] != '+' as u8 { copy string }
            else { str::slice(string, 1, string.len()) }
        ).get()); // XXX handle overflow
        self.consume_numeric_end(string, value)
    }

    fn consume_numeric_fraction(&self, string: ~str)
            -> (Token, Option<ParseError>) {
        let mut string: ~str = string;
        while !self.is_eof() {
            match self.current_char() {
                '0'..'9' => push_char!(string, self.consume_char()),
                _ => match self.consume_scientific_number(string) {
                    Ok(token) => return (token, None),
                    Err(s) => { string = s; break }
                }
            }
        }
        let value = Float(float::from_str(string).get()); // XXX handle overflow
        self.consume_numeric_end(string, value)
    }


    fn consume_numeric_end(&self, string: ~str,
                           value: NumericValue) -> (Token, Option<ParseError>) {
        if self.is_eof() { return (Number(value, string), None) }
        (match self.current_char() {
            '%' => { self.position += 1; Percentage(value, string) },
            _ => {
                match self.consume_ident_string() {
                    Some(unit) => Dimension(value, string, unit),
                    None => Number(value, string),
                }
            },
        }, None)
    }


    fn consume_scientific_number(&self, string: ~str)
            -> Result<Token, ~str> {
        let next_3 = self.next_n_chars(3);
        let mut string: ~str = string;
        if (next_3.len() >= 2
            && (next_3[0] == 'e' || next_3[0] == 'E')
            && (is_match!(next_3[1], '0'..'9'))
        ) {
            push_char!(string, next_3[0]);
            push_char!(string, next_3[1]);
            self.position += 2;
        } else if (
            next_3.len() == 3
            && (next_3[0] == 'e' || next_3[0] == 'E')
            && (next_3[1] == '+' || next_3[1] == '-')
            && is_match!(next_3[2], '0'..'9')
        ) {
            push_char!(string, next_3[0]);
            push_char!(string, next_3[1]);
            push_char!(string, next_3[2]);
            self.position += 3;
        } else {
            return Err(string)
        }
        while !self.is_eof() && is_match!(self.current_char(), '0'..'9') {
            push_char!(string,self.consume_char())
        }
        let value = Float(float::from_str(string).get());
        Ok(Number(value, string))
    }

    pub fn error_token(&self ,t: Token, message: ~str) -> (Token, Option<ParseError>) {
        (t, Some(ParseError{message: message}))
    }

    pub fn css__lexer_create(input: @parserutils_inputstream) -> css_result {
    	CSS_GENERAL_OK
    }
}

pub fn lcss_lexer()->@lcss_lexer {
	@lcss_lexer{ transform_function_whitespace: false,
    input: ~[],
    length: 0, 
    position: 0 }
}


// ===========================================================================================================
// CSS-LEXER implementation/data-structs ends here 
// ===========================================================================================================

pub fn lcss_high_level(/*sheet:@css_stylesheet*/)-> @css_high_level
{
	let lwc_instance= lwc();
	@mut css_high_level
	{
		base:@css_rule
		{
			parent:@rule(0),		
			next:@mut NoRuleNode ,				
		    prev:@mut NoRuleNode ,				
		    rule_type  : CSS_RULE_UNKNOWN,		
			index : 0,		
			items : 0,		
			ptype : 0	
		},
		//rule_type : CSS_RULE_UNKNOWN,
		selector  : @css_rule_selector{
						/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

		 				selectors:~[],
		 				style:css_style
		 				{
		 					bytecode:~[] ,
							//used : 0,
							//allocated: 0,
							sheet:@NoStyleSheetNode
		 				},
					},
		charset   : @css_rule_charset{
		    		/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

		            encoding: lwc_instance.lwc_intern_string(@"")
		    	},
		import    : @css_rule_import{
					/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  :  rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/
					url:lwc_instance.lwc_intern_string(@""),
		            media:0,

		            sheet:@mut NoStyleSheetNode
				},
		media     : @css_rule_media{
					/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

					media:0,

					first_child:@mut NoRuleNode,
					last_child:@mut NoRuleNode
				},
		font_face : @css_rule_font_face{
		    		/*base :css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/
					font_face:@css_font_face 
						{
							font_family:lwc_instance.lwc_intern_string(@""),
							srcs:@css_font_face_src
							{
								location:lwc_instance.lwc_intern_string(@""),	
								bits:~[]
							},
							n_srcs:0,
		
								
							bits:~[]
						}
		    	},
		page      : @css_rule_page{
		    	/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

				selector:@css_selector 
						{
							combinator:~[],		/*< Combining selector */

							rule:@no_high_level_pointer,/*@css_rule_selector
								{//check for errors due to commenting this out
									/*base:css_rule
										{
										parent:@rule(0),		
				        				next:@mut NoRuleNode ,				
		                				prev:@mut NoRuleNode ,				
		                				rule_type  : CSS_RULE_UNKNOWN,		
			            				index : 0,		
			            				items : 0,		
			            				ptype : 0	
										},

		 							selectors:~[],
		 							style:css_style
		 								{
		 									bytecode:~[] ,
											used : 0,
											allocated: 0
		 								},
		 							},*/
							
							
							specificity:CSS_SPECIFICITY_A,			

							data:@css_selector_detail
							{
								qname:css_qname
								{
									ns : lwc_instance.lwc_intern_string(@"") ,
									name : lwc_instance.lwc_intern_string(@"") 
								},			
								value:css_selector_detail_value
								{
									string:~"",		
									a:0,
									b:0
								},	

								type_of     :0 ,    		   
								comb        :0 ,    		    
								next        :0 ,     		     
													            
								value_type  :0,		        
								negate      :0    		    
							}
						}	,
				style:@css_style
						{
		 					bytecode:~[] ,
							//used : 0,
							//allocated: 0,
							sheet:@NoStyleSheetNode
		 				},	
		    },
		   prev: @mut no_high_level_pointer,
		   next:@mut no_high_level_pointer 

	}
}



// ===========================================================================================================
// CSS-SELECTOR implementation/data-structs start here 
// ===========================================================================================================

pub fn css__selector_hash_create()-> css_result
{
	CSS_GENERAL_OK
}


// ===========================================================================================================
// CSS-SELECTOR implementation/data-structs ends here 
// ===========================================================================================================


// ===========================================================================================================
// CSS-STYLESHEET implementation/data-structs start here 
// ===========================================================================================================

pub type  css_import_notification_fn =  @extern fn(pw:~[u8],
		 parent:@css_stylesheet,  url:@lwc_string, media:u64) -> css_result;
pub type  css_url_resolution_fn =  @extern fn(pw:~[u8],
		base:~str, rel:@lwc_string , abs:@lwc_string ) -> css_result;
pub type  css_color_resolution_fn =  @extern fn(pw:~[u8],
		name:@lwc_string,  color:@css_color) -> css_result;
pub type  css_font_resolution_fn =  @extern fn(pw:~[u8],
		name:@lwc_string,  system_font:@css_system_font) -> css_result;


pub fn CINF(pw:~[u8], parent:@css_stylesheet,  url:@lwc_string, media:u64) -> css_result
{
	CSS_GENERAL_OK
}
pub  fn CURF(pw:~[u8],base:~str, rel:@lwc_string , abs:@lwc_string ) -> css_result
{
	CSS_GENERAL_OK
}
pub fn CCRF(pw:~[u8],name:@lwc_string,  color:@css_color) -> css_result
{
	CSS_GENERAL_OK
}
pub fn CFRF(pw:~[u8],name:@lwc_string,  system_font:@css_system_font) -> css_result
{
	CSS_GENERAL_OK
}





pub fn lcss_stylesheet(lwc_inst:&mut ~lwc)->@mut css_stylesheet {
	@mut css_stylesheet{
		                lwc_instance:lwc_inst,
		                //parser_instance: parser_inst,
		            	rule_count:0,			/*< Number of rules in sheet */
						rule_list:@mut no_high_level_pointer/*css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : CSS_RULE_UNKNOWN,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						}*/,			/*< List of rules in sheet */
						last_rule:@mut no_high_level_pointer/*css_rule 
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : CSS_RULE_UNKNOWN,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						}*/,			/*< Last rule in list */

						disabled:false,				/*< Whether this sheet is 
							                          * disabled */

						url:~"",				/*< URL of this sheet */
						title:~"",			/*< Title of this sheet */

						level:CSS_LEVEL_1  ,		/*< Language level of sheet */
						parser:@mut NoParserNode ,			/*< Core parser for sheet */
						parser_frontend:@mut NoLanguageNode,			/*< Frontend parser */
						//propstrings:@ mut[],		/*< Property strings, for parser */

						quirks_allowed:false,			/*< Quirks permitted */
						quirks_used:false,			/*< Quirks actually used */

						inline_style:false,			/*< Is an inline style */

						size:0 ,				/*< Size, in bytes */

		 				import:@CINF,	/*< Import notification function */
						import_pw:~[],			/*< Private word */

		 				resolve:@CURF,		/*< URL resolution function */
						resolve_pw:~[],			/*< Private word */

		 				color:@CCRF,		/*< Colour resolution function */
						color_pw:~[],				/*< Private word */

		/* System font resolution function */
		 				font:@CFRF,		
						font_pw:~[],				/*< Private word */


		// alloc:css_allocator_fn,			/*< Allocation function */
		//pw:~[u8],				/*< Private word */
	  
						cached_style:@NoStyleNode,	/*< Cache for style parsing */
	  
						string_vector:~[],            /*< Bytecode string vector */
						//string_vector_l:0,              /*< The string vector allocated
						// * length in entries */
						//string_vector_c:0 ,
						propstrings_call_count:0,
		propstrings_list:@[@"*", @"charset",@"import",@"media", @ "namespace", @ "font-face", @"page", @"aural",@ "braille", @ "embossed",@"handheld", @"print",
		@"projection", @ "screen", @ "speech", @ "tty", @ "tv", @ "all",@"first-child", @ "link", @ "visited", @ "hover", @ "active", @ "focus",
		@ "lang",@ "first",@ "root", @ "nth-child", @ "nth-last-child", @ "nth-of-type",@"nth-last-of-type", @ "last-child",@ "first-of-type",
		@ "last-of-type", @ "only-child", @ "only-of-type",@ "empty", @"target",@ "enabled", @ "disabled", @ "checked", @"not", @ "first-line", 
		@ "first-letter", @ "before",@ "after",@ "azimuth",@ "background", @ "background-attachment", @ "background-color", @ "background-image", 
		@"background-position",@"background-repeat", @"border",@"border-bottom", @ "border-bottom-color", @ "border-bottom-style", @ "border-bottom-width",
		@"border-collapse",@ "border-color",@ "border-left", @ "border-left-color", @ "border-left-style", @ "border-left-width",@ "border-right",
		@ "border-right-color", @ "border-right-style",@ "border-right-width",@ "border-spacing",@ "border-style", @ "border-top",@"border-top-color",
		@ "border-top-style",@ "border-top-width",@ "border-width", @ "bottom", @ "break-after", @ "break-before", @ "break-inside",@ "caption-side",
		@ "clear",@ "clip",@ "color",@ "columns",@ "column-count",@ "column-fill",@ "column-gap",@ "column-rule", @"column-rule-color",
		@ "column-rule-style",@ "column-rule-width",@ "column-span",@"column-width",@ "content", @ "counter-increment", @ "counter-reset",@"cue",
		@ "cue-after", @ "cue-before",@"cursor", @ "direction",@ "display",@"elevation", @ "empty-cells",@ "float",@ "font", @ "font-family",@"font-size",
		@"font-style", @ "font-variant",@"font-weight", @ "height", @"left", @"letter-spacing", @ "line-height",@ "list-style", @ "list-style-image",
		@"list-style-position",@"list-style-type",@ "margin", @ "margin-bottom",@"margin-left",@ "margin-right", @ "margin-top", @ "max-height",
		@ "max-width", @ "min-height", @ "min-width",@"opacity", @ "orphans",@ "outline", @ "outline-color", @"outline-style",@ "outline-width", @ "overflow",
		@ "padding", @ "padding-bottom",@ "padding-left",@ "padding-right",@ "padding-top",@ "page-break-after", @ "page-break-before",@ "page-break-inside",
		@ "pause",@ "pause-after",@ "pause-before", @ "pitch-range",@"pitch",@"play-during",@ "position",@ "quotes", @ "richness",@"right", @"speak-header",
		@ "speak-numeral",@ "speak-punctuation",@"speak", @"speech-rate",@"stress",@ "table-layout", @ "text-align",@ "text-decoration",@ "text-indent",
		@ "text-transform",@"top", @ "unicode-bidi", @ "vertical-align",@ "visibility",@"voice-family", @"volume",@"white-space",@ "widows", @ "width",
		@ "word-spacing",@ "z-index",@ "inherit",@ "important",@"none",@"both", @ "fixed",@"scroll",@ "transparent", @ "no-repeat",@ "repeat-x",@"repeat-y",
		@ "repeat",@ "hidden",@"dotted", @ "dashed", @ "solid",@ "double",@ "groove", @ "ridge",@"inset",@"outset",@ "thin", @ "medium",@"thick", @"collapse",
		@ "separate",@ "auto",@ "ltr",@ "rtl", @"inline", @ "block",@ "list-item",@ "run-in",@ "inline-block", @ "table",@ "inline-table",@ "table-row-group",
		@ "table-header-group",@ "table-footer-group",@ "table-row", @ "table-column-group", @ "table-column",@ "table-cell",@ "table-caption",@ "below",
		@ "level",@ "above",@ "higher",@ "lower",@ "show",@ "hide",@ "xx-small",@ "x-small",@ "small",@ "large",@ "x-large",@ "xx-large",@ "larger",
		@ "smaller", @ "normal",@ "italic",@ "oblique",@ "small-caps",@ "bold",@"bolder", @ "lighter",@ "inside",@ "outside", @ "disc",
		@ "circle",@"square",@ "decimal",@"decimal-leading-zero", @ "lower-roman", @ "upper-roman", @ "lower-greek",@ "lower-latin",@ "upper-latin",
		@ "armenian",@ "georgian", @ "lower-alpha",@ "upper-alpha",@ "invert",@ "visible",@ "always",@ "avoid",@ "x-low",@"low", @ "high", @ "x-high",
		@"static",@"relative", @ "absolute",@ "once",@ "digits",@ "continuous", @ "code", @ "spell-out",@ "x-slow",@ "slow",@ "fast",@ "x-fast",@ "faster",
		@ "slower",@ "center",@ "justify",@ "capitalize",@ "uppercase",@ "lowercase",@ "embed",@ "bidi-override",@ "baseline",@ "sub",@ "super", 
		@ "text-top",@ "middle",@ "text-bottom",@ "silent",@ "x-soft",@ "soft",@ "loud", @ "x-loud", @"pre",@ "nowrap",@"pre-wrap",@"pre-line",
		@ "leftwards",@ "rightwards",@ "left-side", @ "far-left", @ "center-left",@ "center-right",@ "far-right",@ "right-side",@ "behind",@ "rect",@"open-quote",
		@ "close-quote",@ "no-open-quote",@ "no-close-quote",@ "attr",@ "counter",@ "counters",@ "crosshair",@ "default",@ "pointer",@ "move",@ "e-resize",
		@ "ne-resize",@ "nw-resize",@ "n-resize", @ "se-resize",@ "sw-resize",@ "s-resize",@ "w-resize",@ "text",@ "wait",@ "help",@ "progress",@ "serif",
		@ "sans-serif",@ "cursive",@ "fantasy",@ "monospace",@ "male",@ "female",@ "child",@ "mix",@ "underline",@ "overline",@ "line-through",@ "blink",
		@ "rgb", @ "rgba",@ "hsl",@"hsla",@ "-libcss-left",@ "-libcss-center",@ "-libcss-right",@ "currentColor", @"odd", @ "even",@ "src",@ "local",
		@ "initial",@ "format",@ "woff",@ "truetype",@ "opentype", @"embedded-opentype", @"svg",@ "column",@ "avoid-page", @ "avoid-column",@ "balance",
		@"aliceblue",@ "antiquewhite",@ "aqua",@"aquamarine",@ "azure",@ "beige",@ "bisque",@"black",@"blanchedalmond",@"blue",@ "blueviolet",@"brown",
		@ "burlywood",@ "cadetblue",@ "chartreuse",@ "chocolate", @ "coral",@ "cornflowerblue", @ "cornsilk", @ "crimson", @ "cyan",@ "darkblue",@ "darkcyan",
		@ "darkgoldenrod",@ "darkgray",@ "darkgreen",@ "darkgrey",@"darkkhaki", @ "darkmagenta",@ "darkolivegreen",@ "darkorange",@ "darkorchid",@ "darkred",
		@ "darksalmon",@ "darkseagreen",@ "darkslateblue",@ "darkslategray",@ "darkslategrey",@ "darkturquoise",@ "darkviolet",@ "deeppink", @ "deepskyblue",
		@ "dimgray",@ "dimgrey",@ "dodgerblue",@ "feldspar",@ "firebrick",@ "floralwhite", @ "forestgreen",@ "fuchsia", @ "gainsboro",@ "ghostwhite",
	    @ "gold",@ "goldenrod",@ "gray",@ "green",@ "greenyellow",@ "grey",@ "honeydew",@ "hotpink",@ "indianred",@ "indigo",@ "ivory",@ "khaki",@ "lavender",
	    @ "lavenderblush",@ "lawngreen",@ "lemonchiffon",@ "lightblue",@ "lightcoral",@ "lightcyan",@ "lightgoldenrodyellow",@ "lightgray",@ "lightgreen",
	    @ "lightgrey",@ "lightpink",@ "lightsalmon",@ "lightseagreen",@ "lightskyblue", @ "lightslateblue", @ "lightslategray",@ "lightslategrey",
	    @ "lightsteelblue", @ "lightyellow",@ "lime",@ "limegreen",@ "linen", @ "magenta",@"maroon",@ "mediumaquamarine",@ "mediumblue", @ "mediumorchid",
	    @ "mediumpurple", @ "mediumseagreen",@ "mediumslateblue",@ "mediumspringgreen",@ "mediumturquoise",@"mediumvioletred", @ "midnightblue",@ "mintcream", 
	    @ "mistyrose",@ "moccasin",@ "navajowhite",@ "navy", @ "oldlace", @ "olive",@ "olivedrab",@ "orange",@ "orangered",@"orchid",@"palegoldenrod",
	    @ "palegreen",@ "paleturquoise",@ "palevioletred", @ "papayawhip",@ "peachpuff",@ "peru",@ "pink",@ "plum",@ "powderblue", @ "purple",@ "red",
	    @ "rosybrown",@ "royalblue", @ "saddlebrown",@ "salmon",@ "sandybrown",@ "seagreen",@ "seashell",@ "sienna", @ "silver", @ "skyblue",@ "slateblue",
	    @ "slategray", @"slategrey",@ "snow",@ "springgreen",@ "steelblue", @ "tan", @ "teal",@ "thistle",@ "tomato",@ "turquoise",@"violet",@ "violetred",
	    @ "wheat",@ "white", @ "whitesmoke",@"yellow",@ "yellowgreen"],
	    propstrings:/*lwc_inst.lwc_intern_string(@"")*/~[]
		            } 
}

impl css_stylesheet {

pub fn css__propstrings_unref(&self)
	{
		self.propstrings_call_count  -=1;

		if (self.propstrings_call_count  == 0) {
			let mut  i=0;

			while ( i < self.propstrings_list.len())
			{
				self.lwc_instance.lwc_string_unref(self.propstrings[i]);
				i += 1;
			}
				
		}
	}

	pub fn css__propstrings_get(&self)->css_result
	{
		if (self.propstrings_call_count > 0) {
			self.propstrings_call_count += 1;
		} 
		else {
			let mut i =0;
			while(i < self.propstrings_list.len())
			{
				self.propstrings.push(self.lwc_instance.lwc_intern_string(self.propstrings_list[i]));
				i += 1;
			}
			self.propstrings_call_count += 1;
		}
		
		CSS_PROPSTRINGS_OK(copy self.propstrings)
	}
		

pub fn css__stylesheet_rule_add_selector(&self,/*sheet:  @css_stylesheet , */
		 mut curRule:@mut css_high_level , selector: @css_selector )
{
	match(curRule.base.rule_type)
	 {
	 	CSS_RULE_SELECTOR=>{},
	 	_=>{fail!();}
	 }

		
	curRule.selector.selectors.push(selector);//check later
		
   	
	curRule.base.items += 1;
	curRule.selector.selectors[curRule.base.items].rule = @high_level_pointer(curRule);//problem 2
	 
}


pub fn  css__stylesheet_rule_create(@self,sheet:@css_stylesheet ,  rule_type:css_rule_type/*,
		css_rule **rule*/)->css_result
{
	let mut high_level_css_struct:@css_high_level =  lcss_high_level(/*self*/);
	high_level_css_struct.base.rule_type = rule_type;
	CSS_RULE_CREATED_OK(high_level_css_struct)	
	//CSS_GENERAL_OK
}

pub fn css__stylesheet_string_add(&self,sheet:css_stylesheet , string:@lwc_string /*, uint32_t *string_number*/)-> css_result
{
	let string_number:@mut u32 = @mut 0;
	let strCount =  sheet.string_vector.len() as u32;
    /* search for the string in the existing vector */
	while (*string_number < strCount)
	{
		//let res:lwc_result ;
		let mut isEqual = false;
		isEqual = lwc::lwc_string_isequal(string, sheet.string_vector[*string_number]);
		
		*string_number += 1;
		if(isEqual)
		{
			self.lwc_instance.lwc_string_unref(string);
			return CSS_STRING_ADD_OK(string_number);
		}
		
	}

	/* string does not exist in current vector, add a new one */
	sheet.string_vector.push(copy string);
	return CSS_STRING_ADD_OK(string_number);
	
}

pub fn css__stylesheet_string_get(/*sheet:@css_stylesheet,*/ &self,mut string_number:u32/*, lwc_string **string*/)->css_result
{
	string_number -= 1;
    if string_number > self.string_vector.len() as u32
    {
    	return CSS_BADPARM;
    }
    CSS_STRING_GET(self.string_vector[string_number])
}

 pub fn css_stylesheet_create(&self,params:@css_stylesheet_params /*,
		css_allocator_fn alloc, void *alloc_pw, 
		css_stylesheet **stylesheet*/)->css_result
{
	let sheet = lcss_stylesheet(lwc());
	let mut Result=self.css__propstrings_get();
	match(copy Result) {
		CSS_PROPSTRINGS_OK(x) => sheet.propstrings = x,
		_=>{ return Result}
	}
	sheet.inline_style = params.inline_style;
	let mut charsetDetect :css_charset_source ;
	if(params.charset.len()  == 0)
	{
		charsetDetect=  CSS_CHARSET_DEFAULT;
	}
	else{
		charsetDetect =  CSS_CHARSET_DICTATED;
	}
    //sheet.parser =  @mut SomeParserNode(css__parser_create(copy params.charset,charsetDetect,lcss_language(sheet)));
	if (params.inline_style) {
		
		sheet.parser =  @mut SomeParserNode(css__parser_create_for_inline_style(copy params.charset,charsetDetect,lcss_language(sheet)));
	} else {
		sheet.parser =  @mut SomeParserNode(css__parser_create(copy params.charset,charsetDetect,lcss_language(sheet)));
	}
	sheet.quirks_allowed = params.allow_quirks;
	let mut optparams:@css_parser_optparams = css_parser_optparams_instance() ;
	
	if (params.allow_quirks) {
		optparams.quirks = true;
		match(sheet.parser)
		{
			@SomeParserNode(x)=>Result = x.css__parser_setopt( CSS_PARSER_QUIRKS,optparams),
			_=>{}
		}
		match (copy Result)
		{
			CSS_GENERAL_OK=>{}
			_=>{
				self.css__propstrings_unref();
				return Result;
			}
		}
	}
	sheet.level = params.level;
    Result = css__language_create(sheet, sheet.parser/*alloc, alloc_pw,*//*&sheet->parser_frontend*/);
	match(copy Result)
	{
		CSS_LANGUAGE_CREATED_OK(lan)=> sheet.parser_frontend = @mut SomeLanguageNode(lan),
		_=>{}
	}
    //TODO uncomment when selector hashtable is implemented
	/*Result =  css__selector_hash_create();
	match(copy Result)
	{
		CSS_SELECTOR_CREATE_OK(sel)=> sheet.selector=sel,
		_=>{}
	}
*/

    sheet. url = copy params.url;
    sheet. title = copy params.title;

	sheet.resolve =copy params.resolve;
	sheet.resolve_pw =copy params.resolve_pw;

	sheet.import = copy params.import;
	sheet.import_pw = copy params.import_pw;

	sheet.color = copy params.color;
	sheet.color_pw = copy params.color_pw;

	sheet.font = copy params.font;
	sheet.font_pw = copy params.font_pw;

	/*sheet.alloc = alloc;
	sheet.pw = alloc_pw;*/

	CSS_STYLESHEET_CREATE_OK(sheet)
}

pub fn css_stylesheet_append_data( &self,
		data:~[u8])-> css_result
{
	/*if (sheet == NULL || data == NULL)
		return CSS_BADPARM;

	if (sheet->parser == NULL)
		return CSS_INVALID;*/
		match(self.parser)
		{
			@SomeParserNode(x)=>return x.css__parser_parse_chunk( data),
			_=> return CSS_INVALID
		}

	
}
pub fn css_stylesheet_data_done(&self/*css_stylesheet *sheet*/)-> css_result
{
	let mut Result:css_result;
	match(self.parser)
	{
		@SomeParserNode(x)=> Result = x.css__parser_completed(),
		_=>return CSS_INVALID
	}

	self.parser_frontend = @mut NoLanguageNode;
	self.parser = @mut NoParserNode;

    let mut iter:@mut css_high_level_ptr = self.rule_list;
    loop 
    	{
    		match(iter)
    		{
    			@high_level_pointer(x)=> {
    				iter=x.next;
    				match(x.base.rule_type)
    				{
    					CSS_RULE_UNKNOWN=>{},
    					CSS_RULE_CHARSET=>{},
    					CSS_RULE_IMPORT=>{
    						let mut importOfHighLevel@css_rule_import=x.import;
    						match(x.import.sheet)
    						{
    							@SomeStyleSheetNode(x)=>{},
    							@NoStyleSheetNode=> return CSS_IMPORTS_PENDING
    						}
    						
    					},
    					_=>{break;}
    				}
    			},
    			@no_high_level_pointer=> {break;}    
    		}
       	}
    


	CSS_GENERAL_OK
}
pub fn css_stylesheet_next_pending_import(&self/*,
		url:@lwc_string , media:u64*/)->css_result
{
	let mut iter:@mut css_high_level_ptr = self.rule_list;
	let mut url:@lwc_string;
	let mut media:u64;
    loop 
    	{
    		match(iter)
    		{
    			@high_level_pointer(x)=> {
    				iter=x.next;
    				match(x.base.rule_type)
    				{
    					CSS_RULE_UNKNOWN=>{},
    					CSS_RULE_CHARSET=>{},
    					CSS_RULE_IMPORT=>{
    						let mut importOfHighLevel@css_rule_import=x.import;
    						match(x.import.sheet)
    						{
    							@SomeStyleSheetNode(x)=>{},
    							@NoStyleSheetNode=>  {
    								url = x.import.url;
    								media = x.import.media;
    								return CSS_IMPORTS_PENDING_OK(url,media);
    							}
    						}
    						
    					},
    					_=>{break;}
    				}
    			},
    			@no_high_level_pointer=> {break;}    
    		}
       	}
	

	return CSS_INVALID;
}

pub fn css_stylesheet_register_import(&self,
		import:@mut css_stylesheet)-> css_result
{
	let mut iter:@mut css_high_level_ptr = self.rule_list;
	//let mut url:@lwc_string;
	//let mut media:u64;
    loop 
    	{
    		match(iter)
    		{
    			@high_level_pointer(x)=> {
    				
    				match(x.base.rule_type)
    				{
    					CSS_RULE_UNKNOWN=>{},
    					CSS_RULE_CHARSET=>{},
    					CSS_RULE_IMPORT=>{
    						
    						match(x.import.sheet)
    						{
    							@SomeStyleSheetNode(x)=>{},
    							@NoStyleSheetNode=>  {
    								x.import.sheet=@mut SomeStyleSheetNode(import);
    								return CSS_GENERAL_OK;
    							}
    						}
    						
    					},
    					_=>{break;}
    				}
    				iter=x.next;
    			},
    			@no_high_level_pointer=> {break;}    
    		}
       	}
	

	return CSS_INVALID;
	
}
pub fn css_stylesheet_get_language_level(&self )-> css_result
{
	/*if (sheet == NULL || level == NULL)
		return CSS_BADPARM;

	*level = sheet->level;*/

	return CSS_GET_LANGUAGE_LEVEL(self.level);
}

pub fn css_stylesheet_get_url(&self)-> css_result
{
	/*if (sheet == NULL || url == NULL)
		return CSS_BADPARM;

	*url = sheet->url;*/

	return CSS_GET_URL( copy self.url);
}
pub fn css_stylesheet_get_title(&self)-> css_result
{
	return CSS_GET_TITLE(copy self.title);
}

pub fn css_stylesheet_quirks_allowed(&self)-> css_result
{
	return CSS_IS_QUIRK_ALLOWED(self.quirks_allowed);
}

pub fn css_stylesheet_used_quirks(&self)-> css_result
{
	return CSS_IS_QUIRK_USED(self.quirks_used);
}

pub fn css_stylesheet_get_disabled(&self)-> css_result
{
	return CSS_GET_SHEET_DISABLED(self.disabled);
}
pub fn css_stylesheet_set_disabled(&self,disabled:bool)-> css_result
{
	self.disabled = disabled;
	return CSS_GENERAL_OK;
}
pub fn css_stylesheet_size(&self, size:uint)-> css_result
{
    CSS_GENERAL_OK//(size)
	//not implemented
}
pub fn css__stylesheet_style_create(@mut self)-> css_result
{
	match(self.cached_style)
	{
		@SomeStyleNode(Style)=>{
			self.cached_style= @NoStyleNode;
			CSS_STYLECREATED_OK(@SomeStyleNode(Style));
		},
		@NoStyleNode=>{}
	}
	let mut Style=@css_style
	{
		bytecode:~[] ,
		//used : 0,
		//allocated: CSS_STYLE_DEFAULT_SIZE,
		sheet:@SomeStyleSheetNode(self)

	};
	CSS_STYLECREATED_OK(@SomeStyleNode(Style))
}

 pub fn css__stylesheet_merge_style(target:@css_style ,  style:@css_style)-> css_result
{
	
	target.bytecode = vec::append(copy target.bytecode, style.bytecode);
	CSS_GENERAL_OK

}

pub fn css__stylesheet_style_append(style:@css_style,  css_code:css_code_t)-> css_result
{
  style.bytecode.push(css_code);
  CSS_GENERAL_OK
}
//check this functn
pub fn css__stylesheet_style_vappend(style:@css_style,  css_code:~[css_code_t])-> css_result
{
	style.bytecode = vec::append(copy style.bytecode, css_code);
	CSS_GENERAL_OK
}



}



// ===========================================================================================================
// CSS-STYLESHEET implementation/data-structs ends here 
// ===========================================================================================================

pub fn lcss_language(sheet:@mut css_stylesheet)->@css_language {
	let empty_lwc_string = sheet.lwc_instance.lwc_intern_string(@"");
	let stack:@vec<context_entry> = ~[]; 
	
	//@css_language {
					

				let	css_language_instance = @css_language {
							sheet:sheet,
							lwc_instance:sheet.lwc_instance,		
				    		STACK_CHUNK:32,
							context:stack, 
							state:CHARSET_PERMITTED,	
							strings:copy sheet.propstrings,
							
							default_namespace:empty_lwc_string,	
							
							namespaces:@css_namespace
							{
								prefix:empty_lwc_string,	
								uri:empty_lwc_string	
							},	
							num_namespaces:0	
		
		            };
	return css_language_instance;
}
pub fn  css__language_create( sheet:@css_stylesheet,parserNode:@mut css_parser_node) -> css_result
	{
		let lwc_inst=lwc();
	let empty_lwc_string = lwc_inst.lwc_intern_string(@"");
	let stack:@vec<context_entry> = ~[];
	
	//@css_language {
					

				let	css_language_instance = @mut css_language {
							sheet:sheet,
							lwc_instance:lwc_inst,		
				    		STACK_CHUNK:32,
							context:stack, 
							state:CHARSET_PERMITTED,	
							strings:copy sheet.propstrings,
							
							default_namespace:empty_lwc_string,	
							
							namespaces:@css_namespace
							{
								prefix:empty_lwc_string,	
								uri:empty_lwc_string	
							},	
							num_namespaces:0	
		
		            };
	
		
		css_language_instance.sheet=sheet;

		

		/*let params = @css_parser_optparams {
			quirks:false,
			event_handler: css_parser_event_handler_
			{
				handler:language_handle_event,
				pw:css_language_instance
			}
		};*/ //see later
		
		
		
		return CSS_LANGUAGE_CREATED_OK(css_language_instance);
	}


impl css_language
 {

pub fn  language_handle_event(&self, event_type:css_parser_event, 
			tokens:~[~str], css_language_instance:@css_language)-> css_result
	{
		match (event_type) {
			
			CSS_PARSER_START_STYLESHEET => {
			 	self.handleStartStylesheet(css_language_instance, tokens)
			}
			
			CSS_PARSER_END_STYLESHEET=>{
			 	self.handleEndStylesheet(css_language_instance, tokens)
			}
			
			CSS_PARSER_START_RULESET=>{
			 	self.handleStartRuleset(css_language_instance, tokens)
			}
			
			CSS_PARSER_END_RULESET=>{
			 	self.handleEndRuleset(css_language_instance, tokens)
			}
			
			CSS_PARSER_START_ATRULE=>{
				self.handleStartAtRule(css_language_instance, tokens)
			}
			
			CSS_PARSER_END_ATRULE=>{
				self.handleEndAtRule(css_language_instance, tokens)
			}
			
			CSS_PARSER_START_BLOCK=>{
				self.handleStartBlock(css_language_instance, tokens)
			}
			
			CSS_PARSER_END_BLOCK=>{
				self.handleEndBlock(css_language_instance, tokens)
			}
			
			CSS_PARSER_BLOCK_CONTENT=>{
				self.handleBlockContent(css_language_instance, tokens)
			}
			
			CSS_PARSER_DECLARATION=>{
				self.handleDeclaration(css_language_instance, tokens)
			}
		}
	}


pub fn  css__language_create(@mut self, sheet:@css_stylesheet) -> css_result
	{
		
		self.sheet=sheet;

		

		/*let params = @css_parser_optparams {
			quirks:false,
			event_handler: css_parser_event_handler_
			{
				handler:language_handle_event,
				pw:css_language_instance
			}
		};*/ //see later
		
		
		
		return CSS_LANGUAGE_CREATED(self);
	}

pub fn handleStartStylesheet(&self, c:@css_language, vector:~[~str]) -> css_result
	{
		// let pResult:parserutils_result;
		// UNUSED(vector);
		let entry:context_entry = context_entry 
		{
			event_type: CSS_PARSER_START_STYLESHEET, 
			data:@css_rule 
				{
					parent:@rule(0),		
				    next:@mut NoRuleNode ,				
		            prev:@mut NoRuleNode ,				
		            rule_type  : CSS_RULE_UNKNOWN,		
			        index : 0,		
			        items : 0,		
			        ptype : 0	
				},	
    	 };
	    c.context.push(entry);
		CSS_GENERAL_OK
	}

	pub fn handleEndStylesheet(&self, c:@css_language, vector:~[~str])->css_result
	{


	    if(c.context.len()==0)
	    {
	    	return CSS_INVALID
	    }
		match(c.context.last().event_type)
		{
			CSS_PARSER_START_STYLESHEET=>{},
			_=>return CSS_INVALID
		}

		c.context.pop();
		// parserutils_error perror;
		// context_entry *entry;

		// UNUSED(vector);

		// assert(c != NULL);

		// entry = parserutils_stack_get_current(c->context);
		// if (entry == NULL || entry->type != CSS_PARSER_START_STYLESHEET)
		// 	return CSS_INVALID;

		// perror = parserutils_stack_pop(c->context, NULL);
		// if (perror != PARSERUTILS_OK) {
		// 	return css_result_from_parserutils_error(perror);
		// }

		CSS_GENERAL_OK
	}

	pub fn handleStartRuleset(&self, c:@css_language , vector:~[~str])->css_result 
	{
		/*parserutils_error pResult;
		css_result cResult;
		context_entry entry = { CSS_PARSER_START_RULESET, NULL };*/
		let mut cResult:css_result;
		let entry:context_entry = context_entry 
		{
			event_type: CSS_PARSER_START_STYLESHEET, 
			data:@css_rule 
				{
					parent:@rule(0),		
				    next:@mut NoRuleNode ,				
		            prev:@mut NoRuleNode ,				
		            rule_type  : CSS_RULE_UNKNOWN,		
			        index : 0,		
			        items : 0,		
			        ptype : 0	
				},	
    	 };
		let cur:@context_entry ;
		let mut parent_rule :@css_rule ;
		let mut curRule :@css_high_level ;
		

		// assert(c != NULL);

		/* Retrieve parent rule from stack, if any */
		if c.context.len() !=0
		{
			cur=@ c.context.last();
			match(cur.event_type  )
			{
				CSS_PARSER_START_STYLESHEET =>{},
				_=>{parent_rule = cur.data;}
			}
		}
		
		/*cur = parserutils_stack_get_current(c->context);
		if (cur != NULL && cur->type != CSS_PARSER_START_STYLESHEET)
			parent_rule = cur->data;*/
        match(self.sheet.css__stylesheet_rule_create(c.sheet, CSS_RULE_SELECTOR))
        {

		CSS_RULE_CREATED_OK( css_rule_selector)=>{curRule=css_rule_selector},
		_=>{return CSS_INVALID;}
		
        }
		if vector.len() != 0
		{
			//cResult = self.parseSelectorList(c, vector, curRule);
		}

		// if (vector != NULL) {
		// 	/* Parse selectors, if there are any */
		// 	error = parseSelectorList(c, vector, rule);
		// 	if (error != CSS_OK) {
		// 		css__stylesheet_rule_destroy(c->sheet, rule);
		// 		return error;
		// 	}
		// }

		// entry.data = rule;

		// perror = parserutils_stack_push(c->context, (void *) &entry);
		// if (perror != PARSERUTILS_OK) {
		// 	css__stylesheet_rule_destroy(c->sheet, rule);
		// 	return css_result_from_parserutils_error(perror);
		// }

		// error = css__stylesheet_add_rule(c->sheet, rule, parent_rule);
		// if (error != CSS_OK) {
		// 	parserutils_stack_pop(c->context, NULL);
		// 	css__stylesheet_rule_destroy(c->sheet, rule);
		// 	return error;
		// }

		// /* Flag that we've had a valid rule, so @import/@namespace/@charset 
		//  * have no effect. */
		// c->state = HAD_RULE;

		/* Rule is now owned by the sheet, so no need to destroy it */

		  CSS_GENERAL_OK
	}

pub fn handleEndRuleset(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

pub fn handleStartAtRule(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

pub fn handleEndAtRule(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

pub fn handleStartBlock(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

pub fn handleEndBlock(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

pub fn handleBlockContent(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

pub fn handleDeclaration(&self, c:@css_language , vector:~[~str])->css_result
{
	CSS_GENERAL_OK	
}

 }


// ===========================================================================================================
// CSS-LANGUAGE implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// Important.h and important.c implementation/data-structs starts here 
// ===========================================================================================================

pub fn important()->@important
{
	@important
	{
		 lpu_instance : lpu(),
	     lwc_instance : lwc()
	}
}


impl important
{
	pub fn convert_Int_2_Enum(&self, intVal : uint) -> css_properties_e
	{
		let mut enumVal : css_properties_e = CSS_PROP_AZIMUTH; // Initializing to remove compilation error

		match(intVal)
		{
			0x000 =>   enumVal = 		CSS_PROP_AZIMUTH,		
			      0x001  =>   enumVal = 	  CSS_PROP_BACKGROUND_ATTACHMENT		,
			      0x002                    =>   enumVal =  CSS_PROP_BACKGROUND_COLOR		,
			      0x003                    =>   enumVal =  CSS_PROP_BACKGROUND_IMAGE		 ,
			      0x004          =>   enumVal =   CSS_PROP_BACKGROUND_POSITION,	
			      0x005          =>   enumVal =   CSS_PROP_BACKGROUND_REPEAT	,
			      0x006          =>   enumVal =   CSS_PROP_BORDER_COLLAPSE	,
			      0x007          =>   enumVal =   CSS_PROP_BORDER_SPACING	,	
			      0x008          =>   enumVal =   CSS_PROP_BORDER_TOP_COLOR,
			      0x009          =>   enumVal =   CSS_PROP_BORDER_RIGHT_COLOR,	
			      0x00a                    =>   enumVal = CSS_PROP_BORDER_BOTTOM_COLOR	,
			      0x00b          =>   enumVal =   CSS_PROP_BORDER_LEFT_COLOR	,
			      0x00c                    =>   enumVal = CSS_PROP_BORDER_TOP_STYLE	,
			      0x00d                    =>   enumVal = CSS_PROP_BORDER_RIGHT_STYLE,	
			      0x00e                    =>   enumVal = CSS_PROP_BORDER_BOTTOM_STYLE,	
			      0x00f                    =>   enumVal = CSS_PROP_BORDER_LEFT_STYLE,	
			      0x010                    =>   enumVal = CSS_PROP_BORDER_TOP_WIDTH	,
			      0x011                    =>   enumVal =  CSS_PROP_BORDER_RIGHT_WIDTH	,
			      0x012                    =>   enumVal =  CSS_PROP_BORDER_BOTTOM_WIDTH	,
			      0x013                    =>   enumVal =  CSS_PROP_BORDER_LEFT_WIDTH	,
			      0x014                    =>   enumVal =  CSS_PROP_BOTTOM			,
			      0x015                    =>   enumVal =  CSS_PROP_CAPTION_SIDE	,	
			      0x016          =>   enumVal = CSS_PROP_CLEAR	,
			      0x017          =>   enumVal =  CSS_PROP_CLIP			,
			      0x018          =>   enumVal =  CSS_PROP_COLOR			,
			      0x019          =>   enumVal = CSS_PROP_CONTENT	,
			      0x01a          =>   enumVal = CSS_PROP_COUNTER_INCREMENT	,
			      0x01b          =>   enumVal = CSS_PROP_COUNTER_RESET		,
			      0x01c          =>   enumVal = CSS_PROP_CUE_AFTER		,
			      0x01d          =>   enumVal = CSS_PROP_CUE_BEFORE	,
			      0x01e          =>   enumVal = CSS_PROP_CURSOR		,	
			      0x01f          =>   enumVal = CSS_PROP_DIRECTION	,	
			      0x020          =>   enumVal = CSS_PROP_DISPLAY		,
			      0x021          =>   enumVal = CSS_PROP_ELEVATION	,	
			      0x022          =>   enumVal = CSS_PROP_EMPTY_CELLS	,	
			      0x023          =>   enumVal = CSS_PROP_FLOAT		,	
			      0x024          =>   enumVal = CSS_PROP_FONT_FAMILY	,
			      0x025          =>   enumVal =  CSS_PROP_FONT_SIZE		,
			      0x026          =>   enumVal = CSS_PROP_FONT_STYLE	,	
			      0x027          =>   enumVal = CSS_PROP_FONT_VARIANT	,	
			      0x028          =>   enumVal = CSS_PROP_FONT_WEIGHT	,
			      0x029          =>   enumVal = CSS_PROP_HEIGHT			,
			      0x02a          =>   enumVal = CSS_PROP_LEFT			,
			      0x02b          =>   enumVal =   CSS_PROP_LETTER_SPACING,		
			      0x02c          =>   enumVal = CSS_PROP_LINE_HEIGHT	,	
			      0x02d          =>   enumVal =   CSS_PROP_LIST_STYLE_IMAGE	,
			      0x02e          =>   enumVal =   CSS_PROP_LIST_STYLE_POSITION,	
			      0x02f          =>   enumVal = CSS_PROP_LIST_STYLE_TYPE	,
			      0x030          =>   enumVal =   CSS_PROP_MARGIN_TOP		,
			      0x031          =>   enumVal =   CSS_PROP_MARGIN_RIGHT	,
			      0x032          =>   enumVal =   CSS_PROP_MARGIN_BOTTOM		,
			      0x033          =>   enumVal =   CSS_PROP_MARGIN_LEFT		,
			      0x034          =>   enumVal = CSS_PROP_MAX_HEIGHT		,
			      0x035          =>   enumVal =   CSS_PROP_MAX_WIDTH		,
			      0x036          =>   enumVal =   CSS_PROP_MIN_HEIGHT	,	
			      0x037          =>   enumVal = CSS_PROP_MIN_WIDTH	,	
			      0x038          =>   enumVal = CSS_PROP_ORPHANS			,
			      0x039          =>   enumVal = CSS_PROP_OUTLINE_COLOR,		
			      0x03a          =>   enumVal = CSS_PROP_OUTLINE_STYLE,		
			      0x03b          =>   enumVal = CSS_PROP_OUTLINE_WIDTH	,
			      0x03c          =>   enumVal = CSS_PROP_OVERFLOW			,
			      0x03d          =>   enumVal = CSS_PROP_PADDING_TOP,
			      0x03e          =>   enumVal = CSS_PROP_PADDING_RIGHT	,		
			      0x03f          =>   enumVal = CSS_PROP_PADDING_BOTTOM	,	
			      0x040          =>   enumVal =   CSS_PROP_PADDING_LEFT		,
			      0x041          =>   enumVal = CSS_PROP_PAGE_BREAK_AFTER	,
			      0x042          =>   enumVal = CSS_PROP_PAGE_BREAK_BEFORE,	
			      0x043          =>   enumVal = CSS_PROP_PAGE_BREAK_INSIDE,	
			      0x044          =>   enumVal =   CSS_PROP_PAUSE_AFTER		,
			      0x045          =>   enumVal =   CSS_PROP_PAUSE_BEFORE		,
			      0x046          =>   enumVal =   CSS_PROP_PITCH_RANGE		,
			      0x047          =>   enumVal = CSS_PROP_PITCH			,
			      0x048          =>   enumVal = CSS_PROP_PLAY_DURING		,
			      0x049          =>   enumVal = CSS_PROP_POSITION		,
			      0x04a          =>   enumVal = CSS_PROP_QUOTES		,	
			      0x04b          =>   enumVal = CSS_PROP_RICHNESS		,
			      0x04c          =>   enumVal =   CSS_PROP_RIGHT			,
			      0x04d          =>   enumVal = CSS_PROP_SPEAK_HEADER	,	
			      0x04e          =>   enumVal = CSS_PROP_SPEAK_NUMERAL	,		
			      0x04f          =>   enumVal = CSS_PROP_SPEAK_PUNCTUATION	,
			      0x050          =>   enumVal = CSS_PROP_SPEAK				,
			      0x051          =>   enumVal = CSS_PROP_SPEECH_RATE		,
			      0x052          =>   enumVal = CSS_PROP_STRESS			,
			      0x053          =>   enumVal = CSS_PROP_TABLE_LAYOUT	,
			      0x054          =>   enumVal = CSS_PROP_TEXT_ALIGN		,
			      0x055          =>   enumVal = CSS_PROP_TEXT_DECORATION	,
			      0x056          =>   enumVal = CSS_PROP_TEXT_INDENT		,
			      0x057          =>   enumVal = CSS_PROP_TEXT_TRANSFORM	,	
			      0x058          =>   enumVal = CSS_PROP_TOP			,
			      0x059          =>   enumVal = CSS_PROP_UNICODE_BIDI	,	
			      0x05a          =>   enumVal = CSS_PROP_VERTICAL_ALIGN ,		
			      0x05b          =>   enumVal = CSS_PROP_VISIBILITY	,	
			      0x05c          =>   enumVal =   CSS_PROP_VOICE_FAMILY	,
			      0x05d          =>   enumVal =    CSS_PROP_VOLUME				,
			      0x05e          =>   enumVal =   CSS_PROP_WHITE_SPACE		,
			      0x05f          =>   enumVal =   CSS_PROP_WIDOWS			,
			      0x060          =>   enumVal =   CSS_PROP_WIDTH			,
			      0x061          =>   enumVal =   CSS_PROP_WORD_SPACING	,	
			      0x062          =>   enumVal =   CSS_PROP_Z_INDEX		,
			      0x063          =>   enumVal =   CSS_PROP_OPACITY		,
			      0x064          =>   enumVal =   CSS_PROP_BREAK_AFTER		,	
			      0x065          =>   enumVal =   CSS_PROP_BREAK_BEFORE	,	
			      0x066          =>   enumVal =   CSS_PROP_BREAK_INSIDE	,	
			      0x067          =>   enumVal =   CSS_PROP_COLUMN_COUNT	,	
			      0x068          =>   enumVal =   CSS_PROP_COLUMN_FILL	,	
			      0x069          =>   enumVal =   CSS_PROP_COLUMN_GAP,		
			      0x06a          =>   enumVal =   CSS_PROP_COLUMN_RULE_COLOR,
			      0x06b          =>   enumVal =    CSS_PROP_COLUMN_RULE_STYLE	,
			      0x06c          =>   enumVal =   CSS_PROP_COLUMN_RULE_WIDTH	,
			      0x06d          =>   enumVal =   CSS_PROP_COLUMN_SPAN		,
			      0x06e			=>   enumVal = 	 CSS_PROP_COLUMN_WIDTH		,
			      0x06f			=>   enumVal = 	CSS_N_PROPERTIES		,

		}
		return (enumVal);
	}


	pub fn convert_enum_op_azimuth_2_u32(&self, enumVal : op_azimuth) -> u32
	{
		let value : u32 ;

		match(enumVal)
		{
			AZIMUTH_ANGLE			=>   value   =     0x0080,
							AZIMUTH_LEFTWARDS		=>   value   =     0x0040,
							AZIMUTH_RIGHTWARDS		=>   value   =     0x0041,
							AZIMUTH_BEHIND			=>   value   =     (1<<5),
							AZIMUTH_LEFT_SIDE		=>   value   =     0x0000,
							AZIMUTH_FAR_LEFT		=>   value   =     0x0001,
							AZIMUTH_LEFT			=>   value   =     0x0002,
							AZIMUTH_CENTER_LEFT		=>   value   =     0x0003,
							AZIMUTH_CENTER			=>   value   =     0x0004,
							AZIMUTH_CENTER_RIGHT		=>   value   =     0x0005,
							AZIMUTH_RIGHT			=>   value   =     0x0006,
							AZIMUTH_FAR_RIGHT		=>   value   =     0x0007,
							AZIMUTH_RIGHT_SIDE		=>   value   =     0x0008,
		}
		return (value);
	}


	pub fn convert_enum_op_backgroundColor_2_u32(&self, enumVal : op_background_color) -> u32
	{
		let value : u32 ;

		match(enumVal)
		{
			BACKGROUND_COLOR_TRANSPARENT	=>   value   =     0x0000,
							BACKGROUND_COLOR_CURRENT_COLOR	=>   value   =     0x0001,
							BACKGROUND_COLOR_SET		=>   value   =     0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_backgroundImage_2_u32(&self, enumVal : op_background_image) -> u32
	{
		let value : u32 ;

		match(enumVal)
		{
			BACKGROUND_IMAGE_URI		=>   value   =     0x0080,
							BACKGROUND_IMAGE_NONE		=>   value   =     0x0000,
		}
		return (value);
	}

	pub fn convert_enum_op_backgroundPosition_2_u32(&self, enumVal : op_background_position) -> u32
	{
		let value : u32 ;

		match(enumVal)
		{
			BACKGROUND_POSITION_HORZ_SET	=>   value   =     0x0080,
							BACKGROUND_POSITION_HORZ_CENTER	=>   value   =     0x0000,
							BACKGROUND_POSITION_HORZ_RIGHT	=>   value   =     0x0010,
							BACKGROUND_POSITION_HORZ_LEFT	=>   value   =     0x0020,
							BACKGROUND_POSITION_VERT_SET	=>   value   =     0x0008,
							//BACKGROUND_POSITION_VERT_CENTER	=>   value   =     0x0000,
							BACKGROUND_POSITION_VERT_BOTTOM	=>   value   =     0x0001,
							BACKGROUND_POSITION_VERT_TOP	=>   value   =     0x0002,
		}
		return (value);
	}


	pub fn convert_enum_op_borderSpacing_2_u32(&self, enumVal : op_border_spacing) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			BORDER_SPACING_SET	=>	 value = 0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_borderWidth_2_u32(&self, enumVal : op_border_width) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			BORDER_WIDTH_SET		=>   value   =     0x0080,
							BORDER_WIDTH_THIN		=>   value   =     0x0000,
							BORDER_WIDTH_MEDIUM		=>   value   =     0x0001,
							BORDER_WIDTH_THICK		=>   value   =     0x0002,
		}
		return (value);
	}


	pub fn convert_enum_op_bottom_2_u32(&self, enumVal : op_bottom) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			BOTTOM_SET			=>   value   =     0x0080,
							BOTTOM_AUTO			=>   value   =    0x0000,
		}
		return (value);
	}

	pub fn convert_enum_op_clip_2_u32(&self, enumVal : op_clip) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			CLIP_SHAPE_MASK			=>   value   =     0x0087,
							CLIP_SHAPE_RECT			=>   value   =     0x0080,
							CLIP_RECT_TOP_AUTO		=>   value   =     0x0008,
							CLIP_RECT_RIGHT_AUTO		=>   value   =     0x0010,
							CLIP_RECT_BOTTOM_AUTO		=>   value   =     0x0020,
							CLIP_RECT_LEFT_AUTO		=>   value   =     0x0040,
							CLIP_AUTO			=>   value   =     0x0000,
		}
		return (value);
	}

	pub fn convert_enum_op_color_2_u32(&self, enumVal : op_color) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			COLOR_TRANSPARENT		=>   value   =     0x0000,
							COLOR_CURRENT_COLOR		=>   value   =     0x0001,
							COLOR_SET			=>   value   =     0x0080,
		}
		return (value);
	}

	pub fn convert_enum_op_columCount_2_u32(&self, enumVal : op_column_count) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			COLUMN_COUNT_AUTO		=>   value   =     0x0000,
							COLUMN_COUNT_SET		=>   value   =     0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_content_2_u32(&self, enumVal : op_content) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{	
			CONTENT_STRING			=>   value   =     0x0080,
							CONTENT_URI			=>   value   =     0x0081,
							CONTENT_COUNTER			=>   value   =     0x0082,
							CONTENT_COUNTERS		=>   value   =     0x0083,
							CONTENT_ATTR			=>   value   =     0x0084,
							CONTENT_COUNTER_STYLE_SHIFT	=>   value   =     8,
							//CONTENT_COUNTERS_STYLE_SHIFT	=>   value   =     8,
							CONTENT_NORMAL			=>   value   =     0x0000,
							CONTENT_NONE			=>   value   =     0x0001,
							CONTENT_OPEN_QUOTE		=>   value   =     0x0002,
							CONTENT_CLOSE_QUOTE		=>   value   =     0x0003,
							CONTENT_NO_OPEN_QUOTE		=>   value   =     0x0004,
							CONTENT_NO_CLOSE_QUOTE		=>   value   =     0x0005,
		}
		return (value);
	}


	pub fn convert_enum_op_counterIncrement_2_u32(&self, enumVal : op_counter_increment) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			COUNTER_INCREMENT_NONE		=>   value = 0x0000,
							COUNTER_INCREMENT_NAMED		=>   value = 0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_cursor_2_u32(&self, enumVal : op_cursor) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			CURSOR_URI			=>   value   =     0x0080,

							CURSOR_AUTO			=>   value   =     0x0000,
							CURSOR_CROSSHAIR		=>   value   =     0x0001,
							CURSOR_DEFAULT			=>   value   =     0x0002,
							CURSOR_POINTER			=>   value   =     0x0003,
							CURSOR_MOVE			=>   value   =     0x0004,
							CURSOR_E_RESIZE			=>   value   =     0x0005,
							CURSOR_NE_RESIZE		=>   value   =     0x0006,
							CURSOR_NW_RESIZE		=>   value   =     0x0007,
							CURSOR_N_RESIZE			=>   value   =     0x0008,
							CURSOR_SE_RESIZE		=>   value   =     0x0009,
							CURSOR_SW_RESIZE		=>   value   =     0x000a,
							CURSOR_S_RESIZE			=>   value   =     0x000b,
							CURSOR_W_RESIZE			=>   value   =     0x000c,
							CURSOR_TEXT			=>   value   =     0x000d,
							CURSOR_WAIT			=>   value   =     0x000e,
							CURSOR_HELP			=>   value   =     0x000f,
							CURSOR_PROGRESS			=>   value   =     0x0010,
		}
		return (value);
	}


	pub fn convert_enum_op_elevation_2_u32(&self, enumVal : op_elevation) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			ELEVATION_ANGLE			=>   value   =     0x0080,
							ELEVATION_BELOW			=>   value   =     0x0000,
							ELEVATION_LEVEL			=>   value   =     0x0001,
							ELEVATION_ABOVE			=>   value   =     0x0002,
							ELEVATION_HIGHER		=>   value   =     0x0003,
							ELEVATION_LOWER			=>   value   =     0x0004,
		}
		return (value);
	}



	pub fn convert_enum_op_fontfamily_2_u32(&self, enumVal : op_font_family) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			FONT_FAMILY_STRING		=>   value   =     0x0080,
							FONT_FAMILY_IDENT_LIST		=>   value   =     0x0081,

							FONT_FAMILY_END			=>   value   =     0x0000,

							FONT_FAMILY_SERIF		=>   value   =     0x0001,
							FONT_FAMILY_SANS_SERIF		=>   value   =     0x0002,
							FONT_FAMILY_CURSIVE		=>   value   =     0x0003,
							FONT_FAMILY_FANTASY		=>   value   =     0x0004,
							FONT_FAMILY_MONOSPACE		=>   value   =     0x0005,
		}
		return (value);
	}


	pub fn convert_enum_op_fontSize_2_u32(&self, enumVal : op_font_size) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			FONT_SIZE_DIMENSION		=>   value   =     0x0080,
							FONT_SIZE_XX_SMALL		=>   value   =     0x0000,
							FONT_SIZE_X_SMALL		=>   value   =     0x0001,
							FONT_SIZE_SMALL			=>   value   =     0x0002,
							FONT_SIZE_MEDIUM		=>   value   =     0x0003,
							FONT_SIZE_LARGE			=>   value   =     0x0004,
							FONT_SIZE_X_LARGE		=>   value   =     0x0005,
							FONT_SIZE_XX_LARGE		=>   value   =     0x0006,
							FONT_SIZE_LARGER		=>   value   =     0x0007,
							FONT_SIZE_SMALLER		=>   value   =     0x0008,
		}
		return (value);
	}


	pub fn convert_enum_op_letterSpacing_2_u32(&self, enumVal : op_letter_spacing) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			LETTER_SPACING_SET		=>   value   =     0x0080,
							LETTER_SPACING_NORMAL		=>   value   =     0x0000,
		}
		return (value);
	}


	pub fn convert_enum_op_lineHeight_2_u32(&self, enumVal : op_line_height) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			LINE_HEIGHT_NUMBER		=>   value   =     0x0080,
							LINE_HEIGHT_DIMENSION		=>   value   =     0x0081,
							LINE_HEIGHT_NORMAL		=>   value   =     0x0000,
		}
		return (value);
	}


	pub fn convert_enum_op_maxHeight_2_u32(&self, enumVal : op_max_height) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			MAX_HEIGHT_SET			=>   value   =     0x0080,
							MAX_HEIGHT_NONE			=>   value   =     0x0000,
		}
		return (value);
	}


	pub fn convert_enum_op_minHeight_2_u32(&self, enumVal : op_min_height) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			MIN_HEIGHT_SET			=>	value =  0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_opacity_2_u32(&self, enumVal : op_opacity) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			OPACITY_SET			=>   value   =     0x0080,
		}
		return (value);
	}



	pub fn convert_enum_op_orphans_2_u32(&self, enumVal : op_orphans) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			ORPHANS_SET			=>   value   =     0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_outlineColor_2_u32(&self, enumVal : op_outline_color) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			OUTLINE_COLOR_TRANSPARENT	=>   value   =     0x0000,
							OUTLINE_COLOR_CURRENT_COLOR	=>   value   =     0x0001,
							OUTLINE_COLOR_INVERT		=>   value   =     0x0002,
							OUTLINE_COLOR_SET		=>   value   =     0x0080,
		}
		return (value);
	}


	pub fn convert_enum_op_pitch_2_u32(&self, enumVal : op_pitch) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			PITCH_FREQUENCY			=>   value   =     0x0080,

							PITCH_X_LOW			=>   value   =     0x0000,
							PITCH_LOW			=>   value   =     0x0001,
							PITCH_MEDIUM			=>   value   =     0x0002,
							PITCH_HIGH			=>   value   =     0x0003,
							PITCH_X_HIGH			=>   value   =     0x0004,
		}
		return (value);
	}


	pub fn convert_enum_op_playDuring_2_u32(&self, enumVal : op_play_during) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			PLAY_DURING_TYPE_MASK		=>   value   =     0x009f,
							PLAY_DURING_URI			=>   value   =     0x0080,
							PLAY_DURING_MIX			=>   value   =     (1<<6),
							PLAY_DURING_REPEAT		=>   value   =     (1<<5),
							PLAY_DURING_AUTO		=>   value   =     0x0000,
							PLAY_DURING_NONE		=>   value   =     0x0001,
		}
		return (value);
	}


	pub fn convert_enum_op_Zindex_2_u32(&self, enumVal : op_z_index) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			Z_INDEX_SET			=>   value   =     0x0080,
							Z_INDEX_AUTO			=>   value   =     0x0000,
		}
		return (value);
	}


	pub fn convert_enum_op_volume_2_u32(&self, enumVal : op_volume) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			VOLUME_NUMBER			=>   value   =     0x0080,
							VOLUME_DIMENSION		=>   value   =     0x0081,
							VOLUME_SILENT			=>   value   =     0x0000,
							VOLUME_X_SOFT			=>   value   =     0x0001,
							VOLUME_SOFT			=>   value   =     0x0002,
							VOLUME_MEDIUM			=>   value   =     0x0003,
							VOLUME_LOUD			=>   value   =     0x0004,
							VOLUME_X_LOUD			=>   value   =     0x0005,
		}
		return (value);
	}




	pub fn convert_enum_op_voiceFamily_2_u32(&self, enumVal : op_voice_family) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			VOICE_FAMILY_STRING		=>   value   =         0x0080,
							VOICE_FAMILY_IDENT_LIST		=> 	value =		0x0081,
							VOICE_FAMILY_END		=>   value   =     0x0000,
							VOICE_FAMILY_MALE		=>   value   =     0x0001,
							VOICE_FAMILY_FEMALE		=>   value   =     0x0002,
							VOICE_FAMILY_CHILD		=>   value   =     0x0003,
		}
		return (value);
	}

	pub fn convert_enum_op_verticalAlign_2_u32(&self, enumVal : op_vertical_align) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			VERTICAL_ALIGN_SET		=>   value   =     0x0080,

							VERTICAL_ALIGN_BASELINE		=>   value   =     0x0000,
							VERTICAL_ALIGN_SUB		=>   value   =     0x0001,
							VERTICAL_ALIGN_SUPER		=>   value   =     0x0002,
							VERTICAL_ALIGN_TOP		=>   value   =     0x0003,
							VERTICAL_ALIGN_TEXT_TOP		=>   value   =     0x0004,
							VERTICAL_ALIGN_MIDDLE		=>   value   =     0x0005,
							VERTICAL_ALIGN_BOTTOM		=>   value   =     0x0006,
							VERTICAL_ALIGN_TEXT_BOTTOM	=>   value   =     0x0007,
		}
		return (value);
	}


	pub fn convert_enum_op_playDuring_2_u32(&self, enumVal : op_play_during) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			PLAY_DURING_TYPE_MASK		=>   value   =     0x009f,
							PLAY_DURING_URI			=>   value   =     0x0080,
							PLAY_DURING_MIX			=>   value   =     (1<<6),
							PLAY_DURING_REPEAT		=>   value   =     (1<<5),

							PLAY_DURING_AUTO		=>   value   =     0x0000,
							PLAY_DURING_NONE		=>   value   =     0x0001,
		}
		return (value);
	}


	pub fn convert_enum_op_speechRate_2_u32(&self, enumVal : op_speech_rate) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			SPEECH_RATE_SET			=>   value   =     0x0080,

							SPEECH_RATE_X_SLOW		=>   value   =     0x0000,
							SPEECH_RATE_SLOW		=>   value   =     0x0001,
							SPEECH_RATE_MEDIUM		=>   value   =     0x0002,
							SPEECH_RATE_FAST		=>   value   =     0x0003,
							SPEECH_RATE_X_FAST		=>   value   =     0x0004,
							SPEECH_RATE_FASTER		=>   value   =     0x0005,
							SPEECH_RATE_SLOWER		=>   value   =     0x0006,
		}
		return (value);
	}


	pub fn convert_enum_op_quotes_2_u32(&self, enumVal : op_quotes) -> u32
	{
		let value : u32 ;
		match(enumVal)
		{
			QUOTES_STRING			=>   value   =     0x0080,
							QUOTES_NONE			=>   value   =     0x0000,
		}
		return (value);
	}


	pub fn css_parse_important(&self, cssLang : @css_language, tokenVector : ~[~css_token], result : ~u8) -> css_result
	{
		// I think there is no need of this function, since we can ceck empty space here only...
		//consumeWhiteSpace(vector, cntx); 

		if tokenVector.is_empty() == true
		{
			return CSS_INVALID;
		}

		for tokenVector.each |&token|
		{
			match(token.token_type)
			{
				//condition to replace consumeWhiteSpace(vector, cntx); 
				CSS_TOKEN_S  		=>  (),	// do nothing
							CSS_TOKEN_IDENT		=>    return CSS_INVALID,
							_			=>   {
								// Commenting temporarily to avoid compilation error
								// Q: The problem is lwc_instance is not accessible / recognised from here.

								/*
								   if lwc::lwc_instance.lwc_string_caseless_isequal(token.idata, cssLang.strings[IMPORTANT]) == true 
								   {
								   result |= FLAG_IMPORTANT;
								   }	
								   else
								   {
								   return CSS_INVALID;
								   }
								 */
							}
			} // match block ends
		} // for block ends

		return CSS_GENERAL_OK;
	}

	pub fn css_make_style_important (&self, style : &css_style)
	{
		let mut styleBytecode : ~[u32] = copy style.bytecode;
		//let mut styleLen : u32 = style.used; 
		let mut styleOffset : u32 = 0;

		// while (styleOffset < styleLen)
		while (styleOffset < styleBytecode.len() as u32)
		{
			//let prop :  css_properties_e;
			let mut flags : u32;
			let mut value : u32;	

			// Q: What would below mentioned code return ? i.e
			let mut propVal : u32 = styleBytecode[styleOffset]; 		

			io::println(fmt!("value of propVal is %?", propVal));
			// extracting propVal components, setting IMPORTANT FLAG
			let mut prop  = (propVal as uint) & (0x3ff as uint);
			flags = ((propVal >> 10) & 0xff) | (1<<0);       //  replacing  FLAG_IMPORTANT by 1<<0 ;
			value = propVal >> 18;	

			// writing propVal back to bytecodes
			styleBytecode[styleOffset] = ((prop & 0x3ff) as uint | (flags << 10) as uint | (((value as uint) & (0x3fff as uint)) << 18 as uint)) as u32;

			styleOffset += 1;	

			// Advance past any porp-specific data		
			if ((((propVal >> 10) & 0xff) & 0x2) == false as u32)
			{
				// convert_Int_2_Enum is used to convert int to corresponding enum value
				let enumVal : css_properties_e = self.convert_Int_2_Enum(prop);
				match(enumVal) 
				{
					CSS_PROP_AZIMUTH  => {

						//if ((value & ~self.convert_Enum_2_u32(AZIMUTH_BEHIND)) == self.convert_Enum_2_u32(AZIMUTH_ANGLE))
						if (value & !(self.convert_enum_op_azimuth_2_u32(AZIMUTH_BEHIND))) == self.convert_enum_op_azimuth_2_u32(AZIMUTH_ANGLE)
						{
							styleOffset += 2; // length + units 
						}
					},
							  CSS_PROP_BORDER_TOP_COLOR  |  CSS_PROP_BORDER_RIGHT_COLOR  | CSS_PROP_BORDER_BOTTOM_COLOR  |  CSS_PROP_BORDER_LEFT_COLOR  |  CSS_PROP_BACKGROUND_COLOR | CSS_PROP_COLUMN_RULE_COLOR  =>
							  {
								  	assert!(BACKGROUND_COLOR_SET == BORDER_COLOR_SET);
									assert!(BACKGROUND_COLOR_SET == COLUMN_RULE_COLOR_SET);

								  if (value == self.convert_enum_op_backgroundColor_2_u32(BACKGROUND_COLOR_SET))
								  {
									  styleOffset += 1; // colour 
								  }
							  },

							  CSS_PROP_BACKGROUND_IMAGE |  CSS_PROP_CUE_AFTER  | CSS_PROP_CUE_BEFORE  | CSS_PROP_LIST_STYLE_IMAGE  => 
							  {
								 	assert!(BACKGROUND_IMAGE_URI == CUE_AFTER_URI);
								        assert!(BACKGROUND_IMAGE_URI == CUE_BEFORE_URI);
								  	assert!(BACKGROUND_IMAGE_URI == LIST_STYLE_IMAGE_URI);

								  if (value == self.convert_enum_op_backgroundImage_2_u32(BACKGROUND_IMAGE_URI))
								  {
									  styleOffset += 1; // string table entry 
								  }
							  },

							  CSS_PROP_BACKGROUND_POSITION  =>
							  { 
								  if ((value & 0xf0) == self.convert_enum_op_backgroundPosition_2_u32(BACKGROUND_POSITION_HORZ_SET))
								  {
									  styleOffset += 2; // length + units 
								  }

								  if ((value & 0x0f) == self.convert_enum_op_backgroundPosition_2_u32(BACKGROUND_POSITION_VERT_SET))
								  {
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_BORDER_SPACING  => 
							  {
								  if (value == self.convert_enum_op_borderSpacing_2_u32(BORDER_SPACING_SET))
								  {
									  styleOffset += 4; // two length + units 
								  }
							  },

							  CSS_PROP_BORDER_TOP_WIDTH  | CSS_PROP_BORDER_RIGHT_WIDTH |  CSS_PROP_BORDER_BOTTOM_WIDTH  |  CSS_PROP_BORDER_LEFT_WIDTH  |	 CSS_PROP_OUTLINE_WIDTH  | CSS_PROP_COLUMN_RULE_WIDTH  => 
							  {
								  	assert!(BORDER_WIDTH_SET == OUTLINE_WIDTH_SET);
								  	assert!(BORDER_WIDTH_SET == COLUMN_RULE_WIDTH_SET);

								  if (value == self.convert_enum_op_borderWidth_2_u32(BORDER_WIDTH_SET))
								  {
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_MARGIN_TOP  |  CSS_PROP_MARGIN_RIGHT | CSS_PROP_MARGIN_BOTTOM  | CSS_PROP_MARGIN_LEFT  | CSS_PROP_BOTTOM  | CSS_PROP_LEFT  | CSS_PROP_RIGHT  | CSS_PROP_TOP  | CSS_PROP_HEIGHT |  CSS_PROP_WIDTH  |  CSS_PROP_COLUMN_WIDTH  | CSS_PROP_COLUMN_GAP  => 
							  {
								  	assert!(BOTTOM_SET == LEFT_SET);
								  	assert!(BOTTOM_SET == RIGHT_SET);
								  	assert!(BOTTOM_SET == TOP_SET);
								  	assert!(BOTTOM_SET == HEIGHT_SET);
								  	assert!(BOTTOM_SET == MARGIN_SET);
								  	assert!(BOTTOM_SET == WIDTH_SET);
								   	assert!(BOTTOM_SET == COLUMN_WIDTH_SET);
								   	assert!(BOTTOM_SET == COLUMN_GAP_SET);

								  if (value == self.convert_enum_op_bottom_2_u32(BOTTOM_SET))
								  {
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_CLIP  => 
							  {
								  if ((value & self.convert_enum_op_clip_2_u32(CLIP_SHAPE_MASK)) == self.convert_enum_op_clip_2_u32(CLIP_SHAPE_RECT))
								  {
									  if ((value & self.convert_enum_op_clip_2_u32(CLIP_RECT_TOP_AUTO)) == 0)
									  {
										  styleOffset += 2; // length + units 
									  }
									  if ((value & self.convert_enum_op_clip_2_u32(CLIP_RECT_RIGHT_AUTO)) == 0)
									  {
										  styleOffset += 2; // length + units 
									  }
									  if ((value & self.convert_enum_op_clip_2_u32(CLIP_RECT_BOTTOM_AUTO)) == 0)
									  {
										  styleOffset += 2; // length + units 
									  }
									  if ((value & self.convert_enum_op_clip_2_u32(CLIP_RECT_LEFT_AUTO)) == 0)
									  {
										  styleOffset += 2; // length + units 
									  }
								  }
							  },

							  CSS_PROP_COLOR  => 
							  {
								  if (value == self.convert_enum_op_color_2_u32(COLOR_SET))
								  {				
									  styleOffset += 1; // colour
								  }
							  },

							  CSS_PROP_COLUMN_COUNT  => 
							  {
								  if (value == self.convert_enum_op_columCount_2_u32(COLUMN_COUNT_SET))
								  {
									  styleOffset += 1; // colour 
								  }
							  },

							  CSS_PROP_CONTENT  => 
							  {
								  while (value != self.convert_enum_op_content_2_u32(CONTENT_NORMAL) && value != self.convert_enum_op_content_2_u32(CONTENT_NONE)) 
								  {
									  if (value & 0xff) ==  (self.convert_enum_op_content_2_u32(CONTENT_COUNTER))
									  {
										  styleOffset += 1; // string table entry 
									  }
									  else if (value & 0xff) ==  	 self.convert_enum_op_content_2_u32(CONTENT_URI) 
									  {
										  styleOffset += 1; // string table entry 
									  }
									  else if (value & 0xff) ==  	 self.convert_enum_op_content_2_u32(CONTENT_ATTR)   
									  {
										  styleOffset += 1; // string table entry 
									  }
									  else if (value & 0xff) ==  	 self.convert_enum_op_content_2_u32(CONTENT_STRING) 
									  {
										  styleOffset += 1; // string table entry 
									  }
									  else if (value & 0xff) ==  self.convert_enum_op_content_2_u32(CONTENT_COUNTERS)   
									  {
										  styleOffset+=2; // two string entries 
									  }
									  else 
									  {
										  if (((value & 0xff) == self.convert_enum_op_content_2_u32(CONTENT_OPEN_QUOTE)) ||
												  ((value & 0xff) == self.convert_enum_op_content_2_u32(CONTENT_CLOSE_QUOTE)) ||
												  ((value & 0xff) == self.convert_enum_op_content_2_u32(CONTENT_NO_OPEN_QUOTE)) ||
												  ((value & 0xff) == self.convert_enum_op_content_2_u32(CONTENT_NO_CLOSE_QUOTE)) )
										  {
											  break;
										  }
									  } // else block ends
								  }// while block ends

								  value = styleBytecode[styleOffset];
								  styleOffset += 1;
							  },

							  CSS_PROP_COUNTER_INCREMENT  | CSS_PROP_COUNTER_RESET  => 
							  {
								  // assert(COUNTER_INCREMENT_NONE == COUNTER_RESET_NONE);

								  while (value !=  self.convert_enum_op_counterIncrement_2_u32(COUNTER_INCREMENT_NONE)) {
									  styleOffset += 2; // string + integer 

									  value = styleBytecode[styleOffset];
									  styleOffset += 1;
								  }
							  },

							  CSS_PROP_CURSOR  =>
							  { 
								  while (value == self.convert_enum_op_cursor_2_u32(CURSOR_URI)) {
									  styleOffset += 1; // string table entry 

									  value = styleBytecode[styleOffset];
									  styleOffset += 1;
								  }
							  },	

							  CSS_PROP_ELEVATION  => 
							  {
								  if (value == self.convert_enum_op_elevation_2_u32(ELEVATION_ANGLE)){			
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_FONT_FAMILY  => 
							  {
								  while (value != self.convert_enum_op_fontfamily_2_u32(FONT_FAMILY_END)) 
								  {
									  if ( (value == self.convert_enum_op_fontfamily_2_u32(FONT_FAMILY_STRING)) ||
											  (value == self.convert_enum_op_fontfamily_2_u32(FONT_FAMILY_IDENT_LIST)) )
									  {
										  styleOffset += 1; // string table entry
										  break;
									  }


									  value = styleBytecode[styleOffset];
									  styleOffset += 1;
								  }
							  },

							  CSS_PROP_FONT_SIZE  => 
							  {
								  if (value == self.convert_enum_op_fontSize_2_u32(FONT_SIZE_DIMENSION)) {
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_LETTER_SPACING  |  CSS_PROP_WORD_SPACING  => 
							  {
								  assert!(LETTER_SPACING_SET == WORD_SPACING_SET);

								  if (value == self.convert_enum_op_letterSpacing_2_u32(LETTER_SPACING_SET)){
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_LINE_HEIGHT  => 
							  {
								  if value == self.convert_enum_op_lineHeight_2_u32(LINE_HEIGHT_NUMBER)
								  {
									  styleOffset += 1;
								  }
								  else if value == self.convert_enum_op_lineHeight_2_u32(LINE_HEIGHT_DIMENSION)
								  {
									  styleOffset += 2;
								  }
							  },

							  CSS_PROP_MAX_HEIGHT  |  CSS_PROP_MAX_WIDTH  => 
							  {
								  assert!(MAX_HEIGHT_SET == MAX_WIDTH_SET);

								  if (value == self.convert_enum_op_maxHeight_2_u32(MAX_HEIGHT_SET)){
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_PADDING_TOP |   CSS_PROP_PADDING_RIGHT  |  CSS_PROP_PADDING_BOTTOM  |	 CSS_PROP_PADDING_LEFT  | CSS_PROP_MIN_HEIGHT |	 CSS_PROP_MIN_WIDTH |	 CSS_PROP_PAUSE_AFTER | CSS_PROP_PAUSE_BEFORE  |
								  CSS_PROP_TEXT_INDENT  => 
								  {	
									  assert!(MIN_HEIGHT_SET == MIN_WIDTH_SET);
									  assert!(MIN_HEIGHT_SET == PADDING_SET);
									  assert!(MIN_HEIGHT_SET == PAUSE_AFTER_SET);
									  assert!(MIN_HEIGHT_SET == PAUSE_BEFORE_SET);
									  assert!(MIN_HEIGHT_SET == TEXT_INDENT_SET);

									  if (value == self.convert_enum_op_minHeight_2_u32(MIN_HEIGHT_SET)){
										  styleOffset += 2; // length + units 
									  }
								  },

							  CSS_PROP_OPACITY  => 
							  {
								  if (value == self.convert_enum_op_opacity_2_u32(OPACITY_SET)) { 
									  styleOffset += 1; // value 
								  }
							  },

							  CSS_PROP_ORPHANS  |  CSS_PROP_PITCH_RANGE  | CSS_PROP_RICHNESS  | CSS_PROP_STRESS  |	 CSS_PROP_WIDOWS  =>
							  {
								   assert!(ORPHANS_SET == PITCH_RANGE_SET);
								   assert!(ORPHANS_SET == RICHNESS_SET);
								   assert!(ORPHANS_SET == STRESS_SET);
								   assert!(ORPHANS_SET == WIDOWS_SET);

								  if (value ==  self.convert_enum_op_orphans_2_u32(ORPHANS_SET)){
									  styleOffset  += 1; // value 
								  }
							  },

							  CSS_PROP_OUTLINE_COLOR  => 
							  {
								  if (value ==  self.convert_enum_op_outlineColor_2_u32(OUTLINE_COLOR_SET)){
									  styleOffset += 1; // color 
								  }
							  },

							  CSS_PROP_PITCH  => 
							  {
								  if (value == self.convert_enum_op_pitch_2_u32(PITCH_FREQUENCY)){
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_PLAY_DURING  => 
							  {
								  if (value == self.convert_enum_op_playDuring_2_u32(PLAY_DURING_URI)){
									  styleOffset += 1; // string table entry 
								  }
							  },

							  CSS_PROP_QUOTES  =>
							  { 
								  while (value != self.convert_enum_op_quotes_2_u32(QUOTES_NONE)) {
									  styleOffset += 2; // two string table entries 

									  value = styleBytecode[styleOffset];
									  styleOffset += 1;
								  }
							  },

							  CSS_PROP_SPEECH_RATE  => 
							  {
								  if (value == self.convert_enum_op_speechRate_2_u32(SPEECH_RATE_SET)) {
									  styleOffset += 1; // rate 
								  }
							  },

							  CSS_PROP_VERTICAL_ALIGN  => 
							  {
								  if (value == self.convert_enum_op_verticalAlign_2_u32(VERTICAL_ALIGN_SET)){
									  styleOffset += 2; // length + units 
								  }
							  },

							  CSS_PROP_VOICE_FAMILY  => 
							  {
								  while (value != self.convert_enum_op_voiceFamily_2_u32(VOICE_FAMILY_END)) 
								  {
									  if (value == self.convert_enum_op_voiceFamily_2_u32(VOICE_FAMILY_STRING)) ||
										  (value == self.convert_enum_op_voiceFamily_2_u32(VOICE_FAMILY_IDENT_LIST)) 
										  {
											  value = styleBytecode[styleOffset];
											  styleOffset += 1;
										  }
								  }
							  },

							  CSS_PROP_VOLUME  => 
							  {
								  if value == self.convert_enum_op_volume_2_u32(VOLUME_NUMBER)
								  {
									  styleOffset += 1;  // value 
								  }
								  else if value == self.convert_enum_op_volume_2_u32(VOLUME_DIMENSION)
								  {
									  styleOffset += 2; // value + units 
								  }		
							  },
							  CSS_PROP_Z_INDEX  => 
							  {
								  if (value ==  self.convert_enum_op_Zindex_2_u32(Z_INDEX_SET))
								  {
									  styleOffset += 1; // z index 
								  }
							  },
							  _		=>	()  // Do nothing
				}
			}	

		}	

	}

} // impl important ends




// ===========================================================================================================
// Important.h and important.c implementation/data-structs ends here 
// ===========================================================================================================


pub fn font_face()->@font_face
{
	@font_face
	{
lpu_instance : lpu(),
	     lwc_instance : lwc()				
	}
}

impl font_face
{

	pub fn tokenIsChar(&self, token : @css_token, charData : u8) -> bool
	{
		let mut result : bool = false;

		match (token.token_type)	
		{
			CSS_TOKEN_CHAR	=>		
			{
				let mut tempCharData : u8 = lwc::lwc_string_data(token.idata)[0]; 

				if tempCharData >= 'A' as u8 && tempCharData <= 'Z' as u8
				{
					tempCharData += ('a' - 'A') as u8;	
				}
				if tempCharData == charData
				{
					result = true;	
				}
				else
				{
					result = false;	
				}
			},
				_	=>	()
		}
		return result; 
	}

	// consumes all whiteSpace tokens
	pub fn consumeWhiteSpace(&self, tokenVector : ~[~css_token])
	{	
		for tokenVector.each |&iter|
		{
			match (iter.token_type) 
			{
				CSS_TOKEN_S	=>	break,
						_		=>	{}
			}	
		}	
	}	

	pub fn css_parse_font_descriptor(&self, cssLang : @css_language, descriptor : ~css_token, tokenVector : ~[~css_token], mut rule_face : &css_rule_face) -> css_result
	{
		let mut font_face  = rule_face.font_face ;

		match font_face
		{
			Some(ref data)	=> (), // Do nothing
			None		=> {
						// assigning default value
														   	      // Need to implement enum 2 Int function here also
						font_face = css_font_face{font_family:None, srcs: None, n_srcs: 0, bits:~((CSS_FONT_WEIGHT_NORMAL << 2) | CSS_FONT_STYLE_NORMAL)};
					   }	
		}	

		if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[FONT_FAMILY as int]) == true 
		{
			return self.font_face_parse_font_family(cssLang, tokenVector, font_face);		
		}
		else if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[SRC as int]) ==  true 
		{
			return self.font_face_parse_src(cssLang, tokenVector, font_face);		
		}
		else if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[FONT_STYLE as int]) == true 
		{
			return self.font_face_parse_font_style(cssLang, tokenVector, font_face);		
		}
		else if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[FONT_WEIGHT as int]) ==  true
		{
			return self.font_face_parse_font_weight(cssLang, tokenVector, font_face);		
		}
	}

	pub fn font_rule_font_family_reserved (&self, cssLang : @css_language, identifier : ~css_token) -> bool
	{

		return(		lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[SERIF as int]) == true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[SANS_SERIF as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[CURSIVE as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[FANTASY as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[MONOSPACE as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[INHERIT as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[INITIAL as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[DEFAULT as int]) ==  true
		      ) ;
	}

	// 2 b discussed
	pub fn font_face_parse_font_family (&self, cssLang : @css_language, tokenVector : ~[~css_token], font_face : @css_font_face) -> css_result
	{

		//let bResult = font_rule_font_family_reserved(	
		//css_result = css_ident_list_or_string_to_string(cssLang, tokenVector, font_rule_font_family_reserved,  

		return CSS_GENERAL_OK;
	}

	pub fn font_face_src_parse_format (&self, cssLang : @css_language, tokenVector : ~[~css_token], mut font_face_format : css_font_face_format) -> css_result
	{
		font_face_format = CSS_FONT_FACE_FORMAT_UNSPECIFIED;	

		// since I am doing copy tokenVector, would the called function remvoe all the leading white spaces
		self.consumeWhiteSpace(copy tokenVector);

		for tokenVector.each |&token|
		{
			match (token.token_type)
			{
				// does CSS_TOKEN_STRING accept whiteSpaces also, if yes then below mentioned line might be needed
				// self.consumeWhiteSpace(tokenVector);

				CSS_TOKEN_STRING  =>	
				{
					if  lwc::lwc_string_isequal(token.idata, cssLang.strings[WOFF as int]) == true 
					{
						// Sushanta: How to implement bitwise operator in RUST ?
						// Well, to implement bitwise opearation, function enum to int needs to be called and then apply operation

						// font_face_format |= CSS_FONT_FACE_FORMAT_WOFF ; 	
					}
					else if   lwc::lwc_string_isequal(token.idata, cssLang.strings[TRUETYPE as int]) ==  true  ||
						lwc::lwc_string_isequal(token.idata, cssLang.strings[OPENTYPE as int]) == true 
						{
							// font_face_format |= CSS_FONT_FACE_FORMAT_OPENTYPE ; 	
						}
					else if   lwc::lwc_string_isequal(token.idata, cssLang.strings[EMBEDDED_OPENTYPE as int]) == true 
					{
							//font_face_format |= CSS_FONT_FACE_FORMAT_EMBEDDEDOPENTYPE ; 	
					}
					else if   lwc::lwc_string_isequal(token.idata, cssLang.strings[SVG as int]) == true 
					{
					//	font_face_format |= CSS_FONT_FACE_FORMAT_SVG; 	
					}
					else
					{	
						//font_face_format |= CSS_FONT_FACE_FORMAT_UNKNOWN; 	
					}	
				},
				_		=>	return CSS_INVALID	
			}
		}

		// Temporary comment
		/*
		if (self.tokenIsChar(token, ')') == false)
		{
			return CSS_INVALID;	
		}
		*/

		return CSS_GENERAL_OK;
	}



	pub fn font_face_src_parse_spec_or_name (&self, cssLang : @css_language, tokenVector : ~[~css_token], location : ~lwc_string, mut font_face_location_type : css_font_face_location_type, font_face_format : css_font_face_format) -> css_result
	{
		let mut errorVal : css_result = CSS_GENERAL_OK;
		self.consumeWhiteSpace(copy tokenVector);		

		for tokenVector.each |&token|
		{
			match(token.token_type) 
			{
				CSS_TOKEN_URI	=> {
					
					errorVal = cssLang.sheet.resolve(cssLang.sheet.url, token.idata, location);
					if errorVal != CSS_GENERAL_OK
					{
						return errorVal;
					}
	
					font_face_location_type = CSS_FONT_FACE_LOCATION_TYPE_URI; 
					self.consumeWhiteSpace(tokenVector);		

					match(token.token_type)
					{
						CSS_TOKEN_FUNCTION 	=> {
							if  (lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[FORMAT as int]) == true) 
							{
								errorVal = self.font_face_src_parse_format(cssLang, tokenVector, font_face_format);

								match(errorVal)
								{
									CSS_GENERAL_OK  =>	(),
										_	=>	return errorVal
								}
							}	
						},
						_			=>	()

					}				
				},
				
				CSS_TOKEN_FUNCTION  => {

					if (lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[LOCAL as int]) == true)
					{
						self.consumeWhiteSpace(tokenVector);	
						
						// below mentioned fun is defined in Parse/Properties/Utils.c
						// errorVal = css_ident_list_or_string_to_string(cssLang, tokenVector, location);

						if errorVal != CSS_GENERAL_OK 
						{
							return errorVal;	
						}

						self.consumeWhiteSpace(tokenVector);

						if self.tokenIsChar(token, ')') == false
						{
								return CSS_INVALID;	
						}

						font_face_location_type = CSS_FONT_FACE_LOCATION_TYPE_LOCAL; 
					}
				},
				_		=>{
								return CSS_INVALID;	
						  }
			} // match block ends
		}

		return CSS_GENERAL_OK;
	}

	// In this function, LABELs are used
	pub fn font_face_parse_src (&self, cssLang : @css_language, tokenVector : ~[~css_token], font_face : @css_font_face) -> css_result
	{
		//let orig_cntx : int = *cntx;
		let errorVal : css_result = CSS_GENERAL_OK;	
		let n_srcs : u32 = 0;	
		// Can we initialise srcs as NONE (or anything else as NULL in C)
		let srcs : ~css_font_face_src ; 
		let new_srcs : ~css_font_face_src;

		self.consumeWhiteSpace(copy tokenVector);

		for tokenVector.each |&token|
		{
			let font_face_location : ~lwc_string ;
			let font_face_location_type : css_font_face_location_type = CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED;
			let font_face_format : css_font_face_format = CSS_FONT_FACE_FORMAT_UNSPECIFIED; 	

			errorVal =  self.font_face_src_parse_spec_or_name(cssLang, tokenVector, font_face_location, font_face_location_type, font_face_format);  
			
			match(errorVal)
			{
				CSS_GENERAL_OK 	=>
					{
						// But where new_srcs is initialized ?
						srcs = new_srcs;
						srcs[n_srcs].location = font_face_location; 		
						srcs[n_srcs].bits[0] = font_face_format << 2 | font_face_location_type;
						n_srcs += 1;		
					},
					_	=>	()
			}

			// Do we need this	
			self.consumeWhiteSpace(tokenVector);
		}
		// Q: below mentioned function is defined in src/selects/Font_face.c
		// and hence not yet implemented
		errorVal =  self.css_font_face_set_srcs(font_face, srcs, n_srcs); 
		return errorVal;
	}

	pub fn font_face_parse_font_style (&self, cssLang : @css_language, tokenVector : ~[~css_token], font_face : @css_font_face) -> css_result
	{
		let mut errorVal : css_result = CSS_GENERAL_OK; 
		let mut style : css_font_style_e = CSS_FONT_STYLE_INHERIT; 

		for tokenVector.each |&token|
		{
			match(token.token_type)
			{
				CSS_TOKEN_IDENT		=>	return CSS_INVALID,
				_			=>	{
								if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[NORMAL as int]) == true 
								{
									style = CSS_FONT_STYLE_NORMAL;	
								}
								else if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[ITALIC as int]) == true
								{
									style = CSS_FONT_STYLE_ITALIC;	
								}
								else if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[OBLIQUE as int]) == true
								{
									style = CSS_FONT_STYLE_OBLIQUE;	
								}
								else
								{
									errorVal = CSS_INVALID;
								}
							}

			}	

		}

		   if errorVal == CSS_GENERAL_OK
		   {
		   	font_face.bits[0] = (font_face.bits[0] & 0xfc) | style;
		   }

		return 	errorVal;
	}

	pub fn font_face_parse_font_weight (&self, cssLang : @css_language, tokenVector : ~[~css_token],  font_face : @css_font_face) -> css_result
	{
		let mut errorVal : css_result = CSS_GENERAL_OK; 
		let mut weight : css_font_weight_e = CSS_FONT_WEIGHT_INHERIT ; 

		for tokenVector.each |&token|
		{
			match token.token_type
			{
				CSS_TOKEN_NUMBER	=>{
					let consumed : uint = 0;

					//  is it right to write @consumed, or should I write &consumed
					// I think "&consumed" is for READONLY values

					// below mentioned function namely "css_number_from_lwc_string" is writtten in utils.c
					// sushanta: commenting temporarily 
					/*
					let number : u32 = self.css_number_from_lwc_string(token.idata, true, @consumed);
				
					// invalid if there are trailing characters  
					if consumed != lwc::lwc_string_length(token.idata)
					{
						return CSS_INVALID;	
					}
					*/
					//sushanta: temporary initialization
					let number : u32 = 0x00;

					match(number >>10)
					{
						100	=>	weight = CSS_FONT_WEIGHT_100,		
							200	=>	weight = CSS_FONT_WEIGHT_200,		
							300	=>	weight = CSS_FONT_WEIGHT_300,		
							400	=>	weight = CSS_FONT_WEIGHT_400,		
							500	=>	weight = CSS_FONT_WEIGHT_500,		
							600	=>	weight = CSS_FONT_WEIGHT_600,		
							700	=>	weight = CSS_FONT_WEIGHT_700,		
							800	=>	weight = CSS_FONT_WEIGHT_800,		
							900	=>	weight = CSS_FONT_WEIGHT_900,		
							_	=>	return CSS_INVALID


					}		
				},
							CSS_TOKEN_IDENT		=>{
								if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[NORMAL as int]) == true
								{
									weight = CSS_FONT_WEIGHT_NORMAL;	
								}
								else if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[BOLD as int]) == true
								{
									weight = CSS_FONT_WEIGHT_BOLD;	
								}
								else
								{
									errorVal = CSS_INVALID;
								}	

								match (errorVal)
								{
									CSS_GENERAL_OK  =>	{
										// font_face.bits[0] = (font_face.bits[0] & 0xc3) | (weight << 2);	
									}
									_		=>	{	
										();
									}
								}
							},
				_		=>	()	

			} // match token.token_type ends 
		} // for loop ends
		return errorVal;
	}	// end of font_face_pasre_font_wweight	
					
}

// ===========================================================================================================
// Font_Face.h and Font_Face.c implementation/data-structs ends here 
// ===========================================================================================================


// ===========================================================================================================
// CSS-SELECT implementation/data-structs start here 
// ===========================================================================================================


/*
pub fn lcss_select(sheet:@css_stylesheet)->@css_select {
	
				let	css_select_instance = @css_select ;

	return css_select_instance;
}

impl css_select
{

}
*/
