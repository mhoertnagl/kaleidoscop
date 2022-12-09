use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::*;
use std::ffi::*;
use std::ptr;

pub struct Context {
    context: LLVMContextRef,
}

impl Context {
    pub fn get_global() -> Self {
        let context = unsafe { LLVMGetGlobalContext() };

        Context { context: context }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.context);
        }
    }
}
