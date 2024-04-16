use std::process::exit;

use crate::ir::lexer::Token;

pub(crate) fn error(msg: &str) -> ! {
    eprintln!("Error: {msg}!");
    exit(101);
}

pub(crate) fn get_token(res: Result<Token, ()>) -> Token {
    match res {
        Ok(t) => t,
        Err(_) => error("Error: Couldn't tokenize code properly!"),
    }
}
