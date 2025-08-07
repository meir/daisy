use super::{Result, Statement};
use crate::ast::environment::{Type, Value};
use crate::ast::expression::Expression;

pub fn iter_statement(identifier: String, iterable: Expression, body: Vec<Statement>) -> Statement {
    Box::new(move |ctx, scope| {
        scope.wrap(|inner_scope| {
            let var = iterable(ctx, inner_scope);
            let indices = match var {
                Value::Array(mut list) => list.get_indices(),
                Value::Map(mut map) => map.get_keys(),
                _ => {
                    panic!("Expected an array or map, got {}", var);
                }
            };

            let mut collected_values = vec![];
            inner_scope.define(Type::Any, identifier.to_string(), Value::Nil);
            'mainloop: for index in indices {
                inner_scope.set(identifier.to_string(), Value::String(index));

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
