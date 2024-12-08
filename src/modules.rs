pub mod googleform_to_html;
pub mod markdown_to_html;

#[tokio::main]
pub async fn main() {
    dbg!("module.rs");
    markdown_to_html::main();
    googleform_to_html::main();
}
