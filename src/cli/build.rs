use crate::context::Context;
use crate::resolver::File;
use std::path::Path;
use walkdir::WalkDir;

pub fn save(ctx: &Context, path: &str, content: &File) {
    let src = Path::new(path);
    println!("Saving file: {:?}", src);
}

pub fn build(ctx: &Context) {
    WalkDir::new(&ctx.config.src)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().is_file() && entry.path().extension() == Some("ds".as_ref())
        })
        .for_each(|entry| {
            let path = entry.path();
            let file = File::load_absolute(ctx, path.to_str().unwrap());
            save(ctx, path.to_str().unwrap(), &file);
        });
}
