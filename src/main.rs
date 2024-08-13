mod scanner;
mod parser;
mod evaluator;
mod tokens;
mod assembler;
mod vm;

use scanner::scan;

use parser::parse_exp;

use evaluator::direct_evaluate;
use crate::assembler::ast_to_tam;
use crate::vm::exec_tam;

fn main() -> Result<(), String> {


    let line = "1 != 1".to_string();

    let mut tokens = scan(line)?;

    println!("{:?}", tokens);

    let ast = parse_exp(&mut tokens)?;

    println!("ast: {:?}", ast);

    if tokens.len() > 0 { Err("parser finished early".to_string()) } else { Ok(()) }?;

    let evaluate_with_tam = true;

    if evaluate_with_tam {

        let assembled_code = ast_to_tam(ast);

        println!("{:?}", assembled_code);

        let mut stack = Vec::new();

        let result = exec_tam(&mut stack, assembled_code);
        println!("{:?}", result);

    }
    else {
        let result = direct_evaluate(ast);
        println!("{:?}", result);
    }




    Ok(())







}
