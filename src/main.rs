use clap::{arg, command, Command, ArgMatches};

fn main() {
    let matches = parse_args();
    match matches.subcommand() {
        Some(("init", sub_matches)) => init(sub_matches),
        _ => unreachable!("Exhausted list of subcommands and subcommand _required prevents `None`"),
    }
}

fn parse_args() -> ArgMatches {
    command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
                .about("Initialize the repo")
                .arg(arg!([NAME])),
        )
        .get_matches()
}

fn init(args: &ArgMatches) {
    println!("Hello, World!");
}
