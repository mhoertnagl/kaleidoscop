use crate::rlvm::builder::*;
use crate::rlvm::function::*;
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

pub struct LLBasicBlock {
    builder: Rc<LLBuilder>,
    module: Rc<RefCell<LLModule>>,
    fun: Box<LLFunction>,
    pub bb: LLVMBasicBlockRef,
}

impl LLBasicBlock {
    pub fn new(
        name: &str,
        builder: Rc<LLBuilder>,
        module: Rc<RefCell<LLModule>>,
        fun: Box<LLFunction>,
    ) -> Box<LLBasicBlock> {
        unsafe {
            Box::new(LLBasicBlock {
                builder,
                module,
                fun,
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
