use crate::ast::environment::Value;

use super::Expression;

pub fn subtraction(left: Box<Expression>, right: Box<Expression>) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        match (&left_value, &right_value) {
            (Value::Number(l), Value::Number(r)) => Value::Number(l - r),
            (Value::Float(l), Value::Float(r)) => Value::Float(l - r),
            _ => panic!(
                "Type mismatch in subtraction: {} - {}",
                left_value.get_type(),
                right_value.get_type()
            ),
        }
    })
}
