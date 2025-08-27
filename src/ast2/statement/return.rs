use super::{Result, Statement};
use crate::ast2::expression::Expression;

pub fn return_s(expression: Option<Expression>) -> Statement {
    if let Some(expr) = expression {
        Box::new(move |ctx, env| {
            let value = expr(ctx, env);
            Result::Return(value)
        })
    } else {
        Box::new(|_ctx, _env| Result::Break)
    }
}
