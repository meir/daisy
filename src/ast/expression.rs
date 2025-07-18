use std::process::Command;

use super::{
    environment::{Scope, Type},
    function::call_function,
    statement::Statement,
};
use crate::{ast::environment::Value, context::Context};

#[derive(Clone)]
pub enum Expression {
    Value(Value),
    Call(Box<Expression>, Vec<Expression>),
    Identifier(Vec<String>),
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
    Map(Vec<Statement>),
    Array(Vec<Box<Expression>>),
    For(String, Box<Expression>, Vec<Statement>),
    ForLoop(
        Box<Statement>,
        Box<Expression>,
        Box<Statement>,
        Vec<Statement>,
    ),
    Nil,
}

impl Expression {
    pub fn to_value(&self, ctx: &mut Context, scope: &mut Scope) -> Value {
        match self {
            Expression::Value(value) => match value {
                // to keep the scope that the current element is in so that the element can render
                // properly without missing variables
                Value::Element(element) => Value::ScopedElement(scope.clone(), element.clone()),
                Value::Function(func, body, return_type, params) => Value::ScopedFunction(
                    scope.clone(),
                    func.clone(),
                    body.clone(),
                    return_type.clone(),
                    params.clone(),
                ),
                _ => value.clone(),
            },
            Expression::Call(expr, args) => {
                let value = expr.to_value(ctx, scope);
                call_function(ctx, &value, args, scope)
            }
            Expression::Identifier(name) => {
                let map = Value::Map(scope.clone());
                let mut value: Option<&Value> = Some(&map);
                for part in name {
                    value = if let Some(Value::Map(map)) = value {
                        map.get(part)
                    } else {
                        None
                    };
                }

                if let Some(val) = value {
                    val.clone()
                } else {
                    Value::Nil
                }
            }
            Expression::Addition(left, right) => {
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
                Value::Bool(left_value == right_value)
            }
            Expression::NotEqual(left, right) => {
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
                Value::Bool(left_value != right_value)
            }
            Expression::Or(left, right) => {
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
                let left_value = left.to_value(ctx, scope);
                let right_value = right.to_value(ctx, scope);
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
            Expression::Map(statements) => {
                let mut map_scope = Scope::new();
                for statement in statements {
                    if let Statement::Definition(type_, name, expr) = statement {
                        let value = expr.to_value(ctx, scope);
                        map_scope.define(type_.clone(), name.clone(), value);
                    } else {
                        panic!("Only definitions are allowed in map expressions");
                    }
                }
                Value::Map(map_scope)
            }
            Expression::Array(expressions) => {
                let values: Vec<Value> = expressions
                    .iter()
                    .map(|expr| expr.to_value(ctx, scope))
                    .collect();
                let mut array_scope = Scope::new();
                for (i, value) in values.iter().enumerate() {
                    array_scope.define(value.get_type(), i.to_string(), value.clone());
                }
                array_scope.define_builtin_function(
                    "get".into(),
                    |_, _, inputs, scope| -> Value {
                        scope
                            .get(&inputs[0].to_string())
                            .cloned()
                            .unwrap_or(Value::Nil)
                    },
                    Type::Any,
                );

                Value::Array(array_scope)
            }
            Expression::For(id, iterable, statements) => {
                let iterable_value = iterable.to_value(ctx, scope);
                if let Value::Array(mut array_scope) = iterable_value {
                    scope.wrap(|inner_scope| {
                        let indices = array_scope.get_indices();
                        let mut output_array = Scope::new();

                        inner_scope.define(Type::Any, id.clone(), Value::Nil);
                        for i in indices {
                            let value = array_scope.get(&i).cloned().unwrap_or(Value::Nil);
                            inner_scope.set(id.clone(), value.clone());

                            let mut end_value: Option<Value> = None;
                            for statement in statements {
                                match statement.process(ctx, inner_scope) {
                                    Ok((true, value)) => return value,
                                    Ok((false, value)) => {
                                        if value.get_type() != Type::Nil {
                                            end_value = Some(value);
                                        }
                                    }
                                    Err(err) => panic!("Error processing for loop: {:?}", err),
                                }
                            }

                            if let Some(end_value) = end_value {
                                output_array.array_push(end_value);
                            }
                        }

                        if output_array.get_keys().is_empty() {
                            Value::Nil
                        } else {
                            Value::Array(output_array)
                        }
                    })
                } else {
                    panic!(
                        "Expected an array for for loop, got {}",
                        iterable_value.get_type()
                    );
                }
            }
            Expression::ForLoop(init, condition, increment, statements) => {
                scope.wrap(|inner_scope| {
                    init.process(ctx, inner_scope).unwrap();
                    let mut output_array = Scope::new();
                    loop {
                        let condition_value = condition.to_value(ctx, inner_scope);
                        if let Value::Bool(false) = condition_value {
                            break;
                        }

                        let mut end_value: Option<Value> = None;
                        for statement in statements {
                            match statement.process(ctx, inner_scope) {
                                Ok((true, value)) => return value,
                                Ok((false, value)) => {
                                    if value.get_type() != Type::Nil {
                                        end_value = Some(value);
                                    }
                                }
                                Err(err) => panic!("Error processing for loop: {:?}", err),
                            }
                        }

                        if let Some(end_value) = end_value {
                            output_array.array_push(end_value);
                        }

                        increment.process(ctx, inner_scope).unwrap();
                    }

                    if output_array.get_keys().is_empty() {
                        Value::Nil
                    } else {
                        Value::Array(output_array)
                    }
                })
            }
            //later
            Expression::Nil => Value::Nil,
        }
    }
}
