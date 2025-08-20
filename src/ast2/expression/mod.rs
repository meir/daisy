use crate::context::Context;

use super::environment::Value;
use super::Environment;

pub type Expression = Box<dyn Fn(&Context, &Environment) -> Value + 'static>;
