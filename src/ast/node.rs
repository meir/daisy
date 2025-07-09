use super::environment::Scope;
use super::expression::Expression;
use super::html::Element;
use crate::context::Context;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Insertion(Expression),
}

impl Node {
    pub fn render(&self, ctx: &mut Context, scope: &mut Scope) -> String {
        match self {
            Node::Element(element) => element.render(ctx, scope),
            Node::Text(text) => text.clone(),
            Node::Insertion(expr) => expr.to_value(ctx, scope).render(ctx, scope),
        }
    }
}
