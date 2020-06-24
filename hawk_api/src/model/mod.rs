pub mod config;
pub mod log;
pub mod mysql;
pub mod redis;
pub mod server;
pub mod client;
pub fn default_ip_string() -> String {
    "127.0.0.1".to_string()
}
