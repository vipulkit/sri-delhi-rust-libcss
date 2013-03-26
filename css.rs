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


// ===========================================================================================================
// CSS Public APIs implementation/data-structs Starts here 
// ===========================================================================================================

/*
 * All public APIs and data-structures as exposed by libcss ( please see external include folder of libcss)
 * comes here , APIs and data-structs comes in this section need only to be public interfaces , 
 * rest can be private to this module
 * 
 * Mirror of include folder of libcss
 */

// functypes.h 
// TODO : css_allocator_fn 

// libcss.h 
// Done : nothing remaining 

// errors.h 
//To Do Should move to errors ---- Start
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

pub fn css_result_to_string(css_err : css_result ) -> ~str {

	let mut result : ~str = ~"" ;
	match css_err {
		CSS_IMPORTS_PENDING_OK(x,y)=>{result=~"import pending list created"}
		CSS_LANGUAGE_CREATED_OK(x)=> {result=~"language instance created successfully"},
		CSS_STYLESHEET_CREATE_OK(sheet)=>{result=~"stylesheet successfully created"},
		CSS_STRING_GET(x)=>{result = ~"get the string from vector part of stylesheet"},
		CSS_RULE_CREATED_OK(x) => {result=~"Css rule created successfully"},
		CSS_STRING_ADD_OK(x)  => {result = ~"string added to stylesheet successfully"}, 
		CSS_GET_LANGUAGE_LEVEL(x)=>{result= ~"get language level"},
		CSS_GET_URL(x)=>{result=~"get url"},
		CSS_GET_TITLE(x)=>{result=~"get title"},
		CSS_IS_QUIRK_ALLOWED(x)=>{result = ~"is Quirks allowed?"},
		CSS_IS_QUIRK_USED(x)=>{result= ~"IS_QUIRK_USED?"},
		CSS_GET_SHEET_DISABLED(x)=>{result=~"_GET_if-SHEET_DISABLED"},
		CSS_STYLECREATED_OK(x)=>{result=~"Style created successfully"}
		/*CSS_RULE_SELECTOR_CREATED(x) => {result=~"Css rule selector created successfully"},
		CSS_RULE_CHARSET_CREATED(x) => {result=~"Css rule charset created successfully"},
		CSS_RULE_IMPORT_CREATED(x) => {result=~"Css rule imported successfully"},
		CSS_RULE_MEDIA_CREATED(x) => {result=~"Css rule media created successfully"},
		CSS_RULE_FONT_FACE_CREATED(x) => {result=~"Css rule font-face created successfully"},
		CSS_RULE_PAGE_CREATED(x) => {result=~"Css rule page created successfully"},*/
		CSS_GENERAL_OK=> {result=~"Css Success "},
		CSS_LANGUAGE_CREATED(x) => {result=~"Css language created successfully"},
		CSS_PROPSTRINGS_OK(x) => {result=~"Css propstrings success "},
		CSS_NOMEM=> {result=~"Css error : No-memory"},
		CSS_BADPARM=> {result=~"Css error : bad-parameters "},
		CSS_INVALID=> {result=~"Css error : Invalid operation "},
		CSS_FILENOTFOUND=> {result=~"Css error : file not found"},
		CSS_NEEDDATA=> {result=~"Css error : need more data"},
		CSS_BADCHARSET=> {result=~"Css error : bad charset"},
		CSS_EOF=> {result=~"Css error : end of file "},
		CSS_IMPORTS_PENDING=> {result=~"Css imports pending "},
		CSS_PROPERTY_NOT_SET=> {result=~"Css property not set "},
		// _ => { result=~"Unknown error enumeration" },
	}
	result
}

// hint.h


// font-face.h

// fpmath.h


// types.h
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
// TODO : typedef of structs

// stylesheet.h
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






// select.h

pub enum css_pseudo_element {
	CSS_PSEUDO_ELEMENT_NONE         = 0,
	CSS_PSEUDO_ELEMENT_FIRST_LINE   = 1,
	CSS_PSEUDO_ELEMENT_FIRST_LETTER = 2,
	CSS_PSEUDO_ELEMENT_BEFORE       = 3,
	CSS_PSEUDO_ELEMENT_AFTER        = 4,

	CSS_PSEUDO_ELEMENT_COUNT	= 5	/* < Number of pseudo elements */
} 
// TODO : structs and typedefs

// computed.h

// properties.h
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


// ===========================================================================================================
// CSS Public APIs implementation/data-structs ends here 
// ===========================================================================================================


/*
 * This file is part of Rust-LibCSS.
 */

//TO DO: Should be moved to fpmath
type css_fixed = i32;



//TO DO: Should be moved to libwapcaplet
pub type lwc_hash = u32;
pub type lwc_refcounter = u32;




const CSS_SPECIFICITY_A:u32=0x01000000;
const CSS_SPECIFICITY_B:u32=0x00010000;
const CSS_SPECIFICITY_C:u32=0x00000100;
const CSS_SPECIFICITY_D:u32=0x00000001;






//pub type css_code_t  =  ~[u32];
enum css_style_Node{
	SomeStyleNode(@css_style),
	NoStyleNode
}
pub struct css_style{

	mut bytecode:~[css_code_t] ,
	//mut used : u32,
	//mut allocated: u32,
	mut sheet:@StyleSheetNode


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
// enum css_selector_node
// {
// 	SomeSelectorNode(@mut css_selector_node),
//   	NoSelectorNode
// }




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


enum css_parser_node
{
	SomeParserNode(@ lcss_parser),
  	NoParserNode
}





pub struct size_t{                  
	size:css_fixed,           
	unit:css_unit,
}

pub struct line_height_t{                  
	size:css_fixed,           
	unit:css_unit,
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
 


struct context_entry {
	event_type:css_parser_event,		/**< Type of entry */
	data:@css_rule		/*< Data for context */
} 


	
// ===========================================================================================================
// Lib CSS implementation/data-structs start here 
// ===========================================================================================================





pub struct lcss {
	mut lwc_instance:@lwc,
	mut lpu_instance:@lpu,
	mut lcss_language:@css_language,
	mut lcss_stylesheet:@css_stylesheet,
	mut lcss_parser:@lcss_parser,
	mut lcss_lexer:@lcss_lexer,
	// mut propstrings_call_count:uint,
	// mut propstrings_list:@[@str],
	// mut propstrings:~[@lwc_string]
}


pub fn lcss()->@lcss {
	let lwc_inst        = lwc();
	let lexer_inst      = lcss_lexer();
	let stylesheet_inst = lcss_stylesheet(lwc_inst);
	let language_inst   = lcss_language(stylesheet_inst);
	let parser_inst     = lcss_parser(lexer_inst,language_inst);
	@lcss {
		lwc_instance:lwc_inst,
		lpu_instance:lpu(),
		lcss_language:language_inst,
		lcss_stylesheet:stylesheet_inst,
		lcss_parser:parser_inst,
		lcss_lexer:lexer_inst,

		
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


}

 



// ===========================================================================================================
// Lib CSS- implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// CSS-PARSER implementation/data-structs Starts here 
// ===========================================================================================================


/*
 * Css parser events , sent during parsing
 */
pub enum css_parser_event {
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

/*
 * Css parser opt types
 */
pub enum css_parser_opttype {
	CSS_PARSER_QUIRKS,
	CSS_PARSER_EVENT_HANDLER
}

/*
 * Css parser event handler function pointer
 */

// null function for initializing
pub fn dummy_par_ev_hand(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:@css_language) -> css_result {
	CSS_GENERAL_OK
}


fn Stylesheet_event_handler(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:@css_language)-> css_result
{
	CSS_GENERAL_OK
}

pub type css_parser_event_handler =  @extern fn(css_intance:@lcss, event_type:css_parser_event, 
		tokens:~[~str] , pw:@css_language) -> css_result;

/*
 * Css parser event handler structure
 */
pub struct css_parser_event_handler_struct{
		mut  handler:css_parser_event_handler,
		mut pw:@css_language
}

/*
 * Css parser opt paramemeters
 */
pub struct  css_parser_optparams {
	mut quirks:bool,
	mut event_handler: css_parser_event_handler_struct
	
} 
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
/**
 * Major state numbers
 */
pub enum Parse_state_e {
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

	//mut quirks:bool	/* < Whether to enable parsing quirks */

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
	//mut event:@css_parser_event_handler,	/* < Client's event handler */
	mut event_pw:@css_language,		/* < Client data for event handler */
	mut quirks:bool,
	lcss_lexer_instance:@lcss_lexer,
	lparserutils_instance:@lpu
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
	CSS_GENERAL_OK
	}
	pub fn css__parser_create(&self)  {

	}
	pub fn css__parser_parse_chunk(&self, data:~[u8]) -> css_result{
     CSS_GENERAL_OK
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
		CSS_GENERAL_OK
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

	return CSS_GENERAL_OK;
}
}


// ===========================================================================================================
// CSS-PARSER implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// CSS-LEXER implementation/data-structs starts here 
// ===========================================================================================================
extern mod std;

const ASCII_LOWER_OFFSET: char = 'a' - 'A';

pub pure fn ascii_lower(string: &str) -> ~str {
    do str::map(string) |c| {
        match c {
            'A'..'Z' => c + ASCII_LOWER_OFFSET,
            _ => c,
        }
    }
}

pub struct ParseError {
    message: ~str,
}

pub enum NumericValue {
    Integer(int),
    Float(float),
}

pub enum Token {
    Ident(~str),
    Function(~str),
    AtKeyword(~str),
    Hash(~str),
    String(~str),
    BadString,
    URL(~str),
    BadURL,
    Delim(char),
    Number(NumericValue, ~str), // value, representation
    Percentage(NumericValue, ~str), // value, representation
    Dimension(NumericValue, ~str, ~str), // value, representation, unit
    UnicodeRange(char, char), // start, end
    EmptyUnicodeRange,
    WhiteSpace,
    CDO, // <!--
    CDC, // -->
    Colon, // :
    Semicolon, // ;
    OpenParenthesis, // (
    OpenSquareBraket, // [
    OpenCurlyBraket, // {
    CloseParenthesis, // )
    CloseSquareBraket, // ]
    CloseCurlyBraket, // }
    EOF,
}

const MAX_UNICODE: char = '\U0010FFFF';

pure fn preprocess(input: &str) -> ~str {
    // TODO: Is this faster if done in one pass?
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
        match $value { $pattern => true, _ => false }
    );
)

pub struct lcss_lexer {
    priv transform_function_whitespace: bool,
    priv input: ~[u8],
    priv length: uint, // Counted in bytes, not characters
    priv mut position: uint, // Counted in bytes, not characters
}

impl lcss_lexer {
    static fn from_vec(input: ~[u8], transform_function_whitespace: bool)
            -> ~lcss_lexer {
                let string_from_input = str::from_bytes(input);
                let string_from_input = preprocess(string_from_input);
                let input = str::to_bytes(string_from_input);
        ~lcss_lexer {
            length: input.len(),
            input: input,
            position: 0,
            transform_function_whitespace: transform_function_whitespace
        }
    }

    pub fn css__lexer_get_token(&self) -> (Token, Option<ParseError>) {
        if self.is_eof() { 
            (EOF, None) 
        }
        else { 
            self.consume_token()
        }
    }

    fn handle_transform_function_whitespace(&self, string: ~str)
            -> (Token, Option<ParseError>) {
        while !self.is_eof() {
            match self.current_char() {
                '\t' | '\n' | '\x0C' | ' ' => self.position += 1,
                '(' => { self.position += 1; return (Function(string), None) }
                _ => break,
            }
        }
        // Go back for one whitespace character.
        self.position -= 1;
        (Ident(string), None)
    }

    fn is_eof(&self) -> bool {
        self.position >= self.length
    }

    pub fn consume_token(&self) -> (Token, Option<ParseError>) {
        // Comments are special because they do not even emit a token,
        // unless they reach EOF which is an error.
        match self.consume_comments() {
            Some(result) => return result,
            None => ()
        }
        if self.is_eof() { return (EOF, None) }
        let c = self.current_char();
        match c {
            '-' => {
                if self.match_here(~"-->") {
                    self.position += 3;
                    (CDC, None)
                }
                else if self.next_is_namestart_or_escape() {
                    self.consume_ident()
                } else {
                    self.consume_numeric()
                }
            },
            '<' => {
                if self.match_here(~"<!--") {
                    self.position += 4;
                    (CDO, None)
                } else {
                    self.position += 1;
                    (Delim('<'), None)
                }
            },
            '0'..'9' | '.' | '+' => self.consume_numeric(),
            'u' | 'U' => self.consume_unicode_range(),
            'a'..'z' | 'A'..'Z' | '_' | '\\' => self.consume_ident(),
            _ if c >= '\x80' => self.consume_ident(), // Non-ASCII
            _ => {
                match self.consume_char() {
                    '\t' | '\n' | '\x0C' | ' ' => {
                        while !self.is_eof() {
                            match self.current_char() {
                                '\t' | '\n' | '\x0C' | ' '
                                    => self.position += 1,
                                _ => break,
                            }
                        }
                        (WhiteSpace, None)
                    },
                    '"' => self.consume_quoted_string(false),
                    '#' => self.consume_hash(),
                    '\'' => self.consume_quoted_string(true),
                    '(' => (OpenParenthesis, None),
                    ')' => (CloseParenthesis, None),
                    ':' => (Colon, None),
                    ';' => (Semicolon, None),
                    '@' => self.consume_at_keyword(),
                    '[' => (OpenSquareBraket, None),
                    ']' => (CloseSquareBraket, None),
                    '{' => (OpenCurlyBraket, None),
                    '}' => (CloseCurlyBraket, None),
                    _ => (Delim(c), None)
                }
            }
        }
    }

    fn consume_quoted_string(&self, single_quote: bool)
            -> (Token, Option<ParseError>) {
        let mut string: ~str = ~"";
        while !self.is_eof() {
            match self.consume_char() {
                '"' if !single_quote => return (String(string), None),
                '\'' if single_quote => return (String(string), None),
                '\n' | '\x0C' => {
                    return self.error_token(BadString, ~"Newline in quoted string");
                },
                '\\' => {
                    match self.next_n_chars(1) {
                        // Quoted newline
                        ['\n'] | ['\x0C'] => self.position += 1,
                        [] =>
                            return self.error_token(BadString, ~"EOF in quoted string"),
                        _ => push_char!(string, self.consume_escape())
                    }
                }
                c => push_char!(string, c),
            }
        }
        self.error_token(String(string), ~"EOF in quoted string")
    }

    fn consume_hash(&self) -> (Token, Option<ParseError>) {
        let string = self.consume_ident_string_rest();
        (if string == ~"" { Delim('#') } else { Hash(string) }, None)
    }

    fn consume_char(&self) -> char {
        let range = str::char_range_at(str::from_bytes(self.input), self.position);
        self.position = range.next;
        range.ch
    }

    fn match_here(&self, needle: ~str) -> bool {
        let mut i = self.position;
        if i + needle.len() > self.length { return false }
        let haystack: &str = str::from_bytes(self.input);
        for needle.each |c| { if haystack[i] != c { return false; } i += 1u; }
        return true;
    }

    fn consume_comments(&self)
            -> Option<(Token, Option<ParseError>)> {
        let vec_to_string: ~str = str::from_bytes(self.input);
        while self.match_here(~"/*") {
            self.position += 2; // consume /*
            match str::find_str_from(vec_to_string, "*/", self.position) {
                Some(end_position) => self.position = end_position + 2,
                None => {
                    self.position = self.length;
                    return Some(self.error_token(EOF, ~"Unclosed comment"))
                }
            }
        }
        None
    }

    fn consume_at_keyword(&self) -> (Token, Option<ParseError>) {
        (match self.consume_ident_string() {
            Some(string) => AtKeyword(string),
            None => Delim('@')
        }, None)
    }

    fn current_char(&self) -> char {
        str::char_at(str::from_bytes(self.input) , self.position)
    }

    fn next_is_namestart_or_escape(&self) -> bool {
        self.position += 1;
        let result = !self.is_eof() && self.is_namestart_or_escape();
        self.position -= 1;
        result
    }

    fn next_n_chars(&self, n: uint) -> ~[char] {
        let mut chars: ~[char] = ~[];
        let mut position = self.position;
        for n.times {
            if position >= self.length { break }
            let range = str::char_range_at(str::from_bytes(self.input), position);
            position = range.next;
            chars.push(range.ch);
        }
        chars
    }

    fn is_invalid_escape(&self) -> bool {
        match self.next_n_chars(2) {
            ['\\', '\n'] | ['\\', '\x0C'] | ['\\'] => true,
            _ => false,
        }
    }

    fn is_namestart_or_escape(&self) -> bool {
        match self.current_char() {
            'a'..'z' | 'A'..'Z' | '_' => true,
            '\\' => !self.is_invalid_escape(),
            c => c >= '\x80', // Non-ASCII
        }
    }


    fn consume_ident(&self) -> (Token, Option<ParseError>) {
        match self.consume_ident_string() {
            Some(string) => {
                if self.is_eof() { return (Ident(string), None) }
                match self.current_char() {
                    '\t' | '\n' | '\x0C' | ' '
                            if self.transform_function_whitespace => {
                        self.position += 1;
                        self.handle_transform_function_whitespace(string)
                    }
                    '(' => {
                        self.position += 1;
                        if ascii_lower(string) == ~"url" { self.consume_url() }
                        else { (Function(string), None) }
                    },
                    _ => (Ident(string), None)
                }
            },
            None => match self.current_char() {
                '-' => {
                    self.position += 1;
                    (Delim('-'), None)
                },
                '\\' => {
                    self.position += 1;
                    self.error_token(Delim('\\'), ~"Invalid escape")
                },
                _ => fail!(), // Should not have called consume_ident() here.
            }
        }
    }

    fn consume_ident_string(&self) -> Option<~str> {
        match self.current_char() {
            '-' => if !self.next_is_namestart_or_escape() { None }
                   else { Some(self.consume_ident_string_rest()) },
            '\\' if self.is_invalid_escape() => return None,
            _ if !self.is_namestart_or_escape() => return None,
            _ => Some(self.consume_ident_string_rest())
        }
    }

    fn consume_ident_string_rest(&self) -> ~str {
        let mut string = ~"";
        while !self.is_eof() {
            let c = self.current_char();
            let next_char = match c {
                'a'..'z' | 'A'..'Z' | '0'..'9' | '_' | '-' => {
                    self.position += 1; c },
                _ if c >= '\x80' => self.consume_char(), // Non-ASCII
                '\\' => {
                    if self.is_invalid_escape() { break }
                    self.position += 1;
                    self.consume_escape()
                },
                _ => break
            };
            push_char!(string, next_char)
        }
        string
    }

    fn char_from_hex(&self ,hex: &[char]) -> char {
        uint::from_str_radix(str::from_chars(hex), 16).get() as char
    }

    fn consume_escape(&self) -> char {
        let c = self.consume_char();
        match c {
            '0'..'9' | 'A'..'F' | 'a'..'f' => {
                let mut hex = ~[c];
                while hex.len() < 6 && !self.is_eof() {
                    let c = self.current_char();
                    match c {
                        '0'..'9' | 'A'..'F' | 'a'..'f' => {
                            hex.push(c); self.position += 1 },
                        _ => break
                    }
                }
                if !self.is_eof() {
                    match self.current_char() {
                        '\t' | '\n' | '\x0C' | ' ' => self.position += 1,
                        _ => ()
                    }
                }
                let c = self.char_from_hex(hex);
                if '\x00' < c && c <= MAX_UNICODE { c }
                else { '\uFFFD' } // Replacement character
            },
            c => c
        }
    }

    fn consume_url(&self) -> (Token, Option<ParseError>) {
        while !self.is_eof() {
            match self.current_char() {
                '\t' | '\n' | '\x0C' | ' ' => self.position += 1,
                '"' => return self.consume_quoted_url(false),
                '\'' => return self.consume_quoted_url(true),
                ')' => { self.position += 1; return (URL(~""), None) },
                _ => return self.consume_unquoted_url(),
            }
        }
        self.error_token(BadURL, ~"EOF in URL")
    }

    fn consume_quoted_url(&self, single_quote: bool)
            -> (Token, Option<ParseError>) {
        self.position += 1; // The initial quote
        let (token, err) = self.consume_quoted_string(single_quote);
        match err {
            Some(_) => {
                let (token, _) = self.consume_bad_url();
                (token, err)
            },
            None => match token {
                String(string) => self.consume_url_end(string),
                // consume_quoted_string() never returns a non-String token
                // without error:
                _ => fail!(),
            }
        }
    }

    fn consume_unquoted_url(&self) -> (Token, Option<ParseError>) {
        let mut string = ~"";
        while !self.is_eof() {
            let next_char = match self.consume_char() {
                '\t' | '\n' | '\x0C' | ' '
                    => return self.consume_url_end(string),
                ')' => return (URL(string), None),
                '\x00'..'\x08' | '\x0E'..'\x1F' | '\x7F'..'\x9F' // non-printable
                    | '"' | '\'' | '(' => return self.consume_bad_url(),
                '\\' => match self.next_n_chars(1) {
                    ['\n'] | ['\x0C'] | [] => return self.consume_bad_url(),
                    _ => self.consume_escape()
                },
                c => c
            };
            push_char!(string, next_char)
        }
        self.error_token(BadURL, ~"EOF in URL")
    }

    fn consume_url_end(&self, string: ~str)
            -> (Token, Option<ParseError>) {
        while !self.is_eof() {
            match self.consume_char() {
                '\t' | '\n' | '\x0C' | ' ' => (),
                ')' => return (URL(string), None),
                _ => return self.consume_bad_url()
            }
        }
        self.error_token(BadURL, ~"EOF in URL")
    }

    fn consume_bad_url(&self) -> (Token, Option<ParseError>) {
        // Consume up to the closing )
        while !self.is_eof() {
            match self.consume_char() {
                ')' => break,
                '\\' => self.position += 1, // Skip an escaped ) or \
                _ => ()
            }
        }
        self.error_token(BadURL, ~"Invalid URL syntax")
    }

    fn consume_unicode_range(&self)
            -> (Token, Option<ParseError>) {
        let next_3 = self.next_n_chars(3);
        // We got here with U or u
        assert next_3[0] == 'u' || next_3[0] == 'U';
        // Check if this is indeed an unicode range. Fallback on ident.
        if next_3.len() == 3 && next_3[1] == '+' {
            match next_3[2] {
                '0'..'9' | 'a'..'f' | 'A'..'F' => self.position += 2,
                _ => { return self.consume_ident() }
            }
        } else { return self.consume_ident() }

        let mut hex = ~[];
        while hex.len() < 6 && !self.is_eof() {
            let c = self.current_char();
            match c {
                '0'..'9' | 'A'..'F' | 'a'..'f' => {
                    hex.push(c); self.position += 1 },
                _ => break
            }
        }
        assert hex.len() > 0;
        let max_question_marks = 6u - hex.len();
        let mut question_marks = 0u;
        while question_marks < max_question_marks && !self.is_eof()
                && self.current_char() == '?' {
            question_marks += 1;
            self.position += 1
        }
        let start: char, end: char;
        if question_marks > 0 {
            start = self.char_from_hex(hex + vec::from_elem(question_marks, '0'));
            end = self.char_from_hex(hex + vec::from_elem(question_marks, 'F'));
        } else {
            start = self.char_from_hex(hex);
            hex = ~[];
            if !self.is_eof() && self.current_char() == '-' {
                self.position += 1;
                while hex.len() < 6 && !self.is_eof() {
                    let c = self.current_char();
                    match c {
                        '0'..'9' | 'A'..'F' | 'a'..'f' => {
                            hex.push(c); self.position += 1 },
                        _ => break
                    }
                }
            }
            end = if hex.len() > 0 { self.char_from_hex(hex) } else { start }
        }
        (if start > MAX_UNICODE || end < start {
            EmptyUnicodeRange
        } else {
            let end = if end <= MAX_UNICODE { end } else { MAX_UNICODE };
            UnicodeRange(start, end)
        }, None)
    }

    fn consume_numeric(&self) -> (Token, Option<ParseError>) {
        let c = self.consume_char();
        match c {
            '-' | '+' => self.consume_numeric_sign(c),
            '.' => {
                if self.is_eof() { return (Delim('.'), None) }
                match self.current_char() {
                    '0'..'9' => self.consume_numeric_fraction(~"."),
                    _ => (Delim('.'), None),
                }
            },
            '0'..'9' => self.consume_numeric_rest(c),
            _ => fail!(), 
        }
    }

    fn consume_numeric_sign(&self, sign: char)
            -> (Token, Option<ParseError>) {
        if self.is_eof() { return (Delim(sign), None) }
        match self.current_char() {
            '.' => {
                self.position += 1;
                if !self.is_eof()
                        && is_match!(self.current_char(), '0'..'9') {
                    self.consume_numeric_fraction(str::from_char(sign) + ~".")
                } else {
                    self.position -= 1;
                    (Delim(sign), None)
                }
            },
            '0'..'9' => self.consume_numeric_rest(sign),
            _ => (Delim(sign), None)
        }
    }

    fn consume_numeric_rest(&self, initial_char: char)
            -> (Token, Option<ParseError>) {
        let mut string = str::from_char(initial_char);
        while !self.is_eof() {
            let c = self.current_char();
            match c {
                '0'..'9' => { push_char!(string, c); self.position += 1 },
                '.' => {
                    self.position += 1;
                    if !self.is_eof()
                            && is_match!(self.current_char(), '0'..'9') {
                        push_char!(string, '.');
                        return self.consume_numeric_fraction(string);
                    } else {
                        self.position -= 1; break
                    }
                },
                _ => match self.consume_scientific_number(string) {
                    Ok(token) => return (token, None),
                    Err(s) => { string = s; break }
                }
            }
        }
        let value = Integer(int::from_str(
            // Remove any + sign as int::from_str() does not parse them.
            if string[0] != '+' as u8 { copy string }
            else { str::slice(string, 1, string.len()) }
        ).get()); // XXX handle overflow
        self.consume_numeric_end(string, value)
    }

    fn consume_numeric_fraction(&self, string: ~str)
            -> (Token, Option<ParseError>) {
        let mut string: ~str = string;
        while !self.is_eof() {
            match self.current_char() {
                '0'..'9' => push_char!(string, self.consume_char()),
                _ => match self.consume_scientific_number(string) {
                    Ok(token) => return (token, None),
                    Err(s) => { string = s; break }
                }
            }
        }
        let value = Float(float::from_str(string).get()); // XXX handle overflow
        self.consume_numeric_end(string, value)
    }


    fn consume_numeric_end(&self, string: ~str,
                           value: NumericValue) -> (Token, Option<ParseError>) {
        if self.is_eof() { return (Number(value, string), None) }
        (match self.current_char() {
            '%' => { self.position += 1; Percentage(value, string) },
            _ => {
                match self.consume_ident_string() {
                    Some(unit) => Dimension(value, string, unit),
                    None => Number(value, string),
                }
            },
        }, None)
    }


    fn consume_scientific_number(&self, string: ~str)
            -> Result<Token, ~str> {
        let next_3 = self.next_n_chars(3);
        let mut string: ~str = string;
        if (next_3.len() >= 2
            && (next_3[0] == 'e' || next_3[0] == 'E')
            && (is_match!(next_3[1], '0'..'9'))
        ) {
            push_char!(string, next_3[0]);
            push_char!(string, next_3[1]);
            self.position += 2;
        } else if (
            next_3.len() == 3
            && (next_3[0] == 'e' || next_3[0] == 'E')
            && (next_3[1] == '+' || next_3[1] == '-')
            && is_match!(next_3[2], '0'..'9')
        ) {
            push_char!(string, next_3[0]);
            push_char!(string, next_3[1]);
            push_char!(string, next_3[2]);
            self.position += 3;
        } else {
            return Err(string)
        }
        while !self.is_eof() && is_match!(self.current_char(), '0'..'9') {
            push_char!(string,self.consume_char())
        }
        let value = Float(float::from_str(string).get());
        Ok(Number(value, string))
    }

    pub fn error_token(&self ,t: Token, message: ~str) -> (Token, Option<ParseError>) {
        (t, Some(ParseError{message: message}))
    }

    pub fn css__lexer_create(input: @parserutils_inputstream) -> css_result {
    	CSS_GENERAL_OK
    }
}

pub fn lcss_lexer()->@lcss_lexer {
	@lcss_lexer{ transform_function_whitespace: false,
    input: ~[],
    length: 0, 
    position: 0 }
}


// ===========================================================================================================
// CSS-LEXER implementation/data-structs ends here 
// ===========================================================================================================
pub enum css_high_level_ptr
{
	high_level_pointer(@mut css_high_level),
	no_high_level_pointer
}
pub struct css_high_level
{
	mut base:@css_rule,
	//rule_type : css_rule_type,
	mut selector  : @css_rule_selector,
	mut charset   : @css_rule_charset,
	mut import    : @css_rule_import,
	mut media     : @css_rule_media,
	mut font_face : @css_rule_font_face,
	mut page      : @css_rule_page,
	mut prev      : @mut css_high_level_ptr,
	mut next      : @mut css_high_level_ptr

}
pub fn lcss_high_level(/*sheet:@css_stylesheet*/)-> @css_high_level
{
	let lwc_instance= lwc();
	@css_high_level
	{
		base:@css_rule
		{
			parent:@rule(0),		
			next:@mut NoRuleNode ,				
		    prev:@mut NoRuleNode ,				
		    rule_type  : CSS_RULE_UNKNOWN,		
			index : 0,		
			items : 0,		
			ptype : 0	
		},
		//rule_type : CSS_RULE_UNKNOWN,
		selector  : @css_rule_selector{
						/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

		 				selectors:~[],
		 				style:css_style
		 				{
		 					bytecode:~[] ,
							//used : 0,
							//allocated: 0,
							sheet:@NoStyleSheetNode
		 				},
					},
		charset   : @css_rule_charset{
		    		/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

		            encoding: lwc_instance.lwc_intern_string(@"")
		    	},
		import    : @css_rule_import{
					/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  :  rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/
					url:lwc_instance.lwc_intern_string(@""),
		            media:0,

		            sheet:@mut NoStyleSheetNode
				},
		media     : @css_rule_media{
					/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

					media:0,

					first_child:@mut NoRuleNode,
					last_child:@mut NoRuleNode
				},
		font_face : @css_rule_font_face{
		    		/*base :css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/
					font_face:@css_font_face 
						{
							font_family:lwc_instance.lwc_intern_string(@""),
							srcs:@css_font_face_src
							{
								location:lwc_instance.lwc_intern_string(@""),	
								bits:~[]
							},
							n_srcs:0,
		
								
							bits:~[]
						}
		    	},
		page      : @css_rule_page{
		    	/*base:css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : rule_type,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						},*/

				selector:@css_selector 
						{
							combinator:~[],		/*< Combining selector */

							rule:@no_high_level_pointer,/*@css_rule_selector
								{//check for errors due to commenting this out
									/*base:css_rule
										{
										parent:@rule(0),		
				        				next:@mut NoRuleNode ,				
		                				prev:@mut NoRuleNode ,				
		                				rule_type  : CSS_RULE_UNKNOWN,		
			            				index : 0,		
			            				items : 0,		
			            				ptype : 0	
										},

		 							selectors:~[],
		 							style:css_style
		 								{
		 									bytecode:~[] ,
											used : 0,
											allocated: 0
		 								},
		 							},*/
							
							
							specificity:CSS_SPECIFICITY_A,			

							data:@css_selector_detail
							{
								qname:css_qname
								{
									ns : lwc_instance.lwc_intern_string(@"") ,
									name : lwc_instance.lwc_intern_string(@"") 
								},			
								value:css_selector_detail_value
								{
									string:~"",		
									a:0,
									b:0
								},	

								type_of     :0 ,    		   
								comb        :0 ,    		    
								next        :0 ,     		     
													            
								value_type  :0,		        
								negate      :0    		    
							}
						}	,
				style:@css_style
						{
		 					bytecode:~[] ,
							//used : 0,
							//allocated: 0,
							sheet:@NoStyleSheetNode
		 				},	
		    },
		   prev: @mut no_high_level_pointer,
		   next:@mut no_high_level_pointer 

	}

}


// ===========================================================================================================
// CSS-SELECTOR implementation/data-structs start here 
// ===========================================================================================================

struct css_selector {
	mut combinator:~[@css_selector],		/*< Combining selector */
	mut rule:@css_high_level_ptr ,				/*< Owning rule */
	mut specificity:u32,			//< Specificity of selector 
    mut data:@css_selector_detail		/*< Selector data */
}

pub fn css__selector_hash_create()-> css_result
{
	CSS_GENERAL_OK
}


// ===========================================================================================================
// CSS-SELECTOR implementation/data-structs ends here 
// ===========================================================================================================




// ===========================================================================================================
// CSS-STYLESHEET implementation/data-structs start here 
// ===========================================================================================================
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

	mut rule_type  :  css_rule_type,		/**< css_rule_type */
	mut index : uint,		/**< index in sheet */
	mut items : uint,		/**< # items in rule */
	mut ptype : uint		/*< css_rule_parent_type */
}

pub struct css_rule_selector {
	 //mut base:css_rule,

	 mut selectors:~[@css_selector],
	 style:css_style 
}
pub struct css_rule_media {
	//base:css_rule ,

	media:u64,

	first_child:@mut  css_rule_node,
	last_child:@mut  css_rule_node
}
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

	mut url:@lwc_string,
	mut media:u64,

	mut sheet:@mut StyleSheetNode
}
pub struct css_rule_charset {
	//base:css_rule ,

	encoding:@lwc_string	/* \todo use MIB enum? */
}
pub type  css_import_notification_fn =  @extern fn(pw:~[u8],
		 parent:@css_stylesheet,  url:@lwc_string, media:u64) -> css_result;
pub type  css_url_resolution_fn =  @extern fn(pw:~[u8],
		base:~str, rel:@lwc_string , abs:@lwc_string ) -> css_result;
pub type  css_color_resolution_fn =  @extern fn(pw:~[u8],
		name:@lwc_string,  color:@css_color) -> css_result;
pub type  css_font_resolution_fn =  @extern fn(pw:~[u8],
		name:@lwc_string,  system_font:@css_system_font) -> css_result;








pub fn CINF(pw:~[u8], parent:@css_stylesheet,  url:@lwc_string, media:u64) -> css_result
{
	CSS_GENERAL_OK
}
pub  fn CURF(pw:~[u8],base:~str, rel:@lwc_string , abs:@lwc_string ) -> css_result
{
	CSS_GENERAL_OK
}
pub fn CCRF(pw:~[u8],name:@lwc_string,  color:@css_color) -> css_result
{
	CSS_GENERAL_OK
}
pub fn CFRF(pw:~[u8],name:@lwc_string,  system_font:@css_system_font) -> css_result
{
	CSS_GENERAL_OK
}









/**
 * Parameter block for css_stylesheet_create()
 */
struct css_stylesheet_params {
	/** ABI version of this structure */
		mut params_version:u32 ,

	/** The language level of the stylesheet */
		mut level:css_language_level,

	/** The charset of the stylesheet data, or NULL to detect */
		mut charset:~str,

	/** URL of stylesheet */
		mut url:~str,

	/** Title of stylesheet */
		mut title:~str,

	/** Permit quirky parsing of stylesheet */
		mut allow_quirks:bool,

	/** This stylesheet is an inline style */
		mut inline_style:bool,

	/** URL resolution function */
		mut resolve : @extern fn (pw:~[u8],base:~str, rel:@lwc_string, abs: @lwc_string) -> css_result,

	/** Client private data for resolve */
		mut resolve_pw:~[u8],

	/** Import notification function */
		mut import: @extern fn (pw:~[u8], parent:@css_stylesheet, url:@lwc_string, media:u64) -> css_result,

	/** Client private data for import */
		mut import_pw:~[u8],

	/** Colour resolution function */
		mut color: @extern fn (pw:~[u8], name:@lwc_string, color:@css_color) -> css_result,

	/** Client private data for color */
		mut color_pw:~[u8],

	/** Font resolution function */
		mut font: @extern fn(pw:~[u8], name:@lwc_string, system_font:@css_system_font) -> css_result ,

	/** Client private data for font */
		mut font_pw: ~[u8]
}
pub enum StyleSheetNode
{
	SomeStyleSheetNode(@mut css_stylesheet),
	NoStyleSheetNode
}

pub struct css_stylesheet {
	//selectors:@css_selector_hash,	TODO REPLACE WITH BUILT IN HASH TABLE
		/* < Hashtable of selectors */
	mut lwc_instance:@lwc,
    //parser_instance:@lcss_parser,
	mut rule_count:u32,			/**< Number of rules in sheet */
	mut rule_list:@mut css_high_level_ptr ,			/**< List of rules in sheet */
	mut last_rule:@mut css_high_level_ptr,			/**< Last rule in list */

	mut disabled:bool,				/**< Whether this sheet is 
						 * disabled */

	mut url:~str,				/**< URL of this sheet */
	mut title:~str,			/**< Title of this sheet */

	mut level:css_language_level ,		/**< Language level of sheet */
	mut parser:@mut css_parser_node ,			/**< Core parser for sheet */
	mut parser_frontend:@mut css_language_node,			/**< Frontend parser */////////look for type
	//propstrings:@ mut[@lwc_string ],		/**< Property strings, for parser */

	mut quirks_allowed:bool,			/**< Quirks permitted */
	mut quirks_used:bool,			/**< Quirks actually used */

	mut inline_style:bool,			/**< Is an inline style */

	mut size:uint,				/**< Size, in bytes */

	mut  import:css_import_notification_fn,	/**< Import notification function */
	mut import_pw:~[u8],			/**< Private word *////////look for type

	mut  resolve:css_url_resolution_fn,		/**< URL resolution function */
	mut resolve_pw:~[u8],			/**< Private word *////////look for type

	mut  color:css_color_resolution_fn,		/**< Colour resolution function */
	mut color_pw:~[u8],				/**< Private word *////////look for type

	/** System font resolution function */
	mut  font:css_font_resolution_fn,		
	mut font_pw:~[u8],				/**< Private word *////////look for type


	// alloc:css_allocator_fn,			/**< Allocation function */
	//pw:~[u8],				/**< Private word */
  
	mut cached_style:@ css_style_Node ,		/**< Cache for style parsing */
  
	mut string_vector:~[@lwc_string],            /**< Bytecode string vector */
	//string_vector_l:u32,              /**< The string vector allocated
					// * length in entries */
	//string_vector_c:u32,               /*< The number of string * vector entries used */ 
	mut propstrings_call_count:uint,
    mut propstrings_list:@[@str],
	mut propstrings:~[@lwc_string]					 
}

const CSS_STYLE_DEFAULT_SIZE:u32 =16;
pub fn lcss_stylesheet(lwc_inst:@lwc)->@css_stylesheet {
	@css_stylesheet{
		                lwc_instance:lwc_inst,
		                //parser_instance: parser_inst,
		            	rule_count:0,			/*< Number of rules in sheet */
						rule_list:@mut no_high_level_pointer/*css_rule
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : CSS_RULE_UNKNOWN,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						}*/,			/*< List of rules in sheet */
						last_rule:@mut no_high_level_pointer/*css_rule 
						{
							parent:@rule(0),		
				        	next:@mut NoRuleNode ,				
		                	prev:@mut NoRuleNode ,				
		                	rule_type  : CSS_RULE_UNKNOWN,		
			            	index : 0,		
			            	items : 0,		
			            	ptype : 0	
						}*/,			/*< Last rule in list */

						disabled:false,				/*< Whether this sheet is 
							                          * disabled */

						url:~"",				/*< URL of this sheet */
						title:~"",			/*< Title of this sheet */

						level:CSS_LEVEL_1  ,		/*< Language level of sheet */
						parser:@mut NoParserNode ,			/*< Core parser for sheet */
						parser_frontend:@mut NoLanguageNode,			/*< Frontend parser */
						//propstrings:@ mut[],		/*< Property strings, for parser */

						quirks_allowed:false,			/*< Quirks permitted */
						quirks_used:false,			/*< Quirks actually used */

						inline_style:false,			/*< Is an inline style */

						size:0 ,				/*< Size, in bytes */

		 				import:@CINF,	/*< Import notification function */
						import_pw:~[],			/*< Private word */

		 				resolve:@CURF,		/*< URL resolution function */
						resolve_pw:~[],			/*< Private word */

		 				color:@CCRF,		/*< Colour resolution function */
						color_pw:~[],				/*< Private word */

		/* System font resolution function */
		 				font:@CFRF,		
						font_pw:~[],				/*< Private word */


		// alloc:css_allocator_fn,			/*< Allocation function */
		//pw:~[u8],				/*< Private word */
	  
						cached_style:@NoStyleNode,	/*< Cache for style parsing */
	  
						string_vector:~[],            /*< Bytecode string vector */
						//string_vector_l:0,              /*< The string vector allocated
						// * length in entries */
						//string_vector_c:0 ,
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
	    propstrings:/*lwc_inst.lwc_intern_string(@"")*/~[]
		            } 
}

impl css_stylesheet {


pub fn css__propstrings_unref(&self)
	{
		self.propstrings_call_count  -=1;

		if (self.propstrings_call_count  == 0) {
			let mut  i=0;

			while ( i < self.propstrings_list.len())
			{
				self.lwc_instance.lwc_string_unref(self.propstrings[i]);
				i += 1;
			}
				
		}
	}

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
		

pub fn css__stylesheet_rule_add_selector(&self,/*sheet:  @css_stylesheet , */
		 mut curRule:@mut css_high_level , selector: @css_selector )
{
	match(curRule.base.rule_type)
	 {
	 	CSS_RULE_SELECTOR=>{},
	 	_=>{fail!();}
	 }

		
	curRule.selector.selectors.push(selector);//check later
		
   	
	curRule.base.items += 1;
	curRule.selector.selectors[curRule.base.items].rule = @high_level_pointer(curRule);//problem 2
	 
}


pub fn  css__stylesheet_rule_create(@self,sheet:@css_stylesheet ,  rule_type:css_rule_type/*,
		css_rule **rule*/)->css_result
{
	let mut high_level_css_struct:@css_high_level =  lcss_high_level(/*self*/);
	high_level_css_struct.base.rule_type = rule_type;
	CSS_RULE_CREATED_OK(high_level_css_struct)	
	//CSS_GENERAL_OK
}

pub fn css__stylesheet_string_add(&self,sheet:css_stylesheet , string:@lwc_string /*, uint32_t *string_number*/)-> css_result
{
	let string_number:@mut u32 = @mut 0;
	let strCount =  sheet.string_vector.len() as u32;
    /* search for the string in the existing vector */
	while (*string_number < strCount)
	{
		//let res:lwc_result ;
		let mut isEqual = false;
		isEqual = lwc::lwc_string_isequal(string, sheet.string_vector[*string_number]);
		
		*string_number += 1;
		if(isEqual)
		{
			self.lwc_instance.lwc_string_unref(string);
			return CSS_STRING_ADD_OK(string_number);
		}
		
	}

	/* string does not exist in current vector, add a new one */
	sheet.string_vector.push(copy string);
	return CSS_STRING_ADD_OK(string_number);
	
}

pub fn css__stylesheet_string_get(/*sheet:@css_stylesheet,*/ &self,mut string_number:u32/*, lwc_string **string*/)->css_result
{
	string_number -= 1;
    if string_number > self.string_vector.len() as u32
    {
    	return CSS_BADPARM;
    }
    CSS_STRING_GET(self.string_vector[string_number])
}

 pub fn css_stylesheet_create(&self,params:@css_stylesheet_params /*,
		css_allocator_fn alloc, void *alloc_pw, 
		css_stylesheet **stylesheet*/)->css_result
{
	let sheet = lcss_stylesheet(lwc());
	let mut Result=self.css__propstrings_get();
	match(copy Result) {
		CSS_PROPSTRINGS_OK(x) => sheet.propstrings = x,
		_=>{ return Result}
	}
	sheet.inline_style = params.inline_style;
	let mut charsetDetect :css_charset_source ;
	if(params.charset.len()  == 0)
	{
		charsetDetect=  CSS_CHARSET_DEFAULT;
	}
	else{
		charsetDetect =  CSS_CHARSET_DICTATED;
	}
    //sheet.parser =  @mut SomeParserNode(css__parser_create(copy params.charset,charsetDetect,lcss_language(sheet)));
	if (params.inline_style) {
		
		sheet.parser =  @mut SomeParserNode(css__parser_create_for_inline_style(copy params.charset,charsetDetect,lcss_language(sheet)));
	} else {
		sheet.parser =  @mut SomeParserNode(css__parser_create(copy params.charset,charsetDetect,lcss_language(sheet)));
	}
	sheet.quirks_allowed = params.allow_quirks;
	let mut optparams:@css_parser_optparams = css_parser_optparams_instance() ;
	
	if (params.allow_quirks) {
		optparams.quirks = true;
		match(sheet.parser)
		{
			@SomeParserNode(x)=>Result = x.css__parser_setopt( CSS_PARSER_QUIRKS,optparams),
			_=>{}
		}
		match (copy Result)
		{
			CSS_GENERAL_OK=>{}
			_=>{
				self.css__propstrings_unref();
				return Result;
			}
		}
	}
	sheet.level = params.level;
    Result = css__language_create(sheet, sheet.parser/*alloc, alloc_pw,*//*&sheet->parser_frontend*/);
	match(copy Result)
	{
		CSS_LANGUAGE_CREATED_OK(lan)=> sheet.parser_frontend = @mut SomeLanguageNode(lan),
		_=>{}
	}
    //TODO uncomment when selector hashtable is implemented
	/*Result =  css__selector_hash_create();
	match(copy Result)
	{
		CSS_SELECTOR_CREATE_OK(sel)=> sheet.selector=sel,
		_=>{}
	}
*/

    sheet. url = copy params.url;
    sheet. title = copy params.title;

	sheet.resolve =copy params.resolve;
	sheet.resolve_pw =copy params.resolve_pw;

	sheet.import = copy params.import;
	sheet.import_pw = copy params.import_pw;

	sheet.color = copy params.color;
	sheet.color_pw = copy params.color_pw;

	sheet.font = copy params.font;
	sheet.font_pw = copy params.font_pw;

	/*sheet.alloc = alloc;
	sheet.pw = alloc_pw;*/

	CSS_STYLESHEET_CREATE_OK(sheet)
}

pub fn css_stylesheet_append_data( &self,
		data:~[u8])-> css_result
{
	/*if (sheet == NULL || data == NULL)
		return CSS_BADPARM;

	if (sheet->parser == NULL)
		return CSS_INVALID;*/
		match(self.parser)
		{
			@SomeParserNode(x)=>return x.css__parser_parse_chunk( data),
			_=> return CSS_INVALID
		}

	
}
pub fn css_stylesheet_data_done(&self/*css_stylesheet *sheet*/)-> css_result
{
	let mut Result:css_result;
	match(self.parser)
	{
		@SomeParserNode(x)=> Result = x.css__parser_completed(),
		_=>return CSS_INVALID
	}

	self.parser_frontend = @mut NoLanguageNode;
	self.parser = @mut NoParserNode;

    let mut iter:@mut css_high_level_ptr = self.rule_list;
    loop 
    	{
    		match(iter)
    		{
    			@high_level_pointer(x)=> {
    				iter=x.next;
    				match(x.base.rule_type)
    				{
    					CSS_RULE_UNKNOWN=>{},
    					CSS_RULE_CHARSET=>{},
    					CSS_RULE_IMPORT=>{
    						let mut importOfHighLevel@css_rule_import=x.import;
    						match(x.import.sheet)
    						{
    							@SomeStyleSheetNode(x)=>{},
    							@NoStyleSheetNode=> return CSS_IMPORTS_PENDING
    						}
    						
    					},
    					_=>{break;}
    				}
    			},
    			@no_high_level_pointer=> {break;}    
    		}
       	}
    


	CSS_GENERAL_OK
}
pub fn css_stylesheet_next_pending_import(&self/*,
		url:@lwc_string , media:u64*/)->css_result
{
	let mut iter:@mut css_high_level_ptr = self.rule_list;
	let mut url:@lwc_string;
	let mut media:u64;
    loop 
    	{
    		match(iter)
    		{
    			@high_level_pointer(x)=> {
    				iter=x.next;
    				match(x.base.rule_type)
    				{
    					CSS_RULE_UNKNOWN=>{},
    					CSS_RULE_CHARSET=>{},
    					CSS_RULE_IMPORT=>{
    						let mut importOfHighLevel@css_rule_import=x.import;
    						match(x.import.sheet)
    						{
    							@SomeStyleSheetNode(x)=>{},
    							@NoStyleSheetNode=>  {
    								url = x.import.url;
    								media = x.import.media;
    								return CSS_IMPORTS_PENDING_OK(url,media);
    							}
    						}
    						
    					},
    					_=>{break;}
    				}
    			},
    			@no_high_level_pointer=> {break;}    
    		}
       	}
	

	return CSS_INVALID;
}

pub fn css_stylesheet_register_import(&self,
		import:@mut css_stylesheet)-> css_result
{
	let mut iter:@mut css_high_level_ptr = self.rule_list;
	//let mut url:@lwc_string;
	//let mut media:u64;
    loop 
    	{
    		match(iter)
    		{
    			@high_level_pointer(x)=> {
    				
    				match(x.base.rule_type)
    				{
    					CSS_RULE_UNKNOWN=>{},
    					CSS_RULE_CHARSET=>{},
    					CSS_RULE_IMPORT=>{
    						
    						match(x.import.sheet)
    						{
    							@SomeStyleSheetNode(x)=>{},
    							@NoStyleSheetNode=>  {
    								x.import.sheet=@mut SomeStyleSheetNode(import);
    								return CSS_GENERAL_OK;
    							}
    						}
    						
    					},
    					_=>{break;}
    				}
    				iter=x.next;
    			},
    			@no_high_level_pointer=> {break;}    
    		}
       	}
	

	return CSS_INVALID;
	
}
pub fn css_stylesheet_get_language_level(&self )-> css_result
{
	/*if (sheet == NULL || level == NULL)
		return CSS_BADPARM;

	*level = sheet->level;*/

	return CSS_GET_LANGUAGE_LEVEL(self.level);
}

pub fn css_stylesheet_get_url(&self)-> css_result
{
	/*if (sheet == NULL || url == NULL)
		return CSS_BADPARM;

	*url = sheet->url;*/

	return CSS_GET_URL( copy self.url);
}
pub fn css_stylesheet_get_title(&self)-> css_result
{
	return CSS_GET_TITLE(copy self.title);
}

pub fn css_stylesheet_quirks_allowed(&self)-> css_result
{
	return CSS_IS_QUIRK_ALLOWED(self.quirks_allowed);
}

pub fn css_stylesheet_used_quirks(&self)-> css_result
{
	return CSS_IS_QUIRK_USED(self.quirks_used);
}

pub fn css_stylesheet_get_disabled(&self)-> css_result
{
	return CSS_GET_SHEET_DISABLED(self.disabled);
}
pub fn css_stylesheet_set_disabled(&self,disabled:bool)-> css_result
{
	self.disabled = disabled;
	return CSS_GENERAL_OK;
}
pub fn css_stylesheet_size(&self, size:uint)-> css_result
{
    CSS_GENERAL_OK//(size)
	//not implemented
}
pub fn css__stylesheet_style_create(@mut self)-> css_result
{
	match(self.cached_style)
	{
		@SomeStyleNode(Style)=>{
			self.cached_style= @NoStyleNode;
			CSS_STYLECREATED_OK(@SomeStyleNode(Style));
		},
		@NoStyleNode=>{}
	}
	let mut Style=@css_style
	{
		bytecode:~[] ,
		//used : 0,
		//allocated: CSS_STYLE_DEFAULT_SIZE,
		sheet:@SomeStyleSheetNode(self)

	};
	CSS_STYLECREATED_OK(@SomeStyleNode(Style))
}

static pub fn css__stylesheet_merge_style(target:@css_style ,  style:@css_style)-> css_result
{
	
	target.bytecode = vec::append(copy target.bytecode, style.bytecode);
	CSS_GENERAL_OK

}
pub fn css__stylesheet_style_append(style:@css_style,  css_code:css_code_t)-> css_result
{
  style.bytecode.push(css_code);
  CSS_GENERAL_OK
}
//check this functn
pub fn css__stylesheet_style_vappend(style:@css_style,  css_code:~[css_code_t])-> css_result
{
	style.bytecode = vec::append(copy style.bytecode, css_code);
	CSS_GENERAL_OK
}



}



// ===========================================================================================================
// CSS-STYLESHEET implementation/data-structs ends here 
// ===========================================================================================================


// ===========================================================================================================
// CSS-LANGUAGE implementation/data-structs start here 
// ===========================================================================================================
pub enum css_language_node
{
	SomeLanguageNode(@mut css_language),
  	NoLanguageNode
}

pub struct css_language {
	sheet:@css_stylesheet ,		/**< The stylesheet to parse for */
	mut lwc_instance:@lwc,
//#define STACK_CHUNK 32
    STACK_CHUNK:int,
	context:@DVec<context_entry>,      //parseutils_stack	/**< Context stack */

	 state:language_state,			/**< State flag, for at-rule handling */

	/** Interned strings */
	mut strings: ~[@lwc_string ],

	default_namespace:@lwc_string ,	/**< Default namespace URI */
	namespaces:@css_namespace,	/**< Array of namespace mappings */
	num_namespaces:u32	/*< Number of namespace mappings */

	// css_allocator_fn alloc;		*< Memory (de)allocation function 
	// void *pw;			/**< Client's private data */
}
pub fn lcss_language(sheet:@css_stylesheet)->@css_language {
	let empty_lwc_string = sheet.lwc_instance.lwc_intern_string(@"");
	let stack:@DVec<context_entry> = @dvec::DVec();
	
	//@css_language {
					

				let	css_language_instance = @css_language {
							sheet:sheet,
							lwc_instance:sheet.lwc_instance,		
				    		STACK_CHUNK:32,
							context:stack, 
							state:CHARSET_PERMITTED,	
							strings:copy sheet.propstrings,
							
							default_namespace:empty_lwc_string,	
							
							namespaces:@css_namespace
							{
								prefix:empty_lwc_string,	
								uri:empty_lwc_string	
							},	
							num_namespaces:0	
		
		            };
	return css_language_instance;
}
pub fn  css__language_create( sheet:@css_stylesheet,parserNode:@mut css_parser_node) -> css_result
	{
		let lwc_inst=lwc();
	let empty_lwc_string = lwc_inst.lwc_intern_string(@"");
	let stack:@DVec<context_entry> = @dvec::DVec();
	
	//@css_language {
					

				let	css_language_instance = @mut css_language {
							sheet:sheet,
							lwc_instance:lwc_inst,		
				    		STACK_CHUNK:32,
							context:stack, 
							state:CHARSET_PERMITTED,	
							strings:copy sheet.propstrings,
							
							default_namespace:empty_lwc_string,	
							
							namespaces:@css_namespace
							{
								prefix:empty_lwc_string,	
								uri:empty_lwc_string	
							},	
							num_namespaces:0	
		
		            };
	
		
		css_language_instance.sheet=sheet;

		

		/*let params = @css_parser_optparams {
			quirks:false,
			event_handler: css_parser_event_handler_
			{
				handler:language_handle_event,
				pw:css_language_instance
			}
		};*/ //see later
		
		
		
		return CSS_LANGUAGE_CREATED_OK(css_language_instance);
	}


impl css_language
 {

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


pub fn  css__language_create(@mut self, sheet:@css_stylesheet) -> css_result
	{
		
		self.sheet=sheet;

		

		/*let params = @css_parser_optparams {
			quirks:false,
			event_handler: css_parser_event_handler_
			{
				handler:language_handle_event,
				pw:css_language_instance
			}
		};*/ //see later
		
		
		
		return CSS_LANGUAGE_CREATED(self);
	}

pub fn handleStartStylesheet(&self, c:@css_language, vector:~[~str]) -> css_result
	{
		// let pResult:parserutils_result;
		// UNUSED(vector);
		let entry:context_entry = context_entry 
		{
			event_type: CSS_PARSER_START_STYLESHEET, 
			data:@css_rule 
				{
					parent:@rule(0),		
				    next:@mut NoRuleNode ,				
		            prev:@mut NoRuleNode ,				
		            rule_type  : CSS_RULE_UNKNOWN,		
			        index : 0,		
			        items : 0,		
			        ptype : 0	
				},	
    	 };
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
		/*parserutils_error pResult;
		css_result cResult;
		context_entry entry = { CSS_PARSER_START_RULESET, NULL };*/
		let mut cResult:css_result;
		let entry:context_entry = context_entry 
		{
			event_type: CSS_PARSER_START_STYLESHEET, 
			data:@css_rule 
				{
					parent:@rule(0),		
				    next:@mut NoRuleNode ,				
		            prev:@mut NoRuleNode ,				
		            rule_type  : CSS_RULE_UNKNOWN,		
			        index : 0,		
			        items : 0,		
			        ptype : 0	
				},	
    	 };
		let cur:@context_entry ;
		let mut parent_rule :@css_rule ;
		let mut curRule :@css_high_level ;
		

		// assert(c != NULL);

		/* Retrieve parent rule from stack, if any */
		if c.context.len() !=0
		{
			cur=@ c.context.last();
			match(cur.event_type  )
			{
				CSS_PARSER_START_STYLESHEET =>{},
				_=>{parent_rule = cur.data;}
			}
		}
		
		/*cur = parserutils_stack_get_current(c->context);
		if (cur != NULL && cur->type != CSS_PARSER_START_STYLESHEET)
			parent_rule = cur->data;*/
        match(self.sheet.css__stylesheet_rule_create(c.sheet, CSS_RULE_SELECTOR))
        {

		CSS_RULE_CREATED_OK( css_rule_selector)=>{curRule=css_rule_selector},
		_=>{return CSS_INVALID;}
		
        }
		if vector.len() != 0
		{
			//cResult = self.parseSelectorList(c, vector, curRule);
		}

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

		/* Rule is now owned by the sheet, so no need to destroy it */

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

 }


// ===========================================================================================================
// CSS-LANGUAGE implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// Important.h and important.c implementation/data-structs starts here 
// ===========================================================================================================



pub struct important 
{
	mut lpu_instance : @lpu,
	    mut lwc_instance : @lwc,
}

pub fn important()->@important
{
	@important
	{
lpu_instance : lpu(),
	     lwc_instance : lwc()
	}
}

impl important
{


	pub fn css_parse_important(&self, cssLang : @css_language, tokenVector : ~[~css_token], result : ~u8) -> css_result
	{
		//let mut token : ~css_token = ~{token_type:CSS_TOKEN_EOF, data:{data:~[] ,len:0}, idata:self.lwc_instance.lwc_intern_string(@""), col:0, line:0} ;

		// I think there is no need of this function, since we can ceck empty space here only...
		//consumeWhiteSpace(vector, cntx); 

		if tokenVector.is_empty() == true
		{
			return CSS_INVALID;
		}

		for tokenVector.each |&iter|
		{
			match(iter.token_type)
			{
				CSS_TOKEN_S  =>  {
					// do nothing
				},
					     _	=>  {
						     //token = iter; 
						     match(iter.token_type)
						     {
							     CSS_TOKEN_IDENT		=>	{
								     return CSS_INVALID;
							     },
							     _			=>	()
						     }
					     }

				//  commenting to avoid compilation error
				//  Need to fix it ASAP.

				/*
				   if lwc::lwc_instance.lwc_string_caseless_isequal(token.idata, cssLang.strings[IMPORTANT]) == true 
				   {
				   result |= FLAG_IMPORTANT;
				   }	
				   else
				   {
				   return CSS_INVALID;
				   }
				 */
			}
		}	

		return CSS_GENERAL_OK;
	}

	// Commenting to avoid compilation error
	pub fn css_make_style_important (&self, style : &css_style)
	{

		/*
		   let styleBytecode : ~[u32] = style.bytecode;
		   let styleLen : u32 = style.used; 
		   let styleOffset : u32 = 0;

		   while (styleOffset < styleLen)
		   {
		   let prop :  css_properties_e;
		   let flags : u8;
		   let value : u32;	
		   let propVal : u32 = styleBytecode[styleOffset]; 		

		// extracting propVal components, setting IMPORTANT FLAG
		prop = propVal & 0x3ff;
		flags = ((propVal >> 10) & 0xff) | FLAG_IMPORTANT;
		value = propVal >> 18;	

		// writing propVal back to bytecodes
		styleBytecode[styleOffset] = prop & 0x3ff | flags << 10 | ((value & 0x3fff) << 18);

		styleOffset += 1;	

		// Advance past any porp-specific data		
		if ((((propVal >> 10) & 0xff) & 0x2) == false)
		{
		match(styleBytecode)
		{
		CSS_PROP_AZIMUTH => {
		if ((value & ~AZIMUTH_BEHIND) == AZIMUTH_ANGLE)
		{
		styleOffset += 2; // length + units 
		}
		},
		CSS_PROP_BORDER_TOP_COLOR  |  CSS_PROP_BORDER_RIGHT_COLOR  | CSS_PROP_BORDER_BOTTOM_COLOR  |  CSS_PROP_BORDER_LEFT_COLOR  |  CSS_PROP_BACKGROUND_COLOR | CSS_PROP_COLUMN_RULE_COLOR  =>
		{
		//	assert(BACKGROUND_COLOR_SET == 	BORDER_COLOR_SET);
		//	assert(BACKGROUND_COLOR_SET == 	COLUMN_RULE_COLOR_SET);

		if (value == BACKGROUND_COLOR_SET)
		{
		styleOffset += 1; // colour 
		}
		},

		CSS_PROP_BACKGROUND_IMAGE |  CSS_PROP_CUE_AFTER  | CSS_PROP_CUE_BEFORE  | CSS_PROP_LIST_STYLE_IMAGE  => 
		{
		//	assert(BACKGROUND_IMAGE_URI == CUE_AFTER_URI);
		//      assert(BACKGROUND_IMAGE_URI == CUE_BEFORE_URI);
		//	assert(BACKGROUND_IMAGE_URI == LIST_STYLE_IMAGE_URI);

		if (value == BACKGROUND_IMAGE_URI) 
		{
		styleOffset += 1; // string table entry 
		}
		},

		CSS_PROP_BACKGROUND_POSITION  =>
		{ 
		if ((value & 0xf0) == BACKGROUND_POSITION_HORZ_SET)
		{
		styleOffset += 2; // length + units 
		}

		if ((value & 0x0f) == BACKGROUND_POSITION_VERT_SET)
		{
		styleOffset += 2; // length + units 
		}
		},

		CSS_PROP_BORDER_SPACING  => 
		{
		if (value == BORDER_SPACING_SET)
		{
			styleOffset += 4; // two length + units 
		}
	},

		CSS_PROP_BORDER_TOP_WIDTH  | CSS_PROP_BORDER_RIGHT_WIDTH |	 CSS_PROP_BORDER_BOTTOM_WIDTH  |  CSS_PROP_BORDER_LEFT_WIDTH  |	 CSS_PROP_OUTLINE_WIDTH  | CSS_PROP_COLUMN_RULE_WIDTH  => 
		{
			//	assert(BORDER_WIDTH_SET == OUTLINE_WIDTH_SET);
			//	assert(BORDER_WIDTH_SET == COLUMN_RULE_WIDTH_SET);

			if (value == BORDER_WIDTH_SET)
			{
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_MARGIN_TOP  |  CSS_PROP_MARGIN_RIGHT | CSS_PROP_MARGIN_BOTTOM  | CSS_PROP_MARGIN_LEFT  | CSS_PROP_BOTTOM  | CSS_PROP_LEFT  | CSS_PROP_RIGHT  | CSS_PROP_TOP  | CSS_PROP_HEIGHT |  CSS_PROP_WIDTH  |  CSS_PROP_COLUMN_WIDTH  | CSS_PROP_COLUMN_GAP  => 
		{
			//	assert(BOTTOM_SET == LEFT_SET);
			//	assert(BOTTOM_SET == RIGHT_SET);
			//	assert(BOTTOM_SET == TOP_SET);
			//	assert(BOTTOM_SET == HEIGHT_SET);
			//	assert(BOTTOM_SET == MARGIN_SET);
			//	assert(BOTTOM_SET == WIDTH_SET);
			// 	assert(BOTTOM_SET == COLUMN_WIDTH_SET);
			// 	assert(BOTTOM_SET == COLUMN_GAP_SET);

			if (value == BOTTOM_SET) 
			{
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_CLIP  => 
		{
			if ((value & CLIP_SHAPE_MASK) == CLIP_SHAPE_RECT) {
				if ((value & CLIP_RECT_TOP_AUTO) == 0)
				{
					styleOffset += 2; // length + units 
				}
				if ((value & CLIP_RECT_RIGHT_AUTO) == 0)
				{
					styleOffset += 2; // length + units 
				}
				if ((value & CLIP_RECT_BOTTOM_AUTO) == 0){
					styleOffset += 2; // length + units 
				}
				if ((value & CLIP_RECT_LEFT_AUTO) == 0){
					styleOffset += 2; // length + units 
				}
			}
		},

		CSS_PROP_COLOR  => 
		{
			if (value == COLOR_SET){				
				styleOffset += 1; // colour
			}
		},

		CSS_PROP_COLUMN_COUNT  => 
		{
			if (value == COLUMN_COUNT_SET){
				styleOffset += 1; // colour 
			}
		},

		CSS_PROP_CONTENT  => 
		{
			while (value != CONTENT_NORMAL && value != CONTENT_NONE) 
			{
				match (value & 0xff) 
				{
					CONTENT_COUNTER  |  CONTENT_URI  | CONTENT_ATTR  | CONTENT_STRING  => 
					{
						styleOffset += 1; // string table entry 
					}
					CONTENT_COUNTERS  => 
					{
						styleOffset+=2; // two string entries 
					}
					CONTENT_OPEN_QUOTE  | 	 CONTENT_CLOSE_QUOTE |	 CONTENT_NO_OPEN_QUOTE  | CONTENT_NO_CLOSE_QUOTE  => 
					{
						break;
					}
				}

				value = styleBytecode[styleOffset];
				styleOffset += 1;
			}
		},

		CSS_PROP_COUNTER_INCREMENT  | CSS_PROP_COUNTER_RESET  => 
		{
			// assert(COUNTER_INCREMENT_NONE == COUNTER_RESET_NONE);

			while (value != COUNTER_INCREMENT_NONE) {
				styleOffset += 2; // string + integer 

				value = styleBytecode[styleOffset];
				styleOffset += 1;
			}
		},

		CSS_PROP_CURSOR  =>
		{ 
			while (value == CURSOR_URI) {
				styleOffset += 1; // string table entry 

				value = styleBytecode[styleOffset];
				styleOffset += 1;
			}
		},	

		CSS_PROP_ELEVATION  => 
		{
			if (value == ELEVATION_ANGLE){			
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_FONT_FAMILY  => 
		{
			while (value != FONT_FAMILY_END) 
			{
				match (value) 
				{
					FONT_FAMILY_STRING  | FONT_FAMILY_IDENT_LIST  => 
					{		
						styleOffset += 1; // string table entry
						break;
					},
				}

				value = styleBytecode[styleOffset];
				styleOffset += 1;
			}
		},

		CSS_PROP_FONT_SIZE  => 
		{
			if (value == FONT_SIZE_DIMENSION) {
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_LETTER_SPACING  |  CSS_PROP_WORD_SPACING  => 
		{
			//	assert(LETTER_SPACING_SET == WORD_SPACING_SET);

			if (value == LETTER_SPACING_SET){
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_LINE_HEIGHT  => 
		{
			match (value) 
			{
				LINE_HEIGHT_NUMBER  => styleOffset += 1, 
				LINE_HEIGHT_DIMENSION  => styleOffset += 2
			}
		},

		CSS_PROP_MAX_HEIGHT  |  CSS_PROP_MAX_WIDTH  => 
		{
			// assert(MAX_HEIGHT_SET == MAX_WIDTH_SET);

			if (value == MAX_HEIGHT_SET){
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_PADDING_TOP |   CSS_PROP_PADDING_RIGHT  |  CSS_PROP_PADDING_BOTTOM  |	 CSS_PROP_PADDING_LEFT  | CSS_PROP_MIN_HEIGHT |	 CSS_PROP_MIN_WIDTH |	 CSS_PROP_PAUSE_AFTER | CSS_PROP_PAUSE_BEFORE  |
			CSS_PROP_TEXT_INDENT  => 
			{	
				//assert(MIN_HEIGHT_SET == MIN_WIDTH_SET);
				//assert(MIN_HEIGHT_SET == PADDING_SET);
				//assert(MIN_HEIGHT_SET == PAUSE_AFTER_SET);
				//assert(MIN_HEIGHT_SET == PAUSE_BEFORE_SET);
				//assert(MIN_HEIGHT_SET == TEXT_INDENT_SET);

				if (value == MIN_HEIGHT_SET){
					styleOffset += 2; // length + units 
				}
			},

		CSS_PROP_OPACITY  => 
		{
			if (value == OPACITY_SET) { 
				styleOffset += 1; // value 
			}
		},

		CSS_PROP_ORPHANS  |  CSS_PROP_PITCH_RANGE  | CSS_PROP_RICHNESS  | CSS_PROP_STRESS  |	 CSS_PROP_WIDOWS  =>
		{
			// assert(ORPHANS_SET == PITCH_RANGE_SET);
			// assert(ORPHANS_SET == RICHNESS_SET);
			// assert(ORPHANS_SET == STRESS_SET);
			// assert(ORPHANS_SET == WIDOWS_SET);

			if (value == ORPHANS_SET){
				styleOffset  += 1; // value 
			}
		},

		CSS_PROP_OUTLINE_COLOR  => 
		{
			if (value == OUTLINE_COLOR_SET){
				styleOffset += 1; // color 
			}
		},

		CSS_PROP_PITCH  => 
		{
			if (value == PITCH_FREQUENCY){
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_PLAY_DURING  => 
		{
			if (value == PLAY_DURING_URI)	{
				styleOffset += 1; // string table entry 
			}
		},

		CSS_PROP_QUOTES  =>
		{ 
			while (value != QUOTES_NONE) {
				styleOffset += 2; // two string table entries 

				value = styleBytecode[styleOffset];
				styleOffset += 1;
			}
		},

		CSS_PROP_SPEECH_RATE  => 
		{
			if (value == SPEECH_RATE_SET) {
				styleOffset += 1; // rate 
			}
		},

		CSS_PROP_VERTICAL_ALIGN  => 
		{
			if (value == VERTICAL_ALIGN_SET){
				styleOffset += 2; // length + units 
			}
		},

		CSS_PROP_VOICE_FAMILY  => 
		{
			while (value != VOICE_FAMILY_END) {
				match (value) 
				{
					VOICE_FAMILY_STRING  |	 VOICE_FAMILY_IDENT_LIST  => 
					{
						styleOffset += 1; // string table entry 
						break; // coming out of while loop
					}
				}

				value = styleBytecode[styleOffset];
				styleOffset += 1;
			}
		},

		CSS_PROP_VOLUME  => 
		{
			match (value) 
			{
				VOLUME_NUMBER  => styleOffset += 1, // value 
				VOLUME_DIMENSION  => styleOffset += 2 // value + units 
			}
		},
		CSS_PROP_Z_INDEX  => 
		{
			if (value == Z_INDEX_SET)
			{
				styleOffset += 1; // z index 
			}
		},
	}
	}	

	}	
	*/

	}

} // impl important ends




// ===========================================================================================================
// Important.h and important.c implementation/data-structs ends here 
// ===========================================================================================================



// ===========================================================================================================
// Font_Face.h and Font_Face.c implementation/data-structs starts here 
// ===========================================================================================================


// Sushanta :
// Ref: Propstrings.h
pub enum UniversalEnum
{

	/* Universal selector */
	UNIVERSAL,

	/* At-rules */

	// Sushanta: 
	// CHARSET is removed , 'coz it was clashing with charset valriable used below in the code.
	//CHARSET, LIBCSS_IMPORT, MEDIA, NAMESPACE, FONT_FACE, PAGE,
	LIBCSS_IMPORT, MEDIA, NAMESPACE, FONT_FACE, PAGE,

	/* Media types */
	AURAL, BRAILLE, EMBOSSED, HANDHELD, PRINT, PROJECTION, 
	SCREEN, SPEECH, TTY, TV, ALL,

	/* Pseudo classes */
	FIRST_CHILD, LINK, VISITED, HOVER, ACTIVE, FOCUS, LANG, 
	/* LEFT, RIGHT, -- already in properties */ FIRST,
	ROOT, NTH_CHILD, NTH_LAST_CHILD, NTH_OF_TYPE, NTH_LAST_OF_TYPE,
	LAST_CHILD, FIRST_OF_TYPE, LAST_OF_TYPE, ONLY_CHILD,
	ONLY_OF_TYPE, EMPTY, TARGET, ENABLED, DISABLED, CHECKED, NOT, 

	/* Pseudo elements */
	FIRST_LINE, FIRST_LETTER, BEFORE, AFTER,

	/* Properties */
	FIRST_PROP,


	// Below mentione line is commented temporarily to avoid compilation error
	//AZIMUTH = FIRST_PROP, 
	BACKGROUND, BACKGROUND_ATTACHMENT, 
	BACKGROUND_COLOR, BACKGROUND_IMAGE, BACKGROUND_POSITION, 
	BACKGROUND_REPEAT, BORDER, BORDER_BOTTOM, BORDER_BOTTOM_COLOR, 
	BORDER_BOTTOM_STYLE, BORDER_BOTTOM_WIDTH, BORDER_COLLAPSE, 
	BORDER_COLOR, BORDER_LEFT, BORDER_LEFT_COLOR, BORDER_LEFT_STYLE, 
	BORDER_LEFT_WIDTH, BORDER_RIGHT, BORDER_RIGHT_COLOR, 
	BORDER_RIGHT_STYLE, BORDER_RIGHT_WIDTH, BORDER_SPACING, 
	BORDER_STYLE, BORDER_TOP, BORDER_TOP_COLOR, BORDER_TOP_STYLE, 
	BORDER_TOP_WIDTH, BORDER_WIDTH, BOTTOM, BREAK_AFTER, BREAK_BEFORE,
	BREAK_INSIDE, CAPTION_SIDE, CLEAR, CLIP, COLOR, COLUMNS, COLUMN_COUNT,
	COLUMN_FILL, COLUMN_GAP, COLUMN_RULE, COLUMN_RULE_COLOR,
	COLUMN_RULE_STYLE, COLUMN_RULE_WIDTH, COLUMN_SPAN, COLUMN_WIDTH,
	CONTENT, COUNTER_INCREMENT, COUNTER_RESET, CUE, CUE_AFTER, CUE_BEFORE,
	CURSOR, DIRECTION, DISPLAY, ELEVATION, EMPTY_CELLS, LIBCSS_FLOAT, FONT,
	FONT_FAMILY, FONT_SIZE, FONT_STYLE, FONT_VARIANT, FONT_WEIGHT, HEIGHT,
	LEFT, LETTER_SPACING, LINE_HEIGHT, LIST_STYLE, LIST_STYLE_IMAGE,
	LIST_STYLE_POSITION, LIST_STYLE_TYPE, MARGIN, MARGIN_BOTTOM,
	MARGIN_LEFT, MARGIN_RIGHT, MARGIN_TOP, MAX_HEIGHT, MAX_WIDTH,
	MIN_HEIGHT, MIN_WIDTH, OPACITY, ORPHANS, OUTLINE, OUTLINE_COLOR,
	OUTLINE_STYLE, OUTLINE_WIDTH, OVERFLOW, PADDING, PADDING_BOTTOM,
	PADDING_LEFT, PADDING_RIGHT, PADDING_TOP, PAGE_BREAK_AFTER,
	PAGE_BREAK_BEFORE, PAGE_BREAK_INSIDE, PAUSE, PAUSE_AFTER, PAUSE_BEFORE,
	PITCH_RANGE, PITCH, PLAY_DURING, POSITION, QUOTES, RICHNESS, RIGHT,
	SPEAK_HEADER, SPEAK_NUMERAL, SPEAK_PUNCTUATION, SPEAK, SPEECH_RATE,
	STRESS, TABLE_LAYOUT, TEXT_ALIGN, TEXT_DECORATION, TEXT_INDENT,
	TEXT_TRANSFORM, TOP, UNICODE_BIDI, VERTICAL_ALIGN, VISIBILITY,
	VOICE_FAMILY, VOLUME, WHITE_SPACE, WIDOWS, WIDTH, WORD_SPACING, Z_INDEX,

	// Below mentione line is commented temporarily to avoid compilation error
	//LAST_PROP = Z_INDEX,

	/* Other keywords */
	INHERIT, IMPORTANT, NONE, BOTH, FIXED, SCROLL, TRANSPARENT,
	NO_REPEAT, REPEAT_X, REPEAT_Y, REPEAT, HIDDEN, DOTTED, DASHED,
	SOLID, LIBCSS_DOUBLE, GROOVE, RIDGE, INSET, OUTSET, THIN, MEDIUM, THICK,
	COLLAPSE, SEPARATE, AUTO, LTR, RTL, INLINE, BLOCK, LIST_ITEM, RUN_IN,
	INLINE_BLOCK, TABLE, INLINE_TABLE, TABLE_ROW_GROUP, TABLE_HEADER_GROUP,
	TABLE_FOOTER_GROUP, TABLE_ROW, TABLE_COLUMN_GROUP, TABLE_COLUMN,
	TABLE_CELL, TABLE_CAPTION, BELOW, LEVEL, ABOVE, HIGHER, LOWER,
	SHOW, HIDE, XX_SMALL, X_SMALL, SMALL, LARGE, X_LARGE, XX_LARGE,
	LARGER, SMALLER, NORMAL, ITALIC, OBLIQUE, SMALL_CAPS, BOLD, BOLDER,
	LIGHTER, INSIDE, OUTSIDE, DISC, CIRCLE, SQUARE, DECIMAL, 
	DECIMAL_LEADING_ZERO, LOWER_ROMAN, UPPER_ROMAN, LOWER_GREEK,
	LOWER_LATIN, UPPER_LATIN, ARMENIAN, GEORGIAN, LOWER_ALPHA, UPPER_ALPHA,
	INVERT, VISIBLE, ALWAYS, AVOID, X_LOW, LOW, HIGH, X_HIGH, LIBCSS_STATIC,
	RELATIVE, ABSOLUTE, ONCE, DIGITS, CONTINUOUS, CODE, SPELL_OUT, X_SLOW,
	SLOW, FAST, X_FAST, FASTER, SLOWER, CENTER, JUSTIFY, CAPITALIZE,
	UPPERCASE, LOWERCASE, EMBED, BIDI_OVERRIDE, BASELINE, SUB, SUPER,
	TEXT_TOP, MIDDLE, TEXT_BOTTOM, SILENT, X_SOFT, SOFT, LOUD, X_LOUD,
	PRE, NOWRAP, PRE_WRAP, PRE_LINE, LEFTWARDS, RIGHTWARDS, LEFT_SIDE,
	FAR_LEFT, CENTER_LEFT, CENTER_RIGHT, FAR_RIGHT, RIGHT_SIDE, BEHIND, 
	RECT, OPEN_QUOTE, CLOSE_QUOTE, NO_OPEN_QUOTE, NO_CLOSE_QUOTE, ATTR, 
	COUNTER, COUNTERS, CROSSHAIR, DEFAULT, POINTER, MOVE, E_RESIZE, 
	NE_RESIZE, NW_RESIZE, N_RESIZE, SE_RESIZE, SW_RESIZE, S_RESIZE, 
	W_RESIZE, LIBCSS_TEXT, WAIT, HELP, PROGRESS, SERIF, SANS_SERIF, CURSIVE,
	FANTASY, MONOSPACE, MALE, FEMALE, CHILD, MIX, UNDERLINE, OVERLINE, 
	LINE_THROUGH, BLINK, RGB, RGBA, HSL, HSLA, LIBCSS_LEFT, LIBCSS_CENTER,
	LIBCSS_RIGHT, CURRENTCOLOR, ODD, EVEN, SRC, LOCAL, INITIAL,
	FORMAT, WOFF, TRUETYPE, OPENTYPE, EMBEDDED_OPENTYPE, SVG, COLUMN,
	AVOID_PAGE, AVOID_COLUMN, BALANCE,

	/* Named colours */
	FIRST_COLOUR,

	// Below mentione line is commented temporarily to avoid compilation error
	// ALICEBLUE = FIRST_COLOUR, 
	ANTIQUEWHITE, AQUA, AQUAMARINE, AZURE,
	BEIGE, BISQUE, BLACK, BLANCHEDALMOND, BLUE, BLUEVIOLET, BROWN,
	BURLYWOOD, CADETBLUE, CHARTREUSE, CHOCOLATE, CORAL, CORNFLOWERBLUE,
	CORNSILK, CRIMSON, CYAN, DARKBLUE, DARKCYAN, DARKGOLDENROD, DARKGRAY,
	DARKGREEN, DARKGREY, DARKKHAKI, DARKMAGENTA, DARKOLIVEGREEN, DARKORANGE,
	DARKORCHID, DARKRED, DARKSALMON, DARKSEAGREEN, DARKSLATEBLUE,
	DARKSLATEGRAY, DARKSLATEGREY, DARKTURQUOISE, DARKVIOLET, DEEPPINK,
	DEEPSKYBLUE, DIMGRAY, DIMGREY, DODGERBLUE, FELDSPAR, FIREBRICK,
	FLORALWHITE, FORESTGREEN, FUCHSIA, GAINSBORO, GHOSTWHITE, GOLD, 
	GOLDENROD, GRAY, GREEN, GREENYELLOW, GREY, HONEYDEW, HOTPINK,
	INDIANRED, INDIGO, IVORY, KHAKI, LAVENDER, LAVENDERBLUSH, LAWNGREEN,
	LEMONCHIFFON, LIGHTBLUE, LIGHTCORAL, LIGHTCYAN, LIGHTGOLDENRODYELLOW,
	LIGHTGRAY, LIGHTGREEN, LIGHTGREY, LIGHTPINK, LIGHTSALMON, LIGHTSEAGREEN,
	LIGHTSKYBLUE, LIGHTSLATEBLUE, LIGHTSLATEGRAY, LIGHTSLATEGREY, 
	LIGHTSTEELBLUE, LIGHTYELLOW, LIME, LIMEGREEN, LINEN, MAGENTA, MAROON,
	MEDIUMAQUAMARINE, MEDIUMBLUE, MEDIUMORCHID, MEDIUMPURPLE, 
	MEDIUMSEAGREEN, MEDIUMSLATEBLUE, MEDIUMSPRINGGREEN, MEDIUMTURQUOISE,
	MEDIUMVIOLETRED, MIDNIGHTBLUE, MINTCREAM, MISTYROSE, MOCCASIN,
	NAVAJOWHITE, NAVY, OLDLACE, OLIVE, OLIVEDRAB, ORANGE, ORANGERED,
	ORCHID, PALEGOLDENROD, PALEGREEN, PALETURQUOISE, PALEVIOLETRED,
	PAPAYAWHIP, PEACHPUFF, PERU, PINK, PLUM, POWDERBLUE, PURPLE, RED,
	ROSYBROWN, ROYALBLUE, SADDLEBROWN, SALMON, SANDYBROWN, SEAGREEN,
	SEASHELL, SIENNA, SILVER, SKYBLUE, SLATEBLUE, SLATEGRAY, SLATEGREY,
	SNOW, SPRINGGREEN, STEELBLUE, TAN, TEAL, THISTLE, TOMATO, TURQUOISE, 
	VIOLET, VIOLETRED, WHEAT, WHITE, WHITESMOKE, YELLOW, YELLOWGREEN,

	// Below mentione line is commented to avoid temporarily compilation error
	// LAST_COLOUR = YELLOWGREEN,

	LAST_KNOWN
}


pub enum css_font_face_format {
	CSS_FONT_FACE_FORMAT_UNSPECIFIED	= 	0x00,
	CSS_FONT_FACE_FORMAT_WOFF		= 	0x01,
	CSS_FONT_FACE_FORMAT_OPENTYPE		= 	0x02,
	CSS_FONT_FACE_FORMAT_EMBEDDEDOPENTYPE	= 	0x04,
	CSS_FONT_FACE_FORMAT_SVG		= 	0x08,
	CSS_FONT_FACE_FORMAT_UNKNOWN		= 	0x10
}


pub struct css_rule_face {
	base :css_rule,
	mut font_face:@css_font_face 
}

pub enum css_font_face_location_type{
		CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED 	= 	0,
		CSS_FONT_FACE_LOCATION_TYPE_LOCAL 		= 	1,
		CSS_FONT_FACE_LOCATION_TYPE_URI 		= 	2
			
	}
	

pub struct font_face
{
	mut lpu_instance : @lpu,
	    mut lwc_instance : @lwc,

	    // Q: Should I include below mentioned functionas also in the structure ?
	    //	cssLang : @css_language,		
	    //	rule : @css_rule_face,
	    //	font_face_location_type : @css_font_face_location_type,
}

pub fn font_face()->@font_face
{
	@font_face
	{
lpu_instance : lpu(),
	     lwc_instance : lwc()				
	}
}

impl font_face
{

	pub fn tokenIsChar(&self, token : @css_token, charData : u8) -> bool
	{
		let mut result : bool = false;

		//if token.token_type == 0x09 && lwc::lwc_string_length(token.idata) == 1
		match (token.token_type)	
		{
			CSS_TOKEN_CHAR	=>		
			{
				let mut tempCharData : u8 = lwc::lwc_string_data(token.idata)[0]; 

				// ensuring lowerCase comparison
				if tempCharData >= 'A' as u8 && tempCharData <= 'Z' as u8
				{
					tempCharData += ('a' - 'A') as u8;	
				}
				if tempCharData == charData
				{
					result = true;	
				}
				else
				{
					result = false;	
				}
			},
				_	=>	()
		}
		return result; 
	}

	// consumes all whiteSpace tokens
	pub fn consumeWhiteSpace(&self, tokenVector : ~[~css_token])
	{	
		for tokenVector.each |&iter|
		{
			match (iter.token_type) 
			{
				CSS_TOKEN_S	=>	break,
						_		=>	{}
			}	
		}	
	}	


	//pub fn css_parse_font_descriptor(&self , cssLang : @css_language, descriptor : ~css_token, vector : ~parseutils_vector, cntx : ~int, rule_face : &css_rule_face) -> css_result
	pub fn css_parse_font_descriptor(&self, cssLang : @css_language, descriptor : ~css_token, tokenVector : ~[~css_token], mut rule_face : &css_rule_face) -> css_result
	{
		let mut font_face  = rule_face.font_face ;

		// initializing temporarily
		let errorVal : css_result = CSS_GENERAL_OK;

		// css_font_face_create Need to be implemented	

		// Commenting temporarily to avoicd compilation error
		//errorVal = self.css_font_face_create(cssLang.sheet.alloc, cssLang.sheet.pw, &font_face);  /* &font_face is this right ?*/ 

		match errorVal
		{
			CSS_GENERAL_OK	=> 	{
				rule_face.font_face = font_face;	
			},
					_		=>	{
						return errorVal
					}
		}

		if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[FONT_FAMILY as int]) == true 
		{
			return self.font_face_parse_font_family(cssLang, tokenVector, font_face);		
		}
		else if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[SRC as int]) ==  true 
		{
			return self.font_face_parse_src(cssLang, tokenVector, font_face);		
		}
		else if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[FONT_STYLE as int]) == true 
		{
			return self.font_face_parse_font_style(cssLang, tokenVector, font_face);		
		}
		else if lwc::lwc_string_caseless_isequal(descriptor.idata, cssLang.strings[FONT_WEIGHT as int]) ==  true
		{
			return self.font_face_parse_font_weight(cssLang, tokenVector, font_face);		
		}

		return CSS_GENERAL_OK;	
	}

	pub fn font_rule_font_family_reserved (&self, cssLang : @css_language, identifier : ~css_token) -> bool
	{

		return(		lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[SERIF as int]) == true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[SANS_SERIF as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[CURSIVE as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[FANTASY as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[MONOSPACE as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[INHERIT as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[INITIAL as int]) ==  true || 
				lwc::lwc_string_caseless_isequal(identifier.idata, cssLang.strings[DEFAULT as int]) ==  true
		      ) ;
	}

	// pub fn font_face_parse_font_family (&self, cssLang : @css_language, vector : ~parseutils_vector, cntx : ~int,  font_face : ~css_font_face) -> css_result
	pub fn font_face_parse_font_family (&self, cssLang : @css_language, tokenVector : ~[~css_token], font_face : @css_font_face) -> css_result
	{
		// Corresponding C code shows, that a function itself is being passed as an argument 
		// Need to chk how come it is possible ?
		/*
		   let stringData : ~lwc_instance.lwc_string ;
		   let error : css_result =  	
		 */

		return CSS_GENERAL_OK;
	}

	// pub fn font_face_src_parse_format (&self, cssLang : @css_language, vector : ~parseutils_vector, cntx : ~int,  font_face_format : ~css_font_face) -> css_result
	pub fn font_face_src_parse_format (&self, cssLang : @css_language, tokenVector : ~[~css_token], mut font_face_format : css_font_face_format) -> css_result
	{
		font_face_format = CSS_FONT_FACE_FORMAT_UNSPECIFIED;	

		self.consumeWhiteSpace(copy tokenVector);
		//let token : ~css_token = self.lpu_instance.parseutils_vector_iterate(tokenVector); 

		for tokenVector.each |&token|
		{
			match (token.token_type)
			{
				CSS_TOKEN_STRING	=>	{
					if  lwc::lwc_string_isequal(token.idata, cssLang.strings[WOFF as int]) == true 
					{
						// Sushanta: How to implement bitwise operator in RUST?
						// font_face_format |= CSS_FONT_FACE_FORMAT_WOFF ; 	
					}
					else if   lwc::lwc_string_isequal(token.idata, cssLang.strings[TRUETYPE as int]) ==  true  ||
						lwc::lwc_string_isequal(token.idata, cssLang.strings[OPENTYPE as int]) == true 
						{
							// font_face_format |= CSS_FONT_FACE_FORMAT_OPENTYPE ; 	
						}
					else if   lwc::lwc_string_isequal(token.idata, cssLang.strings[EMBEDDED_OPENTYPE as int]) == true 
					{
							//font_face_format |= CSS_FONT_FACE_FORMAT_EMBEDDEDOPENTYPE ; 	
					}
					else if   lwc::lwc_string_isequal(token.idata, cssLang.strings[SVG as int]) == true 
					{
					//	font_face_format |= CSS_FONT_FACE_FORMAT_SVG; 	
					}
					else
					{	
						//font_face_format |= CSS_FONT_FACE_FORMAT_UNKNOWN; 	
					}	

					// Sushanta: Do we need this ?
					// self.consumeWhiteSpace(tokenVector);

				},
							_			=>	return CSS_INVALID	
			}

		}

		// temporary comment
		/*
		if (self.tokenIsChar(token, ')') == false)
		{
			return CSS_INVALID;	
		}
		*/
		
		return CSS_GENERAL_OK;
	}

	// pub fn font_face_src_parse_spec_or_name (&self, cssLang : @css_language, vector : ~parseutils_vector, cntx : ~int, location : ~lwc_string, font_face_location_type : ~css_font_face_location_type, font_face_format : ~css_font_face) -> css_result
	pub fn font_face_src_parse_spec_or_name (&self, cssLang : @css_language, tokenVector : ~[~css_token], location : ~lwc_string, mut font_face_location_type : css_font_face_location_type, font_face_format : css_font_face_format) -> css_result
	{
		let mut errorVal : css_result = CSS_GENERAL_OK;
		self.consumeWhiteSpace(copy tokenVector);		
		//let token : ~css_token = self.lpu_instance.parseutils_vector_iterate(tokenVector); 

		for tokenVector.each |&token|
		{
			//if token.type == CSS_TOKEN_URI
			match(token.token_type) 
			{
				CSS_TOKEN_URI	=> {
					// Sushanta: temporary comment
					/*
					errorVal = cssLang.sheet.resolve(cssLang.sheet.resolve_pw, cssLang.sheet.url, token.idata, location);
					if errorVal != CSS_GENERAL_OK
					{
						return errorVal;
					}
					*/
	
					font_face_location_type = CSS_FONT_FACE_LOCATION_TYPE_URI; 
					//self.consumeWhiteSpace(tokenVector);		

					// Sushanta: 	
					//token =  self.lpu_instance.parserutils_vector_peek(tokenVector);

					match(token.token_type)
					{
						CSS_TOKEN_FUNCTION 	=> {
							if  (lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[FORMAT as int]) == true) 
							{
								//self.lpu_instance.parserutils_vector_iterate(tokenVector);	
								 //  Sushanta: Do we need this
								/*
								errorVal = self.font_face_src_parse_format(cssLang, tokenVector, font_face_format);

								match(errorVal)
								{
									CSS_GENERAL_OK  =>	return errorVal,
											_	=>	()
								}
								*/													
							}	

						},
									_			=>	()

					}				

					},
						CSS_TOKEN_FUNCTION 	=>	{
							if (lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[LOCAL as int]) == true)
							{
								// Sushanta: Do we need this fun
								//self.consumeWhiteSpace(tokenVector);	
								// below mentioned fun is defined in Parse/Properties/Utils.c
								// think abt 4th parameter namely NULL

								// Below mentioned function calls function pointer, Commenting temporarily
								// Commenting to avoid compilation error
								/*
								errorVal = css_ident_list_or_string_to_string(cssLang, tokenVector, NULL, location);

								if errorVal != CSS_GENERAL_OK 
								{
									return errorVal;	
								}
								*/

								// Sushanta: Do we need this fun
								//self.consumeWhiteSpace(tokenVector);
								
								// see precisely at which condition CSS_INVALID should be called
								/*
								token =  self.lpu_instance.parserutils_vector_iterate(tokenVector);	

								if self.tokenIsChar(token, ')') == false
								{
									return CSS_INVALID;	
								}
								*/

								font_face_location_type = CSS_FONT_FACE_LOCATION_TYPE_LOCAL; 
							}
						},
						_	=>	{
							return CSS_INVALID;	
						}
			} // match block ends
		}

		return CSS_GENERAL_OK;
	}

	// In this function, LABELs are used
	// pub fn font_face_parse_src (&self, cssLang : @css_language, vector : ~parseutils_vector, cntx : ~int,  font_face : ~css_font_face) -> css_result
	pub fn font_face_parse_src (&self, cssLang : @css_language, tokenVector : ~[~css_token], font_face : @css_font_face) -> css_result
	{
		//let orig_cntx : int = *cntx;
		let errorVal : css_result = CSS_GENERAL_OK;	
		let n_srcs : u32 = 0;	
		let srcs : ~css_font_face_src;
		let new_srcs : ~css_font_face_src;

		self.consumeWhiteSpace(copy tokenVector);
		//token = self.lpu_instance.parserutils_vector_iterate(tokenVector);

		//while self.tokenIsChar(token, ',')
			
		for tokenVector.each |&token|
		{
			let font_face_location : ~lwc_string ;
			let font_face_location_type : css_font_face_location_type = CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED;
			let font_face_format : css_font_face_format = CSS_FONT_FACE_FORMAT_UNSPECIFIED; 	

			// Sushanta: Temporary comment
			//errorVal =  self.font_face_src_parse_spec_or_name(cssLang, tokenVector, font_face_location, font_face_location_type, font_face_format);  
			//if errorVal != CSS_GENERAL_OK
			match(errorVal)
			{
				CSS_GENERAL_OK 	=>
				{
					// *cntx = orig_cntx;
					// Q: what to do here ?
					// if srcs != NULL 			
				},
					_		=>	()
			}

			// But how / where new_srcs is initialized ?

			// Sushanta: commenting temporarily
			/*
			srcs = new_srcs;
			srcs[n_srcs].location = font_face_location; 		

			srcs[n_srcs].bits[0] = font_face_format << 2 | font_face_location_type;
			n_srcs += 1;		
			*/
			// Do we need this	
			//self.consumeWhiteSpace(tokenVector);
		}

		// Q: below mentioned function is defined in src/selects/Font_face.c

		// Sushanta: commenting temporarily
		//errorVal =  self.css_font_face_set_srcs(font_face, srcs, n_srcs); 

		return errorVal;
	}

	//pub fn font_face_parse_font_style (&self, cssLang : @css_language, vector : ~parseutils_vector, cntx : ~int,  font_face : ~css_font_face) -> css_result
	pub fn font_face_parse_font_style (&self, cssLang : @css_language, tokenVector : ~[~css_token], font_face : @css_font_face) -> css_result
	{
		let mut errorVal : css_result = CSS_GENERAL_OK; 
		let mut style : css_font_style_e = CSS_FONT_STYLE_INHERIT; 

		for tokenVector.each |&token|
		{
			match(token.token_type)
			{
				CSS_TOKEN_IDENT		=>	return CSS_INVALID,
				_			=>	{
								if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[NORMAL as int]) == true 
								{
									style = CSS_FONT_STYLE_NORMAL;	
								}
								else if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[ITALIC as int]) == true
								{
									style = CSS_FONT_STYLE_ITALIC;	
								}
								else if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[OBLIQUE as int]) == true
								{
									style = CSS_FONT_STYLE_OBLIQUE;	
								}
								else
								{
									errorVal = CSS_INVALID;
								}
							}

			}	

		}

		// commenting temporarily
		/*
		   if errorVal == CSS_GENERAL_OK
		   {
		   font_face.bits[0] = (font_face.bits[0] & 0xfc) | style;
		   }
		   else
		   {
		//*cntx = orig_cntx;	
		();
		}	
		 */

		return 	errorVal;
	}

	// pub fn font_face_parse_font_weight (cssLang : @css_language, vector : ~lpu_instance. parseutils_vector, cntx : ~int,  font_face : ~css_font_face) -> css_result
	pub fn font_face_parse_font_weight (&self, cssLang : @css_language, tokenVector : ~[~css_token],  font_face : @css_font_face) -> css_result
	{
		let mut errorVal : css_result = CSS_GENERAL_OK; 
		let mut weight : css_font_weight_e = CSS_FONT_WEIGHT_INHERIT ; 
		// let token : ~css_token = self.lpu_instance.parseutils_vector_iterate(tokenVector);

		for tokenVector.each |&token|
		{
			match token.token_type
			{
				CSS_TOKEN_NUMBER	=>{
					let consumed : uint = 0;

					//  is it right to write @consumed, or should I write &consumed
					// I think "&consumed" is for READONLY values

					// below mentioned function namely "css_number_from_lwc_string" is writtten in utils.c
					// sushanta: commenting temporarily 
					/*
					let number : u32 = self.css_number_from_lwc_string(token.idata, true, @consumed);
				
					// invalid if there are trailing characters  
					if consumed != lwc::lwc_string_length(token.idata)
					{
						return CSS_INVALID;	
					}
					*/
					//sushanta: temporary initialization
					let number : u32 = 0x00;

					match(number >>10)
					{
						100	=>	weight = CSS_FONT_WEIGHT_100,		
							200	=>	weight = CSS_FONT_WEIGHT_200,		
							300	=>	weight = CSS_FONT_WEIGHT_300,		
							400	=>	weight = CSS_FONT_WEIGHT_400,		
							500	=>	weight = CSS_FONT_WEIGHT_500,		
							600	=>	weight = CSS_FONT_WEIGHT_600,		
							700	=>	weight = CSS_FONT_WEIGHT_700,		
							800	=>	weight = CSS_FONT_WEIGHT_800,		
							900	=>	weight = CSS_FONT_WEIGHT_900,		
							_	=>	return CSS_INVALID


					}		
				},
							CSS_TOKEN_IDENT		=>{
								if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[NORMAL as int]) == true
								{
									weight = CSS_FONT_WEIGHT_NORMAL;	
								}
								else if lwc::lwc_string_caseless_isequal(token.idata, cssLang.strings[BOLD as int]) == true
								{
									weight = CSS_FONT_WEIGHT_BOLD;	
								}
								else
								{
									errorVal = CSS_INVALID;
								}	

								match (errorVal)
								{
									CSS_GENERAL_OK  =>	{
										// font_face.bits[0] = (font_face.bits[0] & 0xc3) | (weight << 2);	
									}
									_		=>	{	
										();
									}
								}
							},
				_		=>	()	

			} // match token.token_type ends 
		} // for loop ends
		return errorVal;
	}	// end of font_face_pasre_font_wweight	
					
}


// ===========================================================================================================
// Font_Face.h and Font_Face.c implementation/data-structs starts here 
// ===========================================================================================================