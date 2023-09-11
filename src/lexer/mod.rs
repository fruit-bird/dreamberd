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
    #[regex("[f?u?n?c?t?i?o?n?]{2,}")]
    KWFunction,
    #[token("return")]
    KWReturn,
    #[token("const")]
    KWConst,
    #[token("var")]
    KWVar,
    #[token("when")]
    KWWhen,
    #[token("export")]
    KWExport,
    #[token("to")]
    KWTo,
    #[token("class")]
    KWClass,
    #[token("new")]
    KWNew,
    #[token("delete")]
    KWDelete,
    #[token("previous")]
    KWPrevious,
    #[token("after")]
    KWAfter,
    #[token("async")]
    KWAsync,
    #[token("noop")]
    KWNoop,
    #[token("use")]
    KWUse,

    // Built-in types (might change this)
    #[token("Bruh")]
    Bruh,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("maybe")]
    Maybe,
    #[regex("Int[0-9]{1,2}")]
    IntN, // matches Int1 -> Int99
    #[regex("/Reg(ular)?[eE]x(press(ion)?|p)?/")]
    Regex,

    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z_0-9]*")]
    Identifier,
    #[regex("[0-9][0-9_]*", parse_int)]
    Integer(u32),

    // Symbols
    #[token("=")]
    Equal,
    #[token(":")]
    Colon,
    #[token(",")]
    Period,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,
    #[token("[")]
    BracketOpen,
    #[token("]")]
    BracketClose,
    #[token("!")]
    Bang,
    #[token("¡")]
    InvertedBang,
    #[token("?")]
    QuestionMark,
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("'")]
    Quote,
    #[token("\"")]
    DoubleQuote,
    #[token("=>")]
    RFatArrow,
    #[token(";")]
    Not,
}
