use std::{
    cell::RefCell,
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
    rc::Rc,
};

use resource::Resource;
use walkdir::WalkDir;

use crate::context::Context;

pub mod file;
pub mod resource;

pub fn load_dir(ctx: Context) {
    let config = ctx.config.borrow();
    WalkDir::new(format!("{}/{}", config.paths.workdir, config.paths.pages))
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension() == Some("ds".as_ref())
        })
        .for_each(|entry| {
            let path = entry.path();
            let file =
                get_file(ctx.clone(), path.to_str().unwrap().to_string()).unwrap_or_else(|err| {
                    panic!("Failed to load file {}: {}", path.display(), err);
                });

            let mut resource = file.borrow_mut();

            if let Resource::File(file) = &mut *resource {
                file.is_page = true;
            } else {
                panic!(
                    "Expected a File resource, got {}",
                    entry.path().to_str().unwrap()
                );
            }
        });
}

pub fn get_all(ctx: &mut Context) -> Vec<Rc<RefCell<Resource>>> {
    ctx.resources.iter().cloned().collect()
}

pub fn get_file(ctx: &Context, src: String) -> Result<Rc<RefCell<Resource>>, String> {
    let config = ctx.config.borrow();
    let src = Path::new(config.paths.workdir.as_str()).join(src);
    if let Some(rs) = ctx.resources.iter().find(|rs| match &*rs.borrow() {
        Resource::File(file) => file.src == src,
        Resource::SCSS(src_file, _, _) => src_file == src.to_str().unwrap(),
        Resource::Other(src_file, _) => src_file == src.to_str().unwrap(),
    }) {
        Ok(rs.clone())
    } else {
        match src.extension().and_then(|ext| ext.to_str()) {
            Some("ds") => {
                let file = file::File::load_absolute(&ctx, src.to_str().unwrap());

                let rc = Rc::new(RefCell::new(Resource::File(file)));
                let mut resources = ctx.resources.borrow_mut();
                resources.push(rc.clone());
                Ok(rc)
            }
            Some("scss") => {
                let content = std::fs::read_to_string(&src)
                    .map_err(|_| format!("Failed to read SCSS file: {}", src.display()))?;
                let css = grass::from_string(content.clone(), &grass::Options::default()).map_err(
                    |err| format!("Failed to compile SCSS file: {}: {}", src.display(), err),
                );

                let name = src.file_stem().unwrap().to_str().unwrap();
                let mut hasher = DefaultHasher::new();
                content.hash(&mut hasher);
                let hash = hasher.finish();
                let path =
                    Resource::get_output_path(&ctx, format!("{}-{}.css", name, hash).as_str())
                        .unwrap();

                let rc = Rc::new(RefCell::new(Resource::SCSS(
                    src.to_str().unwrap().to_string(),
                    path.to_str().unwrap().to_string(),
                    css.unwrap(),
                )));
                ctx.resources.push(rc.clone());
                Ok(rc)
            }
            _ => {
                let with_ext = src.with_extension("ds");
                if with_ext.exists() {
                    get_file(&ctx, with_ext.to_str().unwrap().to_string())
                } else {
                    let relative_path =
                        Resource::get_relative_path_from_root(&ctx, src.to_str().unwrap())
                            .map_err(|err| format!("Failed to get relative path: {}", err))?;

                    let mut output = Resource::get_output_path(&ctx, &relative_path)
                        .map_err(|err| format!("Failed to get output path: {}", err))?;

                    if src.extension().is_none() {
                        output.pop();
                    }

                    let rc = Rc::new(RefCell::new(Resource::Other(
                        src.to_str().unwrap().to_string(),
                        output.to_str().unwrap().to_string(),
                    )));

                    ctx.resources.push(rc.clone());
                    Ok(rc)
                }
            }
        }
    }
}
