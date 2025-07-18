use std::{
    io::Error,
    path::{Path, PathBuf},
};

use crate::{
    ast::environment::{Scope, Value},
    context::Context,
};

use super::file::File;

#[derive(Clone)]
pub enum Resource {
    File(File, Scope, Value),
    SCSS(String, String),
    Other(String, String),
}

impl Resource {
    pub fn get_output_path(ctx: &mut Context, src: &str) -> Result<PathBuf, Error> {
        let mut path = Path::new(src);
        path = path.strip_prefix(ctx.get_page_path()).unwrap_or(path);

        if let Some(ext) = path.extension() {
            if ext == "ds" {
                let name = path.file_stem().unwrap();
                let pathbuf = path.parent().unwrap().join(name);
                path = pathbuf.as_path();
                if name == "index" {
                    path = path.parent().unwrap();
                }

                std::path::absolute(&format!(
                    "{}/{}/index.html",
                    ctx.get_output_path(),
                    path.to_str().unwrap(),
                ))
            } else {
                std::path::absolute(&format!(
                    "{}/{}",
                    ctx.get_output_path(),
                    path.to_str().unwrap(),
                ))
            }
        } else {
            std::path::absolute(&format!("{}/{}/index.html", ctx.get_output_path(), src))
        }
    }

    pub fn get_relative_path(ctx: &mut Context, src: &str) -> Result<String, Error> {
        if let Some(relative_path) = src.strip_prefix(&ctx.get_output_path()) {
            return Ok(format!("/{}", relative_path));
        } else {
            panic!("Failed to strip output directory from path: {}", src,);
        }
    }

    pub fn get_relative_path_from_root(ctx: &mut Context, src: &str) -> Result<String, Error> {
        if let Some(relative_path) = src.strip_prefix(&ctx.config.paths.workdir) {
            return Ok(format!("/{}", relative_path));
        } else {
            panic!("Failed to strip workdir from path: {}", src);
        }
    }
}
