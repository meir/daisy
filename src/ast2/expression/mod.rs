use super::environment::Value;
use super::Environment;
use crate::context::Context;
use crate::prelude::*;

inherits!(
    comparisons,
    [
        equal,
        notequal,
        or,
        and,
        lesser,
        lesser_equal,
        greater,
        greater_equal
    ]
);
inherits!(calculus, [add, subtract, multiply, divide]);
inherit!(call);
inherit!(identifier);
inherit!(list);
inherit!(map);
inherit!(value);
inherit!(script);
inherit!(object_entry);

pub type Expression = Box<dyn Fn(&Context, &Environment) -> Value + 'static>;
