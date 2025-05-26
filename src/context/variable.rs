use crate::ast::Node;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Variable {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Element(Node),
}

impl Variable {
    pub fn is_string(&self) -> bool {
        matches!(self, Variable::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Variable::Number(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Variable::Float(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Variable::Boolean(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Variable::Element(_))
    }
    
    pub fn with_type(&self, type: Type) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub enum Type {
    String,
    Number,
    Float,
    Boolean,
    Element,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
}
