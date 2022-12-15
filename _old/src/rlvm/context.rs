use crate::rlvm::builder::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;

pub struct LLContext {
    context: LLVMContextRef,
}

// pub type LLContextRef = *mut LLContext;

impl LLContext {
    pub fn new() -> LLContext {
        unsafe {
            println!("    Context created");
            LLContext {
                context: LLVMContextCreate(),
            }
        }
    }

    pub fn new_builder(&self) -> LLBuilder {
        unsafe {
            println!("    Builder created");
            LLBuilder {
                builder: LLVMCreateBuilderInContext(self.context),
            }
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
        println!("    Context dropped");
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}
