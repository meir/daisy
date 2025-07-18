use std::fmt::Error;

use crate::context::Context;

use super::{
    environment::{Scope, Type, Value},
    expression::Expression,
    function::call_function,
};

#[derive(Clone)]
pub enum Statement {
    Collect(Expression),
    Call(String, Vec<Expression>),
    Definition(Type, String, Expression),
    Assignment(String, Expression),
    Return(Expression),
    If(Expression, Vec<Statement>),
    For(String, Expression, Vec<Statement>),
    ForLoop(Box<Statement>, Expression, Box<Statement>, Vec<Statement>),
}

impl Statement {
    pub fn process(&self, ctx: &mut Context, scope: &mut Scope) -> Result<(bool, Value), Error> {
        match self {
            Statement::Collect(expression) => {
                let value = expression.to_value(ctx, scope);
                if value.get_type() == Type::Nil {
                    Ok((false, Value::Nil))
                } else {
                    Ok((false, value))
                }
            }
            Statement::Call(name, args) => {
                let value = scope
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Function '{}' not defined", name));
                call_function(ctx, &value, args, scope);
                Ok((false, Value::Nil))
            }
            Statement::Definition(type_, name, expression) => {
                let value = expression.to_value(ctx, scope);
                scope.define(type_.clone(), name.clone(), value);
                Ok((false, Value::Nil))
            }
            Statement::Assignment(name, expression) => {
                let value = expression.to_value(ctx, scope);
                scope.set(name.clone(), value);
                Ok((false, Value::Nil))
            }
            Statement::Return(expression) => {
                let value = expression.to_value(ctx, scope);
                Ok((true, value))
            }
            Statement::If(condition, statements) => {
                let condition_value = condition.to_value(ctx, scope);
                match condition_value {
                    Value::Bool(true) => scope.wrap(|inner_scope| {
                        for stmt in statements {
                            match stmt.process(ctx, inner_scope) {
                                Ok((true, value)) => {
                                    return Ok((true, value));
                                }
                                Ok((false, _)) => {}
                                Err(err) => {
                                    panic!("Error processing statement: {:?}", err);
                                }
                            }
                        }
                        Ok((false, Value::Nil))
                    }),
                    Value::Bool(false) => Ok((false, Value::Nil)),
                    _ => panic!(
                        "Condition in if statement must be a boolean, got {}",
                        condition_value.get_type()
                    ),
                }
            }
            Statement::For(id, var, statements) => scope.wrap(|inner_scope| {
                let var = var.to_value(ctx, inner_scope);
                if let Value::Array(list) = var {
                    let mut keys = list.clone().get_keys();
                    keys.sort_by(|a, b| {
                        a.parse::<i64>()
                            .unwrap_or(0)
                            .cmp(&b.parse::<i64>().unwrap_or(0))
                    });

                    for key in keys {
                        let item = list.get(&key).cloned().unwrap_or(Value::Nil);
                        inner_scope.set(id.clone(), item);

                        for stmt in statements {
                            match stmt.process(ctx, inner_scope) {
                                Ok((true, value)) => {
                                    return Ok((true, value));
                                }
                                Ok((false, _)) => {}
                                Err(err) => {
                                    panic!("Error processing statement: {:?}", err);
                                }
                            }
                        }
                    }
                } else {
                    panic!(
                        "Expected a list for 'for' loop variable, got {}",
                        var.get_type()
                    );
                }
                Ok((false, Value::Nil))
            }),
            Statement::ForLoop(init, condition, increment, statements) => {
                scope.wrap(|inner_scope| {
                    init.process(ctx, inner_scope)?;

                    loop {
                        if let Value::Bool(false) = condition.to_value(ctx, inner_scope) {
                            break;
                        }

                        for stmt in statements {
                            match stmt.process(ctx, inner_scope) {
                                Ok((true, value)) => {
                                    return Ok((true, value));
                                }
                                Ok((false, _)) => {}
                                Err(err) => {
                                    panic!("Error processing statement: {:?}", err);
                                }
                            }

                            increment.process(ctx, inner_scope)?;
                        }
                    }
                    Ok((false, Value::Nil))
                })
            }
        }
    }
}
