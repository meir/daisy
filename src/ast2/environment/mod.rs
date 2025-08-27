use super::Build;
use crate::context::Context;
use crate::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

inherits!(r#type, [Type]);
inherits!(value, [Value, TypeValue, CheckTypeValue]);

mod scope;
use scope::QueryScopes;
use scope::Scope;
use scope::ScopeList;
use scope::UseScope;

mod stack;
use stack::Stack;
use stack::UseStack;

#[derive(Clone)]
pub struct Environment {
    stack: Stack,
    scopes: ScopeList,
    current_scope: usize,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            stack: Rc::new(RefCell::new(Vec::new())),
            scopes: Rc::new(RefCell::new(vec![Rc::new(RefCell::new(HashMap::new()))])),
            current_scope: 0,
        }
    }

    pub fn current_scope(&self) -> Scope {
        self.scopes.borrow()[self.current_scope].clone()
    }

    pub fn increase_scope(&mut self) {
        self.scopes
            .borrow_mut()
            .push(Rc::new(RefCell::new(HashMap::new())));
        self.current_scope += 1;
    }

    pub fn decrease_scope(&mut self) {
        if self.current_scope > 0 {
            self.scopes.borrow_mut().pop();
            self.current_scope -= 1;
            self.clean();
        } else {
            panic!("Cannot decrease scope, already at the global scope.");
        }
    }

    pub fn subscope<T, F>(&mut self, lambda: F) -> T
    where
        F: FnOnce() -> T,
    {
        self.increase_scope();
        let result: T = lambda();
        self.decrease_scope();
        result
    }

    pub fn clean(&mut self) {
        let in_use: Vec<usize> = self
            .scopes
            .borrow()
            .iter()
            .flat_map(|scope| scope.borrow().values().cloned().collect::<Vec<usize>>())
            .clone()
            .collect();
        self.stack.clean(in_use);
    }

    pub fn define(&mut self, name: &str, value: TypeValue) {
        let index = self.stack.push(value);
        self.current_scope().define(name, index);
    }

    pub fn set(&mut self, name: &str, value: TypeValue) {
        let index = self.scopes.exists(name);
        if let Some(index) = index {
            self.stack.set(index, value);
        } else {
            self.define(name, value);
        }
    }

    pub fn assign(&mut self, name: &str, value: TypeValue) {
        let index = self.scopes.exists(name).expect(&format!(
            "Variable '{}' not defined in current scope.",
            name
        ));
        self.stack.set(index, value);
    }

    pub fn get(&self, name: &str) -> Option<TypeValue> {
        if let Some(index) = self.scopes.exists(name) {
            return self.stack.get(index);
        }
        None
    }

    pub fn as_map(&self) -> HashMap<String, TypeValue> {
        let mut map = HashMap::new();
        for (name, index) in self.current_scope().borrow().iter() {
            if let Some(value) = self.stack.get(*index) {
                map.insert(name.to_string(), value);
            }
        }
        map
    }

    pub fn as_vec(&self) -> Vec<TypeValue> {
        self.as_map().into_values().collect()
    }
}

impl Build for Environment {
    fn build(&self, ctx: &Context, env: &Environment) -> String {
        self.as_vec()
            .iter()
            .map(|typevalue| {
                let typevalue = typevalue.borrow();
                typevalue.1.build(ctx, env)
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
