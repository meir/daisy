use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type Scope = Rc<RefCell<HashMap<Box<str>, usize>>>;
pub type ScopeList = Rc<RefCell<Vec<Scope>>>;

pub trait UseScope {
    fn define(&mut self, name: &str, index: usize);
}

impl UseScope for Scope {
    fn define(&mut self, name: &str, index: usize) {
        if self.borrow().contains_key(name) {
            panic!("Scope: Name '{}' already defined in this scope.", name);
        }
        self.borrow_mut().insert(name.into(), index);
    }
}

pub trait QueryScopes {
    fn exists(&self, name: &str) -> Option<usize>;
}

impl QueryScopes for ScopeList {
    fn exists(&self, name: &str) -> Option<usize> {
        for scope in self.borrow().iter().rev() {
            if let Some(index) = scope.borrow().get(name) {
                return Some(*index);
            }
        }
        None
    }
}
