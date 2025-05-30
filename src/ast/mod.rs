use environment::Variable;
use html::Element;

use crate::resolver::File;

pub mod environment;
pub mod html;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Definition(String, Variable),
    Insertion(String),
}

impl Node {
    pub fn render(&self, file: &mut File) -> String {
        match self {
            Node::Element(element) => element.render(file),
            Node::Text(text) => text.clone(),
            Node::Definition(name, variable) => {
                file.environment.define(name.clone(), variable.clone());
                "".to_string()
            }
            Node::Insertion(name) => {
                let value = file.environment.get(name).cloned();
                if let Some(value) = value {
                    value.render(file)
                } else {
                    panic!("Variable '{}' not defined", name);
                }
            }
        }
    }
}
