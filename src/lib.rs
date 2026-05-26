//! # Survey Tool CLI library
//!
//! Supports in handling [**Survey Tool**](https://github.com/HuppiFluppi/survey-tool) configuration files
//!
//! Current commands:
//! - **config_check**: Check an existing survey tool configuration yaml for correctness
//! - **setup_check**: Check local host for prerequisite to run survey tool
//! - *edit (not yet): Allow to edit(add/remove/change) a survey tool config from command line*
//!
//! This lib crate provides the functionality to the binary crate in the same repository.

use std::path::Path;

mod check;
mod config;
mod models;
mod setup;

pub use crate::models::survey_config::{ConditionalSettings, SurveyConfig, SurveyPage, SurveyType};
pub use models::error::STCError;
pub use models::result::CheckResult;

/// Run setup checks to see if local machine can execute the survey tool application
pub fn setup_check() -> Result<CheckResult, STCError> {
    setup::check()
}

/// Validate an existing survey tool configuration file/yaml.
/// file: path to the configuration file
pub fn config_check(file: &str) -> Result<CheckResult, STCError> {
    check::check(Path::new(file))
}

pub fn load_config(file: &str) -> Result<SurveyConfig, STCError> {
    config::load(Path::new(file))
}

pub fn save_config(file: &str, overwrite: bool, config: &SurveyConfig) -> Result<(), STCError> {
    config::save(Path::new(file), overwrite, config)
}

// pub fn get_page_by_title(file: &str, title: &str) -> Result<SurveyPage, Error>;
// pub fn get_page_by_îndex(file: &str, index: usize) -> Result<SurveyPage, Error>;
// pub fn update_page(file: &str, add_if_missing: bool, page: &SurveyPage) -> Result<(), Error>;
// pub fn remove_page_by_title(file: &str, title: &str) -> Result<(), Error>;
// pub fn remove_page_by_index(file: &str, index: usize) -> Result<(), Error>;
