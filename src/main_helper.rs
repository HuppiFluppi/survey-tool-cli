//! # Survey Tool CLI application helper
//! Functions and models supporting the main.rs function workloads

use std::{
    fmt,
    str::{self, FromStr},
};

use colored::Colorize;
use inquire::required;
use strum::VariantNames;
use survey_tool_cli::*;

// ##############################
// ### TOC types and function ###
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
        survey_title: config.title.clone(),
        survey_type: config.survey_type,
        survey_desc: truncate(config.description.as_str()),
        has_conditionals: has_conditionals(config),
        pages: config
            .pages
            .iter()
            .map(|p| ConfigTOCPage {
                page_title: p.title.as_deref().unwrap_or("<not set>").to_string(),
                page_desc: truncate(p.description.as_deref().unwrap_or("<not set>")),
                conditional: collect_conditional_setting(p.conditional.as_ref()),
                elements: Vec::from_iter(p.content.iter().map(|c| {
                    let header = c.get_header();
                    ConfigTOCElement {
                        element_type: c.to_string(),
                        element_title: header.title.clone(),
                        element_required: header.required,
                        conditional: collect_conditional_setting(header.conditional.as_ref()),
                        config: c.format_config(),
                    }
                })),
            })
            .collect(),
    }
}

pub fn collect_conditional_setting(conditional: Option<&ConditionalSettings>) -> Option<String> {
    conditional.as_ref().map(|cs| format!("[{values}] for key '{key}'", values = cs.values.join(", "), key = cs.key))
}

pub fn has_conditionals(config: &SurveyConfig) -> bool {
    for page in &config.pages {
        if page.conditional.is_some() {
            return true;
        }
        if page.content.iter().any(|c| c.get_header().conditional.is_some()) {
            return true;
        }
    }
    false
}

// ###########################
// ### Input(form) helpers ###

pub const NUMBER_ERROR: &str = "Enter valid number";

const MAX_DESC_LENGTH: usize = 80;
pub fn truncate(text: &str) -> String {
    if text.chars().count() > MAX_DESC_LENGTH { format!("{}...", text.chars().take(MAX_DESC_LENGTH - 3).collect::<String>()) } else { text.to_string() }
}

pub fn remove_content(page: &mut SurveyPage) -> Result<(), inquire::InquireError> {
    //check content amount
    if page.content.len() < 2 {
        println!(" ❌ {} Last content on page. Remove not possible", "Error:".red());
        return Ok(());
    }

    //select content position
    let content_options = page
        .content
        .iter()
        .enumerate()
        .map(|(i, c)| SurveyContentEditActions::ContentEdit { index: i, title: c.get_header().title.clone(), content_type: c.to_string() })
        .collect();
    let SurveyContentEditActions::ContentEdit { index, .. } = inquire::Select::new("Select content to remove:", content_options).prompt()? else {
        unreachable!()
    };

    //confirm removal
    if inquire::Confirm::new("Confirm removal").with_default(false).prompt()? {
        page.remove_content(index);
        println!(" 🗑️ removed content");
    }

    Ok(())
}

pub fn move_content(page: &mut SurveyPage) -> Result<(), inquire::InquireError> {
    //check content amount
    if page.content.len() < 2 {
        println!(" ❌ {} Page with single content. No move possible", "Error:".red());
        return Ok(());
    }

    //select content to move
    let content_options = page
        .content
        .iter()
        .enumerate()
        .map(|(i, c)| SurveyContentEditActions::ContentEdit { index: i, title: c.get_header().title.clone(), content_type: c.to_string() })
        .collect();
    let SurveyContentEditActions::ContentEdit { index: old_index, .. } = inquire::Select::new("Select content to move:", content_options).prompt()? else {
        unreachable!()
    };
    let content = page.remove_content(old_index);

    //select destination
    let mut destination_options: Vec<_> = page
        .content
        .iter()
        .enumerate()
        .map(|(i, c)| SurveyContentEditActions::ContentEdit { index: i, title: c.get_header().title.clone(), content_type: c.to_string() })
        .collect();
    destination_options.push(SurveyContentEditActions::ContentEdit {
        index: destination_options.len(),
        title: "---".to_string(),
        content_type: "<Last>".to_string(),
    });

    let SurveyContentEditActions::ContentEdit { index: new_index, .. } =
        inquire::Select::new("Select where to move", destination_options).with_help_message("Content will move before selected").prompt()?
    else {
        unreachable!()
    };

    if old_index == new_index {
        println!(" {}", "Selected same slot/index. No change".yellow());
        return Ok(());
    }

    //add content back
    page.insert_content(new_index, content);
    println!(" ⇄ moved content");

    Ok(())
}

pub fn new_content(page: &mut SurveyPage) -> Result<(), inquire::InquireError> {
    //select content position
    let index = if page.content.is_empty() {
        0
    } else {
        let mut content_options: Vec<_> = page
            .content
            .iter()
            .enumerate()
            .map(|(i, c)| SurveyContentEditActions::ContentEdit { index: i, title: c.get_header().title.clone(), content_type: c.to_string() })
            .collect();
        content_options.push(SurveyContentEditActions::ContentEdit {
            index: content_options.len(),
            title: "---".to_string(),
            content_type: "<Last>".to_string(),
        });

        let SurveyContentEditActions::ContentEdit { index, .. } =
            inquire::Select::new("Select where to add:", content_options).with_help_message("Content will be added before selected").prompt()?
        else {
            unreachable!()
        };

        index
    };

    //select new content type
    let content_type = inquire::Select::new("Select new content type:", SurveyPageContent::VARIANTS.to_vec()).prompt()?;

    //create content stub
    let content = match SurveyPageContent::from_str(content_type) {
        Ok(c) => c,
        Err(e) => {
            println!(" ❌ {} {}", "Error:".red(), e);
            return Ok(());
        },
    };
    page.insert_content(index, content);

    //edit new content
    edit_content(page.content.get_mut(index).unwrap())?;

    println!(" ➕ added new content\n");

    Ok(())
}

pub fn edit_content(content: &mut SurveyPageContent) -> Result<(), inquire::InquireError> {
    //--edit header
    //edit title
    prompt_required_textfield("Edit content title:", &mut content.get_header_mut().title)?;

    //edit required
    content.get_header_mut().required = inquire::Confirm::new("Required content?").with_default(content.get_header().required).prompt()?;

    //edit conditional
    edit_conditional_setting(&mut content.get_header_mut().conditional)?;

    //--edit config
    println!();
    match content {
        SurveyPageContent::Text { config, .. } => {
            //multiline
            prompt_required_bool("Set multiline?", &mut config.multiline)?;
            //pattern
            prompt_optional_textfield("Set pattern:", &mut config.pattern)?;
            //score
            prompt_optional_custom("Enter score:", &mut config.score, NUMBER_ERROR)?;
            //correct_answer
            prompt_optional_textfield("Set correct answer:", &mut config.correct_answer)?;
            //correct_answer_pattern
            prompt_optional_textfield("Set answer pattern:", &mut config.correct_answer_pattern)?;
            //correct_answer_list
            prompt_optional_text_list("Set answer list:", &mut config.correct_answer_list)?;
        },
        SurveyPageContent::Choice { config, .. } => {
            //multiple
            prompt_required_bool("Allow multiple selections?", &mut config.multiple)?;
            //limit
            prompt_required_custom("Limit for multiple selections:", &mut config.limit, "Enter valid number")?;
            //dropdown
            prompt_required_bool("Use dropdown presentation?", &mut config.dropdown)?;
            //horizontal
            prompt_required_bool("Use horizontal presentation?", &mut config.horizontal)?;
            //conditional key
            prompt_optional_textfield("Set optional conditional key:", &mut config.conditional_key)?;
            //choices
            prompt_choice_list(&mut config.choices)?;
        },
        SurveyPageContent::Data { config, .. } => {
            //datatype
            prompt_enum("Choose data type:", &mut config.datatype)?;
            //validation pattern
            prompt_optional_textfield("Validation pattern:", &mut config.validation_pattern)?;
            //use for leaderboard
            prompt_required_bool("Use for leaderboard?", &mut config.use_for_leaderboard)?;
        },
        SurveyPageContent::DateTime { config, .. } => {
            //input type
            prompt_enum("Set type:", &mut config.input_type)?;
            //initial time
            prompt_optional_time("Edit initial time:", &mut config.initial_selected_time)?;
            //initial date
            prompt_optional_date("Edit initial date:", &mut config.initial_selected_date)?;
            //score
            prompt_optional_custom("Enter optional score:", &mut config.score, NUMBER_ERROR)?;
            //correct time
            prompt_optional_time("Edit correct time answer:", &mut config.correct_time_answer)?;
            //correct date
            prompt_optional_date("Edit correct date answer:", &mut config.correct_date_answer)?;
        },
        SurveyPageContent::Rating { config, .. } => {
            //level
            prompt_required_custom("Set rating levels:", &mut config.level, NUMBER_ERROR)?;
            //symbol
            prompt_enum("Select Symbol type:", &mut config.symbol)?;
            //gradient
            prompt_enum("Select gradient type:", &mut config.color_gradient)?;
        },
        SurveyPageContent::Slider { config, .. } => {
            //range
            prompt_required_bool("Range select?", &mut config.range)?;
            //start
            prompt_required_custom("Set start value:", &mut config.start, NUMBER_ERROR)?;
            //end
            prompt_required_custom("Set end value:", &mut config.end, NUMBER_ERROR)?;
            //steps
            prompt_required_custom("Set steps:", &mut config.steps, NUMBER_ERROR)?;
            //decimal presentation
            prompt_required_bool("Show decimals?", &mut config.show_decimals)?;
            //unit
            prompt_optional_textfield("Optional unit:", &mut config.unit)?;
            //score
            prompt_optional_custom("Optional score:", &mut config.score, NUMBER_ERROR)?;
            //correct answer
            prompt_optional_custom("Correct answer:", &mut config.correct_answer, NUMBER_ERROR)?;
        },
        SurveyPageContent::Likert { config, .. } => {
            //choices
            prompt_text_list("Enter likert choices:", &mut config.choices, false)?;
            println!();
            //statements
            prompt_statement_list(&mut config.statements)?;
        },
        SurveyPageContent::Information { description, image, .. } => {
            //desc
            prompt_optional_textfield("Enter optional description:", description)?;
            //image path
            prompt_optional_textfield("Enter optional image path:", image)?;
        },
    }

    Ok(())
}

pub fn edit_page_details(page: &mut SurveyPage) -> Result<(), inquire::InquireError> {
    //edit title
    prompt_optional_textfield("Edit page title:", &mut page.title)?;

    //edit desc
    prompt_optional_textfield("Edit page description:", &mut page.description)?;

    //edit image
    prompt_optional_textfield("Edit page image:", &mut page.image)?;

    //edit conditional
    edit_conditional_setting(&mut page.conditional)?;

    Ok(())
}

pub fn edit_conditional_setting(setting: &mut Option<ConditionalSettings>) -> Result<(), inquire::InquireError> {
    if !inquire::Confirm::new("Set conditional?").with_default(setting.is_some()).prompt()? {
        *setting = None;
        return Ok(());
    }

    //HINT key and value candidates could be prefilled from known values in survey

    //input key
    let mut prompt = inquire::Text::new("Enter conditional key:").with_validator(required!("Conditional key is required"));
    prompt.initial_value = setting.as_ref().map(|s| s.key.as_str());
    let key = prompt.prompt()?;

    //input values
    let mut values = setting.as_ref().unwrap().values.clone();
    prompt_text_list("Enter conditional values:", &mut values, false)?;

    //create new settings
    *setting = Some(ConditionalSettings::new(key, values));

    Ok(())
}

pub fn prompt_text_list(msg: &str, field: &mut Vec<String>, empty_allowed: bool) -> Result<(), inquire::InquireError> {
    println!("{msg}");

    let mut values = Vec::new();

    //existing values
    for value in field.iter() {
        match inquire::Text::new("Check existing value (remove with ESC):").with_initial_value(value).prompt_skippable()?.filter(|t| !t.trim().is_empty()) {
            None => continue,
            Some(v) => values.push(v),
        }
    }

    field.clear();
    field.append(&mut values);

    //new values
    loop {
        match inquire::Text::new("Enter new value (stop with ESC):").prompt_skippable()?.filter(|t| !t.trim().is_empty()) {
            None if field.is_empty() && !empty_allowed => println!(" {} at least one value needed!", "Error:".red()),
            None => break,
            Some(v) => field.push(v),
        }
    }

    Ok(())
}

pub fn prompt_choice_list(field: &mut Vec<ChoiceItem>) -> Result<(), inquire::InquireError> {
    //existing values
    if !field.is_empty() {
        let mut values = Vec::new();

        loop {
            match inquire::Select::new("Edit existing choice items:", field.clone()).with_help_message("Continue to new items with ESC").prompt_skippable()? {
                None => break,
                Some(mut c) => {
                    //title
                    prompt_required_textfield("  Edit choice title:", &mut c.title)?;
                    //score
                    prompt_optional_custom("  Enter optional score:", &mut c.score, NUMBER_ERROR)?;
                    //correct
                    prompt_required_bool("  Correct answer?", &mut c.correct)?;

                    values.push(c);
                },
            }
        }

        field.clear();
        field.append(&mut values);
    }

    //new values
    loop {
        if inquire::Confirm::new("Enter new choice?").prompt()? {
            let mut c = ChoiceItem::default();

            //title
            prompt_required_textfield("  Edit choice title:", &mut c.title)?;
            //score
            prompt_optional_custom("  Enter optional score:", &mut c.score, NUMBER_ERROR)?;
            //correct
            prompt_required_bool("  Correct answer?", &mut c.correct)?;

            field.push(c);
        } else if field.len() > 1 {
            break;
        } else {
            println!(" {} at least two choices needed!", "Error:".red());
        }
    }

    Ok(())
}

pub fn prompt_statement_list(field: &mut Vec<LikertStatement>) -> Result<(), inquire::InquireError> {
    //existing values
    if !field.is_empty() {
        let mut values = Vec::new();

        loop {
            match inquire::Select::new("Edit existing likert statements:", field.clone())
                .with_help_message("Continue to new items with ESC")
                .prompt_skippable()?
            {
                None => break,
                Some(mut c) => {
                    //title
                    prompt_required_textfield("  Edit statement title:", &mut c.title)?;
                    //score
                    prompt_optional_custom("  Enter optional score:", &mut c.score, NUMBER_ERROR)?;
                    //correct
                    prompt_optional_textfield("  Correct answer:", &mut c.correct_choice)?;

                    values.push(c);
                },
            }
        }

        field.clear();
        field.append(&mut values);
    }

    //new values
    loop {
        if inquire::Confirm::new("Enter new statement?").prompt()? {
            let mut c = LikertStatement::default();

            //title
            prompt_required_textfield("  Edit statement text:", &mut c.title)?;
            //score
            prompt_optional_custom("  Enter optional score:", &mut c.score, NUMBER_ERROR)?;
            //correct
            prompt_optional_textfield("  Correct answer:", &mut c.correct_choice)?;

            field.push(c);
        } else if field.len() > 1 {
            break;
        } else {
            println!(" {} at least two statements needed!", "Error:".red());
        }
    }

    Ok(())
}

pub fn prompt_optional_text_list(msg: &str, field: &mut Option<Vec<String>>) -> Result<(), inquire::InquireError> {
    let mut tmp = field.take().unwrap_or_default();

    prompt_text_list(msg, &mut tmp, true)?;

    *field = if tmp.is_empty() { None } else { Some(tmp) };

    Ok(())
}

pub fn prompt_optional_textfield(msg: &str, field: &mut Option<String>) -> Result<(), inquire::InquireError> {
    let mut prompt = inquire::Text::new(msg);
    prompt = match field {
        Some(v) => prompt.with_initial_value(v),
        None => prompt,
    };
    *field = prompt.with_help_message("Unset with empty or ESC").prompt_skippable()?.filter(|t| !t.trim().is_empty());

    Ok(())
}

pub fn prompt_required_textfield(msg: &str, field: &mut String) -> Result<(), inquire::InquireError> {
    *field = inquire::Text::new(msg).with_initial_value(field).with_validator(required!("Required")).prompt()?;
    Ok(())
}

pub fn prompt_required_bool(msg: &str, field: &mut bool) -> Result<(), inquire::InquireError> {
    *field = inquire::Confirm::new(msg).with_default(*field).prompt()?;
    Ok(())
}

pub fn prompt_required_custom<T>(msg: &str, field: &mut T, error: &str) -> Result<(), inquire::InquireError>
where
    T: Copy + str::FromStr + ToString,
{
    *field = inquire::CustomType::<T>::new(msg).with_default(*field).with_error_message(error).prompt()?;
    Ok(())
}

pub fn prompt_optional_custom<T>(msg: &str, field: &mut Option<T>, error: &str) -> Result<(), inquire::InquireError>
where
    T: Copy + str::FromStr + ToString,
{
    let mut prompt = inquire::CustomType::<T>::new(msg);
    prompt = match field {
        Some(v) => prompt.with_default(*v),
        None => prompt,
    };
    *field = prompt.with_error_message(error).with_help_message("Unset with ESC").prompt_skippable()?;

    Ok(())
}

pub fn prompt_enum<T>(msg: &str, field: &mut T) -> Result<(), inquire::InquireError>
where
    T: VariantNames + fmt::Display + str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    let default = T::VARIANTS.iter().position(|x| *x == field.to_string()).unwrap_or_default();
    *field = T::from_str(inquire::Select::new(msg, T::VARIANTS.to_vec()).with_starting_cursor(default).prompt()?).unwrap();

    Ok(())
}

#[derive(Clone)]
struct RegexValidator {
    regex: regex::Regex,
    human_readable_pattern: String,
}
impl inquire::validator::StringValidator for RegexValidator {
    fn validate(&self, input: &str) -> Result<inquire::validator::Validation, inquire::CustomUserError> {
        if self.regex.is_match(input) {
            Ok(inquire::validator::Validation::Valid)
        } else {
            Ok(inquire::validator::Validation::Invalid(inquire::validator::ErrorMessage::Custom(format!(
                "Input doesnt match pattern '{}'",
                self.human_readable_pattern
            ))))
        }
    }
}

pub fn prompt_optional_time(msg: &str, field: &mut Option<String>) -> Result<(), inquire::InquireError> {
    const TIME_REGEX: &str = r"^\d{1,2}:\d\d$"; //this allows invalid times, but good for a start
    const TIME_HUMAN_READABLE: &str = "hh:mm";

    let mut prompt = inquire::Text::new(msg);
    prompt = match field {
        Some(v) => prompt.with_initial_value(v),
        None => prompt,
    };
    *field = prompt
        .with_help_message("Unset with ESC")
        .with_placeholder(TIME_HUMAN_READABLE)
        .with_validator(RegexValidator { regex: regex::Regex::new(TIME_REGEX).unwrap(), human_readable_pattern: TIME_HUMAN_READABLE.to_string() })
        .prompt_skippable()?
        .filter(|t| !t.trim().is_empty());

    Ok(())
}

pub fn prompt_optional_date(msg: &str, field: &mut Option<String>) -> Result<(), inquire::InquireError> {
    const DATE_REGEX: &str = r"^\d\d\d\d-\d\d-\d\d$"; //this allows invalid dates, but good for a start
    const DATE_HUMAN_READABLE: &str = "yyyy-mm-dd";

    let mut prompt = inquire::Text::new(msg);
    prompt = match field {
        Some(v) => prompt.with_initial_value(v),
        None => prompt,
    };
    *field = prompt
        .with_help_message("Unset with ESC")
        .with_placeholder(DATE_HUMAN_READABLE)
        .with_validator(RegexValidator { regex: regex::Regex::new(DATE_REGEX).unwrap(), human_readable_pattern: DATE_HUMAN_READABLE.to_string() })
        .prompt_skippable()?
        .filter(|t| !t.trim().is_empty());

    Ok(())
}

pub fn input_score_settings() -> Result<ScoreSettings, inquire::InquireError> {
    if inquire::Confirm::new("Do you want to use the default score settings?").with_default(true).prompt()? {
        Ok(ScoreSettings::default())
    } else {
        let show_question_scores = inquire::Confirm::new("Show scores on each question?").with_default(false).prompt()?;
        let show_leaderboard = inquire::Confirm::new("Show leaderboard/highscore?").with_default(true).prompt()?;

        let leaderboard = if show_leaderboard {
            let show_scores = inquire::Confirm::new("Show player scores on leaderboard?").with_default(true).prompt()?;
            let show_placeholder = inquire::Confirm::new("Show placeholder instead of empty rows?").with_default(true).prompt()?;
            let limit: u8 =
                inquire::CustomType::new("Number of participants shown on leaderboard?").with_default(10).with_error_message(NUMBER_ERROR).prompt()?;
            let background_image =
                inquire::Text::new("Enter optional leaderboard background image path (skip with ESC):").prompt_skippable()?.filter(|t| !t.trim().is_empty());

            LeaderboardSettings::new(show_scores, show_placeholder, limit.into(), background_image)
        } else {
            LeaderboardSettings::default()
        };

        Ok(ScoreSettings::new(show_question_scores, show_leaderboard, leaderboard))
    }
}

// ###########################
// ### Page selection type ###

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

// ###################################
// ### Survey field selection type ###

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
        write!(f, "{self:?}")
    }
}

// ############################
// ### Error handling macro ###

#[macro_export]
macro_rules! match_error {
    ( $prompt:expr ) => {
        match $prompt {
            Ok(v) => v,
            Err(e) => {
                println!(" ❌ {} {}", "Error:".red(), e);
                return;
            },
        }
    };
    ( $prompt:expr, $ok:ident, $ok_transform:expr ) => {
        match $prompt {
            Ok($ok) => $ok_transform,
            Err(e) => {
                println!(" ❌ {} {}", "Error:".red(), e);
                return;
            },
        }
    };
}

// ##############################
// ### Content selection type ###

#[derive(Clone, PartialEq, Eq)]
pub enum SurveyContentEditActions {
    NewAction,
    RemoveAction,
    MoveAction,
    SaveAction,
    SaveNQuitAction,
    QuitNoSaveAction,
    ContentEdit { index: usize, title: String, content_type: String },
}

impl std::fmt::Display for SurveyContentEditActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SurveyContentEditActions::NewAction => write!(f, "[+  New]"),
            SurveyContentEditActions::RemoveAction => write!(f, "[-  Remove]"),
            SurveyContentEditActions::MoveAction => write!(f, "[<> Move]"),
            SurveyContentEditActions::SaveAction => write!(f, "[*  Save]"),
            SurveyContentEditActions::SaveNQuitAction => write!(f, "[!  Save & Quit]"),
            SurveyContentEditActions::QuitNoSaveAction => write!(f, "[x  Quit w/o Save]"),
            SurveyContentEditActions::ContentEdit { index, title, content_type } => write!(f, "{:2}. {content_type}: {}", index + 1, truncate(title)),
        }
    }
}
