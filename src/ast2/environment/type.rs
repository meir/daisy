use super::{TypeValue, Value};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Result},
    rc::Rc,
};

#[derive(Clone)]
pub enum Type {
    String,
    Number,
    Float,
    Boolean,
    Html,
    Function,
    Map,
    List,
    Nil,
    Any, // not usable in the language, but needed to return any type using "use"
}

impl Type {
    pub fn with_value(&self, value: Value) -> TypeValue {
        Rc::new(RefCell::new((self.clone(), value)))
    }

    pub fn matches(&self, value: &Value) -> bool {
        let t = value.type_of();
        match (self, t) {
            (Type::String, Type::String) => true,
            (Type::Number, Type::Number) => true,
            (Type::Float, Type::Float) => true,
            (Type::Boolean, Type::Boolean) => true,
            (Type::Html, Type::Html) => true,
            (Type::Function, Type::Function) => true,
            (Type::Map, Type::Map) => true,
            (Type::List, Type::List) => true,
            (_, Type::Nil) => true,
            (Type::Any, _) => true,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Type::String => write!(f, "String"),
            Type::Number => write!(f, "Number"),
            Type::Float => write!(f, "Float"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Html => write!(f, "Html"),
            Type::Function => write!(f, "Function"),
            Type::Map => write!(f, "Map"),
            Type::List => write!(f, "List"),
            Type::Nil => write!(f, "Nil"),
            Type::Any => write!(f, "Any"),
        }
    }
}
