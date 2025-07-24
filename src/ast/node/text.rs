use super::Node;

pub fn text(text: String) -> Node {
    Box::new(move |_ctx, _scope| text.clone())
}
