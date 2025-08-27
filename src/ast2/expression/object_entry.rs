use std::rc::Rc;

use crate::ast2::environment::Value;

use super::Expression;

pub fn object_entry(env_obj: Expression, entry: Expression) -> Expression {
    Box::new(move |ctx, env| {
        let env_value = env_obj(ctx, env);
        let key = entry(ctx, env);
        let env_ = match env_value {
            Value::Map(map_value) => map_value,
            Value::List(array_value) => array_value,
            _ => {
                panic!("Expected a map or array, got {}", env_value);
            }
        };

        let key = match key {
            Value::String(s) => s,
            Value::Number(n) => n.to_string(),
            _ => panic!("Expected a string or number as key, got {}", key),
        };
        if let Some(value) = env_.get(&key) {
            match Rc::try_unwrap(value) {
                Ok(ref_cell) => ref_cell.into_inner().1,
                Err(_rc) => {
                    panic!("Identifier value is still referenced elsewhere");
                }
            }
        } else {
            Value::Nil
        }
    })
}
