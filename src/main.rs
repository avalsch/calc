mod eval;
mod lexer;
mod parser;
mod types;

use eval::eval;
use lexer::lex;
use parser::parse;
use std::env::args;
use types::*;

fn main() -> Result<()> {
    let mut args = args();
    let _ = args.next();

    let input = args.next().ok_or(Error::Usage)?;

    if args.next().is_some() {
        return Err(Error::Usage);
    }

    let mut input = input.chars();
    let tokens = lex(&mut input)?;
    let expr = parse(tokens)?;

    let result = eval(expr)?;
    println!("{result}");

    Ok(())
}
