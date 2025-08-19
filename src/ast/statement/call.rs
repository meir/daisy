use super::{Result, Statement};
use crate::ast::environment::Value;
use crate::ast::expression::Expression;
use crate::ast::function::call_function;

pub fn call(identifier: Expression, arguments: Vec<Expression>) -> Statement {
    Box::new(move |ctx, scope| {
        let function = identifier(ctx, scope);
        match function {
            Value::Function(..) => {
                call_function(ctx, &function, &arguments, scope);
            }
            Value::Scoped(inner_scope, value) => {
                let mut inner_scope = inner_scope.clone();
                let (func, vars, return_type, args) = value.try_into_function().unwrap();
                call_function(
                    ctx,
                    &Value::Function(func, vars, return_type, args),
                    &arguments,
                    &mut inner_scope,
                );
            }
            _ => panic!("Expected a function, got {}", function.get_type()),
        }

        Result::NOP
    })
}
