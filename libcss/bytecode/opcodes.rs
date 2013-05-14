pub static AZIMUTH_ANGLE:u16            = 0x0080;

pub static AZIMUTH_LEFTWARDS:u16        = 0x0040;
pub static AZIMUTH_RIGHTWARDS:u16       = 0x0041;

pub static AZIMUTH_BEHIND:u16           = (1<<5);
pub static AZIMUTH_LEFT_SIDE:u16        = 0x0000;
pub static AZIMUTH_FAR_LEFT:u16     = 0x0001;
pub static AZIMUTH_LEFT:u16         = 0x0002;
pub static AZIMUTH_CENTER_LEFT:u16      = 0x0003;
pub static AZIMUTH_CENTER:u16           = 0x0004;
pub static AZIMUTH_CENTER_RIGHT :u16    = 0x0005;
pub static AZIMUTH_RIGHT:u16            = 0x0006;
pub static AZIMUTH_FAR_RIGHT:u16        = 0x0007;
pub static AZIMUTH_RIGHT_SIDE:u16       = 0x0008;


pub static BACKGROUND_ATTACHMENT_FIXED: u16 = 0x0000;
pub static BACKGROUND_ATTACHMENT_SCROLL: u16 = 0x0001;  

pub static BACKGROUND_COLOR_TRANSPARENT: u16   = 0x0000;
pub static BACKGROUND_COLOR_CURRENT_COLOR: u16 = 0x0001;
pub static BACKGROUND_COLOR_SET : u16   = 0x0080 ;

pub static BACKGROUND_IMAGE_URI : u16 = 0x0080;
pub static BACKGROUND_IMAGE_NONE: u16 = 0x0000 ;

pub static BACKGROUND_POSITION_HORZ_SET: u16    = 0x0080;
pub static BACKGROUND_POSITION_HORZ_CENTER: u16 = 0x0000;
pub static BACKGROUND_POSITION_HORZ_RIGHT: u16  = 0x0010;
pub static BACKGROUND_POSITION_HORZ_LEFT: u16   = 0x0020;

pub static BACKGROUND_POSITION_VERT_SET: u16    = 0x0008;
pub static BACKGROUND_POSITION_VERT_CENTER: u16 = 0x0000;
pub static BACKGROUND_POSITION_VERT_BOTTOM: u16 = 0x0001;
pub static BACKGROUND_POSITION_VERT_TOP: u16    = 0x0002 ;

pub static BACKGROUND_REPEAT_NO_REPEAT: u16 = 0x0000;
pub static BACKGROUND_REPEAT_REPEAT_X : u16 = 0x0001;
pub static BACKGROUND_REPEAT_REPEAT_Y : u16 = 0x0002;
pub static BACKGROUND_REPEAT_REPEAT   : u16 = 0x0003 ;

pub static    BORDER_COLLAPSE_SEPARATE : u16    = 0x0000;
pub static    BORDER_COLLAPSE_COLLAPSE : u16    = 0x0001;

pub static BORDER_SPACING_SET: u16 = 0x0080;

pub static BORDER_COLOR_TRANSPARENT: u16    = 0x0000;
pub static BORDER_COLOR_CURRENT_COLOR: u16  = 0x0001;
pub static BORDER_COLOR_SET: u16        = 0x0080;

pub static BORDER_STYLE_NONE: u16       = 0x0000;
pub static BORDER_STYLE_HIDDEN: u16     = 0x0001;
pub static BORDER_STYLE_DOTTED: u16     = 0x0002;
pub static BORDER_STYLE_DASHED: u16     = 0x0003;
pub static BORDER_STYLE_SOLID: u16      = 0x0004;
pub static BORDER_STYLE_DOUBLE: u16     = 0x0005;
pub static BORDER_STYLE_GROOVE: u16     = 0x0006;
pub static BORDER_STYLE_RIDGE: u16      = 0x0007;
pub static BORDER_STYLE_INSET: u16      = 0x0008;
pub static BORDER_STYLE_OUTSET: u16     = 0x0009;

pub static BORDER_WIDTH_SET: u16        = 0x0080;
pub static BORDER_WIDTH_THIN: u16       = 0x0000;
pub static BORDER_WIDTH_MEDIUM: u16     = 0x0001;
pub static BORDER_WIDTH_THICK: u16      = 0x0002;

pub static    BOTTOM_SET: u16          = 0x0080;
pub static    BOTTOM_AUTO: u16         = 0x0000;

pub static    BREAK_AFTER_AUTO: u16        = 0x0000;
pub static    BREAK_AFTER_ALWAYS: u16      = 0x0001;
pub static    BREAK_AFTER_AVOID : u16      = 0x0002;
pub static    BREAK_AFTER_LEFT: u16        = 0x0003;
pub static    BREAK_AFTER_RIGHT : u16      = 0x0004;
pub static    BREAK_AFTER_PAGE  : u16      = 0x0005;
pub static    BREAK_AFTER_COLUMN : u16     = 0x0006;
pub static    BREAK_AFTER_AVOID_PAGE : u16     = 0x0007;
pub static    BREAK_AFTER_AVOID_COLUMN : u16   = 0x0008;

pub static   BREAK_BEFORE_AUTO      : u16 = 0x0000;
pub static   BREAK_BEFORE_ALWAYS    : u16 = 0x0001;
pub static   BREAK_BEFORE_AVOID     : u16 = 0x0002;
pub static   BREAK_BEFORE_LEFT      : u16 = 0x0003;
pub static   BREAK_BEFORE_RIGHT     : u16 = 0x0004;
pub static   BREAK_BEFORE_PAGE      : u16 = 0x0005;
pub static   BREAK_BEFORE_COLUMN    : u16 = 0x0006;
pub static   BREAK_BEFORE_AVOID_PAGE    : u16 = 0x0007;
pub static   BREAK_BEFORE_AVOID_COLUMN : u16 =  0x0008;

pub static    BREAK_INSIDE_AUTO     : u16  = 0x0000;
pub static    BREAK_INSIDE_AVOID    : u16  = 0x0001;
pub static    BREAK_INSIDE_AVOID_PAGE   : u16  = 0x0002;
pub static    BREAK_INSIDE_AVOID_COLUMN : u16  = 0x0003;

pub static    CAPTION_SIDE_TOP    : u16    = 0x0000;
pub static    CAPTION_SIDE_BOTTOM : u16    = 0x0001;

pub static    CLEAR_NONE      : u16    = 0x0000;
pub static    CLEAR_LEFT      : u16    = 0x0001;
pub static    CLEAR_RIGHT     : u16    = 0x0002;
pub static    CLEAR_BOTH      : u16    = 0x0003;

pub static CLIP_SHAPE_MASK: u16         = 0x0087;
pub static CLIP_SHAPE_RECT: u16         = 0x0080;

pub static CLIP_RECT_TOP_AUTO: u16      = 0x0008;
pub static CLIP_RECT_RIGHT_AUTO: u16        = 0x0010;
pub static CLIP_RECT_BOTTOM_AUTO: u16       = 0x0020;
pub static CLIP_RECT_LEFT_AUTO: u16     = 0x0040;

pub static CLIP_AUTO: u16           = 0x0000;

pub static    COLOR_TRANSPARENT   : u16     = 0x0000;
pub static    COLOR_CURRENT_COLOR : u16     = 0x0001;
pub static    COLOR_SET       : u16     = 0x0080;


pub static COLUMN_COUNT_AUTO: u16       = 0x0000;
pub static COLUMN_COUNT_SET: u16        = 0x0080;

pub static    COLUMN_FILL_BALANCE : u16     = 0x0000;
pub static    COLUMN_FILL_AUTO    : u16     = 0x0001;

pub static    COLUMN_GAP_NORMAL   : u16     = 0x0000;
pub static    COLUMN_GAP_SET      : u16     = 0x0080;

pub static COLUMN_RULE_COLOR_TRANSPARENT:u16    = 0x0000;
pub static COLUMN_RULE_COLOR_CURRENT_COLOR:u16  = 0x0001;
pub static COLUMN_RULE_COLOR_INVERT:u16 = 0x0002;
pub static COLUMN_RULE_COLOR_SET:u16        = 0x0080 ;


pub static COLUMN_RULE_STYLE_NONE  : u16  = 0x0000;
pub static COLUMN_RULE_STYLE_HIDDEN: u16  = 0x0001;
pub static COLUMN_RULE_STYLE_DOTTED: u16  = 0x0002;
pub static COLUMN_RULE_STYLE_DASHED: u16  = 0x0003;
pub static COLUMN_RULE_STYLE_SOLID : u16  = 0x0004;
pub static COLUMN_RULE_STYLE_DOUBLE: u16  = 0x0005;
pub static COLUMN_RULE_STYLE_GROOVE: u16  = 0x0006;
pub static COLUMN_RULE_STYLE_RIDGE : u16  = 0x0007;
pub static COLUMN_RULE_STYLE_INSET : u16  = 0x0008;
pub static COLUMN_RULE_STYLE_OUTSET : u16 = 0x0009 ;

pub static COLUMN_RULE_WIDTH_SET   : u16    = 0x0080;
pub static COLUMN_RULE_WIDTH_THIN  : u16    = 0x0000;
pub static COLUMN_RULE_WIDTH_MEDIUM: u16    = 0x0001;
pub static COLUMN_RULE_WIDTH_THICK : u16    = 0x0002;

pub static    COLUMN_SPAN_NONE : u16        = 0x0000;
pub static    COLUMN_SPAN_ALL  : u16        = 0x0001;


pub static COLUMN_WIDTH_AUTO: u16       = 0x0000;
pub static COLUMN_WIDTH_SET: u16        = 0x0080;

pub static CONTENT_STRING: u16          = 0x0080;
pub static CONTENT_URI : u16        = 0x0081;
pub static CONTENT_COUNTER: u16         = 0x0082;
pub static CONTENT_COUNTERS: u16        = 0x0083;
pub static CONTENT_ATTR : u16           = 0x0084;

pub static CONTENT_COUNTER_STYLE_SHIFT: u16 = 8;
pub static CONTENT_COUNTERS_STYLE_SHIFT: u16  = 8;

pub static CONTENT_NORMAL: u16          = 0x0000;
pub static CONTENT_NONE : u16           = 0x0001;
pub static CONTENT_OPEN_QUOTE: u16      = 0x0002;
pub static CONTENT_CLOSE_QUOTE: u16     = 0x0003;
pub static CONTENT_NO_OPEN_QUOTE: u16       = 0x0004;
pub static CONTENT_NO_CLOSE_QUOTE: u16      = 0x0005;


pub static    COUNTER_INCREMENT_NAMED : u16     = 0x0080;

pub static    COUNTER_INCREMENT_NONE  : u16     = 0x0000;

pub static    COUNTER_RESET_NAMED : u16     = 0x0080;

pub static    COUNTER_RESET_NONE  : u16     = 0x0000;

pub static    CUE_AFTER_URI       : u16     = 0x0080;
pub static    CUE_AFTER_NONE      : u16     = 0x0000;

pub static    CUE_BEFORE_URI      : u16     = 0x0080;
pub static    CUE_BEFORE_NONE     : u16     = 0x0000;

pub static CURSOR_URI: u16          = 0x0080;
pub static CURSOR_AUTO: u16         = 0x0000;
pub static CURSOR_CROSSHAIR: u16        = 0x0001;
pub static CURSOR_DEFAULT: u16          = 0x0002;
pub static CURSOR_POINTER: u16          = 0x0003;
pub static CURSOR_MOVE: u16         = 0x0004;
pub static CURSOR_E_RESIZE  : u16       = 0x0005;
pub static CURSOR_NE_RESIZE: u16        = 0x0006;
pub static CURSOR_NW_RESIZE: u16        = 0x0007;
pub static CURSOR_N_RESIZE  : u16       = 0x0008;
pub static CURSOR_SE_RESIZE: u16        = 0x0009;
pub static CURSOR_SW_RESIZE: u16        = 0x000a;
pub static CURSOR_S_RESIZE  : u16       = 0x000b;
pub static CURSOR_W_RESIZE  : u16       = 0x000c;
pub static CURSOR_TEXT      : u16   = 0x000d;
pub static CURSOR_WAIT  : u16       = 0x000e;
pub static CURSOR_HELP  : u16       = 0x000f;
pub static CURSOR_PROGRESS  : u16       = 0x0010;

pub static    DIRECTION_LTR          : u16  = 0x0000;
pub static    DIRECTION_RTL          : u16  = 0x0001;

pub static    DISPLAY_INLINE         : u16  = 0x0000;
pub static    DISPLAY_BLOCK          : u16  = 0x0001;
pub static    DISPLAY_LIST_ITEM      : u16  = 0x0002;
pub static    DISPLAY_RUN_IN         : u16  = 0x0003;
pub static    DISPLAY_INLINE_BLOCK       : u16  = 0x0004;
pub static    DISPLAY_TABLE          : u16  = 0x0005;
pub static    DISPLAY_INLINE_TABLE       : u16  = 0x0006;
pub static    DISPLAY_TABLE_ROW_GROUP    : u16  = 0x0007;
pub static    DISPLAY_TABLE_HEADER_GROUP : u16  = 0x0008;
pub static    DISPLAY_TABLE_FOOTER_GROUP : u16  = 0x0009;
pub static    DISPLAY_TABLE_ROW      : u16  = 0x000a;
pub static    DISPLAY_TABLE_COLUMN_GROUP : u16  = 0x000b;
pub static    DISPLAY_TABLE_COLUMN       : u16  = 0x000c;
pub static    DISPLAY_TABLE_CELL     : u16  = 0x000d;
pub static    DISPLAY_TABLE_CAPTION      : u16  = 0x000e;
pub static    DISPLAY_NONE           : u16  = 0x000f;


pub static ELEVATION_ANGLE: u16 = 0x0080;
pub static ELEVATION_BELOW: u16 = 0x0000;
pub static ELEVATION_LEVEL: u16 = 0x0001;
pub static ELEVATION_ABOVE: u16 = 0x0002;
pub static ELEVATION_HIGHER: u16 = 0x0003;
pub static ELEVATION_LOWER: u16 = 0x0004;

pub static    EMPTY_CELLS_SHOW    : u16    = 0x0000;
pub static    EMPTY_CELLS_HIDE    : u16    = 0x0001;

pub static    FLOAT_LEFT      : u16    = 0x0000;
pub static    FLOAT_RIGHT     : u16    = 0x0001;
pub static    FLOAT_NONE      : u16    = 0x0002;

pub static    FONT_FAMILY_STRING   : u16   = 0x0080;
pub static    FONT_FAMILY_IDENT_LIST   : u16   = 0x0081;

pub static    FONT_FAMILY_END    : u16     = 0x0000;

pub static    FONT_FAMILY_SERIF   : u16    = 0x0001;
pub static    FONT_FAMILY_SANS_SERIF   : u16   = 0x0002;
pub static    FONT_FAMILY_CURSIVE  : u16   = 0x0003;
pub static    FONT_FAMILY_FANTASY  : u16   = 0x0004;
pub static    FONT_FAMILY_MONOSPACE    : u16   = 0x0005;

pub static    FONT_SIZE_DIMENSION  : u16   = 0x0080;

pub static    FONT_SIZE_XX_SMALL  : u16    = 0x0000;
pub static    FONT_SIZE_X_SMALL   : u16    = 0x0001;
pub static    FONT_SIZE_SMALL     : u16    = 0x0002;
pub static    FONT_SIZE_MEDIUM    : u16    = 0x0003;
pub static    FONT_SIZE_LARGE     : u16    = 0x0004;
pub static    FONT_SIZE_X_LARGE   : u16    = 0x0005;
pub static    FONT_SIZE_XX_LARGE  : u16    = 0x0006;
pub static    FONT_SIZE_LARGER    : u16    = 0x0007;
pub static    FONT_SIZE_SMALLER   : u16    = 0x0008;

pub static FONT_STYLE_NORMAL: u16 = 0x0000;
pub static FONT_STYLE_ITALIC: u16 = 0x0001;
pub static FONT_STYLE_OBLIQUE: u16 = 0x0002;

pub static FONT_VARIANT_NORMAL: u16 = 0x0000;
pub static FONT_VARIANT_SMALL_CAPS: u16 = 0x0001;

pub static FONT_WEIGHT_NORMAL: u16 = 0x0000;
pub static FONT_WEIGHT_BOLD: u16 = 0x0001;
pub static FONT_WEIGHT_BOLDER: u16 = 0x0002;
pub static FONT_WEIGHT_LIGHTER: u16 = 0x0003;
pub static FONT_WEIGHT_100: u16 = 0x0004;
pub static FONT_WEIGHT_200: u16 = 0x0005;
pub static FONT_WEIGHT_300: u16 = 0x0006;
pub static FONT_WEIGHT_400: u16 = 0x0007;
pub static FONT_WEIGHT_500: u16 = 0x0008;
pub static FONT_WEIGHT_600: u16 = 0x0009;
pub static FONT_WEIGHT_700: u16 = 0x000a;
pub static FONT_WEIGHT_800: u16 = 0x000b;
pub static FONT_WEIGHT_900: u16 = 0x000c;

pub static    HEIGHT_SET     : u16      = 0x0080;
pub static    HEIGHT_AUTO    : u16      = 0x0000;

pub static    LEFT_SET       : u16      = 0x0080;
pub static    LEFT_AUTO      : u16      = 0x0000;

pub static    LETTER_SPACING_SET : u16      = 0x0080;
pub static    LETTER_SPACING_NORMAL  : u16      = 0x0000;


pub static LINE_HEIGHT_NUMBER: u16   = 0x0080;
pub static LINE_HEIGHT_DIMENSION: u16    = 0x0081;
pub static LINE_HEIGHT_NORMAL: u16   = 0x0000;


pub static LIST_STYLE_IMAGE_URI: u16 = 0x0080;
pub static LIST_STYLE_IMAGE_NONE: u16 = 0x0000;


pub static LIST_STYLE_POSITION_INSIDE: u16 = 0x0000;
pub static LIST_STYLE_POSITION_OUTSIDE: u16 = 0x0001;


pub static LIST_STYLE_TYPE_DISC: u16    = 0x0000;
pub static LIST_STYLE_TYPE_CIRCLE: u16  = 0x0001;
pub static LIST_STYLE_TYPE_SQUARE: u16  = 0x0002;
pub static LIST_STYLE_TYPE_DECIMAL: u16 = 0x0003;
pub static LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO: u16    = 0x0004;
pub static LIST_STYLE_TYPE_LOWER_ROMAN: u16 = 0x0005;
pub static LIST_STYLE_TYPE_UPPER_ROMAN: u16 = 0x0006;
pub static LIST_STYLE_TYPE_LOWER_GREEK: u16 = 0x0007;
pub static LIST_STYLE_TYPE_LOWER_LATIN: u16 = 0x0008;
pub static LIST_STYLE_TYPE_UPPER_LATIN: u16 = 0x0009;
pub static LIST_STYLE_TYPE_ARMENIAN: u16    = 0x000a;
pub static LIST_STYLE_TYPE_GEORGIAN: u16    = 0x000b;
pub static LIST_STYLE_TYPE_LOWER_ALPHA: u16 = 0x000c;
pub static LIST_STYLE_TYPE_UPPER_ALPHA: u16 = 0x000d;
pub static LIST_STYLE_TYPE_NONE: u16    = 0x000e;


pub static MARGIN_SET: u16 = 0x0080;
pub static MARGIN_AUTO: u16 = 0x0000;

pub static    MAX_HEIGHT_SET    : u16      = 0x0080;
pub static    MAX_HEIGHT_NONE   : u16      = 0x0000;

pub static    MAX_WIDTH_SET     : u16      = 0x0080;
pub static    MAX_WIDTH_NONE    : u16      = 0x0000;

pub static    MIN_HEIGHT_SET    : u16      = 0x0080;

pub static    MIN_WIDTH_SET     : u16      = 0x0080;

pub static OPACITY_SET: u16 = 0x0080;

pub static    ORPHANS_SET  : u16       = 0x0080;


pub static OUTLINE_COLOR_TRANSPARENT: u16 = 0x0000;
pub static OUTLINE_COLOR_CURRENT_COLOR: u16 = 0x0001;
pub static OUTLINE_COLOR_INVERT: u16 = 0x0002;
pub static OUTLINE_COLOR_SET: u16 = 0x0080;

pub static OUTLINE_STYLE_NONE  : u16 = 0x0000;
pub static OUTLINE_STYLE_HIDDEN    : u16 = 0x0001;
pub static OUTLINE_STYLE_DOTTED    : u16 = 0x0002;
pub static OUTLINE_STYLE_DASHED    : u16 = 0x0003;
pub static OUTLINE_STYLE_SOLID : u16 = 0x0004;
pub static OUTLINE_STYLE_DOUBLE    : u16 = 0x0005;
pub static OUTLINE_STYLE_GROOVE    : u16 = 0x0006;
pub static OUTLINE_STYLE_RIDGE : u16 = 0x0007;
pub static OUTLINE_STYLE_INSET : u16 = 0x0008;
pub static OUTLINE_STYLE_OUTSET    : u16 = 0x0009;


pub static OUTLINE_WIDTH_SET    : u16 = 0x0080;
pub static OUTLINE_WIDTH_THIN   : u16 = 0x0000;
pub static OUTLINE_WIDTH_MEDIUM     : u16 = 0x0001;
pub static OUTLINE_WIDTH_THICK : u16  = 0x0002;


pub static    OVERFLOW_VISIBLE   : u16     = 0x0000;
pub static    OVERFLOW_HIDDEN    : u16     = 0x0001;
pub static    OVERFLOW_SCROLL    : u16     = 0x0002;
pub static    OVERFLOW_AUTO      : u16     = 0x0003;

pub static    PADDING_SET    : u16     = 0x0080;

pub static    PAGE_BREAK_AFTER_AUTO       : u16 = 0x0000;
pub static    PAGE_BREAK_AFTER_ALWAYS     : u16 = 0x0001;
pub static    PAGE_BREAK_AFTER_AVOID      : u16 = 0x0002;
pub static    PAGE_BREAK_AFTER_LEFT       : u16 = 0x0003;
pub static    PAGE_BREAK_AFTER_RIGHT      : u16 = 0x0004;

pub static    PAGE_BREAK_BEFORE_AUTO      : u16 = 0x0000;
pub static    PAGE_BREAK_BEFORE_ALWAYS    : u16 = 0x0001;
pub static    PAGE_BREAK_BEFORE_AVOID     : u16 = 0x0002;
pub static    PAGE_BREAK_BEFORE_LEFT      : u16 = 0x0003;
pub static    PAGE_BREAK_BEFORE_RIGHT     : u16 = 0x0004;

pub static    PAGE_BREAK_INSIDE_AUTO      : u16 = 0x0000;
pub static    PAGE_BREAK_INSIDE_AVOID     : u16 = 0x0001;

pub static    PAUSE_AFTER_SET         : u16 = 0x0080;

pub static    PAUSE_BEFORE_SET        : u16 = 0x0080;

pub static    PITCH_RANGE_SET         : u16 = 0x0080;

pub static    PITCH_FREQUENCY         : u16 = 0x0080;

pub static    PITCH_X_LOW         : u16 = 0x0000;
pub static    PITCH_LOW           : u16 = 0x0001;
pub static    PITCH_MEDIUM            : u16 = 0x0002;
pub static    PITCH_HIGH          : u16 = 0x0003;
pub static    PITCH_X_HIGH            : u16 = 0x0004;

pub static    PLAY_DURING_TYPE_MASK       : u16 = 0x009f;
pub static    PLAY_DURING_URI         : u16 = 0x0080;
pub static    PLAY_DURING_MIX         : u16 = (1<<6);
pub static    PLAY_DURING_REPEAT      : u16 = (1<<5);

pub static    PLAY_DURING_AUTO        : u16 = 0x0000;
pub static    PLAY_DURING_NONE        : u16 = 0x0001;

pub static    POSITION_STATIC         : u16 = 0x0000;
pub static    POSITION_RELATIVE       : u16 = 0x0001;
pub static    POSITION_ABSOLUTE       : u16 = 0x0002;
pub static    POSITION_FIXED          : u16 = 0x0003;

pub static    QUOTES_STRING           : u16 = 0x0080;
pub static    QUOTES_NONE         : u16 = 0x0000;

pub static    RICHNESS_SET            : u16 = 0x0080;

pub static    RIGHT_SET           : u16 = 0x0080;
pub static    RIGHT_AUTO          : u16 = 0x0000;

pub static    SPEAK_HEADER_ONCE       : u16 = 0x0000;
pub static    SPEAK_HEADER_ALWAYS     : u16 = 0x0001;

pub static    SPEAK_NUMERAL_DIGITS        : u16 = 0x0000;
pub static    SPEAK_NUMERAL_CONTINUOUS    : u16 = 0x0001;

pub static    SPEAK_PUNCTUATION_CODE      : u16 = 0x0000;
pub static    SPEAK_PUNCTUATION_NONE      : u16 = 0x0001;

pub static    SPEAK_NORMAL            : u16 = 0x0000;
pub static    SPEAK_NONE          : u16 = 0x0001;
pub static    SPEAK_SPELL_OUT         : u16 = 0x0002;

pub static    SPEECH_RATE_SET         : u16 = 0x0080;

pub static    SPEECH_RATE_X_SLOW      : u16 = 0x0000;
pub static    SPEECH_RATE_SLOW        : u16 = 0x0001;
pub static    SPEECH_RATE_MEDIUM      : u16 = 0x0002;
pub static    SPEECH_RATE_FAST        : u16 = 0x0003;
pub static    SPEECH_RATE_X_FAST      : u16 = 0x0004;
pub static    SPEECH_RATE_FASTER      : u16 = 0x0005;
pub static    SPEECH_RATE_SLOWER      : u16 = 0x0006;

pub static    STRESS_SET          : u16 = 0x0080;

pub static    TABLE_LAYOUT_AUTO       : u16 = 0x0000;
pub static    TABLE_LAYOUT_FIXED      : u16 = 0x0001;

pub static    TEXT_ALIGN_LEFT         : u16 = 0x0000;
pub static    TEXT_ALIGN_RIGHT        : u16 = 0x0001;
pub static    TEXT_ALIGN_CENTER       : u16 = 0x0002;
pub static    TEXT_ALIGN_JUSTIFY      : u16 = 0x0003;
pub static    TEXT_ALIGN_LIBCSS_LEFT      : u16 = 0x0004;
pub static    TEXT_ALIGN_LIBCSS_CENTER    : u16 = 0x0005;
pub static    TEXT_ALIGN_LIBCSS_RIGHT     : u16 = 0x0006;

pub static    TEXT_DECORATION_NONE        : u16 = 0x0000;

pub static    TEXT_DECORATION_BLINK       : u16 = (1<<3);
pub static    TEXT_DECORATION_LINE_THROUGH    : u16 = (1<<2);
pub static    TEXT_DECORATION_OVERLINE    : u16 = (1<<1);
pub static    TEXT_DECORATION_UNDERLINE   : u16 = (1<<0);

pub static    TEXT_INDENT_SET         : u16 = 0x0080;

pub static    TEXT_TRANSFORM_CAPITALIZE   : u16 = 0x0000;
pub static    TEXT_TRANSFORM_UPPERCASE    : u16 = 0x0001;
pub static    TEXT_TRANSFORM_LOWERCASE    : u16 = 0x0002;
pub static    TEXT_TRANSFORM_NONE     : u16 = 0x0003;

pub static    TOP_SET             : u16 = 0x0080;
pub static    TOP_AUTO            : u16 = 0x0000;

pub static    UNICODE_BIDI_NORMAL     : u16 = 0x0000;
pub static    UNICODE_BIDI_EMBED      : u16 = 0x0001;
pub static    UNICODE_BIDI_BIDI_OVERRIDE  : u16 = 0x0002;

pub static    VERTICAL_ALIGN_SET      : u16 = 0x0080;

pub static    VERTICAL_ALIGN_BASELINE     : u16 = 0x0000;
pub static    VERTICAL_ALIGN_SUB      : u16 = 0x0001;
pub static    VERTICAL_ALIGN_SUPER        : u16 = 0x0002;
pub static    VERTICAL_ALIGN_TOP      : u16 = 0x0003;
pub static    VERTICAL_ALIGN_TEXT_TOP     : u16 = 0x0004;
pub static    VERTICAL_ALIGN_MIDDLE       : u16 = 0x0005;
pub static    VERTICAL_ALIGN_BOTTOM       : u16 = 0x0006;
pub static    VERTICAL_ALIGN_TEXT_BOTTOM  : u16 = 0x0007;

pub static    VISIBILITY_VISIBLE      : u16 = 0x0000;
pub static    VISIBILITY_HIDDEN       : u16 = 0x0001;
pub static    VISIBILITY_COLLAPSE     : u16 = 0x0002;

pub static    VOICE_FAMILY_STRING : u16    = 0x0080;
pub static    VOICE_FAMILY_IDENT_LIST : u16    = 0x0081;

pub static    VOICE_FAMILY_END    : u16    = 0x0000;

pub static    VOICE_FAMILY_MALE   : u16    = 0x0001;
pub static    VOICE_FAMILY_FEMALE : u16    = 0x0002;
pub static    VOICE_FAMILY_CHILD  : u16    = 0x0003;

pub static    VOLUME_NUMBER       : u16    = 0x0080;
pub static    VOLUME_DIMENSION    : u16    = 0x0081;

pub static    VOLUME_SILENT       : u16    = 0x0000;
pub static    VOLUME_X_SOFT       : u16    = 0x0001;
pub static    VOLUME_SOFT     : u16    = 0x0002;
pub static    VOLUME_MEDIUM       : u16    = 0x0003;
pub static    VOLUME_LOUD     : u16    = 0x0004;
pub static    VOLUME_X_LOUD       : u16    = 0x0005;

pub static    WHITE_SPACE_NORMAL  : u16    = 0x0000;
pub static    WHITE_SPACE_PRE     : u16    = 0x0001;
pub static    WHITE_SPACE_NOWRAP  : u16    = 0x0002;
pub static    WHITE_SPACE_PRE_WRAP    : u16    = 0x0003;
pub static    WHITE_SPACE_PRE_LINE    : u16    = 0x0004;

pub static    WIDOWS_SET      : u16    = 0x0080;

pub static    WIDTH_SET       : u16    = 0x0080;

pub static    WIDTH_AUTO      : u16    = 0x0000;

pub static    WORD_SPACING_SET    : u16    = 0x0080;

pub static    WORD_SPACING_NORMAL : u16    = 0x0000;

pub static    Z_INDEX_SET : u16        = 0x0080;

pub static    Z_INDEX_AUTO    : u16        = 0x0000;
