//! # Survey Tool CLI application
//!
//! Supports in handling [**Survey Tool**](https://github.com/HuppiFluppi/survey-tool) configuration files
//!
//! Current commands:
//! - **check**: Check an existing survey tool configuration yaml for correctness
//! - **setup-check**: Check local host for prerequisits to run survey tool
//! - *edit (not yet): Allow to edit(add/remove/change) a survey tool config from command line*
//!
//! This binary crate uses the lib crate in the same repository.
//!
//! Run `survey-tool-cli help` to show the help page

use colored::Colorize;
use std::{error::Error, process::exit};

use clap::{Parser, Subcommand};
use survey_tool_cli::{CheckResult, setup_check, config_check};

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

    // #[command(about = "Edit an existing survey tool configuration file")]
    // Edit {
    //     /// The file to edit
    //     file: String,

    //     /// Creates the file if it doesn't exist. Overwrites existing files
    //     #[arg(long)]
    //     create: bool,

    //     #[command(subcommand)]
    //     subcommand: EditSubcommand
    // },
    #[command(about = "Check capability of local system to run survey tool")]
    SetupCheck,
}

#[derive(Subcommand)]
enum EditSubcommand {
    /// List the survey contents
    #[command(visible_alias = "ls")]
    List,

    /// Edit the survey details
    EditSurveyDetails,

    /// Add a new survey page
    AddPage,

    /// Edit an existing survey page
    EditPage,

    /// Remove a survey page
    RemovePage,
}

// Functions
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Check { file } => display_check_result(config_check(file), cli.verbose),
        Commands::SetupCheck => display_check_result(setup_check(), cli.verbose),
    }
}

fn display_check_result(result: Result<CheckResult, Box<dyn Error>>, verbose: bool) {
    match result {
        Err(err) => {
            println!("{} {}", "Error:".red(), err);
            exit(1)
        }
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
        }
    }
}
