use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::io::{self, Write};

use crate::models::google_form::GoogleForm;

// pub(super) async fn main() {
//     dbg!("fetch_google_forms");
// }

pub(super) async fn get_access_token(
    client_id_in: &str,
    client_secret_in: &str,
    redirect_uri: &str,
    scope: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Google Cloud から取得したクライアントIDとシークレット
    let client_id = ClientId::new(client_id_in.to_string());
    let client_secret = ClientSecret::new(client_secret_in.to_string());

    // GoogleのOAuth 2.0エンドポイント
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string())?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

    // クライアントの設定
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string())?);
    // .set_redirect_uri(RedirectUrl::new("urn:ietf:wg:oauth:2.0:oob".to_string())?);

    // 認可URLを生成
    let (auth_url, _csrf_token) = client
        .authorize_url(|| oauth2::CsrfToken::new_random())
        .add_scope(Scope::new(
            // "https://www.googleapis.com/auth/cloud-platform".to_string(),
            scope.to_string(),
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

pub(super) async fn create_google_form(
    access_token: &str,
    form_id: &str,
    params: GoogleForm,
) -> Result<GoogleForm, Box<dyn std::error::Error>> {
    dbg!("fetch_google_form");
    let url = "https://forms.googleapis.com/v1/forms";
    // ヘッダーを作成
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", access_token.to_string()).parse()?,
    );
    // クライアントを作成
    let client = reqwest::Client::new();
    // POSTリクエストを送信
    let response = client
        .post(url)
        .headers(headers)
        .json(&params)
        .send()
        .await?;
    dbg!(&response);

    // let text = response.text().await?;

    // dbg!(&text);
    // Ok(GoogleForm::default())

    let body = response.json::<GoogleForm>().await?;

    // レスポンスを確認
    dbg!(&body);

    Ok(body)
}

pub(super) async fn update_google_form(
    access_token: &str,
    form_id: &str,
    params: GoogleForm,
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
    // POSTリクエストを送信
    let response = client
        .post(url)
        .headers(headers)
        .json(&params)
        .send()
        .await?;
    dbg!(&response);

    let body = response.json::<GoogleForm>().await?;

    // レスポンスを確認
    dbg!(&body);

    Ok(body)
}
