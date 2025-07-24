use crate::context::Context;

use super::environment::Scope;

pub type Node = Box<dyn Fn(&mut Context, &mut Scope) -> String + 'static>;

mod element;
mod insert;
mod logic;
mod text;

pub use element::element;
pub use insert::insert;
pub use logic::logic_expression;
pub use logic::logic_statement;
pub use text::text;
