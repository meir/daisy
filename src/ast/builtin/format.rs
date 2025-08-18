use crate::ast::environment::{Scope, Value};
use crate::ast::statement::Statement;
use crate::context::Context;

pub fn builtin_format(
    ctx: &mut Context,
    _: &Vec<Statement>,
    inputs: &Vec<Value>,
    scope: &mut Scope,
) -> Value {
    if inputs.len() == 0 {
        panic!(
            "Expected atleast one argument for 'format', got {}",
            inputs.len()
        );
    }

    if let Value::String(src) = &inputs[0] {
        let mut src = src.clone();
        for i in 1..inputs.len() {
            let value = inputs.get(i).unwrap();
            match value.clone() {
                Value::String(str) => src = src.replacen("{}", str.as_str(), 1),
                Value::Number(num) => src = src.replacen("{}", &num.to_string(), 1),
                Value::Bool(bool) => src = src.replacen("{}", &bool.to_string(), 1),
                Value::Map(mut map) => {
                    let keys = map.get_keys();
                    for key in keys {
                        if let Some(val) = map.get(&key) {
                            src = src.replace(&format!("{{{}}}", key), &val.render(ctx, scope));
                        } else {
                            panic!("Key '{}' not found in map", key);
                        }
                    }
                }
                _ => panic!("Unsupported type for 'format': {}", value.get_type()),
            }
        }
        Value::String(src)
    } else {
        panic!(
            "Expected a string argument for 'format', got {}",
            inputs[0].get_type()
        )
    }
}
