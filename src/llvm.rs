use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::*;
use std::ffi::*;
use std::ptr;

// TODO: https://doc.rust-lang.org/std/ops/trait.DerefMut.html

const LLVM_FALSE: LLVMBool = 0;
const LLVM_TRUE: LLVMBool = 1;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

pub struct LLContext {
    context: LLVMContextRef,
}

impl LLContext {
    pub fn new() -> Box<LLContext> {
        unsafe {
            Box::new(LLContext {
                context: LLVMContextCreate(),
            })
        }
    }

    pub fn new_builder(&self) -> Box<LLBuilder> {
        unsafe {
            Box::new(LLBuilder {
                builder: LLVMCreateBuilderInContext(self.context),
            })
        }
    }

    // pub fn int64(&self) -> LLVMTypeRef {
    //     unsafe { LLVMInt64TypeInContext(self.context) }
    // }

    // pub fn const64(&self, num: u64) -> LLVMValueRef {
    //     unsafe { LLVMConstInt(self.int64(), num, 0) }
    // }

    // pub fn zero64(&self) -> LLVMValueRef {
    //     self.const64(0)
    // }
}

impl Drop for LLContext {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}

pub struct LLModule<'a> {
    builder: &'a LLBuilder,
    module: LLVMModuleRef,
}

impl<'a> LLModule<'a> {
    pub fn new(builder: &'a LLBuilder, name: &str) -> Box<LLModule<'a>> {
        unsafe {
            Box::new(LLModule {
                builder,
                module: LLVMModuleCreateWithName(c_str!(name)),
            })
        }
    }

    pub fn new_function(
        &'a self,
        name: String,
        args: &mut [LLVMTypeRef],
        ret: LLVMTypeRef,
    ) -> Box<LLFunction> {
        // TODO: boxing?
        LLFunction::new(self.builder, self, name, args, ret)
    }

    pub fn write_to_file(&self, name: &str) {
        unsafe {
            LLVMWriteBitcodeToFile(self.module, c_str!(name));
            LLVMDumpModule(self.module);
        }
    }
}

impl Drop for LLModule<'_> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}

pub struct LLFunction<'a> {
    builder: &'a LLBuilder,
    module: &'a LLModule<'a>,
    fun: LLVMValueRef,
}

impl<'a> LLFunction<'a> {
    pub fn empty(builder: &'a LLBuilder, module: &'a LLModule<'a>) -> Box<LLFunction<'a>> {
        unsafe {
            Box::new(LLFunction {
                builder,
                module,
                fun: LLVMConstNull(LLVMVoidType()),
            })
        }
    }

    pub fn new(
        builder: &'a LLBuilder,
        module: &'a LLModule,
        name: String,
        args: &mut [LLVMTypeRef],
        ret: LLVMTypeRef,
    ) -> Box<LLFunction<'a>> {
        unsafe {
            let typ = LLVMFunctionType(ret, args.as_mut_ptr(), args.len() as u32, LLVM_FALSE);
            Box::new(LLFunction {
                builder,
                module,
                fun: LLVMAddFunction(module.module, c_str!(name), typ),
            })
        }
    }

    pub fn append_basic_block(&'static self, name: &str) -> Box<LLBasicBlock> {
        LLBasicBlock::new(name, self.builder, self.module, self)
    }
}

pub struct LLBasicBlock<'a> {
    builder: &'a LLBuilder,
    module: &'a LLModule<'a>,
    fun: &'a LLFunction<'a>,
    bb: LLVMBasicBlockRef,
}

impl<'a> LLBasicBlock<'a> {
    pub fn new(
        name: &str,
        builder: &'a LLBuilder,
        module: &'a LLModule,
        fun: &'a LLFunction,
    ) -> Box<LLBasicBlock<'a>> {
        unsafe {
            Box::new(LLBasicBlock {
                builder,
                module,
                fun: &fun,
                bb: LLVMAppendBasicBlock(fun.fun, c_str!(name)),
            })
        }
    }

    pub fn position_at_end(&self) {
        unsafe {
            LLVMPositionBuilderAtEnd((*self.builder).builder, self.bb);
        }
    }
}

pub struct LLBuilder {
    builder: LLVMBuilderRef,
}

impl LLBuilder {
    pub fn mul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildMul(self.builder, lhs, rhs, c_str!("prod")) }
    }

    pub fn div(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildUDiv(self.builder, lhs, rhs, c_str!("div")) }
    }

    pub fn add(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildAdd(self.builder, lhs, rhs, c_str!("sum")) }
    }

    pub fn sub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildSub(self.builder, lhs, rhs, c_str!("diff")) }
    }

    pub fn alloca(&self, name: String, typ: LLVMTypeRef) -> LLVMValueRef {
        unsafe { LLVMBuildAlloca(self.builder, typ, c_str!(name)) }
    }

    pub fn store(&self, val: LLVMValueRef, loc: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildStore(self.builder, val, loc) }
    }

    pub fn load(&self, name: &str, loc: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildLoad(self.builder, loc, c_str!(name)) }
    }

    pub fn cmp_eq(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("cmp.eq", lhs, LLVMIntPredicate::LLVMIntEQ, rhs)
    }

    pub fn cmp_ne(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("cmp.ne", lhs, LLVMIntPredicate::LLVMIntNE, rhs)
    }

    pub fn ucmp_gt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("ucmp.gt", lhs, LLVMIntPredicate::LLVMIntUGT, rhs)
    }

    pub fn ucmp_ge(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("ucmp.ge", lhs, LLVMIntPredicate::LLVMIntUGE, rhs)
    }

    pub fn ucmp_le(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("ucmp.le", lhs, LLVMIntPredicate::LLVMIntULE, rhs)
    }

    pub fn ucmp_lt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("ucmp.lt", lhs, LLVMIntPredicate::LLVMIntULT, rhs)
    }

    pub fn scmp_gt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("scmp.gt", lhs, LLVMIntPredicate::LLVMIntSGT, rhs)
    }

    pub fn scmp_ge(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("scmp.ge", lhs, LLVMIntPredicate::LLVMIntSGE, rhs)
    }

    pub fn scmp_le(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("scmp.le", lhs, LLVMIntPredicate::LLVMIntSLE, rhs)
    }

    pub fn scmp_lt(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        self.cmp("scmp.lt", lhs, LLVMIntPredicate::LLVMIntSLT, rhs)
    }

    pub fn cmp(
        &self,
        name: &str,
        lhs: LLVMValueRef,
        op: LLVMIntPredicate,
        rhs: LLVMValueRef,
    ) -> LLVMValueRef {
        unsafe { LLVMBuildICmp(self.builder, op, lhs, rhs, c_str!(name)) }
    }

    pub fn cond_br(
        &self,
        cond: LLVMValueRef,
        then: Box<LLBasicBlock>,
        els: Box<LLBasicBlock>,
    ) -> LLVMValueRef {
        unsafe { LLVMBuildCondBr(self.builder, cond, then.bb, els.bb) }
    }

    pub fn br(&self, dest: Box<LLBasicBlock>) -> LLVMValueRef {
        unsafe { LLVMBuildBr(self.builder, dest.bb) }
    }

    pub fn ret(&self, val: LLVMValueRef) -> LLVMValueRef {
        unsafe { LLVMBuildRet(self.builder, val) }
    }
}

impl Drop for LLBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}
