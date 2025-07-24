use super::{Result, Statement};
use crate::ast::environment::Value;
use crate::ast::expression::Expression;
use crate::ast::function::call_function;

pub fn call(identifier: Expression, arguments: Vec<Expression>) -> Statement {
    Box::new(move |ctx, scope| {
        let binding = scope.clone();
        let function = identifier(ctx, scope);
        let function = if let Value::String(name) = function {
            name
        } else {
            panic!("Expected a function name as a string, got {}", function);
        };

        let value = binding
            .get(function.as_str())
            .unwrap_or_else(|| panic!("Function '{}' not defined", function));
        call_function(ctx, &value, &arguments, scope);
        Result::NOP
    })
}
