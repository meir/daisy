use super::Value;
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Type {
    String,
    Number,
    Float,
    Bool,
    Element,
    Function,
    Map,
    Array,
    Nil,
    Any, // not usable in the language, but needed to return any type using "use"
}

impl Type {
    pub fn matches(type_: &Type, value: &Value) -> bool {
        match (type_, value) {
            (Type::String, Value::String(_)) => true,
            (Type::Number, Value::Number(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Element, Value::Element(_)) => true,
            (Type::Function, Value::Function(..)) => true,
            (Type::Map, Value::Map(_)) => true,
            (Type::Array, Value::Array(_)) => true,
            (_, Value::Nil) => true,
            (Type::Any, _) => true,
            (_, Value::Scoped(_, value)) => Type::matches(type_, value),
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "String"),
            Type::Number => write!(f, "Integer"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Boolean"),
            Type::Element => write!(f, "Element"),
            Type::Function => write!(f, "Function"),
            Type::Map => write!(f, "Map"),
            Type::Array => write!(f, "Array"),
            Type::Nil => write!(f, "Nil"),
            Type::Any => write!(f, "Any"),
        }
    }
}
