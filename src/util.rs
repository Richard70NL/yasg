/************************************************************************************************/

use yaml_rust::yaml::Yaml;

/************************************************************************************************/

pub fn verbose_println(verbose: bool, message: &str) {
    if verbose {
        println!("{}", message);
    }
}

/************************************************************************************************/

pub fn yaml_value_as_string(value: &Yaml) -> Option<String> {
    match value {
        Yaml::String(s) => Some(s.clone()),
        Yaml::Integer(i) => Some(i.to_string()),
        Yaml::Boolean(b) => Some(b.to_string()),
        Yaml::Real(s) => Some(s.clone()),
        _ => None,
    }
}

/************************************************************************************************/
