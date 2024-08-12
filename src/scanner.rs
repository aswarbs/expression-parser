use std::str::Chars;
use itertools::Itertools;
// tokeniser
use super::tokens::Token;
use super::tokens::Operator;

pub fn scan(input: String) -> Vec<Token> {

    println!("scanning {}", input);

    let chars = &mut input.chars();

    let mut tokens = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenPar),
            ')' => tokens.push(Token::ClosedPar),
            '+' => tokens.push(Token::Oper(Operator::Plus)),
            '-' => tokens.push(Token::Oper(Operator::Minus)),
            '*' => tokens.push(Token::Oper(Operator::Times)),
            '/' => tokens.push(Token::Oper(Operator::Divide)),
            '%' => tokens.push(Token::Oper(Operator::Mod)),
            '|' => tokens.push(Token::Oper(Operator::Abs)),
            ' ' | '\n' => (), // ignore whitespace
            '0'..='9' =>tokens.push(Token::IntLiteral(scan_number(c, chars))),
            _ => panic!("invalid character {}", c)
        }
    }

    tokens
}

pub fn scan_number(c: char, chars: &mut Chars) -> i32 {

    (c.to_string() + &chars.peeking_take_while(|c| c.is_digit(10)).collect::<String>()).parse::<i32>().unwrap()
}