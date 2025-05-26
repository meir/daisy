use crate::ast::AST;
use crate::context::environment::Environment;
use crate::context::variable::Variable;

use crate::context::Context;

#[derive(Debug, Clone)]
pub struct Insert {
    pub literal: String,
}

impl Insert {
    pub fn new(literal: String) -> Self {
        Insert { literal }
    }
}

impl AST for Insert {
    fn str(&self, ctx: &Context, scope: &mut Environment) -> String {
        let opt_var = scope.get(self.literal.clone());
        if let Some(var) = opt_var {
            match var {
                Variable::String(s) => s.to_string(),
                Variable::Number(n) => n.to_string(),
                Variable::Boolean(b) => b.to_string(),
                Variable::Float(f) => f.to_string(),
                Variable::Element(e) => e.clone().str(ctx, scope),
            }
        } else {
            "".to_string()
        }
    }
}
