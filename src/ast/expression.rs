use super::{environment::Scope, function::call_function};
use crate::ast::environment::Value;

#[derive(Clone)]
pub enum Expression {
    Value(Value),
    Call(String, Vec<Value>),
    Identifier(String),
    Addition(Box<Expression>, Box<Expression>),
    Subtraction(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Multiplication(Box<Expression>, Box<Expression>),
    Nil,
}

impl Expression {
    pub fn to_value(&self, scope: &mut Scope) -> Value {
        match self {
            Expression::Value(value) => value.clone(),
            Expression::Call(name, args) => {
                let value = scope
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Function '{}' not defined", name));
                call_function(&value, args, scope)
            }
            Expression::Identifier(name) => scope
                .get(name)
                .cloned()
                .unwrap_or_else(|| panic!("Variable '{}' not defined in the current scope", name)),
            Expression::Addition(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (left_value.clone(), right_value.clone()) {
                    (Value::Num(l), Value::Num(r)) => Value::Num(l + r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l + r),
                    (Value::Str(l), Value::Str(r)) => Value::Str(l + &r),
                    _ => panic!(
                        "Type mismatch in addition: {} + {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::Subtraction(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (left_value.clone(), right_value.clone()) {
                    (Value::Num(l), Value::Num(r)) => Value::Num(l - r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
                    _ => panic!(
                        "Type mismatch in subtraction: {} - {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::Multiplication(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (left_value.clone(), right_value.clone()) {
                    (Value::Num(l), Value::Num(r)) => Value::Num(l * r),
                    (Value::Float(l), Value::Float(r)) => Value::Float(l * r),
                    _ => panic!(
                        "Type mismatch in multiplication: {} * {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::Division(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (left_value.clone(), right_value.clone()) {
                    (Value::Num(l), Value::Num(r)) => {
                        if r == 0 {
                            panic!("Division by zero");
                        }
                        Value::Num(l / r)
                    }
                    (Value::Float(l), Value::Float(r)) => {
                        if r == 0.0 {
                            panic!("Division by zero");
                        }
                        Value::Float(l / r)
                    }
                    _ => panic!(
                        "Type mismatch in division: {} / {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            //later
            Expression::Nil => Value::Nil,
        }
    }
}
