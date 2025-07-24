use crate::ast::environment::{Scope, Type, Value};

use super::Expression;

pub fn array(items: Vec<Expression>) -> Expression {
    Box::new(move |ctx, scope| {
        let values: Vec<Value> = items.iter().map(|item| item(ctx, scope)).collect();
        let mut array = Scope::new();

        for value in values.into_iter() {
            array.array_push(value);
        }

        array.define_builtin_function(
            "get".into(),
            |_, _, inputs, scope| -> Value {
                scope
                    .get(&inputs[0].to_string())
                    .cloned()
                    .unwrap_or(Value::Nil)
            },
            Type::Any,
        );

        Value::Array(array)
    })
}
