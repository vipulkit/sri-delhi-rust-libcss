pub enum parserutils_error {
    PARSERUTILS_OK,
    PARSERUTILS_NOMEM,
    PARSERUTILS_BADPARM,
    PARSERUTILS_INVALID,
    PARSERUTILS_FILENOTFOUND,
    PARSERUTILS_NEEDDATA,
    PARSERUTILS_BADENCODING,
    PARSERUTILS_EOF
}

pub fn parserutils_error_from_string(error_string: ~str) -> parserutils_error {
    if (error_string.starts_with(&"PARSERUTILS_OK")) {
        PARSERUTILS_OK
    } else if (error_string.starts_with(&"PARSERUTILS_NOMEM")) {
        PARSERUTILS_NOMEM
    } else if (error_string.starts_with(&"PARSERUTILS_BADPARM")) {
        PARSERUTILS_BADPARM
    } else if (error_string.starts_with(&"PARSERUTILS_INVALID")) {
        PARSERUTILS_INVALID
    } else if (error_string.starts_with(&"PARSERUTILS_FILENOTFOUND")) {
        PARSERUTILS_FILENOTFOUND
    } else if (error_string.starts_with(&"PARSERUTILS_NEEDDATA")) {
        PARSERUTILS_NEEDDATA
    } else if (error_string.starts_with(&"PARSERUTILS_BADENCODING")) {
        PARSERUTILS_BADENCODING
    } else if (error_string.starts_with(&"PARSERUTILS_EOF")) {
        PARSERUTILS_EOF
    }
    else {
        PARSERUTILS_OK
    }
}

pub fn parserutils_error_to_string(error: parserutils_error) -> ~str {
    match (error) {
        PARSERUTILS_OK => {
            ~"No error"
        },
        PARSERUTILS_NOMEM => {
            ~"Insufficient memory"
        },
        PARSERUTILS_BADPARM => {
            ~"Bad parameter"
        },
        PARSERUTILS_INVALID => {
            ~"Invalid input"
        },
        PARSERUTILS_FILENOTFOUND => {
            ~"File not found"
        },
        PARSERUTILS_NEEDDATA => {
            ~"Insufficient data"
        },
        PARSERUTILS_BADENCODING => {
            ~"Unsupported encoding"
        },
        PARSERUTILS_EOF => {
            ~"EOF"
        }
    }
}