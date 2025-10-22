mod binop;
mod block;
mod flow;
mod unop;

use crate::IRComponent;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::AtomicUsize;

pub trait Instruction: IRComponent {}

pub struct BasicBlock {
    basic_block_index: Arc<AtomicUsize>,
    ssa_register_index: Arc<AtomicUsize>,
    label: String,
    instructions: Vec<Box<dyn Instruction>>,
    pub(crate) children: Vec<BasicBlock>,
}
