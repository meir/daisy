use super::Node;

pub fn text(text: String) -> Node {
    Box::new(move |_ctx, _env| text.to_string())
}
