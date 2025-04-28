use crate::node::{Node, str::Str};

pub struct Element {
    name: Str,
    children: Vec<Box<dyn Node>>,
}

impl Element {
    pub fn new(name: Str) -> Self {
        Element { name, children: Vec::new() }
    }

    pub fn add_child(&mut self, child: Box<dyn Node>) {
        self.children.push(child);
    }
}

impl Node for Element {
    fn out(&self) -> String {
        let name = self.name.out();
        if self.children.len() > 0 {
            let children: Vec<String> = self.children.iter().map(|child| child.out()).collect();
            format!("<{}>{}</{}>", name, children.join(""), name)
        } else {
            format!("<{} />", name)
        }
    }
}
