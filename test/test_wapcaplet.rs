extern mod std;
extern mod wapcaplet;
extern mod test;


use std::arc;
use wapcaplet::*;
use test::*;


fn main() {
	let propstrings_list = ~[~"*", ~"charset",~"import",~"media", ~ "namespace", ~ "font-face", ~"page", ~"aural",~ "braille", ~ "embossed",~"handheld", ~"print",
				~"projection", ~ "screen", ~ "speech", ~ "tty", ~ "tv", ~ "all",~"first-child", ~ "link", ~ "visited", ~ "hover", ~ "active", ~ "focus",
				~ "lang",~ "first",~ "root", ~ "nth-child", ~ "nth-last-child", ~ "nth-of-type",~"nth-last-of-type", ~ "last-child",~ "first-of-type",
				~ "last-of-type", ~ "only-child", ~ "only-of-type",~ "empty", ~"target",~ "enabled", ~ "disabled", ~ "checked", ~"not", ~ "first-line", 
				~ "first-letter", ~ "before",~ "after",~ "azimuth",~ "background", ~ "background-attachment", ~ "background-color", ~ "background-image", 
				~"background-position",~"background-repeat", ~"border",~"border-bottom", ~ "border-bottom-color", ~ "border-bottom-style", ~ "border-bottom-width",
				~"border-collapse",~ "border-color",~ "border-left", ~ "border-left-color", ~ "border-left-style", ~ "border-left-width",~ "border-right",
				~ "border-right-color", ~ "border-right-style",~ "border-right-width",~ "border-spacing",~ "border-style", ~ "border-top",~"border-top-color",
				~ "border-top-style",~ "border-top-width",~ "border-width", ~ "bottom", ~ "break-after", ~ "break-before", ~ "break-inside",~ "caption-side",
				~ "clear",~ "clip",~ "color",~ "columns",~ "column-count",~ "column-fill",~ "column-gap",~ "column-rule", ~"column-rule-color",
				~ "column-rule-style",~ "column-rule-width",~ "column-span",~"column-width",~ "content", ~ "counter-increment", ~ "counter-reset",~"cue",
				~ "cue-after", ~ "cue-before",~"cursor", ~ "direction",~ "display",~"elevation", ~ "empty-cells",~ "float",~ "font", ~ "font-family",~"font-size",
				~"font-style", ~ "font-variant",~"font-weight", ~ "height", ~"left", ~"letter-spacing", ~ "line-height",~ "list-style", ~ "list-style-image",
				~"list-style-position",~"list-style-type",~ "margin", ~ "margin-bottom",~"margin-left",~ "margin-right", ~ "margin-top", ~ "max-height",
				~ "max-width", ~ "min-height", ~ "min-width",~"opacity", ~ "orphans",~ "outline", ~ "outline-color", ~"outline-style",~ "outline-width", ~ "overflow",
				~ "padding", ~ "padding-bottom",~ "padding-left",~ "padding-right",~ "padding-top",~ "page-break-after", ~ "page-break-before",~ "page-break-inside",
				~ "pause",~ "pause-after",~ "pause-before", ~ "pitch-range",~"pitch",~"play-during",~ "position",~ "quotes", ~ "richness",~"right", ~"speak-header",
				~ "speak-numeral",~ "speak-punctuation",~"speak", ~"speech-rate",~"stress",~ "table-layout", ~ "text-align",~ "text-decoration",~ "text-indent",
				~ "text-transform",~"top", ~ "unicode-bidi", ~ "vertical-align",~ "visibility",~"voice-family", ~"volume",~"white-space",~ "widows", ~ "width",
				~ "word-spacing",~ "z-index",~ "inherit",~ "important",~"none",~"both", ~ "fixed",~"scroll",~ "transparent", ~ "no-repeat",~ "repeat-x",~"repeat-y",
				~ "repeat",~ "hidden",~"dotted", ~ "dashed", ~ "solid",~ "double",~ "groove", ~ "ridge",~"inset",~"outset",~ "thin", ~ "medium",~"thick", ~"collapse",
				~ "separate",~ "auto",~ "ltr",~ "rtl", ~"inline", ~ "block",~ "list-item",~ "run-in",~ "inline-block", ~ "table",~ "inline-table",~ "table-row-group",
				~ "table-header-group",~ "table-footer-group",~ "table-row", ~ "table-column-group", ~ "table-column",~ "table-cell",~ "table-caption",~ "below",
				~ "level",~ "above",~ "higher",~ "lower",~ "show",~ "hide",~ "xx-small",~ "x-small",~ "small",~ "large",~ "x-large",~ "xx-large",~ "larger",
				~ "smaller", ~ "normal",~ "italic",~ "oblique",~ "small-caps",~ "bold",~"bolder", ~ "lighter",~ "inside",~ "outside", ~ "disc",
				~ "circle",~"square",~ "decimal",~"decimal-leading-zero", ~ "lower-roman", ~ "upper-roman", ~ "lower-greek",~ "lower-latin",~ "upper-latin",
				~ "armenian",~ "georgian", ~ "lower-alpha",~ "upper-alpha",~ "invert",~ "visible",~ "always",~ "avoid",~ "x-low",~"low", ~ "high", ~ "x-high",
				~"static",~"relative", ~ "absolute",~ "once",~ "digits",~ "continuous", ~ "code", ~ "spell-out",~ "x-slow",~ "slow",~ "fast",~ "x-fast",~ "faster",
				~ "slower",~ "center",~ "justify",~ "capitalize",~ "uppercase",~ "lowercase",~ "embed",~ "bidi-override",~ "baseline",~ "sub",~ "super", 
				~ "text-top",~ "middle",~ "text-bottom",~ "silent",~ "x-soft",~ "soft",~ "loud", ~ "x-loud", ~"pre",~ "nowrap",~"pre-wrap",~"pre-line",
				~ "leftwards",~ "rightwards",~ "left-side", ~ "far-left", ~ "center-left",~ "center-right",~ "far-right",~ "right-side",~ "behind",~ "rect",~"open-quote",
				~ "close-quote",~ "no-open-quote",~ "no-close-quote",~ "attr",~ "counter",~ "counters",~ "crosshair",~ "default",~ "pointer",~ "move",~ "e-resize",
				~ "ne-resize",~ "nw-resize",~ "n-resize", ~ "se-resize",~ "sw-resize",~ "s-resize",~ "w-resize",~ "text",~ "wait",~ "help",~ "progress",~ "serif",
				~ "sans-serif",~ "cursive",~ "fantasy",~ "monospace",~ "male",~ "female",~ "child",~ "mix",~ "underline",~ "overline",~ "line-through",~ "blink",
				~ "rgb", ~ "rgba",~ "hsl",~"hsla",~ "-libcss-left",~ "-libcss-center",~ "-libcss-right",~ "currentColor", ~"odd", ~ "even",~ "src",~ "local",
				~ "initial",~ "format",~ "woff",~ "truetype",~ "opentype", ~"embedded-opentype", ~"svg",~ "column",~ "avoid-page", ~ "avoid-column",~ "balance",
				~"aliceblue",~ "antiquewhite",~ "aqua",~"aquamarine",~ "azure",~ "beige",~ "bisque",~"black",~"blanchedalmond",~"blue",~ "blueviolet",~"brown",
				~ "burlywood",~ "cadetblue",~ "chartreuse",~ "chocolate", ~ "coral",~ "cornflowerblue", ~ "cornsilk", ~ "crimson", ~ "cyan",~ "darkblue",~ "darkcyan",
				~ "darkgoldenrod",~ "darkgray",~ "darkgreen",~ "darkgrey",~"darkkhaki", ~ "darkmagenta",~ "darkolivegreen",~ "darkorange",~ "darkorchid",~ "darkred",
				~ "darksalmon",~ "darkseagreen",~ "darkslateblue",~ "darkslategray",~ "darkslategrey",~ "darkturquoise",~ "darkviolet",~ "deeppink", ~ "deepskyblue",
				~ "dimgray",~ "dimgrey",~ "dodgerblue",~ "feldspar",~ "firebrick",~ "floralwhite", ~ "forestgreen",~ "fuchsia", ~ "gainsboro",~ "ghostwhite",
			    ~ "gold",~ "goldenrod",~ "gray",~ "green",~ "greenyellow",~ "grey",~ "honeydew",~ "hotpink",~ "indianred",~ "indigo",~ "ivory",~ "khaki",~ "lavender",
			    ~ "lavenderblush",~ "lawngreen",~ "lemonchiffon",~ "lightblue",~ "lightcoral",~ "lightcyan",~ "lightgoldenrodyellow",~ "lightgray",~ "lightgreen",
			    ~ "lightgrey",~ "lightpink",~ "lightsalmon",~ "lightseagreen",~ "lightskyblue", ~ "lightslateblue", ~ "lightslategray",~ "lightslategrey",
			    ~ "lightsteelblue", ~ "lightyellow",~ "lime",~ "limegreen",~ "linen", ~ "magenta",~"maroon",~ "mediumaquamarine",~ "mediumblue", ~ "mediumorchid",
			    ~ "mediumpurple", ~ "mediumseagreen",~ "mediumslateblue",~ "mediumspringgreen",~ "mediumturquoise",~"mediumvioletred", ~ "midnightblue",~ "mintcream", 
			    ~ "mistyrose",~ "moccasin",~ "navajowhite",~ "navy", ~ "oldlace", ~ "olive",~ "olivedrab",~ "orange",~ "orangered",~"orchid",~"palegoldenrod",
			    ~ "palegreen",~ "paleturquoise",~ "palevioletred", ~ "papayawhip",~ "peachpuff",~ "peru",~ "pink",~ "plum",~ "powderblue", ~ "purple",~ "red",
			    ~ "rosybrown",~ "royalblue", ~ "saddlebrown",~ "salmon",~ "sandybrown",~ "seagreen",~ "seashell",~ "sienna", ~ "silver", ~ "skyblue",~ "slateblue",
			    ~ "slategray", ~"slategrey",~ "snow",~ "springgreen",~ "steelblue", ~ "tan", ~ "teal",~ "thistle",~ "tomato",~ "turquoise",~"violet",~ "violetred",
			    ~ "wheat",~ "white", ~ "whitesmoke",~"yellow",~ "yellowgreen"];

	let mut test_logger = result::unwrap(test_report(&"temp_log.csv"));
	let module_name: ~str=~"wapcaplet";
	let  file_name : ~str=~"wapcaplet.rs";
	let mut function_name : ~str = ~"";
	let mut test_name : ~str=~"";
	let mut comment: ~str=~"";

	// test 1: Creating a lwc instance
	let mut lwc_instance = lwc();
	function_name = ~"lwc()";
	test_name = ~"Creating a lwc instance";
	comment = ~"lwc instance created";
	test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);



	do lwc_instance.write |l| {
		
		// test 1 intern a vector of string
		function_name = ~"lwc_intern_string_vector";
		test_name = ~"intern a vector of string";
		comment = ~"vector of  string interned";
		test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		// io::println(fmt!("length of vector: %?", propstrings_list.len()));
		//test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , ~"");
		let  p = l.lwc_intern_string_vector(copy propstrings_list);
		
		// test 2: interning a null string
		function_name = ~"lwc_intern_string";
		test_name = ~"interning a null string";
		comment = ~"null string interned";
		test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let  p = l.lwc_intern_string(~"");
		
		// test 3: interning a normal string
		function_name = ~"lwc_intern_string";
		test_name = ~"interning a normal string";
		comment = ~"string interned successfull";
		test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let q = l.lwc_intern_string(~"hellowapcaplet");

		// test 4: interning a sub string with correct offset and length
		function_name = ~"lwc_intern_substring";
		test_name = ~"interning a sub string of a lwc_string";
		comment = ~"internment of a sub string of lenght 5 from offset 2 in hellowapcaplet";
		test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let r = l.lwc_intern_substring(q ,2 , 5);
		
		// test 5: lwc_string_caseless_isequal of two same string in different case
		function_name = ~"lwc_string_caseless_isequal";
		test_name = ~"lwc_string_caseless_isequal of two same string in different case";
		comment = ~"returns true";
		test_logger.pass(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let s = l.lwc_intern_string(~"abc");
		let q = l.lwc_intern_string(~"aBc");
		let r = l.lwc_string_caseless_isequal(s , q);

		// test 6: lwc_string_caseless_isequal of two same string in same case
		function_name = ~"lwc_string_caseless_isequal";
		test_name = ~"lwc_string_caseless_isequal of two same string in same case";
		comment = ~"true";
		test_logger.pass(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let s = l.lwc_intern_string(~"abc");
		let q = l.lwc_intern_string(~"abc");
		let r = l.lwc_string_caseless_isequal(s , q);

		// test 7: ref count increase of a interned string
		function_name = ~"lwc_string_ref";
		test_name = ~"ref count increase of a already interned string";
		comment = ~"ref count increases";
		test_logger.pass(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let t = l.lwc_intern_string(~"abcdef");
		let r = l.lwc_string_ref(t);

		// test 8: ref count decrease of a interned string
		function_name = ~"lwc_string_unref";
		test_name = ~"ref count decrease of a interned string";
		comment = ~"ref count decreases";
		test_logger.pass(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let t = l.lwc_intern_string(~"abcdef");
		let r = l.lwc_string_unref(t);

		// test 9: ref count decrease of a interned string with ref count already 0
		function_name = ~"lwc_string_unref";
		test_name = ~"ref count decrease of a interned string with ref count already 0";
		comment = ~"this case handled : but should return some error or warning if tried";
		test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		let t = l.lwc_intern_string(~"abcdef");
		l.lwc_string_unref(t.clone());
		l.lwc_string_unref(t.clone());



		// test 10: ref count decrease of a interned string with ref count already 0
		// function_name = ~"lwc_string_unref";
		// test_name = ~"ref count decrease of a interned string with ref count already 0";
		// comment = ~"this case handled : but should return some error or warning if tried";
		// test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		// let t = l.lwc_intern_string(~"abcdef");
		// l.lwc_string_unref(t.clone());
		// l.lwc_string_unref(t.clone());

		// test 11: test for a static function
		function_name = ~"lwc_string_data";
		test_name = ~"data of a lwc_string";
		comment = ~"returns data ";
		test_logger.info(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		io::println(fmt!("%?" , lwc::lwc_string_data(p)));

		// test 12: internment of a sub string of lenght 5 from offset 2 in null string(slice with lenght or offset greater than actual length of string)
		// function_name = ~"lwc_intern_substring";
		// test_name = ~"internment of a sub string of lenght 5 from offset 2 in null string(slice with lenght or offset greater than actual length of string)";
		// comment = ~"task fails: index out of bound";
		// test_logger.fail(copy module_name , copy file_name , copy function_name , copy test_name , copy comment);
		// let r = l.lwc_intern_substring(p ,2 , 5);

	}
}