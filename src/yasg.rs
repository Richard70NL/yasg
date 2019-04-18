/************************************************************************************************/

use crate::config::SiteConfig;
use crate::error::YasgError;
use crate::text::sr;
use crate::text::Text::*;
use crate::util::yaml_value_as_string;
use mustache::Data;
use mustache::MapBuilder;
use pulldown_cmark::html;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use yaml_rust::Yaml::Hash;
use yaml_rust::YamlLoader;

/************************************************************************************************/

#[derive(Debug)]
pub struct YasgFile {
    prefix_input_path: PathBuf,
    prefix_output_path: PathBuf,
    relative_path: PathBuf,
    yaml_content: String,
    body_content: String,
    class: Option<YasgClass>,
    for_class: Option<YasgClass>,
    title: Option<String>,
    description: Option<String>,
}

/************************************************************************************************/

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum YasgClass {
    Template,
    Page,
}

/************************************************************************************************/

impl YasgFile {
    /*------------------------------------------------------------------------------------------*/

    fn new() -> YasgFile {
        YasgFile {
            prefix_input_path: PathBuf::new(),
            prefix_output_path: PathBuf::new(),
            relative_path: PathBuf::new(),
            yaml_content: String::new(),
            body_content: String::new(),
            class: None,
            for_class: None,
            title: None,
            description: None,
        }
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn parse(config: &SiteConfig, path: &PathBuf) -> Result<YasgFile, YasgError> {
        let mut yf = YasgFile::new();
        yf.prefix_input_path = config.input.clone();
        yf.prefix_output_path = config.output.clone();
        yf.relative_path = config.relative_to_input(path);

        let f = File::open(yf.full_input_path()).unwrap();
        let reader = BufReader::new(f);
        let mut in_body = false;

        for line in reader.lines() {
            if let Ok(line) = line {
                if in_body {
                    yf.body_content.push_str(line.as_str());
                    yf.body_content.push('\n');
                } else if line.eq("---") {
                    in_body = true;
                } else {
                    yf.yaml_content.push_str(line.as_str());
                    yf.yaml_content.push('\n');
                }
            }
        }

        yf.parse_yaml();

        match yf.validate() {
            Ok(()) => Ok(yf),
            Err(e) => Err(e.add(sr(
                ErrorParseErrorFor,
                &[yf.relative_path.to_str().unwrap()],
            ))),
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
                        } else if key_str == "description" {
                            self.description = yaml_value_as_string(value);
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
                if self.description.is_none() {
                    return Err(YasgError::new(String::from("No description specified.")));
                }
            }
        }

        Ok(())
    }

    /*------------------------------------------------------------------------------------------*/

    fn full_input_path(&self) -> PathBuf {
        let mut full_path = self.prefix_input_path.clone();
        full_path.push(&self.relative_path);

        full_path
    }

    /*------------------------------------------------------------------------------------------*/

    fn full_output_path(&self) -> PathBuf {
        let mut full_path = self.prefix_output_path.clone();
        full_path.push(&self.relative_path);
        full_path.set_extension("html");

        full_path
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn class(&self) -> Option<YasgClass> {
        self.class
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn for_class(&self) -> Option<YasgClass> {
        self.for_class
    }

    /*------------------------------------------------------------------------------------------*/

    pub fn compile(&self, config: &SiteConfig, template: &YasgFile) {
        let mut c_buffer;

        c_buffer = self.compile_body_content_to_html();

        c_buffer = self.compile_template(config, template, c_buffer);

        self.write_output(c_buffer.as_bytes());
    }

    /*------------------------------------------------------------------------------------------*/

    fn compile_body_content_to_html(&self) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        let parser = Parser::new_ext(&self.body_content, options);

        let mut output_buffer = String::new();
        html::push_html(&mut output_buffer, parser);

        output_buffer
    }

    /*------------------------------------------------------------------------------------------*/

    fn compile_template(
        &self,
        config: &SiteConfig,
        template: &YasgFile,
        page_body: String,
    ) -> String {
        let mustache_template = mustache::compile_str(&template.body_content).unwrap(); // FIXME: unwrap
        let data = self.build_data(config, page_body);

        let output_buffer = mustache_template.render_data_to_string(&data).unwrap(); // FIXME: unwrap

        output_buffer
    }

    /*------------------------------------------------------------------------------------------*/

    fn build_data(&self, config: &SiteConfig, page_body: String) -> Data {
        let site_title = config.title.clone();
        let page_title = self.title.clone().unwrap(); // FIXME: unwrap
        let page_description = self.description.clone().unwrap(); // FIXME unwrap

        MapBuilder::new()
            .insert_str("site_title", site_title)
            .insert_str("page_title", page_title)
            .insert_str("page_description", page_description)
            .insert_str("page_body", page_body)
            .build()
    }

    /*------------------------------------------------------------------------------------------*/

    fn write_output(&self, output_buffer: &[u8]) {
        let mut f = File::create(self.full_output_path()).unwrap(); // FIXME unwrap
        f.write_all(output_buffer).unwrap(); // FIXME unwrap
    }

    /*------------------------------------------------------------------------------------------*/
}

/************************************************************************************************/

impl YasgClass {
    /*------------------------------------------------------------------------------------------*/

    fn from(s: &str) -> Option<YasgClass> {
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
