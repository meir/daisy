use super::Node;
use crate::ast2::Environment;
use crate::context::Context;
use std::collections::HashMap;

pub fn html(element: Html) -> Node {
    Box::new(move |ctx, env| element.build(ctx, env))
}

pub struct Html {
    tag: Box<str>,
    attributes: HashMap<String, Vec<Node>>,
    body: Vec<Node>,
}

impl Html {
    pub fn new(tag: Box<str>) -> Self {
        Html {
            tag,
            attributes: HashMap::new(),
            body: vec![],
        }
    }

    pub fn with_attributes(mut self, attributes: Vec<(String, Vec<Node>)>) -> Self {
        for (key, value) in attributes {
            match &*key {
                "id" => {
                    self.attributes.insert(key, value);
                }
                _ => {
                    self.attributes
                        .entry(key)
                        .or_insert_with(Vec::new)
                        .extend(value);
                }
            }
        }
        self
    }

    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.body.extend(children);
        self
    }

    fn build(&self, ctx: &Context, env: &Environment) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|(k, v)| {
                let value = v
                    .iter()
                    .map(|node| node(ctx, env))
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("{}=\"{}\"", k, value)
            })
            .collect::<Vec<String>>()
            .join(" ");

        let body = self
            .body
            .iter()
            .map(|node| node(ctx, env))
            .collect::<Vec<String>>()
            .join("");
        let tag = &*self.tag;

        match (attributes.as_str(), body.as_str()) {
            ("", "") => format!("<{}/>", tag),
            (_, "") => format!("<{} {} />", tag, attributes),
            ("", _) => format!("<{}>{}</{}>", tag, body, tag),
            _ => format!("<{} {}>{}</{}>", tag, attributes, body, tag),
        }
    }
}
