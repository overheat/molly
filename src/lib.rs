pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

pub mod config;

pub use config::Configs;

pub const CONFIG_FILE: &str = "configs/config.toml";