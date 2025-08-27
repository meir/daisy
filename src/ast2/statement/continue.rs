use super::{Result, Statement};

pub fn continue_s() -> Statement {
    Box::new(move |_ctx, _env| Result::Continue)
}
