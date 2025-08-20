use std::rc::Rc;

use crate::context::Context;

use super::{
    environment::{Type, Value},
    Environment, Expression, Statement,
};

pub type FunctionParameter = Rc<Vec<(Type, String, Option<Expression>)>>;
pub type FunctionBody = Rc<Vec<Statement>>;
pub type FunctionValue = (
    Box<dyn FunctionRunner>,
    Rc<Vec<(Type, String, Option<Expression>)>>,
    Type,
    FunctionBody,
);

trait FunctionRunner {
    fn run(
        &self,
        ctx: &Context,
        env: &Environment,
        parameters: Vec<Statement>,
        inputs: Vec<Value>,
    ) -> Value;
}
