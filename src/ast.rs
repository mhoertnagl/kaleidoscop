#[derive(Clone, Debug)]
pub enum Unit {
    Body { stmts: Vec<Stmt> },
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    BinOp {
        left: Box<Expr>,
        op: Opcode,
        right: Box<Expr>,
    },
    Neg(Box<Expr>),
    Id(String),
}

#[derive(Clone, Debug)]
pub enum Opcode {
    Impl,
    BiImpl,
    Or,
    Nor,
    NotXor,
    Xor,
    And,
    Nand,
}
