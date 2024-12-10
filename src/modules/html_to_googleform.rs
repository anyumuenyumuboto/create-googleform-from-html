use scraper::{Html, Selector};
use serde_json::json;

pub fn main() {
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
