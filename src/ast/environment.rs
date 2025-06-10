use super::statement::Statement;
use crate::ast::Node;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Clone)]
pub enum Value {
    Str(String),
    Num(i64),
    Float(f64),
    Bool(bool),
    Element(Box<Node>),
    ScopedElement(Scope, Box<Node>),
    Function(
        fn(&Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
        Vec<Statement>,
        Type,
        Vec<Statement>,
    ),
    Nil,
}

impl Value {
    pub fn render(&self, scope: &mut Scope) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Num(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Element(node) => node.render(scope),
            Value::ScopedElement(scope, element) => element.render(&mut scope.clone()),
            Value::Function(..) => todo!(),
            Value::Nil => "nil".to_string(),
        }
    }

    pub fn get_type(&self) -> Type {
        match &self {
            Value::Str(_) => Type::Str,
            Value::Num(_) => Type::Num,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
            Value::Element(_) => Type::Element,
            Value::ScopedElement(_, _) => Type::Element,
            Value::Function(..) => Type::Function,
            Value::Nil => Type::Nil,
        }
    }

    pub fn set_value(&mut self, value: Value) {
        self.clone_from(&value);
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Str(s) => write!(f, "str\"{}\"", s),
            Value::Num(n) => write!(f, "num({})", n),
            Value::Float(n) => write!(f, "float({})", n),
            Value::Bool(b) => write!(f, "bool({})", b),
            Value::Element(_) => write!(f, "element()"),
            Value::Function(..) => write!(f, "function()"),
            Value::ScopedElement(_, _) => write!(f, "scoped_element()"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Type {
    Str,
    Num,
    Float,
    Bool,
    Element,
    Function,
    Nil,
}

impl Type {
    pub fn matches(type_: &Type, value: &Value) -> bool {
        match (type_, value) {
            (Type::Str, Value::Str(_)) => true,
            (Type::Num, Value::Num(_)) => true,
            (Type::Float, Value::Float(_)) => true,
            (Type::Bool, Value::Bool(_)) => true,
            (Type::Element, Value::Element(_)) => true,
            (Type::Element, Value::ScopedElement(_, _)) => true,
            (Type::Function, Value::Function(..)) => true,
            (_, Value::Nil) => true,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Str => write!(f, "String"),
            Type::Num => write!(f, "Integer"),
            Type::Float => write!(f, "Float"),
            Type::Bool => write!(f, "Boolean"),
            Type::Element => write!(f, "Element"),
            Type::Function => write!(f, "Function"),
            Type::Nil => write!(f, "Nil"),
        }
    }
}

#[derive(Clone)]
pub struct Scope {
    variables: Vec<HashMap<String, (Type, Value)>>,
    current_scope: usize,
}

impl Scope {
    pub fn new() -> Self {
        let mut scope = Self {
            variables: vec![HashMap::new()],
            current_scope: 0,
        };

        scope.define_builtin_function(
            "hello_world".into(),
            |_, _, _| Value::Str("Hello world!".into()),
            Type::Str,
        );

        scope
    }

    #[allow(dead_code)]
    pub fn print_current_scope(&self) {
        for (name, value) in &self.variables[self.current_scope] {
            println!("{}: {} ({})", name, value.1, value.0);
        }
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
        return result;
    }

    pub fn define_builtin_function(
        &mut self,
        name: String,
        func: fn(&Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
        return_type: Type,
    ) {
        self.define(
            Type::Function,
            name,
            Value::Function(func, vec![], return_type, vec![]),
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

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.get_from_scope(name, self.current_scope)
    }

    fn get_from_scope(&self, name: &str, scope: usize) -> Option<&Value> {
        if let Some((_, var)) = self.variables[scope].get(name) {
            return Some(var);
        }
        if self.current_scope > 0 {
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
        } else if self.current_scope > 0 {
            return self.set_in_scope(name, value, scope - 1);
        } else {
            panic!("Value {} not found in any scope", name);
        }
    }
}
