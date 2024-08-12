#[derive(Debug)]
pub enum Token {
    IntLiteral(i32),
    Oper(TokenOperator),
    OpenPar,
    ClosedPar
}

#[derive(Debug)]
pub enum TokenOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Abs,
    Negation,
}

#[derive(Debug)]
pub enum Expr {
    LitInteger(i32),
    BinOp(TokenOperator, Box<Expr>, Box<Expr>),
    UnOp(TokenOperator, Box<Expr>)
}

#[derive(Debug)]
pub enum TAMInst {
    LOADL(i32),
    ADD,
    SUB,
    MUL,
    DIV,
    NEG,
    MOD,
    ABS
}