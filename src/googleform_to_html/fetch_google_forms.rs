use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::io::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GoogleForm {
    #[serde(alias = "formId")]
    pub form_id: String,
    pub info: Info,
    pub settings: FormSettings,
    pub items: Vec<Item>,
    #[serde(alias = "revisionId")]
    pub revision_id: String,
    #[serde(alias = "responderUri")]
    pub responder_uri: String,
    #[serde(alias = "linkedSheetId")]
    pub linked_sheet_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Info {
    pub title: String,
    #[serde(alias = "documentTitle")]
    pub document_title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FormSettings {
    #[serde(alias = "quizSettings")]
    pub quiz_settings: Option<QuizSettings>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QuizSettings {
    #[serde(alias = "isQuiz")]
    pub is_quiz: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Item {
    #[serde(alias = "itemId")]
    pub item_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(alias = "questionItem")]
    pub question_item: Option<QuestionItem>,
    #[serde(alias = "questionGroupItem")]
    pub question_group_item: Option<QuestionGroupItem>,
    #[serde(alias = "pageBreakItem")]
    pub page_break_item: Option<PageBreakItem>,
    #[serde(alias = "textItem")]
    pub text_item: Option<TextItem>,
    #[serde(alias = "imageItem")]
    pub image_item: Option<ImageItem>,
    #[serde(alias = "videoItem")]
    pub video_item: Option<VideoItem>,
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "type", rename_all = "camelCase")]
// pub enum ItemKind {
//     QuestionItem, // {
//                   //     question_item: QuestionItem,
//                   // }
//     QuestionGroupItem {
//         question_group_item: QuestionGroupItem,
//     },
//     PageBreakItem {
//         page_break_item: PageBreakItem,
//     },
//     TextItem {
//         text_item: TextItem,
//     },
//     ImageItem {
//         image_item: ImageItem,
//     },
//     VideoItem {
//         video_item: VideoItem,
//     },
// }
//
// impl Default for ItemKind {
//     fn default() -> Self {
//         ItemKind::TextItem {
//             text_item: TextItem {
//                 // content: String::new(),
//             },
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QuestionItem {
    pub question: Option<Question>,
    pub image: Option<Image>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Question {
    #[serde(alias = "questionId")]
    pub question_id: Option<String>,
    pub required: Option<bool>,
    pub grading: Option<Grading>,
    #[serde(alias = "choiceQuestion")]
    pub choice_question: Option<ChoiceQuestion>,
    #[serde(alias = "textQuestion")]
    pub text_question: Option<TextQuestion>,
    #[serde(alias = "scaleQuestion")]
    pub scale_question: Option<ScaleQuestion>,
    #[serde(alias = "dateQuestion")]
    pub date_question: Option<DateQuestion>,
    #[serde(alias = "timeQuestion")]
    pub time_question: Option<TimeQuestion>,
    #[serde(alias = "fileUploadQuestion")]
    pub file_upload_question: Option<FileUploadQuestion>,
    #[serde(alias = "rowQuestion")]
    pub row_question: Option<RowQuestion>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChoiceQuestion {
    pub r#type: Option<ChoiceType>,
    // APIのレスポンスのJSONではOptionだが、Optionはrustですでに型の名前として使われているので、ChoiceOptionに変更。
    pub options: Vec<ChoiceOption>,
    pub shuffle: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChoiceType {
    CHOICE_TYPE_UNSPECIFIED,
    RADIO,
    CHECKBOX,
    DROP_DOWN,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChoiceOption {
    pub value: Option<String>,
    pub image: Option<Image>,
    #[serde(alias = "isOther")]
    pub is_other: Option<bool>,
    #[serde(alias = "goToAction")]
    pub go_to_action: Option<GoToAction>,
    #[serde(alias = "goToSectionId")]
    pub go_to_section_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GoToAction {
    GO_TO_ACTION_UNSPECIFIED,
    NEXT_SECTION,
    RESTART_FORM,
    SUBMIT_FORM,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TextQuestion {
    pub paragraph: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScaleQuestion {
    pub low: isize,
    pub high: isize,
    #[serde(alias = "lowLabel")]
    pub low_label: Option<String>,
    #[serde(alias = "highLabel")]
    pub high_label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DateQuestion {
    #[serde(alias = "includeTime")]
    pub include_time: Option<bool>,
    #[serde(alias = "includeYear")]
    pub include_year: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TimeQuestion {
    pub duration: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FileUploadQuestion {
    #[serde(alias = "folderId")]
    pub folder_id: Option<String>,
    pub types: Vec<FileType>,
    #[serde(alias = "maxFiles")]
    pub max_files: Option<isize>,
    #[serde(alias = "maxFileSize")]
    pub max_file_size: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileType {
    FILE_TYPE_UNSPECIFIED,
    ANY,
    DOCUMENT,
    PRESENTATION,
    SPREADSHEET,
    DRAWING,
    PDF,
    IMAGE,
    VIDEO,
    AUDIO,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct RowQuestion {
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Grading {
    #[serde(alias = "pointValue")]
    pub point_value: isize,
    #[serde(alias = "correctAnswers")]
    pub correct_answers: Option<CorrectAnswers>,
    #[serde(alias = "whenRight")]
    pub when_right: Option<Feedback>,
    #[serde(alias = "whenWrong")]
    pub when_wrong: Option<Feedback>,
    #[serde(alias = "generalFeedback")]
    pub general_feedback: Option<Feedback>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Feedback {
    pub text: String,
    pub material: Option<ExtraMaterial>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ExtraMaterial {
    pub link: Option<TextLink>,
    pub video: Option<VideoLink>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TextLink {
    pub uri: String,
    #[serde(alias = "displayText")]
    pub display_text: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VideoLink {
    #[serde(alias = "displayText")]
    pub display_text: String,
    #[serde(alias = "youtubeUri")]
    pub youtube_uri: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CorrectAnswers {
    pub answers: Option<CorrectAnswer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CorrectAnswer {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct QuestionGroupItem {
    pub questions: Vec<Question>,
    pub image: Option<Image>,
    pub grid: Option<Grid>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Grid {
    pub columns: ChoiceQuestion,
    #[serde(alias = "shuffleQuestions")]
    pub shuffle_questions: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Image {
    #[serde(alias = "contentUri")]
    pub content_uri: Option<String>,
    #[serde(alias = "altText")]
    pub alt_text: Option<String>,
    pub properties: Option<MediaProperties>,
    #[serde(alias = "sourceUri")]
    pub source_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MediaProperties {
    pub alignment: Option<Alignment>,
    pub width: Option<isize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Alignment {
    ALIGNMENT_UNSPECIFIED,
    LEFT,
    RIGHT,
    CENTER,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PageBreakItem {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TextItem {}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ImageItem {
    pub image: Image,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VideoItem {
    pub video: Video,
    pub caption: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Video {
    #[serde(alias = "youtubeUri")]
    pub youtube_uri: String,
    pub properties: Option<MediaProperties>,
}

// pub(super) async fn main() {
//     dbg!("fetch_google_forms");
// }

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
    dbg!(&response);

    // レスポンスを確認
    // let body = match response.status().is_success() {
    //     true => response.text().await?,
    //     false => {
    //         dbg!("Request failed with status: {}", response.status());
    //         return Err("Request failed".into());
    //     }
    // };

    // dbg!(&body);

    // let body = response.json::<GoogleForm>().await?;
    // let body = response.text().await?;
    let body = response
        .json::<GoogleForm>()
        .await
        .expect("json_parse_error");
    dbg!(&body);

    // let google_form: GoogleForm = serde_json::from_str(&body)?;
    // dbg!(&google_form);

    // let body2 = GoogleForm {
    //     form_id: "a".to_string(),
    // };
    Ok(body)
}

pub(super) async fn fetch_google_form_text(
    access_token: &str,
    form_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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

    let body = response.text().await?;
    dbg!(&body);

    Ok(())
}
