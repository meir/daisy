use crate::ast::environment::Value;

use super::Expression;

pub fn value(value: Value) -> Expression {
    Box::new(move |_ctx, scope| {
        match value {
            // to keep the scope that the current element is in so that the element can render
            // properly without missing variables
            Value::Element(..) => Value::Scoped(scope.clone(), Box::new(value.clone())),
            Value::Function(..) => Value::Scoped(scope.clone(), Box::new(value.clone())),
            _ => value.clone(),
        }
    })
}
