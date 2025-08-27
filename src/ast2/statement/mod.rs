use super::{environment::Value, Environment};
use crate::context::Context;
use crate::prelude::*;

inherit!(call);
inherit!(assign);
inherit!(define);

pub enum Result {
    Collect(Vec<Value>),
    Return(Value),
    Break,
    Continue,
    NOP,
}

pub type Statement = Box<dyn Fn(&Context, &Environment) -> Result + 'static>;
