use crate::ast::str::Str;
use crate::ast::AST;
use crate::context::environment::Environment;
use crate::context::variable::Variable;
use crate::context::Context;

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: Str,
    pub value: Box<Variable>,
}

impl Definition {
    pub fn new(name: Str, value: Variable) -> Self {
        Definition {
            name,
            value: Box::new(value),
        }
    }
}

impl AST for Definition {
    fn str(&self, _ctx: &Context, scope: &mut Environment) -> String {
        scope.define(&self.name.literal, (*self.value).clone());
        "".to_string()
    }
}
