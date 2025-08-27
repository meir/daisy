use crate::ast2::environment::Value;
use crate::ast2::expression::Expression;

pub fn and(left: Expression, right: Expression) -> Expression {
    Box::new(move |ctx, scope| {
        let left_value = left(ctx, scope);
        let right_value = right(ctx, scope);

        match (&left_value, &right_value) {
            (Value::Boolean(l), Value::Boolean(r)) => Value::Boolean(*l && *r),
            _ => {
                panic!(
                    "Type mismatch in logical AND: {} && {}",
                    left_value, right_value
                );
            }
        }
    })
}
