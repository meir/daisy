use crate::ast::builtin;
use crate::ast::environment::{Scope, Type, Value};
use crate::ast::expression::Expression;
use crate::ast::statement::Statement;
use crate::context::Context;
use crate::grammar::Token;
use lalrpop_util::ParseError;
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::resource::Resource;

pub struct File {
    pub src: PathBuf,
    pub is_page: bool,

    pub meta: Option<Expression>,
    pub ast: Vec<Statement>,
}

impl File {
    pub fn load_absolute<P: AsRef<Path>>(ctx: &Context, src: P) -> File {
        let content = fs::read_to_string(&src).unwrap_or_else(|_| {
            panic!("Failed to read file: {:?}", src.as_ref());
        });
        let ast = ctx
            .parser
            .borrow()
            .parse(content.as_str())
            .unwrap_or_else(|err| Self::error_message(src.as_ref(), err, &content));

        File {
            src: src.as_ref().to_path_buf(),
            is_page: false,

            meta: ast.0,
            ast: ast.1,
        }
    }

    pub fn get_scope(&self, ctx: &mut Context) -> Scope {
        let mut default_meta = Scope::new();
        let output_path = Resource::get_output_path(ctx, &self.src.to_str().unwrap())
            .unwrap_or_else(|_| {
                panic!("Failed to get output path for file: {}", self.src.display())
            });
        let relative_path = Resource::get_relative_path(ctx, output_path.to_str().unwrap())
            .unwrap_or_else(|_| {
                panic!(
                    "Failed to get relative path for file: {}",
                    self.src.display()
                )
            });
        default_meta.define(Type::String, "url".into(), Value::String(relative_path));

        let meta = if let Some(meta) = &self.meta {
            let value = meta(ctx, &mut Scope::new());
            if let Value::Map(meta) = value {
                meta
            } else {
                default_meta
            }
        } else {
            default_meta
        };

        let mut scope = Scope::new();
        scope.set_meta(Value::Map(meta));
        builtin::init(&mut scope);
        scope
    }

    fn position_to_line_column(input: &str, pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut last_line_start = 0;

        for (i, c) in input.char_indices() {
            if i >= pos {
                break;
            }
            if c == '\n' {
                line += 1;
                last_line_start = i + 1;
            }
        }

        let column = pos - last_line_start + 1;
        (line, column)
    }

    fn error_message(src: &Path, err: ParseError<usize, Token, &str>, content: &str) -> ! {
        match err {
            ParseError::InvalidToken { location } => {
                let (line, column) = Self::position_to_line_column(&content, location);
                panic!(
                    "Invalid token at {}:{} in file {}",
                    line,
                    column,
                    src.display()
                );
            }
            ParseError::UnrecognizedEof { location, expected } => {
                let (line, column) = Self::position_to_line_column(&content, location);
                panic!(
                    "Unrecognized EOF at {}:{} in file {}. Expected: {:?}",
                    line,
                    column,
                    src.display(),
                    expected
                );
            }
            ParseError::UnrecognizedToken {
                token: (location, token, _),
                expected,
            } => {
                let (line, column) = Self::position_to_line_column(&content, location);
                panic!(
                    "Unrecognized token '{}' at {}:{} in file {}. Expected: {:?}",
                    token,
                    line,
                    column,
                    src.display(),
                    expected
                );
            }
            ParseError::ExtraToken { token } => {
                let (line, column) = Self::position_to_line_column(&content, token.0);
                panic!(
                    "Extra token '{}' at {}:{} in file {}",
                    token.1,
                    line,
                    column,
                    src.display(),
                );
            }
            ParseError::User { error } => {
                panic!("User error: {} in file {}", error, src.display());
            }
        }
    }
}
