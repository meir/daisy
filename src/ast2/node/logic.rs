use std::sync::Arc;

use crate::ast2::statement::Result;
use crate::ast2::{Build, Expression, Statement};

use super::Node;

pub fn logic_statement(logic: Statement) -> Node {
    Arc::new(move |ctx, env| match logic(ctx, env) {
        Result::Collect(value) => value
            .iter()
            .map(|v| v.build(ctx, env))
            .collect::<Vec<String>>()
            .join(""),
        Result::Return(value) => value.build(ctx, env),
        Result::Break | Result::Continue | Result::NOP => String::new(),
    })
}

pub fn logic_expression(logic: Expression) -> Node {
    Arc::new(move |ctx, env| logic(ctx, env).build(ctx, env))
}
