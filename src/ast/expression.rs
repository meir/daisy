use std::process::Command;

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
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    LessThan(Box<Expression>, Box<Expression>),
    LessThanOrEqual(Box<Expression>, Box<Expression>),
    GreaterThan(Box<Expression>, Box<Expression>),
    GreaterThanOrEqual(Box<Expression>, Box<Expression>),
    Script(String),
    Nil,
}

impl Expression {
    pub fn to_value(&self, scope: &mut Scope) -> Value {
        match self {
            Expression::Value(value) => match value {
                // to keep the scope that the current element is in so that the element can render
                // properly without missing variables
                Value::Element(element) => Value::ScopedElement(scope.clone(), element.clone()),
                _ => value.clone(),
            },
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
            Expression::Script(script) => {
                let result = Command::new("bash").arg("-c").arg(script).output();
                match result {
                    Ok(output) => {
                        Value::Str(String::from_utf8_lossy(&output.stdout).trim().to_string())
                    }
                    Err(e) => panic!("Failed to execute script '{}': {}", script, e),
                }
            }
            Expression::Equal(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                Value::Bool(left_value == right_value)
            }
            Expression::NotEqual(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                Value::Bool(left_value != right_value)
            }
            Expression::Or(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (&left_value, &right_value) {
                    (Value::Bool(l), Value::Bool(r)) => Value::Bool(*l || *r),
                    _ => panic!(
                        "Type mismatch in logical OR: {} || {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::And(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (&left_value, &right_value) {
                    (Value::Bool(l), Value::Bool(r)) => Value::Bool(*l && *r),
                    _ => panic!(
                        "Type mismatch in logical AND: {} && {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::LessThan(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (&left_value, &right_value) {
                    (Value::Num(l), Value::Num(r)) => Value::Bool(l < r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l < r),
                    _ => panic!(
                        "Type mismatch in less than: {} < {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::LessThanOrEqual(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (&left_value, &right_value) {
                    (Value::Num(l), Value::Num(r)) => Value::Bool(l <= r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l <= r),
                    _ => panic!(
                        "Type mismatch in less than or equal: {} <= {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::GreaterThan(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (&left_value, &right_value) {
                    (Value::Num(l), Value::Num(r)) => Value::Bool(l > r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l > r),
                    _ => panic!(
                        "Type mismatch in greater than: {} > {}",
                        left_value.get_type(),
                        right_value.get_type()
                    ),
                }
            }
            Expression::GreaterThanOrEqual(left, right) => {
                let left_value = left.to_value(scope);
                let right_value = right.to_value(scope);
                match (&left_value, &right_value) {
                    (Value::Num(l), Value::Num(r)) => Value::Bool(l >= r),
                    (Value::Float(l), Value::Float(r)) => Value::Bool(l >= r),
                    _ => panic!(
                        "Type mismatch in greater than or equal: {} >= {}",
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
