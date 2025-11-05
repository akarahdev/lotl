use crate::context::TyContext;
use lotl_ast::types::AstType;
use std::collections::HashMap;

pub(crate) mod exprs;
pub(crate) mod headers;
pub(crate) mod stmts;

pub struct TypeGatherer<'a> {
    pub ctx: &'a mut TyContext,
}

impl<'a> TypeGatherer<'a> {
    pub fn new(ctx: &'a mut TyContext) -> Self {
        TypeGatherer { ctx }
    }
}

pub struct TypedStackFrame {
    variables: HashMap<String, AstType>,
}

pub struct TypedStack {
    frames: Vec<TypedStackFrame>,
}

impl TypedStack {
    pub fn new() -> Self {
        let mut o = Self { frames: vec![] };
        o.push_frame();
        o
    }

    pub fn push_frame(&mut self) {
        self.frames.push(TypedStackFrame {
            variables: HashMap::new(),
        });
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn lookup_var(&self, name: &str) -> Option<&AstType> {
        for frame in self.frames.iter().rev() {
            if let Some(var) = frame.variables.get(name) {
                return Some(var);
            }
        }
        None
    }

    pub fn write_var(&mut self, name: &str, value: AstType) {
        self.frames
            .last_mut()
            .unwrap()
            .variables
            .insert(name.to_string(), value);
    }
}
