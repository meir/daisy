use super::Environment;
use crate::context::Context;
use crate::prelude::*;

inherits!(html, [html, Html]);
inherit!(insertion);
inherits!(logic, [logic_expression, logic_statement]);
inherit!(text);

pub type Node = Box<dyn Fn(&Context, &Environment) -> String + 'static>;
