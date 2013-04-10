#[link(name = "css_lexer", vers = "0.1")];
#[crate_type = "lib"];

extern mod css_ds;
extern mod css_enum;
extern mod parserutils_inputstream;
extern mod std;


use css_enum::* ;
use css_ds::*;
use parserutils_inputstream::*;

pub fn ascii_lower(string: &str) -> ~str {
    do str::map(string) |c| {
        match c {
            'A'..'Z' => c + ASCII_LOWER_OFFSET,
            _ => c,
        }
    }
}

pub fn preprocess(input: &str) -> ~str {
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
    transform_function_whitespace: bool,
    input: ~[u8],
    length: uint, // Counted in bytes, not characters
    position: uint, // Counted in bytes, not characters
}


impl lcss_lexer {
    fn from_vec(input: ~[u8], transform_function_whitespace: bool)
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

    pub fn css__lexer_get_token(&mut self) -> (Token, Option<ParseError>) {
        if self.is_eof() { 
            (EOF, None) 
        }
        else { 
            self.consume_token()
        }
    }

    fn handle_transform_function_whitespace(&mut self, string: ~str)
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

    pub fn consume_token(&mut self) -> (Token, Option<ParseError>) {
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

    fn consume_quoted_string(&mut self, single_quote: bool)
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

    fn consume_hash(&mut self) -> (Token, Option<ParseError>) {
        let string = self.consume_ident_string_rest();
        (if string == ~"" { Delim('#') } else { Hash(string) }, None)
    }

    fn consume_char(&mut self) -> char {
        let range = str::char_range_at(str::from_bytes(self.input), self.position);
        self.position = range.next;
        range.ch
    }

    fn match_here(&mut self, needle: ~str) -> bool {
        let mut i = self.position;
        if i + needle.len() > self.length { return false }
        let haystack: &str = str::from_bytes(self.input);
        for needle.each |c| { if haystack[i] != c { return false; } i += 1u; }
        return true;
    }

    fn consume_comments(&mut self)
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

    fn consume_at_keyword(&mut self) -> (Token, Option<ParseError>) {
        (match self.consume_ident_string() {
            Some(string) => AtKeyword(string),
            None => Delim('@')
        }, None)
    }

    fn current_char(&mut self) -> char {
        str::char_at(str::from_bytes(self.input) , self.position)
    }

    fn next_is_namestart_or_escape(&mut self) -> bool {
        self.position += 1;
        let result = !self.is_eof() && self.is_namestart_or_escape();
        self.position -= 1;
        result
    }

    fn next_n_chars(&mut self, n: uint) -> ~[char] {
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

    fn is_invalid_escape(&mut self) -> bool {
        match self.next_n_chars(2) {
            ['\\', '\n'] | ['\\', '\x0C'] | ['\\'] => true,
            _ => false,
        }
    }

    fn is_namestart_or_escape(&mut self) -> bool {
        match self.current_char() {
            'a'..'z' | 'A'..'Z' | '_' => true,
            '\\' => !self.is_invalid_escape(),
            c => c >= '\x80', // Non-ASCII
        }
    }


    fn consume_ident(&mut self) -> (Token, Option<ParseError>) {
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

    fn consume_ident_string(&mut self) -> Option<~str> {
        match self.current_char() {
            '-' => if !self.next_is_namestart_or_escape() { None }
                   else { Some(self.consume_ident_string_rest()) },
            '\\' if self.is_invalid_escape() => return None,
            _ if !self.is_namestart_or_escape() => return None,
            _ => Some(self.consume_ident_string_rest())
        }
    }

    fn consume_ident_string_rest(&mut self) -> ~str {
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

    fn consume_escape(&mut self) -> char {
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

    fn consume_url(&mut self) -> (Token, Option<ParseError>) {
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

    fn consume_quoted_url(&mut self, single_quote: bool)
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

    fn consume_unquoted_url(&mut self) -> (Token, Option<ParseError>) {
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

    fn consume_url_end(&mut self, string: ~str)
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

    fn consume_bad_url(&mut self) -> (Token, Option<ParseError>) {
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

    fn consume_unicode_range(&mut self)
            -> (Token, Option<ParseError>) {
        let next_3 = self.next_n_chars(3);
        // We got here with U or u
        assert! (next_3[0] == 'u')||(next_3[0] == 'U') ;
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
        assert! (hex.len() > 0);
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

    fn consume_numeric(&mut self) -> (Token, Option<ParseError>) {
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

    fn consume_numeric_sign(&mut self, sign: char)
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

    fn consume_numeric_rest(&mut self, initial_char: char)
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

        let temp : ~str ;          
        if string[0] != '+' as u8 { temp = copy string; }
        else { temp = str::substr(string, 1, string.len()).to_owned(); }
        let value = Integer(int::from_str(temp
            // Remove any + sign as int::from_str() does not parse them.

        ).get()); // XXX handle overflow
        self.consume_numeric_end(string, value)
    }

    fn consume_numeric_fraction(&mut self, string: ~str)
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


    fn consume_numeric_end(&mut self, string: ~str,
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


    fn consume_scientific_number(&mut self, string: ~str)
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

    pub fn error_token(&mut self ,t: Token, message: ~str) -> (Token, Option<ParseError>) {
        (t, Some(ParseError{message: message}))
    }

    pub fn css__lexer_create(input: ~lpu_inputstream) -> css_result {
        CSS_OK
    }
}

pub fn lcss_lexer()->~lcss_lexer {
    let mut lexer :~lcss_lexer= 
    	~lcss_lexer{ transform_function_whitespace: false,
    	input: ~[],
    	length: 0, 
    	position: 0 } ;
    lexer
}