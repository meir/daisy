use super::statement::Statement;
use crate::ast::node::Node;
use crate::context::Context;
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
        fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
        Vec<Statement>,
        Type,
        Vec<Statement>,
    ),
    ScopedFunction(
        Scope,
        fn(&mut Context, &Vec<Statement>, &Vec<Value>, &mut Scope) -> Value,
        Vec<Statement>,
        Type,
        Vec<Statement>,
    ),
    Map(Scope),
    Array(Scope),
    Nil,
}

impl Value {
    pub fn render(&self, ctx: &mut Context, scope: &mut Scope) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Num(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Element(node) => node.render(ctx, scope),
            Value::ScopedElement(scope, element) => element.render(ctx, &mut scope.clone()),
            Value::Function(..) => "".into(),
            Value::ScopedFunction(..) => "".into(),
            Value::Map(scope) => {
                let mut scope = scope.clone();
                let keys = &scope.get_keys();
                let mut output = String::new();
                for key in keys {
                    if let Some(value) = scope.clone().get(&key) {
                        output.push_str(&value.render(ctx, &mut scope))
                    }
                }
                output
            }
            Value::Array(scope) => {
                let mut scope = scope.clone();
                let mut output = String::new();

                for key in scope.get_indices() {
                    if let Some(value) = scope.clone().get(&key) {
                        output.push_str(&value.render(ctx, &mut scope));
                    }
                }
                output
            }
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
            Value::ScopedFunction(..) => Type::Function,
            Value::Map(..) => Type::Map,
            Value::Array(..) => Type::Array,
            Value::Nil => Type::Nil,
        }
    }

    pub fn set_value(&mut self, value: Value) {
        self.clone_from(&value);
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Str(l), Value::Str(r)) => l == r,
            (Value::Num(l), Value::Num(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Nil, Value::Nil) => true,

            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
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
            Value::ScopedElement(_, _) => write!(f, "scoped_element()"),
            Value::Function(..) => write!(f, "function()"),
            Value::ScopedFunction(..) => write!(f, "scoped_function()"),
            Value::Map(..) => write!(f, "Map()"),
            Value::Array(..) => write!(f, "Array()"),
            Value::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Type {
    Str,
    Num,
    Float,
    Bool,
    Element,
    Function,
    Map,
    Array,
    Nil,
    Any, // not usable in the language, but needed to return any type using "use"
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
            (Type::Function, Value::ScopedFunction(..)) => true,
            (Type::Map, Value::Map(_)) => true,
            (Type::Array, Value::Array(_)) => true,
            (_, Value::Nil) => true,
            (Type::Any, _) => true,
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
            Type::Map => write!(f, "Map"),
            Type::Array => write!(f, "Array"),
            Type::Nil => write!(f, "Nil"),
            Type::Any => write!(f, "Any"),
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
