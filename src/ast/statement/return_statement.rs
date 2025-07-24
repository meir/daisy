use super::{Result, Statement};
use crate::ast::expression::Expression;

pub fn return_statement(expression: Option<Expression>) -> Statement {
    if let Some(expr) = expression {
        Box::new(move |ctx, scope| {
            let value = expr(ctx, scope);
            Result::Return(value)
        })
    } else {
        Box::new(|_ctx, _scope| Result::Break)
    }
}
