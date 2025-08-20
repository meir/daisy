use std::{cell::RefCell, rc::Rc};

use super::CheckTypeValue;
use super::TypeValue;

pub type Stack = Rc<RefCell<Vec<TypeValue>>>;

pub trait UseStack {
    fn set(&mut self, index: usize, value: TypeValue);
    fn get(&self, index: usize) -> Option<TypeValue>;
    fn push(&mut self, value: TypeValue) -> usize;
    fn remove(&mut self, index: usize) -> Option<TypeValue>;
    fn clean(&mut self, in_use: Vec<usize>);
}

impl UseStack for Stack {
    fn set(&mut self, index: usize, value: TypeValue) {
        value.check();
        if index < self.borrow().len() {
            self.borrow_mut()[index] = value;
        } else {
            panic!(
                "Stack: Index out of bounds {} > {}",
                index,
                self.borrow().len()
            );
        }
    }

    fn get(&self, index: usize) -> Option<TypeValue> {
        if index < self.borrow().len() {
            Some(self.borrow()[index].clone())
        } else {
            None
        }
    }

    fn push(&mut self, value: TypeValue) -> usize {
        value.check();
        self.borrow_mut().push(value);
        self.borrow().len() - 1
    }

    fn remove(&mut self, index: usize) -> Option<TypeValue> {
        if index < self.borrow().len() {
            Some(self.borrow_mut().remove(index))
        } else {
            None
        }
    }

    fn clean(&mut self, in_use: Vec<usize>) {
        for i in (0..self.borrow().len()).rev() {
            if !in_use.contains(&i) {
                self.borrow_mut().remove(i);
            }
        }
    }
}
