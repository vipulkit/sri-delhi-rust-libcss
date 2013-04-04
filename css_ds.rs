#[link(name = "css_ds", vers = "0.1")];
#[crate_type = "lib"];

extern mod parserutils;
extern mod wapcaplet;
extern mod css_enum ;

use wapcaplet::*;
use parserutils::* ;
use css_enum::* ;


// =======================================================
// Enum
// =======================================================


// errors.h 
pub enum css_result {
		//CSS_OK  ,
		CSS_LANGUAGE_CREATED_OK(@mut css_language),
		CSS_STYLESHEET_CREATE_OK(@css_stylesheet),
		CSS_STRING_GET(@lwc_string),
		CSS_STRING_ADD_OK(@mut u32),
		CSS_RULE_CREATED_OK(@css_high_level),
		CSS_IMPORTS_PENDING_OK(@lwc_string,u64),
		CSS_GET_LANGUAGE_LEVEL(css_language_level),
		CSS_GET_URL(~str),
		CSS_GET_TITLE(~str),
		CSS_IS_QUIRK_ALLOWED(bool),
		CSS_IS_QUIRK_USED(bool),
		CSS_GET_SHEET_DISABLED(bool),
		CSS_STYLECREATED_OK(@css_style_Node),
		/*CSS_RULE_SELECTOR_CREATED( @css_rule_selector),
		CSS_RULE_CHARSET_CREATED(@css_rule_charset),
		CSS_RULE_IMPORT_CREATED(@css_rule_import),
		CSS_RULE_MEDIA_CREATED(@css_rule_media),
		CSS_RULE_FONT_FACE_CREATED(@css_rule_font_face),
		CSS_RULE_PAGE_CREATED(@css_rule_page),*/
		CSS_GENERAL_OK,
		CSS_LANGUAGE_CREATED(@mut css_language),
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


enum css_style_Node{
	SomeStyleNode(@css_style),
	NoStyleNode
}

enum css_parser_node
{
	SomeParserNode(@ lcss_parser),
  	NoParserNode
}

pub enum css_high_level_ptr
{
	high_level_pointer(@mut css_high_level),
	no_high_level_pointer
}

enum css_rule_node
{
	SomeRuleNode(@mut css_rule),
  	NoRuleNode
}

pub enum StyleSheetNode
{
	SomeStyleSheetNode(@mut css_stylesheet),
	NoStyleSheetNode
}

pub enum css_language_node
{
	SomeLanguageNode(@mut css_language),
  	NoLanguageNode
}




// =======================================================
// Structs 
// =======================================================

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

pub type css_fixed = int;


pub struct line_height_t{                  
	size:css_fixed,           
	unit:css_unit,
}


pub struct css_system_font {
	style: css_font_style_e,
	variant: css_font_variant_e,
	weight:  css_font_weight_e,
	size:uint,
	line_height:line_height_t,
	/* Note: must be a single family name only */
	family: @lwc_string
}


pub type css_code_t  = uint;
pub struct css_style{

	bytecode:~[css_code_t] ,
	//used : u32,
	//allocated: u32,
	 sheet:@StyleSheetNode
}

pub struct css_selector_detail_value {
	string:~str,		/*< Interned string, or NULL */
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


pub struct size_t{                  
	size:css_fixed,           
	unit:css_unit,
}


/* ///////////////////////////////////////////////////////////////////////////////////////////
 * ////////////////////////// bytecode.h /////////////////////////////////////////////////////
   //////////////////////////////////////////////////////////////////////////////////////////*/

/*
////////////////////////////////////////////////////////////////////////////////
pub struct struct_settings{
	encoding: u16 		
}    

pub struct parserutils_filter {
	int_enc: u16,              
	 settings : struct_settings ,
	 iconv_h : u64 ,
	 pw : ~[u8]
}
*/
/////////////////////////////////////////////////////////////////////////////////


struct css_namespace {
	prefix:@lwc_string,	/**< Namespace prefix */
	uri:@lwc_string		/*< Namespace URI */
}
 
pub struct context_entry {
	event_type:css_parser_event,		/**< Type of entry */
	data:@css_rule		/*< Data for context */
} 

pub struct lcss {
	 lwc_instance:@lwc,
	 lpu_instance:@lpu,
	 lcss_language:@css_language,
	 lcss_stylesheet:@css_stylesheet,
	 lcss_parser:@lcss_parser,
	 lcss_lexer:@lcss_lexer,
	//  propstrings_call_count:uint,
	//  propstrings_list:@[@str],
	//  propstrings:~[@lwc_string]
}


/*
 * Css parser event handler structure
 */ 
pub struct css_parser_event_handler_struct{
		// ToDo
		// handler:css_parser_event_handler,
		pw:@css_language
}

/*
 * Css parser opt paramemeters
 */
pub struct  css_parser_optparams {
	 quirks:bool,
	 event_handler: css_parser_event_handler_struct
	
} 

/**
 * Representation of a parser state
 */
pub struct parser_state
{
	state : u16 ,
	substate : u16 
}

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

pub struct ParseError {
    message: ~str,
}

pub struct lcss_lexer {
    priv transform_function_whitespace: bool,
    priv input: ~[u8],
    priv length: uint, // Counted in bytes, not characters
    priv  position: uint, // Counted in bytes, not characters
}

pub struct css_high_level
{
	 base:@css_rule,
	//rule_type : css_rule_type,
	 selector  : @css_rule_selector,
	 charset   : @css_rule_charset,
	 import    : @css_rule_import,
	 media     : @css_rule_media,
	 font_face : @css_rule_font_face,
	 page      : @css_rule_page,
	 prev      : @mut css_high_level_ptr,
	 next      : @mut css_high_level_ptr

}

struct css_selector {
	 combinator:~[@css_selector],		/*< Combining selector */
	 rule:@css_high_level_ptr ,				/*< Owning rule */
	 specificity:u32,			//< Specificity of selector 
     data:@css_selector_detail		/*< Selector data */
}



pub struct css_rule {
	parent:@rule_stylesheet,		
			/**< containing rule or owning 
						 * stylesheet (defined by ptype)
						 */
	next:@mut css_rule_node ,				/**< next in list */
	prev:@mut css_rule_node ,				/**< previous in list */

	 rule_type  :  css_rule_type,		/**< css_rule_type */
	 index : uint,		/**< index in sheet */
	 items : uint,		/**< # items in rule */
	 ptype : uint		/*< css_rule_parent_type */
}

pub struct css_rule_selector {
	 // base:css_rule,

	  selectors:~[@css_selector],
	 style:css_style 
}
pub struct css_rule_media {
	//base:css_rule ,

	media:u64,

	first_child:@mut  css_rule_node,
	last_child:@mut  css_rule_node
}


// ===========================================================================================================
// CSS-STYLESHEET implementation/data-structs start here 
// ===========================================================================================================


pub struct css_rule_font_face {
	//base :css_rule,

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
	bits:~[u8]
	
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
	bits:~[u8]
}

pub struct css_rule_page {
	//base:css_rule ,

	selector:@css_selector ,
	style:@css_style 
}

pub struct css_rule_import {
	//base:css_rule ,

	 url:@lwc_string,
	 media:u64,

	 sheet:@mut StyleSheetNode
}

pub struct css_rule_charset {
	//base:css_rule ,

	encoding:@lwc_string	/* \todo use MIB enum? */
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
		 allow_quirks:bool,

	/** This stylesheet is an inline style */
		 inline_style:bool,

	/** URL resolution function */
		 resolve : @extern fn (pw:~[u8],base:~str, rel:@lwc_string, abs: @lwc_string) -> css_result,

	/** Client private data for resolve */
		 resolve_pw:~[u8],

	/** Import notification function */
		 import: @extern fn (pw:~[u8], parent:@css_stylesheet, url:@lwc_string, media:u64) -> css_result,

	/** Client private data for import */
		 import_pw:~[u8],

	/** Colour resolution function */
		 color: @extern fn (pw:~[u8], name:@lwc_string, color:@css_color) -> css_result,

	/** Client private data for color */
		 color_pw:~[u8],

	/** Font resolution function */
		 font: @extern fn(pw:~[u8], name:@lwc_string, system_font:@css_system_font) -> css_result ,

	/** Client private data for font */
		 font_pw: ~[u8]
}

pub struct css_stylesheet {
	//selectors:@css_selector_hash,	TODO REPLACE WITH BUILT IN HASH TABLE
		/* < Hashtable of selectors */
	 lwc_instance:@lwc,
    //parser_instance:@lcss_parser,
	 rule_count:u32,			/**< Number of rules in sheet */
	 rule_list:@mut css_high_level_ptr ,			/**< List of rules in sheet */
	 last_rule:@mut css_high_level_ptr,			/**< Last rule in list */

	 disabled:bool,				/**< Whether this sheet is 
						 * disabled */

	 url:~str,				/**< URL of this sheet */
	 title:~str,			/**< Title of this sheet */

	 level:css_language_level ,		/**< Language level of sheet */
	 parser:@mut css_parser_node ,			/**< Core parser for sheet */
	 parser_frontend:@mut css_language_node,			/**< Frontend parser */////////look for type
	//propstrings:@ mut[@lwc_string ],		/**< Property strings, for parser */

	 quirks_allowed:bool,			/**< Quirks permitted */
	 quirks_used:bool,			/**< Quirks actually used */

	 inline_style:bool,			/**< Is an inline style */

	 size:uint,				/**< Size, in bytes */

	// TODO
	//   import:css_import_notification_fn,	/**< Import notification function */
	 import_pw:~[u8],			/**< Private word *////////look for type

	// TODO
	//  resolve:css_url_resolution_fn,		/**< URL resolution function */
	 resolve_pw:~[u8],			/**< Private word *////////look for type

		// TODO
	//  color:css_color_resolution_fn,		/**< Colour resolution function */
	 color_pw:~[u8],				/**< Private word *////////look for type

	/** System font resolution function */
		//TODO 
	//  font:css_font_resolution_fn,		
	 font_pw:~[u8],				/**< Private word *////////look for type


	// alloc:css_allocator_fn,			/**< Allocation function */
	//pw:~[u8],				/**< Private word */
  
	 cached_style:@ css_style_Node ,		/**< Cache for style parsing */
  
	 string_vector:~[@lwc_string],            /**< Bytecode string vector */
	//string_vector_l:u32,              /**< The string vector allocated
					// * length in entries */
	//string_vector_c:u32,               /*< The number of string * vector entries used */ 
	 propstrings_call_count:uint,
     propstrings_list:@[@str],
	 propstrings:~[@lwc_string]					 
}

pub struct css_language {
	sheet:@css_stylesheet ,		/**< The stylesheet to parse for */
	 lwc_instance:@lwc,
//#define STACK_CHUNK 32
    STACK_CHUNK:int,
	context:@[context_entry],      //parseutils_stack	/**< Context stack */

	 state:language_state,			/**< State flag, for at-rule handling */

	/** Interned strings */
	 strings: ~[@lwc_string ],

	default_namespace:@lwc_string ,	/**< Default namespace URI */
	namespaces:@css_namespace,	/**< Array of namespace mappings */
	num_namespaces:u32	/*< Number of namespace mappings */

	// css_allocator_fn alloc;		*< Memory (de)allocation function 
	// void *pw;			/**< Client's private data */
}

pub struct important 
{
	 lpu_instance : @lpu,
	     lwc_instance : @lwc,
}

pub struct css_rule_face {
	base :css_rule,
	 font_face:@css_font_face 
}

pub struct font_face
{
     lpu_instance : @lpu,
     lwc_instance : @lwc,
}

// ===========================================================================================================
// CSS-SELECT-HINT implementation/data-structs start here 
// ===========================================================================================================


/**
 * Length object for use in presentational hints
 */
struct css_hint_length {
	value:css_fixed,
	unit:css_unit
}

/**
 * Presentational hints
 */

struct positionStruct{
	h:css_hint_length, 
	v:css_hint_length
}

struct dataStruct {
 	clip:@css_computed_clip_rect,
	color:css_color,
	content:@css_computed_content_item,
	counter:@css_computed_counter,
	fixed:css_fixed,
	integer:i32,
	length:css_hint_length,
	position: positionStruct,
	string: lwc_string,
	strings:@lwc_string
} 

struct css_hint {
	/* Ownership of all data is passed to libcss */
	data : dataStruct,
	status:u8
} 


// ===========================================================================================================
// CSS-SELECT-HINT implementation/data-structs ends here 
// ===========================================================================================================


// ===========================================================================================================
// CSS-SELECT-COMPUTED implementation/data-structs start here 
// ===========================================================================================================


struct css_computed_counter {
	name:@lwc_string,
	value:css_fixed
}


struct css_computed_clip_rect {
	top:css_fixed,
	right:css_fixed,
	bottom:css_fixed,
	left:css_fixed,

	tunit:css_unit,
	runit:css_unit,
	bunit:css_unit,
	lunit:css_unit, 

	top_auto:bool,
	right_auto:bool,
	bottom_auto:bool,
	left_auto:bool
} 

struct counterStruct{
	name:@lwc_string,
	style:u8
}

struct countersStruct{
	name:@lwc_string,
	sep:@lwc_string,
	style:u8
} 

struct dateStruct{
	string:@lwc_string, 
	uri:@lwc_string,
	attr:@lwc_string,
	counter : counterStruct,
	counters: countersStruct
}

struct css_computed_content_item {
	content_type:u8,
	date: dateStruct
}


struct css_computed_uncommon {
/*
 * border_spacing		  1 + 2(4)	  2(4)
 * clip				  2 + 4(4) + 4	  4(4)
 * letter_spacing		  2 + 4		  4
 * outline_color		  2		  4
 * outline_width		  3 + 4		  4
 * word_spacing			  2 + 4		  4
 * 				---		---
 * 				 52 bits	 40 bytes
 *
 * Encode counter_increment and _reset as an array of name, value pairs,
 * terminated with a blank entry.
 *
 * counter_increment		  1		  sizeof(ptr)
 * counter_reset		  1		  sizeof(ptr)
 * 				---		---
 * 				  2 bits	  2sizeof(ptr) bytes
 *
 * Encode cursor uri(s) as an array of string objects, terminated with a
 * blank entry.
 *
 * cursor			  5		  sizeof(ptr)
 * 				---		---
 * 				  5 bits	  sizeof(ptr) bytes
 *
 * Encode content as an array of content items, terminated with a blank entry.
 *
 * content			  2		  sizeof(ptr)
 * 				---		---
 * 				  2 bits	  sizeof(ptr)
 *
 * 				___		___
 * 				 61 bits	 40 + 4sizeof(ptr) bytes
 *
 * 				  8 bytes	 40 + 4sizeof(ptr) bytes
 * 				===================
 * 				 48 + 4sizeof(ptr) bytes
 *
 * Bit allocations:
 *
 *    76543210
 *  1 llllllcc	letter-spacing | outline-color
 *  2 ooooooob	outline-width  | border-spacing
 *  3 bbbbbbbb	border-spacing
 *  4 wwwwwwir	word-spacing   | counter-increment | counter-reset
 *  5 uuuuu...	cursor         | <unused>
 *  6 cccccccc	clip
 *  7 cccccccc	clip
 *  8 ccccccoo	clip           | content
 */
	bits:[u8, ..8],

	border_spacing:[css_fixed, ..2],

	clip:[css_fixed, ..4],

	letter_spacing:css_fixed,

	outline_color:css_color,
	outline_width:css_fixed,

	word_spacing:css_fixed,

	counter_increment:@css_computed_counter,
	counter_reset:@css_computed_counter,

	cursor:@lwc_string,

	content:@css_computed_content_item
} 


 struct css_computed_page {
/*
 * Bit allocations:
 *
 *    76543210
 *  1 aaabbbii	page_break_after | page_break_before | page_break_inside
 *  2 ......wo	widows  | orphans
 */
	// bits:[u8*2],
	bits:[u8, ..2],
	
	widows:css_fixed,
	orphans:css_fixed
}



struct css_computed_style {
/*
 * background_attachment	  2
 * background_repeat		  3
 * border_collapse		  2
 * border_top_style		  4
 * border_right_style		  4
 * border_bottom_style		  4
 * border_left_style		  4
 * caption_side			  2
 * clear			  3
 * direction			  2
 * display			  5
 * empty_cells			  2
 * float			  2
 * font_style			  2
 * font_variant			  2
 * font_weight			  4
 * list_style_position		  2
 * list_style_type		  4
 * overflow			  3
 * outline_style		  4
 * position			  3
 * table_layout			  2
 * text_align			  4
 * text_decoration		  5
 * text_transform		  3
 * unicode_bidi			  2
 * visibility			  2
 * white_space			  3
 *				---
 *				 84 bits
 *
 * Colours are 32bits of AARRGGBB
 * Dimensions are encoded as a fixed point value + 4 bits of unit data
 *
 * background_color		  2		  4
 * background_image		  1		  sizeof(ptr)
 * background_position		  1 + 2(4)	  2(4)
 * border_top_color		  2		  4
 * border_right_color		  2		  4
 * border_bottom_color		  2		  4
 * border_left_color		  2		  4
 * border_top_width		  3 + 4		  4
 * border_right_width		  3 + 4		  4
 * border_bottom_width		  3 + 4		  4
 * border_left_width		  3 + 4		  4
 * top				  2 + 4		  4
 * right			  2 + 4		  4
 * bottom			  2 + 4		  4
 * left				  2 + 4		  4
 * color			  1		  4
 * font_size			  4 + 4		  4
 * height			  2 + 4		  4
 * line_height			  2 + 4		  4
 * list_style_image		  1		  sizeof(ptr)
 * margin_top			  2 + 4		  4
 * margin_right			  2 + 4		  4
 * margin_bottom		  2 + 4		  4
 * margin_left			  2 + 4		  4
 * max_height			  2 + 4		  4
 * max_width			  2 + 4		  4
 * min_height			  1 + 4		  4
 * min_width			  1 + 4		  4
 * padding_top			  1 + 4		  4
 * padding_right		  1 + 4		  4
 * padding_bottom		  1 + 4		  4
 * padding_left			  1 + 4		  4
 * text_indent			  1 + 4		  4
 * vertical_align		  4 + 4		  4
 * width			  2 + 4		  4
 * z_index			  2		  4
 * 				---		---
 *				181 bits	140 + 2sizeof(ptr) bytes
 *
 * Encode font family as an array of string objects, terminated with a 
 * blank entry.
 *
 * font_family			  3		  sizeof(ptr)
 * 				---		---
 * 				  3 bits	  sizeof(ptr)
 *
 * Encode quotes as an array of string objects, terminated with a blank entry.
 *
 * quotes			  1		  sizeof(ptr)
 * 				---		---
 * 				  1 bit		  sizeof(ptr) bytes
 *
 * 				___		___
 *				269 bits	140 + 4sizeof(ptr) bytes
 *
 *				 34 bytes	140 + 4sizeof(ptr) bytes
 *				===================
 *				174 + 4sizeof(ptr) bytes
 *
 * Bit allocations:
 *
 *    76543210
 *  1 vvvvvvvv	vertical-align
 *  2 ffffffff	font-size
 *  3 ttttttti	border-top-width    | background-image
 *  4 rrrrrrrc	border-right-width  | color
 *  5 bbbbbbbl	border-bottom-width | list-style-image
 *  6 lllllllq	border-left-width   | quotes
 *  7 ttttttcc	top                 | border-top-color
 *  8 rrrrrrcc	right               | border-right-color
 *  9 bbbbbbcc	bottom              | border-bottom-color
 * 10 llllllcc	left                | border-left-color
 * 11 hhhhhhbb	height              | background-color
 * 12 llllllzz	line-height         | z-index
 * 13 ttttttbb	margin-top          | background-attachment
 * 14 rrrrrrbb	margin-right        | border-collapse
 * 15 bbbbbbcc	margin-bottom       | caption-side
 * 16 lllllldd	margin-left         | direction
 * 17 mmmmmmee	max-height          | empty-cells
 * 18 mmmmmmff	max-width           | float
 * 19 wwwwwwff	width               | font-style
 * 20 mmmmmbbb	min-height          | background-repeat
 * 21 mmmmmccc	min-width           | clear
 * 22 tttttooo	padding-top         | overflow
 * 23 rrrrrppp	padding-right       | position
 * 24 bbbbbo..	padding-bottom      | opacity               | <unused>
 * 25 lllllttt	padding-left        | text-transform
 * 26 tttttwww	text-indent         | white-space
 * 27 bbbbbbbb	background-position
 * 28 bdddddff	background-position | display               | font-variant
 * 29 tttttfff	text-decoration     | font-family
 * 30 ttttrrrr	border-top-style    | border-right-style
 * 31 bbbbllll	border-bottom-style | border-left-style
 * 32 ffffllll	font-weight         | list-style-type
 * 33 oooottuu	outline-style       | table-layout          | unicode-bidi
 * 34 vvlltttt	visibility          | list-style-position   | text-align
 */
	//bits:[u8*34],
	bits:[u8, ..34],

	unused:[u8, ..2],

	background_color:css_color,
	background_image:@lwc_string,
	background_position:[css_fixed, ..2],

	border_color:[css_color, ..4],
	border_width:[css_fixed, ..4],

	top:css_fixed,
	right:css_fixed,
	bottom:css_fixed,
	left:css_fixed,

	color:css_color,

	font_size:css_fixed,

	height:css_fixed,

	line_height:css_fixed,

	list_style_image:@lwc_string,

	margin:[css_fixed, ..4],

	max_height:css_fixed,
	max_width:css_fixed,

	min_height:css_fixed, 
	min_width:css_fixed,

	opacity:css_fixed,

	padding:[css_fixed, ..4],

	text_indent:css_fixed,

	vertical_align:css_fixed,

	width:css_fixed,

	z_index:i32, 

	font_family:@lwc_string,

	quotes:@lwc_string, 

	uncommon:@css_computed_uncommon,/**< Uncommon properties */
	aural:~[u8],			/**< Aural properties */
	page:@css_computed_page	//< Page properties */
}

// ===========================================================================================================
// CSS-SELECT-COMPUTED implementation/data-structs ends here 
// ===========================================================================================================


// ===========================================================================================================
// CSS-SELECT implementation/data-structs start here 
// ===========================================================================================================


struct css_select_results {
	/**
	 * Array of pointers to computed styles, 
	 * indexed by css_pseudo_element. If there
	 * was no styling for a given pseudo element, 
	 * then no computed style will be created and
	 * the corresponding pointer will be set to NULL
	 */
	styles:@[@css_computed_style] //[CSS_PSEUDO_ELEMENT_COUNT];
}


struct css_select_handler {
	/** ABI version of this structure */
	handler_version: u32,

	node_name:@fn (pw:~[u8], node:~[u8],qname:~css_qname)->css_result,

	node_classes: @fn(pw:~[u8], node:~[u8],	classes:~[@lwc_string],n_classes:@u32)->css_result,

	node_id: @fn (pw:~[u8], node:~[u8],id:@lwc_string ) -> css_result,

	named_ancestor_node:@fn (pw:~[u8], node:~[u8], qname:@css_qname, ancestor:~[u8]) -> css_result,

	named_parent_node:@fn (pw:~[u8], node:~[u8],qname:@css_qname, parent:~[u8]) -> css_result,

	named_sibling_node:@fn (pw:~[u8], node:~[u8], qname:@css_qname, sibling:~[u8]) -> css_result,

	named_generic_sibling_node:@fn (pw:~[u8], node:~[u8], qname:@css_qname, sibling:~[u8]) -> css_result,

	parent_node:@fn (pw:~[u8], node:~[u8], parent:~[u8])->css_result,

	sibling_node:@fn(pw:~[u8], node:~[u8], sibling:~[u8]) -> css_result,

	node_has_name:@fn (pw:~[u8], node:~[u8],qname:@css_qname, matched:bool) -> css_result,

	node_has_class:@fn (pw:~[u8], node:~[u8],name:@lwc_string , matched:bool) -> css_result,

	node_has_id:@fn (pw:~[u8], node:~[u8],name:@lwc_string , matched:bool)-> css_result,

	node_has_attribute:@fn(pw:~[u8], node:~[u8], qname:@css_qname, matched:bool) -> css_result,

	node_has_attribute_equal:@fn(pw:~[u8], node:~[u8], qname:@css_qname, value:@lwc_string, matched:bool) -> css_result,

	node_has_attribute_dashmatch: @fn (pw:~[u8], node:~[u8], qname:@css_qname, value:@lwc_string, matched:bool) -> css_result,

	node_has_attribute_includes:@fn (pw:~[u8], node:~[u8],	qname:@css_qname, value:@lwc_string, matched:bool) -> css_result,

	node_has_attribute_prefix: @fn(pw:~[u8], node:~[u8],qname:@css_qname, value:@lwc_string, matched:bool) -> css_result,

	node_has_attribute_suffix: @fn (pw:~[u8], node:~[u8], qname:@css_qname, value:@lwc_string, matched:bool) -> css_result,

	node_has_attribute_substring: @fn (pw:~[u8], node:~[u8], qname:@css_qname, value:@lwc_string, matched:bool) -> css_result,

	node_is_root:@fn (pw:~[u8], node:~[u8], matched:bool) -> css_result,

	node_count_siblings:@fn (pw:~[u8], node:~[u8], same_name:bool, after:bool, count:@i32) -> css_result,

	node_is_empty:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_link:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_visited:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_hover:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_active:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_focus:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_enabled:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_disabled:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_checked:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_target:@fn (pw:~[u8], node:~[u8], matched:bool)-> css_result,

	node_is_lang:@fn (pw:~[u8], node:~[u8],value:@lwc_string, matched:bool)-> css_result,

	node_presentational_hint:@fn (pw:~[u8], node:~[u8], property:u32, hint:@css_hint)-> css_result,

	ua_default_for_property:@fn (pw:~[u8], property:u32, hint:@css_hint)-> css_result,

	compute_font_size:@fn (pw:~[u8], parent:@css_hint, size:@css_hint)-> css_result

}
/*
pub struct css_select {
	
}
*/

//TO DO: Should be moved to libwapcaplet
pub type lwc_hash = u32;
pub type lwc_refcounter = u32;



static CSS_SPECIFICITY_A:u32=0x01000000;
static CSS_SPECIFICITY_B:u32=0x00010000;
static CSS_SPECIFICITY_C:u32=0x00000100;
static CSS_SPECIFICITY_D:u32=0x00000001;


static MAX_UNICODE: char = '\U0010FFFF';

static ASCII_LOWER_OFFSET: char = 'a' - 'A';

static CSS_STYLE_DEFAULT_SIZE:u32 =16;
