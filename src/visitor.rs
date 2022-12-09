// use crate::ast::*;

// pub trait Visitor {
//     unsafe fn module(&mut self, m: &Module) {
//         for s in m.statements.iter() {
//             self.statement(s)
//         }
//     }

//     unsafe fn statement(&mut self, s: &Statement) {
//         match s {
//             Statement::Let(x) => self.let_statement(x),
//             Statement::Def(x) => self.def_statement(x),
//             Statement::Assign(x) => self.assign_statement(x),
//             Statement::Expr(x) => self.expr_statement(x),
//             Statement::Return(x) => self.return_statement(x),
//         }
//     }

//     unsafe fn let_statement(&mut self, _s: &LetStatement) {}

//     unsafe fn def_statement(&mut self, _s: &DefStatement) {}

//     unsafe fn assign_statement(&mut self, _s: &AssignStatement) {}

//     unsafe fn expr_statement(&mut self, _s: &ExprStatement) {}

//     unsafe fn return_statement(&mut self, _s: &ReturnStatement) {}

//     unsafe fn expr(&mut self, e: &Expr) {
//         match e {
//             Expr::Atom(x) => self.atom(x),
//             Expr::BinOp(x) => self.binop_expr(x),
//             Expr::FunCall(x) => self.funcall_expr(x),
//         }
//     }

//     unsafe fn binop_expr(&mut self, _b: &BinOpExpr) {}

//     unsafe fn funcall_expr(&mut self, _f: &FunCallExpr) {}

//     unsafe fn atom(&mut self, a: &Atom) {
//         match a {
//             Atom::Num(x) => self.num(*x),
//             Atom::Id(x) => self.id(x),
//         }
//     }

//     unsafe fn num(&mut self, _n: f64) {}

//     unsafe fn id(&mut self, _id: &String) {}
// }
