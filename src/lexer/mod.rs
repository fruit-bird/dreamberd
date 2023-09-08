mod custom_parsers;
mod errors;

use self::custom_parsers::parse_int;
use self::errors::LexerError;

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(error = LexerError)]
#[logos(skip r"\s+")]
pub enum GoofyToken {
    // Keywords
    #[regex("[f?u?n?c?t?i?o?n?]{4,}")]
    KWFunction,
    #[token("here you go:")]
    KWReturn,

    // Identifiers
    #[token("bruh")]
    Bruh,
    #[regex("[a-zA-Z_]+")]
    Identifier,
    #[regex("[0-9][0-9_]*", parse_int)]
    Integer(u32),

    // Symbols
    #[token("=")]
    Equal,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[token(";")]
    SemiColon,
}
