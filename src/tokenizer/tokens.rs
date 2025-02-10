use std::ops::RangeInclusive;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Debug, EnumIter)]
#[allow(non_camel_case_types)]
// Order of enum list is also the match priority
pub enum TokenKind {
    // keywords
    ATTRIBUTE,        // attr
    MULTILINE_STRING, // ''
    //
    EMPTY,       // spaces & tabs
    NEWLINE,     // \n
    ENDL,        // ;
    ID,          // #
    CLASS,       // .
    ASSIGN,      // =
    STRING,      // "
    INSERT,      // $
    SCOPE_OPEN,  // {
    SCOPE_CLOSE, // }
    WORD,        // A-Z a-z 0-9 - _
    //
    OTHER, // any other character
}

// TokenValue defines the values that each token can match on
enum TokenValue {
    Char(char),
    Range(RangeInclusive<char>),
    Str(&'static str),
    VecRange(Vec<RangeInclusive<char>>),
    Any,
}

impl TokenKind {
    // from loops through all the token kinds and returns the first match
    pub fn from(data: &str) -> Option<(TokenKind, u32)> {
        for kind in TokenKind::iter() {
            let (matches, len) = kind.matches(data);
            if matches {
                return Some((kind, len));
            }
        }
        None
    }

    // value defines the values that each token can match on
    fn value(&self) -> TokenValue {
        match *self {
            TokenKind::EMPTY => TokenValue::VecRange(vec!['\t'..='\t', ' '..=' ']),
            TokenKind::NEWLINE => TokenValue::Char('\n'),
            TokenKind::ENDL => TokenValue::Char(';'),
            TokenKind::WORD => {
                TokenValue::VecRange(vec!['a'..='z', 'A'..='Z', '0'..='9', '-'..='-', '_'..='_'])
            }
            TokenKind::ID => TokenValue::Char('#'),
            TokenKind::CLASS => TokenValue::Char('.'),
            TokenKind::ASSIGN => TokenValue::Char('='),
            TokenKind::STRING => TokenValue::Char('"'),
            TokenKind::INSERT => TokenValue::Char('$'),
            TokenKind::MULTILINE_STRING => TokenValue::Str("''"),
            TokenKind::SCOPE_OPEN => TokenValue::Char('{'),
            TokenKind::SCOPE_CLOSE => TokenValue::Char('}'),
            TokenKind::ATTRIBUTE => TokenValue::Str("attr"),
            TokenKind::OTHER => TokenValue::Any,
        }
    }

    // matches checks if the token matches the start of the data given
    fn matches(&self, data: &str) -> (bool, u32) {
        let value = self.value();
        let c = data.chars().next().unwrap();
        match value {
            TokenValue::Str(s) => (data.starts_with(s), s.len() as u32),
            TokenValue::VecRange(v) => (
                v.iter().any(|r| r.contains(&c)),
                data.chars()
                    .take_while(|c| v.iter().any(|r| r.contains(c)))
                    .count() as u32,
            ),
            TokenValue::Range(r) => (r.contains(&c), 1),
            TokenValue::Char(v) => (c == v, 1),
            TokenValue::Any => (true, 1),
        }
    }
}

#[derive(Clone, Debug)]
// Position keeps track of the column and line of the tokens
pub struct Position {
    line: u32,
    column: u32,
}

impl Position {
    // new creates a new position
    pub fn new(line: u32, column: u32) -> Position {
        Position { line, column }
    }

    // next increments the column
    pub fn next(&mut self) {
        self.column += 1;
    }

    // line increments the line and resets the column
    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    // str returns the position as a string
    pub fn str(&self) -> Box<str> {
        format!("{}:{}", self.line, self.column).into()
    }
}

#[derive(Debug)]
// Token represents a token in the source code
pub struct Token {
    pub kind: TokenKind,
    position: Position,
    value: Box<str>,
}

impl Token {
    // new creates a new token
    pub fn new(kind: TokenKind, position: &Position, value: &str) -> Token {
        Token {
            kind,
            position: position.clone(),
            value: value.into(),
        }
    }

    // position returns the position of the token as a string
    pub fn position(&self) -> Box<str> {
        self.position.str()
    }

    // str returns the value of the token as a string
    pub fn str(&self) -> &str {
        &self.value
    }
}
