pub mod attribute;
pub mod element;
pub mod str;

// This trait is object-safe as it only has methods with &self
pub trait AST {
    fn str(&self) -> String;
}

pub enum Node {
    Element(element::Element),
    Str(str::Str),
    Attribute(attribute::Attribute),
}

impl AST for Node {
    fn str(&self) -> String {
        match self {
            Node::Element(e) => e.str(),
            Node::Str(s) => s.str(),
            Node::Attribute(a) => a.str(),
        }
    }
}
