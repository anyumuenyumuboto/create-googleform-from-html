use clap::{Parser, Subcommand};
use std::path::Path;
use tokio;

mod models;
mod modules;

use crate::modules::LogLevel;
use crate::modules::MarksurveyArgs;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// input file path
    #[arg(short, long, value_parser = validate_file_extension)]
    pub input: Option<String>,
    /// output file path
    #[arg(short, long, value_parser = validate_file_extension)]
    pub output: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
    // /// google OAuth 2.0 client id
    // #[arg(long)]
    // pub client_id: Option<String>,
    // /// google OAuth 2.0 client secret
    // #[arg(long)]
    // pub client_secret: Option<String>,
    /// google form id
    #[arg(long)]
    pub form_id: Option<String>,
    /// LogLevel
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
    // /// Output detailed standard output
    // #[arg(short, long)]
    // pub verbose: bool,
    /// Run the command in dry-run mode
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

// #[derive(Debug, Clone, ValueEnum)]
// enum LogLevel {
//     Debug,
//     Info,
//     Warn,
//     Error,
//     Critical,
// }

#[derive(Debug, Subcommand)]
enum Commands {
    Googleform {
        /// The client ID for Google API
        #[arg(long, help = "Specify the client ID for the Google API.")]
        client_id: String,
        /// The client secret for Google API
        #[arg(long, help = "Specify the client secret for the Google API.")]
        client_secret: String,
        /// The form id of the Google Form
        #[arg(long, help = "Specify the form id of the Google Form.")]
        form_id: String,
    },
}

impl From<Args> for MarksurveyArgs {
    fn from(args: Args) -> Self {
        // googleformサブコマンドのオプションの値を取得するための処理
        let (client_id_value, client_secret_value, form_id_value) = match &args.command {
            Some(Commands::Googleform {
                client_id,
                client_secret,
                form_id,
            }) => (
                Some(client_id.clone()),
                Some(client_secret.clone()),
                Some(form_id.clone()),
            ),
            _ => (None, None, None),
        };

        MarksurveyArgs {
            input: args.input,
            output: args.output,
            client_id: client_id_value,
            client_secret: client_secret_value,
            form_id: form_id_value,
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
