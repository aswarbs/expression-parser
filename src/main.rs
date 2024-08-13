mod scanner;
mod parser;
mod evaluator;
mod tokens;
mod assembler;
mod vm;

use scanner::scan;
use parser::parse_exp;
use crate::assembler::ast_to_tam;
use crate::vm::exec_tam;

use std::{env, io};
use std::fs;
use std::fs::File;
use crate::tokens::TAMInst;
use std::io::Write;


fn main() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let trace = args.contains(&"trace".to_string());
    let run = args.contains(&"run".to_string());

    let file_path = args.iter().find(|s| s.contains(".exp") || s.contains(".tam")).ok_or("file not specified")?;

    let contents = fs::read_to_string(file_path).expect("Could not read the file");

    let file_extension = file_path.rsplit('.').next().ok_or("invalid file")?;
    let mut file_name = String::from(file_path.rsplit_once('.').ok_or("invalid file")?.0);

    match file_extension {
        "exp" => { file_name.push_str(".tam");
            run_exp(&file_name, contents.clone(), trace, run)},

        "tam" => { run_tam(&contents.clone(), trace)},
        _ => panic!("invalid file type."),
    };


    Ok(())
}


fn run_exp(target: &String, expression:String, trace:bool, run:bool) -> Result<String, String> {

    let mut tokens = scan(expression)?;
    println!("{:?}", tokens);
    let ast = parse_exp(&mut tokens)?;
    println!("ast: {:?}", ast);
    if tokens.len() > 0 { Err("parser finished early".to_string()) } else { Ok(()) }?;
    let assembled_code = ast_to_tam(ast);

    write_tam_to_file(target, assembled_code);

    if run { run_tam(&fs::read_to_string(target).expect("Could not read the file"), trace); }

    Ok("".to_string())
}

fn run_tam(tam_instructions:&String, trace:bool) -> Result<String, String> {

    let assembled_code = read_tam_instructions(tam_instructions).ok_or("failed to parse tam")?;
    let mut stack = Vec::new();
    let result = exec_tam(&mut stack, assembled_code, trace)?;
    let value = result.first().ok_or("result is empty")?;

    println!("{:?}", result);
    Ok(value.to_string())

}


fn read_tam_instructions(tam_instructions: &String) -> Option<Vec<TAMInst>> {
    tam_instructions.lines()
        .map(|line| parse_line(&line))
        .collect()
}

fn parse_line(line: &str) -> Option<TAMInst> {
    let mut parts = line.split_whitespace();
    match (parts.next()?, parts.next()) {
        ("LOADL", Some(arg)) => arg.parse().ok().map(TAMInst::LOADL),
        ("ADD", None) => Some(TAMInst::ADD),
        ("SUB", None) => Some(TAMInst::SUB),
        ("MUL", None) => Some(TAMInst::MUL),
        ("DIV", None) => Some(TAMInst::DIV),
        ("NEG", None) => Some(TAMInst::NEG),
        ("MOD", None) => Some(TAMInst::MOD),
        ("ABS", None) => Some(TAMInst::ABS),
        ("AND", None) => Some(TAMInst::AND),
        ("OR", None) => Some(TAMInst::OR),
        ("NOT", None) => Some(TAMInst::NOT),
        ("GTR", None) => Some(TAMInst::GTR),
        ("LSS", None) => Some(TAMInst::LSS),
        ("EQL", None) => Some(TAMInst::EQL),
        _ => None,
    }
}
fn write_tam_to_file(target: &String, tam_instructions: Vec<TAMInst>) -> io::Result<()> {
    let mut file = File::create(target)?;
    for instruction in tam_instructions {
        writeln!(file, "{}", instruction)?;
    }
    Ok(())
}


