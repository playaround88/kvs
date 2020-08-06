use std::env;
use std::process::exit;

use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() -> Result<()> {
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
            let key = sub_m.value_of("key").expect("key argument missing");
            let value = sub_m.value_of("value").expect("value argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("key argument missing");
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("key argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
