pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

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

#[test]
fn global_config_test() {
    use Configs;
    Configs::init();

    assert_eq!(Configs::global().ca, "certs/AmazonRootCA1.pem");
    assert_eq!(Configs::global().cert, "certs/certificate.pem.crt");
    assert_eq!(Configs::global().key, "certs/private.pem.key");
}
