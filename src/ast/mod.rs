use crate::context::Context;

pub mod attribute;
pub mod element;
pub mod str;

// This trait is object-safe as it only has methods with &self
pub trait AST {
    fn str(&self, ctx: &Context) -> String;
}

pub enum Node {
    Element(element::Element),
    Str(str::Str),
    Attribute(attribute::Attribute),
}

impl AST for Node {
    fn str(&self, ctx: &Context) -> String {
        match self {
            Node::Element(e) => e.str(ctx),
            Node::Str(s) => s.str(ctx),
            Node::Attribute(a) => a.str(ctx),
        }
    }
}
