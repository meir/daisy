use crate::ast2::environment::Value;

use super::Expression;

pub fn value(value: Value) -> Expression {
    Box::new(move |_ctx, _env| value.clone())
}
