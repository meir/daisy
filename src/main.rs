use env_logger::Env;
use lalrpop_util::lalrpop_mod;

// mod ast;
mod ast2;
mod cli;
mod context;
mod parser;
mod prelude;

lalrpop_mod!(grammar);

fn main() {
    let env = Env::default().filter_or("DAISY_LOG", "trace");
    env_logger::init_from_env(env);
    let mut ctx = context::Context::new();

    cli::run(&mut ctx);
}
