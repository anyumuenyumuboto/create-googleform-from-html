use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::io::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleForm {
    pub form_id: String,
    pub info: Info,
    pub settings: FormSettings,
    pub items: Vec<Item>,
    pub revision_id: String,
    pub responder_uri: String,
    pub linked_sheet_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    // Infoの具体的なフィールドをここに追加
    // 例: pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormSettings {
    // FormSettingsの具体的なフィールドをここに追加
    // 例: pub is_public: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    // Itemの具体的なフィールドをここに追加
    // 例: pub question: String,
}

pub(super) async fn main() {
    dbg!("fetch_google_forms");
}

pub(super) async fn get_access_token(
    client_id_in: &str,
    client_secret_in: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Google Cloud から取得したクライアントIDとシークレット
    let client_id = ClientId::new(client_id_in.to_string());
    let client_secret = ClientSecret::new(client_secret_in.to_string());

    // GoogleのOAuth 2.0エンドポイント
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    // クライアントの設定
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new("urn:ietf:wg:oauth:2.0:oob".to_string())?);

    // 認可URLを生成
    let (auth_url, _csrf_token) = client
        .authorize_url(|| oauth2::CsrfToken::new_random())
        .add_scope(Scope::new(
            // "https://www.googleapis.com/auth/cloud-platform".to_string(),
            "https://www.googleapis.com/auth/forms.body.readonly".to_string(),
        ))
        .url();

    // ユーザーに認可URLを表示
    println!(
        "以下のURLをブラウザで開いて認可コードを取得してください:\n\n{}",
        auth_url
    );

    // ユーザーから認可コードを入力
    print!("認可コードを入力してください: ");
    io::stdout().flush()?;
    let mut auth_code = String::new();
    io::stdin().read_line(&mut auth_code)?;
    let auth_code = auth_code.trim().to_string();

    // トークンを取得
    let token = client
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(async_http_client)
        .await?;

    let access_token = token.access_token().secret().to_string();
    println!("アクセストークン: {:?}", token.access_token().secret());
    Ok(access_token)
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

    // レスポンスを確認
    let body = match response.status().is_success() {
        true => response.text().await?,
        false => {
            dbg!("Request failed with status: {}", response.status());
            return Err("Request failed".into());
        }
    };

    // if response.status().is_success() {
    //     body = response.text().await?;
    //     println!("Response body: {}", &body);
    // } else {
    //     println!("Request failed with status: {}", response.status());
    // }

    let google_form: GoogleForm = serde_json::from_str(&body)?;
    dbg!({}, &google_form);

    Ok(google_form)
}
