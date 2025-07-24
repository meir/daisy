use super::{Result, Statement};
use crate::ast::environment::Value;
use crate::ast::expression::Expression;

pub fn loop_statement(
    init: Statement,
    condition: Expression,
    increment: Statement,
    body: Vec<Statement>,
) -> Statement {
    Box::new(move |ctx, scope| {
        scope.wrap(|inner_scope| {
            init(ctx, inner_scope);

            let mut collected_values = vec![];
            'mainloop: loop {
                if let Value::Bool(false) = condition(ctx, inner_scope) {
                    break;
                }

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
                        _ => {
                            return result;
                        }
                    }

                    increment(ctx, inner_scope);
                }
            }

            if collected_values.is_empty() {
                Result::NOP
            } else {
                Result::Collect(collected_values)
            }
        })
    })
}
