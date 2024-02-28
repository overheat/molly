use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::sync::OnceLock;
use serde::Deserialize;
use std::fs;

const CONFIG_FILE: &str = "configs/config.toml";

#[derive(Deserialize, Debug)]
pub struct Configs {
    pub iot: String,
    pub iot_ats: String,
    // pub credential: String,
    // pub jobs: String,
    pub ca: String,
    pub cert: String,
    pub key: String,
}

static CONFIGS: OnceLock<Configs> = OnceLock::new();

impl Configs {
    pub fn init() {
        let config = Configs::from_config_file().unwrap();
    
        CONFIGS.set(config).unwrap();
    }

    fn from_config_file() -> Result<Configs, std::io::Error> {
        let config =
            fs::read_to_string(CONFIG_FILE).expect("Something went wrong reading the file");
        let config: Configs = toml::from_str(&config).unwrap();
        Ok(config)
    }

    pub fn global() -> &'static Configs {
        CONFIGS.get().expect("Configs is not initialized.")
    }
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

    Configs::init();
    print!("{:?}", CONFIGS.get());

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    molly::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}


#[test]
fn find_a_match() {
    let mut result = Vec::new();
    molly::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
    fn global_config_test() {
        use Configs;
        Configs::init();

        assert_eq!(Configs::global().ca, "certs/AmazonRootCA1.pem");
        assert_eq!(Configs::global().cert, "certs/certificate.pem.crt");
        assert_eq!(Configs::global().key, "certs/private.pem.key");
    }
  