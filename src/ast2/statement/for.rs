use super::{Result, Statement};
use crate::ast2::{environment::Value, expression::Expression};

pub fn for_s(
    init: Statement,
    condition: Expression,
    increment: Statement,
    body: Vec<Statement>,
) -> Statement {
    Box::new(move |ctx, env| {
        env.clone().subscope(|| {
            init(ctx, env);

            let mut collected_values = vec![];
            'mainloop: loop {
                if let Value::Boolean(false) = condition(ctx, env) {
                    break;
                }

                for stmt in body.iter() {
                    let result = stmt(ctx, env);
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

                    increment(ctx, env);
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
