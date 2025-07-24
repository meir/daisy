use crate::ast::environment::{Scope, Type, Value};

use super::Expression;

pub fn map(entries: Vec<(Type, String, Option<Expression>)>) -> Expression {
    Box::new(move |ctx, scope| {
        let mut map = Scope::new();
        for entry in entries.iter() {
            let value = if let Some(expr) = &entry.2 {
                expr(ctx, scope)
            } else {
                Value::Nil
            };
            map.define(entry.0.clone(), entry.1.clone(), value);
        }

        Value::Map(map)
    })
}
