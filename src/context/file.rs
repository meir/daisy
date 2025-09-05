use std::{fs::read_to_string, path::PathBuf};

use crate::{
    ast2::{Expression, Statement},
    context::parser,
};

use super::Context;

pub struct File {
    pub is_page: bool,

    pub meta: Option<Expression>,
    pub ast: Vec<Statement>,
}

impl File {
    pub fn new(ctx: Context, location: &PathBuf) -> Self {
        let parser = ctx.parser.borrow();
        let content = read_to_string(&location);
        println!("Loading file: {}", location.to_str().unwrap());

        let content = if let Ok(content) = content {
            content
        } else {
            panic!("Failed to read file: {}", location.to_str().unwrap());
        };

        let result = parser.parse(content.as_str());
        if let Ok((meta, ast)) = result {
            File {
                is_page: false,
                meta,
                ast,
            }
        } else {
            parser::error_message(location, result.err().unwrap(), &content);
        }
    }
}
