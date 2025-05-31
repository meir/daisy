use crate::{
    ast::environment::{Type, Value},
    resolver::File,
};

use super::environment::Scope;

#[derive(Clone)]
pub enum Statement {
    Value(Value),
    Call(String, Vec<Value>),
    Return(Box<Statement>),
    Nil,
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
                            Value::Function(func, params, return_type, body) => {
                                let mut scope = Scope::new(Some(file.environment.clone()));
                                for param in params {
                                    param.render(file);
                                }
                                let return_value = func(&body, &args, &mut scope);

                                if Type::matches(&return_type, &return_value) {
                                    return_value
                                } else {
                                    panic!(
                                        "Type mismatch: expected {}, got {}",
                                        return_type,
                                        return_value.get_type()
                                    );
                                }
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
            Statement::Return(inner) => inner.to_value(file), //Figure out how to differentiate
            //later
            Statement::Nil => Value::Nil,
        }
    }
}
