use crate::ast::environment::{Scope, Value};
use crate::ast::statement::Statement;
use crate::context::Context;

pub fn builtin_print(
    ctx: &mut Context,
    _: &Vec<Statement>,
    inputs: &Vec<Value>,
    scope: &mut Scope,
) -> Value {
    for input in inputs.iter() {
        let rendered = input.render(ctx, scope);
        print!("{}", rendered);
    }
    Value::Nil
}

pub fn builtin_println(
    ctx: &mut Context,
    _: &Vec<Statement>,
    inputs: &Vec<Value>,
    scope: &mut Scope,
) -> Value {
    for input in inputs.iter() {
        let rendered = input.render(ctx, scope);
        println!("{}", rendered);
    }
    Value::Nil
}
