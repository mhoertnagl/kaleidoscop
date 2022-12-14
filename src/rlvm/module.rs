use crate::rlvm::builder::*;
use crate::rlvm::function::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::ffi::*;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

pub struct LLModule {
    name: String,
    builder: LLBuilderRef,
    pub module: LLVMModuleRef,
}

pub type LLModuleRef = *mut LLModule;

impl LLModule {
    pub fn new(builder: LLBuilderRef, name: &str) -> LLModule {
        unsafe {
            println!("    Module [{name}] created");
            LLModule {
                name: String::from(name),
                builder,
                module: LLVMModuleCreateWithName(c_str!(name)),
            }
        }
    }

    pub fn new_function(
        &mut self,
        name: String,
        args: &mut [LLVMTypeRef],
        ret: LLVMTypeRef,
    ) -> LLFunction {
        LLFunction::new(self.builder, self, name, args, ret)
    }

    pub fn write_to_file(&self, name: &str) {
        unsafe {
            LLVMWriteBitcodeToFile(self.module, c_str!(name));
            LLVMDumpModule(self.module);
        }
    }
}

impl Drop for LLModule {
    fn drop(&mut self) {
        let name = &self.name;
        println!("    Module [{name}] dropped");
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}
