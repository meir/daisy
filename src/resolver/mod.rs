use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    rc::Rc,
};

use crate::ast::{
    builtin,
    environment::{Scope, Value},
    function::default_function,
};
use resource::Resource;
use walkdir::WalkDir;

use crate::context::Context;

pub mod file;
pub mod resource;

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
                    let css = grass::from_string(content.clone(), &grass::Options::default())
                        .map_err(|err| {
                            format!("Failed to compile SCSS file: {}: {}", src.display(), err)
                        });

                    let name = src.file_stem().unwrap().to_str().unwrap();
                    let mut hasher = DefaultHasher::new();
                    content.hash(&mut hasher);
                    let hash = hasher.finish();
                    let path =
                        Resource::get_output_path(ctx, format!("{}-{}.css", name, hash).as_str())
                            .unwrap();

                    let rc = Rc::new(Resource::SCSS(
                        path.to_str().unwrap().to_string(),
                        css.unwrap(),
                    ));
                    ctx.resources.push(rc.clone());
                    Ok(rc)
                }
                _ => {
                    let relative_path =
                        Resource::get_relative_path_from_root(ctx, src.to_str().unwrap())
                            .map_err(|err| format!("Failed to get relative path: {}", err))?;

                    let output = Resource::get_output_path(ctx, &relative_path)
                        .map_err(|err| format!("Failed to get output path: {}", err))?;

                    let rc = Rc::new(Resource::Other(
                        src.to_str().unwrap().to_string(),
                        output.to_str().unwrap().to_string(),
                    ));

                    ctx.resources.push(rc.clone());
                    Ok(rc)
                }
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
