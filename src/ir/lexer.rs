use logos::Logos;

/// All meaningful JSON tokens.
///
/// > NOTE: regexes for [`Token::Number`] and [`Token::String`] may not
/// > catch all possible values, especially for strings. If you find
/// > errors, please report them so that we can improve the regex.
#[derive(Debug, Logos)]
#[logos(skip r"[ \t\r\n\f]+")]
pub(crate) enum Token {
    #[regex(r"%[A-Za-z][A-Za-z_\-\/]+", |lex| lex.slice()[1..].to_owned())]
    Descriptor(String),

    #[regex("[A-Za-z][A-Za-z0-9]+", |lex| lex.slice().to_owned())]
    Ident(String),

    #[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    Number(f64),

    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| lex.slice().replace('\"', "").to_owned())]
    String(String),
}

#[cfg(test)]
mod lexer_test {
    use super::*;
    #[test]
    fn test_descriptor() {
        let mut tokens = Token::lexer("%func");

        if let Some(token) = tokens.next() {
            let token = token.unwrap();

            match token {
                Token::Descriptor(d) => {
                    assert_eq!(d, "func".to_string());
                }
                _ => panic!("Not a descriptor!")
            }
        }
    }
    #[test]
    fn test_ident() {
        let mut tokens = Token::lexer("print");

        if let Some(token) = tokens.next() {
            let token = token.unwrap();

            match token {
                Token::Ident(id) => {
                    assert_eq!(id, "print".to_string());
                }
                _ => panic!("Not an identifier!")
            }
        }
    }
    #[test]
    fn test_number() {
        let mut tokens = Token::lexer("123.56");

        if let Some(token) = tokens.next() {
            let token = token.unwrap();

            match token {
                Token::Number(n) => {
                    assert_eq!(n, 123.56_f64);
                }
                _ => panic!("Not a number!")
            }
        }
    }
    #[test]
    fn test_string() {
        let mut tokens = Token::lexer("\"Hello, world!\"");

        if let Some(token) = tokens.next() {
            let token = token.unwrap();

            match token {
                Token::String(s) => {
                    assert_eq!(s, "Hello, world!".to_string());
                }
                _ => panic!("Not a string!")
            }
        }
    }
}