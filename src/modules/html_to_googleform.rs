use log::trace;
use scraper::{Html, Selector};

use crate::models::html_form::ChoiceQuestion;
use crate::models::html_form::HtmlForm;
use crate::models::html_form::Question;

use crate::models::google_form::BatchUpdate;
use crate::models::google_form::Item;

pub mod htmlform_to_googleform;

pub fn main(html_string: &str) -> BatchUpdate {
    trace!("{}", &html_string);
    let html_choice_question = html_to_html_choice_question(&html_string);
    trace!("{:#?}", &html_choice_question);
    let googleform_item: Item =
        htmlform_to_googleform::choice_question_to_googleform_item(html_choice_question);
    trace!("{:#?}", &googleform_item);
    dbg!(&googleform_item);
    // let googleform_choicequestion: GoogleForm =
    //     html_to_googleform::googleform_item_to_googleform_choicequestion(googleform_item);
    // trace!("{:#?}", &googleform_choicequestion);
    let batchupdate: BatchUpdate =
        htmlform_to_googleform::googleform_item_to_batchupdate(googleform_item);
    trace!("{:#?}", &batchupdate);
    batchupdate
}

pub fn html_to_html_choice_question(html_string: &str) -> ChoiceQuestion {
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

    let choice_question = ChoiceQuestion {
        options: html_selected,
        ..ChoiceQuestion::default()
    };
    choice_question
}

pub fn html_to_html_form(html_string: &str) -> HtmlForm {
    dbg!(&html_string);

    // HTMLをパース
    let document = Html::parse_document(html_string);

    // checkboxの選択肢のリストを選択するセレクタ
    let checkbox_list_selector = Selector::parse(r#"li:has(>input[type="checkbox"])"#).unwrap();

    // checkboxの選択肢の文字列のベクトルを取得
    let checkbox_list = document
        .select(&checkbox_list_selector)
        .map(|x| {
            x.text()
                .collect::<Vec<_>>()
                .concat()
                .trim_start()
                .to_string()
        })
        .collect::<Vec<_>>();

    dbg!(&checkbox_list);

    // h1~h6の見出し要素のセレクタを定義
    let heading_selector = Selector::parse("h1,h2,h3,h4,h5,h6").unwrap();
    // チェックボックスのセレクタを定義
    let checkbox_selector = Selector::parse("input[type='checkbox']").unwrap();

    // 最初の見出し要素を見つける
    let heading_element = document.select(&heading_selector).next();
    let mut description: Option<String> = None;
    if let Some(heading) = heading_element {
        // 最初の<input type="checkbox">を見つける
        if let Some(checkbox) = document.select(&checkbox_selector).next() {
            let mut text_between = String::new();
            let mut found_heading = false;

            // 見出し要素からチェックボックスの間までの探索
            for node in document.tree.nodes() {
                if node.id() == heading.id() {
                    found_heading = true;
                } else if node.id() == checkbox.id() {
                    break;
                } else if found_heading {
                    // テキストノードを取得
                    if let Some(text) = node.value().as_text() {
                        text_between.push_str(text);
                        text_between.push(' ');
                    }
                }
            }

            description = Some(text_between.trim().to_string());
            // return text_between.trim().to_string();
        }
    }

    let choice_question = ChoiceQuestion {
        description: description,
        options: checkbox_list,
        ..ChoiceQuestion::default()
    };

    let html_form = HtmlForm {
        questions: vec![Question::ChoiceQuestion(choice_question)],
        ..HtmlForm::default()
    };

    dbg!(&html_form);
    html_form
}

fn f() -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_return_1() {
        assert_eq!(f(), 1);
    }

    #[test]
    fn test_html_to_html_form() {
        let html = r#"
        <h1>Title</h1>
        Some text here
        <p>Paragraph text</p>
        <input type="checkbox">
    "#;

        let html_form = html_to_html_form(html);
        dbg!(html_form);
    }
}

// pub fn create_googleform_choicequestion() -> GoogleForm {
//     // let googleform_default = GoogleForm::default();
//     // let item_default = Item::default();
//     // let question_default = Question::default();
//     // let choice_question_default = ChoiceQuestion::default();
//     let choice_option_tamesi = ChoiceOption {
//         : String::from("apple"),
//         ..ChoiceOption::default()
//     };
//     let choice_question_tamesi = ChoiceQuestion {
//         r#type: Some(ChoiceType::CHECKBOX),
//         options: vec![choice_option_tamesi],
//         ..ChoiceQuestion::default()
//     };
//     let question_tamesi = Question {
//         choice_question: Some(choice_question_tamesi),
//         ..Question::default()
//     };
//     let item_tamesi = Item {
//         title: Some(String::from("質問1")),
//         description: Some(String::from("これは仮の質問です")),
//         question_item: Some(QuestionItem {
//             question: Some(question_tamesi),
//             image: None,
//         }),
//         ..Item::default()
//     };
//     let googleform_choicequestion = GoogleForm {
//         info: Info {
//             title: String::from("試し"),
//             document_title: String::from("試し"),
//             description: None,
//         },
//         items: Some(vec![item_tamesi]),
//         ..GoogleForm::default()
//     };
//     // dbg!(&googleform_choicequestion);
//     // dbg!(&googleform_choicequestion.info.title);
//     googleform_choicequestion
// }

// pub fn sample() {
//     // サンプルHTML
//     let html = r#"
//         <html>
//             <body>
//                 <input type="checkbox" id="chk1" name="option1" checked>
//                 <input type="checkbox" id="chk2" name="option2">
//                 <input type="checkbox" id="chk3" name="option3" checked>
//             </body>
//         </html>
//     "#;
//
//     // HTMLをパース
//     let document = Html::parse_document(html);
//
//     // checkboxを選択するセレクタ
//     let checkbox_selector = Selector::parse(r#"input[type="checkbox"]"#).unwrap();
//
//     // チェックボックス情報を収集
//     let mut checkboxes = Vec::new();
//
//     for checkbox in document.select(&checkbox_selector) {
//         // 属性を取得
//         let id = checkbox.value().attr("id").unwrap_or("").to_string();
//         let name = checkbox.value().attr("name").unwrap_or("").to_string();
//         let checked = checkbox.value().attr("checked").is_some();
//
//         // JSON形式のオブジェクトに変換
//         checkboxes.push(json!({
//             "id": id,
//             "name": name,
//             "checked": checked,
//         }));
//     }
//
//     // JSONを生成
//     let json_output = json!({ "checkboxes": checkboxes });
//
//     // JSONを出力
//     println!("{}", serde_json::to_string_pretty(&json_output).unwrap());
// }
