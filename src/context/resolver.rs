use walkdir::WalkDir;

use super::resource::ResourceListExt;
use super::{resource::Resource, Context, EXTENSION};

pub fn load_dir(ctx: &mut Context) {
    let config = &ctx.config;
    WalkDir::new(config.paths.get_page_path())
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension() == Some(EXTENSION.as_ref())
        })
        .for_each(|entry| {
            let path = entry.path();
            let resource = ctx.resources.load(ctx.clone(), path.to_path_buf());
            let mut resource = resource.borrow_mut();

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
