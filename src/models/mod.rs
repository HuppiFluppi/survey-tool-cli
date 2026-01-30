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
        /// create an empty CheckResult with `all_ok` true
        pub fn all_ok() -> CheckResult {
            CheckResult {
                all_ok: true,
                success_list: Vec::new(),
                error_list: Vec::new(),
                output: None,
            }
        }

        /// create an empty CheckResult with `all_ok` false
        pub fn not_ok() -> CheckResult {
            CheckResult {
                all_ok: false,
                success_list: Vec::new(),
                error_list: Vec::new(),
                output: None,
            }
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
