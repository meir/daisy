use crate::context::Context;

use super::environment::{Scope, Value};

pub type Expression = Box<dyn Fn(&mut Context, &mut Scope) -> Value + 'static>;

pub mod addition;
pub mod and;
pub mod array;
pub mod call;
pub mod division;
pub mod equal;
pub mod greaterthan;
pub mod greaterthanorequal;
pub mod identifier;
pub mod lessthan;
pub mod lessthanorequal;
pub mod map;
pub mod multiplication;
pub mod notequal;
pub mod or;
pub mod scope_entry;
pub mod script;
pub mod subtraction;
pub mod value;

pub use addition::addition;
pub use and::and;
pub use array::array;
pub use call::call;
pub use division::division;
pub use equal::equal;
pub use greaterthan::greaterthan;
pub use greaterthanorequal::greaterthanorequal;
pub use identifier::identifier;
pub use lessthan::lessthan;
pub use lessthanorequal::lessthanorequal;
pub use map::map;
pub use multiplication::multiplication;
pub use notequal::notequal;
pub use or::or;
pub use scope_entry::scope_entry;
pub use script::script;
pub use subtraction::subtraction;
pub use value::value;
