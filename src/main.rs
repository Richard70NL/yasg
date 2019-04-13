/************************************************************************************************/

#[macro_use]
extern crate clap;

/************************************************************************************************/

use clap::SubCommand;
use std::io;

/************************************************************************************************/

fn main() {
    let mut app =
        app_from_crate!().subcommand(SubCommand::with_name("build").about("builds the site"));
    let matches = app.clone().get_matches();

    match matches.subcommand {
        None => {
            let mut out = io::stdout();
            app.write_long_help(&mut out)
                .expect("failed to write to stdout");
            println!();
        }
        Some(cmd) => {
            if cmd.name == "build" {
                perform_build();
            }
        }
    }
}

/************************************************************************************************/

fn perform_build() {
    println!("BUILDING...");
}

/************************************************************************************************/
