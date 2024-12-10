pub mod googleform_to_html;
pub mod html_to_googleform;
pub mod markdown_to_html;

#[tokio::main]
pub async fn main() {
    dbg!("module.rs");
    let _ = markdown_to_html::main();
    let _ = googleform_to_html::main().await.unwrap();
    html_to_googleform::main();
}
