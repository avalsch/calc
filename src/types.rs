use rust_decimal::Decimal;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Usage,
    Unexpected(String),
    UnexpectedEol,
    MissingParen,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let msg = match self {
            Error::Usage => "Usage: <command> <expression>".into(),
            Error::Unexpected(c) => format!("Unexpected character: {c}"),
            Error::UnexpectedEol => "Unexpected end of input".into(),
            Error::MissingParen => "Missing closing parenthesis".into(),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Usage => "Usage: <command> <expression>",
            Error::Unexpected(_) => "Unexpected character",
            Error::UnexpectedEol => "Unexpected end of input",
            Error::MissingParen => "Missing closing parenthesis",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Num(Decimal),
    Op(Op),
    Paren(Paren),
    Eol,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Token::Num(num) => write!(f, "{num}"),
            Token::Op(op) => write!(f, "{op}"),
            Token::Paren(paren) => write!(f, "{paren}"),
            Token::Eol => write!(f, "EOL"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn precedence(&self) -> i32 {
        match self {
            Op::Add | Op::Sub => 10,
            Op::Mul | Op::Div => 20,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Paren {
    Open,
    Close,
}

impl Display for Paren {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Paren::Open => write!(f, "("),
            Paren::Close => write!(f, ")"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(Decimal),
    Unary(Op, Box<Expr>),
    Binary(Box<Expr>, Op, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Expr::Num(num) => write!(f, "{num}"),
            Expr::Unary(op, expr) => write!(f, "({op} {expr})", op = op, expr = expr),
            Expr::Binary(lhs, op, rhs) => {
                write!(f, "({lhs} {op} {rhs})", lhs = lhs, op = op, rhs = rhs)
            }
        }
    }
}
