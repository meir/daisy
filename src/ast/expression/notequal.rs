use crate::ast::environment::Value;

use super::Expression;

pub fn notequal(left: Box<Expression>, right: Box<Expression>) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        Value::Bool(left_value != right_value)
    })
}
