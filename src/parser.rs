use std::cmp::Ordering::Less;
use crate::tokens::{Expr, TokenOperator};
use crate::tokens::Expr::UnOp;
use crate::tokens::TokenOperator::{Abs, And, Divide, Equal, GreaterThan, LessThan, Minus, Mod, Negation, Not, Or, Plus, Times};
use super::tokens::Token;


pub fn parse_exp(tokens: &mut Vec<Token>) -> Result<Expr,String> {
    // exp ::= mexp | mexp + exp | mexp - exp | mexp % exp | mexp || exp

    // | mexp > exp | mexp < exp | mexp = exp

    let node = parse_mexp(tokens)?;

    match tokens.first() {
        Some(Token::Oper(Plus)) => {
            tokens.remove(0); // pop +
            Ok(Expr::BinOp(Plus, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        Some(Token::Oper(Minus)) => {
            tokens.remove(0); // pop -
            Ok(Expr::BinOp(Minus, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        Some(Token::Oper(Mod)) => {
            tokens.remove(0); // pop %
            Ok(Expr::BinOp(Mod, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        Some(Token::Oper(Or)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(Or, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        Some(Token::Oper(LessThan)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(LessThan, Box::from(node), Box::from(parse_exp(tokens)?)))
        }
        Some(Token::Oper(GreaterThan)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(GreaterThan, Box::from(node), Box::from(parse_exp(tokens)?)))
        }
        Some(Token::Oper(Equal)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(Equal, Box::from(node), Box::from(parse_exp(tokens)?)))
        }
        _ => Ok(node)
    }


}

fn parse_mexp(tokens:&mut Vec<Token>) -> Result<Expr,String> {
    // mexp ::= term | term * mexp | term / mexp | term && mexp

    let node = parse_term(tokens)?; // always starts with a term

    match tokens.first() {
        Some(Token::Oper(Times)) => {
            tokens.remove(0); // pop *
            Ok(Expr::BinOp(Times, Box::from(node), Box::from(parse_mexp(tokens)?)))
        },
        Some(Token::Oper(Divide)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(Divide, Box::from(node), Box::from(parse_mexp(tokens)?)))
        },
        Some(Token::Oper(And)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(And, Box::from(node), Box::from(parse_mexp(tokens)?)))
        },
        _ => Ok(node)
    }

}

fn parse_term(tokens:&mut Vec<Token>) -> Result<Expr, String> {
    // term ::= int | - term | ( exp ) | |exp| | !exp

    match tokens.first() {
        Some(Token::IntLiteral(_)) => { if let Token::IntLiteral(i) = tokens.remove(0) {Ok(Expr::LitInteger(i))} else {Err("expecting integer".to_string())} },

        Some(Token::Oper(Minus)) => {
                tokens.remove(0); // pop the terminal (-)
                Ok(UnOp(Negation, Box::from(parse_term(tokens)?)))
            },
        Some(Token::OpenPar) => {
                tokens.remove(0); // pop the terminal (

                let expr = parse_exp(tokens)?;

                if let Token::ClosedPar = tokens.remove(0) { Ok(expr) } else {Err("expecting )".to_string())}
            },
        Some(Token::Oper(Abs)) => {
                tokens.remove(0); // pop the terminal abs

                let expr = parse_exp(tokens)?;

                if let Token::Oper(Abs) = tokens.remove(0) { Ok(UnOp(Abs, Box::from(expr))) } else {Err("expecting |".to_string())}
            },
        Some(Token::Oper(Not)) => {
                tokens.remove(0); // pop the terminal !
                Ok(UnOp(Not, Box::from(parse_exp(tokens)?)))
            },

        _ => Err("invalid token in term".to_string())

    }
}



