/************************************************************************************************/

use super::error::Error;
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
            input: PathBuf::new(),
            output: PathBuf::new(),
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn read_from_yaml() -> Result<SiteConfig, Error> {
        let mut sc = SiteConfig::new();

        sc.parse_yaml();

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

        Ok(self)
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
