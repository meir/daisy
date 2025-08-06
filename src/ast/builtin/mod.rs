use super::environment::{Scope, Type, Value};
use import::builtin_use;

mod format;
mod import;
mod pages;
mod replace;

pub fn init(scope: &mut Scope) {
    let mut builtin = Scope::new();

    builtin.define_builtin_function("format".into(), format::builtin_format, Type::String);
    builtin.define_builtin_function("replace".into(), replace::builtin_replace, Type::String);
    builtin.define_builtin_function("pages".into(), pages::builtin_pages, Type::Any);

    scope.define(Type::Map, "std".into(), Value::Map(builtin));
    scope.define_builtin_function("use".into(), builtin_use, Type::Any);
}
