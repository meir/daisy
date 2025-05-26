use crate::ast::str::Str;
use crate::ast::AST;
use crate::context::environment::Environment;
use crate::context::Context;

#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: Str,
    value: Option<Str>,
}

impl Attribute {
    pub fn new(name: Str, value: Option<Str>) -> Self {
        Attribute { name, value }
    }

    pub fn merge(&self, other: &Self) -> Self {
        let name = self.name.clone();
        if let Some(sv) = &self.value {
            if let Some(ov) = &other.value {
                return Attribute::new(
                    name,
                    Some(Str::new(format!("{} {}", sv.literal, ov.literal))),
                );
            }
            return Attribute::new(name, Some(sv.clone()));
        }
        return Attribute::new(name, None);
    }
}

impl AST for Attribute {
    fn str(&self, ctx: &Context, scope: &mut Environment) -> String {
        if let Some(value) = &self.value {
            return format!(
                "{}=\"{}\"",
                self.name.str(ctx, scope),
                value.str(ctx, scope)
            );
        }
        return format!("{}", self.name.str(ctx, scope));
    }
}
