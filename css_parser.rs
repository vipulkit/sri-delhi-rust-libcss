#[link(name = "css_parser", vers = "0.1")];
#[crate_type = "lib"];

extern mod parserutils;
extern mod css_ds;
extern mod css_enum;
extern mod css_lexer;

use parserutils::*;
use css_enum::*;
use css_ds::*;
use css_lexer::*;

/*
 * Css parser main strcuture
*/

pub struct lcss_parser {

	//stream:@parserutils_inputstream,	/* < The inputstream */
	//lexer:@css_lexer,		/* < The lexer to use */

	// quirks:bool	/* < Whether to enable parsing quirks */

// #define STACK_CHUNK 32
    //STACK_CHUNK: uint,
	// states:DVec<u8>,	/* < Stack of states */

	//tokens:~[u8],	/* < Vector of pending tokens */

	//pushback:@css_token,	/* < Push back buffer */

	//parseError:bool,		/* < A parse error has occurred */
	//open_items:DVec<u8>,	/* < Stack of open brackets */

	//match_char:u8,		/* < Close bracket type for parseAny */

	//last_was_ws:bool,		/* < Last token was whitespace */

	//css_allocator_fn alloc;		/* < Memory (de)allocation function */
	//void *pw;			/**< Client-specific private data */
	// event:@css_parser_event_handler,	/* < Client's event handler */
	event_pw:@css_language,		/* < Client data for event handler */
	quirks:bool,
	lcss_lexer_instance:@lcss_lexer,
	lparserutils_instance:@lpu
}