use crate::context::Context;

use super::{
    environment::{Scope, Type, Value},
    expression::Expression,
    statement::{Result, Statement},
};

pub fn call_function(
    ctx: &mut Context,
    value: &Value,
    args: &Vec<Expression>,
    scope: &mut Scope,
) -> Value {
    if !Type::matches(&Type::Function, &value) {
        panic!("Expected a function, got {}", value.get_type());
    }

    let args: Vec<Value> = args.iter().map(|arg| arg(ctx, scope)).collect();

    match value {
        Value::Function(func, params, return_type, body) => {
            run_function(ctx, func, params, return_type, &args, body, scope)
        }
        Value::Scoped(scope, value) => {
            if let Value::Function(func, params, return_type, body) = value.as_ref() {
                // create a new scope for the function call
                let mut inner_scope = scope.clone();
                run_function(
                    ctx,
                    func,
                    params,
                    return_type,
                    &args,
                    body,
                    &mut inner_scope,
                )
            } else {
                panic!(
                    "Expected a function in scoped value, got {}",
                    value.get_type()
                );
            }
        }
        _ => Value::Nil,
    }
}

fn run_function(
    ctx: &mut Context,
    func: &fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
    params: &Vec<(Type, String, Option<Expression>)>,
    return_type: &Type,
    args: &Vec<Value>,
    body: &Vec<Statement>,
    scope: &mut Scope,
) -> Value {
    scope.wrap(|inner_scope| {
        for param in params {
            inner_scope.define(param.0.clone(), param.1.clone(), Value::Nil);
            if let Some(expr) = &param.2 {
                // If the parameter has a default value, evaluate it
                let default_value = expr(ctx, inner_scope);
                inner_scope.set(param.1.clone(), default_value);
            }
        }

        // set arguments in scope with variables given in call
        if params.len() >= args.len() {
            for i in 0..args.len() {
                let arg = args.get(i).unwrap();
                let param = params.get(i).unwrap();
                inner_scope.set(param.1.clone(), arg.clone());
            }
        }

        let return_value = func(ctx, &body, &args, inner_scope);

        if Type::matches(&return_type, &return_value) {
            return_value
        } else {
            panic!(
                "Type mismatch: expected {}, got {}",
                return_type,
                return_value.get_type()
            );
        }
    })
}

pub fn default_function(
    ctx: &mut Context,
    stmts: &Vec<Statement>,
    _: &Vec<Value>,
    scope: &mut Scope,
) -> Value {
    for stmt in stmts {
        match stmt(ctx, scope) {
            Result::Return(value) => {
                return value;
            }
            Result::Collect(value) => {
                let mut array = Scope::new();
                for val in value {
                    array.array_push(val);
                }
                return Value::Array(array);
            }
            Result::Break => {
                panic!("Break statement outside of loop");
            }
            Result::Continue => {
                panic!("Continue statement outside of loop");
            }
            Result::NOP => {
                // Do nothing, continue processing
            }
        }
    }
    Value::Nil
}
