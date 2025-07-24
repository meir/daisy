use crate::ast::function::call_function;

use super::Expression;

pub fn call(identifier: Box<Expression>, args: Vec<Expression>) -> Expression {
    Box::new(move |ctx, scope| {
        let value = identifier(ctx, scope);
        call_function(ctx, &value, &args, scope)
    })
}
