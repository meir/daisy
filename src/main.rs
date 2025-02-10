#[allow(dead_code)]
mod tokenizer;
use tokenizer::Tokenizer;

fn main() {
    let tokenizer = &mut Tokenizer::new();
    tokenizer.tokenize("div.button onhover=\"console.log(\\\"hovering\\\");\" {\n  attr onclick = ''\n    alert(\"hello there\");\n  '';\n}");

    while tokenizer.next() {
        if let Some(token) = tokenizer.current() {
            print!("{}", token.str());
        }
    }
}
