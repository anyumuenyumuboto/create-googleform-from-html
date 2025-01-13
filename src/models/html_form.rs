use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HtmlForm {
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Question {
    ChoiceQuestion,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
}
