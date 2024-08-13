use crate::tokens::{Expr, TokenOperator, TAMInst};
use crate::tokens::TAMInst::{ABS, ADD, AND, DIV, EQL, GTR, LSS, MOD, MUL, NEG, NOT, OR, SUB};

pub fn ast_to_tam(ast: Expr) ->  Vec<TAMInst> {
    match ast {
        Expr::LitInteger(i) => { vec![TAMInst::LOADL(i)] },
        Expr::BinOp(operator, left, right) => ast_to_tam(*left).into_iter().chain(ast_to_tam(*right)).chain(std::iter::once(op_to_taminst(operator)).flatten()).collect(),
        Expr::UnOp(operator, value) => ast_to_tam(*value).into_iter().chain(std::iter::once(op_to_taminst(operator)).flatten()).collect(),
    }
}

fn op_to_taminst(operator: TokenOperator) -> Vec<TAMInst> {
    match operator {
        TokenOperator::Plus => {vec![ADD]}
        TokenOperator::Minus => {vec![SUB]}
        TokenOperator::Times => {vec![MUL]}
        TokenOperator::Divide => {vec![DIV]}
        TokenOperator::Mod => {vec![MOD]}
        TokenOperator::Abs => {vec![ABS]}
        TokenOperator::Negation => {vec![NEG]}
        TokenOperator::Or => {vec![OR]}
        TokenOperator::And => {vec![AND]}
        TokenOperator::Not => {vec![NOT]}
        TokenOperator::LessThan => {vec![LSS]}
        TokenOperator::GreaterThan => {vec![GTR]}
        TokenOperator::Equal => {vec![EQL]}
        TokenOperator::LessThanOrEqual => {vec![GTR, NOT]}
        TokenOperator::GreaterThanOrEqual => {vec![LSS, NOT]}
        TokenOperator::NotEqual => {vec![EQL, NOT]}
    }
}