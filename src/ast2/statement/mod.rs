use crate::context::Context;

use super::{environment::Value, Environment};

pub enum Result {
    Collect(Vec<Value>),
    Return(Value),
    Break,
    Continue,
    NOP,
}

pub type Statement = Box<dyn Fn(&Context, &Environment) -> Result + 'static>;
