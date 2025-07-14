use std::path::Path;

use crate::context::Context;
use crate::resolver::{self, resource::Resource};

pub fn build(ctx: &mut Context) {
    resolver::load_dir(ctx);

    for rs in resolver::get_all(ctx) {
        match rs.as_ref() {
            Resource::File(file, env, render) => {
                if env.get("url").is_none() {
                    continue;
                }

                let content = &render.render(ctx, &mut env.clone());
                let output_path = file.output_path(ctx);
                let output = ctx.save_content(output_path.as_str(), content);
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
