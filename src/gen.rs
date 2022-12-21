use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};

use crate::ast::Expr;
use crate::ast::Stmt;
use crate::ast::Unit;

pub struct Gen<'ctx> {
    builder: Builder<'ctx>,
    context: &'ctx Context,
    module: Module<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'ctx> Gen<'ctx> {
    pub fn new(builder: Builder<'ctx>, context: &'ctx Context, module: Module<'ctx>) -> Gen<'ctx> {
        Gen {
            builder,
            context,
            module,
            variables: HashMap::new(),
            fn_value_opt: None,
        }
    }

    pub fn compile(&mut self, module: &Unit) {
        match module {
            Unit::Body { ref stmts } => self.stmts(stmts),
        }
    }

    fn stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.stmt(stmt);
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Assign { name, expr } => self.assign_stmt(name, expr),
            Stmt::Def {
                name,
                params,
                stmts,
            } => self.def_stmt(name, params, stmts),
            Stmt::Let { name, expr } => self.let_stmt(name, expr),
            Stmt::Expr(expr) => self.expr_stmt(expr),
            Stmt::If { cond, cons, alt } => self.if_stmt(cond, cons, alt),
            Stmt::Return(expr) => self.return_stmt(expr),
        }
    }

    fn assign_stmt(&mut self, name: &String, expr: &Expr) {}

    fn def_stmt(&mut self, name: &String, params: &Option<Vec<String>>, stmts: &Vec<Stmt>) {}

    fn let_stmt(&mut self, name: &String, expr: &Expr) {}

    fn expr_stmt(&mut self, expr: &Expr) {}

    fn if_stmt(&mut self, cond: &Expr, cons: &Vec<Stmt>, alt: &Vec<Stmt>) {}

    fn return_stmt(&mut self, expr: &Expr) {}
}
