use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Module {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", fmt_statements(&self.statements))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Def(DefStatement),
    Assign(AssignStatement),
    Expr(ExprStatement),
    If(IfStatement),
    Return(ReturnStatement),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let(e) => e.fmt(f),
            Statement::Def(e) => e.fmt(f),
            Statement::Assign(e) => e.fmt(f),
            Statement::Expr(e) => e.fmt(f),
            Statement::If(e) => e.fmt(f),
            Statement::Return(e) => e.fmt(f),
        }
    }
}

fn fmt_statements(s: &Vec<Statement>) -> String {
    let ss: Vec<String> = s.iter().map(|s| s.to_string()).collect();
    return ss.join("\n");
    // s.iter()
    //     .fold(String::new(), |acc, s| acc + &s.to_string() + "\n")
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetStatement {
    pub name: String,
    pub expr: Box<Expr>,
}

impl LetStatement {
    pub fn new(name: String, expr: Box<Expr>) -> Self {
        LetStatement { name, expr }
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {} = {};", self.name, self.expr)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DefStatement {
    pub name: String,
    pub params: Vec<String>,
    pub statements: Vec<Statement>,
}

impl DefStatement {
    pub fn new(
        name: String,
        params: Option<Vec<String>>,
        statements: Vec<Statement>,
    ) -> DefStatement {
        DefStatement {
            name,
            params: params.unwrap_or(vec![]),
            statements,
        }
    }
}

impl fmt::Display for DefStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ps = self.params.join(", ");
        let ss = fmt_statements(&self.statements);
        write!(f, "def {}({})\n{}\nend", self.name, ps, ss)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AssignStatement {
    pub name: String,
    pub expr: Box<Expr>,
}

impl AssignStatement {
    pub fn new(name: String, expr: Box<Expr>) -> Self {
        AssignStatement { name, expr }
    }
}

impl fmt::Display for AssignStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {};", self.name, self.expr)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStatement {
    pub expr: Box<Expr>,
}

impl ExprStatement {
    pub fn new(expr: Box<Expr>) -> Self {
        ExprStatement { expr: expr }
    }
}

impl fmt::Display for ExprStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};", self.expr)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub cond: Box<Expr>,
    pub cons: Vec<Statement>,
    pub alt: Vec<Statement>,
}

impl IfStatement {
    pub fn new(cond: Box<Expr>, cons: Vec<Statement>, alt: Vec<Statement>) -> Self {
        IfStatement { cond, cons, alt }
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cons = fmt_statements(&self.cons);
        let alts = fmt_statements(&self.alt);
        write!(f, "if {} then\n{}\nelse\n{}\nend", self.cond, cons, alts)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnStatement {
    pub expr: Box<Expr>,
}

impl ReturnStatement {
    pub fn new(expr: Box<Expr>) -> Self {
        ReturnStatement { expr }
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {};", self.expr)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Atom(Atom),
    //FunCall(FunCallExpr),
    BinOp(BinOpExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(e) => e.fmt(f),
            //Expr::FunCall(e) => e.fmt(f),
            Expr::BinOp(e) => e.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Atom {
    Num(u64),
    //Num(f64),
    Id(String),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Num(n) => n.fmt(f),
            Atom::Id(id) => id.fmt(f),
        }
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct FunCallExpr {
//     pub name: String,
//     pub args: Vec<Box<Expr>>,
// }

// impl FunCallExpr {
//     pub fn new(name: String, args: Option<Vec<Box<Expr>>>) -> Self {
//         FunCallExpr {
//             name,
//             args: args.unwrap_or(vec![]),
//         }
//     }
// }

// impl fmt::Display for FunCallExpr {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let px: Vec<String> = self.args.iter().map(|a| a.to_string()).collect();
//         let ps = px.join(", ");
//         write!(f, "{}({})", self.name, ps)
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct BinOpExpr {
    pub left: Box<Expr>,
    pub op: Opcode,
    pub right: Box<Expr>,
}

impl BinOpExpr {
    pub fn new(left: Box<Expr>, op: Opcode, right: Box<Expr>) -> Self {
        BinOpExpr { left, op, right }
    }
}

impl fmt::Display for BinOpExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.op, self.right)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::Mul => "*".fmt(f),
            Opcode::Div => "/".fmt(f),
            Opcode::Add => "+".fmt(f),
            Opcode::Sub => "-".fmt(f),
        }
    }
}
