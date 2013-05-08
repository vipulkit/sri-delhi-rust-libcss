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
			~css_parser::parse_stylesheet,
			~css_parser::parse_statement,
			~css_parser::parse_ruleset,
			~css_parser::parse_ruleset_end,
			~css_parser::parse_at_rule,
			~css_parser::parse_at_rule_end,
			~css_parser::parse_block,
			~css_parser::parse_block_content,
			~css_parser::parse_selector,
			~css_parser::parse_declaration,
			~css_parser::parse_decl_list,
			~css_parser::parse_decl_list_end,
			~css_parser::parse_property,
			~css_parser::parse_value_0,
			~css_parser::parse_value_1,
			~css_parser::parse_value,
			~css_parser::parse_any_0,
			~css_parser::parse_any_1,
			~css_parser::parse_any,
			~css_parser::parse_malformed_declaration,
			~css_parser::parse_malformed_selector,
			~css_parser::parse_malformed_at_rule,
			~css_parser::parse_inline_style,
			~css_parser::parse_IS_body_0,
			~css_parser::parse_IS_body
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

		let mut token: Option<~css_token>;

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
						Delim(_) => {
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

		let mut (_,current_substate) = parser.stack.pop();

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
				} /*AfterStylesheet*/,

				_ => {
					fail!();
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

		let mut (_,current_substate) = parser.stack.pop();

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
						fail!();
					}
				}
			} /* while */

		CSS_OK
	} /* parse_stylesheet */

	pub fn parse_statement(parser: &mut ~css_parser) -> css_result
	{
		enum parser_statement_sub_states { 
			Initial = 0 
		};

		let mut to = (sRuleset as uint, Initial as uint);

		let (token_option, parser_error) = parser.get_token();
		if (token_option.is_none()) {
			return parser_error;
		}
		let token = token_option.unwrap();

		match (token.token_type) {
			CSS_TOKEN_ATKEYWORD(_) => {
				to = (sAtRule as uint, Initial as uint);
			}
			_ => {}
		}


		let push_back_result = parser.push_back(token);
		
		match (push_back_result) {
			CSS_OK => {
				/* continue */
			},
			_ => {
				return push_back_result;
			}
		}

		parser.transition_no_ret(to);

		return CSS_OK;
	} /* parse statement */


	pub fn parse_ruleset(parser: &mut ~css_parser) -> css_result {
		enum parse_ruleset_sub_states { 
			Initial = 0, 
			Brace = 1, 
			WS = 2 
		};
		
		let mut (_,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {
				0 /* Initial */ => { 
		                
					parser.tokens.clear();

					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_CHAR (c) => {
							if (c=='{') {
								match (
									parser.language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens)
								) {
									CSS_INVALID => {
										let to = (sMalformedSelector as uint, Initial as uint);
										parser.transition_no_ret(to);

										return CSS_OK;
									} /* CSS_INVALID */
									_ => {
										current_substate = WS as uint;
									}
								}
							}

						}

						_ => {
							let to = (sSelector as uint, Initial as uint);
							let subsequent = (sRuleset as uint, Brace as uint);

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
						}
					} /* match token.token_type */
				} /* Initial */
			
				1 /* Brace */ => {
					if (!parser.parse_error) {
						match (
							parser.language.language_handle_event(CSS_PARSER_START_RULESET, &parser.tokens)
						) {
							CSS_INVALID => {
								parser.parse_error = true;
							}
							_ => {

							}
						}
					} /* if */

					if (parser.parse_error) {
						let to = (sMalformedSelector as uint, Initial as uint);

						parser.transition_no_ret(to);

						return CSS_OK;
					}

					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_EOF => {
							let push_back_result = parser.push_back(token);
							
							match (push_back_result) {
								CSS_OK => {
									parser.done();
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							}
						} /* CSS_TOKEN_EOF */
						CSS_TOKEN_CHAR (c) => {
							if (c != '{') {
								fail!(); // Should not happen
							}
							current_substate = WS as uint;
						}

						_ => {
							fail!(); // Should not happen
						}
					} /* match token_type */
				}

				2 /* WS */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							break;
						}
						_ => {
							return eat_ws_result;
						}
					}
				}

				_ => {
					fail!();
				}
			} /* match current_substate */

		} /* while */

		let mut to = (sRulesetEnd as uint, Initial as uint);
		parser.transition_no_ret(to);

		CSS_OK
	} /* parse_ruleset */


	pub fn parse_ruleset_end(parser:&mut ~css_parser) -> css_result {
		enum parse_ruleset_end_substates { 
			Initial = 0, 
			DeclList = 1, 
			Brace = 2, 
			WS = 3 
		};
		
		let mut (_,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {
				0 /* Initial */ => {
					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_EOF => {
							let push_back_result = parser.push_back(token);
								
							match (push_back_result) {
								CSS_OK => {
									parser.done();
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							}
						} /* CSS_TOKEN_EOF */

						CSS_TOKEN_CHAR(c) => {
							if (c != '}' && c != ';') {
								/* If this can't possibly be the start of a decl-list, then
						 		 * attempt to parse a declaration. This will catch any invalid
		 		 				 * input at this point and read to the start of the next
		 						 * declaration. FIRST(decl-list) = (';', '}') */
								let push_back_result = parser.push_back(token);
								
								match (push_back_result) {
									CSS_OK => {
										let to = (sDeclaration as uint, Initial as uint);
										let subsequent = (sRulesetEnd as uint, DeclList as uint);

										parser.transition(to, subsequent);
										return CSS_OK;
									},
									_ => {
										return push_back_result;
									}
								}

							}
							current_substate = DeclList as uint;
						} /* CSS_TOKEN_CHAR */

						_ => {
							let push_back_result = parser.push_back(token);
								
							match (push_back_result) {
								CSS_OK => {
									let to = (sDeclaration as uint, Initial as uint);
									let subsequent = (sRulesetEnd as uint, DeclList as uint);

									parser.transition(to, subsequent);
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							} /* _ */
						}
					} /* match token_type */
				} /* Initial */

				1 /* DeclList */ =>  {
					let to = (sDeclList as uint, Initial as uint);
					let subsequent = (sRuleset as uint, Brace as uint);

					parser.transition(to,subsequent);
					return CSS_OK;
				} /* DeclList */

				2 /* Brace */ => {
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
									parser.done();
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							}
						} /* CSS_TOKEN_EOF */

						CSS_TOKEN_CHAR(c) => {
							if (c != '}') {
								/* This should never happen, as FOLLOW(decl-list)
					 			 * contains only '}' */
								fail!();
							}
							current_substate = WS as uint;
						}
						_ => {
							fail!();
						}
					}
				} /* Brace */

				3 /* WS */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							break;
						}
						_ => {
							return eat_ws_result;
						}
					}
				} /* WS */
				_ => {
					fail!();
				}
			} /* match current_substate */
		} /* while */

		parser.done();
		CSS_OK
	} /* parse_ruleset_end */

	pub fn parse_at_rule(parser: &mut ~css_parser) -> css_result {
		
		enum parse_at_rule_substates { 
			Initial = 0, 
			WS = 1, 
			Any = 2, 
			AfterAny = 3 
		};

		let mut (_,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {
				0 /* Initial */ => {
					parser.tokens.clear();

					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_ATKEYWORD (_)=> {
							current_substate = WS as uint;		
						}
						_ => {
							fail!();
						}
					}
				} /* Initial */

				1 /* WS */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							current_substate = Any as uint;
						}
						_ => {
							return eat_ws_result;
						}
					}
				} /* WS */

				2 /* Any */ => {
					let to = ( sAny0 as uint, Initial as uint);
					let subsequent = ( sAtRule as uint, AfterAny as uint);

					parser.transition(to, subsequent);
					return CSS_OK;
				} /* Any */

				3 /* AfterAny */ => {
					if (parser.parse_error) {
						let to = (sMalformedAtRule as uint, Initial as uint);

						parser.transition_no_ret(to);
						return CSS_OK;
					} /* if */
					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_CHAR (c)=> {
							if (c==')' || c==']') {
								let to = (sAny0 as uint, Initial as uint);
								let subsequent = (sAtRule as uint, AfterAny as uint);

								parser.transition(to, subsequent);
								return CSS_OK;
							}
							else {
								let push_back_result = parser.push_back(token);

								match (push_back_result) {
									CSS_OK => {
										break;
									},
									_ => {
										return push_back_result;
									}
								}
							}
						}
						_ => {
							let push_back_result = parser.push_back(token);

							match (push_back_result) {
								CSS_OK => {
									break;
								},
								_ => {
									return push_back_result;
								}
							}
						}
					}
				} /* AfterAny */

				_ => {
					fail!();
				} /* _ */
			} /* match current_substate */
		} /* while */

		let to = (sAtRuleEnd as uint, Initial as uint);
		parser.transition_no_ret(to);

		CSS_OK
	} /* parse_at_rule */

	pub fn parse_at_rule_end(parser: &mut ~css_parser) -> css_result {
		
		enum parser_at_rule_end_substates { 
			Initial = 0, 
			WS = 1, 
			AfterBlock = 2 
		};
		
		let mut (_,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {

				0 /* Initial */ => {
					match(parser.language.language_handle_event(CSS_PARSER_START_ATRULE, & parser.tokens)) {
						CSS_INVALID => {
							let to = (sMalformedAtRule as uint, Initial as uint);

							parser.transition_no_ret(to);
							return CSS_OK;
						}
						_=> {}
					}

					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_EOF => {
							let push_back_result = parser.push_back(token);
								
							match (push_back_result) {
								CSS_OK => {
									parser.done();
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							}
						} /* CSS_TOKEN_EOF */

						CSS_TOKEN_CHAR(c) => {
							if (c=='{') {
								let push_back_result = parser.push_back(token);

								match (push_back_result) {
									CSS_OK => {},
									_ => {
										return push_back_result;
									}
								}

								let to = (sBlock as uint, Initial as uint);
								let subsequent = (sAtRuleEnd as uint, AfterBlock as uint);
								parser.transition(to,subsequent);
								return CSS_OK;
							} /* if */
							else if (c==';') {}
							else {
								/* should never happen */
								fail!();
							}
						}

						_ => {
							/* should never happen */
								fail!();
						}
					} /* match token_type */
					current_substate = WS as uint;
				} /* Initial */

				1 /* WS */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							break;
						}
						_ => {
							return eat_ws_result;
						}
					}
				} /* WS */

				2 /* AfterBlock */ => {
					break;
				} /* AfterBlock */

				_ => {
					fail!();
				}

			} /* match current_substate */
		} /* while */
	
		parser.language.language_handle_event(CSS_PARSER_END_ATRULE, &parser.tokens);

		parser.done();
		CSS_OK
	} /* parse_at_rule_end */

	pub fn parse_block(parser: &mut ~css_parser) -> css_result {
		enum parse_block_substates { 
			Initial = 0, 
			WS = 1, 
			Content = 2, 
			Brace = 3, 
			WS2 = 4 
		};

		let mut (_,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {
				0 /* Initial */ => {
					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					parser.language.language_handle_event(CSS_PARSER_START_BLOCK, &parser.tokens);

					match (token.token_type) {
						CSS_TOKEN_CHAR(c) => {
							if (c != '{') {
								/* This should never happen, as FIRST(block) == '{' */
								fail!();
							}
						}
						_ => {
							/* This should never happen, as FIRST(block) == '{' */
							fail!();
						}
					} /* match token_type */

					parser.tokens.clear();
					current_substate = WS as uint;
				} /* Initial */
				
				1 /* WS */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							current_substate = Content as uint;
						}
						_ => {
							return eat_ws_result;
						}
					}
				} /* WS */

				2 /* Content */ => {
					let to = (sBlockContent as uint, Initial as uint);
					let subsequent = (sBlock as uint, Brace as uint);

					parser.transition(to, subsequent);
					return CSS_OK;
				} /* Content */

				3 /* Brace */ => {
					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_EOF => {
							let push_back_result = parser.push_back(token);
								
							match (push_back_result) {
								CSS_OK => {
									parser.done();
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							}
						} /* CSS_TOKEN_EOF */

						CSS_TOKEN_CHAR(c) => {
							if (c != '}') {
								/* This should never happen, as 
								 * FOLLOW(block-content) == '}' */
								fail!();
							}
						} /* CSS_TOKEN_CHAR */

						_ => {
							fail!();
						}
					} /* match token_type */

					current_substate = WS2 as uint;
				} /* Brace */

				4 /* WS2 */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							break;
						}
						_ => {
							return eat_ws_result;
						}
					}
				} /* WS2 */

				_ => {
					fail!();
				}
			} /* match current_substate */
		} /* while */
		
		parser.language.language_handle_event(CSS_PARSER_END_BLOCK, &parser.tokens);
		parser.tokens.clear();
		parser.done();

		CSS_OK
	} /* parse_block */

	pub fn parse_block_content(parser: &mut ~css_parser) -> css_result {
		
			enum parse_block_content_substates { 
				Initial = 0, 
				WS = 1 
			};
			
			let mut (_,current_substate) = parser.stack.pop();

			while (true) {
				match (current_substate) {
					0 /* Initial */ => {
						let (token_option, parser_error) = parser.get_token();
						if (token_option.is_none()) {
							return parser_error;
						}
						let mut token = token_option.unwrap();

						match (token.token_type) {
							CSS_TOKEN_ATKEYWORD(_) => {
								current_substate = WS as uint;
							} /* CSS_TOKEN_ATKEYWORD */
							
							CSS_TOKEN_CHAR(c) => {
								if (c=='{') { /* Grammar ambiguity. Assume block */
									let push_back_result = parser.push_back(token);
										
									match (push_back_result) {
										CSS_OK => {
											parser.language.language_handle_event(CSS_PARSER_BLOCK_CONTENT, &parser.tokens);
											parser.tokens.clear();

											let to = (sBlock as uint, Initial as uint);
											let subsequent = (sBlockContent as uint, Initial as uint);

											parser.transition(to, subsequent);

											return CSS_OK;
										},
										_ => {
											return push_back_result;
										}
									}									
								} /* if */
								else if (c==';') { /* Grammar ambiguity. Assume semi */
									let push_back_result = parser.push_back(token);
										
									match (push_back_result) {
										CSS_OK => {
											parser.language.language_handle_event(CSS_PARSER_BLOCK_CONTENT, &parser.tokens);

											let (token_option, parser_error) = parser.get_token();
											if (token_option.is_none()) {
												return parser_error;
											}
											token = token_option.unwrap();
											// TODO <Abhijeet> : Doesn't get used anywhere, why?

											parser.tokens.clear();

											current_substate = WS as uint;
										},
										_ => {
											return push_back_result;
										}
									} /* match push_back_result */									
								} /* else if */
								else if (c=='}') { /* Grammar ambiguity. Assume end */
									let push_back_result = parser.push_back(token);
										
									match (push_back_result) {
										CSS_OK => {
											parser.language.language_handle_event(CSS_PARSER_BLOCK_CONTENT, &parser.tokens);
											parser.tokens.clear();

											parser.done();
											return CSS_OK;
										},
										_ => {
											return push_back_result;
										}
									} /* match push_back_result */	
								} /* else if */
							} /* CSS_TOKEN_CHAR */

							CSS_TOKEN_EOF => {
								let push_back_result = parser.push_back(token);
									
								match (push_back_result) {
									CSS_OK => {
										parser.language.language_handle_event(CSS_PARSER_BLOCK_CONTENT, &parser.tokens);
										parser.tokens.clear();

										parser.done();
										return CSS_OK;
									},
									_ => {
										return push_back_result;
									}
								} /* match push_back_result */
							} /* CSS_TOKEN_EOF */

							_ => {

							}
						} /* match token_type */

						if (current_substate == Initial as uint) {
							let push_back_result = parser.push_back(token);
								
							match (push_back_result) {
								CSS_OK => {
									let to = (sAny as uint, Initial as uint);
									let subsequent = (sBlockContent as uint, Initial as uint);

									parser.transition(to, subsequent);
									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							} /* match push_back_result */							
						} /* if */

					} /* Initial */

					1 /* WS */ => {
						let eat_ws_result = parser.eat_ws();
						match (eat_ws_result) {
							CSS_OK => {
								
							}
							_ => {
								return eat_ws_result;
							}
						}
						current_substate = Initial as uint;
					} /* WS */

					_ => {
						fail!();
					} /* _ */
				} /* match current_substate */
			} /* while */
		
		parser.done();
		CSS_OK
	} /* parse_block_content */

	pub fn parse_selector(parser: &mut ~css_parser) -> css_result {
		enum parse_selector_substates { 
			Initial = 0, 
			AfterAny1 = 1 
		};
		
		let (_,current_substate) = parser.stack.pop();

		match (current_substate) {
			0 /* Initial */ => {
				parser.tokens.clear();

				let to = (sAny1 as uint, Initial as uint);
				let subsequent = (sSelector as uint, AfterAny1 as uint);

				parser.transition(to, subsequent);
				return CSS_OK;				
			} /* Initial */

			1 /* AfterAny1 */ => {
				/* do nothing */
			} /* AfterAny1 */

			_ => {
				fail!();
			}
		}
		
		parser.done();
		CSS_OK
	} /* parse_selector */

	pub fn parse_declaration(parser: &mut ~css_parser) -> css_result {
		enum parser_declaration_substates { 
			Initial = 0, 
			Colon = 1, 
			WS = 2, 
			AfterValue1 = 3 
		};

		let mut (_,current_substate) = parser.stack.pop();

		while (true) {
			match (current_substate) {
				0 /* Initial */ => {
					parser.tokens.clear();

					let to = ( sProperty as uint, Initial as uint);
					let subsequent = ( sDeclaration as uint, Colon as uint);
					parser.transition(to, subsequent);

					return CSS_OK;
				} /* Initial */
				
				1 /* Colon */ => {
					if (parser.parse_error) {
						let to = ( sMalformedDecl as uint, Initial as uint);
						parser.parse_error = false;

						parser.transition_no_ret(to);

						return CSS_OK;
					}

					let (token_option, parser_error) = parser.get_token();
					if (token_option.is_none()) {
						return parser_error;
					}
					let token = token_option.unwrap();

					match (token.token_type) {
						CSS_TOKEN_EOF => {

						} /* CSS_TOKEN_EOF */

						CSS_TOKEN_CHAR (c) => {
							if (c != ':') { /* parse error -- expected : */
								let push_back_result = parser.push_back(token);
									
								match (push_back_result) {
									CSS_OK => {
										let to = (sMalformedDecl as uint, Initial as uint);
										parser.transition_no_ret(to);

										return CSS_OK;
									},
									_ => {
										return push_back_result;
									}
								} /* match push_back_result */
							} /* if */
						} /* CSS_TOKEN_CHAR */

						_ => { /* parse error -- expected : */
							let push_back_result = parser.push_back(token);
								
							match (push_back_result) {
								CSS_OK => {
									let to = (sMalformedDecl as uint, Initial as uint);
									parser.transition_no_ret(to);

									return CSS_OK;
								},
								_ => {
									return push_back_result;
								}
							} /* match push_back_result */
						}
					} /* match token_type */

					current_substate = WS as uint; /* Fall through */
				} /* Colon */

				2 /* WS */ => {
					let eat_ws_result = parser.eat_ws();
					match (eat_ws_result) {
						CSS_OK => {
							let to = (sValue1 as uint, Initial as uint);
							let subsequent = (sDeclaration as uint, AfterValue1 as uint);
							
							parser.transition(to, subsequent);
							return CSS_OK;							
						}
						_ => {
							return eat_ws_result;
						}
					}
				} /* WS */

				3 /* AfterValue1 */ => {
					if (parser.parse_error) {
						parser.parse_error = false;

						let to = (sMalformedDecl as uint, Initial as uint);
						parser.transition_no_ret(to);

						return CSS_OK;
					}

					parser.language.language_handle_event(CSS_PARSER_DECLARATION, &parser.tokens);
				} /* AfterValue1 */

				_ => {
					fail!();
				}
			} /* match current_substate */
		} /* while */

		parser.done();
		CSS_OK
	} /* parse_declaration */

	pub fn parse_decl_list(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_decl_list */

	pub fn parse_decl_list_end(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_decl_list_end */

	pub fn parse_property(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_property */

	pub fn parse_value_0(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_value_0 */

	pub fn parse_value_1(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_value_1 */

	pub fn parse_value(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_value */

	pub fn parse_any_0(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_any_0 */

	pub fn parse_any_1(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_any_1 */

	pub fn parse_any(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_any */

	pub fn parse_malformed_declaration(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_malformed_declaration */

	pub fn parse_malformed_selector(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_malformed_selector */

	pub fn parse_malformed_at_rule(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_malformed_at_rule */

	pub fn parse_inline_style(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_inline_style */

	pub fn parse_IS_body_0(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_IS_body_0 */

	pub fn parse_IS_body(parser: &mut ~css_parser) -> css_result {
		CSS_OK
	} /* parse_IS_body */

}