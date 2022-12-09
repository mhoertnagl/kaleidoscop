use crate::rlvm::builder::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::*;
use std::ffi::*;
use std::ptr;
use std::rc::Rc;

pub struct LLContext {
    context: LLVMContextRef,
}

impl LLContext {
    pub fn new() -> Rc<LLContext> {
        unsafe {
            Rc::new(LLContext {
                context: LLVMContextCreate(),
            })
        }
    }

    pub fn new_builder(&self) -> Rc<LLBuilder> {
        unsafe {
            Rc::new(LLBuilder {
                builder: LLVMCreateBuilderInContext(self.context),
            })
        }
    }

    pub fn int64(&self) -> LLVMTypeRef {
        unsafe { LLVMInt64TypeInContext(self.context) }
    }

    pub fn const64(&self, num: u64) -> LLVMValueRef {
        unsafe { LLVMConstInt(self.int64(), num, 0) }
    }

    pub fn zero64(&self) -> LLVMValueRef {
        self.const64(0)
    }
}

impl Drop for LLContext {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}
