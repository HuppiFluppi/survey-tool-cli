//! # Survey Tool CLI library
//!
//! Supports in handling [**Survey Tool**](https://github.com/HuppiFluppi/survey-tool) configuration files
//!
//! Current commands:
//! - **config_check**: Check an existing survey tool configuration yaml for correctness
//! - **setup_check**: Check local host for prerequisits to run survey tool
//! - *edit (not yet): Allow to edit(add/remove/change) a survey tool config from command line*
//!
//! This lib crate provides the functionality to the binary crate in the same repository.

use std::error::Error;

mod check;
mod edit;
mod models;
mod setup;

pub use models::result::CheckResult;

/// Run setup checks to see if local machine can execute the survey tool application
pub fn setup_check() -> Result<CheckResult, Box<dyn Error>> {
    setup::check()
}

/// Validate an existing survey tool configuration file/yaml.
/// 
/// file: path to the configuration file
pub fn config_check(file: &str) -> Result<CheckResult, Box<dyn Error>> {
    check::check(file)
}
