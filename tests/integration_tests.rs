use std::fs;
use std::assert_matches;
use survey_tool_cli::*;

//helper
fn minimal_valid_yaml() -> String {
    // description >= 20 chars; page content >= 1 item with a valid question
    [
        "title: Test Survey Title",
        "description: A sufficiently long description for schema",
        "type: survey",
        "---",
        "content:",
        "- type: information",
        "  title: Info title",
    ]
    .join("\n")
}

// ---- config_check ----
#[test]
fn test_check_nonexistent_file() {
    let result = config_check("nonexistent.yaml").unwrap();

    assert!(!result.all_ok);
    assert_eq!(result.error_list.len(), 1);
    assert!(result.error_list[0].contains("File not found"));
}

#[test]
fn test_check_wrong_extension() {
    fs::write("tc_wrong_ext.txt", "test").unwrap();

    let result = config_check("tc_wrong_ext.txt").unwrap();

    assert!(!result.all_ok);
    assert!(result.error_list[0].contains("Yaml"));

    fs::remove_file("tc_wrong_ext.txt").ok();
}

#[test]
fn test_check_insufficient_documents() {
    fs::write("tc_single.yaml", "title: Test\n").unwrap();

    let result = config_check("tc_single.yaml").unwrap();

    assert!(!result.all_ok);
    assert!(result.error_list[0].contains("At least two"));

    fs::remove_file("tc_single.yaml").ok();
}

#[test]
fn test_check_valid_file_passes() {
    let content: &str = &minimal_valid_yaml();
    fs::write("tc_valid.yaml", content).unwrap();

    let result = config_check("tc_valid.yaml").unwrap();

    assert!(result.all_ok, "errors: {:?}", result.error_list);

    fs::remove_file("tc_valid.yaml").ok();
}

#[test]
fn test_check_missing_conditional_key() {
    // page references a conditional key that was never produced by a choice question
    let yaml = [
        "title: Test Survey Title",
        "description: A sufficiently long description for schema",
        "type: survey",
        "---",
        "conditional:",
        "  key: ghost_key",
        "  values:",
        "  - some_value",
        "content:",
        "- type: information",
        "  title: Info title",
    ]
    .join("\n");
    fs::write("tc_cond.yaml", yaml).unwrap();

    let result = config_check("tc_cond.yaml").unwrap();

    assert!(!result.all_ok);
    assert!(result.error_list.iter().any(|e| e.contains("ghost_key")));

    fs::remove_file("tc_cond.yaml").ok();
}

// ---- load_config ----
#[test]
fn test_load_config_not_found() {
    let err = load_config("missing.yaml").unwrap_err();
    assert!(err.to_string().contains("not found") || err.to_string().contains("IO"));
}

#[test]
fn test_load_config_wrong_extension() {
    let content: &str = &minimal_valid_yaml();
    fs::write("lc_ext.txt", content).unwrap();

    assert!(load_config("lc_ext.txt").is_err());

    fs::remove_file("lc_ext.txt").ok();
}

#[test]
fn test_load_config_single_document_fails() {
    fs::write("lc_single.yaml", "title: T\ndescription: D\ntype: survey\n").unwrap();

    assert!(load_config("lc_single.yaml").is_err());

    fs::remove_file("lc_single.yaml").ok();
}

#[test]
fn test_load_config_returns_correct_data() {
    let content: &str = &minimal_valid_yaml();
    fs::write("lc_valid.yaml", content).unwrap();

    let config = load_config("lc_valid.yaml").unwrap();

    assert_eq!(config.title, "Test Survey Title");
    assert_eq!(config.pages.len(), 1);

    fs::remove_file("lc_valid.yaml").ok();
}

// ---- save_config ----
fn minimal_config() -> SurveyConfig {
    let mut c = SurveyConfig::new("Saved".into(), "Desc".into(), None, None, None, None);
    c.add_page(SurveyPage::default());
    c
}

#[test]
fn test_save_config_writes_file() {
    let path = "sc_write.yaml";
    save_config(path, false, &minimal_config()).unwrap();
    assert!(fs::exists(path).unwrap());
    fs::remove_file(path).unwrap();
}

#[test]
fn test_save_config_no_overwrite_fails() {
    let path = "sc_nooverwrite.yaml";
    save_config(path, false, &minimal_config()).unwrap();
    assert!(save_config(path, false, &minimal_config()).is_err());
    fs::remove_file(path).unwrap();
}

#[test]
fn test_save_config_overwrite_succeeds() {
    let path = "sc_overwrite.yaml";
    save_config(path, false, &minimal_config()).unwrap();
    assert!(save_config(path, true, &minimal_config()).is_ok());
    fs::remove_file(path).unwrap();
}

#[test]
fn test_save_config_no_pages_fails() {
    let config = SurveyConfig::new("T".into(), "D".into(), None, None, None, None);
    let result = save_config("sc_nopages.yaml", false, &config);
    assert!(result.is_err());
    assert_matches!(result.unwrap_err(), STCError::YAMLFormat(_));
}

#[test]
fn test_save_config_wrong_extension_fails() {
    assert!(save_config("sc_bad.txt", false, &minimal_config()).is_err());
}

#[test]
fn test_save_load_roundtrip() {
    let path = "sc_roundtrip.yaml";
    let original = minimal_config();
    save_config(path, false, &original).unwrap();
    let loaded = load_config(path).unwrap();
    assert_eq!(loaded.title, original.title);
    assert_eq!(loaded.description, original.description);
    assert_eq!(loaded.pages.len(), original.pages.len());
    fs::remove_file(path).unwrap();
}
