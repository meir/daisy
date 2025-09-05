use crate::ast2::environment::Value;
use crate::ast2::expression::Expression;

pub fn identifier(location: Vec<String>) -> Expression {
    Box::new(move |_ctx, env| {
        let first = location
            .first()
            .expect("Identifier must have at least one part");
        let mut current = env.get(first);

        for part in location.iter().skip(1) {
            current = if let Some(tv) = current {
                let borrowed = tv.borrow();
                if let Value::Map(map) = &borrowed.1 {
                    map.as_map().get(part).cloned()
                } else {
                    None
                }
            } else {
                None
            };
        }

        if let Some(tv) = current {
            tv.borrow().1.clone()
        } else {
            Value::Nil
        }
    })
}
