use crate::ast::AST;
use crate::context::environment::Environment;
use crate::context::variable::Variable;

use crate::context::Context;

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct Statement {
    left: Box<Variable>,
    right: Box<Variable>,
    operator: Operator,
}

impl Statement {
    pub fn new(operator: Operator, left: Variable, right: Variable) -> Self {
        Statement {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        }
    }
}

impl AST for Statement {
    fn str(&self, _ctx: &Context, scope: &mut Environment) -> String {
        "".to_string()
    }
}
