pub mod authentication;
pub mod googleform_to_html;
pub mod html_to_googleform;
pub mod input;
pub mod markdown_to_html;
pub mod output;

use crate::models::google_form::GoogleForm;
use clap::ValueEnum;
use env_logger;
use log::{debug, error, info, trace, warn};
use std::io::Write;
use std::path::Path;

// use tokio;

#[derive(Debug)]
pub struct MarksurveyArgs {
    /// input file path
    pub input: Option<String>,
    /// output file path
    pub output: Option<String>,
    // use google form
    pub google_form: bool,
    /// google form id
    pub client_id: Option<String>,
    /// google OAuth 2.0 client id
    pub client_secret: Option<String>,
    /// google OAuth 2.0 client secret
    pub form_id: Option<String>,
    pub log_level: LogLevel,
    // /// Suppressse output
    // pub quiet: bool,
    /// Run the command in dry-run mode
    pub dry_run: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

// #[tokio::main]
pub async fn main(marksurvey_args: MarksurveyArgs) -> Result<(), Box<dyn std::error::Error>> {
    // dbg!("module.rs");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {} {}:{}] {}",
                ts,
                record.level(),
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args(),
            )
        })
        .init();
    // let _ = input::main();
    // let _ = markdown_to_html::main();
    // let _ = googleform_to_html::main().await.unwrap();
    // let _ = html_to_googleform::main().await.unwrap();
    // let _ = google_form_to_html(
    //     marksurvey_args.client_id,
    //     marksurvey_args.client_secret,
    //     marksurvey_args.form_id,
    // )
    // .await
    // .unwrap();

    if let Some(ref input_path) = marksurvey_args.input {
        let path = Path::new(input_path);
        let path_extension = path.extension().and_then(|ext| ext.to_str());

        if path_extension == Some("md") && marksurvey_args.google_form {
            trace!(".md && googleform subcommand");
            markdown_to_googleform(&input_path).await;
        }

        // match path.extension().and_then(|ext| ext.to_str()) {
        //     Some("md") => {
        //         trace!("Processing a Markdown (.md) file: {}", input_path);
        //     }

        //     _ => {
        //         trace!("Unsupported file type or no extension: {}", input_path);
        //     }
        // }
        trace!("{}", input_path);
    }

    Ok(())
}

pub async fn markdown_to_googleform(markdown_file_path: &str) {
    trace!("markdown_to_googleform");
    match input::read_markdown_from_file(&markdown_file_path) {
        Ok(markdown_contents) => {
            trace!("ファイルを正常に読み込みました: {}", &markdown_contents);
            trace!("{}", &markdown_contents);
            let html_contents = markdown_to_html::parse(&markdown_contents);
            trace!("{}", &html_contents);
        }
        Err(e) => {
            debug!("ファイル読み込みエラー: {}", e);
        }
    }
}

pub async fn google_form_to_html(
    client_id: &str,
    client_secret: &str,
    form_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    dbg!("google_form_to_html");
    Ok(String::from("string"))
}

pub(super) async fn get_google_form(
    client_id: &str,
    client_secret: &str,
    form_id: &str,
) -> Result<GoogleForm, Box<dyn std::error::Error>> {
    dbg!("fetch_google_forms");

    let access_token = authentication::get_access_token(&client_id, &client_secret).await?;
    let google_form: Result<GoogleForm, Box<dyn std::error::Error>> =
        input::fetch_google_form(&access_token, &form_id).await;
    let google_form = match google_form {
        Ok(google_form) => google_form,
        Err(error) => {
            panic!("There was a problem parsing: {:?}", error)
        }
    };
    dbg!(&google_form);
    Ok(google_form)
}
