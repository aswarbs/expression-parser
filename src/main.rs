mod scanner;
mod parser;
mod evaluator;
mod tokens;
mod assembler;

use scanner::scan;

use parser::parse_exp;

use evaluator::direct_evaluate;

fn main() -> Result<(), String> {


    // my negation has the wrong precedence

    let line = "|-5 + 2| - (1 * 3)".to_string();

    let mut tokens = scan(line);

    println!("{:?}", tokens);

    let ast = parse_exp(&mut tokens)?;

    println!("ast: {:?}", ast);

    if tokens.len() > 0 { Err("parser finished early".to_string()) } else { Ok(()) }?;

    //let assembled_code = ast_to_tam(ast);

    let assembled_code = direct_evaluate(ast);

    println!("{:?}", assembled_code);

    Ok(())







}
