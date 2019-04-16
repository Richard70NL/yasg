/************************************************************************************************/

use crate::config::SiteConfig;
use crate::verbose::Verbose;
use std::fs::remove_dir_all;

/************************************************************************************************/

pub fn perform_clean(verbose: &mut Verbose) {
    println!("cleaning...");
    verbose.increate_indent();

    verbose.println("Reading site configuration from Site.yaml.");
    verbose.increate_indent();
    let config = SiteConfig::read_from_yaml(verbose, false, false).unwrap(); // FIXME unwrap
    verbose.decrease_indent();

    if config.output.exists() && config.output.is_dir() {
        verbose.println(format!("Removing directory {}", config.output.to_str().unwrap()).as_str());
        remove_dir_all(config.output).unwrap(); // FIXME: use unwrap_or_else to generate an error
    }

    verbose.decrease_indent();
    println!("done!");
}

/************************************************************************************************/
