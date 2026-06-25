pub mod survey_config;

pub mod result {

    /// Result for check operations
    #[derive(Debug, PartialEq)]
    pub struct CheckResult {
        /// simple flag to specify if the operation encountered any errors/problems
        pub all_ok: bool,

        /// list of all successful checks
        pub success_list: Vec<String>,

        /// list of all errors found
        pub error_list: Vec<String>,

        /// optional additional output
        pub output: Option<String>,
    }
    impl CheckResult {
        /// create an empty [CheckResult] with `all_ok` true
        #[must_use]
        pub fn all_ok() -> CheckResult {
            CheckResult { all_ok: true, success_list: Vec::new(), error_list: Vec::new(), output: None }
        }

        /// create an empty [CheckResult] with `all_ok` false
        #[must_use]
        pub fn not_ok() -> CheckResult {
            CheckResult { all_ok: false, success_list: Vec::new(), error_list: Vec::new(), output: None }
        }
    }
}

pub mod error {
    use std::error;
    use std::fmt;

    /// Error type used throughout survey tool cli
    #[derive(Debug)]
    pub enum STCError {
        Unspecified,
        YAMLParse(serde_saphyr::Error),
        YAMLFormat(String),
        YAMLSerialization(serde_saphyr::ser::Error),
        SchemaLoad(serde_json::Error),
        SchemaValidation(jsonschema::ValidationError<'static>),
        IO(std::io::Error),
    }

    impl fmt::Display for STCError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                STCError::Unspecified => write!(f, "Unspecified error"),
                STCError::YAMLParse(error) => write!(f, "YAML parse error: {error}"),
                STCError::YAMLFormat(message) => write!(f, "YAML format error: {message}"),
                STCError::YAMLSerialization(error) => write!(f, "YAML serialization error: {error}"),
                STCError::SchemaLoad(error) => write!(f, "Error loading schema: {error}"),
                STCError::SchemaValidation(error) => write!(f, "Error on schema validation: {error}"),
                STCError::IO(error) => write!(f, "IO error: {error}"),
            }
        }
    }

    impl error::Error for STCError {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self {
                STCError::Unspecified => None,
                STCError::YAMLParse(error) => Some(error),
                STCError::YAMLFormat(_) => None,
                STCError::YAMLSerialization(error) => Some(error),
                STCError::SchemaLoad(error) => Some(error),
                STCError::SchemaValidation(error) => Some(error),
                STCError::IO(error) => Some(error),
            }
        }
    }

    impl From<std::io::Error> for STCError {
        fn from(value: std::io::Error) -> Self {
            STCError::IO(value)
        }
    }

    impl From<serde_saphyr::Error> for STCError {
        fn from(value: serde_saphyr::Error) -> Self {
            STCError::YAMLParse(value)
        }
    }

    impl From<serde_saphyr::ser::Error> for STCError {
        fn from(value: serde_saphyr::ser::Error) -> Self {
            STCError::YAMLSerialization(value)
        }
    }

    impl From<serde_json::Error> for STCError {
        fn from(value: serde_json::Error) -> Self {
            STCError::SchemaLoad(value)
        }
    }

    impl From<jsonschema::ValidationError<'static>> for STCError {
        fn from(value: jsonschema::ValidationError<'static>) -> Self {
            STCError::SchemaValidation(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::result::*;

    #[test]
    fn test_all_ok_creates_valid_result() {
        let result = CheckResult::all_ok();
        assert!(result.all_ok);
        assert!(result.success_list.is_empty());
        assert!(result.error_list.is_empty());
        assert!(result.output.is_none());
    }

    #[test]
    fn test_not_ok_creates_valid_result() {
        let result = CheckResult::not_ok();
        assert!(!result.all_ok);
        assert!(result.success_list.is_empty());
        assert!(result.error_list.is_empty());
        assert!(result.output.is_none());
    }
}
