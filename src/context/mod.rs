use std::{cell::RefCell, rc::Rc};
mod config;
use config::Config;

use crate::grammar::DaisyParser;

pub struct Context {
    pub parser: DaisyParser,
    pub resources: Vec<Rc<RefCell<Resource>>>,
    pub config: Config,
}

impl Context {
    pub fn new() -> Self {
        Context {
            parser: DaisyParser::new(),
            resources: vec![],
            config: Config::new(),
        }
    }
}
