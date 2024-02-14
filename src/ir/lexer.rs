use logos::Logos;

/// All meaningful JSON tokens.
///
/// > NOTE: regexes for [`Token::Number`] and [`Token::String`] may not
/// > catch all possible values, especially for strings. If you find
/// > errors, please report them so that we can improve the regex.
#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub(crate) enum Token {
    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[regex(r"%[A-Za-z][A-Za-z_\-\/]+", |lex| lex.slice()[1..].to_owned())]
    Descriptor(String),

    #[regex("[A-Za-z][A-Za-z0-9]+", |lex| lex.slice().to_owned())]
    Ident(String),
}
