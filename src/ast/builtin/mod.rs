use crate::ast::function::default_function;
use crate::resolver::File;

use super::environment::{Type, Value};

pub fn init(file: &mut File) {
    file.environment.define_builtin_function(
        "use".into(),
        |ctx, _, inputs, _| {
            if inputs.len() != 1 {
                panic!(
                    "Expected exactly one argument for 'use', got {}",
                    inputs.len()
                );
            }

            match &inputs[0] {
                Value::Str(import) => {
                    let mut import = import.clone();
                    if !import.ends_with(".ds") {
                        import.push_str(".ds");
                    }
                    let mut loaded = File::load(ctx, import.as_str());
                    default_function(ctx, &loaded.ast, &vec![], &mut loaded.environment)
                }
                _ => panic!(
                    "Expected a string argument for 'use', got {}",
                    inputs[0].get_type()
                ),
            }
        },
        Type::Any,
    );
    file.environment.define_builtin_function(
        "hello_world".into(),
        |_, _, _, _| Value::Str("Hello world!".into()),
        Type::Str,
    );
}
