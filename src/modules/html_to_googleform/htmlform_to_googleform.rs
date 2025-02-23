use crate::models::google_form::ChoiceOption;
use crate::models::google_form::ChoiceQuestion;
use crate::models::google_form::ChoiceType;
use crate::models::google_form::GoogleForm;
use crate::models::google_form::Info;
use crate::models::google_form::Item;
use crate::models::google_form::Question;
use crate::models::google_form::QuestionItem;

use crate::models::google_form::BatchUpdate;
use crate::models::google_form::CreateItemRequest;
use crate::models::google_form::Location;
use crate::models::google_form::Request;

use crate::models::google_form::ChoiceType::CHECKBOX;

use crate::models::html_form::ChoiceQuestion as HtmlChoiceQuestion;

pub fn htmlform_to_googleform() {
    dbg!("htmlform_to_googleform");
}

pub fn choice_question_to_googleform_item(markdown_choice_question: HtmlChoiceQuestion) -> Item {
    let options = markdown_choice_question
        .options
        .iter()
        .map(|x| ChoiceOption {
            value: x.to_string(),
            ..ChoiceOption::default()
        })
        .collect::<Vec<_>>();

    let item = Item {
        title: Some(markdown_choice_question.title),
        description: markdown_choice_question.description,
        question_item: Some(QuestionItem {
            question: Some(Question {
                choice_question: Some(ChoiceQuestion {
                    r#type: Some(CHECKBOX),
                    options: options,
                    // vec![ChoiceOption::default()],
                    ..ChoiceQuestion::default()
                }),
                ..Question::default()
            }),
            ..QuestionItem::default()
        }),
        ..Item::default()
    };
    // dbg!(&item);
    item
}

pub fn googleform_item_to_googleform_choicequestion(googleform_item: Item) -> GoogleForm {
    let googleform_choicequestion = GoogleForm {
        info: Info {
            title: String::from("試し"),
            document_title: String::from("試し"),
            description: None,
        },
        items: Some(vec![googleform_item]),
        ..GoogleForm::default()
    };
    // dbg!(&googleform_choicequestion);
    // dbg!(&googleform_choicequestion.info.title);
    googleform_choicequestion
}

pub fn googleform_item_to_batchupdate(googleform_item: Item) -> BatchUpdate {
    let batchupdate = BatchUpdate {
        requests: vec![Request {
            create_item: CreateItemRequest {
                item: googleform_item,
                location: Location { index: 0 },
            },
        }],
    };
    // dbg!(&googleform_choicequestion);
    // dbg!(&googleform_choicequestion.info.title);
    batchupdate
}
