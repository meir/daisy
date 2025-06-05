use crate::ast::environment::{Scope, Type, Value};
use crate::ast::Node;
use crate::context::Context;
use lalrpop_util::ParseError;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct File {
    src: PathBuf,
    content: String,
    pub environment: Scope,

    pub ast: Vec<Node>,
}

impl File {
    #[allow(dead_code)]
    pub fn load(ctx: &Context, file: &str) -> File {
        let src = Path::new(ctx.config.src.as_str()).join(file);
        Self::load_absolute(ctx, src.to_str().unwrap())
    }

    pub fn load_absolute<P: AsRef<Path>>(ctx: &Context, src: P) -> File {
        let content = fs::read_to_string(&src).unwrap_or_else(|_| {
            panic!("Failed to read file: {:?}", src.as_ref());
        });
        let ast = ctx
            .parser
            .parse(content.as_str())
            .unwrap_or_else(|err| match err {
                ParseError::InvalidToken { location } => {
                    let (line, column) = Self::position_to_line_column(&content, location);
                    panic!(
                        "Invalid token at {}:{} in file {}",
                        line,
                        column,
                        src.as_ref().display()
                    );
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let (line, column) = Self::position_to_line_column(&content, location);
                    panic!(
                        "Unrecognized EOF at {}:{} in file {}. Expected: {:?}",
                        line,
                        column,
                        src.as_ref().display(),
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
                        src.as_ref().display(),
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
                        src.as_ref().display(),
                    );
                }
                ParseError::User { error } => {
                    panic!("User error: {} in file {}", error, src.as_ref().display());
                }
            });

        let mut file = File {
            src: src.as_ref().to_path_buf(),
            content,
            environment: Scope::new(),
            ast,
        };

        file.environment.define(
            Type::Str,
            "src".into(),
            Value::Str(file.src.to_str().unwrap().into()),
        );

        file.environment.define(
            Type::Str,
            "content".into(),
            Value::Str(file.content.clone()),
        );

        file
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
}
