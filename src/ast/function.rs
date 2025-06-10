use super::{
    environment::{Scope, Type, Value},
    statement::Statement,
};

pub fn call_function(value: &Value, args: &Vec<Value>, scope: &mut Scope) -> Value {
    if !Type::matches(&Type::Function, &value) {
        panic!("Expected a function, got {}", value.get_type());
    }

    match value {
        Value::Function(func, params, return_type, body) => {
            scope.wrap(|inner_scope| {
                for param in params {
                    param.process(inner_scope).unwrap();
                }

                // set arguments in scope with variables given in call
                for arg in args {
                    let param = params.get(0).unwrap();
                    match param {
                        Statement::Definition(_, name, _) => {
                            inner_scope.set(name.clone(), arg.clone());
                        }
                        _ => panic!("Expected a definition node",),
                    }
                }

                let return_value = func(&body, &args, inner_scope);

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
        _ => Value::Nil,
    }
}

pub fn default_function(stmts: &Vec<Statement>, _: &Vec<Value>, scope: &mut Scope) -> Value {
    for stmt in stmts {
        match stmt.process(scope) {
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
