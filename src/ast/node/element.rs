use super::Node;
use crate::ast::html::Element;

pub fn element(
    identifier: String,
    attributes: Vec<(String, Vec<Node>)>,
    children: Vec<Node>,
) -> Node {
    let element = Element::new(identifier, attributes, children);
    Box::new(move |ctx, scope| element.render(ctx, scope))
}
