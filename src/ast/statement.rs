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
    Continue,
    Break,
    Call(String, Vec<Expression>),
    Definition(Type, String, Expression),
    Assignment(String, Expression),
    Return(Expression),
    If(Expression, Vec<Statement>),
    For(String, Expression, Vec<Statement>),
    ForLoop(Box<Statement>, Expression, Box<Statement>, Vec<Statement>),
}

pub enum ResultType {
    Collect(Vec<Value>),
    Return(Value),
    Break,
    Continue,
    NOP,
}

impl Statement {
    pub fn process(&self, ctx: &mut Context, scope: &mut Scope) -> Result<ResultType, Error> {
        match self {
            Statement::Continue => Ok(ResultType::Continue),
            Statement::Break => Ok(ResultType::Break),
            Statement::Collect(expression) => {
                let value = expression.to_value(ctx, scope);
                Ok(ResultType::Collect(vec![value]))
            }
            Statement::Call(name, args) => {
                let value = scope
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Function '{}' not defined", name));
                call_function(ctx, &value, args, scope);
                Ok(ResultType::NOP)
            }
            Statement::Definition(type_, name, expression) => {
                let value = expression.to_value(ctx, scope);
                scope.define(type_.clone(), name.clone(), value);
                Ok(ResultType::NOP)
            }
            Statement::Assignment(name, expression) => {
                let value = expression.to_value(ctx, scope);
                scope.set(name.clone(), value);
                Ok(ResultType::NOP)
            }
            Statement::Return(expression) => {
                let value = expression.to_value(ctx, scope);
                Ok(ResultType::Return(value))
            }
            Statement::If(condition, statements) => {
                let condition_value = condition.to_value(ctx, scope);
                if let Value::Bool(true) = condition_value {
                    scope.wrap(|inner_scope| {
                        let mut collected_values = vec![];
                        for stmt in statements {
                            let result = stmt.process(ctx, inner_scope);
                            match result {
                                Ok(ResultType::Return(value)) => {
                                    return Ok(ResultType::Return(value));
                                }
                                Ok(ResultType::Break) => {
                                    return Ok(ResultType::Break);
                                }
                                Ok(ResultType::Continue) => {
                                    return Ok(ResultType::Continue);
                                }
                                Ok(ResultType::Collect(values)) => {
                                    collected_values.extend(values);
                                }
                                _ => {
                                    // Continue processing statements
                                }
                            }
                        }
                        if collected_values.is_empty() {
                            Ok(ResultType::NOP)
                        } else {
                            Ok(ResultType::Collect(collected_values))
                        }
                    })
                } else if let Value::Bool(false) = condition_value {
                    Ok(ResultType::NOP)
                } else {
                    panic!(
                        "Expected a boolean condition for 'if', got {}",
                        condition_value.get_type()
                    );
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

                    let mut collected_values = vec![];
                    inner_scope.define(Type::Any, id.clone(), Value::Nil);
                    'mainloop: for key in keys {
                        let item = list.get(&key).cloned().unwrap_or(Value::Nil);
                        inner_scope.set(id.clone(), item);

                        for stmt in statements {
                            match stmt.process(ctx, inner_scope) {
                                Ok(ResultType::Return(value)) => {
                                    return Ok(ResultType::Return(value));
                                }
                                Ok(ResultType::Break) => {
                                    return Ok(ResultType::Break);
                                }
                                Ok(ResultType::Continue) => {
                                    continue 'mainloop;
                                }
                                Ok(ResultType::Collect(values)) => {
                                    collected_values.extend(values);
                                }
                                Err(err) => {
                                    panic!("Error processing statement: {:?}", err);
                                }
                                _ => {}
                            }
                        }
                    }

                    if collected_values.is_empty() {
                        return Ok(ResultType::NOP);
                    } else {
                        return Ok(ResultType::Collect(collected_values));
                    }
                } else {
                    panic!(
                        "Expected a list for 'for' loop variable, got {}",
                        var.get_type()
                    );
                }
            }),
            Statement::ForLoop(init, condition, increment, statements) => {
                scope.wrap(|inner_scope| {
                    init.process(ctx, inner_scope)?;

                    let mut collected_values = vec![];
                    'mainloop: loop {
                        if let Value::Bool(false) = condition.to_value(ctx, inner_scope) {
                            break;
                        }

                        for stmt in statements {
                            match stmt.process(ctx, inner_scope) {
                                Ok(ResultType::Return(value)) => {
                                    return Ok(ResultType::Return(value));
                                }
                                Ok(ResultType::Break) => {
                                    return Ok(ResultType::Break);
                                }
                                Ok(ResultType::Continue) => {
                                    continue 'mainloop;
                                }
                                Ok(ResultType::Collect(values)) => {
                                    collected_values.extend(values);
                                }
                                Err(err) => {
                                    panic!("Error processing statement: {:?}", err);
                                }
                                _ => {}
                            }

                            increment.process(ctx, inner_scope)?;
                        }
                    }

                    if collected_values.is_empty() {
                        Ok(ResultType::NOP)
                    } else {
                        Ok(ResultType::Collect(collected_values))
                    }
                })
            }
        }
    }
}
