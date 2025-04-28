use crate::node::Node;

pub struct Root {
    nodes: Vec<Box<dyn Node>>,
}

impl Root {
    pub fn new() -> Self {
        Root { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        self.nodes.push(node);
    } 

    pub fn out(&self) -> String {
        let mut output = String::new();
        for node in &self.nodes {
            output.push_str(&node.out());
        }
        output
    }
}
