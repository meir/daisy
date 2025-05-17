use env_logger::Env;
use lalrpop_util::lalrpop_mod;

mod ast;
mod cli;
mod context;
mod resolver;

lalrpop_mod!(grammar);

fn main() {
    let env = Env::default().filter_or("DAISY_LOG", "trace");
    env_logger::init_from_env(env);
    let ctx = context::load_config();

    cli::run(&ctx);
}
