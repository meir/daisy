use crate::ast::str::Str;
use crate::ast::{Node, AST};

pub struct Element {
    name: Str,
    attributes: Vec<Node>,
    content: Vec<Node>,
}

impl Element {
    pub fn new(name: Str) -> Self {
        Element {
            name,
            attributes: Vec::new(),
            content: Vec::new(),
        }
    }

    pub fn add_attributes(&mut self, attributes: Vec<Node>) {
        for attr in attributes {
            match attr {
                Node::Attribute(attr) => {
                    for existing_attr in &mut self.attributes {
                        if let Node::Attribute(existing_attr) = existing_attr {
                            if existing_attr.name.str() == attr.name.str() {
                                *existing_attr = existing_attr.merge(&attr);
                                return;
                            }
                        }
                    }

                    self.attributes.push(Node::Attribute(attr));
                }
                _ => {}
            }
        }
    }

    pub fn add_children(&mut self, children: Vec<Node>) {
        self.content.extend(children);
    }
}

impl AST for Element {
    fn str(&self) -> String {
        let mut result = format!("<{}", self.name.str());
        if self.attributes.len() > 0 {
            result.push_str(" ");
            for attr in &self.attributes {
                result.push_str(&attr.str());
            }
        }
        result.push_str(">");

        for node in &self.content {
            result.push_str(&node.str());
        }
        result.push_str(&format!("</{}>", self.name.str()));
        result
    }
}
