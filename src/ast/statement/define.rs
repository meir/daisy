use super::{Result, Statement};
use crate::ast::environment::{Type, Value};
use crate::ast::expression::Expression;

pub fn define(type_: Type, name: String, expression: Option<Expression>) -> Statement {
    Box::new(move |ctx, scope| {
        let value = if let Some(expr) = &expression {
            expr(ctx, scope)
        } else {
            Value::Nil
        };
        scope.define(type_.clone(), name.clone(), value);
        Result::NOP
    })
}
