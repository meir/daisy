use std::path::Path;

use crate::ast::environment::Value;
use crate::context::Context;
use crate::resolver::{self, resource::Resource};

pub fn build(ctx: &mut Context) {
    resolver::load_dir(ctx);

    for rs in resolver::get_all(ctx) {
        match rs.as_ref() {
            Resource::File(file, env, render) => {
                let output_path = if env.get("url").is_some() {
                    let url = env.get("url").unwrap_or_else(|| {
                        panic!(
                            "File {} does not have an output path defined",
                            file.src.to_str().unwrap()
                        )
                    });
                    let url = match url {
                        Value::Str(url) => url.to_string(),
                        _ => panic!("Expected a string for output path, got {}", url.get_type()),
                    };

                    Resource::get_output_path(ctx, url.as_str()).unwrap()
                } else {
                    Resource::get_output_path(ctx, &file.src.to_str().unwrap()).unwrap()
                };

                let content = &render.render(ctx, &mut env.clone());
                let output = ctx.save_content(output_path.to_str().unwrap(), content);
                println!("[DAISY] Built {} -> {}", file.src.to_str().unwrap(), output);
            }
            Resource::SCSS(path, content) => {
                let output = ctx.save_content(path, content);
                println!("[SCSS] Built SCSS {} -> {}", path, output);
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
        }
    }
}
