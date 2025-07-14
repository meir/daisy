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

    if let Value::Str(src) = &inputs[0] {
        let mut src = src.clone();
        for i in 1..inputs.len() {
            let value = inputs.get(i).unwrap();
            let str = value.render(ctx, scope);
            src = src.replacen("{}", str.as_str(), 1);
        }
        Value::Str(src)
    } else {
        panic!(
            "Expected a string argument for 'format', got {}",
            inputs[0].get_type()
        )
    }
}
