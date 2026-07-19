//! # Config module
//!
//! Loads and saves survey tool configuration files (yaml).

use regex::RegexBuilder;

use crate::models::error::STCError;
use crate::models::survey_config::SurveyConfig;
use std::io;
use std::{fs, path};

pub fn load(file: &path::Path) -> Result<SurveyConfig, STCError> {
    //check file exists
    if !fs::exists(file)? {
        return Err(STCError::IO(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' not found", file.display()))));
    }

    //check file extension
    check_file_ext(file)?;

    //load config from file
    let file_content = fs::read_to_string(file)?;
    load_from_string(&file_content)
}

pub fn load_from_string(config: &str) -> Result<SurveyConfig, STCError> {
    let document_divider = RegexBuilder::new(r"^---").multi_line(true).build().unwrap();
    let documents: Vec<&str> = document_divider.split(config).filter(|s| !s.is_empty()).collect();
    if documents.len() < 2 {
        return Err(STCError::YAMLFormat(format!("Config is missing the right number of YAML documents. Minimum: 2, found: {}", documents.len())));
    }

    let mut config: SurveyConfig = serde_saphyr::from_str(documents[0])?;
    for page_document in documents.iter().skip(1) {
        let page = serde_saphyr::from_str(page_document)?;
        config.add_page(page);
    }

    Ok(config)
}

pub fn save(file: &path::Path, overwrite: bool, config: &SurveyConfig) -> Result<(), STCError> {
    //check file exists
    if fs::exists(file)? && !overwrite {
        return Err(STCError::IO(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("File '{}' already exists and overwrite flag is missing", file.display()),
        )));
    }

    //check extension
    check_file_ext(file)?;

    // serialize and save to file
    let serialized_str = serialize_config(config)?;
    fs::write(file, serialized_str)?;

    Ok(())
}

pub fn serialize_config(config: &SurveyConfig) -> Result<String, STCError> {
    //check config
    if config.pages.is_empty() {
        return Err(STCError::YAMLFormat("Config malformed without a single page".to_string()));
    }

    //serialize
    let mut documents = Vec::with_capacity(config.pages.len() + 1);
    documents.push(serde_saphyr::to_string(config)?);
    for page in &config.pages {
        documents.push(serde_saphyr::to_string(page)?);
    }

    Ok(documents.join("\n---\n"))
}

fn check_file_ext(file: &path::Path) -> Result<(), STCError> {
    let Some(ext) = file.extension() else {
        return Err(STCError::IO(io::Error::new(io::ErrorKind::Unsupported, format!("File '{}' has no extension (.yaml or .yml needed)", file.display()))));
    };
    let ext = ext.to_ascii_lowercase();
    if ext != "yaml" && ext != "yml" {
        return Err(STCError::IO(io::Error::new(io::ErrorKind::Unsupported, format!("File '{}' has no Yaml extension (.yaml or .yml)", file.display()))));
    }
    Ok(())
}
