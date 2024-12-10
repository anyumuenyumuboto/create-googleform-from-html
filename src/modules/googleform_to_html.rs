use dotenv::dotenv;
use std::env;

use crate::models::google_form::GoogleForm;
pub mod fetch_google_forms;

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("googleform_to_html");
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let form_id = env::var("FORM_ID").expect("FORM_ID must be set");

    let access_token = fetch_google_forms::get_access_token(&client_id, &client_secret).await?;
    fetch_google_forms::fetch_google_form_text(&access_token, &form_id).await?;
    let google_form: Result<GoogleForm, Box<dyn std::error::Error>> =
        fetch_google_forms::fetch_google_form(&access_token, &form_id).await;
    let google_form = match google_form {
        Ok(google_form) => google_form,
        Err(error) => {
            panic!("There was a problem parsing: {:?}", error)
        }
    };
    // dbg!(&google_form);
    Ok(())
}
