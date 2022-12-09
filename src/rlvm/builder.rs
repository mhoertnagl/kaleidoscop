use crate::rlvm::basic_block::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::*;
use std::ffi::*;
use std::ptr;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

pub struct LLBuilder {
    pub builder: LLVMBuilderRef,
}

impl LLBuilder {
    pub fn new() -> Self {
        unsafe {
            LLBuilder {
                builder: LLVMCreateBuilder(),
            }
        }
    }

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

    // pub fn cond_br(
    //     &self,
    //     cond: LLVMValueRef,
    //     then: LLVMBasicBlockRef,
    //     els: LLVMBasicBlockRef,
    // ) -> LLVMValueRef {
    //     unsafe { LLVMBuildCondBr(self.builder, cond, then, els) }
    // }

    pub fn br(&self, dest: Box<LLBasicBlock>) -> LLVMValueRef {
        unsafe { LLVMBuildBr(self.builder, dest.bb) }
    }

    // pub fn br(&self, dest: LLVMBasicBlockRef) -> LLVMValueRef {
    //     unsafe { LLVMBuildBr(self.builder, dest) }
    // }

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
