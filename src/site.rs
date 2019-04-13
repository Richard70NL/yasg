/************************************************************************************************/

use super::error::Error;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use yaml_rust::Yaml::Hash;
use yaml_rust::YamlLoader;

/************************************************************************************************/

#[derive(Debug)]
pub struct SiteConfig {
    title: String,
    description: String,
    input: PathBuf,
    output: PathBuf,
}

/************************************************************************************************/

impl SiteConfig {
    /*------------------------------------------------------------------------------------------*/

    fn new() -> SiteConfig {
        SiteConfig {
            title: String::new(),
            description: String::new(),
            input: PathBuf::from("contents"),
            output: PathBuf::from("target/site"),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn read_from_yaml(verbose: bool) -> Result<SiteConfig, Error> {
        let mut sc = SiteConfig::new();

        sc.parse_yaml();

        sc.process_io_paths(verbose);

        match sc.validate() {
            Err(e) => return Err(e),
            _ => return Ok(sc),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn parse_yaml(&mut self) {
        let mut f = File::open("Site.yaml").expect("Site.yaml is not found or can't be opened.");
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let doc = docs.first().unwrap();

        match doc {
            Hash(h) => {
                for (key, value) in h {
                    if let Some(key_str) = key.as_str() {
                        if key_str == "title" {
                            self.title = String::from(
                                value
                                    .as_str()
                                    .expect("No valid string value found for the title field."),
                            );
                        } else if key_str == "description" {
                            self.description =
                                String::from(value.as_str().expect(
                                    "No valid string value found for the description field.",
                                ));
                        }
                    }
                }
            }
            _ => (),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn process_io_paths(&mut self, verbose: bool) {
        if self.input.exists() {
            self.input = self.input.canonicalize().unwrap();
        };

        if self.output.exists() {
            self.output = self.output.canonicalize().unwrap();
        } else {
            if verbose {
                println!(
                    "  Creating output directory '{}'.",
                    self.output.to_str().unwrap()
                )
            }
            create_dir_all(self.output.to_str().unwrap()).unwrap();
            self.output = self.output.canonicalize().unwrap();
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn validate(&self) -> Result<&SiteConfig, Error> {
        if self.title.is_empty() {
            return Err(Error::with_reason(
                "Site.yaml should contain a title field.",
            ));
        }

        if self.description.is_empty() {
            return Err(Error::with_reason(
                "Site.yaml should contain a description fields.",
            ));
        }

        if !self.input.exists() {
            return Err(Error::with_reason(
                format!(
                    "Input directory '{}' does not exists.",
                    self.input.to_str().unwrap()
                )
                .as_str(),
            ));
        }

        if !self.output.exists() {
            return Err(Error::with_reason(
                format!(
                    "Output directory '{}' does not exists.",
                    self.output.to_str().unwrap()
                )
                .as_str(),
            ));
        }

        Ok(self)
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
