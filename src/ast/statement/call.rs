use super::{Result, Statement};
use crate::ast::environment::Value;
use crate::ast::expression::Expression;
use crate::ast::function::call_function;

pub fn call(identifier: Expression, arguments: Vec<Expression>) -> Statement {
    Box::new(move |ctx, scope| {
        let function = identifier(ctx, scope);
        if let Value::Function(..) = function {
            call_function(ctx, &function, &arguments, scope);
        } else {
            panic!("Expected a function, got {}", function);
        };

        Result::NOP
    })
}
