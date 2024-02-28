use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn get_log_file() -> String {
    LOG_FILE.lock().unwrap().clone()
}

pub fn set_log_file(file: String) {
    *LOG_FILE.lock().unwrap() = file;
}

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();
    info!("starting up");
    warn!("oops, nothing implemented!");

    set_log_file(String::from("hello"));
    let a = get_log_file();
    info!("global value is {a}");

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    molly::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    info!("global value is {LOG_LEVEL}");
    molly::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

  