use crate::types::*;
use rust_decimal::prelude::*;
use std::iter::Peekable;

pub fn lex(chars: impl Iterator<Item = char>) -> Result<impl Iterator<Item = Token>> {
    let tokens = parse(&mut chars.peekable())?;

    Ok(tokens.into_iter())
}

fn parse(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    while let Some(&c) = chars.peek() {
        let token = match c {
            _ if c.is_ascii_whitespace() => {
                chars.next();
                continue;
            }
            '0'..='9' | '.' => {
                if let Some(num) = parse_num(chars) {
                    Token::Num(num)
                } else {
                    return Err(Error::Unexpected(c.into()));
                }
            }
            '+' => Token::Op(Op::Add),
            '-' => Token::Op(Op::Sub),
            '*' => Token::Op(Op::Mul),
            '/' => Token::Op(Op::Div),
            '(' => Token::Paren(Paren::Open),
            ')' => Token::Paren(Paren::Close),
            _ => return Err(Error::Unexpected(c.into())),
        };

        if matches!(token, Token::Op(_) | Token::Paren(_)) {
            chars.next();
        }

        tokens.push(token);
    }

    tokens.push(Token::Eol);

    Ok(tokens)
}

fn parse_num(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Decimal> {
    let mut num = String::new();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => num.push(c),
            '.' if !num.contains('.') => num.push(c),
            _ => break,
        };
        chars.next();
    }

    Decimal::from_str(&num).ok()
}
