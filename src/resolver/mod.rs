use std::path::Path;

use crate::ast::environment::{Scope, Value};
use file::File;
use walkdir::WalkDir;

use crate::context::Context;

pub mod file;

pub enum Resource {
    File(File, Scope),
    SCSS(String),
    Other(String, String),
}

pub struct Resolver {
    resources: Vec<Resource>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }

    pub fn load_dir(&mut self, ctx: &mut Context) {
        WalkDir::new(format!(
            "{}/{}",
            ctx.config.paths.workdir, ctx.config.paths.pages
        ))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension() == Some("ds".as_ref())
        })
        .for_each(|entry| {
            let path = entry.path();
            let file = file::File::load_absolute(ctx, path.to_str().unwrap());
            let value = file.meta.to_value(ctx, &mut Scope::new());
            let table = match value {
                Value::Table(scope) => scope,
                _ => panic!("Meta must be a table"),
            };
            self.resources.push(Resource::File(file, table));
        });
    }

    pub fn len(&self) -> usize {
        self.resources.len()
    }

    pub fn get(&self, index: usize) -> Option<&Resource> {
        self.resources.get(index)
    }

    pub fn get_file(&mut self, ctx: &mut Context, src: String) -> Result<&mut Resource, String> {
        let src = Path::new(ctx.config.paths.workdir.as_str()).join(src);
        if let Some(rs) = self.resources.iter_mut().find(|rs| match rs {
            Resource::File(file, _) => file.src == src,
            _ => false,
        }) {
            Ok(rs)
        } else {
            panic!("File not found: {}", src.to_str().unwrap());
        }
    }
}
