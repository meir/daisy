use crate::node::Node;

pub struct Str {
    value: String,
}

impl Str {
    pub fn new(value: String) -> Self {
        Str { value }
    }
}

impl Node for Str {
    fn out(&self) -> String {
        self.value.clone()
    }
}
