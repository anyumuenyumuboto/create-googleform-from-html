pub mod authentication;
pub mod googleform_to_html;
pub mod html_to_googleform;
pub mod input;
pub mod markdown_to_html;
pub mod output;

use crate::models::google_form::GoogleForm;
// use tokio;

#[derive(Debug)]
pub struct MarksurveyArgs {
    pub input_type: Option<String>,
    /// input file path
    pub input: Option<String>,
    pub output_type: Option<String>,
    /// output file path
    pub output: Option<String>,
    /// google form id
    pub client_id: Option<String>,
    /// google OAuth 2.0 client id
    pub client_secret: Option<String>,
    /// google OAuth 2.0 client secret
    pub form_id: Option<String>,
    pub log_level: LogLevel,
    /// Suppressse output
    pub quiet: bool,
    /// Run the command in dry-run mode
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

// #[tokio::main]
pub async fn main(marksurvey_args: MarksurveyArgs) -> Result<(), Box<dyn std::error::Error>> {
    dbg!("module.rs");
    // let _ = input::main();
    // let _ = markdown_to_html::main();
    // let _ = googleform_to_html::main().await.unwrap();
    // let _ = html_to_googleform::main().await.unwrap();
    let _ = google_form_to_html(
        marksurvey_args.client_id,
        marksurvey_args.client_secret,
        marksurvey_args.form_id,
    )
    .await
    .unwrap();
    Ok(())
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
