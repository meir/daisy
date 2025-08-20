pub mod environment;
pub mod expression;
pub mod functions;
pub mod node;
pub mod statement;

pub use environment::Environment;
pub use expression::Expression;
pub use node::Node;
pub use statement::Statement;

use crate::context::Context;

pub trait Build {
    fn build(&self, ctx: &Context, env: &Environment) -> String;
}
