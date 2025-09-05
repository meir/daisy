use std::fmt::{Display, Formatter, Result};
use std::{cell::RefCell, rc::Rc};

use crate::ast2::functions::FunctionRunner;
use crate::ast2::Node;
use crate::{ast2::Build, context::Context};

use super::{Environment, Type};

pub type TypeValue = Rc<RefCell<(Type, Value)>>;

pub trait CheckTypeValue {
    fn check(&self);
}

impl CheckTypeValue for TypeValue {
    fn check(&self) {
        let typevalue = self.borrow();
        let t = &typevalue.0;
        let value = &typevalue.1;
        if !t.matches(value) {
            panic!("Type mismatch: expected {}, found {}", "a", "b");
        }
    }
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Html(Node),
    Function(Box<dyn FunctionRunner>),
    Map(Environment),
    List(Environment),
    Nil,
}

impl Value {
    pub fn type_of(&self) -> Type {
        match self {
            Value::String(_) => Type::String,
            Value::Number(_) => Type::Number,
            Value::Float(_) => Type::Float,
            Value::Boolean(_) => Type::Boolean,
            Value::Html(_) => Type::Html,
            Value::Function(_) => Type::Function,
            Value::Map(_) => Type::Map,
            Value::List(_) => Type::List,
            Value::Nil => Type::Nil,
        }
    }

    pub fn as_typevalue(self) -> TypeValue {
        let t = self.type_of();
        Rc::new(RefCell::new((t, self)))
    }
}

impl Build for Value {
    fn build(&self, ctx: &Context, env: &Environment) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Html(node) => node(ctx, env),
            Value::Function(..) => "function".to_string(),
            Value::Map(sub_env) => sub_env.build(ctx, env),
            Value::List(sub_env) => sub_env.build(ctx, env),
            Value::Nil => "nil".to_string(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::Boolean(l), Value::Boolean(r)) => l == r,
            (Value::Nil, Value::Nil) => true,

            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}

impl Display for Value {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let t = self.type_of();
        write!(formatter, "{}", t)
    }
}
