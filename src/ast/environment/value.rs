use super::{Scope, Type};
use crate::ast::expression::Expression;
use crate::ast::node::Node;
use crate::ast::statement::Statement;
use crate::context::Context;
use std::fmt::Display;
use std::rc::Rc;

macro_rules! impl_try_into {
    ($method_name:ident => $variant:ident($($variables:ident),+) -> $type:ty) => {
        #[allow(dead_code)]
        pub fn $method_name(self) -> Option<$type> {
            if let Value::$variant($($variables),+) = self {
                Some(($($variables),+))
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
pub enum Value {
    String(String),
    Number(i64),
    Float(f64),
    Bool(bool),
    Element(Rc<Node>),
    Function(
        fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
        Rc<Vec<(Type, String, Option<Expression>)>>,
        Type,
        Rc<Vec<Statement>>,
    ),
    Map(Scope),
    Array(Scope),
    Nil,

    Scoped(Scope, Box<Value>),
}

impl Value {
    pub fn render(&self, ctx: &mut Context, scope: &mut Scope) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Element(node) => node(ctx, scope),
            Value::Function(..) => "".into(),
            Value::Map(scope) => {
                let mut scope = scope.clone();
                let keys = &scope.get_keys();
                let mut output = String::new();
                for key in keys {
                    if let Some(value) = scope.clone().get(&key) {
                        let result = &value.render(ctx, &mut scope);
                        output.push_str(format!("{}; {}\n", key, result).as_str())
                    }
                }
                output
            }
            Value::Array(scope) => {
                let mut scope = scope.clone();
                let mut output = String::new();

                for key in scope.get_indices() {
                    if let Some(value) = scope.clone().get(&key) {
                        output.push_str(&value.render(ctx, &mut scope));
                    }
                }
                output
            }
            Value::Nil => "nil".to_string(),
            Value::Scoped(scope, value) => value.render(ctx, &mut scope.clone()),
        }
    }

    pub fn get_type(&self) -> Type {
        match &self {
            Value::String(_) => Type::String,
            Value::Number(_) => Type::Number,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
            Value::Element(_) => Type::Element,
            Value::Function(..) => Type::Function,
            Value::Map(..) => Type::Map,
            Value::Array(..) => Type::Array,
            Value::Nil => Type::Nil,
            Value::Scoped(_, value) => value.get_type(),
        }
    }

    pub fn set_value(&mut self, value: Value) {
        self.clone_from(&value);
    }

    #[allow(dead_code)]
    pub fn assert_type(&self, type_: &Type) {
        if !Type::matches(type_, self) {
            panic!("Type mismatch: expected {}, got {}", type_, self.get_type());
        }
    }

    impl_try_into!(try_into_string => String(s) -> String);
    impl_try_into!(try_into_number => Number(n) -> i64);
    impl_try_into!(try_into_float => Float(s) -> f64);
    impl_try_into!(try_into_bool => Bool(s) -> bool);
    impl_try_into!(try_into_element => Element(s) -> Rc<Node>);
    impl_try_into!(try_into_function => Function(func, args, return_type, body) -> (fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value, Rc<Vec<(Type, String, Option<Expression>)>>, Type, Rc<Vec<Statement>>));
    impl_try_into!(try_into_map => Map(scope) -> Scope);
    impl_try_into!(try_into_array => Array(scope) -> Scope);
    impl_try_into!(try_into_scoped => Scoped(scope, value) -> (Scope, Box<Value>));
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Nil, Value::Nil) => true,

            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "str\"{}\"", s),
            Value::Number(n) => write!(f, "num({})", n),
            Value::Float(n) => write!(f, "float({})", n),
            Value::Bool(b) => write!(f, "bool({})", b),
            Value::Element(_) => write!(f, "element()"),
            Value::Function(..) => write!(f, "function()"),
            Value::Map(..) => write!(f, "Map()"),
            Value::Array(..) => write!(f, "Array()"),
            Value::Nil => write!(f, "nil"),
            Value::Scoped(_, value) => write!(f, "Scoped({})", value),
        }
    }
}
