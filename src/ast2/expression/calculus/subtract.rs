use crate::ast2::environment::Value;
use crate::ast2::expression::Expression;

pub fn subtract(left: Expression, right: Expression) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        match (&left_value, &right_value) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
            (Value::String(l), Value::String(r)) => Value::String(format!("{}{}", l, r)),
            _ => panic!(
                "Type mismatch in addition: {} + {}",
                left_value, right_value
            ),
        }
    })
}
