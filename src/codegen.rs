use crate::ast::*;
use crate::rlvm::builder::*;
use crate::rlvm::context::*;
use crate::rlvm::function::*;
use crate::rlvm::module::*;
// use crate::llvm::*;
use llvm_sys::prelude::LLVMValueRef;
use std::collections::HashMap;

// macro_rules! c_str {
//     ($s:expr) => {
//         CString::new($s).unwrap().as_ptr() as *const i8
//     };
// }

type Statements = Vec<Statement>;

// sowas

pub struct Codegen {
    symbols: HashMap<String, LLVMValueRef>,
    context: LLContext,
    builder: LLBuilder,
    module: LLModule,
    function: LLFunction,
}

impl Codegen {
    pub unsafe fn new() -> Self {
        println!("    Codegen created");
        let context = LLContext::new();
        let mut builder = context.new_builder();
        // TODO: builder.new_module("main");
        let mut module = LLModule::new(&mut builder, "main");
        let function = LLFunction::empty(&mut builder, &mut module);
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
        println!("=== BEGIN Statements");
        for t in s.iter() {
            self.statement(t)
        }
        println!("=== END Statements");
    }

    unsafe fn statement(&mut self, s: &Statement) {
        println!("=== BEGIN Statement");
        match s {
            Statement::Let(x) => self.let_statement(x),
            Statement::Def(x) => self.def_statement(x),
            Statement::Assign(x) => self.assign_statement(x),
            Statement::Expr(x) => self.expr_statement(x),
            Statement::If(x) => self.if_statement(x),
            Statement::Return(x) => self.return_statement(x),
        }
        println!("=== END Statement");
    }

    unsafe fn let_statement(&mut self, s: &LetStatement) {
        println!("=== BEGIN Let Statement [{s}]");
        let typ_int64 = self.context.int64();
        let val = self.expr(&s.expr);
        // let loc = self.builder.alloca(s.name.clone(), typ_int64);
        // self.builder.store(val, loc);
        // TODO: Check for redefinition.
        println!("    Symboltable insert [{s}]");
        // self.symbols.insert(s.name.clone(), loc);
        println!("=== END Let Statement [{s}]");
    }

    unsafe fn def_statement(&mut self, s: &DefStatement) {
        println!("=== BEGIN Def Statement [{s}]");
        let typ_int64 = self.context.int64();
        self.function = self.module.new_function(s.name.clone(), &mut [], typ_int64);
        println!("--- Create the entry BB");
        let bb = self.function.append_basic_block("entry");
        bb.position_at_end();
        self.statements(&s.statements);
        println!("=== END Def Statement [{s}]");
    }

    unsafe fn assign_statement(&mut self, s: &AssignStatement) {
        println!("=== BEGIN Assign Statement [{s}]");
        let val = self.expr(&s.expr);
        // let loc = *self.symbols.get(&s.name).unwrap();
        let loc = self.context.const64(0);
        // self.builder.store(val, loc);
        println!("=== END Assign Statement [{s}]");
    }

    // TODO: Actually we only need a function call statement.
    unsafe fn expr_statement(&mut self, _s: &ExprStatement) {}

    unsafe fn if_statement(&mut self, s: &IfStatement) {
        println!("=== BEGIN If Statement [{s}]");
        let zero_int64 = self.context.zero64();
        let cond = self.expr(&s.cond);
        let nonzero_cond = self.builder.cmp_ne(cond, zero_int64);
        let mut then_block = self.function.append_basic_block("ifcond");
        let mut els_block = self.function.append_basic_block("ifalt");
        let mut merge_block = self.function.append_basic_block("ifmerge");
        self.builder
            .cond_br(nonzero_cond, &mut then_block, &mut els_block);
        then_block.position_at_end();
        self.statements(&s.cons);
        self.builder.br(&mut merge_block);
        els_block.position_at_end();
        self.statements(&s.alt);
        self.builder.br(&mut merge_block);
        merge_block.position_at_end();
        println!("=== END If Statement [{s}]");
    }

    unsafe fn return_statement(&mut self, s: &ReturnStatement) {
        println!("=== BEGIN Return Statement [{s}]");
        let val = self.expr(&s.expr);
        self.builder.ret(val);
        println!("=== END Return Statement [{s}]");
    }

    unsafe fn expr(&mut self, e: &Expr) -> LLVMValueRef {
        println!("=== BEGIN Expression [{e}]");
        let val = match e {
            Expr::Atom(x) => self.atom(x),
            Expr::BinOp(x) => self.binop_expr(x),
            //Expr::FunCall(x) => self.funcall_expr(x),
        };
        println!("=== END Expression [{e}]");
        val
    }

    unsafe fn atom(&mut self, a: &Atom) -> LLVMValueRef {
        println!("=== BEGIN Atom [{a}]");
        let val = match a {
            Atom::Num(x) => self.num(*x),
            Atom::Id(x) => self.id(x),
        };
        println!("=== END Atom [{a}]");
        val
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
        // // self.symbols
        // //     .get(&String::from(id))
        // //     .map(|loc| self.builder.load(id, *loc))
        // //     .unwrap_or(self.context.const64(0))
        // let loc = *self.symbols.get(&String::from(id)).unwrap();
        // self.builder.load(id, loc)
        self.context.const64(0)
    }
}

impl Drop for Codegen {
    fn drop(&mut self) {
        println!("    Codegen dropped");
    }
}
