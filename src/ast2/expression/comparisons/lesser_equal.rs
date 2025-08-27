use crate::ast2::environment::Value;
use crate::ast2::expression::Expression;

pub fn lesser_equal(left: Expression, right: Expression) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        match (left_value, right_value) {
            (Value::Number(l), Value::Number(r)) => Value::Boolean(l <= r),
            (Value::String(l), Value::String(r)) => Value::Boolean(l <= r),
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(l <= r),
            _ => panic!("Cannot compare values of different types or unsupported types"),
        }
    })
}
