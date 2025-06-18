use crate::context::Context;
use clap::Command;

mod build;

fn build_command() -> Command {
    Command::new("daisy")
        .arg_required_else_help(true)
        .subcommand(Command::new("build").about("Build the site"))
}

pub fn run(ctx: &Context) {
    let matches = build_command().get_matches();

    match matches.subcommand() {
        Some(("build", _)) => {
            build::build(ctx);
        }
        _ => {
            println!("Unknown command");
        }
    }
}
