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
        }
    }
}
