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


/**
 * Major state numbers
 */
pub enum parse_states {
	sStart = 0,
	sStylesheet = 1,
	sStatement = 2,
	sRuleset = 3,
	sRulesetEnd = 4,
	sAtRule = 5,
	sAtRuleEnd = 6,
	sBlock = 7,
	sBlockContent = 8,
	sSelector = 9,
	sDeclaration = 10,
	sDeclList = 11,
	sDeclListEnd = 12,
	sProperty = 13,
	sValue0 = 14,
	sValue1 = 15,
	sValue = 16,
	sAny0 = 17,
	sAny1 = 18,
	sAny = 19,
	sMalformedDecl = 20,
	sMalformedSelector = 21,
	sMalformedAtRule = 22,
	sInlineStyle = 23,
	sISBody0 = 24,
	sISBody = 25
}

type state =  ~extern fn(parser: &mut ~css_parser) ->css_result;

pub struct css_parser {
	language: ~css_language,
	lexer: ~css_lexer,
	lwc: arc::RWARC<~lwc>,

	last_was_ws : bool,
	parse_error : bool,
	pushback: Option<~css_token>,
	stack: ~[(uint,uint)], /*Parser state stack*/
	states: ~[state],
	tokens: ~[~css_token],
}

impl css_parser {

	/* constructor */
	pub fn css_parser(language: ~css_language, lexer: ~css_lexer, lwc: arc::RWARC<~lwc> ) 
		-> Option<~css_parser> {
		
		let mut states = ~[
			~css_parser::parse_start,
			~css_parser::parse_stylesheet
		];

		Some(~css_parser {
			language: language,
			lexer: lexer,
			lwc: lwc.clone(),

			last_was_ws: false,
			parse_error: false,
			pushback: None,
			stack: ~[],
			states: states,
			tokens: ~[],
			
		})
	}

	/* Utility functions */

	/* writing this function in our current architecture is too cumbersome */
	/* the functionality should be implemented by each calling function */
	/* pub fn expect(&mut self, css_token_type token_type) -> css_result */

	pub fn transition(&mut self, to:(uint,uint), subsequent:(uint,uint))
	{
		
		/* Replace current state on the stack with the subsequent one */
		self.stack.pop();
		self.stack.push(subsequent);

		/* Push next state on the stack */
		self.stack.push(to);

		self.parse_error = false;
	}

	pub fn transition_no_ret(&mut self, to:(uint,uint))
	{
		/* Replace current state on the stack with destination */
		self.stack.pop();
		self.stack.push(to);

		self.parse_error = false;
	}

	pub fn done(&mut self)
	{
		/* Pop current state from stack */
		self.stack.pop();
	}

	pub fn eat_ws(&mut self) -> css_result
	{
		let (token_option, parser_error) = self.get_token();

		if (token_option.is_none()) {
			return parser_error;
		}

		let token = token_option.unwrap();

		match token.token_type {
			CSS_TOKEN_S => {
				/* do nothing */
			}
			_=> {
				return(self.push_back(token));
			}
			
		}

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

	pub fn get_token(&mut self) -> (Option<~css_token>, css_result) {

		let mut token: Option<~css_token> = None;

		/* Use pushback, if it exists */
		if self.pushback.is_some() {
			token = Some(self.pushback.swap_unwrap());
		}
		else {
			/* Otherwise, ask the lexer */
			let (lexer_token_option, lexer_error) = self.lexer.get_token();

			match lexer_error {
				LEXER_OK => {
					/* Lexer has returned a valid token with no errors */
					let lexer_token = lexer_token_option.unwrap();
					
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
						CSS_TOKEN_DIMENSION(_ , _ , copy value2) => {
							token = Some (~css_token {
								token_type : lexer_token,
								idata : Some(self.intern_string(value2)),
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
					/*Lexer doesn't have enough data to create a token*/
					return (None, CSS_NEEDDATA);
				}

				LEXER_INVALID => {
					/*Lexer had encountered invalid data, cannot proceed*/
					return (None, CSS_INVALID);
				}
			}
		}


		(token, CSS_OK)
	}

	/* parser states */
	pub fn parse_start(parser:&mut ~css_parser) -> css_result {
		enum parse_start_sub_states { 
			Initial = 0, 
			AfterWS = 1, 
			AfterStylesheet = 2 
		};

		let mut (current_state,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {
				0 /*Initial*/ => {
					parser.language.language_handle_event(CSS_PARSER_START_STYLESHEET, &parser.tokens);
					current_substate = AfterWS as uint;
				},
				1 /*AfterWS*/ => {
					let to = (sStylesheet as uint, Initial as uint);
					let subsequent = (sStart as uint, AfterWS as uint);
					parser.transition(to, subsequent);

					return CSS_OK;
				},
				2 /*AfterStylesheet*/ => {
					let (token_option, parser_error) = parser.get_token();

							if (token_option.is_none()) {
								return parser_error;
							}

							let token = token_option.unwrap();

							match token.token_type {
								CSS_TOKEN_EOF => {
									/* do nothing, as expected*/
									break;
								}
								_=> {
									let push_back_result = parser.push_back(token);
									match (push_back_result) {
										CSS_OK => {
											return CSS_INVALID;
										},
										_ => {
											return push_back_result;
										}
									}
								}
								
							}
				},

				_ => {
					return CSS_INVALID;
				}
			}
		}


		parser.language.language_handle_event(CSS_PARSER_END_STYLESHEET, &parser.tokens);
		parser.tokens.clear();

		return CSS_OK;
	} /* parse_start */


	pub fn parse_stylesheet(parser:&mut ~css_parser) -> css_result {
		enum parse_stylesheet_sub_states { 
			Initial = 0, 
			WS = 1 
		};

		let mut (current_state,current_substate) = parser.stack.pop();

			while (true) {
				match (current_substate) {
					0 /*Initial*/=> {
						let (token_option, parser_error) = parser.get_token();

						if (token_option.is_none()) {
							return parser_error;
						}

						let token = token_option.unwrap();

						match token.token_type {
							CSS_TOKEN_EOF => {
								let push_back_result = parser.push_back(token);
									match (push_back_result) {
										CSS_OK => {
											parser.tokens.clear();
											parser.done();
											return CSS_OK;
										},
										_ => {
											return push_back_result;
										}
									}
								} /* CSS_TOKEN_EOF */
							CSS_TOKEN_CDO | CSS_TOKEN_CDC => {
								/*do nothing*/
							}
							_ => {
								let to = (sStatement as uint, Initial as uint);
								let subsequent = (sStylesheet as uint, WS as uint);
								
								let push_back_result = parser.push_back(token);
								
								match (push_back_result) {
									CSS_OK => {
										/* continue */
									},
									_ => {
										return push_back_result;
									}
								}

								parser.transition(to, subsequent);

								return CSS_OK;
							} /* _ */
						}
						current_substate = WS as uint;
					} /* Initial */

					1 /* WS */=> {
						let eat_ws_result = parser.eat_ws();
						match (eat_ws_result) {
							CSS_OK => {
								current_substate = Initial as uint;
							}
							_ => {
								return eat_ws_result;
							}
						}
					} /* WS */

					_ => {
						/* error */
						return CSS_INVALID;
					}
				}
			} /* while */

		CSS_OK
	} /* parse_stylesheet */

	
}