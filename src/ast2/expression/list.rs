use crate::ast2::environment::{Environment, Value};

use super::Expression;

pub fn list(items: Vec<Expression>) -> Expression {
    Box::new(move |ctx, env| {
        let values: Vec<Value> = items.iter().map(|item| item(ctx, env)).collect();
        let mut list = Environment::new();

        for (index, value) in values.into_iter().enumerate() {
            list.set(index.to_string().as_str(), value.as_typevalue());
        }

        Value::List(list)
    })
}
