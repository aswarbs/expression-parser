use crate::tokens::{Expr, TokenOperator, TAMInst};
use crate::tokens::TAMInst::{ABS, ADD, AND, DIV, EQL, GTR, LSS, MOD, MUL, NEG, NOT, OR, SUB};

pub fn ast_to_tam(ast: Expr) ->  Vec<TAMInst> {
    match ast {
        Expr::LitInteger(i) => { vec![TAMInst::LOADL(i)] },
        Expr::BinOp(operator, left, right) => ast_to_tam(*left).into_iter().chain(ast_to_tam(*right)).chain(std::iter::once(op_to_taminst(operator))).collect(),
        Expr::UnOp(operator, value) => ast_to_tam(*value).into_iter().chain(std::iter::once(op_to_taminst(operator))).collect(),
    }
}

fn op_to_taminst(operator: TokenOperator) -> TAMInst {
    match operator {
        TokenOperator::Plus => {ADD}
        TokenOperator::Minus => {SUB}
        TokenOperator::Times => {MUL}
        TokenOperator::Divide => {DIV}
        TokenOperator::Mod => {MOD}
        TokenOperator::Abs => {ABS}
        TokenOperator::Negation => {NEG}
        TokenOperator::Or => {OR}
        TokenOperator::And => {AND}
        TokenOperator::Not => {NOT}
        TokenOperator::LessThan => {LSS}
        TokenOperator::GreaterThan => {GTR}
        TokenOperator::Equal => {EQL}
    }
}