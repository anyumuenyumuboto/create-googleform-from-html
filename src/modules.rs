pub mod googleform_to_html;
pub mod html_to_googleform;
pub mod input;
pub mod markdown_to_html;
pub mod output;

#[tokio::main]
pub async fn main(args: Args) {
    dbg!("module.rs");
    let _ = input::main(&args);
    let _ = markdown_to_html::main();
    // let _ = googleform_to_html::main().await.unwrap();
    let _ = html_to_googleform::main().await.unwrap();
}
