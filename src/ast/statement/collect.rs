use super::{Result, Statement};
use crate::ast::expression::Expression;

pub fn collect(expression: Expression) -> Statement {
    Box::new(move |ctx, scope| {
        let value = expression(ctx, scope);
        println!("Collecting value: {}", value);

        Result::Collect(vec![value])
    })
}
