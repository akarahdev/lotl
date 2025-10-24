mod aggregate;
mod binop;
mod block;
mod flow;
mod unop;
mod memory;

use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::AtomicUsize;

pub trait Instruction: IRComponent {}

pub struct BasicBlock {
    basic_block_index: Arc<AtomicUsize>,
    ssa_register_index: Arc<AtomicUsize>,
    label: String,
    instructions: Vec<Box<dyn Instruction>>,
    pub(crate) children: Vec<BasicBlock>,
}

pub struct BasicBlockHandle<'a>(pub(crate) &'a mut BasicBlock);

impl Deref for BasicBlockHandle<'_> {
    type Target = BasicBlock;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl DerefMut for BasicBlockHandle<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}
