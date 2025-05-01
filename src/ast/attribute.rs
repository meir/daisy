use crate::ast::str::Str;
use crate::ast::AST;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Str,
    value: Str,
}

impl Attribute {
    pub fn new(name: Str, value: Str) -> Self {
        Attribute { name, value }
    }

    pub fn merge(&self, other: &Self) -> Self {
        if self.name.str() == other.name.str() {
            Attribute {
                name: self.name.clone(),
                value: Str::new(format!("{} {}", self.value.str(), other.value.str())),
            }
        } else {
            Attribute {
                name: self.name.clone(),
                value: self.value.clone(),
            }
        }
    }
}

impl AST for Attribute {
    fn str(&self) -> String {
        format!("{}=\"{}\"", self.name.str(), self.value.str())
    }
}
