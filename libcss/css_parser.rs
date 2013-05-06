#[link(name = "css_parser", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_properties;
extern mod css_language;
extern mod css_lexer;
extern mod css_enum;
extern mod wapcaplet;
extern mod std;

use css_properties::*;
use css_language::*;
use css_lexer::*;
use css_enum::*;
use wapcaplet::*;
use std::arc;


struct parser_state {
	state: uint,
	substate: uint
}


pub struct css_parser {
	lexer: ~css_lexer,
	//language: ~css_language,
	lwc: arc::RWARC<~lwc>,
	states: ~[parser_state],
	tokens: ~[~css_token],
	pushback: Option<~css_token>,
	last_was_ws : bool

}

impl css_parser {
	pub fn eat_ws(&mut self) -> css_result
	{
		/*const css_token *token;
		css_error error;

		error = getToken(parser, &token);
		if (error != CSS_OK)
			return error;

		if (token->type != CSS_TOKEN_S) {
			error = pushBack(parser, token);
			if (error != CSS_OK)
				return error;
		}*/

		CSS_OK
	}

	pub fn push_back(&mut self, token: ~css_token) -> css_result {
		assert!(self.pushback.is_none());

		self.pushback = Some(token);
		self.tokens.pop();

		CSS_OK
	}


	fn intern_string (&mut self, string: ~str) -> arc::RWARC<~lwc_string> {
		let mut interned_string: Option<arc::RWARC<~lwc_string>> = None;

		do self.lwc.write |lwc| {
			interned_string = Some(lwc.lwc_intern_string(copy string));
		}

		interned_string.unwrap()
	}

	pub fn get_token(&mut self) -> (css_result, Option<~css_token>) {

		let mut token: Option<~css_token> = None;

		/* Use pushback, if it exists */
		if self.pushback.is_some() {
			token = Some(self.pushback.swap_unwrap());
		}
		else {
			let (lexer_token_option, lexer_error) = self.lexer.get_token();

			match lexer_error {
				LEXER_OK => {
					let lexer_token = lexer_token_option.unwrap();
					// lexer has returned a token
					match (lexer_token) {
						CSS_TOKEN_IDENT(copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						},
						CSS_TOKEN_ATKEYWORD(copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						},
						CSS_TOKEN_HASH(copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						},
						CSS_TOKEN_FUNCTION(copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						}, 
						CSS_TOKEN_STRING(copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						}, 
						CSS_TOKEN_INVALID_STRING => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						}, 
						CSS_TOKEN_URI(copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						}, 
						CSS_TOKEN_UNICODE_RANGE(_ , _) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						}, 
						CSS_TOKEN_CHAR(_) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						},
						CSS_TOKEN_NUMBER(_ , copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						}, 
						CSS_TOKEN_PERCENTAGE(_ , copy value) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value)),
							})
						}, 
						CSS_TOKEN_DIMENSION(_ , copy value1, copy value2) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						},
						CSS_TOKEN_CDO => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						}, 
						CSS_TOKEN_CDC => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						}, 
						CSS_TOKEN_S => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						},
						Delim(char) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						},
						CSS_TOKEN_EOF => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : None,
							})
						}
					}
				}

				LEXER_NEEDDATA => {
					return (CSS_NEEDDATA, None);
				}

				LEXER_INVALID => {
					return (CSS_INVALID, None);
				}
			}
		}


		(CSS_OK, token)
	}
}