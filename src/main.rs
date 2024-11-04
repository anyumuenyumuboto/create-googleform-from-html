use pulldown_cmark;
use std::fs::file; 

fn main() {
    println!("Hello, world!");
     
    let filename = "README.md";
    let mut f = File::open(filename)
        // 
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file")
    tamesi();
}

fn tamesi(markdown_input: &str) {
    // Create parser with example Markdown text.
    // let markdown_input = "hello world";
    let parser = pulldown_cmark::Parser::new(markdown_input);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    // assert_eq!(&html_output, "<p>hello world</p>\n");
    dbg!(&html_output);
}
