use inkwell::basic_block::BasicBlock;
use inkwell::values::FunctionValue;

pub trait BasicBlockExt {
    /// Returns true iff the basic block ends with a
    /// terminator instruction.
    fn has_terminator(self) -> bool;

    /// Returns true iff the basic block does not end with a
    /// terminator instruction.
    fn has_no_terminator(self) -> bool;
}

impl<'ctx> BasicBlockExt for BasicBlock<'ctx> {
    fn has_terminator(self) -> bool {
        self.get_terminator().is_some()
    }

    fn has_no_terminator(self) -> bool {
        self.get_terminator().is_none()
    }
}

pub trait FunctionValueExt<'ctx> {
    /// Returns true iff the function does not end with a
    /// terminator instruction.
    fn has_no_terminator(self) -> bool;
}

impl<'ctx> FunctionValueExt<'ctx> for FunctionValue<'ctx> {
    fn has_no_terminator(self) -> bool {
        self.get_last_basic_block().unwrap().has_no_terminator()
    }
}
