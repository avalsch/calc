use std::iter::Peekable;

use crate::types::*;

pub fn parse(tokens: impl Iterator<Item = Token>) -> Result<Expr> {
    let mut tokens = tokens.peekable();
    parse_expr(&mut tokens)
}

fn parse_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr> {
    let expr = null_denotation(tokens)?;

    todo!()
}

fn null_denotation(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr> {
    let token = tokens.next().ok_or(Error::UnexpectedEol)?;
    let expr = match token {
        Token::Num(num) => Expr::Num(num),
        Token::Op(Op::Add) => parse_expr(tokens)?,
        Token::Op(Op::Sub) => Expr::Unary(Op::Sub, Box::new(parse_expr(tokens)?)),
        Token::Paren(Paren::Open) => {
            let expr = parse_expr(tokens)?;
            tokens.next().ok_or(Error::MissingParen)?;
            expr
        }
        _ => return Err(Error::Unexpected(token.to_string())),
    };

    Ok(expr)
}

fn left_denotation(tokens: &mut Peekable<impl Iterator<Item = Token>>, left: Expr) -> Result<Expr> {
    todo!()
}

fn binding_power(op: Op) -> i32 {
    match op {
        Op::Add | Op::Sub => 10,
        Op::Mul | Op::Div => 20,
    }
}
