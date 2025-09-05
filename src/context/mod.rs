use std::{cell::RefCell, collections::HashMap, rc::Rc};
mod config;
use config::Config;
pub mod resource;
use resource::ResourceList;
pub mod parser;
mod resolver;
use crate::grammar::DaisyParser;
pub use resolver::load_dir;

mod file;

const EXTENSION: &str = "ds";

#[derive(Clone)]
pub struct Context {
    pub parser: Rc<RefCell<DaisyParser>>,
    pub resources: ResourceList,
    pub config: Config,
}

impl Context {
    pub fn new() -> Self {
        Context {
            parser: Rc::new(RefCell::new(DaisyParser::new())),
            resources: HashMap::new(),
            config: Config::new(),
        }
    }
}
