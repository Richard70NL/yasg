/************************************************************************************************/

use crate::config::SiteConfig;
use crate::verbose::Verbose;
use crate::yasg::YasgClass;
use crate::yasg::YasgFile;
use std::collections::HashMap;
use std::fs::copy;
use std::fs::create_dir_all;
use std::path::PathBuf;

/************************************************************************************************/

pub fn perform_build(verbose: &mut Verbose) {
    println!("building...");
    verbose.increate_indent();

    verbose.println("Reading site configuration from Site.yaml.");
    verbose.increate_indent();
    let config = SiteConfig::read_from_yaml(verbose, true, true).unwrap();
    verbose.decrease_indent();

    verbose.println("Building file list.");
    verbose.increate_indent();
    let file_list = build_file_list(&config);
    verbose.decrease_indent();

    verbose.println("Processing files.");
    verbose.increate_indent();
    process_files(verbose, &config, &file_list);
    verbose.decrease_indent();

    verbose.decrease_indent();
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

fn process_files(verbose: &mut Verbose, config: &SiteConfig, file_list: &[PathBuf]) {
    let mut templates = HashMap::new();
    let mut pages = Vec::new();

    for path in file_list.iter() {
        let extension = path.extension().unwrap();

        if extension.eq("yasg") {
            let yasg_file = YasgFile::parse(path).unwrap();

            if yasg_file.class().is_some() {
                match yasg_file.class().unwrap() {
                    YasgClass::Template => {
                        templates.insert(yasg_file.for_class().unwrap(), yasg_file);
                    }
                    YasgClass::Page => pages.push(yasg_file),
                }
            }
        } else {
            copy_file(verbose, config, path);
        }
    }

    verbose.println("Processing pages.");
    verbose.increate_indent();
    process_pages(verbose, config, &templates, &pages);
    verbose.decrease_indent();
}

/************************************************************************************************/

fn copy_file(verbose: &mut Verbose, config: &SiteConfig, from_path: &PathBuf) {
    let relative = config.relative_to_input(from_path);

    verbose.println(format!("Copy {}.", relative.to_str().unwrap()).as_str());

    let mut to = config.output.clone();
    to.push(relative);

    let to_dir = to.parent().unwrap();

    if !to_dir.exists() {
        create_dir_all(to_dir).unwrap();
    }

    copy(from_path.to_str().unwrap(), to.to_str().unwrap()).unwrap();
}

/************************************************************************************************/

fn process_pages(
    _verbose: &mut Verbose,
    _config: &SiteConfig,
    templates: &HashMap<YasgClass, YasgFile>,
    pages: &[YasgFile],
) {
    dbg!(templates);
    dbg!(pages);
}

/************************************************************************************************/
