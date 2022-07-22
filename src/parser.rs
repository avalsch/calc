use std::iter::Peekable;

use crate::types::*;

pub fn parse(tokens: impl Iterator<Item = Token>) -> Result<Expr> {
    let mut tokens = tokens.peekable();
    parse_expr(&mut tokens, 0)
}

fn parse_expr(tokens: &mut Peekable<impl Iterator<Item = Token>>, precedence: i32) -> Result<Expr> {
    let mut expr = null_denotation(tokens)?;

    while let Some(&token) = tokens.peek() {
        let op = match token {
            Token::Paren(Paren::Close) | Token::Eol => break,
            Token::Op(op) => op,
            _ => return Err(Error::Unexpected(token.to_string())),
        };

        if op.precedence() < precedence {
            break;
        }

        tokens.next();
        expr = left_denotation(tokens, expr, op)?;
    }

    Ok(expr)
}

fn null_denotation(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expr> {
    let token = tokens.next().ok_or(Error::UnexpectedEol)?;

    let expr = match token {
        Token::Num(num) => Expr::Num(num),
        Token::Op(Op::Add) => parse_expr(tokens, 0)?,
        Token::Op(Op::Sub) => Expr::Unary(Op::Sub, Box::new(parse_expr(tokens, 0)?)),
        Token::Paren(Paren::Open) => {
            let expr = parse_expr(tokens, 0)?;
            tokens.next().ok_or(Error::MissingParen)?;
            expr
        }
        _ => return Err(Error::Unexpected(token.to_string())),
    };

    Ok(expr)
}

fn left_denotation(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    left: Expr,
    op: Op,
) -> Result<Expr> {
    let expr = Expr::Binary(
        Box::new(left),
        op,
        Box::new(parse_expr(tokens, op.precedence())?),
    );
    Ok(expr)
}
