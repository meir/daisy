mod html;
mod insertion;
mod logic;
mod text;

pub use html::html;
pub use html::Html;
pub use insertion::insertion;
pub use logic::{logic_expression, logic_statement};
pub use text::text;

use crate::context::Context;

use super::Environment;

pub type Node = Box<dyn Fn(&Context, &Environment) -> String + 'static>;
