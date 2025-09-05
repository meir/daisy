use std::{cell::RefCell, rc::Rc};

use crate::context::Context;

use super::{
    environment::{Type, Value},
    statement::Result,
    Environment, Expression, Statement,
};

pub type FunctionParameter = Rc<Vec<(Type, String, Option<Expression>)>>;
pub type FunctionBody = Rc<Vec<Statement>>;
pub type FunctionValue = (
    Rc<Vec<(Type, String, Option<Expression>)>>,
    Type,
    FunctionBody,
);

pub trait FunctionRunner: FunctionRunnerClone {
    fn call(&self, ctx: &Context, env: &Environment, inputs: &Vec<Expression>) -> Value;
}

pub trait FunctionRunnerClone {
    fn clone_box(&self) -> Box<dyn FunctionRunner>;
}

impl<T> FunctionRunnerClone for T
where
    T: FunctionRunner + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn FunctionRunner> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FunctionRunner> {
    fn clone(&self) -> Box<dyn FunctionRunner> {
        self.as_ref().clone_box()
    }
}

impl FunctionRunner for FunctionValue {
    fn call(&self, ctx: &Context, env: &Environment, inputs: &Vec<Expression>) -> Value {
        let (parameters, return_type, body) = self;

        // init params and override with inputs
        for (i, param) in parameters.iter().enumerate() {
            let (param_type, param_name, default) = param;
            let input = inputs.get(i);
            let value = if let Some(input) = input {
                input(ctx, env)
            } else if let Some(default) = default {
                default(ctx, env)
            } else {
                Value::Nil
            };

            env.clone().set(
                param_name,
                Rc::new(RefCell::new((param_type.clone(), value))),
            );
        }

        env.clone().subscope(|| {
            for statement in body.iter() {
                let value = match statement(ctx, env) {
                    Result::Return(value) => Some(value),
                    Result::Collect(value) => {
                        let list = Environment::new();
                        for _ in value {
                            todo!()
                        }
                        Some(Value::List(list))
                    }
                    Result::NOP => None,
                    _ => panic!("Unexpected result from statement"),
                };

                if let Some(value) = value {
                    if return_type.matches(&value) {
                        return value;
                    } else {
                        panic!("Type mismatch: expected {}, got {}", return_type, value);
                    }
                }
            }

            Value::Nil
        })
    }
}
