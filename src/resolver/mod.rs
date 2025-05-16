use crate::ast::AST;
use crate::context::Context;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct File {
    src: PathBuf,
    content: String,

    pub ast: Vec<Box<dyn AST>>,
}

impl File {
    pub fn load(ctx: &Context, file: &str) -> File {
        let src = Path::new(ctx.config.src.as_str()).join(file);
        Self::load_absolute(ctx, src.to_str().unwrap())
    }

    pub fn load_absolute<P: AsRef<Path>>(ctx: &Context, src: P) -> File {
        let content = fs::read_to_string(&src).unwrap_or_else(|_| {
            panic!("Failed to read file: {:?}", src.as_ref());
        });
        let ast = ctx.parser.parse(content.as_str()).unwrap_or_else(|err| {
            panic!("Failed to parse file: {:?}: {}", src.as_ref(), err);
        });

        File {
            src: src.as_ref().to_path_buf(),
            content,
            ast,
        }
    }
}
