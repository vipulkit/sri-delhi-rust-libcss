use parserutils::input::inputstream::*;

pub enum css_token_type {
	CSS_TOKEN_IDENT(~str),
	CSS_TOKEN_ATKEYWORD(~str),
	CSS_TOKEN_HASH(~str),
	CSS_TOKEN_FUNCTION(~str), 
	CSS_TOKEN_STRING(~str), 
	CSS_TOKEN_INVALID_STRING, 
	CSS_TOKEN_URI(~str), 
	CSS_TOKEN_UNICODE_RANGE(char , char), 
	CSS_TOKEN_CHAR(char),
	CSS_TOKEN_NUMBER(NumericValue , ~str), 
	CSS_TOKEN_PERCENTAGE(NumericValue , ~str), 
	CSS_TOKEN_DIMENSION(NumericValue , ~str, ~str),
	CSS_TOKEN_CDO, 
	CSS_TOKEN_CDC, 
	CSS_TOKEN_S,
	// F(char),
	// CSS_TOKEN_COMMENT, 
	// CSS_TOKEN_INCLUDES, CSS_TOKEN_DASHMATCH, CSS_TOKEN_PREFIXMATCH, 
	// CSS_TOKEN_SUFFIXMATCH, CSS_TOKEN_SUBSTRINGMATCH, 
	CSS_TOKEN_EOF 
}

pub enum NumericValue {
	Integer(int),
	Float(float),
}

static MAX_UNICODE: char = '\U0010FFFF';

static ASCII_LOWER_OFFSET: char = 'a' - 'A';

pub enum lexer_error {
	LEXER_OK = 0,
	LEXER_NEEDDATA = 1,
	LEXER_INVALID = 2
}

pub fn ascii_lower(string: &str) -> ~str {
	do str::map(string) |c| {
		match c {
			'A'..'Z' => c + ASCII_LOWER_OFFSET,
			_ => c,
		}
	}
}

pub fn preprocess(input: &str) -> ~str {
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
		match $value {
			$pattern => true,
			_ => false 
		}
	);
)

pub struct css_lexer {
	transform_function_whitespace: bool,
	internal_vector: ~[u8],
	length: uint, // Counted in bytes, not characters
	position: uint, // Counted in bytes, not characters
	inputstream_instance: ~inputstream,
	inputstream_eof: bool,
	eof_token_sent: bool,
	flagConsumeComments:bool,
}


pub impl css_lexer {

	pub fn css__lexer_create( inputstream: ~inputstream ) -> ~css_lexer {
		~css_lexer{ 
			transform_function_whitespace: false,
			internal_vector: ~[],
			length: 0, 
			position: 0, 
			inputstream_instance: inputstream,
			inputstream_eof: false,
			eof_token_sent: false,
			flagConsumeComments:false,
		}
	}

   
	pub fn lexer_append_data(&mut self, input_data: ~[u8]) {
		self.inputstream_instance.parserutils_inputstream_append(input_data);
		self.read_from_inputstream();
	}

	fn read_from_inputstream(&mut self) {
		let mut cursor_position = 0;
		let mut data:~[u8]=~[];
		let opt_value_main_tuple = self.inputstream_instance.parserutils_inputstream_peek(cursor_position);
		match opt_value_main_tuple {
			(opt_value ,_)=> {
				match opt_value {
					None=>{},
					Some(opt_value_secondary_tuple)=>{
						match opt_value_secondary_tuple  {
							(data_vector,_)=>{
								data = data_vector ;
							}
						}
					}
				}
			}
		}
		
		let mut string_from_data = str::from_bytes(data);
		string_from_data = preprocess(string_from_data);
		self.internal_vector += str::to_bytes(string_from_data);
		
		
		self.length = self.internal_vector.len();
		// self.streamLen= data.len();
		  
		self.inputstream_instance.parserutils_inputstream_advance(data.len());
		 
	}
	

	pub fn get_token(&mut self) -> (Option<css_token_type>, lexer_error) {
		// if self.streamLen == 0 {
		//   return (None, LEXER_NEEDDATA)  ;
		// }
		if self.is_eof() {
			if self.eof_token_sent{
				return(None , LEXER_INVALID)
			}
			self.eof_token_sent = true;
			return (Some(CSS_TOKEN_EOF), LEXER_OK) 
		}

		else { 
			if(self.flagConsumeComments) {
				let (token , error)=self.consume_comments();
				match(error) {
					LEXER_OK=>{},
					_=>return(token,error)
				}
			}
			self.consume_token()
		}
	}

	fn handle_transform_function_whitespace(&mut self, string: ~str) -> (Option<css_token_type> , lexer_error) {
		
		// io::println("handle_transform_function_whitespace : inside function");
		let mut error_condition: lexer_error = LEXER_OK;
		let mut position = self.position; //

		while !self.is_eof() {
			let c:char=
			match self.current_char() {
				(Some(ch),_)=>ch,
				_=> return (None,LEXER_NEEDDATA)
			};
			match c {
				'\t' | '\n' | '\x0C' | ' ' => 
					if  position+1 < self.length {
						position += 1
					} 
					else {
						position += 1;
						error_condition=LEXER_NEEDDATA;
					},
				'(' => { 
					if  position+1 < self.length {
						 position += 1
					} 
					else {
						position += 1;
						error_condition= LEXER_NEEDDATA;
					} 
					self.position = position;
					return (Some(CSS_TOKEN_FUNCTION(string)), error_condition);
				},
				_ => break,
			}
		}

		// Go back for one whitespace character.
		position -= 1;
		self.position = position;
		(Some(CSS_TOKEN_IDENT(string)), error_condition)
	}

	fn is_eof(&self) -> bool {
		// io::println("is_eof : inside function");
		// io::println(fmt!("is_eof: position = %? , length = %? , inputstream_eof = %?" , self.position , self.length , self.inputstream_eof));
		self.inputstream_eof && self.position >= self.length
	}

	pub fn data_done(&mut self) {
		// io::println("data_done : inside function");
		self.inputstream_eof = true;
	}

	pub fn consume_token(&mut self) -> (Option<css_token_type>, lexer_error) {
		// Comments are special because they do not even emit a token, unless they reach EOF which is an error.
		// io::println("consume_token : inside function");
		let head_position = self.position;
		let mut error_condition:lexer_error= LEXER_OK;
		
		let c:char= match self.current_char() {
				(Some(ch),_)=>ch,
				_=> return (None,LEXER_NEEDDATA)
		};
		
		match c {
			'/' => {
				if (((self.internal_vector.len() - self.position) >2) && self.match_here(~"/*")) {
					self.position += 2;
					self.consume_comments()
				}
				else {
					(None , LEXER_NEEDDATA)
				}
			},

			'-' => {
				if (self.internal_vector.len() - self.position) > 3 {
					if self.match_here(~"-->") {
						self.position += 3;
						(Some(CSS_TOKEN_CDC), LEXER_OK)
					}
					else if 
						match self.next_is_namestart_or_escape() {
							   (None ,x) => return (None , x),
							   (Some(x),_) =>  x 
						} {
						self.consume_ident()
					}
					else {
						self.consume_numeric()
					}
				}
				else {
					(None , LEXER_NEEDDATA)
				}
			},

			'<' => {
				if (self.internal_vector.len() - self.position) > 4 {
					if self.match_here(~"<!--") {
						self.position += 4;
						(Some(CSS_TOKEN_CDO), LEXER_OK)
					} 
					else {
						if  self.position+1 < self.length {
							self.position += 1
						} 
						else {
							self.position += 1;
							error_condition= LEXER_NEEDDATA;
						}  
						(Some(CSS_TOKEN_CHAR('<')), error_condition)
					}
				}
				else {
					(None , LEXER_NEEDDATA)
				}
			},

			'0'..'9' | '.' | '+' => self.consume_numeric(),

			'u' | 'U' => self.consume_unicode_range(),

			'a'..'z' | 'A'..'Z' | '_' | '\\' => self.consume_ident(),

			_ if c >= '\x80' => self.consume_ident(), // Non-ASCII

			_ => {
				match self.consume_char() {
					(Some(ch),x) => {
						error_condition= match(error_condition) {
							LEXER_NEEDDATA=>LEXER_NEEDDATA,
							_=>x
						};
						match(ch) {
							'\t' | '\n' | '\x0C' | ' ' => {
								while !self.is_eof() && self.position+1 < self.length {
									let c:char=
										match self.current_char() {
											(Some(ch),_)=>ch,
											_=> {
												self.position = head_position;
												return (None,LEXER_NEEDDATA);
											}
									};
									match  c {
										'\t' | '\n' | '\x0C' | ' ' => if  self.position+1 < self.length {
											self.position += 1
										}
										else {
											self.position += 1;
											error_condition=LEXER_NEEDDATA;
										},
										_ => break,
									}
								}
								(Some(CSS_TOKEN_S), error_condition)
							},
							'"' => match (self.consume_quoted_string(false)) {
										(_, LEXER_NEEDDATA) => {
											self.position -= 1; // undo consume-char
											return (None, LEXER_NEEDDATA);
										}
										(x, y) => {
											return(x, y);
										}
									},
							'#' => match (self.consume_hash()) {
										(_, LEXER_NEEDDATA) => {
											self.position -= 1; // undo consume-char
											return (None, LEXER_NEEDDATA);
										}
										(x, y) => {
											return(x, y);
										}
									},
							'\'' => match (self.consume_quoted_string(true)) {
										(_, LEXER_NEEDDATA) => {
											self.position -= 1; // undo consume-char
											return (None, LEXER_NEEDDATA);
										}
										(x, y) => {
											return(x, y);
										}
									},
							'(' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							')' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							':' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							';' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							'@' => match (self.consume_at_keyword()) {
										(_, LEXER_NEEDDATA) => {
											self.position -= 1; // undo consume-char
											return (None, LEXER_NEEDDATA);
										}
										(x, y) => {
											return(x, y);
										}
									},
							'[' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							']' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							'{' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							'}' => (Some(CSS_TOKEN_CHAR(c)), error_condition),
							_ => (Some(CSS_TOKEN_CHAR(c)), error_condition)
						}
					},
					(None,_)=>{
						self.position = head_position;
						return (None,LEXER_NEEDDATA);
					}
				}
			}
		}
	}

	fn consume_quoted_string(&mut self, single_quote: bool) -> (Option<css_token_type> , lexer_error) {

		// io::println("consume_quoted_string : inside function");
		let head_position = self.position;
		let mut string: ~str = ~"";
		while !self.is_eof() {
			match self.consume_char() {
				(Some(ch),x)=>match(ch) {
					'"' if !single_quote => return (Some(CSS_TOKEN_STRING(string)), x),
					'\'' if single_quote => return (Some(CSS_TOKEN_STRING(string)), x),
					'\n' | '\x0C' => {
						return (Some(CSS_TOKEN_INVALID_STRING), LEXER_INVALID);
					},
					'\\' => {
						match self.next_n_chars(1) {
							// Quoted newline
							['\n'] | ['\x0C'] => 
									if  self.position+1 < self.length {
										self.position += 1
									} 
									else {
										self.position = head_position;	
										return (None , LEXER_NEEDDATA)
									},
							[] =>
								return (Some(CSS_TOKEN_INVALID_STRING), LEXER_INVALID),
							_ => match self.consume_escape() {
								(None,_)=> {
									self.position = head_position;
									return (None,LEXER_NEEDDATA);
								},
								(Some(ch),_)=> push_char!(string,ch) 
							}
						}
					}
					c => push_char!(string, c),
				},
				(None,_)=>{
					self.position = head_position;
					return (None,LEXER_NEEDDATA);
				}
			}
		}
		(Some(CSS_TOKEN_STRING(string)), LEXER_INVALID)
	}

	fn consume_hash(&mut self) -> (Option<css_token_type> , lexer_error) {
		// io::println("consume_hash : inside function");
		let (StringOpt,error_condition) = self.consume_ident_string_rest();
		let mut string: ~str = ~"";
		match StringOpt {
			None => {},
			Some(x) => string += x
		}
		
		match(error_condition) {
			LEXER_NEEDDATA=> {
				// self.position = head_position;
				return (None,LEXER_NEEDDATA);
			},
			_=>()
		}
		(if string == ~"" {
			Some(CSS_TOKEN_CHAR('#')) 
		} 
		else { 
			Some(CSS_TOKEN_HASH(string)) 
		}, LEXER_OK)
	}

	fn consume_char(&mut self) -> (Option<char>,lexer_error) {
		// io::println("consume_char : inside function");
		if self.position + 1 > self.length {
			return (None,LEXER_NEEDDATA)
		}
		
		let range = str::char_range_at(str::from_bytes(self.internal_vector), self.position);
		
		self.position = range.next;
		if self.position  >= self.length {
			(Some(range.ch),LEXER_NEEDDATA)
		} 
		else { 
			(Some(range.ch),LEXER_OK)
		}
	}

	fn match_here(&mut self, needle: ~str) -> bool {
		let mut i = self.position;
		if i + needle.len() > self.length { 
			return false; 
		}
		
		let haystack: &str = str::from_bytes(self.internal_vector);
		for needle.each |c| { 
			if haystack[i] != c { 
				return false; 
			} 
			i += 1u; 
		}
		return true;
	}

	fn consume_comments(&mut self)-> (Option<css_token_type> , lexer_error) {
	   
		// io::println("consume_comments : inside function");
		let head_position = self.position;
		let vec_to_string: ~str = str::from_bytes(self.internal_vector);
			match str::find_str_from(vec_to_string, "*/", self.position) {
				Some(end_position) => {
					self.position = end_position + 2;
					
					if self.position >= self.length {
						self.flagConsumeComments=true;
						self.position = head_position;
						return(None , LEXER_NEEDDATA);
					}
					else {
						self.flagConsumeComments=false;
					}
				},
				None => {
					self.flagConsumeComments=true;
					self.position = self.length;
					if self.is_eof() {
						return (None , LEXER_INVALID);  
					}
					self.position = head_position;
					return (None , LEXER_NEEDDATA);
				}
			}
		
		return(None , LEXER_OK);
	}

	fn consume_at_keyword(&mut self) -> (Option<css_token_type> , lexer_error) {
		// io::println("consume_at_keyword : inside function");
		(match self.consume_ident_string() {
			(Some(string),x) => (Some(CSS_TOKEN_ATKEYWORD(string)),x),
			(None,x) => match(x) {
				LEXER_NEEDDATA =>(None,LEXER_NEEDDATA),
				_=>(Some(CSS_TOKEN_CHAR('@')),LEXER_OK)
			}
		})
	}

	fn current_char(&mut self) -> (Option<char>,lexer_error) {
		
		// io::println("current_char : inside function");
		if self.position >= self.length {
			return (None,LEXER_NEEDDATA);
		}
		(Some(self.internal_vector[self.position] as char),LEXER_OK)
	}

	fn next_is_namestart_or_escape(&mut self) -> (Option<bool>, lexer_error) {
		
		// io::println("next_is_namestart_or_escape : inside function");
		if  self.position+1 >= self.length {
			return (None , LEXER_NEEDDATA)
		} 
		else {
			self.position += 1;
			let result = !self.is_eof() && match(self.is_namestart_or_escape()) {
				Some(x)=>x,
				None=>return(None,LEXER_NEEDDATA)
			};
			self.position -= 1;
			(Some(result) , LEXER_OK)
		}
	}

	fn next_n_chars(&mut self, n: uint) -> ~[char] {
		// io::println("next_n_chars : inside function");
		let mut chars: ~[char] = ~[];
		let mut position = self.position;
		for n.times {
			if position >= self.length { 
				break 
			}
			let range = str::char_range_at(str::from_bytes(self.internal_vector), position);
		   
			position = range.next;
			chars.push(range.ch);
		}
		chars
	}

	fn is_invalid_escape(&mut self) -> bool {
		match self.next_n_chars(2) {
			['\\', '\n'] | ['\\', '\x0C'] | ['\\'] => true,
			_ => false,
		}
	}

	fn is_namestart_or_escape(&mut self) -> Option<bool> {
		// io::println("is_namestart_or_escape : inside function");
		let c:char= match self.current_char() {
				(Some(ch),_)=>ch,
				_=> return None
		};
		match c {
			'a'..'z' | 'A'..'Z' | '_' => Some(true),
			'\\' => Some(!self.is_invalid_escape()),
			c =>Some( c >= '\x80'), // Non-ASCII
		}
	}


	fn consume_ident(&mut self) -> (Option<css_token_type> , lexer_error) {
		// io::println("consume_ident : inside function");
		let c:char;
		match self.consume_ident_string() {
			(Some(string),_) => {
				if self.is_eof() { 
					// io::println(fmt!("consume_ident : in self is eof string is %? " , string));
					return (Some(CSS_TOKEN_IDENT(string)), LEXER_OK);
				}
				let c:char= match self.current_char() {
						(Some(ch),_)=>ch,
						_=> return (None,LEXER_NEEDDATA)
				};            
				// io::println(fmt!("consume_ident: value of c is %?" , c));
				match c {
					'\t' | '\n' | '\x0C' | ' ' if self.transform_function_whitespace => {
						if  self.position+1 <= self.length {
							self.position += 1
						} 
						else {
							return (None , LEXER_NEEDDATA)
						}
						self.handle_transform_function_whitespace(string)
					}
					'(' => {
						// io::println("consume_ident : in match case (");
						// io::println(fmt!("consume_ident: value of self.position is %? and self.length is %?" , self.position , self.length));	
						if  self.position+1 <= self.length && ascii_lower(string) == ~"url" {
							self.position += 1;
							let (token , lexer_error) = self.consume_url();
							// io::println("consume_ident : in match case (");

							match lexer_error {
								LEXER_NEEDDATA => { 
									self.position = self.position - 4;
									return (None , LEXER_NEEDDATA);
								},
								LEXER_OK => return (token , LEXER_OK),
								LEXER_INVALID => return (token , LEXER_INVALID)
							}
							// return (Some(CSS_TOKEN_FUNCTION(string)), LEXER_OK);
						} 
						else if self.position+1 <= self.length {
							return (Some(CSS_TOKEN_FUNCTION(string)), LEXER_OK);
							// return (None , LEXER_NEEDDATA)
						}
						else {
							return (None , LEXER_NEEDDATA)
						}
					},
					_ => {
						return (Some(CSS_TOKEN_IDENT(string)), LEXER_OK)
					}
				}
			},
			(None,x) => {
				match x {
					LEXER_NEEDDATA=> return (None,LEXER_NEEDDATA) ,
					_=> { 
						c= match self.current_char() {
							(Some(ch),_)=>ch,
							_=> return (None,LEXER_NEEDDATA)
						};
						match c {
							'-' => {
								if  self.position+1 <= self.length {
									self.position += 1
								} 
								else {
									return (None , LEXER_NEEDDATA)
								}
								(Some(CSS_TOKEN_CHAR('-')), LEXER_OK)
							},
							'\\' => {
								if  self.position+1 <= self.length {
									self.position += 1
								} 
								else {
									return (None , LEXER_NEEDDATA)
								}
								(Some(CSS_TOKEN_CHAR('\\')), LEXER_INVALID)
							},
							_ => {
								(None , LEXER_INVALID) 
							}
						}// Should not have called consume_ident() here.
					}
				}
			}
		}
	}

	fn consume_ident_string(&mut self) -> (Option<~str>,lexer_error) {
		// io::println("consume_ident_string : inside function");
		let c:char= match self.current_char() {
			(Some(ch),_)=>ch,
			_=> return (None,LEXER_NEEDDATA)
		};
		match c {
			'-' => {
				if  match self.next_is_namestart_or_escape() {
						(None ,_) => return (None , LEXER_NEEDDATA),
						(Some(x),_) =>  x 
					} {
					(None,LEXER_OK) 
				}
				else { 
					self.consume_ident_string_rest()
				}
			},
			'\\' if self.is_invalid_escape() => {
				return (None,LEXER_OK)
			},
			_ if match(self.is_namestart_or_escape() ) {
				Some(x)=>!x,
				None=>return (None,LEXER_NEEDDATA)
			}=> {
				return (None,LEXER_OK)
			},
			_ => {
				self.consume_ident_string_rest()
			}
		}
	}


	fn consume_ident_string_rest(&mut self) -> (Option<~str>,lexer_error) {
		// io::println("consume_ident_string_rest : inside function");
		let mut string = ~"";
		let mut head_position = self.position;
		let mut error_condition:lexer_error= LEXER_OK;
		// let mut i = 0;   // check why is used?
		while !self.is_eof() {
			// i +=1;
			let c:char= match self.current_char() {
				(Some(ch),_)=>ch,
				_=>  {
					self.position = head_position;
					return (None,LEXER_NEEDDATA);
				}
			};
			// io::println(fmt!("consume_ident_string_rest: value of c is %?" , c));
			let next_char = match c {
				'a'..'z' | 'A'..'Z' | '0'..'9' | '_' | '-' => {
					if  self.position+1 <= self.length {
						self.position += 1;
					} 
					else {
						self.position = head_position;
						return (None , LEXER_NEEDDATA);
					} 
					c 
				},
				_ if c >= '\x80' => 
					match(self.consume_char()) {
						(Some(ch),x)=>{
							error_condition=x;
							ch
						},
						(None,_)=>{
							self.position = head_position;
							return (None,LEXER_NEEDDATA)
						}
					 }, // Non-ASCII
				'\\' => {
					if self.is_invalid_escape() { 
						break 
					}
					if  self.position+1 <= self.length {
						self.position+=1;
					}
					else {
						self.position = head_position;    
						return (None , LEXER_NEEDDATA);
					}
					match(self.consume_escape()) {
						(None,_) =>  {
							self.position = head_position;
							return (None,LEXER_NEEDDATA);
						},
						(Some(ch),_)=>ch
					}
				},
				_ => break
			};
			// io::println(fmt!("consume_ident_string_rest: value of next_char is %?" , next_char));
			// io::println(fmt!("consume_ident_string_rest: value of position is %?" , self.position));
			push_char!(string, next_char)
		}
		(Some(string),error_condition)
	}

	fn char_from_hex(&self ,hex: &[char]) -> char {
		// io::println("char_from_hex : inside function");
		uint::from_str_radix(str::from_chars(hex), 16).unwrap() as char
	}

	fn consume_escape(&mut self) -> (Option<char>,lexer_error) {
		// io::println("consume_escape : inside function");
		let head_position = self.position;
		let mut error_condition:lexer_error;
		let c = match(self.consume_char()) {
			(Some(ch),x)=>{
				error_condition=x;
				ch
			},
			(None,_)=> {
				self.position = head_position;
				return (None,LEXER_NEEDDATA);
			}
		};
		match c {
			'0'..'9' | 'A'..'F' | 'a'..'f' => {
				let mut hex = ~[c];
				while hex.len() < 6 && !self.is_eof() {
					let c:char= match self.current_char() {
						(Some(ch),_)=>ch,
						_=> return (None,LEXER_NEEDDATA)
					};
					match c {
						'0'..'9' | 'A'..'F' | 'a'..'f' => {
							hex.push(c);
							if  self.position >= self.length {
								self.position = head_position;
								return (None , LEXER_NEEDDATA);
							}
							else {
								self.position += 1
							} 
						},
						_ => break
					}
				}
				if !self.is_eof() {
					let c:char= match self.current_char() {
						(Some(ch),_)=>ch,
						_=> {
							self.position = head_position;
							return (None,LEXER_NEEDDATA);
						}
					};
					match c {
						'\t' | '\n' | '\x0C' | ' ' =>  {
							if  self.position >= self.length {
								self.position = head_position;
								return (None , LEXER_NEEDDATA)
							} 
							else {
								self.position += 1
							}
						},
						_ => {}
					}
				}
				let c = self.char_from_hex(hex);
				if '\x00' < c && c <= MAX_UNICODE {
					(Some(c) , error_condition) 
				}
				else {
					(Some('\uFFFD') , error_condition) 
				} // Replacement character
			},
			c => (Some(c), error_condition)
			//c => c
		}
	}

	fn consume_url(&mut self) -> (Option<css_token_type> , lexer_error) {
		// io::println("consume_url : inside function");
		let head_position = self.position;
		while !self.is_eof() {
			let c:char= match self.current_char() {
				(Some(ch),_)=>ch,
				_=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA);
				}
			};
			match c {
				'\t' | '\n' | '\x0C' | ' ' =>if  self.position+1 <= self.length {
					self.position += 1
				} 
				else {
					self.position = head_position;
					return (None , LEXER_NEEDDATA);
				},
				'"' => {
					let (token , lexer_error) = self.consume_quoted_url(false);
					match lexer_error {
						LEXER_NEEDDATA => {
							self.position = head_position;
							return (token , lexer_error)
						},
						_ => return (token , lexer_error)
					}
				}
				'\'' =>  {
					let (token , lexer_error) = self.consume_quoted_url(false);
					match lexer_error {
						LEXER_NEEDDATA => {
							self.position = head_position;
							return (token , lexer_error)
						},
						_ => return (token , lexer_error)
					}
				},
				')' => {  
					if  self.position+1 <= self.length {
						self.position += 1
					} 
					else {
						self.position = head_position;
						return (None , LEXER_NEEDDATA);
					};
					return (Some(CSS_TOKEN_URI(~"")), LEXER_OK) 
				},
				_ => {
					let (token , lexer_error) = self.consume_unquoted_url();
					match lexer_error {
						LEXER_NEEDDATA => {
							self.position = head_position;
							return (token , lexer_error)
						},
						_ => return (token , lexer_error)
					}
				},
			}
		}
		(Some(CSS_TOKEN_INVALID_STRING) , LEXER_INVALID)
	}

	fn consume_quoted_url(&mut self, single_quote: bool) -> (Option<css_token_type>, lexer_error) {
		// io::println("consume_quoted_url : inside function");
		let head_position = self.position;
		if  self.position+1 <= self.length {
			self.position += 1
		}
		else {
			self.position = head_position;
			return (None , LEXER_NEEDDATA);
		} // The initial quote
		let (token, err) = self.consume_quoted_string(single_quote);
		match err {
			LEXER_INVALID =>
				match token.unwrap() {

					CSS_TOKEN_STRING(string) => {
						return self.consume_url_end(string);
					},
					// consume_quoted_string() never returns a non-String token
					// without error:
					_ => (None , LEXER_INVALID)
			},
			_ => {
				let (token, _) = self.consume_bad_url();
				(token, err)
			}
		}
	}

	fn consume_unquoted_url(&mut self) -> (Option<css_token_type>, lexer_error) {
		// io::println("consume_unquoted_url : inside function");
		let head_position = self.position;
		let mut string = ~"";
		let mut error_condition:lexer_error;
		while !self.is_eof() {
			let next_char = match self.consume_char() {
				(Some(ch),x)=>{
					// io::println(fmt!("consume_unquoted_url: value of ch is %?" , ch));
					error_condition=x;
					match(ch) {
						'\t' | '\n' | '\x0C' | ' '
							=> return self.consume_url_end(string),
						')' => return (Some(CSS_TOKEN_URI(string)), error_condition),
						'\x00'..'\x08' | '\x0E'..'\x1F' | '\x7F'..'\x9F' // non-printable
							| '"' | '\'' | '(' => return self.consume_bad_url(),
						'\\' => match self.next_n_chars(1) {
									['\n'] | ['\x0C'] | [] => return self.consume_bad_url(),
									_ => match self.consume_escape() {
											(Some(x) ,_) => x,
											_ => {
												self.position = head_position;
												return (None , LEXER_NEEDDATA);
											}
										}
							},
						c => c
					}
				},
				(None,_)=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA);
				}
			};
			push_char!(string, next_char)
		}
		(Some(CSS_TOKEN_INVALID_STRING) , LEXER_INVALID)
	}

	fn consume_url_end(&mut self, string: ~str) -> (Option<css_token_type>, lexer_error) {
		// io::println("consume_url_end : inside function");
		let head_position = self.position;
		let mut error_condition:lexer_error;
		while !self.is_eof() {
			match self.consume_char() {
				(Some(ch),x)=>{
					error_condition=x;
					match(ch) {
						'\t' | '\n' | '\x0C' | ' ' => (),
						')' => return (Some(CSS_TOKEN_URI(string)), error_condition),
						_ => {
							let (some_token , error) = self.consume_bad_url();
							match error {
								LEXER_NEEDDATA => {
									self.position = head_position;
									return(some_token , error);
								},
								_=>return(some_token , error)
							}
						}
					}
				},
				(None,_)=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA) ;
				}
			}
		}
		(Some(CSS_TOKEN_INVALID_STRING) , LEXER_INVALID)
	}

	fn consume_bad_url(&mut self) -> (Option<css_token_type>, lexer_error) {
		// io::println("consume_bad_url : inside function");
		// Consume up to the closing )
		let head_position = self.position;
		// let mut error_condition:lexer_error;
		while !self.is_eof() {
			match self.consume_char() {
				(Some(ch),_)=>{
					// error_condition=x;
					match(ch) {
						')' => break,
						'\\' => if  self.position+1 <= self.length {
									self.position += 1
								} 
								else {
									self.position = head_position;
									return (None , LEXER_NEEDDATA);
								}, // Skip an escaped ) or \
						_ => ()
					}
				},
				(None,_)=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA);
				}
			}
		}
		(Some(CSS_TOKEN_INVALID_STRING) , LEXER_INVALID)
	}

	fn consume_unicode_range(&mut self)-> (Option<css_token_type>, lexer_error) {
		let head_position = self.position;
		// io::println("consume_unicode_range : inside function");
		let next_3 = self.next_n_chars(3);
		// We got here with U or u
		// io::println(fmt!("consume_unicode_range : inside function %?" , next_3));
		assert! ((next_3[0] == 'U')||(next_3[0] == 'u')); //'U'
		// io::println("consume_unicode_range : after assert");
		// Check if this is indeed an unicode range. Fallback on ident.
		if next_3.len() == 3 && next_3[1] == '+' {
			match next_3[2] {
				'0'..'9' | 'a'..'f' | 'A'..'F' => if  self.position+2 < self.length {
					self.position += 2
				} 
				else {
					self.position = head_position;
					return (None , LEXER_NEEDDATA)
				},
				_ => {
					let (some_token , error) = self.consume_ident() ;
					match error {
						LEXER_NEEDDATA => {
							self.position = head_position;
							return(some_token , error);
						},
						_=> return(some_token , error)
						
					}
				}
			}
		} 
		else {
			let (some_token , error) = self.consume_ident() ;
			match error {
				LEXER_NEEDDATA => {
					self.position = head_position;
					return(some_token , error);
				},
				_=> return(some_token , error)
				
			}
		}

		let mut hex = ~[];
		while hex.len() < 6 && !self.is_eof() {
			let c:char=
			match self.current_char() {
				(Some(ch),_)=>ch,
				_=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA)
				}
			};
			match c {
				'0'..'9' | 'A'..'F' | 'a'..'f' => {
					hex.push(c); if  self.position+1 <= self.length {
						self.position += 1
					} 
					else {
						self.position = head_position;
						return (None , LEXER_NEEDDATA)
					} 
				},
				_ => break
			}
		}
		
		assert! (hex.len() > 0);
		let max_question_marks = 6u - hex.len();
		let mut question_marks = 0u;
		while question_marks < max_question_marks && !self.is_eof() 
				&& match self.current_char() {
						(Some(ch),_)=>ch=='?',
						_=> {
							self.position = head_position;
							return (None,LEXER_NEEDDATA)
						}
					} {
			question_marks += 1;
			if  self.position+1 <= self.length {
				self.position += 1
			} 
			else {
				self.position = head_position;
				return (None , LEXER_NEEDDATA)
			}
		}
		let start: char, end: char;
		if question_marks > 0 {
			start = self.char_from_hex(hex + vec::from_elem(question_marks, '0'));
			end = self.char_from_hex(hex + vec::from_elem(question_marks, 'F'));
		} 
		else {
			start = self.char_from_hex(hex);
			hex = ~[];
			if !self.is_eof() && match self.current_char() {
				(Some(ch),_)=>ch== '-',
				_=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA)
				}
			} {
				if  self.position+1 <= self.length {
					self.position += 1
				} 
				else {
					self.position = head_position;
					return (None , LEXER_NEEDDATA)
				};
				while hex.len() < 6 && !self.is_eof() {
					let c:char= match self.current_char() {
						(Some(ch),_)=>ch,
						_=> {
							self.position = head_position;
							return (None,LEXER_NEEDDATA)
						}
					};
					match c {
						'0'..'9' | 'A'..'F' | 'a'..'f' => {
							hex.push(c); if  self.position+1 <= self.length {
								self.position += 1
							} 
							else {
								self.position = head_position;
								return (None , LEXER_NEEDDATA)
							} 
						},
						_ => break
					}
				}
			}
			end = if hex.len() > 0 {
				self.char_from_hex(hex) 
			} 
			else { 
				start 
			}
		}
		(if start > MAX_UNICODE || end < start {
			Some(CSS_TOKEN_INVALID_STRING)
		} 
		else {
			let end = if end <= MAX_UNICODE { 
				end 
			} 
			else { 
				MAX_UNICODE 
			};
			Some(CSS_TOKEN_UNICODE_RANGE(start, end))
		}, LEXER_OK)
	}

	fn consume_numeric(&mut self) -> (Option<css_token_type> , lexer_error) {
		let head_position = self.position;
		// io::println(fmt!("consume_numeric: head position is %?" , head_position));
		// io::println("consume_numeric: inside function");
		let mut error_condition:lexer_error;
		let c = match(self.consume_char()) {
			(Some(ch),x)=>{
				error_condition=x;
				ch
			},
			(None,_)=> {
				// io::println(fmt!("consume_numeric: self position is %?" , self.position));
				self.position = head_position;
				return (None,LEXER_NEEDDATA)
			}
		};
		// io::println(fmt!("consume_numeric before mATCH C %?" , c));
		match c {
			'-' | '+' => {
				let (some_token , error) = self.consume_numeric_sign(c);
				match error {
					LEXER_NEEDDATA => {
						self.position = head_position;
						return (some_token , error);
					},
					_ => return(some_token , error)
				}
			},
			'.' => {
				if self.is_eof() { 
					return (Some(CSS_TOKEN_CHAR('.')), error_condition) 
				}
				let c:char= match self.current_char() {
					(Some(ch),_)=>ch,
					_=> {
						self.position = head_position;
						return (None,LEXER_NEEDDATA)
					}
				};
				// io::println(fmt!("consume_numeric inside mATCH C %?" , c));
				match c {
					'0'..'9' => {
						let (some_token , error) = self.consume_numeric_fraction(~".");
						match error {
							LEXER_NEEDDATA => {
								self.position = head_position;
								return (some_token , error);
							},
							_ => return(some_token , error)
						}
					},
					_ => (Some(CSS_TOKEN_CHAR('.')), error_condition),
				}
			},
			'0'..'9' => {
				let (some_token , error) = self.consume_numeric_rest(c);
					match error {
						LEXER_NEEDDATA => {
							self.position = head_position;
							return (some_token , error);
						}
						_=> return(some_token , error)
					}
			},
			_ => {
				self.position = head_position;
				(None , LEXER_INVALID)
			}, // initially fail statement
		}
	}

	fn consume_numeric_sign(&mut self, sign: char) -> (Option<css_token_type> , lexer_error) {
		// io::println("consume_numeric_sign : inside function");
		let head_position = self.position;
		if self.is_eof() { 
			return (Some(CSS_TOKEN_CHAR(sign)), LEXER_OK) 
		}
		let c:char= match self.current_char() {
					(Some(ch),_)=>ch,
					_=> return (None,LEXER_NEEDDATA)
		};
		match c {
			'.' => {
				if  self.position+1 <= self.length {
					self.position += 1
				} 
				else {
					self.position = head_position;
					return (None , LEXER_NEEDDATA)
				}
				if !self.is_eof() && is_match! (match self.current_char() {
													(Some(ch),_)=>ch,
													_=>{
														self.position = head_position;
														return (None,LEXER_NEEDDATA);
													}
												},'0'..'9') {
					let (some_token , error) = self.consume_numeric_fraction(str::from_char(sign) + ~".");
					match error {
						LEXER_NEEDDATA => {
							self.position = head_position;
							return (some_token , error);
						},
						_=> return (some_token , error)
					}
				} 
				else {
					self.position -= 1;
					(Some(CSS_TOKEN_CHAR(sign)), LEXER_OK)
				}
			},

			'0'..'9' => {
				let (some_token , error) = self.consume_numeric_rest(sign);
				match error {
					LEXER_NEEDDATA => {
						self.position = head_position;
						return (some_token , error);
					},
					_ => return(some_token , error)
				}
			},
			_ => (Some(CSS_TOKEN_CHAR(sign)), LEXER_OK)
		}
	}

	fn consume_numeric_rest(&mut self, initial_char: char) -> (Option<css_token_type> , lexer_error) {
		// io::println(fmt!("consume_numeric_rest: here initial char is %?" , initial_char));

		let head_position = self.position;
		// io::println(fmt!("consume_numeric_rest: head position is %?" , head_position));
		let mut string = str::from_char(initial_char);
		while !self.is_eof() {
			// io::println("consume_numeric_rest:inside while");
			let c:char= match self.current_char() {
						(Some(ch),_)=>ch,
						_=> return (None,LEXER_NEEDDATA)
			};
			// io::println(fmt!("consume_numeric_rest before mATCH C %?" , c));
			match c {
				'0'..'9' => { 
					push_char!(string, c); 
					if  self.position+1 <= self.length {
						self.position += 1
					} 
					else {
						self.position = head_position;
						return (None , LEXER_NEEDDATA)
					} 
				},
				'.' => {
					if  self.position+1 <= self.length {
						self.position += 1
					} 
					else {
						self.position = head_position;
						return (None , LEXER_NEEDDATA)
					}
					if !self.is_eof() && is_match!(match self.current_char() {
														(Some(ch),_)=>ch,
														_=>  {
															self.position = head_position;
															return (None,LEXER_NEEDDATA)
														}
													 }, '0'..'9') {
						push_char!(string, '.');
						let (some_token , error) = self.consume_numeric_fraction(string);
						match error {
							LEXER_NEEDDATA => {
								self.position = head_position;
								return (some_token , error);
							},
							_=> return (some_token , error)
						}
					} 
					else {
						self.position -= 1; break
					}
				},
				_ => {
					match self.consume_scientific_number(string) {
						(Ok(token),x)=> return (Some(token), x),
						(Err(_),LEXER_NEEDDATA) =>{
							self.position = head_position;
							return (None,LEXER_NEEDDATA)
						}
						(Err(s),_) => { 
							string = s;
							break 
						},
						
					}
				}
			}
		}

		let temp : ~str ;          
		if string[0] != '+' as u8 { temp = copy string; }
		else { 
			temp = str::substr(string, 1, string.len()).to_owned(); 
		}
		let value = Integer(int::from_str(temp).unwrap()); // Remove any + sign as int::from_str() does not parse them.  // XXX handle overflow
		let (some_token , error) = self.consume_numeric_end(string, value);
		match error {
			LEXER_NEEDDATA => {
				self.position = head_position;
				return (some_token , error);
			},
			_ => return(some_token , error)
		}
	}

	fn consume_numeric_fraction(&mut self, string: ~str) -> (Option<css_token_type> , lexer_error) {
		// io::println(fmt!("consume_numeric_fraction: here string is %?" , string));
		let head_position = self.position;
		let mut string: ~str = string;
		let mut error_condition:lexer_error= LEXER_OK;
		while !self.is_eof() {
			let ch:char=  match self.consume_char() {
				(Some(c),_)=>c,
				_=> {
					self.position = head_position;
					return (None,LEXER_NEEDDATA);
				}
			};
			// io::println(fmt!("consume_numeric_fraction: here char is %?" , ch));
			match ch {

				'0'..'9' => push_char!(string,ch),
				'%' => {
					self.position -= 1;
					break
				},
				_ => match self.consume_scientific_number(string) {
						(Ok(token),x) => return (Some(token), x),
						(Err(_),LEXER_NEEDDATA)=> {
							self.position = head_position;
							return (None, LEXER_NEEDDATA);
						}
						(Err(s),_) => { 
							string = s; 
							break 
						},
				}
			}
		}

		// io::println(fmt!("consume_numeric_fraction: here number is %?" , string));
		let value = Float(float::from_str(string).unwrap()); // XXX handle overflow
		
		let (some_token , error) = self.consume_numeric_end(string, value);
		match error {
			LEXER_NEEDDATA => {
				self.position = head_position;
				return(some_token , error);
			},
			_=> return(some_token , error)
		}
	}


	fn consume_numeric_end(&mut self, string: ~str, value: NumericValue) -> (Option<css_token_type> , lexer_error) {
		// io::println("consume_numeric_end : inside function");
		let head_position = self.position;
		if self.is_eof() { 
			return (Some(CSS_TOKEN_NUMBER(value, string)), LEXER_OK) 
		}
		let c:char= match self.current_char() {
			(Some(ch),_)=>ch,
			_=> return (None,LEXER_NEEDDATA)
		};
		match c {
			'%' => {
				if  self.position+1 <= self.length {
					self.position += 1;
					let c: char = match self.current_char() {
						(Some(ch) , _)=> ch,
						_ => return(None , LEXER_NEEDDATA)
					};
					match c {
						'\t' | '\n' | '\x0C' | ' '|';' => {
							(Some(CSS_TOKEN_PERCENTAGE(value, string)),LEXER_OK)	
						},
						_ => {
							loop {
								let c: char = match self.current_char() {
									(Some(ch) , _)=> ch,
									_ => return(None , LEXER_NEEDDATA)
								};
								match c {
									'\t' | '\n' | '\x0C' | ' '|';' => {
										break
									},
									_ => {
										if self.position <= self.length {
											self.position += 1;
										}
										else {
											self.position = head_position;
											return (None , LEXER_NEEDDATA)
										}
									}
								}
							}
							(Some(CSS_TOKEN_INVALID_STRING) , LEXER_OK)
						}
					}
				} 
				else {
					self.position = head_position;
					return (None , LEXER_NEEDDATA)
				} 
			},
			_ => {
				match self.consume_ident_string() {
					(Some(unit),_) => (Some(CSS_TOKEN_DIMENSION(value, string, unit)),LEXER_OK),
					(None,x) => match x {
						LEXER_NEEDDATA =>  {
							self.position = head_position;
							(None,LEXER_NEEDDATA)
						}, 
						_=>  (Some(CSS_TOKEN_NUMBER(value, string)),LEXER_OK)
					}
				}
			},
		}
	}


	fn consume_scientific_number(&mut self, string: ~str) -> (Result<css_token_type, ~str>,lexer_error) {
		// io::println("consume_scientific_number: inside fn");
		let head_position = self.position;
		let next_3 = self.next_n_chars(3);
		let mut error_condition:lexer_error= LEXER_OK;
		let mut string: ~str = string;
		if (next_3.len() >= 2
			&& (next_3[0] == 'e' || next_3[0] == 'E' || next_3[0] == '.')
			&& (is_match!(next_3[1], '0'..'9'))
		) {
			push_char!(string, next_3[0]);
			push_char!(string, next_3[1]);
			self.position += 2;
		} 
		else if (
			next_3.len() == 3
			&& (next_3[0] == 'e' || next_3[0] == 'E')
			&& (next_3[1] == '+' || next_3[1] == '-')
			&& is_match!(next_3[2], '0'..'9')
		) {
			push_char!(string, next_3[0]);
			push_char!(string, next_3[1]);
			push_char!(string, next_3[2]);
			self.position += 3;
		} 
		else {
			return (Err(string),error_condition)
		}
		while !self.is_eof() && is_match!(
			match self.current_char() {
				(Some(ch),_)=>ch,
				_=> {
					self.position = head_position;
					return (Err(string),LEXER_NEEDDATA);
				}
			}, '0'..'9') {
			let c= match(self.consume_char()) {
					(Some(ch),x)=>{error_condition=x;ch},
					(None,_)=> {
						self.position = head_position;
						return (Err(string),LEXER_NEEDDATA);
					}
			};
			push_char!(string,c)
		}
		let value = Float(float::from_str(string).unwrap());
		// io::println(fmt!("consume_scientific_number: here number is %?" , string));
		let (some_token , error) = self.consume_numeric_end(copy string, value);
		match error {
			LEXER_NEEDDATA => {
				self.position = head_position;
				return(Ok(some_token.unwrap()) , error);
			},
			_=> return(Err(string) , error),
		}
	}
}