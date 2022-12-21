use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, IntMathValue, IntValue, PointerValue};
use inkwell::IntPredicate;

use crate::ast::Expr;
use crate::ast::Opcode;
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

    fn expr_stmt(&mut self, expr: &Expr) {
        self.expr(expr);
    }

    fn if_stmt(&mut self, cond: &Expr, cons: &Vec<Stmt>, alt: &Vec<Stmt>) {
        let cond = self.expr(cond);
        let parent = self.fn_value_opt.unwrap();
        let then_bb = self.context.append_basic_block(parent, "then");
        let else_bb = self.context.append_basic_block(parent, "else");
        let cont_bb = self.context.append_basic_block(parent, "ifcont");

        self.builder
            .build_conditional_branch(cond.into_int_value(), then_bb, else_bb);
        // https://github.com/TheDan64/inkwell/blob/master/examples/kaleidoscope/main.rs
    }

    fn return_stmt(&mut self, expr: &Expr) {
        let val = self.expr(expr);
        self.builder.build_return(Some(&val));
    }

    fn expr(&self, expr: &Expr) -> BasicValueEnum {
        match expr {
            Expr::Num(num) => self.num_expr(*num),
            Expr::Id(id) => self.id_expr(id),
            Expr::Text(text) => todo!(),
            Expr::BinOp { left, op, right } => self.binop_expr(left, op, right),
            Expr::Call { name, args } => todo!(),
        }
    }

    fn num_expr(&self, num: u64) -> BasicValueEnum {
        let val = self.context.i64_type().const_int(num, false);
        BasicValueEnum::from(val)
    }

    fn id_expr(&self, id: &String) -> BasicValueEnum {
        match self.variables.get(id) {
            Some(ptr) => self.builder.build_load(*ptr, id),
            None => todo!(),
        }
    }

    fn binop_expr(&self, left: &Expr, op: &Opcode, right: &Expr) -> BasicValueEnum {
        let lhs = self.expr(left).into_int_value();
        let rhs = self.expr(right).into_int_value();
        let val = match op {
            Opcode::Mul => self.builder.build_int_mul(lhs, rhs, "tmul"),
            Opcode::Div => self.builder.build_int_unsigned_div(lhs, rhs, "tudiv"),
            Opcode::Add => self.builder.build_int_add(lhs, rhs, "tadd"),
            Opcode::Sub => self.builder.build_int_sub(lhs, rhs, "tsub"),
            Opcode::EQ => self
                .builder
                .build_int_compare(IntPredicate::EQ, lhs, rhs, "teq"),
            Opcode::NE => self
                .builder
                .build_int_compare(IntPredicate::NE, lhs, rhs, "tne"),
            Opcode::LT => self
                .builder
                .build_int_compare(IntPredicate::ULT, lhs, rhs, "tult"),
            Opcode::LE => self
                .builder
                .build_int_compare(IntPredicate::ULE, lhs, rhs, "tule"),
            Opcode::GT => self
                .builder
                .build_int_compare(IntPredicate::UGT, lhs, rhs, "tugt"),
            Opcode::GE => self
                .builder
                .build_int_compare(IntPredicate::UGE, lhs, rhs, "tuge"),
        };
        BasicValueEnum::from(val)
    }
}
