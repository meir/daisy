use super::{environment::Value, Environment};
use crate::context::Context;
use crate::prelude::*;

inherit!(call);
inherit!(assign);
inherit!(define);
inherit!(collect);
inherits!(r#return, [return_s]);
inherits!(r#break, [break_s]);
inherits!(r#continue, [continue_s]);
inherits!(r#if, [if_s]);
inherits!(r#for, [for_s]);
inherit!(iter);

pub enum Result {
    Collect(Vec<Value>),
    Return(Value),
    Break,
    Continue,
    NOP,
}

pub type Statement = Box<dyn Fn(&Context, &Environment) -> Result + 'static>;
