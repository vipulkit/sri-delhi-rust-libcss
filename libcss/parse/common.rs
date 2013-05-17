
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
