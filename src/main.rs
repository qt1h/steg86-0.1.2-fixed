use anyhow::Result;
use clap::{App, Arg};

use std::process;

mod steg86;

fn run() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .infer_subcommands(true) // Replaced versionless_subcommands with infer_subcommands
        .subcommand(
            App::new("profile")
                .about("Profile a binary for steganographic storage capacity")
                .arg(
                    Arg::new("raw")
                        .long("raw")
                        .short('r')
                        .help("Treat the input as a raw binary"),
                )
                .arg(
                    Arg::new("bitness")
                        .long("bitness")
                        .short('b')
                        .takes_value(true)
                        .possible_values(&["16", "32", "64"])
                        .requires("raw")
                        .help("The bitness of the raw binary"),
                )
                .arg(
                    Arg::new("input")
                        .index(1)
                        .required(true)
                        .help("The binary to profile"),
                ),
        )
        .subcommand(
            App::new("embed")
                .about("Embed some data into a binary steganographically")
                .arg(
                    Arg::new("raw")
                        .long("raw")
                        .short('r')
                        .help("Treat the input as a raw binary"),
                )
                .arg(
                    Arg::new("bitness")
                        .long("bitness")
                        .short('b')
                        .takes_value(true)
                        .possible_values(&["16", "32", "64"])
                        .requires("raw")
                        .help("The bitness of the raw binary"),
                )
                .arg(
                    Arg::new("input")
                        .index(1)
                        .required(true)
                        .help("The binary to embed into"),
                )
                .arg(
                    Arg::new("output")
                        .index(2)
                        .required(false)
                        .help("The path to write the steg'd binary to"),
                ),
        )
        .subcommand(
            App::new("extract")
                .about("Extract the hidden data from a binary")
                .arg(
                    Arg::new("raw")
                        .long("raw")
                        .short('r')
                        .help("Treat the input as a raw binary"),
                )
                .arg(
                    Arg::new("bitness")
                        .long("bitness")
                        .short('b')
                        .takes_value(true)
                        .possible_values(&["16", "32", "64"])
                        .requires("raw")
                        .help("The bitness of the raw binary"),
                )
                .arg(
                    Arg::new("input")
                        .index(1)
                        .required(true)
                        .help("The binary to extract from"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("profile", matches)) => steg86::command::profile(&matches),
        Some(("embed", matches)) => steg86::command::embed(&matches),
        Some(("extract", matches)) => steg86::command::extract(&matches),
        _ => unreachable!(),
    }
}

fn main() {
    env_logger::init();

    process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("Fatal: {}", e);
            1
        }
    });
}
