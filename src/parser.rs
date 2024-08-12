use itertools::PeekingNext;
use crate::tokens::Expr;
use crate::tokens::Expr::UnOp;
use crate::tokens::Operator::Plus;
use super::tokens::{Token, Operator, UnOperator};


pub fn parse_exp(tokens: &mut Vec<Token>) -> Result<Expr,String> {
    // exp ::= mexp | mexp + exp | mexp - exp

    let node = parse_mexp(tokens)?;

    match tokens.first() {
        Some(Token::Oper(Operator::Plus)) => {
            tokens.remove(0); // pop +
            Ok(Expr::BinOp(Operator::Plus, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        Some(Token::Oper(Operator::Minus)) => {
            tokens.remove(0); // pop -
            Ok(Expr::BinOp(Operator::Minus, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        Some(Token::Oper(Operator::Mod)) => {
            tokens.remove(0); // pop %
            Ok(Expr::BinOp(Operator::Mod, Box::from(node), Box::from(parse_exp(tokens)?)))
        },
        _ => Ok(node)
    }


}

fn parse_mexp(tokens:&mut Vec<Token>) -> Result<Expr,String> {
    // mexp ::= term | term * mexp | term / mexp

    let node = parse_term(tokens)?; // always starts with a term

    match tokens.first() {
        Some(Token::Oper(Operator::Times)) => {
            tokens.remove(0); // pop *
            Ok(Expr::BinOp(Operator::Times, Box::from(node), Box::from(parse_mexp(tokens)?)))
        },
        Some(Token::Oper(Operator::Divide)) => {
            tokens.remove(0); // pop /
            Ok(Expr::BinOp(Operator::Divide, Box::from(node), Box::from(parse_mexp(tokens)?)))
        },
        _ => Ok(node)
    }

}

fn parse_term(tokens:&mut Vec<Token>) -> Result<Expr, String> {
    // term ::= int | - term | ( exp )

    match tokens.first() {
        Some(Token::IntLiteral(_)) => { if let (Token::IntLiteral(i)) = tokens.remove(0) {Ok(Expr::LitInteger(i))} else {Err("expecting integer".to_string())} },

        Some(Token::Oper(Operator::Minus)) =>
            {
                tokens.remove(0); // pop the terminal (-)
                Ok(Expr::UnOp(UnOperator::Negation, Box::from(parse_exp(tokens)?)))
            },

        Some(Token::OpenPar) =>
            {
                tokens.remove(0); // pop the terminal (

                let expr = parse_exp(tokens)?;

                if let Token::ClosedPar = tokens.remove(0) { Ok(expr) } else {Err("expecting )".to_string())}
            },

        Some(Token::Oper(Operator::Abs)) =>
            {
                tokens.remove(0); // pop the terminal abs

                let expr = parse_exp(tokens)?;

                if let Token::Oper(Operator::Abs) = tokens.remove(0) { Ok(UnOp(UnOperator::Abs, Box::from(expr))) } else {Err("expecting |".to_string())}
            },

        _ => Err("invalid token in term".to_string())

    }
}



