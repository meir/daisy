use super::{Result, Statement};
use crate::ast2::{environment::Value, expression::Expression};

pub fn if_s(condition: Expression, body: Vec<Statement>) -> Statement {
    Box::new(move |ctx, env| {
        let condition_value = condition(ctx, env);
        let mut collected_values = vec![];
        let mut result = Result::NOP;
        if let Value::Boolean(true) = condition_value {
            result = env.clone().subscope(|| {
                for stmt in body.iter() {
                    let result = stmt(ctx, env);
                    match result {
                        Result::Collect(values) => {
                            collected_values.extend(values);
                        }
                        Result::NOP => {}
                        _ => {
                            return result;
                        }
                    }
                }
                return Result::NOP;
            });
        }

        if collected_values.is_empty() {
            result
        } else {
            Result::Collect(collected_values)
        }
    })
}
