use crate::rlvm::builder::*;
use crate::rlvm::function::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::cell::RefCell;
use std::ffi::*;
use std::ptr;
use std::rc::Rc;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

pub struct LLModule {
    builder: Rc<LLBuilder>,
    pub module: LLVMModuleRef,
}

impl LLModule {
    pub fn new(builder: Rc<LLBuilder>, name: &str) -> Rc<LLModule> {
        unsafe {
            Rc::new(LLModule {
                builder,
                module: LLVMModuleCreateWithName(c_str!(name)),
            })
        }
    }

    pub fn new_function(
        &self,
        name: String,
        args: &mut [LLVMTypeRef],
        ret: LLVMTypeRef,
    ) -> Rc<LLFunction> {
        LLFunction::new(self.builder.clone(), Rc::new(*self), name, args, ret)
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
        unsafe {
            LLVMDisposeModule(self.module);
        }
    }
}
