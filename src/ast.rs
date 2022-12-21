#[derive(Clone, Debug)]
pub enum Unit {
    Body { stmts: Vec<Stmt> },
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Let {
        name: String,
        expr: Box<Expr>,
    },
    Def {
        name: String,
        params: Option<Vec<String>>,
        stmts: Vec<Stmt>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    Expr(Box<Expr>),
    If {
        cond: Box<Expr>,
        cons: Vec<Stmt>,
        alt: Vec<Stmt>,
    },
    Return(Box<Expr>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Num(u64),
    Id(String),
    Text(String),
    BinOp {
        left: Box<Expr>,
        op: Opcode,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: Option<Vec<Box<Expr>>>,
    },
}

#[derive(Clone, Debug)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,

    EQ,
    NE,
    LT,
    LE,
    GT,
    GE,
}
