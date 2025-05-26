use crate::context::variable::Variable;
use std::collections::HashMap;

pub struct Environment<'e> {
    variables: HashMap<String, Variable>,
    parent: Option<&'e mut Environment<'e>>,
}

impl<'e> Environment<'e> {
    pub fn new(parent: Option<&'e mut Environment<'e>>) -> Self {
        Environment {
            variables: HashMap::new(),
            parent,
        }
    }

    pub fn get(&self, name: String) -> Option<&Variable> {
        self.variables
            .get(name.as_str())
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get(name)))
    }

    pub fn define(&mut self, name: &String, value: Variable) {
        if self.variables.contains_key(name) {
            panic!("Variable '{}' is already defined in this scope", name);
        }
        self.variables.insert(name.clone(), value);
    }

    pub fn set(&mut self, name: String, value: Variable) {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value);
        }
    }
}
