use kvs::{KvsEngine, Result};
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

extern crate clap;
#[macro_use]
extern crate log;
extern crate stderrlog;
use clap::{App, Arg};

//use log::{LevelFilter, SetLoggerError};
//use stderrlog::StdErrLog;

fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity")
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .help("Silence all output")
        )
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .help("address listen for incoming connection, either v4 or v6, and a port number, with the format IP:PORT.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("engine ")
                .long("engine")
                .help("store engine, either 'kvs' or 'sled'.")
                .takes_value(true)
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
    info!("binding address: {}", addr);
    let engine = matches.value_of("engine").unwrap_or("kvs");
    info!("use engine: {}", engine);

    info!("init store engine...");
    /*
        let mut store: impl KvsEngine;
        if engine.eq_ignore_ascii_case("sled") {
        } else {
            store = KvStore::open(current_dir()?)?;
        }
    */
    info!("start server listener:{}", addr);
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = String::from("");
    let _size = stream.read_to_string(&mut buf);
    debug!("get string from connection: {}", &buf);
    let _size = stream.write(buf.as_bytes());
    stream.flush().expect("flush buf fail!");
    let _ = stream.shutdown(Shutdown::Both);
}
