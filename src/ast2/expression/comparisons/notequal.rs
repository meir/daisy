use crate::ast2::environment::Value;
use crate::ast2::expression::Expression;

pub fn notequal(left: Expression, right: Expression) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        Value::Boolean(left_value != right_value)
    })
}
