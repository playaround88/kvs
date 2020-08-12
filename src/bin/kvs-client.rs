use std::env;
use std::io::{Read, Write};

use kvs::Result;
use std::net::{Shutdown, TcpStream};

#[macro_use]
extern crate log;
extern crate clap;
extern crate stderrlog;
use clap::{App, Arg, SubCommand};

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Silence all output"),
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .help("address for connect to the server.")
                .takes_value(true),
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

    let quiet = matches.is_present("quiet");
    let verbose = matches.occurrences_of("verbosity") as usize;

    stderrlog::new()
        .module(module_path!())
        .quiet(quiet)
        .verbosity(verbose)
        .init()
        .unwrap();
    info!("log init OK!");

    let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
    info!("server address:{}", addr);

    let mut stream = TcpStream::connect(addr)?;
    let mut out_stream = stream.try_clone()?;

    match matches.subcommand() {
        ("set", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("key argument missing");
            let value = sub_m.value_of("value").expect("value argument missing");

            let mut buf = String::from("set ");
            buf.push_str(key);
            buf.push_str(" ");
            buf.push_str(value);
            debug!("send cmd:{}", buf);

            let _size = stream.write(buf.as_bytes());
            stream.flush()?;
            stream
                .shutdown(Shutdown::Write)
                .expect("shutdown call failed");

            let mut result = String::new();
            let _size = out_stream.read_to_string(&mut result);
            println!("{}", result)
        }
        ("get", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("key argument missing");
            let mut buf = String::from("get ");
            buf.push_str(key);
            debug!("send cmd:{}", buf);

            let _size = stream.write(buf.as_bytes());
            stream.flush()?;
            stream
                .shutdown(Shutdown::Write)
                .expect("shutdown call failed");

            let mut result = String::new();
            let _size = out_stream.read_to_string(&mut result);
            println!("{}", result)
        }
        ("rm", Some(sub_m)) => {
            let key = sub_m.value_of("key").expect("key argument missing");

            let mut buf = String::from("rm ");
            buf.push_str(key);
            debug!("send cmd:{}", buf);

            let _size = stream.write(buf.as_bytes());
            stream.flush()?;
            stream
                .shutdown(Shutdown::Write)
                .expect("shutdown call failed");

            let mut result = String::new();
            let _size = out_stream.read_to_string(&mut result);
            println!("{}", result)
        }
        _ => unreachable!(),
    }

    out_stream
        .shutdown(Shutdown::Read)
        .expect("shutdown out_stream");
    Ok(())
}
