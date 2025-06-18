use super::environment::{Type, Value};
use crate::resolver::File;
use import::builtin_use;

mod import;

pub fn init(file: &mut File) {
    file.environment
        .define_builtin_function("use".into(), builtin_use, Type::Any);
    file.environment.define_builtin_function(
        "hello_world".into(),
        |_, _, _, _| Value::Str("Hello world!".into()),
        Type::Str,
    );
}
