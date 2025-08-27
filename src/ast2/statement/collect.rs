use super::{Result, Statement};
use crate::ast2::expression::Expression;

pub fn collect(expression: Expression) -> Statement {
    Box::new(move |ctx, env| {
        let value = expression(ctx, env);
        Result::Collect(vec![value])
    })
}
