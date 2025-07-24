use super::{Result, Statement};

pub fn break_statement() -> Statement {
    Box::new(move |_ctx, _scope| {
        // Continue does not return any value, breaks the loop
        Result::Break
    })
}
