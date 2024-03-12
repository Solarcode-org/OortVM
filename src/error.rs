use std::collections::HashMap;
use std::sync::{LockResult, MutexGuard};
use crate::ir::compile::Compile;
use crate::ir::lexer::Token;

pub(crate) fn error<T: ToString>(msg: T) -> ! {
    panic!("{}", msg.to_string());
}

pub(crate) fn get_token(res: Result<Token, ()>) -> Token {
    match res {
        Ok(t) => t,
        Err(_) => error("Error: Couldn't tokenize code properly!"),
    }
}

pub(crate) fn cure_poison(res: LockResult<MutexGuard<HashMap<String, Compile>>>)
    -> MutexGuard<HashMap<String, Compile>> {
    match res {
        Ok(mg) => mg,
        Err(err) => {
            error(err);
        }
    }
}
