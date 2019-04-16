/************************************************************************************************/

mod build;
mod error;
mod config;
mod util;
mod verbose;
mod yasg;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use build::perform_build;
use clap::Arg;
use clap::SubCommand;
use std::io;
use verbose::Verbose;

/************************************************************************************************/

fn main() {
    let mut app = app_from_crate!().subcommand(
        SubCommand::with_name("build").about("Builds the site").arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Use verbose output"),
        ),
    );
    let matches = app.clone().get_matches();

    match matches.subcommand {
        None => {
            let mut out = io::stdout();
            app.write_long_help(&mut out).unwrap();
            println!();
        }
        Some(cmd) => {
            if cmd.name == "build" {
                let mut verbose = Verbose::new();
                if cmd.matches.is_present("verbose") {
                    verbose.enable();
                }
                perform_build(&mut verbose);
            }
        }
    }
}

/************************************************************************************************/
