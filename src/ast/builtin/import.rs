use crate::ast::environment::{Scope, Value};
use crate::ast::function::default_function;
use crate::ast::statement::Statement;
use crate::context::Context;
use crate::resolver::{self, Resource};

pub fn builtin_use(
    ctx: &mut Context,
    _: &Vec<Statement>,
    inputs: &Vec<Value>,
    _: &mut Scope,
) -> Value {
    if inputs.len() == 0 {
        panic!(
            "Expected atleast one argument for 'use', got {}",
            inputs.len()
        );
    }

    match &inputs[0] {
        Value::Str(import) => {
            let resource = resolver::get_file(ctx, import.clone());
            match resource.unwrap().as_ref() {
                Resource::File(_, _, output) => output.clone(),
                Resource::SCSS(path, _) => Value::Str(path.clone()),
                _ => todo!(),
            }
            // check if file can be found
            // let mut path = Path::new(ctx.config.paths.workdir.as_str()).join(import.as_str());
            // let mut import = import.clone();
            // if !path.exists() {
            //     if !import.ends_with(".ds") {
            //         let tmp_import = format!("{}.ds", import);
            //         let tmp_path =
            //             Path::new(ctx.config.paths.workdir.as_str()).join(tmp_import.as_str());
            //         if !tmp_path.exists() {
            //             panic!("File not found: {}", import);
            //         }
            //
            //         import = tmp_import;
            //         path = tmp_path;
            //     }
            //     if !path.exists() {
            //         panic!("File not found: {}", import);
            //     }
            // }
            //
            // match path.extension() {
            //     Some(ext) if ext == "ds" => {
            //         let loaded = File::load_absolute(ctx, path.to_str().unwrap());
            //         let mut scope = Scope::new();
            //         super::init(&mut scope);
            //         default_function(ctx, &loaded.ast, &vec![], &mut scope)
            //     }
            //     Some(ext) if ext == "scss" => {
            //         let content = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            //             panic!("Failed to read SCSS file: {}", path.to_str().unwrap());
            //         });
            //
            //         let css = grass::from_string(content.to_owned(), &grass::Options::default())
            //             .unwrap_or_else(|err| {
            //                 panic!(
            //                     "Failed to compile SCSS file: {}: {}",
            //                     path.to_str().unwrap(),
            //                     err
            //                 );
            //             });
            //
            //         let name = path.file_stem().unwrap().to_str().unwrap();
            //         let path = ctx.save_asset(format!("{}.css", name).as_str(), css.as_str());
            //
            //         Value::Str(path)
            //     }
            //     _ => {
            //         let path = ctx.use_asset(path.to_str().unwrap(), use_literal);
            //         Value::Str(path)
            //     }
            // }
        }
        _ => panic!(
            "Expected a string argument for 'use', got {}",
            inputs[0].get_type()
        ),
    }
}
