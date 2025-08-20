use std::{cell::RefCell, rc::Rc};

use crate::{
    ast2::{functions::FunctionValue, Build, Node},
    context::Context,
};

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

pub enum Value {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Html(Node),
    Function(FunctionValue),
    Map(Environment),
    List(Environment),
    Nil,
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
