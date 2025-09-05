use std::path::Path;

use crate::grammar::Token;
use lalrpop_util::ParseError;

pub fn error_message(src: &Path, err: ParseError<usize, Token, &str>, content: &str) -> ! {
    match err {
        ParseError::InvalidToken { location } => {
            let (line, column) = position_to_line_column(&content, location);
            panic!(
                "Invalid token at {}:{} in file {}",
                line,
                column,
                src.display()
            );
        }
        ParseError::UnrecognizedEof { location, expected } => {
            let (line, column) = position_to_line_column(&content, location);
            panic!(
                "Unrecognized EOF at {}:{} in file {}. Expected: {:?}",
                line,
                column,
                src.display(),
                expected
            );
        }
        ParseError::UnrecognizedToken {
            token: (location, token, _),
            expected,
        } => {
            let (line, column) = position_to_line_column(&content, location);
            panic!(
                "Unrecognized token '{}' at {}:{} in file {}. Expected: {:?}",
                token,
                line,
                column,
                src.display(),
                expected
            );
        }
        ParseError::ExtraToken { token } => {
            let (line, column) = position_to_line_column(&content, token.0);
            panic!(
                "Extra token '{}' at {}:{} in file {}",
                token.1,
                line,
                column,
                src.display(),
            );
        }
        ParseError::User { error } => {
            panic!("User error: {} in file {}", error, src.display());
        }
    }
}

fn position_to_line_column(input: &str, pos: usize) -> (usize, usize) {
    let mut line = 1;
    let mut last_line_start = 0;

    for (i, c) in input.char_indices() {
        if i >= pos {
            break;
        }
        if c == '\n' {
            line += 1;
            last_line_start = i + 1;
        }
    }

    let column = pos - last_line_start + 1;
    (line, column)
}
