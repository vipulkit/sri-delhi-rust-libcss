use wapcaplet::*;

/* Font face */
pub enum css_font_face_format {
    CSS_FONT_FACE_FORMAT_UNSPECIFIED    =   0x00,
    CSS_FONT_FACE_FORMAT_WOFF       =   0x01, // WOFF (Web Open Font Format); .woff */
    CSS_FONT_FACE_FORMAT_OPENTYPE       =   0x02, // TrueType or OpenType; .ttf, .otf */
    CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE  =   0x04, // Embedded OpenType; .eot */
    CSS_FONT_FACE_FORMAT_SVG        =   0x08, // SVG Font; .svg, .svgz */
    CSS_FONT_FACE_FORMAT_UNKNOWN        =   0x10 // Format specified, but not recognised */
     // We don't define CSS_FONT_FACE_SRC_FORMAT_TRUETYPE as might be 
     // * expected, because the CSS3 specification 
     // *  (http://www.w3.org/TR/css3-fonts/, Â§4.3) says:
     // *  "Given the overlap in common usage between TrueType and
     // *   OpenType, the format hints "truetype" and "opentype" must be 
     // *   considered as synonymous"
     // * so we compute a hint of 'truetype' to css_font_face_format_opentype.
     
}

pub enum css_font_face_location_type{
    CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED     =   0,
    CSS_FONT_FACE_LOCATION_TYPE_LOCAL       =   1,
    CSS_FONT_FACE_LOCATION_TYPE_URI         =   2           
}

pub struct css_font_face_src {
    location:Option<@mut lwc_string>,
    /*
    * Bit allocations:
    *
    * 76543210
    * 1 _fffffll format | location type
    */
    bits:~[u8]
}

pub struct css_font_face {
    font_family:Option< @mut lwc_string >,
    srcs:~[~css_font_face_src],
    /*
    * Bit allocations:
    *
    * 76543210
    * 1 __wwwwss font-weight | font-style
    */
    bits:~[u8]
}
/* font face end */
