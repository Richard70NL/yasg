/************************************************************************************************/

use super::site::SiteConfig;

/************************************************************************************************/

pub fn perform_build(verbose: bool) {
    println!("building...");

    if verbose {
        println!("  Reading site configuration from Site.yaml.")
    }
    let site_config = SiteConfig::read_from_yaml().unwrap();

    dbg!(site_config);

    println!("done!");
}

/************************************************************************************************/
