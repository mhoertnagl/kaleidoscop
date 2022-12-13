use crate::rlvm::basic_block::*;
use crate::rlvm::builder::*;
use crate::rlvm::module::*;
use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::*;
use std::cell::RefCell;
use std::ffi::*;
use std::ptr;
use std::rc::Rc;

macro_rules! c_str {
    ($s:expr) => {
        CString::new($s).unwrap().as_ptr() as *const i8
    };
}

const LLVM_FALSE: LLVMBool = 0;
const LLVM_TRUE: LLVMBool = 1;

pub struct LLFunction {
    builder: Rc<LLBuilder>,
    module: Rc<LLModule>,
    pub fun: LLVMValueRef,
}

impl LLFunction {
    pub fn empty(builder: Rc<LLBuilder>, module: Rc<LLModule>) -> Rc<LLFunction> {
        unsafe {
            Rc::new(LLFunction {
                builder,
                module,
                fun: LLVMConstNull(LLVMVoidType()),
            })
        }
    }

    pub fn new(
        builder: Rc<LLBuilder>,
        module: Rc<LLModule>,
        name: String,
        args: &mut [LLVMTypeRef],
        ret: LLVMTypeRef,
    ) -> Rc<LLFunction> {
        unsafe {
            let typ = LLVMFunctionType(ret, args.as_mut_ptr(), args.len() as u32, LLVM_FALSE);
            Rc::new(LLFunction {
                builder,
                module: module.clone(),
                fun: LLVMAddFunction(module.module, c_str!(name), typ),
            })
        }
    }

    pub fn append_basic_block(&self, name: &str) -> Rc<LLBasicBlock> {
        LLBasicBlock::new(
            name,
            self.builder.clone(),
            self.module.clone(),
            Rc::new(*self),
        )
    }
}
