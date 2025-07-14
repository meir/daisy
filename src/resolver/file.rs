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

#[derive(Clone)]
pub struct File {
    pub src: PathBuf,
    pub content: String,

    pub meta: Expression,
    pub ast: Vec<Statement>,
}

impl File {
    #[allow(dead_code)]
    pub fn load(ctx: &mut Context, file: &str) -> File {
        let src = Path::new(ctx.config.paths.workdir.as_str()).join(file);
        Self::load_absolute(ctx, src.to_str().unwrap())
    }

    pub fn load_absolute<P: AsRef<Path>>(ctx: &mut Context, src: P) -> File {
        let content = fs::read_to_string(&src).unwrap_or_else(|_| {
            panic!("Failed to read file: {:?}", src.as_ref());
        });
        let ast = ctx
            .parser
            .parse(content.as_str())
            .unwrap_or_else(|err| Self::error_message(src.as_ref(), err, &content));

        File {
            src: src.as_ref().to_path_buf(),
            content,

            meta: ast.0,
            ast: ast.1,
        }
    }

    pub fn output_path(&self, ctx: &mut Context) -> String {
        let path = self.src.to_str().unwrap();
        let src = Path::new(path).strip_prefix(format!(
            "{}/{}",
            ctx.config.paths.workdir, ctx.config.paths.pages
        ));
        let output = Resource::get_output_path(ctx, src.unwrap().to_str().unwrap()).unwrap();

        output.to_str().unwrap().to_string()
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
