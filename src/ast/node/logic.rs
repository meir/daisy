use super::Node;
use crate::ast::expression::Expression;
use crate::ast::statement::{Result, Statement};

pub fn logic_statement(statement: Statement) -> Node {
    Box::new(move |ctx, scope| match statement(ctx, scope) {
        result => match result {
            Result::Collect(value) => {
                let mut output = String::new();
                for val in value {
                    output.push_str(&val.render(ctx, scope));
                }
                output
            }
            Result::Return(value) => value.render(ctx, scope),
            Result::Break | Result::Continue | Result::NOP => String::new(),
        },
    })
}

pub fn logic_expression(expression: Expression) -> Node {
    Box::new(move |ctx, scope| expression(ctx, scope).render(ctx, scope))
}
