use crate::{ast::node::Node, context::Context};
use std::collections::HashMap;

use super::environment::Scope;

#[derive(Clone)]
pub struct Element {
    tag: String,
    attributes: HashMap<String, Vec<Node>>,
    content: Vec<Node>,
}

impl Element {
    pub fn new(tag: String, attributes: Vec<(String, Vec<Node>)>, content: Vec<Node>) -> Self {
        let mut element = Element {
            tag,
            content,
            attributes: HashMap::new(),
        };

        element.add_attribute(attributes);

        element
    }

    pub fn add_attribute(&mut self, attributes: Vec<(String, Vec<Node>)>) {
        for (key, value) in attributes {
            match key.as_str() {
                "id" => {
                    self.attributes.insert(key, value);
                }
                _ => {
                    if let Some(existing) = self.attributes.get_mut(&key) {
                        existing.extend(value);
                    } else {
                        self.attributes.insert(key, value);
                    }
                }
            };
        }
    }

    pub fn render(&self, ctx: &mut Context, scope: &mut Scope) -> String {
        let attributes: HashMap<String, String> = self
            .attributes
            .iter()
            .map(|(k, v)| {
                let value = v
                    .iter()
                    .map(|node| node.render(ctx, scope))
                    .collect::<Vec<String>>()
                    .join(" ");
                (k.clone(), value)
            })
            .collect();

        let mut output = if attributes.is_empty() {
            format!("<{}>", self.tag)
        } else {
            let attrs: String = attributes
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<String>>()
                .join(" ");
            format!("<{} {}>", self.tag, attrs)
        };

        for node in &self.content {
            output.push_str(node.render(ctx, scope).as_str());
        }
        output.push_str(&format!("</{}>", self.tag));
        output
    }
}
