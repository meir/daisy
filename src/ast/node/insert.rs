use super::Node;
use crate::ast::expression::Expression;

pub fn insert(expr: Expression) -> Node {
    Box::new(move |ctx, scope| expr(ctx, scope).render(ctx, scope))
}
