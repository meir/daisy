use crate::ast::environment::Value;

use super::Expression;

pub fn or(left: Box<Expression>, right: Box<Expression>) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        match (&left_value, &right_value) {
            (Value::Bool(l), Value::Bool(r)) => Value::Bool(*l || *r),
            _ => panic!(
                "Type mismatch in logical OR: {} || {}",
                left_value.get_type(),
                right_value.get_type()
            ),
        }
    })
}
