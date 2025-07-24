use super::{Result, Statement};

pub fn continue_statement() -> Statement {
    Box::new(move |_ctx, _scope| {
        // Continue does not return any value, it just continues to the next iteration
        Result::Continue
    })
}
