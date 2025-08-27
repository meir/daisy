use crate::ast2::{
    environment::{Type, Value},
    Environment,
};

use super::Expression;

pub fn map(entries: Vec<(Type, String, Option<Expression>)>) -> Expression {
    Box::new(move |ctx, env| {
        let mut map = Environment::new();
        for entry in entries.iter() {
            let value = if let Some(expr) = &entry.2 {
                expr(ctx, env)
            } else {
                Value::Nil
            };
            map.define(entry.1.as_str(), entry.0.with_value(value));
        }

        Value::Map(map)
    })
}
