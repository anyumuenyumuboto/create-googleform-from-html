pub mod authentication;
pub mod googleform_to_html;
pub mod html_to_googleform;
pub mod input;
pub mod markdown_to_html;
pub mod output;

use crate::models::google_form::BatchUpdate;
use crate::models::google_form::GoogleForm;
use crate::models::google_form::Item;
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

    if let Some(ref input_path) = marksurvey_args.input {
        let path = Path::new(input_path);
        trace!("{}", input_path);
        let path_extension = path.extension().and_then(|ext| ext.to_str());

        if path_extension == Some("md") && marksurvey_args.google_form {
            trace!(".md && googleform subcommand");
            if let (Some(ref client_id), Some(ref client_secret), Some(ref form_id)) = (
                marksurvey_args.client_id,
                marksurvey_args.client_secret,
                marksurvey_args.form_id,
            ) {
                markdown_to_googleform(&input_path, &client_id, &client_secret, &form_id).await;
            }
        }
    }

    Ok(())
}

pub async fn markdown_to_googleform(
    markdown_file_path: &str,
    client_id: &str,
    client_secret: &str,
    form_id: &str,
) {
    trace!("markdown_to_googleform");
    match input::read_markdown_from_file(&markdown_file_path) {
        Ok(markdown_contents) => {
            trace!("ファイルを正常に読み込みました: {}", &markdown_contents);
            trace!("{}", &markdown_contents);
            let html_contents: String = markdown_to_html::parse(&markdown_contents);
            trace!("{}", &html_contents);
            let html_choice_question =
                html_to_googleform::html_to_html_choice_question(&html_contents);
            trace!("{:#?}", &html_choice_question);
            let googleform_item: Item =
                html_to_googleform::choice_question_to_googleform_item(html_choice_question);
            trace!("{:#?}", &googleform_item);
            dbg!(&googleform_item);
            // let googleform_choicequestion: GoogleForm =
            //     html_to_googleform::googleform_item_to_googleform_choicequestion(googleform_item);
            // trace!("{:#?}", &googleform_choicequestion);
            let batchupdate: BatchUpdate =
                html_to_googleform::googleform_item_to_batchupdate(googleform_item);
            trace!("{:#?}", &batchupdate);
            match authentication::get_access_token(
                &client_id,
                &client_secret,
                "https://www.googleapis.com/auth/forms.body",
            )
            .await
            {
                Ok(access_token) => {
                    dbg!(&access_token);
                    match output::create_google_forms::update_google_form(
                        &access_token,
                        &form_id,
                        batchupdate,
                    )
                    .await
                    {
                        Ok(update_response) => {
                            dbg!(&update_response);
                        }
                        Err(e) => {
                            debug!("Failed to update googleform: {}", e);
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to retrieve the token: {}", e);
                }
            };
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

    let access_token = authentication::get_access_token(
        &client_id,
        &client_secret,
        "https://www.googleapis.com/auth/forms.body.readonly",
    )
    .await?;
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
