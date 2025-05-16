use crate::context::Context;
use clap::{Arg, Command};

mod build;

fn build_command() -> Command {
    Command::new("daisy")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("build")
                .about("Build the site")
                .arg(
                    Arg::new("directory")
                        .required(false)
                        .help("Directory to build")
                        .default_value("./src"),
                )
                .arg(
                    Arg::new("output")
                        .required(false)
                        .help("Folder to output in")
                        .default_value("./site"),
                ),
        )
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
