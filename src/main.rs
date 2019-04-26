/************************************************************************************************/

mod build;
mod clean;
mod config;
mod constants;
mod error;
mod new;
mod text;
mod util;
mod verbose;
mod yasg;

/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use crate::build::perform_build;
use crate::clean::perform_clean;
use crate::constants::*;
use crate::error::YasgError;
use crate::new::perform_new;
use crate::text::s;
use crate::text::so;
use crate::text::Text::*;
use crate::verbose::Verbose;
use clap::Arg;
use clap::SubCommand;
use std::io;
use std::process::exit;

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
        )
        .subcommand(
            SubCommand::with_name(COMMAND_NEW_NAME)
                .about(s(CliNewAbout))
                .arg(
                    Arg::with_name(ARG_SITE_NAME)
                        .required(true)
                        .help(s(CliSiteHelp)),
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
            } else if cmd.name == COMMAND_NEW_NAME {
                perform_new(&mut verbose);
            }

            Ok(())
        }
    }
}

/************************************************************************************************/
