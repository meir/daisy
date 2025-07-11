use crate::context::Context;
use crate::resolver::{self, Resource};

pub fn build(ctx: &mut Context) {
    resolver::load_dir(ctx);

    for rs in resolver::get_all(ctx) {
        match rs.as_ref() {
            Resource::File(file, env, render) => {
                if env.get("url").is_none() {
                    continue;
                }

                let content = &render.render(ctx, &mut env.clone());
                let output = ctx.save_content(file.output_path(ctx).as_str(), content);
                println!("Built {} -> {}", file.src.to_str().unwrap(), output);
            }
            Resource::SCSS(path, content) => {
                let output = ctx.save_content(path, content);
                println!("Built SCSS {} -> {}", path, output);
            }
            _ => todo!(),
        }
    }
}
