//! # Survey Tool CLI application helper
//! Functions and models supporting the main.rs function workloads

use colored::Colorize;
use inquire::required;
use survey_tool_cli::*;

pub struct ConfigTOC {
    pub survey_title: String,
    pub survey_type: SurveyType,
    pub survey_desc: String,
    pub has_conditionals: bool,

    pub pages: Vec<ConfigTOCPage>,
}

pub struct ConfigTOCPage {
    pub page_title: String,
    pub page_desc: String,
    pub conditional: Option<String>,

    pub elements: Vec<ConfigTOCElement>,
}

pub struct ConfigTOCElement {
    pub element_type: String,
    pub element_title: String,
    pub element_required: bool,
    pub conditional: Option<String>,
    pub config: String,
}

pub fn build_toc(config: &SurveyConfig) -> ConfigTOC {
    ConfigTOC {
        survey_title: config.title.to_string(),
        survey_type: config.survey_type,
        survey_desc: truncate(config.description.as_str()),
        has_conditionals: has_conditionals(config),
        pages: config.pages.iter().map(|p| ConfigTOCPage {
            page_title: p.title.as_deref().unwrap_or("<not set>").to_string(),
            page_desc: truncate(p.description.as_deref().unwrap_or("<not set>")),
            conditional: collect_conditional_setting(&p.conditional),
            elements: Vec::from_iter(p.content.iter().map(|c| {
                let header = c.get_header();
                ConfigTOCElement {
                    element_type: c.type_string(),
                    element_title: header.title.to_string(),
                    element_required: header.required,
                    conditional: collect_conditional_setting(&header.conditional),
                    config: c.format_config(),
                }
            })),
        }).collect(),
    }
}

pub fn collect_conditional_setting(conditional: &Option<ConditionalSettings>) -> Option<String> {
    conditional.as_ref().map(|cs| format!("[{values}] for key '{key}'", values = cs.values.join(", "), key = cs.key))
}

const MAX_DESC_LENGTH: usize = 80;
pub fn truncate(text: &str) -> String {
    if text.chars().count() > MAX_DESC_LENGTH { format!("{}...", text.chars().take(MAX_DESC_LENGTH - 3).collect::<String>()) } else { text.to_string() }
}

pub fn has_conditionals(config: &SurveyConfig) -> bool {
    for page in config.pages.iter() {
        if page.conditional.is_some() {
            return true;
        }
        if page.content.iter().any(|c| c.get_header().conditional.is_some()) {
            return true;
        }
    }
    false
}

pub fn input_survey_page() -> Result<SurveyPage, inquire::InquireError> {
    //input page title
    let title = inquire::Text::new("Enter page title (skip with ESC):").prompt_skippable()?.filter(|t| !t.trim().is_empty());

    //input page description
    let desc = inquire::Text::new("Enter page description (skip with ESC):").prompt_skippable()?.filter(|t| !t.trim().is_empty());

    //input optional image
    let image = inquire::Text::new("Enter optional image path (skip with ESC):").prompt_skippable()?.filter(|t| !t.trim().is_empty());

    Ok(SurveyPage::new(title, desc, image, input_conditional_setting()?))
}

pub fn input_conditional_setting() -> Result<Option<ConditionalSettings>, inquire::InquireError> {
    let conditional = inquire::Confirm::new("Make conditional?").with_default(false).prompt()?;
    if !conditional {
        return Ok(None);
    }

    //HINT key and value candidates could be prefilled
    let key = inquire::Text::new("Enter conditional key:").with_validator(required!("Conditional key is required")).prompt()?;

    let mut values = Vec::new();
    loop {
        match inquire::Text::new("Enter conditional value (stop with ESC):").prompt_skippable()?.filter(|t| !t.trim().is_empty()) {
            None if values.is_empty() => println!(" {} at least one value needed!", "Error:".red()),
            None => break,
            Some(v) => values.push(v),
        }
    }

    let mut cond = ConditionalSettings::new(key);
    cond.values = values;

    Ok(Some(cond))
}

pub struct PageSelectOption {
    pub title: Option<String>,
    pub index: usize,
}

impl std::fmt::Display for PageSelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.title {
            Some(s) => write!(f, "{:2}. {}", self.index + 1, s),
            None => write!(f, "{:2}. <not set>", self.index + 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SurveyDetailFields {
    Title,
    Description,
    Type,
    Image,
    BackgroundImage,
    Score,
}

impl std::fmt::Display for SurveyDetailFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn query_score_settings() -> Result<ScoreSettings, inquire::InquireError> {
    match inquire::Confirm::new("Do you want to use the default score settings?").with_default(true).prompt()? {
        true => Ok(ScoreSettings::default()),
        false => {
            let show_question_scores = inquire::Confirm::new("Show scores on each question?").with_default(false).prompt()?;
            let show_leaderboard = inquire::Confirm::new("Show leaderboard/highscore?").with_default(true).prompt()?;

            let leaderboard = if show_leaderboard {
                let show_scores = inquire::Confirm::new("Show player scores on leaderboard?").with_default(true).prompt()?;
                let show_placeholder = inquire::Confirm::new("Show placeholder instead of empty rows?").with_default(true).prompt()?;
                let limit: u8 = inquire::CustomType::new("Number of participants shown on leaderboard?")
                    .with_default(10)
                    .with_error_message("Please type a valid number")
                    .prompt()?;
                let background_image = inquire::Text::new("Enter optional leaderboard background image path (skip with ESC):")
                    .prompt_skippable()?
                    .filter(|t| !t.trim().is_empty());

                LeaderboardSettings::new(show_scores, show_placeholder, limit.into(), background_image)
            } else {
                LeaderboardSettings::default()
            };

            Ok(ScoreSettings::new(show_question_scores, show_leaderboard, leaderboard))
        },
    }
}
