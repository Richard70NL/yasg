/************************************************************************************************/

use crate::config::SiteConfig;
use crate::text::s;
use crate::text::sr;
use crate::text::Text::*;
use crate::verbose::Verbose;
use std::fs::remove_dir_all;

/************************************************************************************************/

pub fn perform_clean(verbose: &mut Verbose) {
    println!("{}", s(VerboseCleaning));
    verbose.increate_indent();

    verbose.println(s(VerboseReadingSiteConfig));
    verbose.increate_indent();
    let config = SiteConfig::read_from_yaml(verbose, false, false).unwrap(); // FIXME unwrap
    verbose.decrease_indent();

    if config.output.exists() && config.output.is_dir() {
        verbose.println(&sr(
            VerboseDeletingDirectory,
            &[&config.output.to_str().unwrap()],
        ));

        remove_dir_all(config.output).unwrap(); // FIXME: use unwrap_or_else to generate an error
    }

    verbose.decrease_indent();
    println!("{}", s(VerboseDone));
}

/************************************************************************************************/
