use crate::context::Context;

use super::{
    environment::{Scope, Type, Value},
    expression::Expression,
    statement::Statement,
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

    let args: Vec<Value> = args.iter().map(|arg| arg.to_value(ctx, scope)).collect();

    match value {
        Value::Function(func, params, return_type, body) => {
            run_function(ctx, func, params, return_type, &args, body, scope)
        }
        Value::ScopedFunction(scope_func, func, params, return_type, body) => {
            // create a new scope for the function call
            let mut inner_scope = scope_func.clone();
            run_function(
                ctx,
                func,
                params,
                return_type,
                &args,
                body,
                &mut inner_scope,
            )
        }
        _ => Value::Nil,
    }
}

fn run_function(
    ctx: &mut Context,
    func: &fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
    params: &Vec<Statement>,
    return_type: &Type,
    args: &Vec<Value>,
    body: &Vec<Statement>,
    scope: &mut Scope,
) -> Value {
    scope.wrap(|inner_scope| {
        for param in params {
            param.process(ctx, inner_scope).unwrap();
        }

        // set arguments in scope with variables given in call
        if params.len() >= args.len() {
            for i in 0..args.len() {
                let arg = args.get(i).unwrap();
                let param = params.get(i).unwrap();
                match param {
                    Statement::Definition(_, name, _) => {
                        inner_scope.set(name.clone(), arg.clone());
                    }
                    _ => panic!("Expected a definition node",),
                }
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
        match stmt.process(ctx, scope) {
            Ok((true, value)) => {
                return value;
            }
            Ok((false, _)) => {}
            Err(err) => {
                panic!("Error processing statement: {:?}", err);
            }
        }
    }
    Value::Nil
}
