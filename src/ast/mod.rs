use crate::context::environment::Environment;
use crate::context::Context;

pub mod attribute;
pub mod element;
pub mod insert;
pub mod statement;
pub mod str;
pub mod variable;

// This trait is object-safe as it only has methods with &self
pub trait AST {
    fn str(&self, ctx: &Context, scope: &mut Environment) -> String;
}

#[derive(Debug, Clone)]
pub enum Node {
    Element(element::Element),
    Str(str::Str),
    Attribute(attribute::Attribute),
    Definition(variable::Definition),
    Insert(insert::Insert),
    Statement(statement::Statement),
}

impl AST for Node {
    fn str(&self, ctx: &Context, scope: &mut Environment) -> String {
        match self {
            Node::Element(e) => e.str(ctx, scope),
            Node::Str(s) => s.str(ctx, scope),
            Node::Attribute(a) => a.str(ctx, scope),
            Node::Definition(d) => d.str(ctx, scope),
            Node::Insert(i) => i.str(ctx, scope),
            Node::Statement(s) => s.str(ctx, scope),
            Node::Variable(v) => v.str(ctx, scope),
        }
    }
}
