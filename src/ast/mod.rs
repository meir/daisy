use environment::{Type, Value};
use html::Element;
use statement::Statement;

use crate::resolver::File;

pub mod environment;
pub mod functions;
pub mod html;
pub mod statement;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Definition(Type, String, Statement),
    Value(Value),
    Insertion(String),
    Function(Box<Node>),
}

impl Node {
    pub fn render(&self, file: &mut File) -> String {
        match self {
            Node::Element(element) => element.render(file),
            Node::Text(text) => text.clone(),
            Node::Definition(type_, name, statement) => {
                let value = statement.to_value(file);
                file.environment.define(type_.clone(), name.clone(), value);
                "".to_string()
            }
            Node::Insertion(name) => {
                let value = file.environment.get(name).cloned();
                if let Some(value) = value {
                    value.render(file)
                } else {
                    panic!("Value '{}' not defined", name);
                }
            }
            Node::Function(_node) => todo!(),
            _ => todo!(),
        }
    }
}
