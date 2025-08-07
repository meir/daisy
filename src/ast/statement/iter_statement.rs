use super::{Result, Statement};
use crate::ast::environment::{Type, Value};
use crate::ast::expression::Expression;

pub fn iter_statement(
    identifiers: (String, Option<String>),
    iterable: Expression,
    body: Vec<Statement>,
) -> Statement {
    Box::new(move |ctx, scope| {
        scope.wrap(|inner_scope| {
            let (key_name, value_name) = identifiers.clone();
            let var = iterable(ctx, inner_scope);
            let (_scope, indices) = match var {
                Value::Array(mut list) => {
                    let indices = list.get_indices();
                    (list, indices)
                }
                Value::Map(mut map) => {
                    let keys = map.get_keys();
                    (map, keys)
                }
                _ => {
                    panic!("Expected an array or map, got {}", var);
                }
            };

            let mut collected_values = vec![];
            inner_scope.define(Type::Any, key_name.to_string(), Value::Nil);
            if let Some(value_name) = &value_name {
                inner_scope.define(Type::Any, value_name.to_string(), Value::Nil);
            }
            'mainloop: for index in indices {
                inner_scope.set(key_name.to_string(), Value::String(index.clone()));
                if let Some(value_name) = &value_name {
                    inner_scope.set(
                        value_name.to_string(),
                        _scope.get(&index).unwrap_or(&Value::Nil).clone(),
                    );
                }

                for statement in body.iter() {
                    let result = statement(ctx, inner_scope);
                    match result {
                        Result::Continue => {
                            continue 'mainloop;
                        }
                        Result::Collect(values) => {
                            collected_values.extend(values);
                        }
                        Result::NOP => {}
                        _ => {
                            return result;
                        }
                    }
                }
            }

            if collected_values.is_empty() {
                return Result::NOP;
            } else {
                return Result::Collect(collected_values);
            }
        })
    })
}
