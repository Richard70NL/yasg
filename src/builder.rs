/************************************************************************************************/

use super::{file::FileInfo, site::SiteConfig, util::verbose_println};
use std::path::PathBuf;

/************************************************************************************************/

pub fn perform_build(verbose: bool) {
    println!("building...");

    verbose_println(verbose, "  Reading site configuration from Site.yaml.");
    let config = SiteConfig::read_from_yaml(verbose).unwrap();

    verbose_println(verbose, "  Building file list.");
    let file_list = build_file_list(&config);

    dbg!(file_list);

    println!("done!");
}

/************************************************************************************************/

fn build_file_list(config: &SiteConfig) -> Vec<FileInfo> {
    let mut file_list = Vec::new();

    scan_directory(&mut file_list, &config.input);

    file_list
}

/************************************************************************************************/

fn scan_directory(file_list: &mut Vec<FileInfo>, dir: &PathBuf) {
    if dir.is_dir() {
        for entry in dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    scan_directory(file_list, &entry.path());
                } else {
                    let fi = FileInfo { path: entry.path() };
                    file_list.push(fi);
                }
            }
        }
    }
}

/************************************************************************************************/
