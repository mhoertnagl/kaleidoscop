use crate::rlvm::builder::*;
use crate::rlvm::function::*;
use crate::rlvm::module::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::*;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

pub struct LLBasicBlock {
    name: String,
    builder: LLBuilderRef,
    module: LLModuleRef,
    fun: LLFunctionRef,
    pub bb: LLVMBasicBlockRef,
}

pub type LLBasicBlockRef = *mut LLBasicBlock;

impl LLBasicBlock {
    pub fn new(
        name: &str,
        builder: LLBuilderRef,
        module: LLModuleRef,
        fun: LLFunctionRef,
    ) -> LLBasicBlock {
        unsafe {
            println!("    Basic block [{name}] created");
            LLBasicBlock {
                name: String::from(name),
                builder,
                module,
                fun,
                bb: LLVMAppendBasicBlock(fun.as_ref().unwrap().fun, c_str!(name)),
            }
        }
    }

    pub fn position_at_end(&self) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder.as_ref().unwrap().builder, self.bb);
        }
    }
}

impl Drop for LLBasicBlock {
    fn drop(&mut self) {
        let name = &self.name;
        println!("    Basic block [{name}] dropped");
    }
}
