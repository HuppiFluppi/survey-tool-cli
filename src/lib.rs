//! # Survey Tool CLI library
//!
//! Supports in handling [**Survey Tool**](https://github.com/HuppiFluppi/survey-tool) configuration files
//!
//! Current commands:
//! - **config_check**: Check an existing survey tool configuration yaml for correctness
//! - **setup_check**: Check local host for prerequisite to run survey tool
//! - **config**: Allow to load & save a survey tool config from command line*
//!
//! This lib crate provides the functionality to the binary crate in the same repository.

use std::path::Path;

mod check;
mod config;
mod models;
mod setup;

pub use crate::models::survey_config::*;
pub use models::error::STCError;
pub use models::result::CheckResult;

/// Run setup checks to see if local machine can execute the survey tool application
pub fn setup_check() -> Result<CheckResult, STCError> {
    setup::check()
}

/// Validate an existing survey tool configuration file(yaml).
///
/// *file*: path to the configuration file
pub fn config_check(file: &str) -> Result<CheckResult, STCError> {
    check::check(Path::new(file))
}

/// Load an existing survey tool configuration file(yaml) and return its representation ([SurveyConfig]).
///
/// *file*: path to the configuration file
pub fn load_config(file: &str) -> Result<SurveyConfig, STCError> {
    config::load(Path::new(file))
}

/// Load an existing survey tool configuration from string (yaml format) and return its representation ([SurveyConfig]).
///
/// *config*: content of a survey configuration
pub fn load_config_from_string(config: &str) -> Result<SurveyConfig, STCError> {
    config::load_from_string(config)
}

/// Save survey tool configuration model([SurveyConfig]).
///
/// *file*: path to the configuration file
/// *overwrite*: whether to overwrite. will fail if file exists otherwise
/// *config*: the config to save
pub fn save_config(file: &str, overwrite: bool, config: &SurveyConfig) -> Result<(), STCError> {
    config::save(Path::new(file), overwrite, config)
}

/// Serialize a survey tool configuration model([SurveyConfig]).
///
/// *config*: the config to save
pub fn serialize_config(config: &SurveyConfig) -> Result<String, STCError> {
    config::serialize_config(config)
}
