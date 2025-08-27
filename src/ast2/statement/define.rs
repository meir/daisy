use super::{Result, Statement};
use crate::ast2::{
    environment::{Type, Value},
    expression::Expression,
};

pub fn define(t: Type, name: String, expression: Option<Expression>) -> Statement {
    Box::new(move |ctx, env| {
        let value = if let Some(expr) = &expression {
            expr(ctx, env)
        } else {
            Value::Nil
        };
        env.clone().define(name.as_str(), t.with_value(value));
        Result::NOP
    })
}
