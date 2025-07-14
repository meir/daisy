use crate::ast::environment::{Scope, Value};
use crate::ast::statement::Statement;
use crate::context::Context;
use crate::resolver::{self, resource::Resource};

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
                Resource::SCSS(path, _) => {
                    let relative_path = Resource::get_relative_path(ctx, path).unwrap().to_string();
                    Value::Str(relative_path)
                }
                Resource::Other(_, output) => Resource::get_relative_path(ctx, output)
                    .map(|path| Value::Str(path))
                    .unwrap_or_else(|_| {
                        panic!("Failed to get relative path for resource: {}", import);
                    }),
            }
        }
        _ => panic!(
            "Expected a string argument for 'use', got {}",
            inputs[0].get_type()
        ),
    }
}
