// 副作用のある関数はinput.rsとoutput.rs以下に置く。

use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::fs::File;
use std::io::Read;

use crate::models::google_form::GoogleForm;
// pub mod fetch_google_forms;

pub async fn main() {
    dbg!("input.rs");
    // fetch_google_forms::main(args.client_id, args.client_secret, args.form_id);
}

pub fn read_markdown_from_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    dbg!(&contents);
    Ok(contents)
}

pub(super) async fn fetch_google_form(
    access_token: &str,
    form_id: &str,
) -> Result<GoogleForm, Box<dyn std::error::Error>> {
    dbg!("fetch_google_form");
    let url = format!(
        "https://forms.googleapis.com/v1/forms/{}",
        form_id.to_string()
    );
    // ヘッダーを作成
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", access_token.to_string()).parse()?,
    );
    // クライアントを作成
    let client = reqwest::Client::new();
    // GETリクエストを送信
    let response = client.get(url).headers(headers).send().await?;
    dbg!(&response);

    let body = response.json::<GoogleForm>().await?;

    // レスポンスを確認
    dbg!(&body);

    Ok(body)
}
