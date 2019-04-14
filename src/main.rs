/************************************************************************************************/

mod builder;
mod error;
mod site;
mod util;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use builder::perform_build;
use clap::Arg;
use clap::SubCommand;
use std::io;

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
                let verbose = cmd.matches.is_present("verbose");
                perform_build(verbose);
            }
        }
    }
}

/************************************************************************************************/
