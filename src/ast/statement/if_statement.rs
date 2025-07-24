use super::{Result, Statement};
use crate::ast::environment::Value;
use crate::ast::expression::Expression;

pub fn if_statement(condition: Expression, body: Vec<Statement>) -> Statement {
    Box::new(move |ctx, scope| {
        let condition_value = condition(ctx, scope);
        if let Value::Bool(true) = condition_value {
            scope.wrap(|inner_scope| {
                let mut collected_values = vec![];
                for stmt in body.iter() {
                    let result = stmt(ctx, inner_scope);
                    match result {
                        Result::Collect(values) => {
                            collected_values.extend(values);
                        }
                        _ => {
                            return result;
                        }
                    }
                }
                if collected_values.is_empty() {
                    Result::NOP
                } else {
                    Result::Collect(collected_values)
                }
            })
        } else if let Value::Bool(false) = condition_value {
            Result::NOP
        } else {
            panic!(
                "Expected a boolean condition for 'if', got {}",
                condition_value.get_type()
            );
        }
    })
}
