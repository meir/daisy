use crate::ast::builtin;
use crate::ast::function::default_function;
use crate::resolver::{Resolver, Resource};
use crate::{ast::environment::Value, context::Context};

pub fn build(ctx: &mut Context) {
    let mut resolver = Resolver::new();
    resolver.load_dir(ctx);

    for i in 0..resolver.len() {
        let rs = resolver.get(i).unwrap();
        match rs {
            Resource::File(file, meta) => {
                if meta.get("url").is_none() {
                    continue;
                }

                let env = &mut meta.clone();
                builtin::init(env);
                let render: Value = default_function(ctx, &file.ast, &vec![], env);
                let content = &render.render(ctx, env);
                let output = ctx.save_page(file.src.to_str().unwrap(), content);
                println!("Built {} -> {}", file.src.to_str().unwrap(), output);
            }
            _ => todo!(),
        }
    }
}
