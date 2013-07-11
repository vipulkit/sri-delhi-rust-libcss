extern mod std;
use std::io;

fn main() {
    select_auto();   // ok
    parse2_auto();   // ok
    // parse_auto();
    lex_auto();      // ok
    number();        // ok
}

fn readExpected(filename: ~str) -> ~[~[~str]] {
    let r: @Reader = io::file_reader(&Path(filename)).get();
    let mut results = ~[];

    while !r.eof() {
        let mut line = r.read_line();
        if line == ~"#expected" {
            let mut expected = ~[];
            while !r.eof() {
                line = r.read_line();
                if line == ~"#reset" {
                    break;
                }
                expected.push(line);
            }
            results.push(expected);
        }
    }
    return results;
}


fn compare(expected: &[~[~str]], output: &[~[~str]]) -> bool {
    let mut ok=true;
    for uint::range(0, expected.len()) |i| {
        for uint::range(0, expected[i].len()) |j| {
            if !str::eq(&expected[i][j], &output[i][j]) {
                io::println(fmt!("error: %s != %s", expected[i][j], output[i][j]));
                ok=false;
                // println(expected[i][j]);
                // println(output[i][j]);

                // assert!(false);
                return false;
            }
        }
    }
    return true;
}

// #[test]
pub fn select_auto() {
    let expected = readExpected(~"data/select/tests1.dat");
    let output = readExpected(~"output/select-auto.out");

    compare(expected, output);
}

// #[test]
fn parse2_auto() {
    let mut results = readExpected(~"data/parse2/au.dat");
    results.push_all(readExpected(~"data/parse2/bg.dat"));
    results.push_all(readExpected(~"data/parse2/bgpos.dat"));
    results.push_all(readExpected(~"data/parse2/border.dat"));
    results.push_all(readExpected(~"data/parse2/comments.dat"));
    results.push_all(readExpected(~"data/parse2/eof.dat"));
    results.push_all(readExpected(~"data/parse2/font.dat"));
    // readExpected(~"data/parse2/illegal-values.dat");
    results.push_all(readExpected(~"data/parse2/list.dat"));
    results.push_all(readExpected(~"data/parse2/malformed-declarations.dat"));
    results.push_all(readExpected(~"data/parse2/margin.dat"));
    results.push_all(readExpected(~"data/parse2/multicol.dat"));
    results.push_all(readExpected(~"data/parse2/outline.dat"));
    results.push_all(readExpected(~"data/parse2/padding.dat"));
    results.push_all(readExpected(~"data/parse2/selectors.dat"));
    results.push_all(readExpected(~"data/parse2/tests1.dat"));
    results.push_all(readExpected(~"data/parse2/unknown-properties.dat"));
    let output = readExpected(~"output/parse2-auto.out");

    compare(results, output);
}

// #[test]
fn parse_auto() {
    let mut results = ~[];
    results.push_all(readExpected(~"data/parse/tests1.dat"));
    results.push_all(readExpected(~"data/parse/atrules.dat"));
    results.push_all(readExpected(~"data/parse/colours.dat"));
    results.push_all(readExpected(~"data/parse/colours-hsl.dat"));
    results.push_all(readExpected(~"data/parse/nth.dat"));
    // error: cannot dump
    // results.push_all(readExpected(~"data/parse/properties.dat"));
    results.push_all(readExpected(~"data/parse/selectors.dat"));

    let output = readExpected(~"output/parse-auto.out");
    compare(results, output);
}

// #[test]
fn lex_auto() {
    let mut results = ~[];
    results.push_all(readExpected(~"data/lex/tests1.dat"));
    results.push_all(readExpected(~"data/lex/tests2.dat"));
    results.push_all(readExpected(~"data/lex/regression.dat"));

    let output = readExpected(~"output/lex-auto.out");
    compare(results, output);
}

fn number() {
    let mut results = ~[];
    results.push_all(readExpected(~"data/number/number.dat"));
    let output = readExpected(~"output/number.out");
    compare(results, output);
}


