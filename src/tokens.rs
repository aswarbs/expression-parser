use std::fmt;

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
    Or,
    And,
    Not,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,

    //Conditional,
    //Then,
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
    ABS,
    AND,
    OR,
    NOT,
    GTR,
    LSS,
    EQL,
}

impl fmt::Display for TAMInst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TAMInst::LOADL(value) => write!(f, "LOADL {}", value),
            TAMInst::ADD => write!(f, "ADD"),
            TAMInst::SUB => write!(f, "SUB"),
            TAMInst::MUL => write!(f, "MUL"),
            TAMInst::DIV => write!(f, "DIV"),
            TAMInst::MOD => write!(f, "MOD"),
            TAMInst::NEG => write!(f, "NEG"),
            TAMInst::ABS => write!(f, "ABS"),
            TAMInst::AND => write!(f, "ADD"),
            TAMInst::OR => write!(f, "OR"),
            TAMInst::NOT => write!(f, "NOT"),
            TAMInst::GTR => write!(f, "GTR"),
            TAMInst::LSS => write!(f, "LSS"),
            TAMInst::EQL => write!(f, "EQL"),
        }
    }
}