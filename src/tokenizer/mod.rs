pub mod tokens;

use crate::tokenizer::tokens::{Position, Token, TokenKind};

pub struct Tokenizer {
    init: bool,
    pub index: u32,

    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new() -> Tokenizer {
        Tokenizer {
            init: true,
            index: 0,
            tokens: Vec::new(),
        }
    }

    pub fn tokenize(&mut self, data: &str) {
        let pos = &mut Position::new(1, 0);
        let mut i = 0;
        while i < data.len() {
            if let Some((kind, len)) = TokenKind::from(&data[i..]) {
                if kind.eq(&TokenKind::NEWLINE) {
                    pos.newline();
                }

                self.tokens
                    .push(Token::new(kind, &pos, &data[i..i + len as usize]));
                i += len as usize;
                pos.next();
            }
        }
    }

    pub fn next(&mut self) -> bool {
        if self.init {
            self.init = false;
        } else {
            self.index += 1;
        }
        return self.index < self.tokens.len() as u32;
    }

    pub fn current(&self) -> Option<&Token> {
        if self.index < self.tokens.len() as u32 {
            self.tokens.get(self.index as usize)
        } else {
            None
        }
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else if self.index == 0 {
            self.init = true;
        }
    }

    pub fn peek(&mut self, amount: u32) -> Option<&Token> {
        if self.index + amount < self.tokens.len() as u32 {
            self.tokens.get((self.index + amount) as usize)
        } else {
            None
        }
    }

    pub fn print(&self) {
        println!("{:#?}", self.tokens);
    }
}
