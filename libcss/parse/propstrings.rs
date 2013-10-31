
use libwapcaplet::wapcaplet::*;
use stylesheet::*;

pub enum index_property {
     // Universal selector 
    UNIVERSAL ,
     
     // At-rules 
    CHARSET,
    LIBCSS_IMPORT, 
    MEDIA, 
    NAMESPACE, 
    FONT_FACE, 
    PAGE,

     // Media types 
    AURAL, 
    BRAILLE, 
    EMBOSSED, 
    HANDHELD, 
    PRINT, 
    PROJECTION, 
    SCREEN, 
    SPEECH, 
    TTY, 
    TV, 
    ALL,

     // Pseudo classes 
    FIRST_CHILD, 
    LINK, 
    VISITED, 
    HOVER, 
    ACTIVE, 
    FOCUS, 
    LANG, 

     // LEFT, RIGHT, -- already in properties  
    FIRST,
    ROOT, 
    NTH_CHILD, 
    NTH_LAST_CHILD, 
    NTH_OF_TYPE, 
    NTH_LAST_OF_TYPE,
    LAST_CHILD, 
    FIRST_OF_TYPE, 
    LAST_OF_TYPE, 
    ONLY_CHILD,
    ONLY_OF_TYPE, 
    EMPTY, 
    TARGET, 
    ENABLED, 
    DISABLED, 
    CHECKED, 
    NOT, 

     // Pseudo elements 
    FIRST_LINE, 
    FIRST_LETTER, 
    BEFORE, 
    AFTER,

     // Properties 
    // FIRST_PROP = AZIMUTH,

    AZIMUTH, 
    BACKGROUND, 
    BACKGROUND_ATTACHMENT, 
    BACKGROUND_COLOR, 
    BACKGROUND_IMAGE, 
    BACKGROUND_POSITION, 
    BACKGROUND_REPEAT, 
    BORDER, 
    BORDER_BOTTOM, 
    BORDER_BOTTOM_COLOR, 
    BORDER_BOTTOM_STYLE, 
    BORDER_BOTTOM_WIDTH, 
    BORDER_COLLAPSE, 
    BORDER_COLOR, 
    BORDER_LEFT, 
    BORDER_LEFT_COLOR, 
    BORDER_LEFT_STYLE, 
    BORDER_LEFT_WIDTH, 
    BORDER_RIGHT, 
    BORDER_RIGHT_COLOR, 
    BORDER_RIGHT_STYLE, 
    BORDER_RIGHT_WIDTH, 
    BORDER_SPACING, 
    BORDER_STYLE, 
    BORDER_TOP, 
    BORDER_TOP_COLOR, 
    BORDER_TOP_STYLE, 
    BORDER_TOP_WIDTH, 
    BORDER_WIDTH, 
    BOTTOM, 
    BREAK_AFTER,
    BREAK_BEFORE,
    BREAK_INSIDE, 
    CAPTION_SIDE, 
    CLEAR, 
    CLIP, 
    COLOR, 
    COLUMNS, 
    COLUMN_COUNT,
    COLUMN_FILL, 
    COLUMN_GAP, 
    COLUMN_RULE, 
    COLUMN_RULE_COLOR,
    COLUMN_RULE_STYLE, 
    COLUMN_RULE_WIDTH, 
    COLUMN_SPAN, 
    COLUMN_WIDTH,
    CONTENT, 
    COUNTER_INCREMENT, 
    COUNTER_RESET, 
    CUE, 
    CUE_AFTER, 
    CUE_BEFORE,
    CURSOR, 
    DIRECTION, 
    DISPLAY, 
    ELEVATION, 
    EMPTY_CELLS, 
    LIBCSS_FLOAT,
    FONT,
    FONT_FAMILY, 
    FONT_SIZE, 
    FONT_STYLE, 
    FONT_VARIANT, 
    FONT_WEIGHT, 
    HEIGHT,
    LEFT, 
    LETTER_SPACING, 
    LINE_HEIGHT, 
    LIST_STYLE, 
    LIST_STYLE_IMAGE,
    LIST_STYLE_POSITION, 
    LIST_STYLE_TYPE, 
    MARGIN, 
    MARGIN_BOTTOM,
    MARGIN_LEFT, 
    MARGIN_RIGHT, 
    MARGIN_TOP, 
    MAX_HEIGHT, 
    MAX_WIDTH,
    MIN_HEIGHT, 
    MIN_WIDTH, 
    OPACITY, 
    ORPHANS, 
    OUTLINE, 
    OUTLINE_COLOR,
    OUTLINE_STYLE, 
    OUTLINE_WIDTH, 
    OVERFLOW, 
    PADDING, 
    PADDING_BOTTOM,
    PADDING_LEFT, 
    PADDING_RIGHT, 
    PADDING_TOP, 
    PAGE_BREAK_AFTER,
    PAGE_BREAK_BEFORE, 
    PAGE_BREAK_INSIDE, 
    PAUSE, 
    PAUSE_AFTER, 
    PAUSE_BEFORE,
    PITCH_RANGE, 
    PITCH, 
    PLAY_DURING, 
    POSITION, 
    QUOTES, 
    RICHNESS, 
    RIGHT,
    SPEAK_HEADER, 
    SPEAK_NUMERAL, 
    SPEAK_PUNCTUATION, 
    SPEAK, 
    SPEECH_RATE,
    STRESS, 
    TABLE_LAYOUT, 
    TEXT_ALIGN, 
    TEXT_DECORATION, 
    TEXT_INDENT,
    TEXT_TRANSFORM, 
    TOP, 
    UNICODE_BIDI, 
    VERTICAL_ALIGN, 
    VISIBILITY,
    VOICE_FAMILY, 
    VOLUME, 
    WHITE_SPACE, 
    WIDOWS, 
    WIDTH, 
    WORD_SPACING, 
    Z_INDEX,

    // LAST_PROP = Z_INDEX as int,

     // Other keywords 
    INHERIT, 
    IMPORTANT, 
    NONE, 
    BOTH, 
    FIXED, 
    SCROLL, 
    TRANSPARENT,
    NO_REPEAT, 
    REPEAT_X, 
    REPEAT_Y, 
    REPEAT, 
    HIDDEN,
    DOTTED, 
    DASHED,
    SOLID, 
    LIBCSS_DOUBLE, 
    GROOVE, 
    RIDGE, 
    INSET, 
    OUTSET, 
    THIN, 
    MEDIUM, 
    THICK,
    COLLAPSE, 
    SEPARATE, 
    AUTO, 
    LTR, 
    RTL, 
    INLINE, 
    BLOCK, 
    LIST_ITEM, 
    RUN_IN,
    INLINE_BLOCK, 
    TABLE, 
    INLINE_TABLE, 
    TABLE_ROW_GROUP, 
    TABLE_HEADER_GROUP,
    TABLE_FOOTER_GROUP, 
    TABLE_ROW, 
    TABLE_COLUMN_GROUP, 
    TABLE_COLUMN,
    TABLE_CELL, 
    TABLE_CAPTION,
    BELOW, 
    LEVEL, 
    ABOVE, 
    HIGHER, 
    LOWER,
    SHOW, 
    HIDE, 
    XX_SMALL, 
    X_SMALL, 
    SMALL, 
    LARGE, 
    X_LARGE,
    XX_LARGE,
    LARGER, 
    SMALLER, 
    NORMAL, 
    ITALIC, 
    OBLIQUE, 
    SMALL_CAPS, 
    BOLD, 
    BOLDER,
    LIGHTER, 
    INSIDE,
    OUTSIDE, 
    DISC, 
    CIRCLE, 
    SQUARE, 
    DECIMAL, 
    DECIMAL_LEADING_ZERO, 
    LOWER_ROMAN, 
    UPPER_ROMAN, 
    LOWER_GREEK,
    LOWER_LATIN,
    UPPER_LATIN, 
    ARMENIAN, 
    GEORGIAN, 
    LOWER_ALPHA, 
    UPPER_ALPHA,
    INVERT, 
    VISIBLE, 
    ALWAYS, 
    AVOID, 
    X_LOW, 
    LOW, 
    HIGH, 
    X_HIGH, 
    LIBCSS_STATIC,
    RELATIVE, 
    ABSOLUTE, 
    ONCE, 
    DIGITS, 
    CONTINUOUS, 
    CODE, 
    SPELL_OUT, 
    X_SLOW,
    SLOW, 
    FAST, 
    X_FAST, 
    FASTER, 
    SLOWER, 
    CENTER, 
    JUSTIFY, 
    CAPITALIZE,
    UPPERCASE, 
    LOWERCASE, 
    EMBED, 
    BIDI_OVERRIDE, 
    BASELINE, 
    SUB, 
    SUPER,
    TEXT_TOP, 
    MIDDLE, 
    TEXT_BOTTOM, 
    SILENT, 
    X_SOFT, 
    SOFT, 
    LOUD, 
    X_LOUD,
    PRE, 
    NOWRAP, 
    PRE_WRAP, 
    PRE_LINE, 
    LEFTWARDS, 
    RIGHTWARDS, 
    LEFT_SIDE,
    FAR_LEFT, 
    CENTER_LEFT, 
    CENTER_RIGHT, 
    FAR_RIGHT, 
    RIGHT_SIDE, 
    BEHIND, 
    RECT, 
    OPEN_QUOTE, 
    CLOSE_QUOTE, 
    NO_OPEN_QUOTE, 
    NO_CLOSE_QUOTE, 
    ATTR, 
    COUNTER, 
    COUNTERS, 
    CROSSHAIR, 
    DEFAULT, 
    POINTER, 
    MOVE, 
    E_RESIZE, 
    NE_RESIZE, 
    NW_RESIZE, 
    N_RESIZE, 
    SE_RESIZE, 
    SW_RESIZE, 
    S_RESIZE, 
    W_RESIZE, 
    LIBCSS_TEXT, 
    WAIT, 
    HELP, 
    PROGRESS, 
    SERIF, 
    SANS_SERIF, 
    CURSIVE,
    FANTASY, 
    MONOSPACE, 
    MALE, 
    FEMALE, 
    CHILD, 
    MIX, 
    UNDERLINE, 
    OVERLINE, 
    LINE_THROUGH, 
    BLINK, 
    RGB, 
    RGBA,
    HSL, 
    HSLA, 
    LIBCSS_LEFT, 
    LIBCSS_CENTER,
    LIBCSS_RIGHT, 
    CURRENTCOLOR, 
    ODD, 
    EVEN, 
    SRC, 
    LOCAL, 
    INITIAL,
    FORMAT, 
    WOFF, 
    TRUETYPE, 
    OPENTYPE, 
    EMBEDDED_OPENTYPE, 
    SVG, 
    COLUMN,
    AVOID_PAGE, 
    AVOID_COLUMN,
    BALANCE,

     // Named colours 
    //FIRST_COLOUR = ALICEBLUE,

    ALICEBLUE, 
    ANTIQUEWHITE, 
    AQUA, 
    AQUAMARINE, 
    AZURE,
    BEIGE, 
    BISQUE, 
    BLACK, 
    BLANCHEDALMOND, 
    BLUE, 
    BLUEVIOLET, 
    BROWN,
    BURLYWOOD, 
    CADETBLUE, 
    CHARTREUSE, 
    CHOCOLATE, 
    CORAL, 
    CORNFLOWERBLUE,
    CORNSILK, 
    CRIMSON, 
    CYAN, 
    DARKBLUE, 
    DARKCYAN, 
    DARKGOLDENROD, 
    DARKGRAY,
    DARKGREEN, 
    DARKGREY, 
    DARKKHAKI, 
    DARKMAGENTA, 
    DARKOLIVEGREEN, 
    DARKORANGE,
    DARKORCHID, 
    DARKRED, 
    DARKSALMON, 
    DARKSEAGREEN, 
    DARKSLATEBLUE,
    DARKSLATEGRAY, 
    DARKSLATEGREY, 
    DARKTURQUOISE, 
    DARKVIOLET, 
    DEEPPINK,
    DEEPSKYBLUE, 
    DIMGRAY, 
    DIMGREY, 
    DODGERBLUE, 
    FELDSPAR, 
    FIREBRICK,
    FLORALWHITE, 
    FORESTGREEN, 
    FUCHSIA, 
    GAINSBORO, 
    GHOSTWHITE, 
    GOLD, 
    GOLDENROD, 
    GRAY, 
    GREEN, 
    GREENYELLOW, 
    GREY, 
    HONEYDEW, 
    HOTPINK,
    INDIANRED, 
    INDIGO, 
    IVORY, 
    KHAKI, 
    LAVENDER, 
    LAVENDERBLUSH, 
    LAWNGREEN,
    LEMONCHIFFON, 
    LIGHTBLUE, 
    LIGHTCORAL, 
    LIGHTCYAN, 
    LIGHTGOLDENRODYELLOW,
    LIGHTGRAY, 
    LIGHTGREEN, 
    LIGHTGREY, 
    LIGHTPINK, 
    LIGHTSALMON, 
    LIGHTSEAGREEN,
    LIGHTSKYBLUE, 
    LIGHTSLATEBLUE, 
    LIGHTSLATEGRAY, 
    LIGHTSLATEGREY, 
    LIGHTSTEELBLUE, 
    LIGHTYELLOW, 
    LIME, 
    LIMEGREEN, 
    LINEN, 
    MAGENTA, 
    MAROON,
    MEDIUMAQUAMARINE, 
    MEDIUMBLUE, 
    MEDIUMORCHID, 
    MEDIUMPURPLE, 
    MEDIUMSEAGREEN, 
    MEDIUMSLATEBLUE, 
    MEDIUMSPRINGGREEN, 
    MEDIUMTURQUOISE,
    MEDIUMVIOLETRED, 
    MIDNIGHTBLUE, 
    MINTCREAM, 
    MISTYROSE, 
    MOCCASIN,
    NAVAJOWHITE, 
    NAVY, 
    OLDLACE, 
    OLIVE, 
    OLIVEDRAB, 
    ORANGE, 
    ORANGERED,
    ORCHID, 
    PALEGOLDENROD, 
    PALEGREEN, 
    PALETURQUOISE, 
    PALEVIOLETRED,
    PAPAYAWHIP, 
    PEACHPUFF, 
    PERU, 
    PINK, 
    PLUM, 
    POWDERBLUE, 
    PURPLE, 
    RED,
    ROSYBROWN, 
    ROYALBLUE, 
    SADDLEBROWN, 
    SALMON, 
    SANDYBROWN, 
    SEAGREEN,
    SEASHELL, 
    SIENNA, 
    SILVER, 
    SKYBLUE, 
    SLATEBLUE, 
    SLATEGRAY, 
    SLATEGREY,
    SNOW, 
    SPRINGGREEN, 
    STEELBLUE, 
    TAN, 
    TEAL,
    THISTLE, 
    TOMATO, 
    TURQUOISE, 
    VIOLET, 
    VIOLETRED, 
    WHEAT, 
    WHITE, 
    WHITESMOKE, 
    YELLOW, 
    YELLOWGREEN,

    // LAST_COLOUR = YELLOWGREEN as int,

    LAST_KNOWN
}


pub struct css_propstrings {
    propstrings: ~[uint],
    pseudo_class_list:~[index_property],
    pseudo_element_list:~[index_property]
}


// impl Clone for css_propstrings {  
//     fn clone(&self) -> css_propstrings {     
//         css_propstrings{  
//             propstrings:self.propstrings.clone(),  
//             pseudo_class_list:self.pseudo_class_list,
//             pseudo_element_list: self.pseudo_element_list  
//         }  
//     }  
// }  

impl css_propstrings {

    pub fn css_propstrings(lwc_ref:&mut ~lwc) -> css_propstrings {

        let propstrings_list = &[&"*", &"charset",&"import",&"media", &"namespace", &"font-face", &"page", &"aural",&"braille", &"embossed",&"handheld", &"print",
            &"projection", &"screen", &"speech", &"tty", &"tv", &"all",&"first-child", &"link", &"visited", &"hover", &"active", &"focus",
            &"lang",&"first",&"root", &"nth-child", &"nth-last-child", &"nth-of-type",&"nth-last-of-type", &"last-child",&"first-of-type",
            &"last-of-type", &"only-child", &"only-of-type",&"empty", &"target",&"enabled", &"disabled", &"checked", &"not", &"first-line", 
            &"first-letter", &"before",&"after",&"azimuth",&"background", &"background-attachment", &"background-color", &"background-image", 
            &"background-position",&"background-repeat", &"border",&"border-bottom", &"border-bottom-color", &"border-bottom-style", &"border-bottom-width",
            &"border-collapse",&"border-color",&"border-left", &"border-left-color", &"border-left-style", &"border-left-width",&"border-right",
            &"border-right-color", &"border-right-style",&"border-right-width",&"border-spacing",&"border-style", &"border-top",&"border-top-color",
            &"border-top-style",&"border-top-width",&"border-width", &"bottom", &"break-after", &"break-before", &"break-inside",&"caption-side",
            &"clear",&"clip",&"color",&"columns",&"column-count",&"column-fill",&"column-gap",&"column-rule", &"column-rule-color",
            &"column-rule-style",&"column-rule-width",&"column-span",&"column-width",&"content", &"counter-increment", &"counter-reset",&"cue",
            &"cue-after", &"cue-before",&"cursor", &"direction",&"display",&"elevation", &"empty-cells",&"float",&"font", &"font-family",&"font-size",
            &"font-style", &"font-variant",&"font-weight", &"height", &"left", &"letter-spacing", &"line-height",&"list-style", &"list-style-image",
            &"list-style-position",&"list-style-type",&"margin", &"margin-bottom",&"margin-left",&"margin-right", &"margin-top", &"max-height",
            &"max-width", &"min-height", &"min-width",&"opacity", &"orphans",&"outline", &"outline-color", &"outline-style",&"outline-width", &"overflow",
            &"padding", &"padding-bottom",&"padding-left",&"padding-right",&"padding-top",&"page-break-after", &"page-break-before",&"page-break-inside",
            &"pause",&"pause-after",&"pause-before", &"pitch-range",&"pitch",&"play-during",&"position",&"quotes", &"richness",&"right", &"speak-header",
            &"speak-numeral",&"speak-punctuation",&"speak", &"speech-rate",&"stress",&"table-layout", &"text-align",&"text-decoration",&"text-indent",
            &"text-transform",&"top", &"unicode-bidi", &"vertical-align",&"visibility",&"voice-family", &"volume",&"white-space",&"widows", &"width",
            &"word-spacing",&"z-index",&"inherit",&"important",&"none",&"both", &"fixed",&"scroll",&"transparent", &"no-repeat",&"repeat-x",&"repeat-y",
            &"repeat",&"hidden",&"dotted", &"dashed", &"solid",&"double",&"groove", &"ridge",&"inset",&"outset",&"thin", &"medium",&"thick", &"collapse",
            &"separate",&"auto",&"ltr",&"rtl", &"inline", &"block",&"list-item",&"run-in",&"inline-block", &"table",&"inline-table",&"table-row-group",
            &"table-header-group",&"table-footer-group",&"table-row", &"table-column-group", &"table-column",&"table-cell",&"table-caption",&"below",
            &"level",&"above",&"higher",&"lower",&"show",&"hide",&"xx-small",&"x-small",&"small",&"large",&"x-large",&"xx-large",&"larger",
            &"smaller", &"normal",&"italic",&"oblique",&"small-caps",&"bold",&"bolder", &"lighter",&"inside",&"outside", &"disc",
            &"circle",&"square",&"decimal",&"decimal-leading-zero", &"lower-roman", &"upper-roman", &"lower-greek",&"lower-latin",&"upper-latin",
            &"armenian",&"georgian", &"lower-alpha",&"upper-alpha",&"invert",&"visible",&"always",&"avoid",&"x-low",&"low", &"high", &"x-high",
            &"static",&"relative", &"absolute",&"once",&"digits",&"continuous", &"code", &"spell-out",&"x-slow",&"slow",&"fast",&"x-fast",&"faster",
            &"slower",&"center",&"justify",&"capitalize",&"uppercase",&"lowercase",&"embed",&"bidi-override",&"baseline",&"sub",&"super", 
            &"text-top",&"middle",&"text-bottom",&"silent",&"x-soft",&"soft",&"loud", &"x-loud", &"pre",&"nowrap",&"pre-wrap",&"pre-line",
            &"leftwards",&"rightwards",&"left-side", &"far-left", &"center-left",&"center-right",&"far-right",&"right-side",&"behind",&"rect",&"open-quote",
            &"close-quote",&"no-open-quote",&"no-close-quote",&"attr",&"counter",&"counters",&"crosshair",&"default",&"pointer",&"move",&"e-resize",
            &"ne-resize",&"nw-resize",&"n-resize", &"se-resize",&"sw-resize",&"s-resize",&"w-resize",&"text",&"wait",&"help",&"progress",&"serif",
            &"sans-serif",&"cursive",&"fantasy",&"monospace",&"male",&"female",&"child",&"mix",&"underline",&"overline",&"line-through",&"blink",
            &"rgb", &"rgba",&"hsl",&"hsla",&"-libcss-left",&"-libcss-center",&"-libcss-right",&"currentColor", &"odd", &"even",&"src",&"local",
            &"initial",&"format",&"woff",&"truetype",&"opentype", &"embedded-opentype", &"svg",&"column",&"avoid-page", &"avoid-column",&"balance",
            &"aliceblue",&"antiquewhite",&"aqua",&"aquamarine",&"azure",&"beige",&"bisque",&"black",&"blanchedalmond",&"blue",&"blueviolet",&"brown",
            &"burlywood",&"cadetblue",&"chartreuse",&"chocolate", &"coral",&"cornflowerblue", &"cornsilk", &"crimson", &"cyan",&"darkblue",&"darkcyan",
            &"darkgoldenrod",&"darkgray",&"darkgreen",&"darkgrey",&"darkkhaki", &"darkmagenta",&"darkolivegreen",&"darkorange",&"darkorchid",&"darkred",
            &"darksalmon",&"darkseagreen",&"darkslateblue",&"darkslategray",&"darkslategrey",&"darkturquoise",&"darkviolet",&"deeppink", &"deepskyblue",
            &"dimgray",&"dimgrey",&"dodgerblue",&"feldspar",&"firebrick",&"floralwhite", &"forestgreen",&"fuchsia", &"gainsboro",&"ghostwhite",
            &"gold",&"goldenrod",&"gray",&"green",&"greenyellow",&"grey",&"honeydew",&"hotpink",&"indianred",&"indigo",&"ivory",&"khaki",&"lavender",
            &"lavenderblush",&"lawngreen",&"lemonchiffon",&"lightblue",&"lightcoral",&"lightcyan",&"lightgoldenrodyellow",&"lightgray",&"lightgreen",
            &"lightgrey",&"lightpink",&"lightsalmon",&"lightseagreen",&"lightskyblue", &"lightslateblue", &"lightslategray",&"lightslategrey",
            &"lightsteelblue", &"lightyellow",&"lime",&"limegreen",&"linen", &"magenta",&"maroon",&"mediumaquamarine",&"mediumblue", &"mediumorchid",
            &"mediumpurple", &"mediumseagreen",&"mediumslateblue",&"mediumspringgreen",&"mediumturquoise",&"mediumvioletred", &"midnightblue",&"mintcream", 
            &"mistyrose",&"moccasin",&"navajowhite",&"navy", &"oldlace", &"olive",&"olivedrab",&"orange",&"orangered",&"orchid",&"palegoldenrod",
            &"palegreen",&"paleturquoise",&"palevioletred", &"papayawhip",&"peachpuff",&"peru",&"pink",&"plum",&"powderblue", &"purple",&"red",
            &"rosybrown",&"royalblue", &"saddlebrown",&"salmon",&"sandybrown",&"seagreen",&"seashell",&"sienna", &"silver", &"skyblue",&"slateblue",
            &"slategray", &"slategrey",&"snow",&"springgreen",&"steelblue", &"tan", &"teal",&"thistle",&"tomato",&"turquoise",&"violet",&"violetred",
            &"wheat",&"white", &"whitesmoke",&"yellow",&"yellowgreen"];

        let length = propstrings_list.len();
        let mut ele = 0;
        let mut _propstrings = ~[];
        //_propstrings.reserve_at_least(length);
        
        while ele < length {
            _propstrings.push(lwc_ref.lwc_intern_string(propstrings_list[ele]));
            ele +=1;
        }

        let css_propstrings_instance = css_propstrings {
            propstrings: _propstrings,
            pseudo_class_list : ~[ 
                                    FIRST_CHILD,
                                    LINK,
                                    VISITED,
                                    HOVER,
                                    ACTIVE,
                                    FOCUS,
                                    LANG,
                                    LEFT,
                                    RIGHT,
                                    FIRST,
                                    ROOT,
                                    NTH_CHILD,
                                    NTH_LAST_CHILD,
                                    NTH_OF_TYPE,
                                    NTH_LAST_OF_TYPE,
                                    LAST_CHILD,
                                    FIRST_OF_TYPE,
                                    LAST_OF_TYPE,
                                    ONLY_CHILD,
                                    ONLY_OF_TYPE,
                                    EMPTY,
                                    TARGET,
                                    ENABLED,
                                    DISABLED,
                                    CHECKED,
                                    NOT,
                                ],

            pseudo_element_list : ~[
                                    FIRST_LINE, 
                                    FIRST_LETTER, 
                                    BEFORE, 
                                    AFTER,
                                ]
        };

        css_propstrings_instance
    }

    #[inline]

    pub fn lwc_string_caseless_isequal(&self, lwc_ref:&mut ~lwc, lwc_string_instance: uint , string_index: uint) -> bool {
       lwc_ref.lwc_string_caseless_isequal(lwc_string_instance, self.propstrings[string_index])
   
    }

    #[inline]
    pub fn lwc_string_isequal(&self, lwc_ref:&mut ~lwc, lwc_string_instance: uint , string_index: uint) -> bool {
        lwc_ref.lwc_string_isequal(lwc_string_instance , self.propstrings[string_index])    
    }

    #[inline]
    pub fn lwc_string_data(&self, lwc_ref:&mut ~lwc,string_index:uint) -> ~str {
        lwc_ref.lwc_string_data(self.propstrings[string_index])
    }

    #[inline]
    pub fn get_lwc_string(&self, string_index:uint) -> uint {
        self.propstrings[string_index]
    }

    

    pub fn is_selector_pseudo(&self, lwc_ref:&mut ~lwc, name: uint) -> Option<(css_selector_type, index_property)> {
        
        let mut return_value : Option<(css_selector_type, index_property)> = None;

		for &string_index in self.pseudo_class_list.iter() {
			if  (
					lwc_ref.lwc_string_caseless_isequal(
						name,
						self.propstrings[string_index as uint]
					) 
				) {
				return_value = Some((CSS_SELECTOR_PSEUDO_CLASS, string_index));
                break;
			}
		}

		for &string_index in self.pseudo_element_list.iter() {
			if (
				lwc_ref.lwc_string_caseless_isequal(
					name, 
					self.propstrings[string_index as uint]
				)
			) {
				return_value = Some((CSS_SELECTOR_PSEUDO_ELEMENT , string_index));
                break;
			}
		}
        

        return_value
    }
}
