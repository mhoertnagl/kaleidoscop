use crate::ast::*;
use crate::rlvm::builder::Builder;
use crate::rlvm::context::Context;
// use crate::llvm::*;

use llvm_sys::prelude::LLVMValueRef;
use std::collections::HashMap;

type Statements = Vec<Statement>;

pub struct Codegen<'a> {
    symbols: HashMap<&'a str, LLVMValueRef>,
    context: Context, // context: Box<LLContext>,
                      // builder: Box<LLBuilder>,
                      // module: Box<LLModule<'a>>,
                      // function: Box<LLFunction<'a>>
}

impl<'a> Codegen<'a> {
    pub unsafe fn new() -> Self {
        let context = Context::get_global();
        let builder = Builder::new();
        // TODO: builder.new_module("main");
        let module = LLModule::new(&builder, "main");
        let function = LLFunction::empty(&builder, &module);
        Codegen {
            symbols: HashMap::new(),
            context,
            builder,
            module,
            function,
        }
    }

    pub unsafe fn emit(&'a self, module: &'a Module) {
        // self.module(module);
        self.module.write_to_file("main.bc");
    }
}

// impl Visitor for Codegen {
impl<'a> Codegen<'a> {
    // unsafe fn module(&'a mut self, m: &'a Module) {
    //     self.statements(&m.statements);
    // }

    // unsafe fn statements(&'a mut self, s: &'a Statements) {
    //     for t in s.iter() {
    //         self.statement(t)
    //     }
    // }

    // unsafe fn statement(&'a mut self, s: &'a Statement) {
    //     match s {
    //         Statement::Let(x) => self.let_statement(x),
    //         Statement::Def(x) => self.def_statement(x),
    //         Statement::Assign(x) => self.assign_statement(x),
    //         Statement::Expr(x) => self.expr_statement(x),
    //         Statement::If(x) => self.if_statement(x),
    //         Statement::Return(x) => self.return_statement(x),
    //     }
    // }

    // unsafe fn let_statement(&mut self, s: &'a LetStatement) {
    //     let typ_int64 = self.context.int64();
    //     let val = self.expr(&s.expr);
    //     let loc = self.builder.alloca(s.name, typ_int64);
    //     self.builder.store(val, loc);
    //     // TODO: Check for redefinition.
    //     self.symbols.insert(&s.name, loc);
    // }

    // unsafe fn def_statement(&'a mut self, s: &'a DefStatement) {
    //     let typ_int64 = self.context.int64();
    //     self.function = self.module.new_function(s.name, &mut [], typ_int64);
    //     let bb = self.function.append_basic_block("entry");
    //     bb.position_at_end();
    //     self.statements(&s.statements);
    // }

    // unsafe fn assign_statement(&mut self, s: &'a AssignStatement) {
    //     let val = self.expr(&s.expr);
    //     let loc = *self.symbols.get(s.name.as_str()).unwrap();
    //     self.builder.store(val, loc);
    // }

    // // TODO: Actually we only need a function call statement.
    // unsafe fn expr_statement(&'a mut self, _s: &'a ExprStatement) {}

    // unsafe fn if_statement(&'a mut self, s: &'a IfStatement) {
    //     let zero_int64 = self.context.zero64();
    //     let cond = self.expr(&s.cond);
    //     let nonzero_cond = self.builder.cmp_ne(cond, zero_int64);
    //     let then_block = self.function.append_basic_block("ifcond");
    //     let els_block = self.function.append_basic_block("ifalt");
    //     let merge_block = self.function.append_basic_block("ifmerge");
    //     self.builder.cond_br(nonzero_cond, then_block, els_block);
    //     then_block.position_at_end();
    //     self.statements(&s.cons);
    //     self.builder.br(merge_block);
    //     els_block.position_at_end();
    //     self.statements(&s.alt);
    //     self.builder.br(merge_block);
    //     merge_block.position_at_end();
    // }

    // unsafe fn return_statement(&mut self, s: &'a ReturnStatement) {
    //     let val = self.expr(&s.expr);
    //     self.builder.ret(val);
    // }

    // unsafe fn expr(&mut self, e: &'a Expr) -> LLVMValueRef {
    //     match e {
    //         Expr::Atom(x) => self.atom(x),
    //         Expr::BinOp(x) => self.binop_expr(x),
    //         //Expr::FunCall(x) => self.funcall_expr(x),
    //     }
    // }

    // unsafe fn atom(&mut self, a: &'a Atom) -> LLVMValueRef {
    //     match a {
    //         Atom::Num(x) => self.num(*x),
    //         Atom::Id(x) => self.id(x),
    //     }
    // }

    // unsafe fn binop_expr(&mut self, b: &'a BinOpExpr) -> LLVMValueRef {
    //     let left = self.expr(&b.left);
    //     let right = self.expr(&b.right);
    //     match b.op {
    //         Opcode::Add => self.builder.add(left, right),
    //         Opcode::Div => self.builder.div(left, right),
    //         Opcode::Mul => self.builder.mul(left, right),
    //         Opcode::Sub => self.builder.sub(left, right),
    //     }
    // }

    // //unsafe fn funcall_expr(&mut self, _f: &'a FunCallExpr) {}

    // unsafe fn num(&mut self, n: u64) -> LLVMValueRef {
    //     self.context.const64(n)
    // }

    // unsafe fn id(&mut self, id: &'a str) -> LLVMValueRef {
    //     // TODO: Check for existence.
    //     let loc = *self.symbols.get(id).unwrap();
    //     self.builder.load(id, loc)
    // }
}
