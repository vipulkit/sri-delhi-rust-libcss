use libparserutils::utils::errors::*;
use utils::errors::*;

pub fn css_error_from_parserutils_error(css_result : parserutils_error) -> css_error{
	match css_result{

		PARSERUTILS_OK => { 
			return CSS_OK
		},
		PARSERUTILS_NOMEM => {
			return CSS_NOMEM
		},
		PARSERUTILS_BADPARM => {
			return CSS_BADPARM
		},
		PARSERUTILS_INVALID => {
			return CSS_INVALID
		},
		PARSERUTILS_FILENOTFOUND => {
			return CSS_FILENOTFOUND
		},
		PARSERUTILS_NEEDDATA => {
			return CSS_NEEDDATA
		},
		PARSERUTILS_BADENCODING => {
			return CSS_BADCHARSET
		},
		PARSERUTILS_EOF => {
			return CSS_EOF
		},
	}
}