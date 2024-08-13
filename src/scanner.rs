use std::str::Chars;
use std::thread::current;
use itertools::Itertools;
// tokeniser
use super::tokens::Token;
use super::tokens::TokenOperator;


pub fn tokenize_all(input: String) -> Result <Vec<Token>, String> {

    if input.is_empty() {
        return Ok(Vec::new());
    }

    let (current_tokens, rest_of_input) = scan(input)?;


    let mut all_tokens = current_tokens;

    all_tokens.append(&mut tokenize_all(rest_of_input)?);

    Ok(all_tokens)
}

fn scan(input: String) -> Result<(Vec<Token>, String), String> {

    let c = input.chars().next().ok_or("Missing Char")?;

    match c {

        // SINGLE CHAR SYMBOLS

        '(' => Ok((vec!(Token::OpenPar), input)),
        ')' => Ok((vec!(Token::ClosedPar), input)),
        '+' => Ok((vec!(Token::Oper(TokenOperator::Plus)),input)),
        '-' => Ok((vec!(Token::Oper(TokenOperator::Minus)), input)),
        '*' => Ok((vec!(Token::Oper(TokenOperator::Times)), input)),
        '/' => Ok((vec!(Token::Oper(TokenOperator::Divide)), input)),
        '%' => Ok((vec!(Token::Oper(TokenOperator::Mod)), input)),

        /*'?' => tokens.push(Token::Oper(TokenOperator::Conditional)),
        ':' => tokens.push(Token::Oper(TokenOperator::Then)),*/

        ' ' | '\n' | '\r' => Ok((vec!(), input)), // ignore whitespace
        '0'..='9' => Ok((vec!(Token::IntLiteral((c.to_string() + &input.chars().peeking_take_while(|c| c.is_digit(10)).collect::<String>()).parse::<i32>().unwrap())), input)),

        // DOUBLE CHAR SYMBOLS

        '=' => { if input.chars().next() == Some('=') {Ok((vec!(Token::Oper(TokenOperator::Equal)), input))} else {return Err("invalid character".to_string())} },
        '&' => { if input.chars().next() == Some('&') {Ok((vec!(Token::Oper(TokenOperator::And)), input))} else {return Err("invalid character".to_string())} },


        // AMBIGUOUS SYMBOLS: LOOK AHEAD 1

        '!' => {
            match input.chars().peekable().peek() {

                Some('=') => {
                    input.chars().next();
                    Ok((vec!(Token::Oper(TokenOperator::NotEqual)), input))
                },
                _ => Ok((vec!(Token::Oper(TokenOperator::Not)), input))
            }
        }

        '|' => {
            match input.chars().peekable().peek() {
                Some('|') => {
                    input.chars().next();
                    Ok((vec!(Token::Oper(TokenOperator::Or)), input))
                },
                _ => Ok((vec!(Token::Oper(TokenOperator::Abs)), input))
            }
        },

        '<' => {
            match input.chars().peekable().peek() {
                Some('=') => {
                    input.chars().next();
                    Ok((vec!(Token::Oper(TokenOperator::LessThanOrEqual)), input))
                },
                _ => Ok((vec!(Token::Oper(TokenOperator::LessThan)), input))
            }
        },
        '>' => {
            match input.chars().peekable().peek() {
                Some('=') => {
                    input.chars().next();
                    Ok((vec!(Token::Oper(TokenOperator::GreaterThanOrEqual)), input))
                },
                _ => Ok((vec!(Token::Oper(TokenOperator::GreaterThan)), input))
            }
        }

        _ => { return Err("invalid character".to_string())}
    }
}