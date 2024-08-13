use std::str::Chars;
use itertools::Itertools;
// tokeniser
use super::tokens::Token;
use super::tokens::TokenOperator;

pub fn scan(input: String) -> Result<Vec<Token>, String> {

    let chars = &mut input.chars();

    let mut tokens = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '(' => tokens.push(Token::OpenPar),
            ')' => tokens.push(Token::ClosedPar),
            '+' => tokens.push(Token::Oper(TokenOperator::Plus)),
            '-' => tokens.push(Token::Oper(TokenOperator::Minus)),
            '*' => tokens.push(Token::Oper(TokenOperator::Times)),
            '/' => tokens.push(Token::Oper(TokenOperator::Divide)),
            '%' => tokens.push(Token::Oper(TokenOperator::Mod)),

            '<' => tokens.push(Token::Oper(TokenOperator::LessThan)),
            '>' => tokens.push(Token::Oper(TokenOperator::GreaterThan)),
            '=' => tokens.push(Token::Oper(TokenOperator::Equal)),

            '!' => tokens.push(Token::Oper(TokenOperator::Not)),
            '&' => { if chars.next() == Some('&') {tokens.push(Token::Oper(TokenOperator::And))} else {return Err("invalid character".to_string())} },

            // if it is ||, it is logical or. if it is |???|, it is abs.
            '|' => {
                match chars.peekable().peek() {
                    Some('|') => {
                        chars.next(); // pop the next character
                        tokens.push(Token::Oper(TokenOperator::Or))
                    },
                    _ => tokens.push(Token::Oper(TokenOperator::Abs))
                }
            },

            ' ' | '\n' => (), // ignore whitespace
            '0'..='9' =>tokens.push(Token::IntLiteral(scan_number(c, chars))),
            _ => { return Err("invalid character".to_string())}
        }
    }

    Ok(tokens)
}

pub fn scan_number(c: char, chars: &mut Chars) -> i32 {

    (c.to_string() + &chars.peeking_take_while(|c| c.is_digit(10)).collect::<String>()).parse::<i32>().unwrap()
}