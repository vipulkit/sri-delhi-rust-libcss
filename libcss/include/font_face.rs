use wapcaplet::*;
use std::arc;

/* Font face */
pub enum css_font_face_format {
    CSS_FONT_FACE_FORMAT_UNSPECIFIED    =   0x00,
    CSS_FONT_FACE_FORMAT_WOFF       =   0x01, // WOFF (Web Open Font Format); .woff */
    CSS_FONT_FACE_FORMAT_OPENTYPE       =   0x02, // TrueType or OpenType; .ttf, .otf */
    CSS_FONT_FACE_FORMAT_EMBEDDED_OPENTYPE  =   0x04, // Embedded OpenType; .eot */
    CSS_FONT_FACE_FORMAT_SVG        =   0x08, // SVG Font; .svg, .svgz */
    CSS_FONT_FACE_FORMAT_UNKNOWN        =   0x10 // Format specified, but not recognised */
}

pub enum css_font_face_location_type{
    CSS_FONT_FACE_LOCATION_TYPE_UNSPECIFIED     =   0,
    CSS_FONT_FACE_LOCATION_TYPE_LOCAL       =   1,
    CSS_FONT_FACE_LOCATION_TYPE_URI         =   2           
}

pub struct css_font_face_src {
    location:Option<arc::RWARC<~lwc_string>>,
    /*
    * Bit allocations:
    *
    * 76543210
    * 1 _fffffll format | location type
    */
    bits:~[u8]
}

pub struct css_font_face {
    font_family:Option< arc::RWARC<~lwc_string> >,
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