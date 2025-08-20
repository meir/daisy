use env_logger::Env;
use lalrpop_util::lalrpop_mod;

mod ast;
mod ast2;
use ast2::environment::Environment;
// mod cli;
mod context;
mod parser;
// mod resolver;

lalrpop_mod!(grammar);

fn main() {
    let env = Env::default().filter_or("DAISY_LOG", "trace");
    env_logger::init_from_env(env);
    let ctx = context::Context::load_config();
    _ = ctx;

    // cli::run(&mut ctx);
}
