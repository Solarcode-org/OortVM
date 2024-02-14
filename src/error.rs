use crate::ir::lexer::Token;

pub(crate) fn error(msg: &str) -> ! {
    panic!("{msg}");
}

pub(crate) fn get_token(res: Result<Token, ()>) -> Token {
    match res {
        Ok(t) => t,
        Err(_) => error("Error: Couldn't tokenize code properly!"),
    }
}
