use std::sync::Arc;

use crate::ast2::Build;
use crate::ast2::Expression;

use super::Node;

pub fn insertion(expr: Expression) -> Node {
    Arc::new(move |ctx, env| expr(ctx, env).build(ctx, env))
}
