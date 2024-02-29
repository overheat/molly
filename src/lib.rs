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

pub const HELLO_WORLD_TOPIC: &str = "hello/world";
pub const AWS_IOT_MQTT_ALPN: &str = "mqtt";
// both Signature Version 4	and Custom authentication.
pub const MQTT_OVER_WSS_PORT: u16 = 443;
// X.509 client certificate
pub const MQTT_WITH_ALPN_PORT: u16 = 443;
pub const MQTT_OVER_TLS_PORT: u16 = 8883;
// Custom authentication
pub const MQTT_CUSTOM_AUTH_PORT: u16 = 443;
// MQTT without TLS, not support by AWS IoT core.
pub const MQTT_PORT: u16 = 8883;
// MQTT keepalive interval support by AWS IoT core.
// Minimal is 30 secondes.
pub const KEEP_ALIVE_INTERVAL: u64 = 30;