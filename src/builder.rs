/************************************************************************************************/

use super::{site::SiteConfig, util::verbose_println};

/************************************************************************************************/

pub fn perform_build(verbose: bool) {
    println!("building...");

    verbose_println(verbose, "  Reading site configuration from Site.yaml.");
    let site_config = SiteConfig::read_from_yaml(verbose).unwrap();

    dbg!(site_config);

    println!("done!");
}

/************************************************************************************************/
