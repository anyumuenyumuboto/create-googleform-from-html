// 副作用のある関数はinput.rsとoutput.rs以下に置く。
use std::fs::File;
use std::io::Write;
pub mod create_google_forms;

pub fn write_html_to_file(
    file_path: &str,
    html_content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // ファイルを作成または開く
    let mut file = File::create(file_path)?;

    // HTML文字列を書き込む
    file.write_all(html_content.as_bytes())?;

    println!("HTMLファイルが作成されました: {}", file_path);
    Ok(())
}
