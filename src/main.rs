extern crate clap;
use clap::{App, Arg, SubCommand};
use lockjaw;
use std::process;

fn main() {
    let matches = App::new("LockJaw")
        .version("0.1.0")
        .author("Neil Locketz <neil@nlocketz.com>")
        .about("CLI Secret Manager with Linux Keyring Support")
        .subcommand(SubCommand::with_name("list").about("lists available secrets"))
        .get_matches();

    if let Err(_) = lockjaw::init() {
        println!("Lockjaw init failed!");
        process::exit(1);
    }

    let exit_code = if let Some(_matches) = matches.subcommand_matches("list") {
        list()
    } else {
        1
    };
    process::exit(exit_code);
}

fn list() -> i32 {
    unimplemented!("list");
}
