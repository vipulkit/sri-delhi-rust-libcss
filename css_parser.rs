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

pub fn lcss_parser()->~lcss_parser {
    let mut parser :~lcss_parser= 
    	~lcss_lexer{ transform_function_whitespace: false,
    	input: ~[],
    	lcss_lexer_instance: lcss_lexer() , 
    	position: 0 } ;
    parser
}

/*
 * Css parser event handler function pointer
 */

// null function for initializing
pub fn dummy_par_ev_hand(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:&css_language) -> css_result {
	CSS_OK
}


fn Stylesheet_event_handler(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:&css_language)-> css_result
{
	CSS_OK
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
	CSS_OK
	}
	pub fn css__parser_create(&self)  {

	}
	pub fn css__parser_parse_chunk(&self, data:~[u8]) -> css_result{
     CSS_OK
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
		CSS_OK
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

	return CSS_OK;
}
}
