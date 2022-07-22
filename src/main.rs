mod lexer;
mod parser;
mod types;

use lexer::lex;
use parser::parse;
use std::env::args;
use types::*;

const USAGE: &str = "Usage: <command> <expression>";

fn main() -> Result<()> {
    let mut args = args();
    let _ = args.next();
    let input = args.next().expect(USAGE);
    if args.next().is_some() {
        panic!("{USAGE}");
    }

    let mut input = input.chars();
    let tokens = lex(&mut input)?;
    let expr = parse(tokens)?;

    println!("{:?}", expr);
    Ok(())
}
