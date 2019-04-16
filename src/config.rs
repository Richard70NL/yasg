/************************************************************************************************/

use crate::error::YasgError;
use crate::verbose::Verbose;
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
    pub input: PathBuf,
    pub output: PathBuf,
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

    pub fn read_from_yaml(
        verbose: &mut Verbose,
        perform_validation: bool,
        create_output_dir: bool,
    ) -> Result<SiteConfig, YasgError> {
        let mut sc = SiteConfig::new();

        sc.parse_yaml();

        sc.process_io_paths(verbose, create_output_dir);

        if perform_validation {
            match sc.validate() {
                Ok(()) => Ok(sc),
                Err(e) => Err(e), // FIXME: use add_reason for this error
            }
        } else {
            Ok(sc)
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn parse_yaml(&mut self) {
        // FIXME: replace all .expect and .unwrap calls to proper error handling/generation

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

    fn process_io_paths(&mut self, verbose: &mut Verbose, create_output_dir: bool) {
        if self.input.exists() {
            self.input = self.input.canonicalize().unwrap();
        };

        if self.output.exists() {
            self.output = self.output.canonicalize().unwrap();
        } else {
            if create_output_dir {
                verbose.println(
                    format!(
                        "Creating output directory '{}'.",
                        self.output.to_str().unwrap()
                    )
                    .as_str(),
                );
                create_dir_all(self.output.to_str().unwrap()).unwrap();
                self.output = self.output.canonicalize().unwrap();
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn validate(&self) -> Result<(), YasgError> {
        // title is mandatory
        if self.title.is_empty() {
            return Err(YasgError::new(String::from(
                "Site.yaml should contain a title field.",
            )));
        }

        // description is mandatory
        if self.description.is_empty() {
            return Err(YasgError::new(String::from(
                "Site.yaml should contain a description fields.",
            )));
        }

        // input path needs to exist
        if !self.input.exists() {
            return Err(YasgError::new(format!(
                "Input directory '{}' does not exists.",
                self.input.to_str().unwrap()
            )));
        }

        // input path needs to be a directory
        if !self.input.is_dir() {
            return Err(YasgError::new(format!(
                "Input '{}' is not a directory.",
                self.input.to_str().unwrap()
            )));
        }

        // output path needs to exist
        if !self.output.exists() {
            return Err(YasgError::new(format!(
                "Output directory '{}' does not exists.",
                self.output.to_str().unwrap()
            )));
        }

        // output path needs to be a directory
        if !self.output.is_dir() {
            return Err(YasgError::new(format!(
                "Output '{}' is not a directory.",
                self.output.to_str().unwrap()
            )));
        }

        // output path needs to be empty
        if self.output.read_dir().unwrap().count() > 0 {
            return Err(YasgError::new(format!(
                "Output directory '{}' is not empty.",
                self.output.to_str().unwrap()
            )));
        }

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn relative_to_input(&self, path: &PathBuf) -> PathBuf {
        let prefix = self.input.to_str().unwrap();
        let relative = path.strip_prefix(prefix).unwrap();

        relative.to_path_buf()
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/