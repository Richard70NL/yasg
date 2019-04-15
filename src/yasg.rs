/************************************************************************************************/

use crate::error::YasgError;
use crate::util::yaml_value_as_string;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use yaml_rust::Yaml::Hash;
use yaml_rust::YamlLoader;

/************************************************************************************************/

#[derive(Debug)]
pub struct YasgFile {
    path: PathBuf,
    yaml_content: String,
    body_content: String,
    pub class: Option<YasgClass>,
    for_class: Option<YasgClass>,
    title: Option<String>,
}

/************************************************************************************************/

#[derive(Debug, Copy, Clone)]
pub enum YasgClass {
    Template,
    Page,
}

/************************************************************************************************/

impl YasgFile {
    /*------------------------------------------------------------------------------------------*/

    fn new() -> YasgFile {
        YasgFile {
            path: PathBuf::new(),
            yaml_content: String::new(),
            body_content: String::new(),
            class: None,
            for_class: None,
            title: None,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn parse(path: &PathBuf) -> Result<YasgFile, YasgError> {
        let mut yf = YasgFile::new();
        yf.path = path.clone();

        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        let mut in_body = false;

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if in_body {
                        yf.body_content.push_str(line.as_str());
                        yf.body_content.push('\n');
                    } else {
                        if line.eq("---") {
                            in_body = true;
                        } else {
                            yf.yaml_content.push_str(line.as_str());
                            yf.yaml_content.push('\n');
                        }
                    }
                }
                _ => (),
            }
        }

        yf.parse_yaml();

        match yf.validate() {
            Ok(()) => return Ok(yf),
            Err(e) => {
                return Err(e.add_reason(format!("Parse error for {}", yf.path.to_str().unwrap())))
            }
        }
    }

    /*------------------------------------------------------------------------------------------*/

    fn parse_yaml(&mut self) {
        let docs = YamlLoader::load_from_str(self.yaml_content.as_str()).unwrap();
        if !docs.is_empty() {
            let doc = docs.first().unwrap();

            if let Hash(h) = doc {
                for (key, value) in h {
                    if let Some(key_str) = key.as_str() {
                        if key_str == "class" {
                            if let Some(s) = yaml_value_as_string(value) {
                                self.class = YasgClass::from(&s)
                            }
                        } else if key_str == "title" {
                            self.title = yaml_value_as_string(value);
                        } else if key_str == "for-class" {
                            if let Some(s) = yaml_value_as_string(value) {
                                self.for_class = YasgClass::from(&s)
                            }
                        };
                    } // if let Some
                } // for (key, value)
            } // if let Hash
        } // if !docs.is_empty
    }

    /*------------------------------------------------------------------------------------------*/

    fn validate(&self) -> Result<(), YasgError> {
        if self.class.is_none() {
            return Err(YasgError::new(String::from("No class specified.")));
        }

        match self.class.unwrap() {
            YasgClass::Template => {
                if self.for_class.is_none() {
                    return Err(YasgError::new(String::from(
                        "No for-class or invalid for-class specified.",
                    )));
                }
            }
            YasgClass::Page => {
                if self.title.is_none() {
                    return Err(YasgError::new(String::from("No title specified.")));
                }
            }
        }

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

impl YasgClass {
    /*------------------------------------------------------------------------------------------*/

    fn from(s: &String) -> Option<YasgClass> {
        if s == "template" {
            Some(YasgClass::Template)
        } else if s == "page" {
            Some(YasgClass::Page)
        } else {
            None
        }
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/
