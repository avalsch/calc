use std::fmt::{Display, Formatter, Result as FmtResult};
use rust_decimal::Decimal;


#[derive(Debug)]
pub enum Error {
    Unexpected(char),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Error::Unexpected(c) => format!("Unexpected character: {c}"),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Unexpected(_) => "Unexpected character",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Num(Decimal),
    Op(Op),
    Paren(Paren),
    Eol
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paren {
    Open,
    Close,
}
