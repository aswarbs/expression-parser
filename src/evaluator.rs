use crate::tokens::Expr;
use crate::tokens::TokenOperator::{Minus, Plus, Times, Divide, Mod, Negation, Abs};

pub fn direct_evaluate(expr: Expr) -> Result<i32, String> {

    match expr {

        Expr::BinOp(operator, left, right) => {
            match operator {
                Plus=> {Ok(direct_evaluate(*left)? + direct_evaluate(*right)?)},
                Minus => {Ok(direct_evaluate(*left)? - direct_evaluate(*right)?)},
                Times => {Ok(direct_evaluate(*left)? * direct_evaluate(*right)?)},
                Divide => {Ok(direct_evaluate(*left)? / direct_evaluate(*right)?)},
                Mod => {Ok(direct_evaluate(*left)? % direct_evaluate(*right)?)},
                _ => Err("invalid operator".to_string())
            }
        },

        Expr::UnOp(operator, value) => {

            match operator {
                Negation => {Ok(-direct_evaluate(*value)?)},
                Abs => {Ok(direct_evaluate(*value)?.abs())}
                _ => Err("invalid unary operator".to_string())
            }

        },

        Expr::LitInteger(value)=> {
            Ok(value)
        },

    }

}

