use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};

use ast::Stmt;
use ast::Unit;

use crate::ast;

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
            Stmt::Assign { name, expr } => todo!(),
            Stmt::Def {
                name,
                params,
                stmts,
            } => todo!(),
            Stmt::Let { name, expr } => todo!(),
            Stmt::Expr(_) => todo!(),
            Stmt::If { cond, cons, alt } => todo!(),
            Stmt::Return(_) => todo!(),
        }
    }
}
