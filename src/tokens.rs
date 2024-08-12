#[derive(Debug)]
pub enum Token {
    IntLiteral(i32),
    Oper(Operator),
    OpenPar,
    ClosedPar
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    Abs,
}


#[derive(Debug)]
pub enum UnOperator {
    Negation,
    Abs
}

#[derive(Debug)]
pub enum Expr {
    LitInteger(i32),
    BinOp(Operator, Box<Expr>, Box<Expr>),
    UnOp(UnOperator, Box<Expr>)
}
