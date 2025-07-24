use crate::ast::environment::Value;

use super::Expression;

pub fn identifier(location: Vec<String>) -> Expression {
    Box::new(move |_ctx, scope| {
        let first = location
            .first()
            .expect("Identifier must have at least one part");
        let mut value: Option<&Value> = scope.get(first);
        for part in location.iter().skip(1) {
            value = if let Some(Value::Map(map)) = value {
                map.get(&part)
            } else {
                None
            };
        }

        if let Some(value) = value {
            value.clone()
        } else {
            Value::Nil
        }
    })
}
