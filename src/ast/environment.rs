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
    Function(fn(&Vec<Value>, &mut Scope) -> Value),
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
            Value::Function(function) => todo!(),
            Value::Nil => "nil".to_string(),
        }
    }

    pub fn get_type(&self) -> Type {
        match &self {
            Value::Str(_) => Type::Str,
            Value::Num(_) => Type::Num,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
            Value::Element(_) => Type::Element,
            Value::Function(_) => Type::Function,
            Value::Nil => Type::Nil,
        }
    }

    pub fn set_value(&mut self, value: Value) {
        if !Type::matches(&self.get_type(), &value) {
            panic!("Type mismatch: expected {}, got {}", self.get_type(), value);
        }
        self.clone_from(&value);
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
            Value::Function(_) => write!(f, "function()"),
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
    Function,
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
            (Type::Function, Value::Function(_)) => true,
            (_, Value::Nil) => true,
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
            Type::Function => write!(f, "Function"),
            Type::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Clone)]
pub struct Scope {
    variables: HashMap<String, Value>,
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
            let mut scope = Self {
                variables: HashMap::new(),
                parent: None,
            };

            scope.define(
                Type::Function,
                "hello_world".to_string(),
                Value::Function(|_args, scope| Value::Str("Hello, World!".to_string())),
            );

            scope
        }
    }

    pub fn define(&mut self, type_: Type, name: String, value: Value) {
        if self.variables.contains_key(&name) {
            panic!("Value {} already defined in this scope", name);
        }
        if !Type::matches(&type_, &value) {
            panic!("Type mismatch: expected {}, got {}", type_, value);
        }
        self.variables.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
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
            panic!("Value {} not found in any scope", name);
        }
    }
}
