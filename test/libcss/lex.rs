extern mod std;
extern mod parserutils;
extern mod css;

use parserutils::charset::csdetect::*;
use parserutils::charset::aliases::*;
use parserutils::input::inputstream::*;
use css::lex::lexer::*;

fn main() {
    io::println("lex");
    // lex(~"data/lex/tests1.dat");
}

fn lex(fileName: ~str) {
    let file=fileName;
    let (inputStreamOption, ParserUtilsError)= inputstream(Some(~"UTF-8"),Some(CSS_CHARSET_DEFAULT), Some(~css__charset_extract));
    match(ParserUtilsError) {
        PARSERUTILS_OK=>{}
        _ => {assert!(false);} // when inputstream is not created
    }
    let inputstream = 
        match(inputStreamOption) {
            Some(x)   => x,
            None => {
                // io::println("InputStream is not created, hence lexer can't be initialised");                        
                fail!(~"inputstream is None");
            }
        };

    let mut lexer = css_lexer::css__lexer_create(inputstream);

    let r: @Reader = io::file_reader(&Path(file)).get();
    let mut dataFlag = false;
    // let mut expectedFlag = false;
    let mut resetFlag = false;
    let mut finalstr: ~str = ~"";
    let mut final_buf: ~[u8] = ~[];

    while !r.eof() {
        let buf = r.read_line();

        if buf == ~"#data" {
            // io::println(buf);
            dataFlag = true;
            // expectedFlag = false;
            resetFlag = false; 
        }
        else if buf == ~"#errors" {
            dataFlag = false;
            // expectedFlag = false;
            resetFlag = false;
        }
        else if buf == ~"#expected" {
            // expectedFlag = true;
            dataFlag = false;
            resetFlag = false;

        }
        else if buf == ~"#reset" {
            dataFlag = false;
            // expectedFlag = false;
            resetFlag = true;
        }
        
        else if dataFlag {
            // io::println(buf);
            finalstr.push_str(buf);
            // io::println(finalstr);
        }

        if resetFlag {
            for str::each_char(finalstr) |i| {
                final_buf.push(i as u8);
            }
            finalstr = ~"";
            // io::println(str::from_bytes(final_buf));
            lexer.lexer_append_data(copy final_buf);
            final_buf = ~[];

            let mut tok:css_token_type;
            loop {
                // io::println("hi");
                let mut (tokOption, STATUS) = lexer.get_token();
                match STATUS {
                    LEXER_NEEDDATA => {

                        if tokOption.is_some() {
                            // io::println("hi 1");
                            tok = tokOption.unwrap();
                        }
                        break
                    },
                    _ => {}
                }
                //failing at .5%
                tok = match tokOption {
                    Some(tok) => tok,
                    None => {
                        break
                    }
                };
                // io::println(fmt!("%?" , tok));
                match tok {
                    CSS_TOKEN_EOF => break,
                    _ => {}
                }
            }
        }
    }
}

#[test]
fn tests1() {
    lex(~"data/lex/tests1.dat");
}

#[test]
fn tests2() {
    lex(~"data/lex/tests2.dat");
}

#[test]
fn regression() {
    lex(~"data/lex/regression.dat");
}