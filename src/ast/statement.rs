use crate::{
    ast::environment::{Type, Value},
    resolver::File,
};

use super::environment::Scope;

#[derive(Clone)]
pub enum Statement {
    Value(Value),
    Call(String, Vec<Value>),
}

impl Statement {
    pub fn to_value(&self, file: &mut File) -> Value {
        match self {
            Statement::Value(value) => value.clone(),
            Statement::Call(name, args) => {
                let value = file.environment.get(name).cloned();
                if let Some(value) = value {
                    if Type::matches(&Type::Function, &value) {
                        match value {
                            Value::Function(func) => {
                                let mut scope = Scope::new(Some(file.environment.clone()));
                                let return_value = func(&args, &mut scope);
                                return return_value;
                            }
                            _ => Value::Nil,
                        }
                    } else {
                        panic!("Expected a function, got {}", value.get_type());
                    }
                } else {
                    panic!("Function '{}' not defined", name);
                }
            }
        }
    }
}
