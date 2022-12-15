use crate::rlvm::basic_block::*;
use crate::rlvm::builder::*;
use crate::rlvm::module::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::*;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

const LLVM_FALSE: LLVMBool = 0;
// const LLVM_TRUE: LLVMBool = 1;

pub struct LLFunction {
    name: String,
    builder: LLBuilderRef,
    module: LLModuleRef,
    pub fun: LLVMValueRef,
}

pub type LLFunctionRef = *mut LLFunction;

impl LLFunction {
    pub fn empty(builder: LLBuilderRef, module: LLModuleRef) -> LLFunction {
        unsafe {
            println!("    Empty function created");
            LLFunction {
                name: String::from("<empty>"),
                builder,
                module,
                fun: LLVMConstNull(LLVMVoidType()),
            }
        }
    }

    pub fn new(
        builder: LLBuilderRef,
        module: LLModuleRef,
        name: String,
        args: &mut [LLVMTypeRef],
        ret: LLVMTypeRef,
    ) -> LLFunction {
        unsafe {
            println!("    Function [{name}] created");
            let typ = LLVMFunctionType(ret, args.as_mut_ptr(), args.len() as u32, LLVM_FALSE);
            LLFunction {
                name: name.clone(),
                builder,
                module,
                fun: LLVMAddFunction(module.as_ref().unwrap().module, c_str!(name), typ),
            }
        }
    }

    pub fn append_basic_block(&mut self, name: &str) -> LLBasicBlock {
        LLBasicBlock::new(name, self.builder, self.module, self)
    }
}

impl Drop for LLFunction {
    fn drop(&mut self) {
        let name = &self.name;
        println!("    Function [{name}] dropped");
    }
}
