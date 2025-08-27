use crate::ast2::environment::Value;
use crate::ast2::expression::Expression;

pub fn call(value: Expression, arguments: Vec<Expression>) -> Expression {
    Box::new(move |ctx, env| {
        let function = value(ctx, env);
        match function {
            Value::Function(function) => function.call(ctx, env, &arguments),
            _ => panic!("Expected a function, got {}", function),
        }
    })
}
