use crate::context::Context;

use super::environment::{Scope, Value};

pub enum Result {
    Collect(Vec<Value>),
    Return(Value),
    Break,
    Continue,
    NOP,
}

pub type Statement = Box<dyn Fn(&mut Context, &mut Scope) -> Result + 'static>;

pub mod assign;
pub mod break_statement;
pub mod call;
pub mod collect;
pub mod continue_statement;
pub mod define;
pub mod for_statement;
pub mod if_statement;
pub mod loop_statement;
pub mod return_statement;

pub use assign::assign;
pub use break_statement::break_statement;
pub use call::call;
pub use collect::collect;
pub use continue_statement::continue_statement;
pub use define::define;
pub use for_statement::for_statement;
pub use if_statement::if_statement;
pub use loop_statement::loop_statement;
pub use return_statement::return_statement;
