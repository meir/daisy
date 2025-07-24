use super::environment::{Scope, Type, Value};
use import::builtin_use;

mod format;
mod import;
mod pages;

pub fn init(scope: &mut Scope) {
    let mut builtin = Scope::new();

    builtin.define_builtin_function("format".into(), format::builtin_format, Type::String);
    builtin.define_builtin_function("pages".into(), pages::builtin_pages, Type::Any);
    builtin.define_builtin_function(
        "hello_world".into(),
        |_, _, _, _| Value::String("Hello world!".into()),
        Type::String,
    );

    scope.define(Type::Map, "std".into(), Value::Map(builtin));
    scope.define_builtin_function("use".into(), builtin_use, Type::Any);
}
