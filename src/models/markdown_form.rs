use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MarkdownForm {
    title: String,
    question: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Question {
    ChoiceQuestion,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChoiceQuestion {
    title: String,
    description: Option<String>,
    choice_option: Vec<String>,
}
