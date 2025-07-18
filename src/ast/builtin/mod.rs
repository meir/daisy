use super::environment::{Scope, Type, Value};
use import::builtin_use;

mod format;
mod import;
mod pages;

pub fn init(scope: &mut Scope) {
    let mut builtin = Scope::new();

    builtin.define_builtin_function("format".into(), format::builtin_format, Type::Str);
    builtin.define_builtin_function("pages".into(), pages::builtin_pages, Type::Any);
    builtin.define_builtin_function(
        "hello_world".into(),
        |_, _, _, _| Value::Str("Hello world!".into()),
        Type::Str,
    );

    scope.define(Type::Map, "std".into(), Value::Map(builtin));
    scope.define_builtin_function("use".into(), builtin_use, Type::Any);
}
