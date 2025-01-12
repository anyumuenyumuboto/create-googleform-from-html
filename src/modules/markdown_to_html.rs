// use std::fs::File;
// use std::io::Read;
// use std::io::Write;

// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     dbg!("markdown_to_html");
//     let markdown_file_path = "README.md";
//     // let markdown_string: String = read_markdown_from_file(&markdown_file_path)?;
//     // let html_string = parse(&markdown_string)?;
//     // let html_file_path = "README.html";
//     // write_html_to_file(&html_file_path, &html_string)?;
//     Ok(())
// }

// fn read_markdown_from_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let mut f = File::open(file_path)?;
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;
//     dbg!(&contents);
//     Ok(contents)
// }

pub fn parse(markdown_input: &str) -> String
// Result<String, Box<dyn std::error::Error>>
{
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = pulldown_cmark::Parser::new_ext(markdown_input, options);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    // dbg!(&html_output);
    html_output
    // Ok(html_output)
}

// fn write_html_to_file(
//     file_path: &str,
//     html_content: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // ファイルを作成または開く
//     let mut file = File::create(file_path)?;
//
//     // HTML文字列を書き込む
//     file.write_all(html_content.as_bytes())?;
//
//     println!("HTMLファイルが作成されました: {}", file_path);
//     Ok(())
// }
