use crate::ast::environment::{Scope, Value};
use crate::ast::function::default_function;
use crate::ast::statement::Statement;
use crate::context::Context;
use crate::resolver::{self, resource::Resource};

pub fn builtin_use(
    ctx: &mut Context,
    _: &Vec<Statement>,
    inputs: &Vec<Value>,
    env: &mut Scope,
) -> Value {
    if inputs.len() == 0 {
        panic!(
            "Expected atleast one argument for 'use', got {}",
            inputs.len()
        );
    }

    if let Value::String(import) = &inputs[0] {
        let resource = resolver::get_file(ctx, import.clone());
        match &*resource.unwrap().borrow() {
            Resource::File(file) => {
                let meta = env.get_meta();
                let mut scope = Scope::new();
                super::init(&mut scope);
                if let Some(meta) = meta {
                    scope.set_meta(meta.clone());
                } else {
                    panic!("No meta found in environment for 'use' statement");
                }
                default_function(ctx, &file.ast, &vec![], &mut scope)
            }
            Resource::SCSS(_, path, _) => {
                let relative_path = Resource::get_relative_path(ctx, path).unwrap().to_string();
                Value::String(relative_path)
            }
            Resource::Other(_, output) => Resource::get_relative_path(ctx, output)
                .map(|path| Value::String(path))
                .unwrap_or_else(|_| {
                    panic!("Failed to get relative path for resource: {}", import);
                }),
        }
    } else {
        panic!(
            "Expected a string argument for 'use', got {}",
            inputs[0].get_type()
        )
    }
}
