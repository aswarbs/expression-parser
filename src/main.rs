mod scanner;
mod parser;
mod evaluator;
mod tokens;

use std::io;
use std::io::{stdin, Error, Write};
use scanner::scan;

use parser::parse_exp;

use evaluator::evaluate;

fn main() -> Result<(), String> {

    let line = "1 + 2 * 3".to_string();

    let mut tokens = scan(line);

    println!("{:?}", tokens);

    let ast = parse_exp(&mut tokens)?;

    println!("ast: {:?}", ast);

    if tokens.len() > 0 {Err("parser finished early".to_string())} else {Ok(())};

    let result = evaluate(ast)?;

    println!("{}", result);

    Ok(())







}
