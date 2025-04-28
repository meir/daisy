
pub mod str;
pub mod element;
pub mod root;

pub trait Node {
    fn out(&self) -> String;
}

pub trait Body {
    fn add_child(&mut self, child: Box<dyn Node>);
}
