use super::{Result, Statement};
use crate::ast::expression::Expression;

pub fn assign(name: String, expression: Expression) -> Statement {
    Box::new(move |ctx, scope| {
        let value = expression(ctx, scope);
        scope.set(name.clone(), value);
        Result::NOP
    })
}
