/************************************************************************************************/

use super::site::SiteConfig;
use super::util::verbose_println;
use super::yasg::YasgClass;
use super::yasg::YasgFile;
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
    let mut template_list = Vec::new();
    let mut page_list = Vec::new();

    for path in file_list.iter() {
        let extension = path.extension().unwrap();

        if extension.eq("yasg") {
            let yasg_file = YasgFile::parse(path);

            if yasg_file.class.is_some() {
                match yasg_file.class.unwrap() {
                    YasgClass::Template => template_list.push(yasg_file),
                    YasgClass::Page => page_list.push(yasg_file),
                }
            }
        } else {
            copy_file(verbose, config, path);
        }
    }

    process_pages(verbose, config, &template_list, &page_list);
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

fn process_pages(verbose: bool, _config: &SiteConfig, templates: &[YasgFile], pages: &[YasgFile]) {
    verbose_println(verbose, "    Processing pages.");

    dbg!(templates);
    dbg!(pages);

    unimplemented!();
}

/************************************************************************************************/
