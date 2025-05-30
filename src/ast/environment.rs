use crate::ast::Node;
use crate::resolver::File;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(i64),
    Float(f64),
    Bool(bool),
    Element(Box<Node>),
    Nil,
}

impl Value {
    pub fn render(&self, file: &mut File) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Num(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Element(node) => node.render(file),
            Value::Nil => "nil".to_string(),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "str\"{}\"", s),
            Value::Num(n) => write!(f, "num({})", n),
            Value::Float(n) => write!(f, "float({})", n),
            Value::Bool(b) => write!(f, "bool({})", b),
            Value::Element(_) => write!(f, "element()"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone)]
pub enum Type {
    Str,
    Num,
    Float,
    Bool,
    Element,
    Nil,
}

impl Type {
    pub fn matches(type_: &Type, value: &Value) -> bool {
        match (type_, value) {
            (Type::Str, Value::Str(_)) => true,
            (Type::Num, Value::Num(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Element, Value::Element(_)) => true,
            (Type::Nil, _) => true,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Str => write!(f, "String"),
            Type::Num => write!(f, "Integer"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Boolean"),
            Type::Element => write!(f, "Element"),
            Type::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Clone)]
pub struct Variable {
    type_: Type,
    value: Value,
}

impl Variable {
    pub fn new(type_: Type, value: Value) -> Self {
        if !Type::matches(&type_, &value) {
            panic!("Type mismatch: expected {}, got {}", type_, value);
        }
        Variable { type_, value }
    }

    pub fn get_type(&self) -> &Type {
        &self.type_
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }

    pub fn set_value(&mut self, value: Value) {
        if !Type::matches(&self.type_, &value) {
            panic!("Type mismatch: expected {}, got {}", self.type_, value);
        }
        self.value = value;
    }

    pub fn render(&self, file: &mut File) -> String {
        self.value.render(file)
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.type_, self.value)
    }
}

pub struct Scope {
    variables: HashMap<String, Variable>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Scope>) -> Self {
        if let Some(parent) = parent {
            Self {
                variables: HashMap::new(),
                parent: Some(Box::new(parent)),
            }
        } else {
            Self {
                variables: HashMap::new(),
                parent: None,
            }
        }
    }

    pub fn define(&mut self, name: String, variable: Variable) {
        if self.variables.contains_key(&name) {
            panic!("Variable {} already defined in this scope", name);
        }
        self.variables.insert(name, variable);
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        if let Some(var) = self.variables.get(name) {
            return Some(var);
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        None
    }

    pub fn set(&mut self, name: String, value: Value) {
        if let Some(var) = self.variables.get_mut(&name) {
            var.set_value(value);
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value);
        } else {
            panic!("Variable {} not found in any scope", name);
        }
    }
}
