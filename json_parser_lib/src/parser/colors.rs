use std::fmt::Display;

use crate::utils::cache::INT_PTR_0_USIZE;
use crate::utils::json::Json;

use jsonschema::Draft;
use jsonschema::JSONSchema;
use log::{error, info};
use regex::Regex;
use serde_json::json;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Colors {
    name: String,
    value: String,
}

// no override
impl ColorsFieldValidate for Colors {}

// Colors Struct Default Field Validation Trait
trait ColorsFieldValidate {
    const NAME_LENGTH_MAX: usize = 100;
    const NAME_LENGTH_MIN: &usize = INT_PTR_0_USIZE;

    const VALUE_LENGTH_MAX: usize = 7;
    const VALUE_LENGTH_MIN: usize = 2;

    fn name_valid(name: String) -> bool {
        if name.is_empty() {
            eprintln!("name is empty");
            return false;
        }
        let length: usize = name.len();
        if !(length > *Self::NAME_LENGTH_MIN && length <= Self::NAME_LENGTH_MAX) {
            eprintln!(
                "name length {length} > {} && {length} <= {} is required",
                *Self::NAME_LENGTH_MIN,
                Self::NAME_LENGTH_MAX
            );
            return false;
        }
        true
    }

    fn value_valid(value: String) -> bool {
        if value.is_empty() {
            eprintln!("value is empty");
            return false;
        }
        let length: usize = value.len();
        if !(length >= Self::VALUE_LENGTH_MIN && length <= Self::VALUE_LENGTH_MAX) {
            eprintln!(
                "value length {length} > {} && {length} <= {} is required",
                Self::VALUE_LENGTH_MIN,
                Self::VALUE_LENGTH_MAX
            );
            return false;
        }
        let re: Regex = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
        if !re.is_match(&value) {
            eprintln!("value is regex not match. Input Value = \"{value}\" ");
            return false;
        }

        true
    }
}

// struct Colors to_string custom print
impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "struct Colors - (name={:?}, value={:?})",
            self.name, self.value
        )
    }
}

// struct Colors parse and validate
impl Json<Colors> for Colors {
    fn parse(json: &str) -> Vec<Colors> {
        let colors: Vec<Colors> = serde_json::from_str(json).unwrap();
        colors
    }

    // Json-Schema validation problem...
    fn validate(json: &str) -> Result<(), ()> {
        let schema = json!({
            //"$schema": "http://json-schema.org/draft-07/schema#",
            "maxLength": 1000,
            "type": "string",
            "properties": {
                "string": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string",
                                "minLength": 1,
                                "maxLength": 100
                            },
                            "value": {
                                "type": "string",
                                "minLength": 7,
                                "maxLength": 7,
                                "pattern" : "^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$"
                            }
                        },
                        "required": ["name", "value"],
                        "propertyNames": {
                            "pattern": "^(name|value)$"
                        },
                    }
                }
            }
        });

        let instance = json!(json);
        let compiled = JSONSchema::options()
            .with_draft(Draft::Draft7)
            .should_validate_formats(true)
            .compile(&schema)
            .expect("A valid schema");
        let result = compiled.validate(&instance);

        if let Err(errors) = result {
            error!("Json String Deserialize Error");
            for error in errors {
                error!("Validation error: {}", error);
                error!("Instance path: {}", error.instance_path);
            }
            return Err(());
        }
        info!("Json Struct/String Deserialize Validation Success");
        Ok(())
    }

    fn validate_data(vec: Vec<Colors>) -> bool {
        let mut valid = true;
        for e in vec {
            let name = e.name;
            let value = e.value;
            let name_valid = Colors::name_valid(name);
            let value_valid = Colors::value_valid(value);
            if !(name_valid && value_valid) {
                eprintln!("Struct Colors Validation Failed");
                valid = false;
                break;
            }
        }
        return valid;
    }
}

// struct Colors testing
#[cfg(test)]
pub mod tests {
    use std::fs;

    use crate::utils::json::Json;

    use super::Colors;

    // struct Colors Json parser and Validate testing
    #[test]
    fn colors_valid_test() {
        {
            let binding: String = fs::read_to_string("./../resources/colors.json").unwrap();
            let json: &str = binding.as_str();
            {
                assert!(!json.is_empty(), "file_contents not empty");
                let _: Result<(), ()> = Colors::validate(json);
            }
            {
                let vec: Vec<Colors> = Colors::parse(json);
                let valid: bool = Colors::validate_data(vec);
                assert!(valid);
            }
        }
        {
            let binding: String = fs::read_to_string("./../resources/colors2.json").unwrap();
            let json: &str = binding.as_str();
            {
                assert!(!json.is_empty(), "file_contents not empty");
                let _: Result<(), ()> = Colors::validate(json);
            }
            {
                let vec: Vec<Colors> = Colors::parse(json);
                let valid: bool = Colors::validate_data(vec);
                assert!(!valid);
            }
        }
    }
}
