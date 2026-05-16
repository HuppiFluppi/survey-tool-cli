//! # Config check module
//!
//! Checks survey tool configuration files (yaml) for correctnes according to schema.
//! The schema is embedded at build time.

use crate::models::error;
use crate::models::result;
use std::{fs, path::Path};

// For now, the schema.json is embedded in the app.
// Even more, the logic in the actual survey tool is not based on the schema.
// This might lead to problems in the future.
static SCHEMA: &str = include_str!("schema.json");

/// check a given file for correctness
pub fn check(file: &Path) -> Result<result::CheckResult, error::STCError> {
    //check file exists
    if !fs::exists(file)? {
        let mut ret = result::CheckResult::not_ok();
        ret.error_list.push("File not found".to_string());
        return Ok(ret);
    }

    //check file extension
    let Some(ext) = file.extension() else {
        let mut ret = result::CheckResult::not_ok();
        ret.error_list.push("File has no extension (.yaml or .yml needed)".to_string());
        return Ok(ret);
    };
    let ext = ext.to_ascii_lowercase();
    if ext != "yaml" && ext != "yml" {
        let mut ret = result::CheckResult::not_ok();
        ret.error_list.push("File has no Yaml extension (.yaml or .yml)".to_string());
        return Ok(ret);
    }

    //load file
    let instances: Vec<serde_json::Value> = serde_saphyr::from_multiple(&fs::read_to_string(file)?)?;

    //check available documents
    if instances.len() < 2 {
        let mut ret = result::CheckResult::not_ok();
        ret.error_list.push(format!(
            "Found only {} documents in yaml. At least two (survey header and one page) are needed",
            instances.len()
        ));
        return Ok(ret);
    }

    //check documents
    let validator = jsonschema::validator_for(&serde_json::from_str(SCHEMA)?)?;
    let mut result = result::CheckResult::all_ok();

    for (i, x) in instances.iter().enumerate() {
        check_document(x, &validator, &mut result, i, file)?;
    }

    Ok(result)
}

fn check_document(
    instance: &serde_json::Value,
    validator: &jsonschema::Validator,
    result: &mut result::CheckResult,
    document: usize,
    file: &Path,
) -> Result<(), error::STCError> {
    let evaluation = validator.evaluate(instance);

    if evaluation.flag().valid {
        check_files(instance, result, document, file)?;
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

fn check_files(instance: &serde_json::Value, result: &mut result::CheckResult, document: usize, template: &Path) -> Result<(), error::STCError> {
    const FILE_POINTERS: [&str; 3] = [
        "/background_image",
        "/image", //maps survey level image and page image element
        "/score/leaderboard/background_image",
    ];

    // check for no content images
    for p in FILE_POINTERS {
        if let Some(serde_json::Value::String(file)) = instance.pointer(p)
            && !fs::exists(template.parent().unwrap().join(file))?
        {
            result.all_ok = false;
            result.error_list.push(format!("File '{file}', referenced at '{p}', not found"));
        }
    }

    // check for information block image
    if let Some(serde_json::Value::Array(array)) = instance.pointer("/content") {
        for (i, c) in array.iter().enumerate() {
            if let serde_json::Value::Object(o) = c
                && let Some(serde_json::Value::String(s)) = o.get("type")
                && s == "information"
            {
                if let Some(serde_json::Value::String(file)) = o.get("image")
                    && !fs::exists(template.parent().unwrap().join(file))?
                {
                    result.all_ok = false;
                    result
                        .error_list
                        .push(format!("File '{file}', referenced at page {} - element {}, not found", document + 1, i + 1));
                }
            }
        }
    }

    Ok(())
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
        check_document(&instance, &get_validator(), result, 0, Path::new("/test/file.yml")).unwrap();

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
        check_document(&instance, &get_validator(), result, 0, Path::new("/test/file.yml")).unwrap();

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
        check_document(&instance, &get_validator(), result, 0, Path::new("/test/file.yml")).unwrap();

        assert!(result.all_ok);
        assert_eq!(result.error_list.len(), 0);
    }
}
