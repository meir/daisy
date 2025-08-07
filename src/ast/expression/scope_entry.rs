use crate::ast::environment::Value;

use super::Expression;

pub fn scope_entry(scope_obj: Expression, entry: Expression) -> Expression {
    Box::new(move |ctx, scope| {
        let scope_value = scope_obj(ctx, scope);
        let key = entry(ctx, scope);
        let scope_ = match scope_value {
            Value::Map(map_value) => map_value,
            Value::Array(array_value) => array_value,
            _ => {
                panic!("Expected a map or array, got {}", scope_value);
            }
        };

        let key = match key {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            _ => panic!("Expected a string or number as key, got {}", key),
        };
        if let Some(value) = scope_.get(&key) {
            value.clone()
        } else {
            Value::Nil
        }
    })
}
