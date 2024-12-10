use scraper::{Html, Selector};
use serde_json::json;
use std::fs;
use std::path::Path;

use crate::models::google_form::ChoiceOption;
use crate::models::google_form::ChoiceQuestion;
use crate::models::google_form::ChoiceType;
use crate::models::google_form::GoogleForm;
use crate::models::google_form::Info;
use crate::models::google_form::Item;
use crate::models::google_form::Question;
use crate::models::google_form::QuestionItem;

use crate::models::markdown_form::ChoiceQuestion as MarkdownChoiceQuestion;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // sample();
    create_googleform_choicequestion();
    let html = read_html_file(Path::new("README.html"))?;
    dbg!(html_to_choice_question(&html));
    Ok(())
}

pub fn read_html_file(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    dbg!(&content);
    Ok(content)
}

pub fn html_to_choice_question(html_string: &str) -> MarkdownChoiceQuestion {
    dbg!(&html_string);

    // HTMLをパース
    let document = Html::parse_document(html_string);

    // checkboxの選択肢のリストを選択するセレクタ
    let checkbox_selector = Selector::parse(r#"li:has(>input[type="checkbox"])"#).unwrap();

    //  let html_selected = document
    //      .select(&checkbox_selector)
    //      .nth(0)
    //      .expect("REASON")
    //      .text()
    //      // .value();
    //      .collect::<Vec<_>>()
    //      .concat();

    let html_selected = document
        .select(&checkbox_selector)
        .map(|x| {
            x.text()
                .collect::<Vec<_>>()
                .concat()
                .trim_start()
                .to_string()
        })
        .collect::<Vec<_>>();

    dbg!(&html_selected);

    let choice_question = MarkdownChoiceQuestion {
        options: html_selected,
        ..MarkdownChoiceQuestion::default()
    };
    choice_question
}

pub fn create_googleform_choicequestion() -> GoogleForm {
    // let googleform_default = GoogleForm::default();
    // let item_default = Item::default();
    // let question_default = Question::default();
    // let choice_question_default = ChoiceQuestion::default();
    let choice_option_tamesi = ChoiceOption {
        value: String::from("apple"),
        ..ChoiceOption::default()
    };
    let choice_question_tamesi = ChoiceQuestion {
        r#type: Some(ChoiceType::CHECKBOX),
        options: vec![choice_option_tamesi],
        ..ChoiceQuestion::default()
    };
    let question_tamesi = Question {
        choice_question: Some(choice_question_tamesi),
        ..Question::default()
    };
    let item_tamesi = Item {
        title: Some(String::from("質問1")),
        description: Some(String::from("これは仮の質問です")),
        question_item: Some(QuestionItem {
            question: Some(question_tamesi),
            image: None,
        }),
        ..Item::default()
    };
    let googleform_choicequestion = GoogleForm {
        info: Info {
            title: String::from("試し"),
            document_title: String::from("試し"),
            description: None,
        },
        items: vec![item_tamesi],
        ..GoogleForm::default()
    };
    dbg!(&googleform_choicequestion);
    dbg!(&googleform_choicequestion.info.title);
    googleform_choicequestion
}

pub fn sample() {
    // サンプルHTML
    let html = r#"
        <html>
            <body>
                <input type="checkbox" id="chk1" name="option1" checked>
                <input type="checkbox" id="chk2" name="option2">
                <input type="checkbox" id="chk3" name="option3" checked>
            </body>
        </html>
    "#;

    // HTMLをパース
    let document = Html::parse_document(html);

    // checkboxを選択するセレクタ
    let checkbox_selector = Selector::parse(r#"input[type="checkbox"]"#).unwrap();

    // チェックボックス情報を収集
    let mut checkboxes = Vec::new();

    for checkbox in document.select(&checkbox_selector) {
        // 属性を取得
        let id = checkbox.value().attr("id").unwrap_or("").to_string();
        let name = checkbox.value().attr("name").unwrap_or("").to_string();
        let checked = checkbox.value().attr("checked").is_some();

        // JSON形式のオブジェクトに変換
        checkboxes.push(json!({
            "id": id,
            "name": name,
            "checked": checked,
        }));
    }

    // JSONを生成
    let json_output = json!({ "checkboxes": checkboxes });

    // JSONを出力
    println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
}
