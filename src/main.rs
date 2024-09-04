use crate::base::write_tree;
use crate::data::{get_object, hash_object};
use clap::{arg, command, ArgMatches, Command};
use std::env;
use std::fs;
use std::io::{self, Write};

#[macro_use]
mod data;
#[macro_use]
mod base;

fn main() {
    let matches = parse_args();
    match matches.subcommand() {
        Some(("init", sub_matches)) => init(sub_matches),
        Some(("hash-object", sub_matches)) => hash_obj(sub_matches),
        Some(("cat-file", sub_matches)) => cat_file(sub_matches),
        Some(("write-tree", _sub_matches)) => write_tree_(),
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
        .subcommand(
            Command::new("write-tree")
                .about("Take the current working directory and store it to object database"),
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

fn hash_obj(args: &ArgMatches) {
    let filepath = args.get_one::<String>("FILE").unwrap();
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    println!("{}", hash_object!(contents));
}

fn cat_file(args: &ArgMatches) {
    let mut stdout = io::stdout().lock();
    let oid = args.get_one::<String>("OID").unwrap();
    let content = get_object!(oid).unwrap();
    stdout.write_all(&content).unwrap();
}

fn write_tree_() {
    println!("{}", write_tree!());
}
