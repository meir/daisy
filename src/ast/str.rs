use crate::ast::AST;

#[derive(Debug, Clone)]
pub struct Str {
    literal: String,
}

impl Str {
    pub fn new(literal: String) -> Self {
        Str { literal }
    }
}

impl AST for Str {
    fn str(&self) -> String {
        self.literal.clone()
    }
}
