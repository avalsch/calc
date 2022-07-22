use rust_decimal::Decimal;
use crate::types::*;

pub fn eval(expr: Expr) -> Result<Decimal> {
    Ok(match expr {
        Expr::Num(num) => num,
        Expr::Unary(op, expr) => eval_unary(op, eval(*expr)?)?,
        Expr::Binary(left, op, right) => eval_binary(op, eval(*left)?, eval(*right)?)?,
    })
}

fn eval_unary(op: Op, num: Decimal) -> Result<Decimal> {
    match op {
        Op::Add => Ok(num),
        Op::Sub => Ok(-num),
        _ => Err(Error::Unexpected(op.to_string())),
    }
}

fn eval_binary(op: Op, left: Decimal, right: Decimal) -> Result<Decimal> {
    match op {
        Op::Add => Ok(left + right),
        Op::Sub => Ok(left - right),
        Op::Mul => Ok(left * right),
        Op::Div => Ok(left / right),
    }
}