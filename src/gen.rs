use std::collections::HashMap;
use std::path::Path;

use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValueEnum, FunctionValue, IntMathValue, IntValue, PointerValue};
use inkwell::IntPredicate;

use crate::ast::Expr;
use crate::ast::Opcode;
use crate::ast::Stmt;
use crate::ast::Unit;
use crate::ext::{BasicBlockExt, FunctionValueExt};

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
        let path = Path::new("main.bc");
        match module {
            Unit::Body { stmts } => {
                self.stmts(stmts);
                match self.module.verify() {
                    Ok(_) => {
                        self.module.print_to_file("main.ll").unwrap();
                        self.module.write_bitcode_to_path(path);
                    }
                    Err(msg) => {
                        println!("{:?}", msg);
                        self.module.print_to_stderr();
                    }
                }
            }
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

    fn assign_stmt(&mut self, name: &String, expr: &Expr) {
        let val = self.expr(expr);
        match self.variables.get(name) {
            Some(ptr) => self.builder.build_store(*ptr, val),
            None => todo!(),
        };
    }

    fn def_stmt(&mut self, name: &String, params: &Option<Vec<String>>, stmts: &Vec<Stmt>) {
        let zero = self.context.i64_type().const_int(0, false);
        let ret_type = self.context.i64_type();
        let fun_type = ret_type.fn_type(&[], false);
        let fun = self.module.add_function(name, fun_type, None);
        let entry_bb = self.context.append_basic_block(fun, "entry");
        self.fn_value_opt = Some(fun);
        self.builder.position_at_end(entry_bb);
        self.stmts(stmts);
        // Test if the last basic block in the function ends with a termination
        // instruction. If it does not, return 0.
        // The function contains at least the entry block.
        if fun.has_no_terminator() {
            self.builder.build_return(Some(&zero));
        }
    }

    fn let_stmt(&mut self, name: &String, expr: &Expr) {
        let val = self.expr(expr);
        let ptr = self.builder.build_alloca(self.context.i64_type(), name);
        self.builder.build_store(ptr, val);
        self.variables.insert(name.to_string(), ptr);
    }

    fn expr_stmt(&mut self, expr: &Expr) {
        self.expr(expr);
    }

    fn if_stmt(&mut self, cond: &Expr, cons: &Vec<Stmt>, alt: &Option<Vec<Stmt>>) {
        // let zero = self.context.i64_type().const_int(0, false);

        // Append three new basic blocks to the current function.
        let fun = self.fn_value_opt.unwrap();
        let then_bb = self.context.append_basic_block(fun, "if.then");

        // TODO: Handle missing else.
        // TODO: Handle optional elsif.

        let else_bb = self.context.append_basic_block(fun, "if.else");
        let cont_bb = self.context.append_basic_block(fun, "if.cont");
        // Evaluate the condition.
        let cond = self.expr(cond).into_int_value();
        self.builder
            .build_conditional_branch(cond, then_bb, else_bb);

        // Generate the then-branch.
        self.builder.position_at_end(then_bb);
        self.stmts(cons);
        // Add an unconditional branch instruction to then_bb if it does
        // not end with a termination instruction already.
        if then_bb.has_no_terminator() {
            self.builder.build_unconditional_branch(cont_bb);
        }

        // Generate the optional else-branch.
        if let Some(alt) = alt {
            self.builder.position_at_end(else_bb);
            self.stmts(alt);
            // Add an unconditional branch instruction to else_bb if it does
            // not end with a termination instruction already.
            if else_bb.has_no_terminator() {
                self.builder.build_unconditional_branch(cont_bb);
            }
        }

        self.builder.position_at_end(cont_bb);
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
