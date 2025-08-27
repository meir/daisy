use std::rc::Rc;

use crate::ast2::environment::Value;

use super::Expression;

pub fn value(value: Value) -> Expression {
    let value = Rc::new(value);
    Box::new(move |_ctx, _env| {
        Rc::try_unwrap(value.clone())
            .unwrap_or_else(|_rc| panic!("Cannot extract value with multiple references"))
    })
}
