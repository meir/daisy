use std::{path::Path, rc::Rc};

use crate::ast::{
    builtin,
    environment::{Scope, Value},
    function::default_function,
};
use file::File;
use walkdir::WalkDir;

use crate::context::Context;

pub mod file;

#[derive(Clone)]
pub enum Resource {
    File(File, Scope, Value),
    SCSS(String, String),
    Other(String, String),
}

pub fn load_dir(ctx: &mut Context) {
    WalkDir::new(format!(
        "{}/{}",
        ctx.config.paths.workdir, ctx.config.paths.pages
    ))
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.file_type().is_file() && entry.path().extension() == Some("ds".as_ref()))
    .for_each(|entry| {
        let path = entry.path();
        get_file(ctx, path.to_str().unwrap().to_string()).unwrap_or_else(|err| {
            panic!("Failed to load file {}: {}", path.display(), err);
        });
    });
}

pub fn get_all(ctx: &mut Context) -> Vec<Rc<Resource>> {
    ctx.resources.iter().cloned().collect()
}

pub fn get_file(ctx: &mut Context, src: String) -> Result<Rc<Resource>, String> {
    let src = Path::new(ctx.config.paths.workdir.as_str()).join(src);
    if let Some(rs) = ctx.resources.iter().find(|rs| match rs.as_ref() {
        Resource::File(file, _, _) => file.src == src,
        _ => false,
    }) {
        Ok(rs.clone())
    } else {
        if let Some(ext) = src.extension() {
            match ext.to_str() {
                Some("ds") => {
                    let file = file::File::load_absolute(ctx, src.to_str().unwrap());
                    let value = file.meta.to_value(ctx, &mut Scope::new());

                    let mut env = match value {
                        Value::Table(scope) => scope,
                        _ => panic!("Meta must be a table"),
                    };
                    builtin::init(&mut env);

                    let output = default_function(ctx, &file.ast, &vec![], &mut env);
                    let rc = Rc::new(Resource::File(file, env, output));
                    ctx.resources.push(rc.clone());
                    Ok(rc)
                }
                Some("scss") => {
                    let content = std::fs::read_to_string(&src)
                        .map_err(|_| format!("Failed to read SCSS file: {}", src.display()))?;
                    let css =
                        grass::from_string(content, &grass::Options::default()).map_err(|err| {
                            format!("Failed to compile SCSS file: {}: {}", src.display(), err)
                        });

                    let name = src.file_stem().unwrap().to_str().unwrap();
                    let uuid = uuid::Uuid::new_v4();
                    let path = ctx
                        .get_output_path(format!("{}-{}.css", name, uuid).as_str())
                        .unwrap();

                    let rc = Rc::new(Resource::SCSS(
                        path.to_str().unwrap().to_string(),
                        css.unwrap(),
                    ));
                    ctx.resources.push(rc.clone());
                    Ok(rc)
                }
                _ => todo!(),
            }
        } else {
            let with_ext = src.with_extension("ds");
            if with_ext.exists() {
                get_file(ctx, with_ext.to_str().unwrap().to_string())
            } else {
                panic!(
                            "File not found: {}. Please ensure the file exists or has the correct extension.",
                            src.display()
                        );
            }
        }
    }
}
