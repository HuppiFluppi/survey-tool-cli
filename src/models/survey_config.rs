//! # Common models used within this crates modules
//! Other modules use them to work with survey configuration
//!
//! Translated from the survey-tool kotlin project and therefore not really rustacean

use core::fmt;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};

// helper functions to be used for serde default till https://github.com/serde-rs/serde/pull/3066 is available
fn bool_true() -> bool {
    true
}
fn bool_false() -> bool {
    false
}
fn survey_type() -> SurveyType {
    SurveyType::Survey
}
fn data_question_type() -> DataQuestionType {
    DataQuestionType::Name
}
fn datetime_type() -> DateTimeType {
    DateTimeType::DateTime
}
fn choice_limit() -> usize {
    2
}
fn likert_end() -> f64 {
    1.0
}
fn rating_level() -> usize {
    5
}
fn rating_symbol() -> RatingSymbol {
    RatingSymbol::Star
}
fn rating_gradient() -> RatingColorGradient {
    RatingColorGradient::None
}

/// Root configuration model describing a single survey or quiz.
/// This immutable data class is typically created by parsing a configuration source (e.g. via YAML)
#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyConfig {
    pub title: String,
    pub description: String,
    #[serde(rename = "type", default = "survey_type")]
    pub survey_type: SurveyType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<ScoreSettings>,
    #[serde(skip)]
    pub pages: Vec<SurveyPage>,
}

impl SurveyConfig {
    pub fn new(
        title: String,
        description: String,
        survey_type: Option<SurveyType>,
        image: Option<String>,
        background_image: Option<String>,
        score: Option<ScoreSettings>,
    ) -> SurveyConfig {
        SurveyConfig { title, description, survey_type: survey_type.unwrap_or(SurveyType::Survey), image, background_image, score, pages: Vec::new() }
    }

    /// add a page to the survey and returns its index
    pub fn add_page(&mut self, page: SurveyPage) -> usize {
        self.pages.push(page);
        self.pages.len() - 1
    }

    /// Removes a page from the survey and returns the removed page
    /// #Panics
    /// Panics if index is out of bounds.
    pub fn remove_page(&mut self, index: usize) -> SurveyPage {
        self.pages.remove(index)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SurveyType {
    Survey,
    Quiz,
}

impl fmt::Display for SurveyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SurveyType::Survey => write!(f, "survey"),
            SurveyType::Quiz => write!(f, "quiz"),
        }
    }
}

/// Global scoring options for a survey/quiz.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ScoreSettings {
    pub show_question_scores: bool,
    pub show_leaderboard: bool,
    pub leaderboard: LeaderboardSettings,
}

impl Default for ScoreSettings {
    /// Create settings with default values
    fn default() -> ScoreSettings {
        ScoreSettings { show_question_scores: false, show_leaderboard: true, leaderboard: LeaderboardSettings::default() }
    }
}

impl ScoreSettings {
    pub fn new(show_question_scores: bool, show_leaderboard: bool, leaderboard: LeaderboardSettings) -> ScoreSettings {
        ScoreSettings { show_question_scores, show_leaderboard, leaderboard }
    }
}

/// Leaderboard configuration details.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LeaderboardSettings {
    pub show_scores: bool,
    pub show_placeholder: bool,
    pub limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
}

impl Default for LeaderboardSettings {
    /// Create settings with default values
    fn default() -> LeaderboardSettings {
        LeaderboardSettings { show_scores: true, show_placeholder: true, limit: 10, background_image: None }
    }
}

impl LeaderboardSettings {
    pub fn new(show_scores: bool, show_placeholder: bool, limit: usize, background_image: Option<String>) -> LeaderboardSettings {
        LeaderboardSettings { show_scores, show_placeholder, limit, background_image }
    }
}

/// Settings for conditional display
#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionalSettings {
    pub key: String,
    pub values: Vec<String>,
}

impl ConditionalSettings {
    pub fn new(key: String, values: Vec<String>) -> ConditionalSettings {
        ConditionalSettings { key, values }
    }
}

/// A single page in the survey.
#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<ConditionalSettings>,
    pub content: Vec<SurveyPageContent>,
}

impl Default for SurveyPage {
    /// Create page with default values
    fn default() -> SurveyPage {
        SurveyPage { title: None, description: None, image: None, conditional: None, content: Vec::new() }
    }
}

impl SurveyPage {
    pub fn new(title: Option<String>, description: Option<String>, image: Option<String>, conditional: Option<ConditionalSettings>) -> SurveyPage {
        SurveyPage { title, description, image, conditional, content: Vec::new() }
    }

    // Add content to a survey page and returns the index
    pub fn add_content(&mut self, content: SurveyPageContent) -> usize {
        self.content.push(content);
        self.content.len() - 1
    }

    // Insert content to a survey page
    pub fn insert_content(&mut self, index: usize, content: SurveyPageContent) {
        self.content.insert(index, content);
    }

    // Remove content from a survey page, returning the removed element
    pub fn remove_content(&mut self, index: usize) -> SurveyPageContent {
        self.content.remove(index)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurveyPageContentHeader {
    // While type is part of the header, we need to use serde internally tagged enum representation to properly determine the enum variant.
    // This would clash with this definition. Ergo, type will end up in the yaml, but based on SurveyPageContent config and enum variant
    // #[serde(rename = "type")]
    // pub content_type: SurveyContentType,
    pub title: String,
    #[serde(default = "bool_true")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<ConditionalSettings>,
}

impl Default for SurveyPageContentHeader {
    fn default() -> Self {
        Self { title: Default::default(), required: true, conditional: None }
    }
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
// pub enum SurveyContentType {
//     Text,
//     Choice,
//     Data,
//     Rating,
//     Likert,
//     Information,
//     DateTime,
//     Slider,
// }

#[derive(Debug, Serialize, Deserialize, EnumString, Display, VariantNames)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum SurveyPageContent {
    /// Free-text question.
    Text {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: TextConfig,
    },

    /// Choice-based question.
    Choice {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: ChoiceConfig,
    },

    /// Question for capturing participant’s details
    Data {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: DataConfig,
    },

    /// DateTime question.
    DateTime {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: DateTimeConfig,
    },

    /// Numeric rating question.
    Rating {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: RatingConfig,
    },

    /// Slider question.
    Slider {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: SliderConfig,
    },

    /// Likert scale question.
    Likert {
        #[serde(flatten)]
        header: SurveyPageContentHeader,
        config: LikertConfig,
    },

    Information {
        #[serde(flatten)]
        header: SurveyPageContentHeader,

        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        image: Option<String>,
    },
}

impl SurveyPageContent {
    pub fn get_header(&self) -> &SurveyPageContentHeader {
        match self {
            SurveyPageContent::Text { header, .. } => header,
            SurveyPageContent::Choice { header, .. } => header,
            SurveyPageContent::Data { header, .. } => header,
            SurveyPageContent::DateTime { header, .. } => header,
            SurveyPageContent::Rating { header, .. } => header,
            SurveyPageContent::Slider { header, .. } => header,
            SurveyPageContent::Likert { header, .. } => header,
            SurveyPageContent::Information { header, .. } => header,
        }
    }

    pub fn get_header_mut(&mut self) -> &mut SurveyPageContentHeader {
        match self {
            SurveyPageContent::Text { header, .. } => header,
            SurveyPageContent::Choice { header, .. } => header,
            SurveyPageContent::Data { header, .. } => header,
            SurveyPageContent::DateTime { header, .. } => header,
            SurveyPageContent::Rating { header, .. } => header,
            SurveyPageContent::Slider { header, .. } => header,
            SurveyPageContent::Likert { header, .. } => header,
            SurveyPageContent::Information { header, .. } => header,
        }
    }

    pub fn format_config(&self) -> String {
        match self {
            SurveyPageContent::Text { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::Choice { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::Data { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::DateTime { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::Rating { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::Slider { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::Likert { config, .. } => SurveyPageContent::format_fields(config),
            SurveyPageContent::Information { description, image, .. } => {
                let mut parts = Vec::new();
                if let Some(desc) = description {
                    parts.push(format!("description: {}", desc));
                }
                if let Some(image) = image {
                    parts.push(format!("image: {}", image));
                }
                parts.join("\n")
            },
        }
    }

    fn format_fields<T: Serialize>(value: &T) -> String {
        serde_saphyr::to_string(value).unwrap_or_default()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TextConfig {
    #[serde(default = "bool_false")]
    pub multiline: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer_pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer_list: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataConfig {
    #[serde(default = "data_question_type")]
    pub datatype: DataQuestionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_pattern: Option<String>,
    #[serde(default = "bool_true")]
    pub use_for_leaderboard: bool,
}

impl Default for DataConfig {
    fn default() -> Self {
        Self { datatype: Default::default(), validation_pattern: Default::default(), use_for_leaderboard: true }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DateTimeConfig {
    #[serde(default = "datetime_type")]
    pub input_type: DateTimeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_selected_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_selected_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_time_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_date_answer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChoiceConfig {
    #[serde(default = "bool_false")]
    pub multiple: bool,
    #[serde(default = "choice_limit")]
    pub limit: usize,
    #[serde(default = "bool_false")]
    pub dropdown: bool,
    #[serde(default = "bool_false")]
    pub horizontal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_key: Option<String>,
    pub choices: Vec<ChoiceItem>,
}

impl Default for ChoiceConfig {
    fn default() -> Self {
        Self {
            multiple: Default::default(),
            limit: 2,
            dropdown: Default::default(),
            horizontal: true,
            conditional_key: Default::default(),
            choices: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LikertConfig {
    pub choices: Vec<String>,
    pub statements: Vec<LikertStatement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingConfig {
    #[serde(default = "rating_level")]
    pub level: usize,
    #[serde(default = "rating_symbol")]
    pub symbol: RatingSymbol,
    #[serde(default = "rating_gradient")]
    pub color_gradient: RatingColorGradient,
}

impl Default for RatingConfig {
    fn default() -> Self {
        Self { level: 5, symbol: Default::default(), color_gradient: Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SliderConfig {
    #[serde(default = "bool_false")]
    pub range: bool,
    #[serde(default)]
    pub start: f64,
    #[serde(default = "likert_end")]
    pub end: f64,
    #[serde(default)]
    pub steps: usize,
    #[serde(default = "bool_false")]
    pub show_decimals: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_answer: Option<f64>,
}

impl Default for SliderConfig {
    fn default() -> Self {
        Self {
            range: Default::default(),
            start: Default::default(),
            end: 1.0,
            steps: Default::default(),
            show_decimals: Default::default(),
            unit: Default::default(),
            score: Default::default(),
            correct_answer: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChoiceItem {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(default = "bool_false")]
    pub correct: bool,
}

impl fmt::Display for ChoiceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - score: {:?} - correct: {}", self.title, self.score, self.correct)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, VariantNames, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum DataQuestionType {
    #[default]
    Name,
    Email,
    Phone,
    Custom,
    Nickname,
    Age,
    Birthday,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, VariantNames, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum DateTimeType {
    Date,
    Time,
    #[default]
    DateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, VariantNames, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum RatingSymbol {
    #[default]
    Star,
    Heart,
    Like,
    Smile,
    Number,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, VariantNames, Display, EnumString)]
#[serde(rename_all = "lowercase")]
pub enum RatingColorGradient {
    #[default]
    None,
    Red2Green,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct LikertStatement {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_choice: Option<String>,
}

impl fmt::Display for LikertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - score: {:?} - correct: {:?}", self.title, self.score, self.correct_choice)
    }
}

// ------- Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survey_config_new() {
        let config = SurveyConfig::new("Test Survey".to_string(), "Description".to_string(), None, None, None, None);
        assert_eq!(config.title, "Test Survey");
        assert_eq!(config.description, "Description");
        assert!(config.pages.is_empty());
    }

    #[test]
    fn test_survey_config_add_remove_page() {
        let mut config = SurveyConfig::new("Test".to_string(), "Desc".to_string(), None, None, None, None);
        let page = SurveyPage::default();
        config.add_page(page);
        assert_eq!(config.pages.len(), 1);
        config.remove_page(0);
        assert_eq!(config.pages.len(), 0);
    }

    #[test]
    fn test_score_settings_default() {
        let settings = ScoreSettings::default();
        assert!(!settings.show_question_scores);
        assert!(settings.show_leaderboard);
    }

    #[test]
    fn test_leaderboard_settings_default() {
        let settings = LeaderboardSettings::default();
        assert!(settings.show_scores);
        assert!(settings.show_placeholder);
        assert_eq!(settings.limit, 10);
    }

    #[test]
    fn test_survey_page_add_remove_content() {
        let mut page = SurveyPage::default();
        let content = SurveyPageContent::Information {
            header: SurveyPageContentHeader { title: "Info".to_string(), required: false, conditional: None },
            description: None,
            image: None,
        };
        page.add_content(content);
        assert_eq!(page.content.len(), 1);
        page.remove_content(0);
        assert_eq!(page.content.len(), 0);
    }
}
