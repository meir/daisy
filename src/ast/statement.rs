use std::fmt::Error;

use super::{
    environment::{Scope, Type, Value},
    expression::Expression,
    function::call_function,
};

#[derive(Clone)]
pub enum Statement {
    Call(String, Vec<Value>),
    Definition(Type, String, Expression),
    Assignment(String, Expression),
    Return(Expression),
    If(Expression, Vec<Statement>),
    For(String, Expression, Vec<Statement>),
    ForLoop(Box<Statement>, Expression, Expression, Vec<Statement>),
}

impl Statement {
    pub fn process(&self, scope: &mut Scope) -> Result<(bool, Value), Error> {
        match self {
            Statement::Call(name, args) => {
                let value = scope
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Function '{}' not defined", name));
                call_function(&value, args, scope);
                Ok((false, Value::Nil))
            }
            Statement::Definition(type_, name, expression) => {
                let value = expression.to_value(scope);
                scope.define(type_.clone(), name.clone(), value);
                Ok((false, Value::Nil))
            }
            Statement::Assignment(name, expression) => {
                let value = expression.to_value(scope);
                scope.set(name.clone(), value);
                Ok((false, Value::Nil))
            }
            Statement::Return(expression) => {
                let value = expression.to_value(scope);
                Ok((true, value))
            }
            Statement::If(condition, statements) => {
                let condition_value = condition.to_value(scope);
                match condition_value {
                    Value::Bool(true) => scope.wrap(|inner_scope| {
                        for stmt in statements {
                            match stmt.process(inner_scope) {
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
            Statement::For(variable, iterable, statements) => Ok((false, Value::Nil)),
            Statement::ForLoop(init, condition, increment, statements) => Ok((false, Value::Nil)),
        }
    }
}
