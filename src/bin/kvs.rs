use std::env;
use std::process::exit;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("get the value from kvs")
                .arg(Arg::with_name("key").help("The key of k-v").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set value for kvs")
                .arg(
                    Arg::with_name("key")
                        .help("store key of the value")
                        .required(true),
                )
                .arg(
                    Arg::with_name("value")
                        .help("value tobe store")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("remove value with key")
                .arg(
                    Arg::with_name("key")
                        .help("store key of the value")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
