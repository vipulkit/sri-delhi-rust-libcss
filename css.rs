#[link(name = "css", vers = "0.1")];
#[crate_type = "lib"];



extern mod parserutils;
extern mod wapcaplet;

use wapcaplet::*;
use core::cast;
use core::vec::* ; 
use core::str::* ;
use core::str::raw::* ;
use core::vec::raw::* ;
use parserutils::* ;
use core::dvec::DVec;

/*
 * This file is part of Rust-LibCSS.
 */

//TO DO: Should be moved to fpmath
type css_fixed = i32;



//TO DO: Should be moved to libwapcaplet
pub type lwc_hash = u32;
pub type lwc_refcounter = u32;

//TO DO: Should be moved to types  ---- Start
/**
 * Source of charset information, in order of importance.
 * A client-dictated charset will override all others.
 * A document-specified charset will override autodetection or the default.
 */
pub enum css_charset_source {
	CSS_CHARSET_DEFAULT          = 0,	/**< Default setting */
	CSS_CHARSET_REFERRED         = 1,	/**< From referring document */
	CSS_CHARSET_METADATA         = 2,	/**< From linking metadata */
	CSS_CHARSET_DOCUMENT         = 3,	/**< Defined in document */
	CSS_CHARSET_DICTATED         = 4	//< Dictated by client 
}

/**
 * Stylesheet language level -- defines parsing rules and supported properties
 */

pub enum css_language_level {
	CSS_LEVEL_1                 = 0,	/**< CSS 1 */
	CSS_LEVEL_2                 = 1,	/**< CSS 2 */
	CSS_LEVEL_21                = 2,	/**< CSS 2.1 */
	CSS_LEVEL_3                 = 3		//< CSS 3 
}

const   CSS_LEVEL_DEFAULT :  css_language_level = CSS_LEVEL_21;	//< Default level >
/**
 * Stylesheet media types
 */
pub enum css_media_type {
	CSS_MEDIA_AURAL             = (1 << 0),
	CSS_MEDIA_BRAILLE           = (1 << 1),
	CSS_MEDIA_EMBOSSED          = (1 << 2),
	CSS_MEDIA_HANDHELD          = (1 << 3),
	CSS_MEDIA_PRINT             = (1 << 4),
	CSS_MEDIA_PROJECTION        = (1 << 5),
	CSS_MEDIA_SCREEN            = (1 << 6),
	CSS_MEDIA_SPEECH            = (1 << 7),
	CSS_MEDIA_TTY               = (1 << 8),
	CSS_MEDIA_TV                = (1 << 9),
	CSS_MEDIA_ALL		    = (1 << 0) |(1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) |(1 << 9)		
} 

/**
 * Stylesheet origin
 */
pub enum css_origin {
	CSS_ORIGIN_UA               = 0,	/**< User agent stylesheet */
	CSS_ORIGIN_USER             = 1,	/**< User stylesheet */
	CSS_ORIGIN_AUTHOR           = 2		//< Author stylesheet 
}

/** CSS colour -- AARRGGBB */
type css_color = u32;

/* CSS unit */
enum css_unit {
	CSS_UNIT_PX                 = 0x0,
	CSS_UNIT_EX                 = 0x1,
	CSS_UNIT_EM                 = 0x2,
	CSS_UNIT_IN                 = 0x3,
	CSS_UNIT_CM                 = 0x4,
	CSS_UNIT_MM                 = 0x5,
	CSS_UNIT_PT                 = 0x6,
	CSS_UNIT_PC                 = 0x7,

	CSS_UNIT_PCT                = 0x8,	/* Percentage */

	CSS_UNIT_DEG                = 0x9,
	CSS_UNIT_GRAD               = 0xa,
	CSS_UNIT_RAD                = 0xb,

	CSS_UNIT_MS                 = 0xc,
	CSS_UNIT_S                  = 0xd,

	CSS_UNIT_HZ                 = 0xe,
	CSS_UNIT_KHZ                = 0xf
}

/**
 * Type of a qualified name
 */
pub struct css_qname {
	/**
	 * Namespace URI:
	 *
	 * NULL for no namespace
	 * '*' for any namespace (including none)
	 * URI for a specific namespace
	 */
	ns : @lwc_string ,

	/**
	 * Local part of qualified name
	 */
	name : @lwc_string 
}  
//pub type css_code_t  =  ~[u32];

pub struct css_style{

	bytecode:~[css_code_t] ,
	used : u32,
	allocated: u32/*,
	sheet:@css_stylesheet*/


}

pub enum css_selector_type {
	CSS_SELECTOR_ELEMENT,
	CSS_SELECTOR_CLASS,
	CSS_SELECTOR_ID,
	CSS_SELECTOR_PSEUDO_CLASS,
	CSS_SELECTOR_PSEUDO_ELEMENT,
	CSS_SELECTOR_ATTRIBUTE,
	CSS_SELECTOR_ATTRIBUTE_EQUAL,
	CSS_SELECTOR_ATTRIBUTE_DASHMATCH,
	CSS_SELECTOR_ATTRIBUTE_INCLUDES,
	CSS_SELECTOR_ATTRIBUTE_PREFIX,
	CSS_SELECTOR_ATTRIBUTE_SUFFIX,
	CSS_SELECTOR_ATTRIBUTE_SUBSTRING

	}


pub	enum css_combinator {
	CSS_COMBINATOR_NONE,
	CSS_COMBINATOR_ANCESTOR,
	CSS_COMBINATOR_PARENT,
	CSS_COMBINATOR_SIBLING,
	CSS_COMBINATOR_GENERIC_SIBLING
} 


pub enum css_selector_detail_value_type {
	CSS_SELECTOR_DETAIL_VALUE_STRING,
	CSS_SELECTOR_DETAIL_VALUE_NTH
} 


pub struct css_selector_detail_value {
	string:&str,		/*< Interned string, or NULL */
	a:u32,
	b:u32			/*< Data for x = an + b */
} 

pub struct css_selector_detail {
	qname:css_qname,			/*< Interned name */
	value:css_selector_detail_value,	/** Detail value */

	type_of     :int ,    		    /*< Type of selector */
	comb        :int ,    		    /*< Type of combinator */
	next        :int ,     		    /*< Another selector detail 
						             * follows */
	value_type  :int,		        /*< Type of value field */
	negate      :int ,   		    /*< Detail match is inverted */
}
// enum css_selector_node
// {
// 	SomeSelectorNode(@mut css_selector_node),
//   	NoSelectorNode
// }
struct css_selector {
	combinator:@[css_selector],		/*< Combining selector */

	rule:@css_rule ,				/*< Owning rule */

/*#define CSS_SPECIFICITY_A 0x01000000
#define CSS_SPECIFICITY_B 0x00010000
#define CSS_SPECIFICITY_C 0x00000100
#define CSS_SPECIFICITY_D 0x00000001*/
	CSS_SPECIFICITY_A:uint,
	CSS_SPECIFICITY_B:uint,
	CSS_SPECIFICITY_C:uint,
	CSS_SPECIFICITY_D:uint,

	specificity:u32,			/*< Specificity of selector */

	 data:css_selector_detail		/*< Selector data */
}


pub enum css_rule_type {
	CSS_RULE_UNKNOWN,
	CSS_RULE_SELECTOR,
	CSS_RULE_CHARSET,
	CSS_RULE_IMPORT,
	CSS_RULE_MEDIA,
	CSS_RULE_FONT_FACE,
	CSS_RULE_PAGE
}

pub enum css_rule_parent_type {
	CSS_RULE_PARENT_STYLESHEET,
	CSS_RULE_PARENT_RULE
}
enum rule_stylesheet
{
	rule(int),  //update int toproper value
	stylesheet(int)
}
enum css_rule_node
{
	SomeRuleNode(@mut css_rule),
  	NoRuleNode
}
pub struct css_rule {
	parent:@rule_stylesheet,		
			/**< containing rule or owning 
						 * stylesheet (defined by ptype)
						 */
	next:@mut css_rule_node ,				/**< next in list */
	prev:@mut css_rule_node ,				/**< previous in list */

	rule_type  :  uint,		/**< css_rule_type */
		     index : uint,		/**< index in sheet */
		     items : uint,		/**< # items in rule */
		     ptype : uint		/*< css_rule_parent_type */
}

pub struct css_rule_selector {
	 base:css_rule,

	 selectors:@@css_selector,
	 style:css_style 
}
pub struct css_rule_media {
	base:css_rule ,

	media:u64,

	first_child:@css_rule,
	last_child:@css_rule
}
pub struct css_rule_font_face {
	base :css_rule,

	font_face:@css_font_face 
}

pub struct css_font_face {
	font_family:@lwc_string,
	srcs:@css_font_face_src,
	n_srcs:u32,
	
	/*
	 * Bit allocations:
	 *
	 *    76543210
	 *  1 __wwwwss	font-weight | font-style
	 */
	bits:~[u8*1]
	
	//css_allocator_fn alloc;
	//void *pw;
}

pub struct css_font_face_src {
	location:@lwc_string,
	/*
	 * Bit allocations:
	 *
	 *    76543210
	 *  1 _fffffll	format | location type
	 */
	bits:~[u8*1]
}

pub struct css_rule_page {
	base:css_rule ,

	selector:@css_selector ,
	style:@css_style 
}

pub struct css_rule_import {
	base:css_rule ,

	url:@lwc_string,
	media:u64,

	sheet:@css_stylesheet
}
pub struct css_rule_charset {
	base:css_rule ,

	encoding:@lwc_string	/* \todo use MIB enum? */
}
pub type  css_import_notification_fn =  ~extern fn(pw:~[u8],
		 parent:@css_stylesheet,  url:@lwc_string, media:u64) -> css_result;
pub type  css_url_resolution_fn =  ~extern fn(pw:~[u8],
		base:~str, rel:@lwc_string , abs:@lwc_string ) -> css_result;
pub type  css_color_resolution_fn =  ~extern fn(pw:~[u8],
		name:@lwc_string,  color:@css_color) -> css_result;
pub type  css_font_resolution_fn =  ~extern fn(pw:~[u8],
		name:@lwc_string,  system_font:@css_system_font) -> css_result;
pub struct css_stylesheet {
	//selectors:@css_selector_hash,	TODO REPLACE WITH BUILT IN HASH TABLE
		/* < Hashtable of selectors */

	rule_count:u32,			/**< Number of rules in sheet */
	rule_list:@css_rule ,			/**< List of rules in sheet */
	last_rule:@css_rule ,			/**< Last rule in list */

	disabled:bool,				/**< Whether this sheet is 
						 * disabled */

	url:~str,				/**< URL of this sheet */
	title:~str,			/**< Title of this sheet */

	level:css_language_level ,		/**< Language level of sheet */
	parser:@mut css_parser_node ,			/**< Core parser for sheet */
	parser_frontend:~[u8],			/**< Frontend parser */////////look for type
	propstrings:@ mut[@lwc_string ],		/**< Property strings, for parser */

	quirks_allowed:bool,			/**< Quirks permitted */
	quirks_used:bool,			/**< Quirks actually used */

	inline_style:bool,			/**< Is an inline style */

	size:size_t,				/**< Size, in bytes */

	 import:css_import_notification_fn,	/**< Import notification function */
	import_pw:~[u8],			/**< Private word *////////look for type

	 resolve:css_url_resolution_fn,		/**< URL resolution function */
	resolve_pw:~[u8],			/**< Private word *////////look for type

	 color:css_color_resolution_fn,		/**< Colour resolution function */
	color_pw:~[u8],				/**< Private word *////////look for type

	/** System font resolution function */
	 font:css_font_resolution_fn,		
	font_pw:~[u8],				/**< Private word *////////look for type


	// alloc:css_allocator_fn,			/**< Allocation function */
	//pw:~[u8],				/**< Private word */
  
	cached_style:@css_style ,		/**< Cache for style parsing */
  
	string_vector:@[@lwc_string],            /**< Bytecode string vector */
	string_vector_l:u32,              /**< The string vector allocated
					 * length in entries */
	string_vector_c:u32               /*< The number of string 
						 * vector entries used */ 
}
enum css_parser_event {
	CSS_PARSER_START_STYLESHEET,
	CSS_PARSER_END_STYLESHEET,
	CSS_PARSER_START_RULESET,
	CSS_PARSER_END_RULESET,
	CSS_PARSER_START_ATRULE,
	CSS_PARSER_END_ATRULE,
	CSS_PARSER_START_BLOCK,
	CSS_PARSER_END_BLOCK,
	CSS_PARSER_BLOCK_CONTENT,
	CSS_PARSER_DECLARATION
}
pub enum css_token_type { 
	CSS_TOKEN_IDENT, CSS_TOKEN_ATKEYWORD, CSS_TOKEN_HASH,
	CSS_TOKEN_FUNCTION, CSS_TOKEN_STRING, CSS_TOKEN_INVALID_STRING, 
	CSS_TOKEN_URI, CSS_TOKEN_UNICODE_RANGE, CSS_TOKEN_CHAR, 
	CSS_TOKEN_NUMBER, CSS_TOKEN_PERCENTAGE, CSS_TOKEN_DIMENSION,

	/* Those tokens that want strings interned appear above */
	CSS_TOKEN_LAST_INTERN,

 	CSS_TOKEN_CDO, CSS_TOKEN_CDC, CSS_TOKEN_S, CSS_TOKEN_COMMENT, 
	CSS_TOKEN_INCLUDES, CSS_TOKEN_DASHMATCH, CSS_TOKEN_PREFIXMATCH, 
	CSS_TOKEN_SUFFIXMATCH, CSS_TOKEN_SUBSTRINGMATCH, CSS_TOKEN_EOF 
}
struct CONTEXT{
		first:u8,		/**< First character read for token */
		 origBytes:size_t,	/**< Storage of current number of 
					 * bytes read, for rewinding */
		 lastWasStar:bool,	/**< Whether the previous character 
					 * was an asterisk */
		lastWasCR:bool ,		/**< Whether the previous character
					 * was CR */
		 bytesForURL:size_t,	/**< Input bytes read for "url(", for 
					 * rewinding */
		dataLenForURL:size_t ,	/**< Output length for "url(", for
					 * rewinding */
		 hexCount:int		/*< Counter for reading hex digits */
	} 
struct css_lexer
{
	 input:@parserutils_inputstream,	/**< Inputstream containing CSS */

	bytesReadForToken:size_t ,	/**< Total bytes read from the 
					 * inputstream for the current token */

	 token:css_token,		/**< The current token */

	escapeSeen:bool ,		/**< Whether an escape sequence has 
					 * been seen while processing the input
					 * for the current token */
	unescapedTokenData:~[u8],	/**< Buffer containing 
					 	 * unescaped token data 
						 * (used iff escapeSeen == true)
						 */

	state    : uint,	/**< Current state */
		     substate :uint,	/**< Current substate */

	 context:CONTEXT,			/**< Context for the current state */

	emit_comments:bool ,		/**< Whether to emit comment tokens */

	currentCol:u32,		/**< Current column in source */
	currentLine:u32		/*< Current line in source */

	//css_allocator_fn alloc;		/**< Memory (de)allocation function */
	//void *pw;			/**< Pointer to client-specific data */
}

struct DATA{
                data:~[u8],
                len:size_t,
        }
 struct css_token {
	token_type:css_token_type,

    data:DATA,

	idata:@lwc_string ,
	
	col:u32,
	line:u32,
}
pub type css_parser_event_handler =  ~extern fn( event_type:css_parser_event, 
		tokens:~[u8] , pw:@css_language) -> css_result;

enum css_parser_node
{
	SomeParserNode(@mut css_parser),
  	NoParserNode
}

pub struct css_parser
{
	stream:@parserutils_inputstream,	/**< The inputstream */
	lexer:@css_lexer,		/**< The lexer to use */

	mut quirks:bool,			/**< Whether to enable parsing quirks */

//#define STACK_CHUNK 32
    STACK_CHUNK: uint,
	states:DVec<u8>,	/**< Stack of states */

	tokens:~[u8],	/**< Vector of pending tokens */

	pushback:@css_token,	/**< Push back buffer */

	parseError:bool,		/**< A parse error has occurred */
	open_items:DVec<u8>,	/**< Stack of open brackets */

	match_char:u8,		/**< Close bracket type for parseAny */

	last_was_ws:bool,		/**< Last token was whitespace */

	mut event:css_parser_event_handler,	/**< Client's event handler */
	mut event_pw:@css_language		/*< Client data for event handler */

	//css_allocator_fn alloc;		/**< Memory (de)allocation function */
	//void *pw;			/**< Client-specific private data */
}


/*struct css_selector_hash {
	elements:hash_t,

	classes:hash_t,

	ids:hash_t,

	universal:hash_entry,

	hash_size:size_t

	//css_allocator_fn alloc;
	//void *pw;
}*/
//TO DO Stop for types

//To Do Should be moved to properties
// enum lwc_error
// {
// 	lwc_error_ok		= 0,	/**< No error. */
// 	lwc_error_oom		= 1,	*< Out of memory. 
// 	lwc_error_range		= 2	/*< Substring internment out of range. */
// }


	pub enum css_background_attachment_e {
		CSS_BACKGROUND_ATTACHMENT_INHERIT	= 0x0,
		CSS_BACKGROUND_ATTACHMENT_FIXED		= 0x1,
		CSS_BACKGROUND_ATTACHMENT_SCROLL	= 0x2
	}

	pub enum css_background_color_e {
		CSS_BACKGROUND_COLOR_INHERIT		= 0x0,
		CSS_BACKGROUND_COLOR_COLOR		= 0x1,
		CSS_BACKGROUND_COLOR_CURRENT_COLOR	= 0x2
	}

	pub enum css_background_image_e {
		CSS_BACKGROUND_IMAGE_INHERIT		= 0x0,
	/* Consult pointer in struct to determine which */
		CSS_BACKGROUND_IMAGE_NONE		= 0x1,
		CSS_BACKGROUND_IMAGE_IMAGE		= 0x2
	}

	pub enum css_background_position_e {
		CSS_BACKGROUND_POSITION_INHERIT		= 0x0,
		CSS_BACKGROUND_POSITION_SET		= 0x1
	}

	pub enum css_background_repeat_e {
		CSS_BACKGROUND_REPEAT_INHERIT		= 0x0,
		CSS_BACKGROUND_REPEAT_REPEAT_X		= 0x1,
		CSS_BACKGROUND_REPEAT_REPEAT_Y		= 0x2,
		CSS_BACKGROUND_REPEAT_REPEAT		= 0x3,
		CSS_BACKGROUND_REPEAT_NO_REPEAT		= 0x4
	}

	pub enum css_border_collapse_e {
		CSS_BORDER_COLLAPSE_INHERIT		= 0x0,
		CSS_BORDER_COLLAPSE_SEPARATE		= 0x1,
		CSS_BORDER_COLLAPSE_COLLAPSE		= 0x2
	}

	pub enum css_border_spacing_e {
		CSS_BORDER_SPACING_INHERIT		= 0x0,
		CSS_BORDER_SPACING_SET			= 0x1
	}

	pub enum css_border_color_e {
		CSS_BORDER_COLOR_INHERIT		= 0x0,
		CSS_BORDER_COLOR_COLOR			= 0x1,
		CSS_BORDER_COLOR_CURRENT_COLOR		= 0x2
	}

	pub enum css_border_style_e {
		CSS_BORDER_STYLE_INHERIT		= 0x0,
		CSS_BORDER_STYLE_NONE			= 0x1,
		CSS_BORDER_STYLE_HIDDEN			= 0x2,
		CSS_BORDER_STYLE_DOTTED			= 0x3,
		CSS_BORDER_STYLE_DASHED			= 0x4,
		CSS_BORDER_STYLE_SOLID			= 0x5,
		CSS_BORDER_STYLE_DOUBLE			= 0x6,
		CSS_BORDER_STYLE_GROOVE			= 0x7,
		CSS_BORDER_STYLE_RIDGE			= 0x8,
		CSS_BORDER_STYLE_INSET			= 0x9,
		CSS_BORDER_STYLE_OUTSET			= 0xa
	}

	pub enum css_border_width_e {
		CSS_BORDER_WIDTH_INHERIT		= 0x0,
		CSS_BORDER_WIDTH_THIN			= 0x1,
		CSS_BORDER_WIDTH_MEDIUM			= 0x2,
		CSS_BORDER_WIDTH_THICK			= 0x3,
		CSS_BORDER_WIDTH_WIDTH			= 0x4
	}

	pub enum css_bottom_e {
		CSS_BOTTOM_INHERIT			= 0x0,
		CSS_BOTTOM_SET				= 0x1,
		CSS_BOTTOM_AUTO				= 0x2
	}

	pub enum css_break_after_e {
		CSS_BREAK_AFTER_INHERIT			= 0x0,
		CSS_BREAK_AFTER_AUTO			= 0x1,
		CSS_BREAK_AFTER_AVOID			= 0x2,
		CSS_BREAK_AFTER_ALWAYS			= 0x3,
		CSS_BREAK_AFTER_LEFT			= 0x4,
		CSS_BREAK_AFTER_RIGHT			= 0x5,
		CSS_BREAK_AFTER_PAGE			= 0x6,
		CSS_BREAK_AFTER_COLUMN			= 0x7,
		CSS_BREAK_AFTER_AVOID_PAGE		= 0x8,
		CSS_BREAK_AFTER_AVOID_COLUMN		= 0x9
	}	

	pub enum css_break_before_e {
		CSS_BREAK_BEFORE_INHERIT		= 0x0,
		CSS_BREAK_BEFORE_AUTO			= 0x1,
		CSS_BREAK_BEFORE_AVOID			= 0x2,
		CSS_BREAK_BEFORE_ALWAYS			= 0x3,
		CSS_BREAK_BEFORE_LEFT			= 0x4,
		CSS_BREAK_BEFORE_RIGHT			= 0x5,
		CSS_BREAK_BEFORE_PAGE			= 0x6,
		CSS_BREAK_BEFORE_COLUMN			= 0x7,
		CSS_BREAK_BEFORE_AVOID_PAGE		= 0x8,
		CSS_BREAK_BEFORE_AVOID_COLUMN		= 0x9
	}

	pub enum css_break_inside_e {
		CSS_BREAK_INSIDE_INHERIT		= 0x0,
		CSS_BREAK_INSIDE_AUTO			= 0x1,
		CSS_BREAK_INSIDE_AVOID			= 0x2,
		CSS_BREAK_INSIDE_AVOID_PAGE		= 0x8,
		CSS_BREAK_INSIDE_AVOID_COLUMN		= 0x9
	}

	pub enum css_caption_side_e {
		CSS_CAPTION_SIDE_INHERIT		= 0x0,
		CSS_CAPTION_SIDE_TOP			= 0x1,
		CSS_CAPTION_SIDE_BOTTOM			= 0x2
	}

	pub enum css_clear_e {
		CSS_CLEAR_INHERIT			= 0x0,
		CSS_CLEAR_NONE				= 0x1,
		CSS_CLEAR_LEFT				= 0x2,
		CSS_CLEAR_RIGHT				= 0x3,
		CSS_CLEAR_BOTH				= 0x4
	}

	pub enum css_clip_e {
		CSS_CLIP_INHERIT			= 0x0,
		CSS_CLIP_AUTO				= 0x1,
		CSS_CLIP_RECT				= 0x2
	}

	pub enum css_color_e {
		CSS_COLOR_INHERIT			= 0x0,
		CSS_COLOR_COLOR				= 0x1
	}

	pub enum css_column_count_e {
		CSS_COLUMN_COUNT_INHERIT		= 0x0,
		CSS_COLUMN_COUNT_AUTO			= 0x1,
		CSS_COLUMN_COUNT_SET			= 0x2
	}

	pub enum css_column_fill_e {
		CSS_COLUMN_FILL_INHERIT			= 0x0,
		CSS_COLUMN_FILL_BALANCE			= 0x1,
		CSS_COLUMN_FILL_AUTO			= 0x2
	}

	pub enum css_column_gap_e {
		CSS_COLUMN_GAP_INHERIT			= 0x0,
		CSS_COLUMN_GAP_NORMAL			= 0x1,
		CSS_COLUMN_GAP_SET			= 0x2
	}

	pub enum css_column_rule_color_e {
		CSS_COLUMN_RULE_COLOR_INHERIT		= 0x0,
		CSS_COLUMN_RULE_COLOR_COLOR		= 0x1,
		CSS_COLUMN_RULE_COLOR_CURRENT_COLOR	= 0x2
	}

	pub enum css_column_rule_style_e {
		CSS_COLUMN_RULE_STYLE_INHERIT		= 0x0,
		CSS_COLUMN_RULE_STYLE_NONE		= 0x1,
		CSS_COLUMN_RULE_STYLE_DOTTED		= 0x3,
		CSS_COLUMN_RULE_STYLE_DASHED		= 0x4,
		CSS_COLUMN_RULE_STYLE_SOLID		= 0x5,
		CSS_COLUMN_RULE_STYLE_DOUBLE		= 0x6,
		CSS_COLUMN_RULE_STYLE_GROOVE		= 0x7,
		CSS_COLUMN_RULE_STYLE_RIDGE		= 0x8,
		CSS_COLUMN_RULE_STYLE_INSET		= 0x9,
		CSS_COLUMN_RULE_STYLE_OUTSET		= 0xa
	}

	pub enum css_column_rule_width_e {
		CSS_COLUMN_RULE_WIDTH_INHERIT		= 0x0,
		CSS_COLUMN_RULE_WIDTH_THIN		= 0x1,
		CSS_COLUMN_RULE_WIDTH_MEDIUM		= 0x2,
		CSS_COLUMN_RULE_WIDTH_THICK		= 0x3,
		CSS_COLUMN_RULE_WIDTH_WIDTH		= 0x4
	}

	pub enum css_column_span_e {
		CSS_COLUMN_SPAN_INHERIT			= 0x0,
		CSS_COLUMN_SPAN_NONE			= 0x1,
		CSS_COLUMN_SPAN_ALL			= 0x2
	}

	pub enum css_column_width_e {
		CSS_COLUMN_WIDTH_INHERIT		= 0x0,
		CSS_COLUMN_WIDTH_AUTO			= 0x1,
		CSS_COLUMN_WIDTH_SET			= 0x2
	}

	pub enum css_content_e {
		CSS_CONTENT_INHERIT			= 0x0,
		CSS_CONTENT_NONE			= 0x1,
		CSS_CONTENT_NORMAL			= 0x2,
		CSS_CONTENT_SET				= 0x3
	}

	pub enum css_counter_increment_e {
		CSS_COUNTER_INCREMENT_INHERIT		= 0x0,
	/* Consult pointer in struct to determine which */
		CSS_COUNTER_INCREMENT_NAMED		= 0x1,
		CSS_COUNTER_INCREMENT_NONE		= 0x2
	}

	pub enum css_counter_reset_e {
		CSS_COUNTER_RESET_INHERIT		= 0x0,
	/* Consult pointer in struct to determine which */
		CSS_COUNTER_RESET_NAMED			= 0x1,
		CSS_COUNTER_RESET_NONE			= 0x2
	}

	pub enum css_cursor_e {
		CSS_CURSOR_INHERIT			= 0x00,
	/* URLs exist if pointer is non-NULL */
		CSS_CURSOR_AUTO				= 0x01,
		CSS_CURSOR_CROSSHAIR			= 0x02,
		CSS_CURSOR_DEFAULT			= 0x03,
		CSS_CURSOR_POINTER			= 0x04,
		CSS_CURSOR_MOVE				= 0x05,
		CSS_CURSOR_E_RESIZE			= 0x06,
		CSS_CURSOR_NE_RESIZE			= 0x07,
		CSS_CURSOR_NW_RESIZE			= 0x08,
		CSS_CURSOR_N_RESIZE			= 0x09,
		CSS_CURSOR_SE_RESIZE			= 0x0a,
		CSS_CURSOR_SW_RESIZE			= 0x0b,
		CSS_CURSOR_S_RESIZE			= 0x0c,
		CSS_CURSOR_W_RESIZE			= 0x0d,
		CSS_CURSOR_TEXT				= 0x0e,
		CSS_CURSOR_WAIT				= 0x0f,
		CSS_CURSOR_HELP				= 0x10,
		CSS_CURSOR_PROGRESS			= 0x11
	}

	pub enum css_direction_e {
		CSS_DIRECTION_INHERIT			= 0x0,
		CSS_DIRECTION_LTR			= 0x1,
		CSS_DIRECTION_RTL			= 0x2
	}

	pub enum css_display_e {
		CSS_DISPLAY_INHERIT			= 0x00,
		CSS_DISPLAY_INLINE			= 0x01,
		CSS_DISPLAY_BLOCK			= 0x02,
		CSS_DISPLAY_LIST_ITEM			= 0x03,
		CSS_DISPLAY_RUN_IN			= 0x04,
		CSS_DISPLAY_INLINE_BLOCK		= 0x05,
		CSS_DISPLAY_TABLE			= 0x06,
		CSS_DISPLAY_INLINE_TABLE		= 0x07,
		CSS_DISPLAY_TABLE_ROW_GROUP		= 0x08,
		CSS_DISPLAY_TABLE_HEADER_GROUP		= 0x09,
		CSS_DISPLAY_TABLE_FOOTER_GROUP		= 0x0a,
		CSS_DISPLAY_TABLE_ROW			= 0x0b,
		CSS_DISPLAY_TABLE_COLUMN_GROUP		= 0x0c,
		CSS_DISPLAY_TABLE_COLUMN		= 0x0d,
		CSS_DISPLAY_TABLE_CELL			= 0x0e,
		CSS_DISPLAY_TABLE_CAPTION		= 0x0f,
		CSS_DISPLAY_NONE			= 0x10
	}

	pub enum css_empty_cells_e {
		CSS_EMPTY_CELLS_INHERIT			= 0x0,
		CSS_EMPTY_CELLS_SHOW			= 0x1,
		CSS_EMPTY_CELLS_HIDE			= 0x2
	}

	pub enum css_float_e {
		CSS_FLOAT_INHERIT			= 0x0,
		CSS_FLOAT_LEFT				= 0x1,
		CSS_FLOAT_RIGHT				= 0x2,
		CSS_FLOAT_NONE				= 0x3
	}

	pub enum css_font_family_e {
		CSS_FONT_FAMILY_INHERIT			= 0x0,
		/* Named fonts exist if pointer is non-NULL */
		CSS_FONT_FAMILY_SERIF			= 0x1,
		CSS_FONT_FAMILY_SANS_SERIF		= 0x2,
		CSS_FONT_FAMILY_CURSIVE			= 0x3,
		CSS_FONT_FAMILY_FANTASY			= 0x4,
		CSS_FONT_FAMILY_MONOSPACE		= 0x5
	}

	pub enum css_font_size_e {
		CSS_FONT_SIZE_INHERIT			= 0x0,
		CSS_FONT_SIZE_XX_SMALL			= 0x1,
		CSS_FONT_SIZE_X_SMALL			= 0x2,
		CSS_FONT_SIZE_SMALL			= 0x3,
		CSS_FONT_SIZE_MEDIUM			= 0x4,
		CSS_FONT_SIZE_LARGE			= 0x5,
		CSS_FONT_SIZE_X_LARGE			= 0x6,
		CSS_FONT_SIZE_XX_LARGE			= 0x7,
		CSS_FONT_SIZE_LARGER			= 0x8,
		CSS_FONT_SIZE_SMALLER			= 0x9,
		CSS_FONT_SIZE_DIMENSION			= 0xa
	}

	pub enum css_font_style_e {
		CSS_FONT_STYLE_INHERIT			= 0x0,
		CSS_FONT_STYLE_NORMAL			= 0x1,
		CSS_FONT_STYLE_ITALIC			= 0x2,
		CSS_FONT_STYLE_OBLIQUE			= 0x3
	}

	pub enum css_font_variant_e {
		CSS_FONT_VARIANT_INHERIT		= 0x0,
		CSS_FONT_VARIANT_NORMAL			= 0x1,
		CSS_FONT_VARIANT_SMALL_CAPS		= 0x2
	}

	pub enum css_font_weight_e {
		CSS_FONT_WEIGHT_INHERIT			= 0x0,
		CSS_FONT_WEIGHT_NORMAL			= 0x1,
		CSS_FONT_WEIGHT_BOLD			= 0x2,
		CSS_FONT_WEIGHT_BOLDER			= 0x3,
		CSS_FONT_WEIGHT_LIGHTER			= 0x4,
		CSS_FONT_WEIGHT_100			= 0x5,
		CSS_FONT_WEIGHT_200			= 0x6,
		CSS_FONT_WEIGHT_300			= 0x7,
		CSS_FONT_WEIGHT_400			= 0x8,
		CSS_FONT_WEIGHT_500			= 0x9,
		CSS_FONT_WEIGHT_600			= 0xa,
		CSS_FONT_WEIGHT_700			= 0xb,
		CSS_FONT_WEIGHT_800			= 0xc,
		CSS_FONT_WEIGHT_900			= 0xd
	}

	pub enum css_height_e {
		CSS_HEIGHT_INHERIT			= 0x0,
		CSS_HEIGHT_SET				= 0x1,
		CSS_HEIGHT_AUTO				= 0x2
	}

	pub enum css_left_e {
		CSS_LEFT_INHERIT			= 0x0,
		CSS_LEFT_SET				= 0x1,
		CSS_LEFT_AUTO				= 0x2
	}

	pub enum css_letter_spacing_e {
		CSS_LETTER_SPACING_INHERIT		= 0x0,
		CSS_LETTER_SPACING_SET			= 0x1,
		CSS_LETTER_SPACING_NORMAL		= 0x2
	}

	pub enum css_line_height_e {
		CSS_LINE_HEIGHT_INHERIT			= 0x0,
		CSS_LINE_HEIGHT_NUMBER			= 0x1,
		CSS_LINE_HEIGHT_DIMENSION		= 0x2,
		CSS_LINE_HEIGHT_NORMAL			= 0x3
	}

	pub enum css_list_style_image_e {
		CSS_LIST_STYLE_IMAGE_INHERIT		= 0x0,
		/* Consult pointer in struct to determine which */
		CSS_LIST_STYLE_IMAGE_URI		= 0x1,
		CSS_LIST_STYLE_IMAGE_NONE		= 0x2
	}

	pub enum css_list_style_position_e {
		CSS_LIST_STYLE_POSITION_INHERIT		= 0x0,
		CSS_LIST_STYLE_POSITION_INSIDE		= 0x1,
		CSS_LIST_STYLE_POSITION_OUTSIDE		= 0x2
	}

	pub enum css_list_style_type_e {
		CSS_LIST_STYLE_TYPE_INHERIT		= 0x0,
		CSS_LIST_STYLE_TYPE_DISC		= 0x1,
		CSS_LIST_STYLE_TYPE_CIRCLE		= 0x2,
		CSS_LIST_STYLE_TYPE_SQUARE		= 0x3,
		CSS_LIST_STYLE_TYPE_DECIMAL		= 0x4,
		CSS_LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO= 0x5,
		CSS_LIST_STYLE_TYPE_LOWER_ROMAN		= 0x6,
		CSS_LIST_STYLE_TYPE_UPPER_ROMAN		= 0x7,
		CSS_LIST_STYLE_TYPE_LOWER_GREEK		= 0x8,
		CSS_LIST_STYLE_TYPE_LOWER_LATIN		= 0x9,
		CSS_LIST_STYLE_TYPE_UPPER_LATIN		= 0xa,
		CSS_LIST_STYLE_TYPE_ARMENIAN		= 0xb,
		CSS_LIST_STYLE_TYPE_GEORGIAN		= 0xc,
		CSS_LIST_STYLE_TYPE_LOWER_ALPHA		= 0xd,
		CSS_LIST_STYLE_TYPE_UPPER_ALPHA		= 0xe,
		CSS_LIST_STYLE_TYPE_NONE		= 0xf
	}

	pub enum css_margin_e {
		CSS_MARGIN_INHERIT			= 0x0,
		CSS_MARGIN_SET				= 0x1,
		CSS_MARGIN_AUTO				= 0x2
	}

	pub enum css_max_height_e {
		CSS_MAX_HEIGHT_INHERIT			= 0x0,
		CSS_MAX_HEIGHT_SET			= 0x1,
		CSS_MAX_HEIGHT_NONE			= 0x2
	}

	pub enum css_max_width_e {
		CSS_MAX_WIDTH_INHERIT			= 0x0,
		CSS_MAX_WIDTH_SET			= 0x1,
		CSS_MAX_WIDTH_NONE			= 0x2
	}

	pub enum css_min_height_e {
		CSS_MIN_HEIGHT_INHERIT			= 0x0,
		CSS_MIN_HEIGHT_SET			= 0x1
	}

	pub enum css_min_width_e {
		CSS_MIN_WIDTH_INHERIT			= 0x0,
		CSS_MIN_WIDTH_SET			= 0x1
	}

	pub enum css_opacity_e {
		CSS_OPACITY_INHERIT			= 0x0,
		CSS_OPACITY_SET				= 0x1
	}

	pub enum css_outline_color_e {
		CSS_OUTLINE_COLOR_INHERIT		= 0x0,
		CSS_OUTLINE_COLOR_COLOR			= 0x1,
		CSS_OUTLINE_COLOR_CURRENT_COLOR		= 0x2,
		CSS_OUTLINE_COLOR_INVERT		= 0x3
	}

	pub enum css_outline_style_e {
		CSS_OUTLINE_STYLE_INHERIT		= 0x0,
		CSS_OUTLINE_STYLE_NONE			= 0x1,
		CSS_OUTLINE_STYLE_DOTTED		= 0x3,
		CSS_OUTLINE_STYLE_DASHED		= 0x4,
		CSS_OUTLINE_STYLE_SOLID			= 0x5,
		CSS_OUTLINE_STYLE_DOUBLE		= 0x6,
		CSS_OUTLINE_STYLE_GROOVE		= 0x7,
		CSS_OUTLINE_STYLE_RIDGE			= 0x8,
		CSS_OUTLINE_STYLE_INSET			= 0x9,
		CSS_OUTLINE_STYLE_OUTSET		= 0xa
	}

	pub enum css_outline_width_e {
		CSS_OUTLINE_WIDTH_INHERIT		= 0x0,
		CSS_OUTLINE_WIDTH_THIN			= 0x1,
		CSS_OUTLINE_WIDTH_MEDIUM		= 0x2,
		CSS_OUTLINE_WIDTH_THICK			= 0x3,
		CSS_OUTLINE_WIDTH_WIDTH			= 0x4
	}

	pub enum css_overflow_e {
		CSS_OVERFLOW_INHERIT			= 0x0,
		CSS_OVERFLOW_VISIBLE			= 0x1,
		CSS_OVERFLOW_HIDDEN			= 0x2,
		CSS_OVERFLOW_SCROLL			= 0x3,
		CSS_OVERFLOW_AUTO			= 0x4
	}

	pub enum css_orphans_e {
		CSS_ORPHANS_INHERIT			= 0x0,
		CSS_ORPHANS_SET				= 0x1
	}

	pub enum css_padding_e {
		CSS_PADDING_INHERIT			= 0x0,
		CSS_PADDING_SET				= 0x1
	}

	pub enum css_page_break_after_e {
		CSS_PAGE_BREAK_AFTER_INHERIT		= 0x0,
		CSS_PAGE_BREAK_AFTER_AUTO		= 0x1,
		CSS_PAGE_BREAK_AFTER_AVOID		= 0x2,
		CSS_PAGE_BREAK_AFTER_ALWAYS		= 0x3,
		CSS_PAGE_BREAK_AFTER_LEFT		= 0x4,
		CSS_PAGE_BREAK_AFTER_RIGHT		= 0x5
	}	

	pub enum css_page_break_before_e {
		CSS_PAGE_BREAK_BEFORE_INHERIT		= 0x0,
		CSS_PAGE_BREAK_BEFORE_AUTO		= 0x1,
		CSS_PAGE_BREAK_BEFORE_AVOID		= 0x2,
		CSS_PAGE_BREAK_BEFORE_ALWAYS		= 0x3,
		CSS_PAGE_BREAK_BEFORE_LEFT		= 0x4,
		CSS_PAGE_BREAK_BEFORE_RIGHT		= 0x5
	}

	pub enum css_page_break_inside_e {
		CSS_PAGE_BREAK_INSIDE_INHERIT		= 0x0,
		CSS_PAGE_BREAK_INSIDE_AUTO		= 0x1,
		CSS_PAGE_BREAK_INSIDE_AVOID		= 0x2
	}

	pub enum css_position_e {
		CSS_POSITION_INHERIT			= 0x0,
		CSS_POSITION_STATIC			= 0x1,
		CSS_POSITION_RELATIVE			= 0x2,
		CSS_POSITION_ABSOLUTE			= 0x3,
		CSS_POSITION_FIXED			= 0x4
	}

	pub enum css_quotes_e {
		CSS_QUOTES_INHERIT			= 0x0,
	/* Consult pointer in struct to determine which */
		CSS_QUOTES_STRING			= 0x1,
		CSS_QUOTES_NONE				= 0x2
	}

	pub enum css_right_e {
		CSS_RIGHT_INHERIT			= 0x0,
		CSS_RIGHT_SET				= 0x1,
		CSS_RIGHT_AUTO				= 0x2
	}

	pub enum css_table_layout_e {
		CSS_TABLE_LAYOUT_INHERIT		= 0x0,
		CSS_TABLE_LAYOUT_AUTO			= 0x1,
		CSS_TABLE_LAYOUT_FIXED			= 0x2
	}

	pub enum css_text_align_e {
		CSS_TEXT_ALIGN_INHERIT			= 0x0,
		CSS_TEXT_ALIGN_INHERIT_IF_NON_MAGIC	= 0x1,
		CSS_TEXT_ALIGN_LEFT			= 0x2,
		CSS_TEXT_ALIGN_RIGHT			= 0x3,
		CSS_TEXT_ALIGN_CENTER			= 0x4,
		CSS_TEXT_ALIGN_JUSTIFY			= 0x5,
		CSS_TEXT_ALIGN_DEFAULT			= 0x6,
		CSS_TEXT_ALIGN_LIBCSS_LEFT		= 0x7,
		CSS_TEXT_ALIGN_LIBCSS_CENTER		= 0x8,
		CSS_TEXT_ALIGN_LIBCSS_RIGHT		= 0x9
	}

	pub enum css_text_decoration_e {
		CSS_TEXT_DECORATION_INHERIT		= 0x00,
		CSS_TEXT_DECORATION_NONE		= 0x10,
		CSS_TEXT_DECORATION_BLINK		= (1<<3),
		CSS_TEXT_DECORATION_LINE_THROUGH	= (1<<2),
		CSS_TEXT_DECORATION_OVERLINE		= (1<<1),
		CSS_TEXT_DECORATION_UNDERLINE		= (1<<0)
	}

	pub enum css_text_indent_e {
		CSS_TEXT_INDENT_INHERIT			= 0x0,
		CSS_TEXT_INDENT_SET			= 0x1
	}

	pub enum css_text_transform_e {
		CSS_TEXT_TRANSFORM_INHERIT		= 0x0,
		CSS_TEXT_TRANSFORM_CAPITALIZE		= 0x1,
		CSS_TEXT_TRANSFORM_UPPERCASE		= 0x2,
		CSS_TEXT_TRANSFORM_LOWERCASE		= 0x3,
		CSS_TEXT_TRANSFORM_NONE			= 0x4
	}

	pub enum css_top_e {
		CSS_TOP_INHERIT				= 0x0,
		CSS_TOP_SET				= 0x1,
		CSS_TOP_AUTO				= 0x2
	}

	pub enum css_unicode_bidi_e {
		CSS_UNICODE_BIDI_INHERIT		= 0x0,
		CSS_UNICODE_BIDI_NORMAL			= 0x1,
		CSS_UNICODE_BIDI_EMBED			= 0x2,
		CSS_UNICODE_BIDI_BIDI_OVERRIDE		= 0x3
	}

	pub enum css_vertical_align_e {
		CSS_VERTICAL_ALIGN_INHERIT		= 0x0,
		CSS_VERTICAL_ALIGN_BASELINE		= 0x1,
		CSS_VERTICAL_ALIGN_SUB			= 0x2,
		CSS_VERTICAL_ALIGN_SUPER		= 0x3,
		CSS_VERTICAL_ALIGN_TOP			= 0x4,
		CSS_VERTICAL_ALIGN_TEXT_TOP		= 0x5,
		CSS_VERTICAL_ALIGN_MIDDLE		= 0x6,
		CSS_VERTICAL_ALIGN_BOTTOM		= 0x7,
		CSS_VERTICAL_ALIGN_TEXT_BOTTOM		= 0x8,
		CSS_VERTICAL_ALIGN_SET			= 0x9
	}

	pub enum css_visibility_e {
		CSS_VISIBILITY_INHERIT			= 0x0,
		CSS_VISIBILITY_VISIBLE			= 0x1,
		CSS_VISIBILITY_HIDDEN			= 0x2,
		CSS_VISIBILITY_COLLAPSE			= 0x3
	}

	pub enum css_white_space_e {
		CSS_WHITE_SPACE_INHERIT			= 0x0,
		CSS_WHITE_SPACE_NORMAL			= 0x1,
		CSS_WHITE_SPACE_PRE			= 0x2,
		CSS_WHITE_SPACE_NOWRAP			= 0x3,
		CSS_WHITE_SPACE_PRE_WRAP		= 0x4,
		CSS_WHITE_SPACE_PRE_LINE		= 0x5
	}

	pub enum css_widows_e {
		CSS_WIDOWS_INHERIT			= 0x0,
		CSS_WIDOWS_SET				= 0x1
	}	
	
	pub enum css_width_e {
		CSS_WIDTH_INHERIT			= 0x0,
		CSS_WIDTH_SET				= 0x1,
		CSS_WIDTH_AUTO				= 0x2
	}

	pub enum css_word_spacing_e {
		CSS_WORD_SPACING_INHERIT		= 0x0,
		CSS_WORD_SPACING_SET			= 0x1,
		CSS_WORD_SPACING_NORMAL			= 0x2
	}

	pub enum css_z_index_e {
		CSS_Z_INDEX_INHERIT			= 0x0,
		CSS_Z_INDEX_SET				= 0x1,
		CSS_Z_INDEX_AUTO			= 0x2
	}

//Stop for properties

//To Do Should move to errors ---- Start
pub enum css_result {
		//CSS_OK  ,
		CSS_GENERAL_OK,
		CSS_LANGUAGE_CREATED(@css_language),
		CSS_PROPSTRINGS_OK(~[@lwc_string]),
		CSS_NOMEM,
		CSS_BADPARM,
		CSS_INVALID,
		CSS_FILENOTFOUND,
		CSS_NEEDDATA,
		CSS_BADCHARSET,
		CSS_EOF,
		CSS_IMPORTS_PENDING,
		CSS_PROPERTY_NOT_SET,
		//CSS_LWC_INTERN_STRING_OK([@lwc_string])
		
	}
enum css_parser_opttype {
	CSS_PARSER_QUIRKS,
	CSS_PARSER_EVENT_HANDLER
}



//Stop for error

/*
 * Callback to resolve an URL
 *
 * \param pw    Client data
 * \param dict  String internment context
 * \param base  Base URI (absolute)
 * \param rel   URL to resolve, either absolute or relative to base
 * \param abs   Pointer to location to receive result
 * \return CSS_OK on success, appropriate error otherwise.
 */
//fn css_url_resolution_fn() -> @fn (pw:~[u8],base:~str, rel:lwc_string, abs: ~str) -> css_result{ }

/*
 * Callback to be notified of the need for an imported stylesheet
 *
 * \param pw      Client data
 * \param parent  Stylesheet requesting the import
 * \param url     URL of the imported sheet
 * \param media   Applicable media for the imported sheet
 * \return CSS_OK on success, appropriate error otherwise
 *
 * \note This function will be invoked for notification purposes
 *       only. The client may use this to trigger a parallel fetch
 *       of the imported stylesheet. The imported sheet must be
 *       registered with its parent using the post-parse import
 *       registration API.
 */

//fn css_import_notification_fn() -> @fn (pw:~[u8], parent: &css_stylesheet, url:~lwc_string, media:u64) -> css_result{}

/*
 * Callback use to resolve system colour names to RGB values
 *
 * \param pw     Client data
 * \param name   System colour name
 * \param color  Pointer to location to receive color value
 * \return CSS_OK       on success,
 *         CSS_INVALID  if the name is unknown.
 */
//fn css_color_resolution_fn() ->  @fn (pw:~[u8], name:~lwc_string, color:&css_color) -> css_result{}

pub struct size_t{                  
	size:css_fixed,           
	unit:css_unit,
}

pub struct line_height_t{                  
	size:css_fixed,           
	unit:css_unit,
}
/** System font callback result data. */
pub struct css_system_font {
	style: css_font_style_e,
	variant: css_font_variant_e,
	weight:  css_font_weight_e,
	size:size_t,
	line_height:line_height_t,
	/* Note: must be a single family name only */
	family: @lwc_string
}

/**
 * Callback use to resolve system font names to font values
 *
 * \param pw           Client data
 * \param name         System font identifier
 * \param system_font  Pointer to system font descriptor to be filled
 * \return CSS_OK       on success,
 *         CSS_INVALID  if the name is unknown.
 */
//fn css_font_resolution_fn()-> @fn(pw:~[u8], name:~lwc_string, system_font:&css_system_font) -> css_result {}

pub enum css_stylesheet_params_version {
	CSS_STYLESHEET_PARAMS_VERSION_1 = 1
}



/**
 * Parameter block for css_stylesheet_create()
 */
	struct css_stylesheet_params {
	/** ABI version of this structure */
		params_version:u32 ,

	/** The language level of the stylesheet */
		level:css_language_level,

	/** The charset of the stylesheet data, or NULL to detect */
		charset:~str,

	/** URL of stylesheet */
		url:~str,

	/** Title of stylesheet */
		title:~str,

	/** Permit quirky parsing of stylesheet */
		mut allow_quirks:bool,

	/** This stylesheet is an inline style */
		mut inline_style:bool,

	/** URL resolution function */
		mut resolve : @fn (pw:~[u8],base:~str, rel:@lwc_string, abs: ~str) -> css_result,

	/** Client private data for resolve */
		mut resolve_pw:~[u8],

	/** Import notification function */
	//	mut import: @fn (pw:~[u8], parent:&css_stylesheet, url:~lwc_string, media:u64) -> css_result,

	/** Client private data for import */
		mut import_pw:~[u8],

	/** Colour resolution function */
		mut color: @fn (pw:~[u8], name:@lwc_string, color:&css_color) -> css_result,

	/** Client private data for color */
		mut color_pw:~[u8],

	/** Font resolution function */
		mut font: @fn(pw:~[u8], name:@lwc_string, system_font:&css_system_font) -> css_result ,

	/** Client private data for font */
		mut font_pw: ~[u8]
	}


pub enum op_azimuth {
	AZIMUTH_ANGLE			= 0x0080,

	AZIMUTH_LEFTWARDS		= 0x0040,
	AZIMUTH_RIGHTWARDS		= 0x0041,

	AZIMUTH_BEHIND			= (1<<5),
	AZIMUTH_LEFT_SIDE		= 0x0000,
	AZIMUTH_FAR_LEFT		= 0x0001,
	AZIMUTH_LEFT			= 0x0002,
	AZIMUTH_CENTER_LEFT		= 0x0003,
	AZIMUTH_CENTER			= 0x0004,
	AZIMUTH_CENTER_RIGHT		= 0x0005,
	AZIMUTH_RIGHT			= 0x0006,
	AZIMUTH_FAR_RIGHT		= 0x0007,
	AZIMUTH_RIGHT_SIDE		= 0x0008
} 

pub enum op_background_attachment {
	BACKGROUND_ATTACHMENT_FIXED	= 0x0000,
	BACKGROUND_ATTACHMENT_SCROLL	= 0x0001
} 

pub enum op_background_color {
	BACKGROUND_COLOR_TRANSPARENT	= 0x0000,
	BACKGROUND_COLOR_CURRENT_COLOR	= 0x0001,
	BACKGROUND_COLOR_SET		= 0x0080
} 

pub enum op_background_image {
	BACKGROUND_IMAGE_URI		= 0x0080,
	BACKGROUND_IMAGE_NONE		= 0x0000
} 

pub enum op_background_position {
	BACKGROUND_POSITION_HORZ_SET	= 0x0080,
	BACKGROUND_POSITION_HORZ_CENTER	= 0x0000,
	BACKGROUND_POSITION_HORZ_RIGHT	= 0x0010,
	BACKGROUND_POSITION_HORZ_LEFT	= 0x0020,

	BACKGROUND_POSITION_VERT_SET	= 0x0008,
	//BACKGROUND_POSITION_VERT_CENTER	= 0x0000,
	BACKGROUND_POSITION_VERT_BOTTOM	= 0x0001,
	BACKGROUND_POSITION_VERT_TOP	= 0x0002
} 
const   BACKGROUND_POSITION_VERT_CENTER :  op_background_position = BACKGROUND_POSITION_HORZ_CENTER;	

pub enum op_background_repeat {
	BACKGROUND_REPEAT_NO_REPEAT	= 0x0000,
	BACKGROUND_REPEAT_REPEAT_X	= 0x0001,
	BACKGROUND_REPEAT_REPEAT_Y	= 0x0002,
	BACKGROUND_REPEAT_REPEAT	= 0x0003
} 

pub enum op_border_collapse {
	BORDER_COLLAPSE_SEPARATE	= 0x0000,
	BORDER_COLLAPSE_COLLAPSE	= 0x0001
} 

pub enum op_border_spacing {
	BORDER_SPACING_SET		= 0x0080
} 

pub enum op_border_color {
	BORDER_COLOR_TRANSPARENT	= 0x0000,
	BORDER_COLOR_CURRENT_COLOR	= 0x0001,
	BORDER_COLOR_SET		= 0x0080
} 

pub enum op_border_style {
	BORDER_STYLE_NONE		= 0x0000,
	BORDER_STYLE_HIDDEN		= 0x0001,
	BORDER_STYLE_DOTTED		= 0x0002,
	BORDER_STYLE_DASHED		= 0x0003,
	BORDER_STYLE_SOLID		= 0x0004,
	BORDER_STYLE_DOUBLE		= 0x0005,
	BORDER_STYLE_GROOVE		= 0x0006,
	BORDER_STYLE_RIDGE		= 0x0007,
	BORDER_STYLE_INSET		= 0x0008,
	BORDER_STYLE_OUTSET		= 0x0009
} 

pub enum op_border_width {
	BORDER_WIDTH_SET		= 0x0080,
	BORDER_WIDTH_THIN		= 0x0000,
	BORDER_WIDTH_MEDIUM		= 0x0001,
	BORDER_WIDTH_THICK		= 0x0002
} 

pub enum op_bottom {
	BOTTOM_SET			= 0x0080,
	BOTTOM_AUTO			= 0x0000
} 

pub enum op_break_after {
	BREAK_AFTER_AUTO		= 0x0000,
	BREAK_AFTER_ALWAYS		= 0x0001,
	BREAK_AFTER_AVOID		= 0x0002,
	BREAK_AFTER_LEFT		= 0x0003,
	BREAK_AFTER_RIGHT		= 0x0004,
	BREAK_AFTER_PAGE		= 0x0005,
	BREAK_AFTER_COLUMN		= 0x0006,
	BREAK_AFTER_AVOID_PAGE		= 0x0007,
	BREAK_AFTER_AVOID_COLUMN	= 0x0008
} 

pub enum op_break_before {
	BREAK_BEFORE_AUTO		= 0x0000,
	BREAK_BEFORE_ALWAYS		= 0x0001,
	BREAK_BEFORE_AVOID		= 0x0002,
	BREAK_BEFORE_LEFT		= 0x0003,
	BREAK_BEFORE_RIGHT		= 0x0004,
	BREAK_BEFORE_PAGE		= 0x0005,
	BREAK_BEFORE_COLUMN		= 0x0006,
	BREAK_BEFORE_AVOID_PAGE		= 0x0007,
	BREAK_BEFORE_AVOID_COLUMN	= 0x0008
} 

pub enum op_break_inside {
	BREAK_INSIDE_AUTO		= 0x0000,
	BREAK_INSIDE_AVOID		= 0x0001,
	BREAK_INSIDE_AVOID_PAGE		= 0x0002,
	BREAK_INSIDE_AVOID_COLUMN	= 0x0003
} 

pub enum op_caption_side {
	CAPTION_SIDE_TOP		= 0x0000,
	CAPTION_SIDE_BOTTOM		= 0x0001
} 

pub enum op_clear {
	CLEAR_NONE			= 0x0000,
	CLEAR_LEFT			= 0x0001,
	CLEAR_RIGHT			= 0x0002,
	CLEAR_BOTH			= 0x0003
} 

pub enum op_clip {
	CLIP_SHAPE_MASK			= 0x0087,
	CLIP_SHAPE_RECT			= 0x0080,

	CLIP_RECT_TOP_AUTO		= 0x0008,
	CLIP_RECT_RIGHT_AUTO		= 0x0010,
	CLIP_RECT_BOTTOM_AUTO		= 0x0020,
	CLIP_RECT_LEFT_AUTO		= 0x0040,

	CLIP_AUTO			= 0x0000
} 

pub enum op_color {
	COLOR_TRANSPARENT		= 0x0000,
	COLOR_CURRENT_COLOR		= 0x0001,
	COLOR_SET			= 0x0080
} 

pub enum op_column_count {
	COLUMN_COUNT_AUTO		= 0x0000,
	COLUMN_COUNT_SET		= 0x0080
} 

pub enum op_column_fill {
	COLUMN_FILL_BALANCE		= 0x0000,
	COLUMN_FILL_AUTO		= 0x0001
} 

pub enum op_column_gap {
	COLUMN_GAP_NORMAL		= 0x0000,
	COLUMN_GAP_SET			= 0x0080
} 

pub enum op_column_rule_color {
	COLUMN_RULE_COLOR_TRANSPARENT	= 0x0000,
	COLUMN_RULE_COLOR_CURRENT_COLOR	= 0x0001,
	COLUMN_RULE_COLOR_INVERT	= 0x0002,
	COLUMN_RULE_COLOR_SET		= 0x0080
} 

pub enum op_column_rule_style {
	COLUMN_RULE_STYLE_NONE		= 0x0000,
	COLUMN_RULE_STYLE_HIDDEN	= 0x0001,
	COLUMN_RULE_STYLE_DOTTED	= 0x0002,
	COLUMN_RULE_STYLE_DASHED	= 0x0003,
	COLUMN_RULE_STYLE_SOLID		= 0x0004,
	COLUMN_RULE_STYLE_DOUBLE	= 0x0005,
	COLUMN_RULE_STYLE_GROOVE	= 0x0006,
	COLUMN_RULE_STYLE_RIDGE		= 0x0007,
	COLUMN_RULE_STYLE_INSET		= 0x0008,
	COLUMN_RULE_STYLE_OUTSET	= 0x0009
} 

pub enum op_column_rule_width {
	COLUMN_RULE_WIDTH_SET		= 0x0080,
	COLUMN_RULE_WIDTH_THIN		= 0x0000,
	COLUMN_RULE_WIDTH_MEDIUM	= 0x0001,
	COLUMN_RULE_WIDTH_THICK		= 0x0002
} 

pub enum op_column_span {
	COLUMN_SPAN_NONE		= 0x0000,
	COLUMN_SPAN_ALL			= 0x0001
} 

pub enum op_column_width {
	COLUMN_WIDTH_AUTO		= 0x0000,
	COLUMN_WIDTH_SET		= 0x0080
} 

pub enum op_content {
	CONTENT_STRING			= 0x0080,
	CONTENT_URI			= 0x0081,
	CONTENT_COUNTER			= 0x0082,
	CONTENT_COUNTERS		= 0x0083,
	CONTENT_ATTR			= 0x0084,

	CONTENT_COUNTER_STYLE_SHIFT	= 8,
	//CONTENT_COUNTERS_STYLE_SHIFT	= 8,

	CONTENT_NORMAL			= 0x0000,
	CONTENT_NONE			= 0x0001,
	CONTENT_OPEN_QUOTE		= 0x0002,
	CONTENT_CLOSE_QUOTE		= 0x0003,
	CONTENT_NO_OPEN_QUOTE		= 0x0004,
	CONTENT_NO_CLOSE_QUOTE		= 0x0005
} 
const   CONTENT_COUNTERS_STYLE_SHIFT :  op_content = CONTENT_COUNTER_STYLE_SHIFT;	

pub enum op_counter_increment {
	COUNTER_INCREMENT_NAMED		= 0x0080,

	COUNTER_INCREMENT_NONE		= 0x0000
} 

pub enum op_counter_reset {
	COUNTER_RESET_NAMED		= 0x0080,

	COUNTER_RESET_NONE		= 0x0000
} 

pub enum op_cue_after {
	CUE_AFTER_URI			= 0x0080,
	CUE_AFTER_NONE			= 0x0000
} 

pub enum op_cue_before {
	CUE_BEFORE_URI			= 0x0080,
	CUE_BEFORE_NONE			= 0x0000
} 

pub enum op_cursor {
	CURSOR_URI			= 0x0080,

	CURSOR_AUTO			= 0x0000,
	CURSOR_CROSSHAIR		= 0x0001,
	CURSOR_DEFAULT			= 0x0002,
	CURSOR_POINTER			= 0x0003,
	CURSOR_MOVE			= 0x0004,
	CURSOR_E_RESIZE			= 0x0005,
	CURSOR_NE_RESIZE		= 0x0006,
	CURSOR_NW_RESIZE		= 0x0007,
	CURSOR_N_RESIZE			= 0x0008,
	CURSOR_SE_RESIZE		= 0x0009,
	CURSOR_SW_RESIZE		= 0x000a,
	CURSOR_S_RESIZE			= 0x000b,
	CURSOR_W_RESIZE			= 0x000c,
	CURSOR_TEXT			= 0x000d,
	CURSOR_WAIT			= 0x000e,
	CURSOR_HELP			= 0x000f,
	CURSOR_PROGRESS			= 0x0010
} 

pub enum op_direction {
	DIRECTION_LTR			= 0x0000,
	DIRECTION_RTL			= 0x0001
} 

pub enum op_display {
	DISPLAY_INLINE			= 0x0000,
	DISPLAY_BLOCK			= 0x0001,
	DISPLAY_LIST_ITEM		= 0x0002,
	DISPLAY_RUN_IN			= 0x0003,
	DISPLAY_INLINE_BLOCK		= 0x0004,
	DISPLAY_TABLE			= 0x0005,
	DISPLAY_INLINE_TABLE		= 0x0006,
	DISPLAY_TABLE_ROW_GROUP		= 0x0007,
	DISPLAY_TABLE_HEADER_GROUP	= 0x0008,
	DISPLAY_TABLE_FOOTER_GROUP	= 0x0009,
	DISPLAY_TABLE_ROW		= 0x000a,
	DISPLAY_TABLE_COLUMN_GROUP	= 0x000b,
	DISPLAY_TABLE_COLUMN		= 0x000c,
	DISPLAY_TABLE_CELL		= 0x000d,
	DISPLAY_TABLE_CAPTION		= 0x000e,
	DISPLAY_NONE			= 0x000f
} 

pub enum op_elevation {
	ELEVATION_ANGLE			= 0x0080,
	ELEVATION_BELOW			= 0x0000,
	ELEVATION_LEVEL			= 0x0001,
	ELEVATION_ABOVE			= 0x0002,
	ELEVATION_HIGHER		= 0x0003,
	ELEVATION_LOWER			= 0x0004
} 

pub enum op_empty_cells {
	EMPTY_CELLS_SHOW		= 0x0000,
	EMPTY_CELLS_HIDE		= 0x0001
} 

pub enum op_float {
	FLOAT_LEFT			= 0x0000,
	FLOAT_RIGHT			= 0x0001,
	FLOAT_NONE			= 0x0002
} 

pub enum op_font_family {
	FONT_FAMILY_STRING		= 0x0080,
	FONT_FAMILY_IDENT_LIST		= 0x0081,

	FONT_FAMILY_END			= 0x0000,

	FONT_FAMILY_SERIF		= 0x0001,
	FONT_FAMILY_SANS_SERIF		= 0x0002,
	FONT_FAMILY_CURSIVE		= 0x0003,
	FONT_FAMILY_FANTASY		= 0x0004,
	FONT_FAMILY_MONOSPACE		= 0x0005
} 

pub enum op_font_size {
	FONT_SIZE_DIMENSION		= 0x0080,

	FONT_SIZE_XX_SMALL		= 0x0000,
	FONT_SIZE_X_SMALL		= 0x0001,
	FONT_SIZE_SMALL			= 0x0002,
	FONT_SIZE_MEDIUM		= 0x0003,
	FONT_SIZE_LARGE			= 0x0004,
	FONT_SIZE_X_LARGE		= 0x0005,
	FONT_SIZE_XX_LARGE		= 0x0006,
	FONT_SIZE_LARGER		= 0x0007,
	FONT_SIZE_SMALLER		= 0x0008
} 

pub enum op_font_style {
	FONT_STYLE_NORMAL		= 0x0000,
	FONT_STYLE_ITALIC		= 0x0001,
	FONT_STYLE_OBLIQUE		= 0x0002
} 

pub enum op_font_variant {
	FONT_VARIANT_NORMAL		= 0x0000,
	FONT_VARIANT_SMALL_CAPS		= 0x0001
} 

pub enum op_font_weight {
	FONT_WEIGHT_NORMAL		= 0x0000,
	FONT_WEIGHT_BOLD		= 0x0001,
	FONT_WEIGHT_BOLDER		= 0x0002,
	FONT_WEIGHT_LIGHTER		= 0x0003,
	FONT_WEIGHT_100			= 0x0004,
	FONT_WEIGHT_200			= 0x0005,
	FONT_WEIGHT_300			= 0x0006,
	FONT_WEIGHT_400			= 0x0007,
	FONT_WEIGHT_500			= 0x0008,
	FONT_WEIGHT_600			= 0x0009,
	FONT_WEIGHT_700			= 0x000a,
	FONT_WEIGHT_800			= 0x000b,
	FONT_WEIGHT_900			= 0x000c
} 

pub enum op_height {
	HEIGHT_SET			= 0x0080,
	HEIGHT_AUTO			= 0x0000
} 

pub enum op_left {
	LEFT_SET			= 0x0080,
	LEFT_AUTO			= 0x0000
} 

pub enum op_letter_spacing {
	LETTER_SPACING_SET		= 0x0080,
	LETTER_SPACING_NORMAL		= 0x0000
} 

pub enum op_line_height {
	LINE_HEIGHT_NUMBER		= 0x0080,
	LINE_HEIGHT_DIMENSION		= 0x0081,
	LINE_HEIGHT_NORMAL		= 0x0000
} 

pub enum op_list_style_image {
	LIST_STYLE_IMAGE_URI		= 0x0080,
	LIST_STYLE_IMAGE_NONE		= 0x0000
} 

pub enum op_list_style_position {
	LIST_STYLE_POSITION_INSIDE	= 0x0000,
	LIST_STYLE_POSITION_OUTSIDE	= 0x0001
} 

pub enum op_list_style_type {
	LIST_STYLE_TYPE_DISC		= 0x0000,
	LIST_STYLE_TYPE_CIRCLE		= 0x0001,
	LIST_STYLE_TYPE_SQUARE		= 0x0002,
	LIST_STYLE_TYPE_DECIMAL		= 0x0003,
	LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO	= 0x0004,
	LIST_STYLE_TYPE_LOWER_ROMAN	= 0x0005,
	LIST_STYLE_TYPE_UPPER_ROMAN	= 0x0006,
	LIST_STYLE_TYPE_LOWER_GREEK	= 0x0007,
	LIST_STYLE_TYPE_LOWER_LATIN	= 0x0008,
	LIST_STYLE_TYPE_UPPER_LATIN	= 0x0009,
	LIST_STYLE_TYPE_ARMENIAN	= 0x000a,
	LIST_STYLE_TYPE_GEORGIAN	= 0x000b,
	LIST_STYLE_TYPE_LOWER_ALPHA	= 0x000c,
	LIST_STYLE_TYPE_UPPER_ALPHA	= 0x000d,
	LIST_STYLE_TYPE_NONE		= 0x000e
} 

pub enum op_margin {
	MARGIN_SET			= 0x0080,
	MARGIN_AUTO			= 0x0000
} 

pub enum op_max_height {
	MAX_HEIGHT_SET			= 0x0080,
	MAX_HEIGHT_NONE			= 0x0000
} 

pub enum op_max_width {
	MAX_WIDTH_SET			= 0x0080,
	MAX_WIDTH_NONE			= 0x0000
} 

pub enum op_min_height {
	MIN_HEIGHT_SET			= 0x0080
} 

pub enum op_min_width {
	MIN_WIDTH_SET			= 0x0080
} 

pub enum op_opacity {
	OPACITY_SET			= 0x0080
} 

pub enum op_orphans {
	ORPHANS_SET			= 0x0080
} 

pub enum op_outline_color {
	OUTLINE_COLOR_TRANSPARENT	= 0x0000,
	OUTLINE_COLOR_CURRENT_COLOR	= 0x0001,
	OUTLINE_COLOR_INVERT		= 0x0002,
	OUTLINE_COLOR_SET		= 0x0080
} 

pub enum op_outline_style {
	OUTLINE_STYLE_NONE		= 0x0000,
	OUTLINE_STYLE_HIDDEN		= 0x0001,
	OUTLINE_STYLE_DOTTED		= 0x0002,
	OUTLINE_STYLE_DASHED		= 0x0003,
	OUTLINE_STYLE_SOLID		= 0x0004,
	OUTLINE_STYLE_DOUBLE		= 0x0005,
	OUTLINE_STYLE_GROOVE		= 0x0006,
	OUTLINE_STYLE_RIDGE		= 0x0007,
	OUTLINE_STYLE_INSET		= 0x0008,
	OUTLINE_STYLE_OUTSET		= 0x0009
} 

pub enum op_outline_width {
	OUTLINE_WIDTH_SET		= 0x0080,
	OUTLINE_WIDTH_THIN		= 0x0000,
	OUTLINE_WIDTH_MEDIUM		= 0x0001,
	OUTLINE_WIDTH_THICK		= 0x0002
} 

pub enum op_overflow {
	OVERFLOW_VISIBLE		= 0x0000,
	OVERFLOW_HIDDEN			= 0x0001,
	OVERFLOW_SCROLL			= 0x0002,
	OVERFLOW_AUTO			= 0x0003
} 

pub enum op_padding {
	PADDING_SET			= 0x0080
} 

pub enum op_page_break_after {
	PAGE_BREAK_AFTER_AUTO		= 0x0000,
	PAGE_BREAK_AFTER_ALWAYS		= 0x0001,
	PAGE_BREAK_AFTER_AVOID		= 0x0002,
	PAGE_BREAK_AFTER_LEFT		= 0x0003,
	PAGE_BREAK_AFTER_RIGHT		= 0x0004
} 

pub enum op_page_break_before {
	PAGE_BREAK_BEFORE_AUTO		= 0x0000,
	PAGE_BREAK_BEFORE_ALWAYS	= 0x0001,
	PAGE_BREAK_BEFORE_AVOID		= 0x0002,
	PAGE_BREAK_BEFORE_LEFT		= 0x0003,
	PAGE_BREAK_BEFORE_RIGHT		= 0x0004
} 

pub enum op_page_break_inside {
	PAGE_BREAK_INSIDE_AUTO		= 0x0000,
	PAGE_BREAK_INSIDE_AVOID		= 0x0001
} 

pub enum op_pause_after {
	PAUSE_AFTER_SET			= 0x0080
} 

pub enum op_pause_before {
	PAUSE_BEFORE_SET		= 0x0080
} 

pub enum op_pitch_range {
	PITCH_RANGE_SET			= 0x0080
} 

pub enum op_pitch {
	PITCH_FREQUENCY			= 0x0080,

	PITCH_X_LOW			= 0x0000,
	PITCH_LOW			= 0x0001,
	PITCH_MEDIUM			= 0x0002,
	PITCH_HIGH			= 0x0003,
	PITCH_X_HIGH			= 0x0004
} 

pub enum op_play_during {
	PLAY_DURING_TYPE_MASK		= 0x009f,
	PLAY_DURING_URI			= 0x0080,
	PLAY_DURING_MIX			= (1<<6),
	PLAY_DURING_REPEAT		= (1<<5),

	PLAY_DURING_AUTO		= 0x0000,
	PLAY_DURING_NONE		= 0x0001
} 

pub enum op_position {
	POSITION_STATIC			= 0x0000,
	POSITION_RELATIVE		= 0x0001,
	POSITION_ABSOLUTE		= 0x0002,
	POSITION_FIXED			= 0x0003
} 

pub enum op_quotes {
	QUOTES_STRING			= 0x0080,

	QUOTES_NONE			= 0x0000
} 

pub enum op_richness {
	RICHNESS_SET			= 0x0080
} 

pub enum op_right {
	RIGHT_SET			= 0x0080,
	RIGHT_AUTO			= 0x0000
} 

pub enum op_speak_header {
	SPEAK_HEADER_ONCE		= 0x0000,
	SPEAK_HEADER_ALWAYS		= 0x0001
} 

pub enum op_speak_numeral {
	SPEAK_NUMERAL_DIGITS		= 0x0000,
	SPEAK_NUMERAL_CONTINUOUS	= 0x0001
} 

pub enum op_speak_punctuation {
	SPEAK_PUNCTUATION_CODE		= 0x0000,
	SPEAK_PUNCTUATION_NONE		= 0x0001
} 

pub enum op_speak {
	SPEAK_NORMAL			= 0x0000,
	SPEAK_NONE			= 0x0001,
	SPEAK_SPELL_OUT			= 0x0002
} 

pub enum op_speech_rate {
	SPEECH_RATE_SET			= 0x0080,

	SPEECH_RATE_X_SLOW		= 0x0000,
	SPEECH_RATE_SLOW		= 0x0001,
	SPEECH_RATE_MEDIUM		= 0x0002,
	SPEECH_RATE_FAST		= 0x0003,
	SPEECH_RATE_X_FAST		= 0x0004,
	SPEECH_RATE_FASTER		= 0x0005,
	SPEECH_RATE_SLOWER		= 0x0006
} 

pub enum op_stress {
	STRESS_SET			= 0x0080
} 

pub enum op_table_layout {
	TABLE_LAYOUT_AUTO		= 0x0000,
	TABLE_LAYOUT_FIXED		= 0x0001
} 

pub enum op_text_align {
	TEXT_ALIGN_LEFT			= 0x0000,
	TEXT_ALIGN_RIGHT		= 0x0001,
	TEXT_ALIGN_CENTER		= 0x0002,
	TEXT_ALIGN_JUSTIFY		= 0x0003,
	TEXT_ALIGN_LIBCSS_LEFT		= 0x0004,
	TEXT_ALIGN_LIBCSS_CENTER	= 0x0005,
	TEXT_ALIGN_LIBCSS_RIGHT		= 0x0006
} 

pub enum op_text_decoration {
	TEXT_DECORATION_NONE		= 0x0000,

	TEXT_DECORATION_BLINK		= (1<<3),
	TEXT_DECORATION_LINE_THROUGH	= (1<<2),
	TEXT_DECORATION_OVERLINE	= (1<<1),
	TEXT_DECORATION_UNDERLINE	= (1<<0)
} 

pub enum op_text_indent {
	TEXT_INDENT_SET			= 0x0080
} 

pub enum op_text_transform {
	TEXT_TRANSFORM_CAPITALIZE	= 0x0000,
	TEXT_TRANSFORM_UPPERCASE	= 0x0001,
	TEXT_TRANSFORM_LOWERCASE	= 0x0002,
	TEXT_TRANSFORM_NONE		= 0x0003
} 

pub enum op_top {
	TOP_SET				= 0x0080,
	TOP_AUTO			= 0x0000
} 

pub enum op_unicode_bidi {
	UNICODE_BIDI_NORMAL		= 0x0000,
	UNICODE_BIDI_EMBED		= 0x0001,
	UNICODE_BIDI_BIDI_OVERRIDE	= 0x0002
} 

pub enum op_vertical_align {
	VERTICAL_ALIGN_SET		= 0x0080,

	VERTICAL_ALIGN_BASELINE		= 0x0000,
	VERTICAL_ALIGN_SUB		= 0x0001,
	VERTICAL_ALIGN_SUPER		= 0x0002,
	VERTICAL_ALIGN_TOP		= 0x0003,
	VERTICAL_ALIGN_TEXT_TOP		= 0x0004,
	VERTICAL_ALIGN_MIDDLE		= 0x0005,
	VERTICAL_ALIGN_BOTTOM		= 0x0006,
	VERTICAL_ALIGN_TEXT_BOTTOM	= 0x0007
} 

pub enum op_visibility {
	VISIBILITY_VISIBLE		= 0x0000,
	VISIBILITY_HIDDEN		= 0x0001,
	VISIBILITY_COLLAPSE		= 0x0002
} 

pub enum op_voice_family {
	VOICE_FAMILY_STRING		= 0x0080,
	VOICE_FAMILY_IDENT_LIST		= 0x0081,

	VOICE_FAMILY_END		= 0x0000,

	VOICE_FAMILY_MALE		= 0x0001,
	VOICE_FAMILY_FEMALE		= 0x0002,
	VOICE_FAMILY_CHILD		= 0x0003
} 

pub enum op_volume {
	VOLUME_NUMBER			= 0x0080,
	VOLUME_DIMENSION		= 0x0081,

	VOLUME_SILENT			= 0x0000,
	VOLUME_X_SOFT			= 0x0001,
	VOLUME_SOFT			= 0x0002,
	VOLUME_MEDIUM			= 0x0003,
	VOLUME_LOUD			= 0x0004,
	VOLUME_X_LOUD			= 0x0005
} 

pub enum op_white_space {
	WHITE_SPACE_NORMAL		= 0x0000,
	WHITE_SPACE_PRE			= 0x0001,
	WHITE_SPACE_NOWRAP		= 0x0002,
	WHITE_SPACE_PRE_WRAP		= 0x0003,
	WHITE_SPACE_PRE_LINE		= 0x0004
} 

pub enum op_widows {
	WIDOWS_SET			= 0x0080
} 

pub enum op_width {
	WIDTH_SET			= 0x0080,

	WIDTH_AUTO			= 0x0000
} 

pub enum op_word_spacing {
	WORD_SPACING_SET		= 0x0080,

	WORD_SPACING_NORMAL		= 0x0000
} 

pub enum op_z_index {
	Z_INDEX_SET			= 0x0080,

	Z_INDEX_AUTO			= 0x0000
} 




/* ///////////////////////////////////////////////////////////////////////////////////////////
 * ////////////////////////// bytecode.h /////////////////////////////////////////////////////
   //////////////////////////////////////////////////////////////////////////////////////////*/


////////////////////////////////////////////////////////////////////////////////
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
	PARSERUTILS_OK(@parserutils_filter),
	PARSERUTILS_SUCCESS,
    PARSERUTILS_BADPARAM,
    PARSERUTILS_NOMEM,
    PARSERUTILS_BADENCODING,
    PARSERUTILS_DESTROY_SUCCESS,
    PARSERUTILS_ICONV_ERROR
}


/////////////////////////////////////////////////////////////////////////////////


pub enum css_properties_e {
		CSS_PROP_AZIMUTH			= 0x000,
		CSS_PROP_BACKGROUND_ATTACHMENT		= 0x001,
		CSS_PROP_BACKGROUND_COLOR		= 0x002,
		CSS_PROP_BACKGROUND_IMAGE		= 0x003,
		CSS_PROP_BACKGROUND_POSITION		= 0x004,
		CSS_PROP_BACKGROUND_REPEAT		= 0x005,
		CSS_PROP_BORDER_COLLAPSE		= 0x006,
		CSS_PROP_BORDER_SPACING			= 0x007,
		CSS_PROP_BORDER_TOP_COLOR		= 0x008,
		CSS_PROP_BORDER_RIGHT_COLOR		= 0x009,
		CSS_PROP_BORDER_BOTTOM_COLOR		= 0x00a,
		CSS_PROP_BORDER_LEFT_COLOR		= 0x00b,
		CSS_PROP_BORDER_TOP_STYLE		= 0x00c,
		CSS_PROP_BORDER_RIGHT_STYLE		= 0x00d,
		CSS_PROP_BORDER_BOTTOM_STYLE		= 0x00e,
		CSS_PROP_BORDER_LEFT_STYLE		= 0x00f,
		CSS_PROP_BORDER_TOP_WIDTH		= 0x010,
		CSS_PROP_BORDER_RIGHT_WIDTH		= 0x011,
		CSS_PROP_BORDER_BOTTOM_WIDTH		= 0x012,
		CSS_PROP_BORDER_LEFT_WIDTH		= 0x013,
		CSS_PROP_BOTTOM				= 0x014,
		CSS_PROP_CAPTION_SIDE			= 0x015,
		CSS_PROP_CLEAR				= 0x016,
		CSS_PROP_CLIP				= 0x017,
		CSS_PROP_COLOR				= 0x018,
		CSS_PROP_CONTENT			= 0x019,
		CSS_PROP_COUNTER_INCREMENT		= 0x01a,
		CSS_PROP_COUNTER_RESET			= 0x01b,
		CSS_PROP_CUE_AFTER			= 0x01c,
		CSS_PROP_CUE_BEFORE			= 0x01d,
		CSS_PROP_CURSOR				= 0x01e,
		CSS_PROP_DIRECTION			= 0x01f,
		CSS_PROP_DISPLAY			= 0x020,
		CSS_PROP_ELEVATION			= 0x021,
		CSS_PROP_EMPTY_CELLS			= 0x022,
		CSS_PROP_FLOAT				= 0x023,
		CSS_PROP_FONT_FAMILY			= 0x024,
		CSS_PROP_FONT_SIZE			= 0x025,
		CSS_PROP_FONT_STYLE			= 0x026,
		CSS_PROP_FONT_VARIANT			= 0x027,
		CSS_PROP_FONT_WEIGHT			= 0x028,
		CSS_PROP_HEIGHT				= 0x029,
		CSS_PROP_LEFT				= 0x02a,
		CSS_PROP_LETTER_SPACING			= 0x02b,
		CSS_PROP_LINE_HEIGHT			= 0x02c,
		CSS_PROP_LIST_STYLE_IMAGE		= 0x02d,
		CSS_PROP_LIST_STYLE_POSITION		= 0x02e,
		CSS_PROP_LIST_STYLE_TYPE		= 0x02f,
		CSS_PROP_MARGIN_TOP			= 0x030,
		CSS_PROP_MARGIN_RIGHT			= 0x031,
		CSS_PROP_MARGIN_BOTTOM			= 0x032,
		CSS_PROP_MARGIN_LEFT			= 0x033,
		CSS_PROP_MAX_HEIGHT			= 0x034,
		CSS_PROP_MAX_WIDTH			= 0x035,
		CSS_PROP_MIN_HEIGHT			= 0x036,
		CSS_PROP_MIN_WIDTH			= 0x037,
		CSS_PROP_ORPHANS			= 0x038,
		CSS_PROP_OUTLINE_COLOR			= 0x039,
		CSS_PROP_OUTLINE_STYLE			= 0x03a,
		CSS_PROP_OUTLINE_WIDTH			= 0x03b,
		CSS_PROP_OVERFLOW			= 0x03c,
		CSS_PROP_PADDING_TOP			= 0x03d,
		CSS_PROP_PADDING_RIGHT			= 0x03e,
		CSS_PROP_PADDING_BOTTOM			= 0x03f,
		CSS_PROP_PADDING_LEFT			= 0x040,
		CSS_PROP_PAGE_BREAK_AFTER		= 0x041,
		CSS_PROP_PAGE_BREAK_BEFORE		= 0x042,
		CSS_PROP_PAGE_BREAK_INSIDE		= 0x043,
		CSS_PROP_PAUSE_AFTER			= 0x044,
		CSS_PROP_PAUSE_BEFORE			= 0x045,
		CSS_PROP_PITCH_RANGE			= 0x046,
		CSS_PROP_PITCH				= 0x047,
		CSS_PROP_PLAY_DURING			= 0x048,
		CSS_PROP_POSITION			= 0x049,
		CSS_PROP_QUOTES				= 0x04a,
		CSS_PROP_RICHNESS			= 0x04b,
		CSS_PROP_RIGHT				= 0x04c,
		CSS_PROP_SPEAK_HEADER			= 0x04d,
		CSS_PROP_SPEAK_NUMERAL			= 0x04e,
		CSS_PROP_SPEAK_PUNCTUATION		= 0x04f,
		CSS_PROP_SPEAK				= 0x050,
		CSS_PROP_SPEECH_RATE			= 0x051,
		CSS_PROP_STRESS				= 0x052,
		CSS_PROP_TABLE_LAYOUT			= 0x053,
		CSS_PROP_TEXT_ALIGN			= 0x054,
		CSS_PROP_TEXT_DECORATION		= 0x055,
		CSS_PROP_TEXT_INDENT			= 0x056,
		CSS_PROP_TEXT_TRANSFORM			= 0x057,
		CSS_PROP_TOP				= 0x058,
		CSS_PROP_UNICODE_BIDI			= 0x059,
		CSS_PROP_VERTICAL_ALIGN			= 0x05a,
		CSS_PROP_VISIBILITY			= 0x05b,
		CSS_PROP_VOICE_FAMILY			= 0x05c,
		CSS_PROP_VOLUME				= 0x05d,
		CSS_PROP_WHITE_SPACE			= 0x05e,
		CSS_PROP_WIDOWS				= 0x05f,
		CSS_PROP_WIDTH				= 0x060,
		CSS_PROP_WORD_SPACING			= 0x061,
		CSS_PROP_Z_INDEX			= 0x062,
		CSS_PROP_OPACITY			= 0x063,
		CSS_PROP_BREAK_AFTER			= 0x064,
		CSS_PROP_BREAK_BEFORE			= 0x065,
		CSS_PROP_BREAK_INSIDE			= 0x066,
		CSS_PROP_COLUMN_COUNT			= 0x067,
		CSS_PROP_COLUMN_FILL			= 0x068,
		CSS_PROP_COLUMN_GAP			= 0x069,
		CSS_PROP_COLUMN_RULE_COLOR		= 0x06a,
		CSS_PROP_COLUMN_RULE_STYLE		= 0x06b,
		CSS_PROP_COLUMN_RULE_WIDTH		= 0x06c,
		CSS_PROP_COLUMN_SPAN			= 0x06d,
		CSS_PROP_COLUMN_WIDTH			= 0x06e,

		CSS_N_PROPERTIES = 0x06f
} 




type css_code_t = u32 ; 


pub enum flag {
	FLAG_IMPORTANT			= (1<<0),
	FLAG_INHERIT			= (1<<1)
}

pub enum unit {
	UNIT_PX   = 0,
	UNIT_EX   = 1,
	UNIT_EM   = 2,
	UNIT_IN   = 3,
	UNIT_CM   = 4,
	UNIT_MM   = 5,
	UNIT_PT   = 6,
	UNIT_PC   = 7,

	UNIT_PCT  = (1 << 8),

	UNIT_DEG  = (1 << 9) + 0,
	UNIT_GRAD = (1 << 9) + 1,
	UNIT_RAD  = (1 << 9) + 2,

	UNIT_MS   = (1 << 10) + 0,
	UNIT_S    = (1 << 10) + 1,

	UNIT_HZ   = (1 << 11) + 0,
	UNIT_KHZ  = (1 << 11) + 1
} 
const   UNIT_ANGLE :  unit = UNIT_DEG ;	//< Default level >
const   UNIT_TIME  :  unit = UNIT_MS  ;	//< Default level >
const   UNIT_FREQ  :  unit = UNIT_HZ  ;	//< Default level >

type  colour =  u32;

pub enum shape {
	SHAPE_RECT = 0
} 

enum language_state
{

		CHARSET_PERMITTED,
		IMPORT_PERMITTED,
		NAMESPACE_PERMITTED,
		HAD_RULE
	
}

struct css_namespace {
	prefix:@lwc_string,	/**< Namespace prefix */
	uri:@lwc_string		/*< Namespace URI */
}
 struct css_language {
	sheet:@css_stylesheet ,		/**< The stylesheet to parse for */

//#define STACK_CHUNK 32
    STACK_CHUNK:int,
	context:@DVec<context_entry>,      //parseutils_stack	/**< Context stack */

	 state:language_state,			/**< State flag, for at-rule handling */

	/** Interned strings */
	strings:@ mut[@lwc_string ],

	default_namespace:@lwc_string ,	/**< Default namespace URI */
	namespaces:@css_namespace,	/**< Array of namespace mappings */
	num_namespaces:u32	/*< Number of namespace mappings */

	// css_allocator_fn alloc;		*< Memory (de)allocation function 
	// void *pw;			/**< Client's private data */
}
/*pub type  css_parser_event_handler =  ~extern fn( event_type:css_parser_event, 
		 tokens:~[u8], pw:~[u8]) -> css_result;*/
struct css_parser_event_handler_{
		 handler:css_parser_event_handler,
		pw:@css_language
	} 
pub struct  css_parser_optparams {
	quirks:bool,
	event_handler: css_parser_event_handler_
	
} 

struct context_entry {
	event_type:css_parser_event,		/**< Type of entry */
	data:~[u8]		/*< Data for context */
} 

pub struct lcss {
	mut lwc_instance:@lwc,
	mut lpu_instance:@lpu,
	mut propstrings_call_count:uint,
	mut propstrings_list:@[@str],
	mut propstrings:~[@lwc_string]
}


pub fn lcss()->@lcss {
	@lcss {
		lwc_instance:lwc(),
		lpu_instance:lpu(),
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
	    propstrings:~[]
	}
}



impl lcss {

	static pub fn isDigit( c:char)-> bool
	{
		return '0' <= c && c <= '9';
	}

	static pub fn isHex(c:char)->bool 
	{
		return lcss::isDigit(c) || ('a' <= c && c <= 'f') || ('A' <= c && c <= 'F');
	}

	static pub fn charToHex( mut c: char)-> u32
	{
		c -= '0';

		if (c as u8> 9)
			{c -= 'A' - '9' - 1 as char;}

		if (c as u8 > 15)
			{c -= 'a' - 'A';}

		return c as u32;
	}

	//pub type css_fixed = 32;

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
	static pub fn css__number_from_lwc_string(string:@lwc_string,
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

	static pub fn css__number_from_string(data:@str/*, len:size_t*/ ,
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
				if (len == 1 || data[iter+1] < '0' as u8|| '9' as u8 < data[iter+1]) {
					*consumed = 0 ;
					return 0;
				}
			} else if (data[iter] < '0' as u8 || '9' as u8 < data[iter]) {
				*consumed = 0;
				return 0;
			}
		}


	    /* Now extract intpart, assuming base 10 */
		while (len > 0) {
			/* Stop on first non-digit */
			if (data[iter] < '0' as u8 || '9' as u8 < data[iter])
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
				if (data[iter] < '0' as u8 || '9' as u8 < data[iter])
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

	// pub fn css__propstrings_unref()
	// {
	// 	css__propstrings.count -=1;

	// 	if (css__propstrings.count == 0) {
	// 		let mut  i=0;

	// 		while ( i < vec2.len())
	// 		{
	// 			lwc_string_unref(css__propstrings.strings[i]);
	// 			i += 1;
	// 		}
				
	// 	}
	// }

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
	// pub fn css_result_from_lwc_error( err:lwc_error)->css_result
	// {
	//         match (err) {
	//          lwc_error_ok=>
	//                 {return CSS_OK;},
	//          lwc_error_oom=>
	//                 {return CSS_NOMEM;},
	//          lwc_error_range=>
	//                 {return CSS_BADPARM;}/*,
	//         _=>{}*/
	                

	//         }
	//         return CSS_INVALID;
	// }
	/**
	 * Create a CSS language parser
	 *
	 * \param sheet	    The stylesheet object to parse for
	 * \param parser    The core parser object to use
	 * \param alloc	    Memory (de)allocation function
	 * \param pw	    Pointer to client-specific private data
	 * \param language  Pointer to location to receive parser object
	 * \return CSS_OK on success,
	 *	   CSS_BADPARM on bad parameters,
	 *	   CSS_NOMEM on memory exhaustion
	 */
	pub fn  css__language_create(&self, sheet:@css_stylesheet,  parser:@css_parser) -> css_result
	{
		let css_language_instance:@css_language;
		
		let presult:parserutils_result;
		let err:css_result;
		let stack:@DVec<context_entry> = @dvec::DVec();

		// if (sheet == NULL || parser == NULL || alloc == NULL || 
		// 		language == NULL)
		// 	return CSS_BADPARM;

		let empty_lwc_string = self.lwc_instance.lwc_intern_string(@"");

		css_language_instance = @css_language {
				sheet:sheet ,		
	    		STACK_CHUNK:32,
				context:stack, 
				state:CHARSET_PERMITTED,	
				strings:sheet.propstrings,
				
				default_namespace:empty_lwc_string,	
				
				namespaces:@css_namespace
				{
					prefix:empty_lwc_string,	
					uri:empty_lwc_string	
				},	
				num_namespaces:0	
		
		};

		// let params = @css_parser_optparams {
		// 	quirks:false,
		// 	event_handler: css_parser_event_handler_
		// 	{
		// 		handler:&(self.language_handle_event),
		// 		pw:css_language_instance
		// 	}
		// }; //see later
		
		
		//let err = css__parser_setopt(parser, CSS_PARSER_EVENT_HANDLER, params);
		

		// c->sheet = sheet;
		// c->state = CHARSET_PERMITTED;
		// c->default_namespace = NULL;
		// c->namespaces = NULL;
		// c->num_namespaces = 0;
		// c->strings = sheet->propstrings;
		// c->alloc = alloc;
		// c->pw = pw;

		return CSS_LANGUAGE_CREATED(css_language_instance);
	}

	 pub fn css__parser_setopt(&self, parser:@css_parser,  opt_type:css_parser_opttype,
			params:@css_parser_optparams)-> css_result
	{
		// if (parser == NULL || params == NULL)
		// 	return CSS_BADPARM;

		match(opt_type) 
		{
	     	CSS_PARSER_QUIRKS=>
	     		{
					parser.quirks = params.quirks;
	     		},
	     	CSS_PARSER_EVENT_HANDLER=>
	     		{
	     			parser.event = copy params.event_handler.handler;
		 			parser.event_pw = copy params.event_handler.pw;
	     		}
		}
		

		CSS_GENERAL_OK
	}


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

	pub fn handleStartStylesheet(&self, c:@css_language, vector:~[~str]) -> css_result
	{
		// let pResult:parserutils_result;
		// UNUSED(vector);
		let entry:context_entry = context_entry {event_type: CSS_PARSER_START_STYLESHEET, data:~[] };
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
		// parserutils_error perror;
		// css_result error;
		// context_entry entry = { CSS_PARSER_START_RULESET, NULL };
		// let entry:context_entry = context_entry {event_type: CSS_PARSER_START_RULESET, data:~[] };
		// context_entry *cur;
		// css_rule *parent_rule = NULL;
		// css_rule *rule = NULL;

		// assert(c != NULL);

		// /* Retrieve parent rule from stack, if any */
		// cur = parserutils_stack_get_current(c->context);
		// if (cur != NULL && cur->type != CSS_PARSER_START_STYLESHEET)
		// 	parent_rule = cur->data;

		// error = css__stylesheet_rule_create(c->sheet, CSS_RULE_SELECTOR, &rule);
		// if (error != CSS_OK)
		// 	return error;

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

		// /* Rule is now owned by the sheet, so no need to destroy it */

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
	/**
	 * Destroy a CSS language parser
	 *
	 * \param language  The parser to destroy
	 * \return CSS_OK on success, appropriate error otherwise
	 */
	/*css_result css__language_destroy(css_language *language)
	{
		uint32_t i;
		
		if (language == NULL)
			return CSS_BADPARM;

		if (language->default_namespace != NULL)
			lwc_string_unref(language->default_namespace);

		if (language->namespaces != NULL) {
			for (i = 0; i < language->num_namespaces; i++) {
				lwc_string_unref(language->namespaces[i].prefix);
				lwc_string_unref(language->namespaces[i].uri);
			}

			language->alloc(language->namespaces, 0, language->pw);
		}

		parserutils_stack_destroy(language->context);
		
		language->alloc(language, 0, language->pw);

		return CSS_OK;
	}*/

	/**
	 * Handler for core parser events
	 *
	 * \param type	  The event type
	 * \param tokens  Vector of tokens read since last event, or NULL
	 * \param pw	  Pointer to handler context
	 * \return CSS_OK on success, CSS_INVALID to indicate parse error, 
	 *	   appropriate error otherwise.
	 */


	/*pub fn css__propstrings_get(lwc_string ***strings)->css_result
	{

	let 
		if (css__propstrings.count > 0) {
			css__propstrings.count++;
		} else {
			let mut i:int =0;
			let mut lerror:lwc_error;

			// Intern all known strings 
			while ( i < LAST_KNOWN) {
				lerror = lwc_intern_string(stringmap[i].data,
						stringmap[i].len,
						&css__propstrings.strings[i]);

				match(lerror)
				{
					lwc_error_ok => {},
					_=>return CSS_NOMEM 
				}
				
				i += 1;
			}
			css__propstrings.count++;
		}

		*strings = css__propstrings.strings;

		return CSS_OK;
	}*/

	static pub fn buildOPV(opcode : css_properties_e , flags : u8 , value : u16 ) -> css_code_t {

		(( (opcode as int)  & 0x3ff) | ((flags as int)<< 10) | (((value as int)& 0x3fff)  << 18) ) as u32
	}

	static pub fn getOpcode(OPV : css_code_t ) -> css_properties_e {

		 //((OPV & 0x3ff) as int) as opcode_t
		 let op_code : int = (OPV & 0x3ff) as int ;
		 unsafe { cast::transmute(&op_code) }
	}

	static pub fn getFlags(OPV : css_code_t ) -> u8 {

		((OPV >> 10) & 0xff) as u8
	}

	static pub fn getValue(OPV : css_code_t ) -> u16 {

		 (OPV >> 18) as u16
	}

	static pub fn isImportant(OPV : css_code_t ) -> bool {

		if (lcss::getFlags(OPV) & 0x1)==0 {
		 	false
		 }
		 else {
		 	true
		 }
	}

	static pub fn isInherit(OPV : css_code_t ) -> bool {

		if (lcss::getFlags(OPV) & 0x2)==0 {
			false 
		}
		else {
			true
		}
	}

	static pub fn memcmp(str1 : &[u8] , str2 : &[u8] , len : uint ) -> int {
		let mut i : uint = 0 ;
		while ( i<len ) {
			if str1[i] != str2[i] {
				return ( (str1[i]-str2[i]) as int) ;
			}
			i = i+1 ; 
		}
		0
	}

	// Sushanta
	// Purpose of this function is to identify the encoding technique used in the CSS file and so that the data can be read

	pub fn try_utf32_charset(&self, data : ~[u8]) -> parserutils::parserutils_result {

		let mut charset: u16 = 0;
		let CHARSET_BE : &[u8] = ['0' as u8, '0' as u8, '0' as u8, '@' as u8, '0' as u8, '0' as u8, '0' as u8, 'c' as u8, '0' as u8, '0' as u8, '0' as u8, 'h' as u8, '0' as u8, '0' as u8, '0' as u8, 'a' as u8, '0' as u8, '0' as u8, '0' as u8, 'r' as u8, '0' as u8, '0' as u8, '0' as u8, 's' as u8, '0' as u8, '0' as u8, '0' as u8, 'e' as u8, '0' as u8, '0' as u8, '0' as u8, 't' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '"' as u8] ; 
		let CHARSET_LE : &[u8] = [ '@' as u8,'0' as u8,'0' as u8,'0' as u8,'c' as u8,'0' as u8,'0' as u8,'0' as u8,'h' as u8,'0' as u8,'0' as u8,'0' as u8,'a' as u8,'0' as u8,'0' as u8,'0' as u8,'r' as u8,'0' as u8,'0' as u8,'0' as u8,'s' as u8,'0' as u8,'0' as u8,'0' as u8,'e' as u8,'0' as u8,'0' as u8,'0' as u8,'t' as u8,'0' as u8,'0' as u8,' ' as u8,'0' as u8,'0' as u8,'0' as u8,'"' as u8,'0' as u8,'0' as u8,'0' as u8, ] ;

		io::println("\n Sushanta1: Inside CHARSET_LE 32 bit");
		io::println(fmt!("value of data.len() is %?", data.len()));
		io::println(fmt!("value of CHARSET_LE.len() is %?", CHARSET_LE.len()));

		// Here, when the data.len() is equals to CHARSET_LE.len() then it returns, then how would the next case would paas when again we re asking it to pass when length are equal ??
		if data.len() <= CHARSET_LE.len()
		{
			return parserutils::PARSERUTILS_BADPARAM;
		}

		// if (memcmp(copy data, copy CHARSET_LE, CHARSET_LE.len()) == 0) 
		 //if (memcmp(data, CHARSET_LE, CHARSET_LE.len()) == 0) 

		let retVal : int = lcss::memcmp(data, CHARSET_LE, CHARSET_LE.len());
		io::println(fmt!("value of retVal is %?", retVal));
		if (retVal == 0) 
		{
			io::println("\n Inside CHARSET_LE 32 bit");

			io::println("\n 1 ");
			let startIndex : uint = data.len() + CHARSET_LE.len();
			let mut endIndex : uint = startIndex;

			io::println("\n 2 ");
			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;

			io::println("\n 3 ");

			//io::println(fmt!( ));	
			while endIndex < (CHARSET_LE.len() -1)
			{
				io::println("\n Sushanta1: while loop");
				let value1 : u8 = data[endIndex] | data[endIndex + 1] << 8 | data[endIndex + 2] << 16 | data[endIndex + 3] << 24 ;
		
				if value1 > 0x007f
				{
					break;
				}	

				if (value1 == '"' as u8) && (endIndex < data.len() + CHARSET_LE.len() - 8)		
				{
					let value2 = data[endIndex + 4] | data[endIndex + 5] << 8 | data[endIndex + 6] << 16 | data[endIndex + 7] << 24 ;

					if value2 == ';' as u8	
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u8 && value1 <= 'z' as u8
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				endIndex += 4;	
			} // while loop ends		
			
			// After while loop ends
			if (endIndex == data.len() - 4)
			{
				return parserutils::PARSERUTILS_NEEDDATA;
			}


			// Convert to MIB enum 
			unsafe {
				charset = self.lpu_instance.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_LE.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 

				charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16") ||

				charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{

				charset = 0;
			}
		}
		
		 let retVal2 : int = lcss::memcmp(data, CHARSET_BE, CHARSET_LE.len());
		 io::println(fmt!("value of retVal is %?", retVal2));
		 if (retVal2 == 0) 
		{
			io::println("\n 11: If condition passed ");

			let startIndex : uint = CHARSET_BE.len() - 1;
			let mut endIndex : uint = startIndex;

			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;
			
			// Is this condition right ?
			while (endIndex < (data.len() - 4))
			{
				let value1 : u8 = data[endIndex + 3] | data[endIndex + 2] << 8 | data[endIndex + 1] << 16 | data[endIndex] << 24 ;
				
				if value1 > 0x007f
				{
					break;
				}	

				// Is this condition right ?
				if (value1 == '"' as u8) && (endIndex < data.len() + CHARSET_BE.len() - 8)		
				{
					let value2 = data[endIndex + 7] | data[endIndex + 6] << 8 | data[endIndex + 5] << 16 | data[endIndex + 4] << 24 ;

					if value2 == ';' as u8	
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u8 && value1 <= 'z' as u8
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				// termination conditioning for while loop	
				endIndex += 4;	

			} // while loop ends

			if (endIndex == data.len() - 4)
			{
				return parserutils::PARSERUTILS_NEEDDATA;
			}

			// Convert to MIB enum 
			unsafe {
				charset = self.lpu_instance.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_BE.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 
			    charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16") ||
			    charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{
				charset = 0;
			}
		}
		else
		{
			io::println("\n Sushanta1: Inside NOWHERE, means ERROR ");
		}
		
		PARSERUTILS_CHARSET_TRY_OK(@4)
	}

	pub fn try_utf16_charset(&self, data : &[u8]) -> parserutils::parserutils_result {
	//pub fn try_utf16_charset(data : &[u8]) -> parserutils::parserutils_result {
		let mut charset: u16 = 0;
		let CHARSET_BE : &[u8] = ['0' as u8, '@' as u8, '0' as u8, 'c' as u8, '0' as u8, 'h' as u8, '0' as u8, 'a' as u8, '0' as u8, 'r' as u8, '0' as u8, 's' as u8, '0' as u8, 'e' as u8, '0' as u8, 't' as u8, '0' as u8, ' ' as u8,'0' as u8, '"' as u8] ; 
		
		let CHARSET_LE : &[u8] = ['@' as u8, '0' as u8, 'c' as u8, '0' as u8, 'h' as u8, '0' as u8, 'a' as u8, '0' as u8, 'r' as u8, '0' as u8, 's' as u8, '0' as u8, 'e' as u8, '0' as u8, 't' as u8, '0' as u8, ' ' as u8, '0' as u8, '"' as u8, '0' as u8] ; 

		io::println("\n Sushanta1: Inside CHARSET_LE 16 bit");
		io::println(fmt!("value of data.len() is %?", data.len()));
		io::println(fmt!("value of CHARSET_LE.len() is %?", CHARSET_LE.len()));
		io::println(fmt!("value of CHARSET_BE.len() is %?", CHARSET_BE.len()));

		io::println(fmt!("value of data is %?", data));
		io::println(fmt!("value of CHARSET_LE is %?", CHARSET_LE));
		io::println(fmt!("value of CHARSET_BE is %?", CHARSET_BE));
		
		if data.len() <= CHARSET_LE.len()
		{
			return parserutils::PARSERUTILS_BADPARAM;
		}

		if (lcss::memcmp(data, CHARSET_LE, CHARSET_LE.len()) == 0) 
		{
			io::println("Sushanta1: Inside CHARSET_LE 16 bits ");

			let startIndex : uint = CHARSET_LE.len() - 1 ;
			let mut endIndex : uint = startIndex;

			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;

			while endIndex < (data.len()- 2)
			{
				io::println("Sushanta1: Inside while loop for CHARSET_LE 16 bits ");
				let value1 : u16 = (data[endIndex] | data[endIndex + 1] << 8) as u16 ;

				/*	
				if value1 > 0x007f
				{
					break;
				}	
				*/

				if (value1 == '"' as u16) && (endIndex < data.len() + CHARSET_LE.len() - 4)		
				{
					let value2 : u16 = (data[endIndex + 2] | data[endIndex + 3] << 8) as u16 ;

					if value2 == ';' as u16	
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u16 && value1 <= 'z' as u16
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				// termination conditioning for while loop	
				endIndex += 2;	
			} // while loop ends		
			
			// After while loop ends
			if (endIndex == data.len() + CHARSET_LE.len() - 2)
			{
				return parserutils::PARSERUTILS_NEEDDATA;
			}


			// Convert to MIB enum 
			unsafe {
				charset = self.lpu_instance.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_LE.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 

				charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16") ||

				charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{

				charset = 0;
			}
		 }	
		else if (lcss::memcmp(data, CHARSET_BE, CHARSET_BE.len()) == 0) 
		{
			io::println("Sushanta1: Inside CHARSET_BE 16 bits ");

			let startIndex : uint = (CHARSET_BE.len() - 1);
			let mut endIndex : uint = startIndex;

			io::println(fmt!("value of startIndex is %?", startIndex));
			
			// values are only for initialization
			let mut buffMemory : ~[u8] = ~[ '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8, '0' as u8 ];
			let mut buffMemoryIndex : u8 = 0;

			while endIndex < (data.len() - 2)
			{
				io::println("Sushanta1: Inside while loop for CHARSET_BE 16 bits ");

				io::println(fmt!("value of data[endIndex] is %?", data[endIndex]));
				io::println(fmt!("value of data[endIndex + 1] is %?", data[endIndex + 1]));
				
				io::println(fmt!("value of data[endIndex + 1]<<8 is %?", data[endIndex + 1]<<8));
				io::println(fmt!("value of data[endIndex]<<8 is %?", data[endIndex]<<8));
				io::println(fmt!("value of data[endIndex+1] | data[endIndex]<<8 is %?", data[endIndex] | data[endIndex]<<8));

				// Since it is Big-endian, data at MSB would be at lower address space
				/*
				let value1 : u16 = (data[endIndex + 1] | data[endIndex] << 8) as u16 ;
				io::println(fmt!("value of value1 is %?", value1));
				*/		
				
				let mut value1 : u16 = data[endIndex] as u16;
				io::println(fmt!("value of value1 is %?", value1));
				value1 = value1 << 8;
				io::println(fmt!("value of value1 is %?", value1));
				value1 = value1 + data[endIndex+1] as u16;
				io::println(fmt!("value of value1 is %?", value1));
				io::println(fmt!("value of 0x007f is %?", 0x007f));

				// value1 is getting bigger val then 0x007f
				/*
				if value1 > 0x007f
				{
					io::println("Sushanta1: value1 > 0x007f is satisfied, Going to break...");
					break;
				}	
				*/

				if (value1 == '"' as u16) && (endIndex < data.len() - 4)		
				{
					io::println(" CONDITION is passed...");
					let value2 = (data[endIndex + 3] | data[endIndex + 2] << 8) as u16;

					if value2 == ';' as u16
					{
						break;
					}
				}			
			
				if buffMemoryIndex < buffMemory.len() as u8
				{
					if value1 >= 'a' as u16 && value1 <= 'z' as u16
					{
						buffMemory[buffMemoryIndex] = (value1 & !0x20) as u8;			
					}
					else
					{
						buffMemory[buffMemoryIndex];	
					}
					buffMemoryIndex += 1;	
				}	
				
				// termination conditioning for while loop	
				endIndex += 2;	
			} // while loop ends		
			
			if (endIndex == data.len()- 2)
			{
				return parserutils::PARSERUTILS_NEEDDATA;
			}

			io::println(" Outside while loop ...");

			// Convert to MIB enum 
			unsafe {
				io::println(" B4 condn in UNSAFE...");
				charset = self.lpu_instance.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET_BE.len(), data.len()-1)) , data.len()-1 ) );
				io::println(" After condn in UNSAFE...");
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 
			    charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16") ||
			    charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{
				charset = 0;
			}
		}// else if terminates
		else
		{
			io::println("\n Sushanta1: Inside NOWHERE 16 BITS means ERROR ");
		}

		PARSERUTILS_CHARSET_TRY_OK(@4)
	}


	// Sushanta: This is a sample implementation
	pub fn  try_ascii_compatible_charset(&self, data : ~[u8]) -> parserutils::parserutils_result {

		let mut charset : u16 = 0;
		let CHARSET : ~[u8] = ~[ '@' as u8, 'c' as u8, 'h' as u8, 'a' as u8 , 'r' as u8, 's' as u8, 'e' as u8, 't' as u8, ' ' as u8 , '\"'  as u8] ;

		io::println("\n Sushanta1: Inside ASCII fun");
		io::println(fmt!("value of data.len() is %?", data.len()));
		io::println(fmt!("value of CHARSET.len() is %?", CHARSET.len()));
		
		io::println(fmt!("value of data is %?", data));
		io::println(fmt!("value of CHARSET is %?", CHARSET));

		if (data.len() <= CHARSET.len() ) {
			return parserutils::PARSERUTILS_NEEDDATA;
		}

		// Look for @charset, assuming ASCII-compatible source data 
		//if ( memcmp(data, CHARSET, CHARSET.len() ) == 0) 
		 let retVal : int = lcss::memcmp(data, CHARSET, CHARSET.len());
		 io::println(fmt!("value of retVal is %?", retVal));
		 if (retVal == 0) 
		{
			io::println("INSIDE ASCII if condition ");

			let mut indexVal = CHARSET.len()-1;
			io::println(fmt!("value of indexVal at CHARSETlen() is %?", indexVal));

			// Looking for "; at the end of charset declaration
			while (indexVal < data.len()) 
			{
				io::println(fmt!("value of indexVal is %?", indexVal));
				io::println(fmt!("value of data[indexVal] is %?", data[indexVal]));

				//if data[indexVal] == ('"' as u8) && data[indexVal+1] == (';' as u8) && indexVal < (data.len()-1)  
				if data[indexVal] == ('"' as u8) && data[indexVal+1] == (';' as u8) && indexVal < (data.len())  
				{
					io::println(fmt!(" 3. Going to break with indexVal is %?", indexVal));
					break ;
				}
				indexVal = indexVal + 1 ;
			}

			// if this condition is true then, the input CSS file doesn't have anything except <charset>  string
			if indexVal == data.len() {
				io::println("INSIDE parserutils::PARSERUTILS_NEEDDATA error block");
				return parserutils::PARSERUTILS_NEEDDATA;
			}
			io::println("OUTSIDE parserutils::PARSERUTILS_BADPARAM error block");

			// Convert to MIB enum 
			unsafe {
				charset = self.lpu_instance.parserutils_charset_mibenum_from_name(
						from_buf_len(to_const_ptr(copy data.slice(CHARSET.len(), data.len()-1)) , data.len()-1 ) );
			}

			// Any non-ASCII compatible charset must be ignored, as
			// we've just used an ASCII parser to read it. 
			if (charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32") ||  charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32LE") || 

				charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-32BE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16") ||

				charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16LE") || charset == self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-16BE") ) 
			{

				charset = 0;
			}
		}
		else
		{
			io::println("INSIDE ASCII no where, means ERROR");
		}
		PARSERUTILS_CHARSET_TRY_OK(@4)
	}


	pub fn css_charset_read_bom_or_charset(&self, data : ~[u8], mibenum : ~u16 ) -> parserutils::parserutils_result {

		let mut err : parserutils::parserutils_result ;
		let mut charset : u16  = 0;
		let mut parser : @lpu = lpu();


		if (data.len()<4) {
			return parserutils::PARSERUTILS_BADPARAM;
		}


		// Look for BOM 
		if (data[0] == 0x00 && data[1] == 0x00 && 
				data[2] == 0xFE && data[3] == 0xFF) {
			charset = parser.parserutils_charset_mibenum_from_name(~"UTF-32BE");
		} else if (data[0] == 0xFF && data[1] == 0xFE &&
				data[2] == 0x00 && data[3] == 0x00) {
			charset = parser.parserutils_charset_mibenum_from_name(~"UTF-32LE");
		} else if (data[0] == 0xFE && data[1] == 0xFF) {
			charset = parser.parserutils_charset_mibenum_from_name(~"UTF-16BE");
		} else if (data[0] == 0xFF && data[1] == 0xFE) {
			charset = parser.parserutils_charset_mibenum_from_name(~"UTF-16LE");
		} else if (data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF) {
			charset = parser.parserutils_charset_mibenum_from_name(~"UTF-8");
		}

		if (charset!=0) {
			return parserutils::PARSERUTILS_CHARSET_TRY_OK(@charset);
		}
		
		err = self.try_utf32_charset(copy data);
		// Sushanta
		//err = try_utf32_charset(data, parser);
		match(err) {
			PARSERUTILS_CHARSET_TRY_OK(x) => return err ,
			_ => {}	
		}

		err = self.try_utf16_charset(copy data);
		//err = try_utf16_charset(data,parser);
		match(err) {
			PARSERUTILS_CHARSET_TRY_OK(x) => return err ,
			_ => {}	
		}
		
		return self.try_ascii_compatible_charset(copy data/*,parser*/);
	}


	pub fn css__charset_extract(&self,  data : ~[u8] ,
			mibenum : ~u16 , source : ~u32) -> parserutils::parserutils_result {

		let mut err : parserutils::parserutils_result;
		let mut charset : @u16 = @0;
		let mut src : @u32 = @0 ;

		if (data.len()==(0 as uint))  || mibenum==~(0 as u16) || source==~(0 as u32) {
			return parserutils::PARSERUTILS_BADPARAM ;
		}

		// If the charset was dictated by the client, we've nothing to detect 
		if source==~4 /*CSS_CHARSET_DICTATED*/ {
			charset=@*mibenum ;
			return parserutils::PARSERUTILS_CHARSET_EXT_OK((charset,@4));
		}

		// Look for a BOM and/or @charset 
		err = self.css_charset_read_bom_or_charset(data, copy ~*charset);
		match(err) {
			PARSERUTILS_CHARSET_TRY_OK(x) => {} ,
			_ => return parserutils::PARSERUTILS_BADPARAM	
		}

		if charset!=@0 {
			//mibenum = charset;
			src = @3 ; // CSS_CHARSET_DOCUMENT;
			return parserutils::PARSERUTILS_CHARSET_EXT_OK((charset,src));
		}

		// If we've already got a charset from the linking mechanism or 
		//  referring document, then we've nothing further to do 
		if source!=~0 /* CSS_CHARSET_DEFAULT */ {
			src=@*source;
			return parserutils::PARSERUTILS_CHARSET_EXT_OK((charset,src));
		}

		// We've not yet found a charset, so use the default fallback 
		charset = @self.lpu_instance.parserutils_charset_mibenum_from_name(~"UTF-8");

		if charset==@0 {
			return parserutils::PARSERUTILS_BADENCODING ;
		}

		//mibenum = charset ;
		src = @0 ; // CSS_CHARSET_DEFAULT;

		return parserutils::PARSERUTILS_CHARSET_EXT_OK((charset,src));
	}
	
	static pub fn css_result_to_string(css_err : css_result ) -> ~str {
		let mut result : ~str ;


		match css_err {
		 CSS_OK =>
			result = ~"No error",

		 CSS_NOMEM =>
			result = ~"Insufficient memory",

		 CSS_BADPARM => 
			result = ~"Bad parameter",

		 CSS_INVALID =>
			result = ~"Invalid input",

		 CSS_FILENOTFOUND =>
			result = ~"File not found",

		 CSS_NEEDDATA =>
			result = ~"Insufficient data",

		 CSS_BADCHARSET => 
			result = ~"BOM and charset mismatch",

		 CSS_EOF => 
			result = ~"EOF encountered",
		
		 CSS_IMPORTS_PENDING =>
			result = ~"Imports pending",
			
		CSS_PROPERTY_NOT_SET =>
			result = ~"Property not set",
			/*CSS_LWC_INTERN_STRING_OK(temp)
			result= ~"intern string returned"*/
			
		}

		result
	}



}
