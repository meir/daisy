use super::{
    environment::{Scope, Type},
    expression::Expression,
    function::call_function,
};
use crate::ast::environment::Value;

#[derive(Clone)]
pub enum Statement {
    Call(String, Vec<Value>),
    Definition(Type, String, Expression),
    Assignment(String, Expression),
    Return(Expression),
}

impl Statement {
    pub fn process(&self, scope: &mut Scope) {
        match self {
            Statement::Call(name, args) => {
                let value = scope
                    .get(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Function '{}' not defined", name));
                call_function(&value, args, scope);
            }
            Statement::Definition(type_, name, expression) => {
                let value = expression.to_value(scope);
                scope.define(type_.clone(), name.clone(), value);
            }
            Statement::Assignment(name, expression) => {
                let value = expression.to_value(scope);
                scope.set(name.clone(), value);
            }
            Statement::Return(expression) => {
                let _value = expression.to_value(scope);
            }
        }
    }
}
