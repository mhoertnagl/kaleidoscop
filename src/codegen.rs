use crate::ast::*;
use crate::rlvm::builder::*;
use crate::rlvm::context::*;
use crate::rlvm::function::*;
use crate::rlvm::module::*;
// use crate::llvm::*;
use llvm_sys::prelude::LLVMValueRef;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::*;
use std::ptr;
use std::rc::Rc;

// macro_rules! c_str {
//     ($s:expr) => {
//         CString::new($s).unwrap().as_ptr() as *const i8
//     };
// }

type Statements = Vec<Statement>;

// sowas

pub struct Codegen {
    symbols: HashMap<Rc<String>, LLVMValueRef>,
    context: Rc<LLContext>,
    builder: Rc<LLBuilder>,
    module: Rc<LLModule>,
    function: Rc<LLFunction>,
}

impl Codegen {
    pub unsafe fn new() -> Self {
        let context = LLContext::new();
        let builder = context.new_builder();
        // TODO: builder.new_module("main");
        let module = LLModule::new(builder.clone(), "main");
        let function = LLFunction::empty(builder.clone(), module.clone());
        Codegen {
            symbols: HashMap::new(),
            context,
            builder,
            module,
            function,
        }
    }

    pub unsafe fn emit(&mut self, module: &Module) {
        self.module(module);
        self.module.write_to_file("main.bc");
    }
}

// impl Visitor for Codegen {
impl Codegen {
    unsafe fn module(&mut self, m: &Module) {
        self.statements(&m.statements);
    }

    unsafe fn statements(&mut self, s: &Statements) {
        for t in s.iter() {
            self.statement(t)
        }
    }

    unsafe fn statement(&mut self, s: &Statement) {
        match s {
            Statement::Let(x) => self.let_statement(x),
            Statement::Def(x) => self.def_statement(x),
            Statement::Assign(x) => self.assign_statement(x),
            Statement::Expr(x) => self.expr_statement(x),
            Statement::If(x) => self.if_statement(x),
            Statement::Return(x) => self.return_statement(x),
        }
    }

    unsafe fn let_statement(&mut self, s: &LetStatement) {
        let typ_int64 = self.context.int64();
        let val = self.expr(&s.expr);
        let loc = self.builder.alloca(s.name.clone(), typ_int64);
        self.builder.store(val, loc);
        // TODO: Check for redefinition.
        self.symbols.insert(Rc::new(s.name.clone()), loc);
    }

    unsafe fn def_statement(&mut self, s: &DefStatement) {
        let typ_int64 = self.context.int64();
        self.function = self.module.new_function(s.name.clone(), &mut [], typ_int64);
        let bb = self.function.append_basic_block("entry");
        bb.position_at_end();
        self.statements(&s.statements);
    }

    unsafe fn assign_statement(&mut self, s: &AssignStatement) {
        let val = self.expr(&s.expr);
        let loc = *self.symbols.get(&s.name).unwrap();
        self.builder.store(val, loc);
    }

    // TODO: Actually we only need a function call statement.
    unsafe fn expr_statement(&mut self, _s: &ExprStatement) {}

    unsafe fn if_statement(&mut self, s: &IfStatement) {
        let zero_int64 = self.context.zero64();
        let cond = self.expr(&s.cond);
        let nonzero_cond = self.builder.cmp_ne(cond, zero_int64);
        let then_block = self.function.append_basic_block("ifcond");
        let els_block = self.function.append_basic_block("ifalt");
        let merge_block = self.function.append_basic_block("ifmerge");
        self.builder
            .cond_br(nonzero_cond, then_block.clone(), els_block.clone());
        then_block.position_at_end();
        self.statements(&s.cons);
        self.builder.br(merge_block.clone());
        els_block.position_at_end();
        self.statements(&s.alt);
        self.builder.br(merge_block.clone());
        merge_block.position_at_end();
    }

    unsafe fn return_statement(&mut self, s: &ReturnStatement) {
        let val = self.expr(&s.expr);
        self.builder.ret(val);
    }

    unsafe fn expr(&mut self, e: &Expr) -> LLVMValueRef {
        match e {
            Expr::Atom(x) => self.atom(x),
            Expr::BinOp(x) => self.binop_expr(x),
            //Expr::FunCall(x) => self.funcall_expr(x),
        }
    }

    unsafe fn atom(&mut self, a: &Atom) -> LLVMValueRef {
        match a {
            Atom::Num(x) => self.num(*x),
            Atom::Id(x) => self.id(x),
        }
    }

    unsafe fn binop_expr(&mut self, b: &BinOpExpr) -> LLVMValueRef {
        let left = self.expr(&b.left);
        let right = self.expr(&b.right);
        match b.op {
            Opcode::Add => self.builder.add(left, right),
            Opcode::Div => self.builder.div(left, right),
            Opcode::Mul => self.builder.mul(left, right),
            Opcode::Sub => self.builder.sub(left, right),
        }
    }

    //unsafe fn funcall_expr(&mut self, _f: &FunCallExpr) {}

    unsafe fn num(&mut self, n: u64) -> LLVMValueRef {
        self.context.const64(n)
    }

    unsafe fn id(&mut self, id: &str) -> LLVMValueRef {
        // TODO: Check for existence.
        let loc = *self.symbols.get(&String::from(id)).unwrap();
        self.builder.load(id, loc)
    }
}
