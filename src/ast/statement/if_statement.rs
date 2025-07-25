use super::{Result, Statement};
use crate::ast::environment::Value;
use crate::ast::expression::Expression;

pub fn if_statement(condition: Expression, body: Vec<Statement>) -> Statement {
    Box::new(move |ctx, scope| {
        let condition_value = condition(ctx, scope);
        let mut collected_values = vec![];
        let mut result = Result::NOP;
        if let Value::Bool(true) = condition_value {
            result = scope.wrap(|inner_scope| {
                for stmt in body.iter() {
                    let result = stmt(ctx, inner_scope);
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
