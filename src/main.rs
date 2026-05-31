//! # Survey Tool CLI application
//!
//! Supports in handling [**Survey Tool**](https://github.com/HuppiFluppi/survey-tool) configuration files
//!
//! Current commands:
//! - **check**: Check an existing survey tool configuration yaml for correctness
//! - **setup-check**: Check local host for prerequisites to run survey tool
//! - **config**: Allow to handle survey tool config from command line*
//!
//! This binary crate uses the lib crate in the same repository.
//!
//! Run `survey-tool-cli help` to show the help page

use colored::Colorize;
use inquire::required;
use std::process::exit;

use clap::{Parser, Subcommand};
use survey_tool_cli::*;

mod main_helper;

// Cli model with clap configuration
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Increase output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Check a survey tool configuration file (yaml) for correctness")]
    Check {
        /// The file to check
        file: String,
    },

    #[command(about = "Interactively edit a survey tool configuration file")]
    Config {
        /// The file to edit
        file: String,

        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },

    #[command(about = "Check capability of local system to run survey tool")]
    SetupCheck,
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// List the contents of a survey config file
    #[command(visible_alias = "ls")]
    List {
        /// path traversing the config structure. e.g. '1/1' for first page, first element'
        path: Option<String>,

        /// List elements(questions)
        #[arg(short, long)]
        show_elements: bool,

        /// Don't display the top section with information about the survey
        #[arg(short, long)]
        no_header: bool,
    },

    /// Initialize a config file based on user input
    #[command(visible_alias = "in")]
    Init {
        /// Overwrite and truncate an existing file
        #[arg(long)]
        overwrite: bool,
    },

    /// Edit the survey details
    #[command(visible_alias = "sd")]
    SurveyDetails,

    // /// Add a new survey page
    // #[command(visible_alias = "np")]
    // AddPage,

    // /// Edit an existing survey page
    // #[command(visible_alias = "ep")]
    // EditPage,
    /// Remove a survey page
    #[command(visible_alias = "rm")]
    RemovePage,
}

// Functions
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { file } => display_check_result(config_check(file), cli.verbose),
        Commands::SetupCheck => display_check_result(setup_check(), cli.verbose),
        Commands::Config { file, subcommand } => config_cmd(file, subcommand),
    }
}

fn display_check_result(result: Result<CheckResult, STCError>, verbose: bool) {
    match result {
        Err(err) => {
            println!(" ❌ {} {}", "Error:".red(), err);
            exit(1)
        },
        Ok(result) => {
            if result.all_ok {
                println!("{}", "### All OK ###".green().bold());
                if verbose {
                    result.success_list.iter().for_each(|x| println!(" ✅ {x}"));
                }
            } else {
                println!("{}", format!("### {} errors ###", result.error_list.len()).yellow().bold());
                if verbose {
                    println!("{}", "Successful:".green().underline().bold());
                    result.success_list.iter().for_each(|x| println!(" ✅ {x}"));

                    println!("{}", "Failed:".red().underline().bold());
                }
                result.error_list.iter().for_each(|x| println!(" ❌ {x}"));
            }
        },
    }
}

fn config_cmd(file: &str, subcommand: &ConfigSubcommand) {
    match subcommand {
        ConfigSubcommand::List { path, show_elements, no_header } => list_cmd(file, path.as_ref(), *show_elements, *no_header),
        ConfigSubcommand::Init { overwrite } => init_cmd(file, *overwrite),
        ConfigSubcommand::SurveyDetails => todo!(),
        // ConfigSubcommand::AddPage { num } => todo!(),
        // ConfigSubcommand::EditPage { page } => todo!(),
        ConfigSubcommand::RemovePage => remove_cmd(file),
    }
}

fn remove_cmd(file: &str) {
    //get file
    let mut config = match load_config(file) {
        Ok(c) => c,
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //check page count
    if config.pages.len() < 2 {
        println!(" {} Can't remove page from single page survey", "Error: ".red());
        return;
    }

    //select page
    let options = config.pages.iter().enumerate().map(|(i, page)| main_helper::PageOption { title: page.title.to_owned(), index: i }).collect();
    let page = match inquire::Select::new("Select page to delete", options).prompt() {
        Ok(p) => p,
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //confirm remove
    match inquire::Confirm::new(&format!("Confirm removal of page {}", page.index + 1)).with_default(false).prompt() {
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
        Ok(false) => {
            println!(" {}", "Cancelled".red());
            return;
        },
        Ok(_) => (),
    };

    //remove page
    config.remove_page(page.index);

    //save
    match save_config(file, true, &config) {
        Ok(_) => println!(" 👍 successfully removed page"),
        Err(e) => println!(" ❌ {} {}", "Error:".red(), e),
    }
}

fn init_cmd(file: &str, overwrite: bool) {
    println!();
    println!("📝 {}", "Enter survey information".bold());
    println!();

    //select survey type
    let survey_type = match inquire::Select::new("Which type do you want?", vec![SurveyType::Survey, SurveyType::Quiz]).prompt() {
        Ok(v) => v,
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //input survey/quiz title
    let title = match inquire::Text::new(&format!("Enter {survey_type} title:")).with_validator(required!("Title is required")).prompt() {
        Ok(v) => v,
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //input survey/quiz description
    let desc = match inquire::Text::new(&format!("Enter {survey_type} description:")).with_validator(required!("Description is required")).prompt() {
        Ok(v) => v,
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //input optional image
    let image = match inquire::Text::new("Enter optional image path (skip with ESC):").prompt_skippable() {
        Ok(str) => str.filter(|t| !t.trim().is_empty()),
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //input optional background image
    let background_image = match inquire::Text::new("Enter optional background image path (skip with ESC):").prompt_skippable() {
        Ok(str) => str.filter(|t| !t.trim().is_empty()),
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //get score info on quizes
    let score: Option<ScoreSettings>;
    if matches!(survey_type, SurveyType::Quiz) {
        match inquire::Confirm::new("Do you want to use the default score settings?").with_default(true).prompt() {
            Ok(true) => score = Some(ScoreSettings::default()),
            Ok(false) => {
                let show_question_scores = match inquire::Confirm::new("Show scores on each question?").with_default(false).prompt() {
                    Ok(v) => v,
                    Err(e) => {
                        println!(" {} {}", "Error: ".red(), e);
                        return;
                    },
                };
                let show_leaderboard = match inquire::Confirm::new("Show leaderboard/highscore?").with_default(true).prompt() {
                    Ok(v) => v,
                    Err(e) => {
                        println!(" {} {}", "Error: ".red(), e);
                        return;
                    },
                };

                let leaderboard = if show_leaderboard {
                    let show_scores = match inquire::Confirm::new("Show player scores on leaderboard?").with_default(true).prompt() {
                        Ok(v) => v,
                        Err(e) => {
                            println!(" {} {}", "Error: ".red(), e);
                            return;
                        },
                    };
                    let show_placeholder = match inquire::Confirm::new("Show placeholder instead of empty rows?").with_default(true).prompt() {
                        Ok(v) => v,
                        Err(e) => {
                            println!(" {} {}", "Error: ".red(), e);
                            return;
                        },
                    };
                    let limit: u8 = match inquire::CustomType::new("Number of participants shown on leaderboard?")
                        .with_error_message("Please type a valid number")
                        .prompt()
                    {
                        Ok(v) => v,
                        Err(e) => {
                            println!(" {} {}", "Error: ".red(), e);
                            return;
                        },
                    };
                    let background_image = match inquire::Text::new("Enter optional leaderboard background image path (skip with ESC):").prompt_skippable() {
                        Ok(v) => v.filter(|t| !t.trim().is_empty()),
                        Err(e) => {
                            println!(" {} {}", "Error: ".red(), e);
                            return;
                        },
                    };

                    LeaderboardSettings::new(show_scores, show_placeholder, limit.into(), background_image)
                } else {
                    LeaderboardSettings::default()
                };

                score = Some(ScoreSettings::new(show_question_scores, show_leaderboard, leaderboard))
            },
            Err(e) => {
                println!(" {} {}", "Error: ".red(), e);
                return;
            },
        }
    } else {
        score = None;
    }

    //input first page
    println!();
    println!("  {}", "Enter first page information".bold());
    println!();

    let page = match main_helper::input_survey_page() {
        Ok(v) => v,
        Err(e) => {
            println!(" {} {}", "Error: ".red(), e);
            return;
        },
    };

    //create and write config
    let mut config = SurveyConfig::new(title, desc, Some(survey_type), image, background_image, score);
    config.add_page(page);

    match save_config(file, overwrite, &config) {
        Ok(_) => println!(" 👍 successfully init file. You can now add content(questions) and more pages"),
        Err(e) => println!(" ❌ {} {}", "Error:".red(), e),
    }
}

fn list_cmd(file: &str, path: Option<&String>, show_elements: bool, no_header: bool) {
    //get config
    let config = match load_config(file) {
        Ok(c) => c,
        Err(e) => {
            println!(" ❌ {} {}", "Error:".red(), e);
            return;
        },
    };

    //build table of contents
    let toc = main_helper::build_toc(&config);

    //output based on cmd arguments
    if let Some(path) = path {
        //extract and check path elements
        let split_path: Vec<&str> = path.split_terminator(['/', '\\']).collect();
        if split_path.len() > 2 {
            println!(" {} path argument in wrong format. Should be one or two numbers, specifying the page and element, separated by '/'", "Error:".red(),);
            return;
        }
        let Ok(page_select) = split_path[0].parse::<usize>() else {
            println!(" {} path argument in wrong format. '{}' not a number", "Error:".red(), split_path[0]);
            return;
        };
        if page_select > toc.pages.len() {
            println!(" {} only {} pages in survey, but requested #{}", "Error:".red(), toc.pages.len(), page_select);
            return;
        }
        let page_select = page_select - 1; //adjust to 0 based index
        let element_select = if split_path.len() > 1 {
            let Ok(val) = split_path[1].parse::<usize>() else {
                println!(" {} path argument in wrong format. '{}' not a number", "Error:".red(), split_path[1]);
                return;
            };
            if val > toc.pages[page_select].elements.len() {
                println!(
                    " {} only {} elements in page {}, but requested element {}",
                    "Error:".red(),
                    toc.pages[page_select].elements.len(),
                    page_select + 1,
                    val
                );
                return;
            }
            Some(val - 1) //adjust to 0 based index
        } else {
            None
        };

        //display selection
        if let Some(element_select) = element_select {
            let element = &toc.pages[page_select].elements[element_select];
            println!();
            println!("〰 {} {}", "Element title:".dimmed(), element.element_title);
            println!("   {}  {}", "Element type:".dimmed(), element.element_type);
            println!(
                "   {req}      {reqv}  {space}  {cond} {condv}",
                req = "Required:".dimmed(),
                reqv = element.element_required,
                space = " ".repeat(20),
                cond = "Conditional:".dimmed(),
                condv = element.conditional.as_deref().unwrap_or("No")
            );
            println!();
            println!("   Config:");
            for line in element.config.lines() {
                println!("       {}", line);
            }
            println!()
        } else {
            let page = &toc.pages[page_select];
            println!();
            if !no_header {
                println!("📄 {} {}", "Page title:".dimmed(), page.page_title);
                println!("   {}  {}", "Page desc:".dimmed(), page.page_desc);
                println!("   {} {}", "Conditional:".dimmed(), page.conditional.as_deref().unwrap_or("No"));
                println!();
                println!("{}", "─".repeat(100));
            }
            println!("  ###  Type          Flags  Title");
            println!("  ---  ----          -----  -----");
            for (i, element) in page.elements.iter().enumerate() {
                let mut flags = String::with_capacity(5);
                if element.conditional.is_some() {
                    flags.push('c');
                }
                if element.element_required {
                    flags.push('r');
                }
                println!("  {:>2}.  {:12}  {:5}  {}", i + 1, element.element_type, flags, element.element_title);
            }
            println!()
        }
    } else {
        println!();
        if !no_header {
            println!("📘 {} {}", "Survey title:".dimmed(), toc.survey_title);
            println!("   {}  {}", "Survey desc:".dimmed(), toc.survey_desc);
            println!("   {type}  {typev:8} {space} {cond} {condv}", type = "Survey type:".dimmed(), typev = toc.survey_type, space = " ".repeat(40), cond = "Conditionals:".dimmed(), condv = toc.has_conditionals);
            println!();
            println!("{}", "─".repeat(100));
        }
        println!("  Pages({}):", toc.pages.len());
        for (i, page) in toc.pages.iter().enumerate() {
            println!("  {:>2}. {}", i + 1, page.page_title);
            if show_elements {
                for (k, element) in page.elements.iter().enumerate() {
                    println!("      └── {:>2}. {}", k + 1, element.element_title)
                }
            }
        }
        println!()
    }
}
