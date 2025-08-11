use std::path::Path;

use crate::ast::environment::Value;
use crate::ast::function::default_function;
use crate::context::Context;
use crate::resolver::{self, resource::Resource};

pub fn build(ctx: &mut Context) {
    resolver::load_dir(ctx);

    // Process pages
    resolver::get_all(ctx).iter().for_each(|resource| {
        if let Resource::File(file) = &*resource.borrow() {
            if !file.is_page {
                return;
            }

            let mut scope = file.get_scope(ctx);

            let output_path = if let Some(Value::Map(meta)) = scope.get("meta") {
                let url = meta.get("url").unwrap_or_else(|| {
                    panic!(
                        "File {} does not have an output path defined",
                        file.src.to_str().unwrap()
                    )
                });
                let url = match url {
                    Value::String(url) => url.to_string(),
                    _ => panic!("Expected a string for output path, got {}", url.get_type()),
                };

                Resource::get_output_path(ctx, url.as_str()).unwrap()
            } else {
                Resource::get_output_path(ctx, &file.src.to_str().unwrap()).unwrap()
            };

            let content = default_function(ctx, &file.ast, &vec![], &mut scope.clone());
            let content = content.render(ctx, &mut scope);
            let output = ctx.save_content(output_path.to_str().unwrap(), content.as_str());
            println!("[DAISY] Built {} -> {}", file.src.to_str().unwrap(), output);
        } else {
            panic!("Expected a File resource for page");
        }
    });

    // after pages have been process, new resources have been added, process these resources
    resolver::get_all(ctx)
        .iter()
        .for_each(|resource| match &*resource.borrow() {
            Resource::SCSS(src, path, content) => {
                let output = ctx.save_content(path, content);
                println!("[SCSS] Built SCSS {} -> {}", src, output);
            }
            Resource::Other(src, output) => {
                std::fs::create_dir_all(Path::new(output).parent().unwrap()).unwrap_or_else(
                    |err| {
                        panic!("Failed to create directory {}: {}", output, err);
                    },
                );

                std::fs::copy(src, output).unwrap_or_else(|err| {
                    panic!(
                        "Failed to copy resource from {} to {}: {}",
                        src, output, err
                    );
                });
                println!("[ASSET] Copied {} -> {}", src, output);
            }
            _ => {}
        });
}
