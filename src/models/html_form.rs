use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HtmlForm {
    pub title: String,
    pub questions: Vec<Question>,
}

// Rustでは列挙型のバリアントにフィールドが含まれる場合、それはタプル型構造体のように扱われ、値をラップする際に適切な構文を使う必要があります。
#[derive(Serialize, Deserialize, Debug)]
pub enum Question {
    ChoiceQuestion(ChoiceQuestion),
    // ChoiceQuestion,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
}
