use crate::ast::environment::{Scope, Type, Value};
use crate::ast::statement::Statement;
use crate::context::Context;
use crate::resolver;
use crate::resolver::resource::Resource;

pub fn builtin_pages(
    ctx: &mut Context,
    _: &Vec<Statement>,
    _: &Vec<Value>,
    _scope: &mut Scope,
) -> Value {
    let mut array = Scope::new();

    let resources = resolver::get_all(ctx);
    let mut index = 0;
    for rc in resources {
        if let Resource::File(file) = &*rc.borrow() {
            if !file.is_page {
                continue; // Skip non-page files
            }

            let page_details = file.get_scope(ctx);

            array.define(Type::Map, index.to_string(), Value::Map(page_details));
            index += 1;
        }
    }

    Value::Array(array)
}
