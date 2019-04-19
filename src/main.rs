/************************************************************************************************/

mod build;
mod clean;
mod config;
mod constants;
mod error;
mod text;
mod util;
mod verbose;
mod yasg;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::constants::*;
use crate::error::YasgError;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
use build::perform_build;
use clap::Arg;
use clap::SubCommand;
use clean::perform_clean;
use std::io;
use std::process::exit;
use verbose::Verbose;

/************************************************************************************************/

fn main() {
    exit(match run() {
        Ok(()) => 0,
        Err(err) => {
            err.show();
            1
        }
    });
}

/************************************************************************************************/

fn run() -> Result<(), YasgError> {
    let mut app = app_from_crate!()
        .subcommand(
            SubCommand::with_name(COMMAND_BUILD_NAME)
                .about(s(CliBuildAbout))
                .arg(
                    Arg::with_name(ARG_VERBOSE_NAME)
                        .short(ARG_VERBOSE_SHORT)
                        .long(ARG_VERBOSE_LONG)
                        .help(s(CliVerboseHelp)),
                ),
        )
        .subcommand(
            SubCommand::with_name(COMMAND_CLEAN_NAME)
                .about(s(CliCleanAbout))
                .arg(
                    Arg::with_name(ARG_VERBOSE_NAME)
                        .short(ARG_VERBOSE_SHORT)
                        .long(ARG_VERBOSE_LONG)
                        .help(s(CliVerboseHelp)),
                ),
        );
    let matches = app.clone().get_matches();

    match matches.subcommand {
        None => {
            let mut out = io::stdout();
            match app.write_long_help(&mut out) {
                Ok(()) => {
                    println!();
                    Ok(())
                }
                Err(clap_err) => Err(YasgError::new(so(ErrorWriteLongHelp)).add(clap_err.message)),
            }
        }
        Some(cmd) => {
            let mut verbose = Verbose::new();
            if cmd.matches.is_present(ARG_VERBOSE_NAME) {
                verbose.enable();
            }

            if cmd.name == COMMAND_BUILD_NAME {
                perform_build(&mut verbose)?;
            } else if cmd.name == COMMAND_CLEAN_NAME {
                perform_clean(&mut verbose);
            }

            Ok(())
        }
    }
}

/************************************************************************************************/
