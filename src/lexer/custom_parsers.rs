use logos::Lexer;

use super::GoofyToken;

pub fn parse_int(lex: &Lexer<GoofyToken>) -> Option<u32> {
    let slice = lex.slice();
    slice.parse().ok()
}
