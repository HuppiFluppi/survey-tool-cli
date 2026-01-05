//! # Config check module
//!
//! Checks survey tool configuration files (yaml) for correctnes according to schema.
//! The schema is embedded at build time.
//!
//! As the Survey Tool is a Kotlin JVM application with Compose runtime, some restrictions apply.

use crate::models::result;
use std::{error::Error, fs};

static SCHEMA: &str = include_str!("schema.json");

/// check a given file for correctness
pub fn check(file: &str) -> Result<result::CheckResult, Box<dyn Error>> {
    //check file
    fs::exists(file)?;
    if !file.ends_with(".yaml") && !file.ends_with(".yml") {
        let mut ret = result::CheckResult::not_ok();
        ret.error_list.push("File has no Yaml extension (.yaml or .yml)".to_string());
        return Ok(ret);
    }

    //load file
    let instances: Vec<serde_json::Value> = serde_saphyr::from_multiple(&fs::read_to_string(file)?)?;

    //check available documents
    if instances.len() < 2 {
        let mut ret = result::CheckResult::not_ok();
        ret.error_list.push(format!("Found only {} documents in yaml. At least two (survey header and one page) are needed", instances.len()));
        return Ok(ret);
    }

    //check documents
    let validator = jsonschema::validator_for(&serde_json::from_str(SCHEMA)?)?;
    let mut result = result::CheckResult::all_ok();

    for (i, x) in instances.iter().enumerate() {
        check_document(x, &validator, &mut result, i)?;
    }

    Ok(result)
}

fn check_document(instance: &serde_json::Value, validator: &jsonschema::Validator, result: &mut result::CheckResult, document: usize) -> Result<(), Box<dyn Error>> {
    let evaluation = validator.evaluate(instance);

    if evaluation.flag().valid {
        Ok(())
    } else {
        result.all_ok = false;

        evaluation.iter_errors().for_each(|x| {
            result.error_list.push(format!(
                "{} (document {}, loc {} - schema {})",
                x.error,
                document + 1,
                x.instance_location,
                x.schema_location
            ))
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_validator() -> jsonschema::Validator {
        let schema = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "type": "object",
            "properties": {
                "productId": {
                    "type": "integer"
                },
                "productName": {
                    "type": "string"
                }
            },
            "required": [
                "productId"
            ]
        });
        jsonschema::validator_for(&schema).unwrap()
    }

    #[test]
    fn test_one_wrong_config() {
        let instance = json!({
            //"productId": "1",
            "productName": "A green door"
        });

        let result = &mut result::CheckResult::all_ok();
        check_document(&instance, &get_validator(), result, 0).unwrap();

        assert!(!result.all_ok);
        assert_eq!(result.error_list.len(), 1);
    }

    #[test]
    fn test_two_wrong_config() {
        let instance = json!({
            "productId": "1",
            "productName": true
        });

        let result = &mut result::CheckResult::all_ok();
        check_document(&instance, &get_validator(), result, 0).unwrap();

        assert!(!result.all_ok);
        assert_eq!(result.error_list.len(), 2);
    }

    #[test]
    fn test_right_config() {
        let instance = json!({
            "productId": 1,
            "productName": "A green door"
        });

        let result = &mut result::CheckResult::all_ok();
        check_document(&instance, &get_validator(), result, 0).unwrap();

        assert!(result.all_ok);
        assert_eq!(result.error_list.len(), 0);
    }
}
