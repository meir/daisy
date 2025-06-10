use environment::Scope;
use expression::Expression;
use html::Element;
use statement::Statement;

pub mod environment;
pub mod expression;
pub mod function;
pub mod html;
pub mod statement;

#[derive(Clone)]
pub enum Node {
    Element(Element),
    Text(String),
    Insertion(Expression),
    Statement(Statement),
}

impl Node {
    pub fn render(&self, scope: &mut Scope) -> String {
        match self {
            Node::Element(element) => element.render(scope),
            Node::Text(text) => text.clone(),
            Node::Statement(stmt) => {
                stmt.process(scope).unwrap();
                "".to_string()
            }
            Node::Insertion(expr) => expr.to_value(scope).render(scope),
        }
    }
}
