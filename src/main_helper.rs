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
        pages: Vec::from_iter(config.pages.iter().map(|p| ConfigTOCPage {
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
        })),
    }
}

pub fn collect_conditional_setting(conditional: &Option<ConditionalSettings>) -> Option<String> {
    conditional.as_ref().map(|cs| format!("[{values}] for key '{key}'", values = cs.values.join(", "), key = cs.key))
}

const MAX_DESC_LENGTH: usize = 80;
pub fn truncate(text: &str) -> String {
    if text.len() > MAX_DESC_LENGTH { format!("{}...", text.chars().take(MAX_DESC_LENGTH - 3).collect::<String>()) } else { text.to_string() }
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
