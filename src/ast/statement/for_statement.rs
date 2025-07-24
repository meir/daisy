use super::{Result, Statement};
use crate::ast::environment::{Type, Value};
use crate::ast::expression::Expression;

pub fn for_statement(identifier: String, iterable: Expression, body: Vec<Statement>) -> Statement {
    Box::new(move |ctx, scope| {
        scope.wrap(|inner_scope| {
            let var = iterable(ctx, inner_scope);
            if let Value::Array(list) = var {
                let mut keys = list.clone().get_keys();
                keys.sort_by(|a, b| {
                    a.parse::<i64>()
                        .unwrap_or(0)
                        .cmp(&b.parse::<i64>().unwrap_or(0))
                });

                let mut collected_values = vec![];
                inner_scope.define(Type::Any, identifier.clone(), Value::Nil);
                'mainloop: for key in keys {
                    let item = list.get(&key).unwrap_or(&Value::Nil);
                    inner_scope.set(identifier.clone(), item.clone());

                    for stmt in body.iter() {
                        let result = stmt(ctx, inner_scope);
                        match result {
                            Result::Continue => {
                                continue 'mainloop;
                            }
                            Result::Collect(values) => {
                                collected_values.extend(values);
                            }
                            Result::NOP => {}
                            _ => return result,
                        }
                    }
                }

                if collected_values.is_empty() {
                    return Result::NOP;
                } else {
                    return Result::Collect(collected_values);
                }
            } else {
                panic!(
                    "Expected a list for 'for' loop variable, got {}",
                    var.get_type()
                );
            }
        })
    })
}
