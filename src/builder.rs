/************************************************************************************************/

use super::site::SiteConfig;
use super::util::verbose_println;
use std::fs::copy;
use std::fs::create_dir_all;
use std::path::PathBuf;

/************************************************************************************************/

pub fn perform_build(verbose: bool) {
    println!("building...");

    verbose_println(verbose, "  Reading site configuration from Site.yaml.");
    let config = SiteConfig::read_from_yaml(verbose).unwrap();

    verbose_println(verbose, "  Building file list.");
    let file_list = build_file_list(&config);

    verbose_println(verbose, "  Processing files.");
    process_files(verbose, &config, &file_list);

    println!("done!");
}

/************************************************************************************************/

fn build_file_list(config: &SiteConfig) -> Vec<PathBuf> {
    let mut file_list = Vec::new();

    scan_directory(config, &mut file_list, &config.input);

    file_list
}

/************************************************************************************************/

fn scan_directory(config: &SiteConfig, file_list: &mut Vec<PathBuf>, dir: &PathBuf) {
    if dir.is_dir() {
        for entry in dir.read_dir().unwrap() {
            if let Ok(entry) = entry {
                if entry.path().is_dir() {
                    scan_directory(config, file_list, &entry.path());
                } else {
                    file_list.push(entry.path());
                }
            }
        }
    }
}

/************************************************************************************************/

fn process_files(verbose: bool, config: &SiteConfig, file_list: &[PathBuf]) {
    for path in file_list.iter() {
        let extension = path.extension().unwrap();

        if extension.eq("yasg") {
            verbose_println(
                verbose,
                format!("    Compile {}.", path.to_str().unwrap()).as_str(),
            );
        } else {
            copy_file(verbose, config, path);
        }
    }
}
/************************************************************************************************/

fn copy_file(verbose: bool, config: &SiteConfig, from_path: &PathBuf) {
    let relative = config.relative_to_input(from_path);

    verbose_println(
        verbose,
        format!("    Copy {}.", relative.to_str().unwrap()).as_str(),
    );

    let mut to = config.output.clone();
    to.push(relative);

    let to_dir = to.parent().unwrap();

    if !to_dir.exists() {
        create_dir_all(to_dir).unwrap();
    }

    copy(from_path.to_str().unwrap(), to.to_str().unwrap()).unwrap();
}

/************************************************************************************************/
