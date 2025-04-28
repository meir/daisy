use std::io::{self, BufRead, Write};

mod node;

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `daisy.l` into scope. By default the
// module name will be `daisy_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("daisy.l");
// Using `lrpar_mod!` brings the parser for `daisy.y` into scope. By default the
// module name will be `daisy_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("daisy.y");

fn main() {
    // Get the `LexerDef` for the `daisy` language.
    let lexerdef = daisy_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                // Now we create a lexer with the `lexer` method with which
                // we can lex an input.
                let lexer = lexerdef.lexer(l);
                // Pass the lexer to the parser and lex and parse the input.
                let (res, errs) = daisy_y::parse(&lexer);
                for e in errs {
                    println!("{}", e.pp(&lexer, &daisy_y::token_epp));
                }
                match res {
                    Some(Ok(r)) => println!("{}", r.out()),
                    _ => eprintln!("Unable to evaluate expression.")
                }
            }
            _ => break
        }
    }
}
