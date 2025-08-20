use super::{Type, Value};
use crate::ast::statement::Statement;
use crate::context::Context;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type StackValue = Rc<RefCell<(Type, Value)>>;
pub type Scope = Rc<RefCell<HashMap<String, usize>>>;
pub type SharedEnvironment = Rc<RefCell<Environment>>;

pub trait CallableSharedEnvironment {
    fn within_scope<T, F>(&mut self, lambda: F) -> T
    where
        F: FnOnce(&mut SharedEnvironment) -> T;
}

impl CallableSharedEnvironment for SharedEnvironment {
    fn within_scope<T, F>(&mut self, lambda: F) -> T
    where
        F: FnOnce(&mut SharedEnvironment) -> T,
    {
        let self_clone = self.clone();
        let mut env = self_clone.borrow_mut();
        env.increase_scope();
        let result: T = lambda(self);
        env.decrease_scope();
        return result;
    }
}

#[derive(Clone)]
pub struct Environment {
    stack: Vec<StackValue>,
    scopes: Vec<Scope>,
    current_scope: usize,
}

impl Environment {
    pub fn new() -> SharedEnvironment {
        Rc::new(RefCell::new(Environment {
            stack: Vec::new(),
            scopes: vec![Rc::new(RefCell::new(HashMap::new()))],
            current_scope: 0,
        }))
    }

    pub fn current_scope(&self) -> Rc<RefCell<HashMap<String, usize>>> {
        self.scopes[self.current_scope].clone()
    }

    pub fn get_stack_value(&self, index: usize) -> Option<Rc<RefCell<(Type, Value)>>> {
        self.stack.get(index).map(|s| s.clone()).or_else(|| None)
    }

    pub fn as_map(&mut self) -> HashMap<String, StackValue> {
        let mut map = HashMap::new();
        for scope in &self.scopes {
            for (name, index) in scope.borrow().iter() {
                if let Some(value) = self.get_stack_value(*index) {
                    map.insert(name.clone(), value);
                }
            }
        }
        map
    }

    pub fn as_vec(&mut self) -> Vec<StackValue> {
        self.as_map().into_iter().map(|(_, value)| value).collect()
    }

    pub fn set(&mut self, type_: Type, name: String, value: Value) {
        if !Type::matches(&type_, &value) {
            panic!("Type mismatch: expected {}, got {}", type_, value);
        }
        self.current_scope()
            .borrow_mut()
            .insert(name, self.stack.len());
    }

    pub fn exists(&mut self, name: &str) -> Option<usize> {
        for (_, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(index) = scope.borrow().get(name) {
                return Some(*index);
            }
        }
        None
    }

    pub fn define(&mut self, type_: Type, name: String, value: Value) {
        if self.current_scope().borrow().contains_key(&name) {
            panic!("Value {} already defined in this scope", name);
        }
        self.set(type_, name, value);
    }

    pub fn get(&mut self, name: &str) -> Option<StackValue> {
        if let Some(index) = self.exists(name) {
            return self.get_stack_value(index);
        }
        None
    }

    pub fn assign(&mut self, name: String, value: Value) {
        if let Some(stackvalue) = self.get(&name) {
            let (type_, mut var) = stackvalue.borrow_mut().clone();
            if !Type::matches(&type_, &value) {
                panic!("Type mismatch: expected {}, got {}", type_, value);
            }

            var.set_value(value);
        } else {
            panic!("Value {} not found in any scope", name);
        }
    }

    pub fn increase_scope(&mut self) {
        self.current_scope += 1;
        for _ in self.scopes.len()..=self.current_scope {
            self.scopes.push(Rc::new(RefCell::new(HashMap::new())));
        }
    }

    pub fn decrease_scope(&mut self) {
        if self.current_scope > 0 {
            self.current_scope -= 1;
            self.collect_garbage();
        } else {
            panic!("Cannot decrease scope below 0");
        }
    }

    pub fn collect_garbage(&mut self) {
        let length = self.scopes.len();

        if self.current_scope < length - 1 {
            self.scopes.truncate(self.current_scope + 1);
        }

        let in_use: Vec<usize> = self.current_scope().borrow().values().cloned().collect();
        for i in (0..self.stack.len()).rev() {
            if !in_use.contains(&i) {
                self.stack.remove(i);
            }
        }
    }

    pub fn define_builtin_function(
        &mut self,
        name: String,
        func: fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
        return_type: Type,
    ) {
        self.define(
            Type::Function,
            name,
            Value::Function(func, vec![].into(), return_type, vec![].into()),
        );
    }
}
