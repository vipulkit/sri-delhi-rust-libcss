use lex::lexer::*;
/*
 * Css parser events , sent during parsing
 */
pub enum css_parser_event {
    CSS_PARSER_START_STYLESHEET,
    CSS_PARSER_END_STYLESHEET,
    CSS_PARSER_START_RULESET,
    CSS_PARSER_END_RULESET,
    CSS_PARSER_START_ATRULE,
    CSS_PARSER_END_ATRULE,
    CSS_PARSER_START_BLOCK,
    CSS_PARSER_END_BLOCK,
    CSS_PARSER_BLOCK_CONTENT,
    CSS_PARSER_DECLARATION
}

pub fn vector_peek<'r>(vector:&'r ~[@css_token], ctx: &mut uint) -> Option<&'r @css_token> {
    if (*ctx >= vector.len()) {
        None
    }
    else {
        Some(&vector[*ctx])
    }
}

pub fn vector_iterate<'r>(vector:&'r ~[@css_token], ctx: &mut uint) -> Option<&'r @css_token> {
    if (*ctx >= vector.len()) {
        None
    }
    else {
        let token = Some(&vector[*ctx]);
        *ctx += 1;
        token
    }
}