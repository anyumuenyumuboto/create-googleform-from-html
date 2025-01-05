use clap::{Parser, ValueEnum};
use std::path::Path;
use tokio;

mod models;
mod modules;

use crate::modules::MarksurveyArgs;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[arg(long)]
    pub input_type: Option<String>,
    /// input file path
    #[arg(short, long , value_parser = validate_file_extension)]
    pub input: Option<String>,
    #[arg(long)]
    pub output_type: Option<String>,
    /// output file path
    #[arg(short, long)]
    pub output: Option<String>,
    /// google form id
    #[arg(long)]
    pub client_id: Option<String>,
    /// google OAuth 2.0 client id
    #[arg(long)]
    pub client_secret: Option<String>,
    /// google OAuth 2.0 client secret
    #[arg(long)]
    pub form_id: Option<String>,
    /// LogLevel
    #[arg(long, Value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
    // /// Output detailed standard output
    // #[arg(short, long)]
    // pub verbose: bool,
    /// Run the command in dry-run mode
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

#[derive(Debyg, Clone, ValueEnum)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl From<Args> for MarksurveyArgs {
    fn from(args: Args) -> Self {
        MarksurveyArgs {
            input_type: args.input_type,
            input: args.input,
            output_type: args.output_type,
            output: args.output,
            client_id: args.client_id,
            client_secret: args.client_secret,
            form_id: args.form_id,
            // verbose: args.verbose,
            log_level: args.log_level,
            dry_run: args.dry_run,
        }
    }
}

fn validate_file_extension(file: &str) -> Result<String, String> {
    let path = Path::new(file);
    if let Some(ext) = path.extension() {
        if ext == "html" || ext == "md" {
            return Ok(file.to_string());
        }
    }
    Err(String::from(
        "Invalid file extension. Only .html and .md files are allowed.",
    ))
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    dbg!(&args);

    dbg!("main.rs");

    let marksurvey_args: MarksurveyArgs = args.into();
    modules::main(marksurvey_args).await.unwrap();
}
