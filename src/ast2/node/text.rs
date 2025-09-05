use std::sync::Arc;

use super::Node;

pub fn text(text: String) -> Node {
    Arc::new(move |_ctx, _env| text.to_string())
}
