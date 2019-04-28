/************************************************************************************************/

use crate::constants::*;
use crate::error::YasgError;
use crate::text::so;
use crate::text::sr;
use crate::text::Text::*;
use crate::util::yaml_value_as_string;
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
    pub title: String,
    pub input: PathBuf,
    pub output: PathBuf,
}

/************************************************************************************************/

impl SiteConfig {
    /*------------------------------------------------------------------------------------------*/

    fn new() -> SiteConfig {
        SiteConfig {
            title: String::new(),
            input: PathBuf::from(DEFAULT_INPUT_DIRECTORY),
            output: PathBuf::from(DEFAULT_OUTPUT_DIRECTORY),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn read_from_yaml(
        verbose: &mut Verbose,
        perform_validation: bool,
        create_output_dir: bool,
    ) -> Result<SiteConfig, YasgError> {
        let mut sc = SiteConfig::new();

        sc.parse_yaml()?;

        sc.process_io_paths(verbose, create_output_dir);

        if perform_validation {
            match sc.validate() {
                Ok(()) => Ok(sc),
                Err(e) => Err(e.add(so(ErrorValidatingSiteYaml))),
            }
        } else {
            Ok(sc)
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn parse_yaml(&mut self) -> Result<(), YasgError> {
        let mut f = File::open("Site.yaml").or_else(|e| {
            Err(YasgError::new(format!("{}", e)).add(so(ErrorWhileReadingSiteYaml)))
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s).or_else(|e| {
            Err(YasgError::new(format!("{}", e)).add(so(ErrorWhileReadingSiteYaml)))
        })?;
        let docs = YamlLoader::load_from_str(&s).or_else(|e| {
            Err(YasgError::new(format!("{}", e)).add(so(ErrorWhileReadingSiteYaml)))
        })?;
        let doc = docs
            .first()
            .ok_or_else(|| YasgError::new(so(ErrorWhileReadingSiteYaml)))?;

        if let Hash(h) = doc {
            for (key, value) in h {
                if let Some(key_str) = key.as_str() {
                    // FIXME change key_str == ... to a match structure.
                    if key_str == YAML_TITLE {
                        self.title = yaml_value_as_string(value).ok_or_else(|| {
                            YasgError::new(sr(ErrorNoValidValueField, &[YAML_TITLE]))
                        })?;
                    } else if key_str == YAML_INPUT_PATH {
                        self.input =
                            PathBuf::from(yaml_value_as_string(value).ok_or_else(|| {
                                YasgError::new(sr(ErrorNoValidValueField, &[YAML_INPUT_PATH]))
                            })?);
                    } else if key_str == YAML_OUTPUT_PATH {
                        self.output =
                            PathBuf::from(yaml_value_as_string(value).ok_or_else(|| {
                                YasgError::new(sr(ErrorNoValidValueField, &[YAML_OUTPUT_PATH]))
                            })?);
                    }
                } // if let Some
            } // for (key, value)
        } // if let Hash

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/

    fn process_io_paths(&mut self, verbose: &mut Verbose, create_output_dir: bool) {
        // FIXME replace unwrap with proper error handling/generating

        if self.input.exists() {
            self.input = self.input.canonicalize().unwrap();
        };

        if self.output.exists() {
            self.output = self.output.canonicalize().unwrap();
        } else if create_output_dir {
            verbose.println(&sr(
                VerboseCreatingOutputDirectory,
                &[&self.output.to_str().unwrap()],
            ));

            create_dir_all(self.output.to_str().unwrap()).unwrap();
            self.output = self.output.canonicalize().unwrap();
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn validate(&self) -> Result<(), YasgError> {
        // title is mandatory
        if self.title.is_empty() {
            return Err(YasgError::new(sr(ErrorNoValidValueField, &[YAML_TITLE])));
        }

        // input path needs to exist
        if !self.input.exists() {
            return Err(YasgError::new(sr(
                ErrorInputDirectoryNotExisting,
                &[self.input.to_str().unwrap()],
            )));
        }

        // input path needs to be a directory
        if !self.input.is_dir() {
            return Err(YasgError::new(sr(
                ErrorInputIsNotDirectory,
                &[self.input.to_str().unwrap()],
            )));
        }

        // output path needs to exist
        if !self.output.exists() {
            return Err(YasgError::new(sr(
                ErrorOutputDirectoryNotExisting,
                &[self.output.to_str().unwrap()],
            )));
        }

        // output path needs to be a directory
        if !self.output.is_dir() {
            return Err(YasgError::new(sr(
                ErrorOutputIsNotDirectory,
                &[self.output.to_str().unwrap()],
            )));
        }

        // output path needs to be empty
        if self.output.read_dir().unwrap().count() > 0 {
            return Err(YasgError::new(sr(
                ErrorOutputIsNotEmpty,
                &[self.output.to_str().unwrap()],
            )));
        }

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn relative_to_input(&self, path: &PathBuf) -> PathBuf {
        // FIXME replace unwrap with proper error handling/generating

        let prefix = self.input.to_str().unwrap();
        let relative = path.strip_prefix(prefix).unwrap();

        relative.to_path_buf()
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
