use wapcaplet::*;
use std::arc;
use parserutils::charset::csdetect::*;
use parserutils::input::inputstream::*;
use parserutils::charset::csdetect::*;

// libcss uses
use lex::lexer::*;
use parse::language::*;
use parse::parse::*;
use stylesheet::*;
use utils::errors::*;

pub struct css {
	priv lwc:arc::RWARC<~lwc>,
	priv stylesheet:@mut css_stylesheet,
	priv parser:~css_parser,

}

pub struct css_params {
		/** ABI version of this structure */
		params_version : uint,

		/** The language level of the stylesheet */
		level: css_language_level,

		/** The charset of the stylesheet data, or NULL to detect */
		charset : Option<~str>,
		/** URL of stylesheet */
		url : ~str,
		/** Title of stylesheet */
		title : ~str,

		/** Permit quirky parsing of stylesheet */
		allow_quirks : bool,
		/** This stylesheet is an inline style */
		inline_style : bool,

		/** URL resolution function */
		resolve : css_url_resolution_fn,

		/** Import notification function */
		import : Option<css_import_notification_fn>,

		/** Colour resolution function */
		color : Option<css_color_resolution_fn>,

		/** Font resolution function */
		font : Option<css_font_resolution_fn>,
}

pub impl css {
	pub fn css_create(params: css_params) -> ~css {
		// create lwc
		let lwc = lwc();

		// create inputstream
		let (inputstream_option, inputstream_error) =  
			match copy params.charset {
				None => inputstream(None, None ,Some(~css__charset_extract)),
				Some(charset) => inputstream(Some(charset), Some(CSS_CHARSET_DICTATED), Some(~css__charset_extract))
			};

		// create lexer
		let lexer = css_lexer::css__lexer_create(inputstream_option.unwrap());

		// create stylesheet
		let stylesheet = @mut css_stylesheet {
			selectors:css_selector_hash::css__selector_hash_create(),       
			rule_count:0,                        
			rule_list:None,   
			last_rule:None,   
			disabled:false,                          
			url:copy params.url,                               
			title:copy params.title,                             
			level:params.level,               
			quirks_allowed:params.allow_quirks,                    
			quirks_used:false,                       
			inline_style:params.inline_style,                      
			cached_style:None,    
			string_vector:~[],
			resolve : params.resolve, 
			import : params.import, 
			font : params.font,   
			color: params.color
		};

		// create language
		let language = css_language(stylesheet, lwc.clone());

		// create parser
		let parser = match params.inline_style {
		    false => css_parser::css__parser_create(language, lexer, lwc.clone()),
		    true => css_parser::css__parser_create_for_inline_style(language, lexer, lwc.clone())
		}; 

		~ css {
			lwc:lwc.clone(),
			parser:parser.unwrap(),
			stylesheet:stylesheet
		}
	}

	pub fn css_stylesheet_append_data(&mut self, data:~[u8]) -> css_result {
		self.parser.css__parser_parse_chunk(data)
	}

	pub fn css_stylesheet_data_done(&mut self) -> (css_result , Option<@mut css_stylesheet>) {
		let error = self.parser.css__parser_completed();

		self.stylesheet.cached_style = None;

		// TODO <Abhijeet>: Handle pending imports

		(error , Some(self.stylesheet))
	}
}