use std::fs;
use std::io::Write;
use survey_tool_cli::*;

#[test]
fn test_check_nonexistent_file() {
    let result = config_check("nonexistent.yaml").unwrap();
    assert!(!result.all_ok);
    assert_eq!(result.error_list.len(), 1);
    assert!(result.error_list[0].contains("File not found"));
}

#[test]
fn test_check_wrong_extension() {
    let mut file = fs::File::create("test.txt").unwrap();
    writeln!(file, "test").unwrap();

    let result = config_check("test.txt").unwrap();
    assert!(!result.all_ok);
    assert!(result.error_list[0].contains("Yaml"));

    fs::remove_file("test.txt").ok();
}

#[test]
fn test_check_insufficient_documents() {
    let mut file = fs::File::create("test_single.yaml").unwrap();
    writeln!(file, "title: Test").unwrap();

    let result = config_check("test_single.yaml").unwrap();
    assert!(!result.all_ok);
    assert!(result.error_list[0].contains("At least two"));

    fs::remove_file("test_single.yaml").ok();
}
