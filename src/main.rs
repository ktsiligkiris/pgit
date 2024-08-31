use clap::{arg, command, ArgMatches, Command};
use std::env;
use std::fs;
use std::io::{self, Write};

mod data;

fn main() {
    let matches = parse_args();
    match matches.subcommand() {
        Some(("init", sub_matches)) => init(sub_matches),
        Some(("hash-object", sub_matches)) => hash_object(sub_matches),
        Some(("cat-file", sub_matches)) => cat_file(sub_matches),
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
        .subcommand(
            Command::new("hash-object")
                .about("Create a blob of an object")
                .arg(arg!([FILE])),
        )
        .subcommand(
            Command::new("cat-file")
                .about("cat the contents of a hash object")
                .arg(arg!([OID])),
        )
        .get_matches()
}

fn init(_args: &ArgMatches) {
    let _ = data::init(); // Ignore for now the possibility of an error
    println!(
        "Initialized empty pgit directory in {}/{}",
        env::current_dir().unwrap().display(),
        data::GIT_DIR
    )
}

fn hash_object(args: &ArgMatches) {
    let filepath = args.get_one::<String>("FILE").unwrap();
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    println!("{}", data::hash_object(contents));
}

fn cat_file(args: &ArgMatches) {
    let mut stdout = io::stdout().lock();
    let oid = args.get_one::<String>("OID").unwrap();
    let content = data::get_object(oid).unwrap();
    stdout.write_all(&content).unwrap();
}
