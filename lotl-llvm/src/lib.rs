//! Lotl-LLVM is a LLVM IR text generator.
//!
//! Note that documentation is sometimes borrowed from the LLVM IR LangRef,
//! which is licensed under the Apache 2.0 license.
//! Some text has been modified / paraphrased to be more concise and make sense in a Rust context.
//!
//! https://llvm.org/docs/LangRef.html

#![no_std]
extern crate alloc;

pub mod module;
pub mod types;
pub mod value;

use alloc::string::String;

/// Represents that a struct is able to emitted as LLVM IR.
pub trait IRComponent {
    /// Appends the corresponding LLVM IR for this component onto the string.
    fn append_to_string(&self, string: &mut String);
    
    /// Generates the corresponding LLVM IR for this component into a new string buffer,
    /// and returns the output.
    fn emit(&self) -> String {
        let mut s = String::new();
        self.append_to_string(&mut s);
        s
    }
}
