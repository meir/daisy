use super::Value;

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
    pub fn matches(&self, value: &Value) -> bool {
        match (self, value) {
            (Type::String, Value::String(_)) => true,
            (Type::Number, Value::Number(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::Boolean, Value::Boolean(_)) => true,
            (Type::Html, Value::Html(_)) => true,
            (Type::Function, Value::Function(_)) => true,
            (Type::Map, Value::Map(_)) => true,
            (Type::List, Value::List(_)) => true,
            (_, Value::Nil) => true,
            (Type::Any, _) => true,
            _ => false,
        }
    }
}
