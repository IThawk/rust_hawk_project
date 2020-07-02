pub mod client;
pub mod config;
pub mod log;
pub mod mysql;
pub mod postgres;
pub mod redis;
pub mod server;

pub fn default_ip_string() -> String {
    "127.0.0.1".to_string()
}
