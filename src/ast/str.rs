use crate::ast::AST;

use crate::context::Context;

#[derive(Debug, Clone)]
pub struct Str {
    pub literal: String,
}

impl Str {
    pub fn new(literal: String) -> Self {
        Str { literal }
    }
}

impl AST for Str {
    fn str(&self, _ctx: &Context) -> String {
        self.literal.clone()
    }
}
