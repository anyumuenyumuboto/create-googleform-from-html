use html_builder::Buffer;
use html_builder::Html5;
use std::fmt::Write;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("googleform_to_html");
    let mut buf = Buffer::new(); // Contents added to buffer by each statement:
    let mut html = buf.html().attr("lang='en'"); // <html lang='en'>
    writeln!(html.head().title(), "Title!")?; // <head><title>Title!
    writeln!(html.body().h1(), "Header!")?; // </title></head><body><h1>Header!
    let html_string = buf.finish(); // </h1></body></html>
    dbg!("{}", html_string);
    Ok(())
}
