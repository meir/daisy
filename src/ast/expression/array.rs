use crate::ast::environment::{Scope, Value};

use super::Expression;

pub fn array(items: Vec<Expression>) -> Expression {
    Box::new(move |ctx, scope| {
        let values: Vec<Value> = items.iter().map(|item| item(ctx, scope)).collect();
        let mut array = Scope::new();

        for value in values.into_iter() {
            array.array_push(value);
        }

        Value::Array(array)
    })
}
