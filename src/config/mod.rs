//! # Config module
//!
//! Loads and saves survey tool configuration files (yaml).

use regex::RegexBuilder;

use crate::models::error::STCError;
use crate::models::survey_config::*;
use std::io;
use std::{fs, path};

pub fn load(file: &path::Path) -> Result<SurveyConfig, STCError> {
    //check file exists
    if !fs::exists(file)? {
        return Err(STCError::IO(io::Error::new(std::io::ErrorKind::NotFound, format!("File '{}' not found", file.display()))));
    }

    //check file extension
    check_file_ext(file)?;

    //load file
    let file_content = fs::read_to_string(file)?;
    let document_divider = RegexBuilder::new(r"^---").multi_line(true).build().unwrap();
    let documents: Vec<&str> = document_divider.split(&file_content).filter(|s| !s.is_empty()).collect();
    if documents.len() < 2 {
        return Err(STCError::YAMLFormat(format!(
            "File '{}' is missing the right number of YAML documents. Minimum: 2, found: {}",
            file.display(),
            documents.len()
        )));
    }

    let mut config: SurveyConfig = serde_saphyr::from_str(documents.first().unwrap())?;
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
            std::io::ErrorKind::AlreadyExists,
            format!("File '{}' already exists and overwrite flag is missing", file.display()),
        )));
    }

    //check extension
    check_file_ext(file)?;

    //check config
    if config.pages.is_empty() {
        return Err(STCError::YAMLFormat("Config malformed without a single page".to_string()));
    }

    //save file
    let mut documents = Vec::with_capacity(config.pages.len());
    documents.push(serde_saphyr::to_string(config)?);
    for page in config.pages.iter() {
        documents.push(serde_saphyr::to_string(page)?);
    }

    fs::write(file, documents.join("\n---\n"))?;

    Ok(())
}

fn check_file_ext(file: &path::Path) -> Result<(), STCError> {
    let Some(ext) = file.extension() else {
        return Err(STCError::IO(io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!("File '{}' has no extension (.yaml or .yml needed)", file.display()),
        )));
    };
    let ext = ext.to_ascii_lowercase();
    if ext != "yaml" && ext != "yml" {
        return Err(STCError::IO(io::Error::new(std::io::ErrorKind::Unsupported, format!("File '{}' has no Yaml extension (.yaml or .yml)", file.display()))));
    }
    Ok(())
}
