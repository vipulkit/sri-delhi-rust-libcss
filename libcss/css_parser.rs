#[link(name = "css_parser", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_properties;
extern mod css_language;
extern mod css_lexer;

use css_properties::*;
use css_language::*;
use css_lexer::*;


struct parser_state {
	state: uint,
	substate: uint
}


pub struct css_parser {
	lexer: ~css_lexer,
	language: ~css_language,
	states: ~[parser_state],tokens: ~[~css_token],
	pushback: ~css_token,

}
