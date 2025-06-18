use crate::ast::environment::Value;
use crate::ast::function::default_function;
use crate::context::Context;
use crate::resolver::File;
use walkdir::WalkDir;

pub fn build(ctx: &Context) {
    WalkDir::new(format!(
        "{}/{}",
        ctx.config.paths.workdir, ctx.config.paths.pages
    ))
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.file_type().is_file() && entry.path().extension() == Some("ds".as_ref()))
    .for_each(|entry| {
        let path = entry.path();
        let mut file = File::load_absolute(ctx, path.to_str().unwrap());
        let ast = file.ast.clone();
        let render: Value = default_function(ctx, &ast, &vec![], &mut file.environment);
        let content = &render.render(ctx, &mut file.environment);
        let output = ctx.save_page(path.to_str().unwrap(), content);
        println!("Built {} -> {}", path.to_str().unwrap(), output);
    });
}
