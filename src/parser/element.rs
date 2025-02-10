pub struct Element {
    parent: Option<Box<Element>>,

    tag: &'static str,
    attributes: Vec<Attribute>,
    content: Vec<Element>,
}
