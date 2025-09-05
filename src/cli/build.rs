use crate::ast2::statement::Result;
use crate::ast2::{Build, Environment};
use crate::context::resource::{Resource, ResourceListExt};
use crate::{context, context::Context};

pub fn build(ctx: &mut Context) {
    context::load_dir(ctx);

    ctx.resources.get_pages().iter().for_each(|resource| {
        if let Resource::File(file) = &*resource.borrow() {
            let environment = if let Some(meta) = &file.meta {
                let meta = meta(ctx, &Environment::new());
                let mut env = Environment::new();
                env.set("meta", meta.as_typevalue());
                env
            } else {
                Environment::new()
            };

            for ast in &file.ast {
                let result = ast(ctx, &environment);
                match result {
                    Result::Return(result) => {
                        let value = result.build(ctx, &environment);
                        println!("{}", value)
                    }
                    _ => {}
                }
            }
        } else {
            panic!("Expected a File resource for page");
        }
    });

    // Process pages
    // resolver::get_all(ctx).iter().for_each(|resource| {
    //     if let Resource::File(file) = &*resource.borrow() {
    //         if !file.is_page {
    //             return;
    //         }
    //
    //         let mut scope = file.get_scope(ctx);
    //
    //         let output_path = if let Some(meta) = scope.get("meta") {
    //             let meta = meta.clone().try_into_map().unwrap();
    //             let url = meta.get("url").unwrap();
    //             let url = url.clone().try_into_string().unwrap();
    //
    //             Resource::get_output_path(ctx, url.as_str()).unwrap()
    //         } else {
    //             Resource::get_output_path(ctx, &file.src.to_str().unwrap()).unwrap()
    //         };
    //
    //         let content = default_function(ctx, &file.ast, &vec![], &mut scope.clone());
    //         let content = content.render(ctx, &mut scope);
    //         let output = ctx.save_content(output_path.to_str().unwrap(), content.as_str());
    //         println!("[DAISY] Built {} -> {}", file.src.to_str().unwrap(), output);
    //     } else {
    //         panic!("Expected a File resource for page");
    //     }
    // });
    //
    // // after pages have been process, new resources have been added, process these resources
    // resolver::get_all(ctx)
    //     .iter()
    //     .for_each(|resource| match &*resource.borrow() {
    //         Resource::SCSS(src, path, content) => {
    //             let output = ctx.save_content(path, content);
    //             println!("[SCSS] Built SCSS {} -> {}", src, output);
    //         }
    //         Resource::Other(src, output) => {
    //             std::fs::create_dir_all(Path::new(output).parent().unwrap()).unwrap_or_else(
    //                 |err| {
    //                     panic!("Failed to create directory {}: {}", output, err);
    //                 },
    //             );
    //
    //             std::fs::copy(src, output).unwrap_or_else(|err| {
    //                 panic!(
    //                     "Failed to copy resource from {} to {}: {}",
    //                     src, output, err
    //                 );
    //             });
    //             println!("[ASSET] Copied {} -> {}", src, output);
    //         }
    //         _ => {}
    //     });
}
