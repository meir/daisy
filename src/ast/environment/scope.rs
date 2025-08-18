use super::{Type, Value};
use crate::ast::statement::Statement;
use crate::context::Context;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Scope {
    variables: Vec<HashMap<String, (Type, Value)>>,
    current_scope: usize,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            variables: vec![HashMap::new()],
            current_scope: 0,
        }
    }

    #[allow(dead_code)]
    pub fn print_current_scope(&self) {
        println!("Current scope: {}", self.current_scope);
        for (name, value) in &self.variables[self.current_scope] {
            println!("{}: {} ({})", name, value.1, value.0);
        }
    }

    pub fn get_keys(&mut self) -> Vec<String> {
        self.variables[self.current_scope].keys().cloned().collect()
    }

    pub fn get_indices(&mut self) -> Vec<String> {
        let mut indices: Vec<String> = self.variables[self.current_scope]
            .keys()
            .filter_map(|key| key.parse::<i64>().ok().map(|i| i.to_string()))
            .collect();
        indices.sort_by_key(|k| k.parse::<i64>().unwrap_or(0));
        indices
    }

    pub fn array_push(&mut self, value: Value) {
        let index = self.get_indices().len();
        self.define(Type::Any, index.to_string(), value);
    }

    pub fn sync_scope(&mut self) {
        let length = self.variables.len();

        if self.current_scope >= length {
            for _ in length..=self.current_scope {
                self.variables.push(HashMap::new());
            }
        } else if self.current_scope < length - 1 {
            self.variables.truncate(self.current_scope + 1);
        }
    }

    pub fn wrap<T, F>(&mut self, lambda: F) -> T
    where
        F: FnOnce(&mut Scope) -> T,
    {
        self.current_scope += 1;
        self.sync_scope();
        let result: T = lambda(self);
        self.current_scope -= 1;
        self.sync_scope();
        return result;
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

    pub fn define(&mut self, type_: Type, name: String, value: Value) {
        if self.variables[self.current_scope].contains_key(&name) {
            panic!("Value {} already defined in this scope", name);
        }
        if !Type::matches(&type_, &value) {
            panic!("Type mismatch: expected {}, got {}", type_, value);
        }
        self.variables[self.current_scope].insert(name, (type_, value));
    }

    pub fn set_meta(&mut self, value: Value) {
        self.overwrite(Type::Map, "meta".into(), value);
    }

    pub fn get_meta(&self) -> Option<&Value> {
        self.get_from_scope("meta", self.current_scope)
    }

    pub fn overwrite(&mut self, type_: Type, name: String, value: Value) {
        if self.variables[self.current_scope].contains_key(&name) {
            self.variables[self.current_scope].remove(&name);
        }
        if !Type::matches(&type_, &value) {
            panic!("Type mismatch: expected {}, got {}", type_, value);
        }
        self.variables[self.current_scope].insert(name, (type_, value));
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.get_from_scope(name, self.current_scope)
    }

    fn get_from_scope(&self, name: &str, scope: usize) -> Option<&Value> {
        if let Some((_, var)) = self.variables[scope].get(name) {
            return Some(var);
        }
        if scope > 0 {
            return self.get_from_scope(name, scope - 1);
        }
        None
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.set_in_scope(name, value, self.current_scope);
    }

    fn set_in_scope(&mut self, name: String, value: Value, scope: usize) {
        if let Some((type_, var)) = self.variables[self.current_scope].get_mut(&name) {
            if !Type::matches(type_, &value) {
                panic!("Type mismatch: expected {}, got {}", type_, value);
            }
            var.set_value(value);
        } else if scope > 0 {
            return self.set_in_scope(name, value, scope - 1);
        } else {
            panic!("Value {} not found in any scope", name);
        }
    }
}
