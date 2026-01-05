//! # Setup check module
//!
//! Checks host for needed prerequisites to run the survey tool application
//!
//! As the Survey Tool is a Kotlin JVM application with Compose runtime, some restrictions apply.

use regex::Regex;
use std::env;
use std::error::Error;
use std::process;

use crate::models::result;

const MINIMUM_JAVA_VERSION: usize = 21;

/// Check the system for compatibility
pub fn check() -> Result<result::CheckResult, Box<dyn Error>> {
    let mut result = result::CheckResult::all_ok();

    //check operating system
    match env::consts::OS {
        "windows" | "linux" => result.success_list.push("Supported operating system found".to_string()),
        _ => {
            result.all_ok = false;
            result.error_list.push(format!("Only Windows and Linux supported as operating system. Found '{}'", env::consts::OS));
        }
    }

    //check java installation
    let output = match process::Command::new("java").arg("--version").output() {
        Ok(out) if out.status.success() => Some(String::from_utf8_lossy(&out.stdout).to_string()),
        _ => {
            if let Ok(java_home) = env::var("JAVA_HOME") {
                let java_path = format!("{java_home}/bin/java");
                match process::Command::new(java_path).arg("--version").output() {
                    Ok(out) if out.status.success() => Some(String::from_utf8_lossy(&out.stdout).to_string()),
                    _ => None,
                }
            } else {
                None
            }
        }
    };

    if output.is_none() {
        result.all_ok = false;
        result.error_list.push("Java not found. Please install Java or set JAVA_HOME".to_string());
    } else {
        result.success_list.push("Java installation found".to_string());
    }

    //check java version
    if let Some(out) = output {
        let re = Regex::new(r" (\d+).(\d+).(\d+) ").unwrap();
        match re.captures(&out) {
            Some(captures) => {
                let found = captures.get_match().as_str().trim();
                if captures[1].parse::<usize>().unwrap_or_default() < MINIMUM_JAVA_VERSION {
                    result.all_ok = false;
                    result.error_list.push(format!("Installed Java version too low. Minimum needed: {MINIMUM_JAVA_VERSION} - found: {found}"));
                } else {
                    result.success_list.push(format!("Installed Java version good. Minimum needed: {MINIMUM_JAVA_VERSION} - found: {found}"));
                }
            }
            None => {
                result.all_ok = false;
                result.error_list.push("Could not detect any Java version".to_string());
            }
        }
    } else {
        result.all_ok = false;
        result.error_list.push("Java not found. Cant check it's version".to_string());
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_good() {
        let result = check().unwrap();
        assert!(result.all_ok);
        assert!(result.error_list.is_empty());
    }
}
