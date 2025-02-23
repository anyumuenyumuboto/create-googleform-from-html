use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GoogleForm {
    #[serde(alias = "formId")]
    pub form_id: String,
    pub info: Info,
    pub settings: Option<FormSettings>,
    // pub items: Vec<Item>,
    pub items: Option<Vec<Item>>,
    #[serde(alias = "revisionId")]
    pub revision_id: String,
    #[serde(alias = "responderUri")]
    pub responder_uri: Option<String>,
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

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub enum ChoiceType {
    CHOICE_TYPE_UNSPECIFIED,
    RADIO,
    CHECKBOX,
    DROP_DOWN,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChoiceOption {
    // pub value: Option<String>,
    pub value: String,
    pub image: Option<Image>,
    #[serde(alias = "isOther")]
    pub is_other: Option<bool>,
    #[serde(alias = "goToAction")]
    pub go_to_action: Option<GoToAction>,
    #[serde(alias = "goToSectionId")]
    pub go_to_section_id: Option<String>,
}

#[allow(non_camel_case_types)]
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

#[allow(non_camel_case_types)]
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

#[allow(non_camel_case_types)]
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

// batchupdate
// ref [Method: forms.batchUpdate  |  Google Forms  |  Google for Developers](https://developers.google.com/forms/api/reference/rest/v1/forms/batchUpdate?hl=ja)
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BatchUpdate {
    pub requests: Vec<Request>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Request {
    #[serde(alias = "createItem")]
    pub create_item: CreateItemRequest,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CreateItemRequest {
    pub item: Item,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Location {
    pub index: isize,
}
