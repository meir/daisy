use crate::ast::environment::{Scope, Value};
use crate::ast::statement::Statement;
use crate::context::Context;

pub fn builtin_replace(
    ctx: &mut Context,
    _: &Vec<Statement>,
    inputs: &Vec<Value>,
    scope: &mut Scope,
) -> Value {
    if inputs.len() != 3 {
        panic!("Expected 3 argument for 'replace', got {}", inputs.len());
    }

    if let Value::String(src) = &inputs[0] {
        let old = inputs[1].render(ctx, scope);
        let new = inputs[2].render(ctx, scope);

        Value::String(src.replace(&old, &new))
    } else {
        panic!(
            "Expected a string argument for 'replace', got {}",
            inputs[0].get_type()
        )
    }
}
