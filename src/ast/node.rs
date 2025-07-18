use super::environment::Scope;
use super::expression::Expression;
use super::html::Element;
use super::statement::Statement;
use crate::context::Context;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Insertion(Expression),
    Logic(Statement),
}

impl Node {
    pub fn render(&self, ctx: &mut Context, scope: &mut Scope) -> String {
        match self {
            Node::Element(element) => element.render(ctx, scope),
            Node::Text(text) => text.clone(),
            Node::Insertion(expr) => expr.to_value(ctx, scope).render(ctx, scope),
            Node::Logic(statement) => match statement.process(ctx, scope) {
                Ok(result) => match result {
                    super::statement::ResultType::Collect(value) => {
                        let mut output = String::new();
                        for val in value {
                            output.push_str(&val.render(ctx, scope));
                        }
                        output
                    }
                    super::statement::ResultType::Return(value) => value.render(ctx, scope),
                    super::statement::ResultType::Break
                    | super::statement::ResultType::Continue
                    | super::statement::ResultType::NOP => String::new(),
                },
                Err(e) => panic!("Error processing statement: {}", e),
            },
        }
    }
}
