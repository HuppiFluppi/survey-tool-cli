//! # Common models used within this crates modules
//! Other modules use them to work with survey configuration
//!
//! Translated from the survey-tool kotlin project and therefore not really rustacean

/// Root configuration model describing a single survey or quiz.
/// This immutable data class is typically created by parsing a configuration source (e.g. via YAML)
pub struct SurveyConfig {
    pub title: String,
    pub description: String,
    pub survey_type: SurveyType,
    pub image_path: Option<String>,
    pub score: ScoreSettings,
    pub pages: Vec<SurveyPage>,
}

impl SurveyConfig {
    pub fn new(title: String, description: String, survey_type: Option<SurveyType>, image_path: Option<String>, score: Option<ScoreSettings>) -> SurveyConfig {
        SurveyConfig {
            title,
            description,
            survey_type: survey_type.unwrap_or(SurveyType::Survey),
            image_path,
            score: score.unwrap_or_default(),
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page: SurveyPage) {
        self.pages.push(page);
    }

    pub fn remove_page(&mut self, index: usize) {
        self.pages.remove(index);
    }
}

pub enum SurveyType {
    Survey,
    Quiz,
}

/// Global scoring options for a survey/quiz.
pub struct ScoreSettings {
    pub show_question_scores: bool,
    pub show_leaderboard: bool,
    pub leaderboard: LeaderboardSettings,
}

impl Default for ScoreSettings {
    /// Create settings with default values
    fn default() -> ScoreSettings {
        ScoreSettings {
            show_question_scores: false,
            show_leaderboard: true,
            leaderboard: LeaderboardSettings::default(),
        }
    }
}

impl ScoreSettings {
    pub fn new(show_question_scores: bool, show_leaderboard: bool, leaderboard: LeaderboardSettings) -> ScoreSettings {
        ScoreSettings {
            show_question_scores,
            show_leaderboard,
            leaderboard,
        }
    }
}

/// Leaderboard configuration details.
pub struct LeaderboardSettings {
    pub show_scores: bool,
    pub show_placeholder: bool,
    pub limit: usize,
}

impl Default for LeaderboardSettings {
    /// Create settings with default values
    fn default() -> LeaderboardSettings {
        LeaderboardSettings {
            show_scores: true,
            show_placeholder: true,
            limit: 10,
        }
    }
}

impl LeaderboardSettings {
    pub fn new(show_scores: bool, show_placeholder: bool, limit: usize) -> LeaderboardSettings {
        LeaderboardSettings {
            show_scores,
            show_placeholder,
            limit,
        }
    }
}

/// A single page in the survey.
pub struct SurveyPage {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_path: Option<String>,
    pub content: Vec<SurveyPageContent>,
}

impl Default for SurveyPage {
    /// Create page with default values
    fn default() -> SurveyPage {
        SurveyPage {
            title: None,
            description: None,
            image_path: None,
            content: Vec::new(),
        }
    }
}

impl SurveyPage {
    pub fn new(title: Option<String>, description: Option<String>, image_path: Option<String>) -> SurveyPage {
        SurveyPage {
            title,
            description,
            image_path,
            content: Vec::new(),
        }
    }

    pub fn add_content(&mut self, content: SurveyPageContent) {
        self.content.push(content);
    }

    pub fn remove_content(&mut self, index: usize) {
        self.content.remove(index);
    }
}

pub struct SurveyPageContentHeader {
    pub content_type: SurveyContentType,
    pub title: String,
    pub required: bool,
}

pub enum SurveyContentType {
    Text,
    Choice,
    Data,
    Rating,
    Likert,
    Information,
    DateTime,
    Slider,
}

pub enum SurveyPageContent {
    /// Free-text question.
    TextQuestion {
        header: SurveyPageContentHeader,

        multiline: bool,
        pattern: Option<String>,
        score: Option<usize>,
        correct_answer: Option<String>,
        correct_answer_pattern: Option<String>,
        correct_answer_list: Option<Vec<String>>,
    },

    /// Choice-based question.
    ChoiceQuestion {
        header: SurveyPageContentHeader,

        multiple: bool,
        limit: usize,
        dropdown: bool,
        horizontal: bool,
        choices: Vec<ChoiceItem>,
    },

    /// Question for capturing participant’s details
    DataQuestion {
        header: SurveyPageContentHeader,

        data_type: DataQuestionType,
        validation_pattern: Option<String>,
        use_for_leaderboard: bool,
    },

    /// DateTime question.
    DateTimeQuestion {
        header: SurveyPageContentHeader,

        input_type: DateTimeType,
        initial_selected_time: Option<String>,
        initial_selected_date: Option<String>,
        score: Option<usize>,
        correct_time_answer: Option<String>,
        correct_date_answer: Option<String>,
    },

    /// Numeric rating question.
    RatingQuestion {
        header: SurveyPageContentHeader,

        level: usize,
        symbol: RatingSymbol,
        color_gradient: RatingColorGradient,
    },

    /// Slider question.
    SliderQuestion {
        header: SurveyPageContentHeader,

        range: bool,
        start: f64,
        end: f64,
        steps: usize,
        show_decimals: bool,
        unit: Option<String>,
        score: Option<usize>,
        correct_answer: Option<f64>,
    },

    /// Likert scale question.
    LikertQuestion {
        header: SurveyPageContentHeader,

        choices: Vec<String>,
        statements: Vec<LikertStatement>,
    },
    InformationBlock {
        header: SurveyPageContentHeader,

        description: Option<String>,
        image_path: Option<String>,
    },
}

pub struct ChoiceItem {
    pub title: String,
    pub score: Option<usize>,
    pub correct: bool,
}

pub enum DataQuestionType {
    Name,
    Email,
    Phone,
    Custom,
    Nickname,
    Age,
    Birthday,
}

pub enum DateTimeType {
    Date,
    Time,
    DateTime,
}

pub enum RatingSymbol {
    Star,
    Heart,
    Like,
    Smile,
    Number,
}

pub enum RatingColorGradient {
    None,
    Red2Green,
}

pub struct LikertStatement {
    pub title: String,
    pub score: Option<usize>,
    pub correct_choice: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survey_config_new() {
        let config = SurveyConfig::new("Test Survey".to_string(), "Description".to_string(), None, None, None);
        assert_eq!(config.title, "Test Survey");
        assert_eq!(config.description, "Description");
        assert!(config.pages.is_empty());
    }

    #[test]
    fn test_survey_config_add_remove_page() {
        let mut config = SurveyConfig::new("Test".to_string(), "Desc".to_string(), None, None, None);
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
        let content = SurveyPageContent::InformationBlock {
            header: SurveyPageContentHeader {
                content_type: SurveyContentType::Information,
                title: "Info".to_string(),
                required: false,
            },
            description: None,
            image_path: None,
        };
        page.add_content(content);
        assert_eq!(page.content.len(), 1);
        page.remove_content(0);
        assert_eq!(page.content.len(), 0);
    }
}
