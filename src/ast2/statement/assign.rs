use super::{Result, Statement};
use crate::ast2::expression::Expression;

pub fn assign(name: String, expression: Expression) -> Statement {
    Box::new(move |ctx, env| {
        let value = expression(ctx, env);
        env.clone().assign(name.as_str(), value.as_typevalue());
        Result::NOP
    })
}
