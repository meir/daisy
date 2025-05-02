use lalrpop_util::lalrpop_mod;
use std::io::{self, BufRead, Write};

mod ast;

lalrpop_mod!(grammar);

fn main() {
    // Get the `LexerDef` for the `daisy` language.
    let parserdef = grammar::TermParser::new();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }

                parserdef
                    .parse(l.trim())
                    .map(|ast| {
                        for ast_node in &ast {
                            println!("{}", ast_node.str());
                        }
                    })
                    .unwrap_or_else(|err| {
                        eprintln!("Error: {}", err);
                    });
            }
            _ => break,
        }
    }
}
