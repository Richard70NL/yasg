/************************************************************************************************/

use super::error::Error;
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    path::PathBuf,
};
use yaml_rust::{Yaml::Hash, YamlLoader};

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
            Err(e) => Err(e),
            _ => Ok(sc),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn parse_yaml(&mut self) {
        let mut f = File::open("Site.yaml").expect("Site.yaml is not found or can't be opened.");
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let docs = YamlLoader::load_from_str(&s).unwrap();
        let doc = docs.first().unwrap();

        if let Hash(h) = doc {
            for (key, value) in h {
                if let Some(key_str) = key.as_str() {
                    if key_str == "title" {
                        self.title = String::from(
                            value
                                .as_str()
                                .expect("No valid string value found for the title field."),
                        );
                    } else if key_str == "description" {
                        self.description = String::from(
                            value
                                .as_str()
                                .expect("No valid string value found for the description field."),
                        );
                    } else if key_str == "input-path" {
                        self.input = PathBuf::from(
                            value
                                .as_str()
                                .expect("No valid string value found for the input-path field."),
                        )
                    } else if key_str == "output-path" {
                        self.output = PathBuf::from(
                            value
                                .as_str()
                                .expect("No valid string value found for the output-path field."),
                        )
                    }
                } // if let Some
            } // for (key, value)
        } // if let Hash
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
        // title is mandatory
        if self.title.is_empty() {
            return Err(Error::with_reason(
                "Site.yaml should contain a title field.",
            ));
        }

        // description is mandatory
        if self.description.is_empty() {
            return Err(Error::with_reason(
                "Site.yaml should contain a description fields.",
            ));
        }

        // input path needs to exist
        if !self.input.exists() {
            return Err(Error::with_reason(
                format!(
                    "Input directory '{}' does not exists.",
                    self.input.to_str().unwrap()
                )
                .as_str(),
            ));
        }

        // input path needs to be a directory
        if !self.input.is_dir() {
            return Err(Error::with_reason(
                format!(
                    "Input '{}' is not a directory.",
                    self.input.to_str().unwrap()
                )
                .as_str(),
            ));
        }

        // output path needs to exist
        if !self.output.exists() {
            return Err(Error::with_reason(
                format!(
                    "Output directory '{}' does not exists.",
                    self.output.to_str().unwrap()
                )
                .as_str(),
            ));
        }

        // output path needs to be a directory
        if !self.output.is_dir() {
            return Err(Error::with_reason(
                format!(
                    "Output '{}' is not a directory.",
                    self.output.to_str().unwrap()
                )
                .as_str(),
            ));
        }

        // output path needs to be empty
        if self.output.read_dir().unwrap().count() > 0 {
            return Err(Error::with_reason(
                format!(
                    "Output directory '{}' is not empty.",
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
