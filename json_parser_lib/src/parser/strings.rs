// parser module 
// strings json model & validate & parser
use std::fmt::Display;

use crate::utils::cache::INT_PTR_0_USIZE;
use crate::utils::json::Json;

use jsonschema::Draft;
use jsonschema::JSONSchema;
use log::{error, info};
use serde_json::json;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Strings {
    name: String,
    value: String,
}

// no override
impl StringsFieldValidate for Strings {}

// Strings Struct Default Field Validation Trait
trait StringsFieldValidate {
    const NAME_LENGTH_MAX: usize = 100;
    const NAME_LENGTH_MIN: &usize = INT_PTR_0_USIZE;

    const VALUE_LENGTH_MAX: usize = 100;
    const VALUE_LENGTH_MIN: &usize = INT_PTR_0_USIZE;

    // struct Strings field name validation code...
    // return true; ->  Valid Ok
    // return false; -> Valid Err
    // Valid Error eprintln(...); 
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
}

// struct Strings to_string custom print
impl Display for Strings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "struct Strings - (name={:?}, value={:?})",
            self.name, self.value
        )
    }
}

// struct Strings parse and validate
impl Json<Strings> for Strings {
    fn parse(json: &str) -> Vec<Strings> {
        let strings: Vec<Strings> = serde_json::from_str(json).unwrap();
        strings
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
                                "minLength": 1,
                                "maxLength": 100
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

    fn validate_data(vec: Vec<Strings>) -> bool {
        let mut valid = true;
        for e in vec {
            let name = e.name;
            let value = e.value;
            let name_valid = Strings::name_valid(name);
            let value_valid = Strings::value_valid(value);
            if !(name_valid && value_valid) {
                eprintln!("Struct String Validation Failed");
                valid = false;
                break;
            }
        }
        valid
    }
}


// struct Strings testing
#[cfg(test)]
pub mod tests {
    use std::fs;

    use crate::utils::json::Json;

    use super::Strings;

    // struct Strings Json parser and Validate testing
    #[test]
    fn strings_valid_test() {
        {
            let binding: String = fs::read_to_string("./../resources/strings.json").unwrap();
            let json: &str = binding.as_str();
            {
                assert!(!json.is_empty(), "file_contents not empty");
                let _: Result<(), ()> = Strings::validate(json);
            }
            {
                let vec: Vec<Strings> = Strings::parse(json);
                let valid: bool = Strings::validate_data(vec);
                assert!(valid);
            }
        }
        {
            let binding: String = fs::read_to_string("./../resources/strings2.json").unwrap();
            let json: &str = binding.as_str();
            {
                assert!(!json.is_empty(), "file_contents not empty");
                let _: Result<(), ()> = Strings::validate(json);
            }
            {
                let vec: Vec<Strings> = Strings::parse(json);
                let valid: bool = Strings::validate_data(vec);
                assert!(!valid);
            }
        }
    }
}
