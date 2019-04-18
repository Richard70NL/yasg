/************************************************************************************************/

mod build;
mod clean;
mod config;
mod error;
mod text;
mod util;
mod verbose;
mod yasg;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::text::s;
use crate::text::Text::*;
use build::perform_build;
use clap::Arg;
use clap::SubCommand;
use clean::perform_clean;
use std::io;
use verbose::Verbose;

/************************************************************************************************/

const BUILD_COMMAND_NAME: &str = "build";
const CLEAN_COMMAND_NAME: &str = "clean";

const VERBOSE_ARG_NAME: &str = "verbose";
const VERBOSE_ARG_SHORT: &str = "v";
const VERBOSE_ARG_LONG: &str = "verbose";

/************************************************************************************************/

fn main() {
    let mut app = app_from_crate!()
        .subcommand(
            SubCommand::with_name(BUILD_COMMAND_NAME).about(s(CliBuildAbout)).arg(
                Arg::with_name(VERBOSE_ARG_NAME)
                    .short(VERBOSE_ARG_SHORT)
                    .long(VERBOSE_ARG_LONG)
                    .help(s(CliVerboseHelp)),
            ),
        )
        .subcommand(
            SubCommand::with_name(CLEAN_COMMAND_NAME).about(s(CliCleanAbout)).arg(
                Arg::with_name(VERBOSE_ARG_NAME)
                    .short(VERBOSE_ARG_SHORT)
                    .long(VERBOSE_ARG_LONG)
                    .help(s(CliVerboseHelp)),
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
            let mut verbose = Verbose::new();
            if cmd.matches.is_present(VERBOSE_ARG_NAME) {
                verbose.enable();
            }

            if cmd.name == BUILD_COMMAND_NAME {
                perform_build(&mut verbose);
            } else if cmd.name == CLEAN_COMMAND_NAME {
                perform_clean(&mut verbose);
            }
        }
    }
}

/************************************************************************************************/
