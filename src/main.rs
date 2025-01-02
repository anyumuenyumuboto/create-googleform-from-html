use clap::Parser;
use std::path::Path;

mod models;
mod modules;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// input file path
    #[arg(short, long , value_parser = validate_file_extension)]
    pub input: Option<String>,
    /// output file path
    #[arg(short, long)]
    pub output: Option<String>,
    /// google form id
    #[arg(long)]
    pub client_id: Option<String>,
    /// google OAuth 2.0 client id
    #[arg(long)]
    pub client_secret: Option<String>,
    /// google OAuth 2.0 client secret
    #[arg(long)]
    pub form_id: Option<String>,
    /// Suppressse output
    #[arg(short, long)]
    pub quiet: bool,
    /// Run the command in dry-run mode
    #[arg(long = "dry-run")]
    pub dry_run: bool,
}

fn validate_file_extension(file: &str) -> Result<String, String> {
    let path = Path::new(file);
    if let Some(ext) = path.extension() {
        if ext == "html" || ext == "md" {
            return Ok(file.to_string());
        }
    }
    Err(String::from(
        "Invalid file extension. Only .html and .md files are allowed.",
    ))
}

fn main() {
    let args = Args::parse();
    dbg!(args);

    dbg!("main.rs");
    modules::main(args);
}
