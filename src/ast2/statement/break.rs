use super::{Result, Statement};

pub fn break_s() -> Statement {
    Box::new(move |_ctx, _env| Result::Break)
}
