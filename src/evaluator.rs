use crate::tokens::{Expr, Operator, UnOperator};

pub fn evaluate(expr: Expr) -> Result<i32, String> {

    match expr {

        Expr::BinOp(operator, left, right) => {
            match operator {
                Operator::Plus => {Ok(evaluate(*left)? + evaluate(*right)?)},
                Operator::Minus => {Ok(evaluate(*left)? - evaluate(*right)?)},
                Operator::Times => {Ok(evaluate(*left)? * evaluate(*right)?)},
                Operator::Divide => {Ok(evaluate(*left)? / evaluate(*right)?)},
                Operator::Mod => {Ok(evaluate(*left)? % evaluate(*right)?)},
                _ => Err("invalid operator".to_string())
            }
        },

        Expr::UnOp(operator, value) => {

            match operator {
                UnOperator::Negation => {Ok(-evaluate(*value)?)},
                UnOperator::Abs => {Ok(evaluate(*value)?.abs())}
            }

        },

        Expr::LitInteger(value)=> {
            Ok(value)
        },

    }

}