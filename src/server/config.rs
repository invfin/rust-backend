use crate::db::DatabasePools;
use axum::http::HeaderValue;
use menva::{get_env, get_int_env};
use std::net::IpAddr;

#[derive(Debug)]
pub enum EnvIs {
    Dev,
    Prod,
}

#[derive(Debug)]
pub struct Config {
    pub env: EnvIs,
    pub ip: IpAddr,
    pub port: u16,
    pub max_blocking_threads: Option<usize>,
    pub db: DatabasePools,
    pub session_key: cookie::Key,
    pub max_upload_size: u64,
    pub domain_name: String,
    pub allowed_origins: AllowedOrigins,
    pub ips_database: String,

    /// Should the server serve the frontend assets in the `dist` directory?
    pub serve_dist: bool,

    /// Should the server serve the frontend `index.html` for all
    /// non-API requests?
    pub serve_html: bool,
}

impl Config {
    /// Returns a default value for the application's config
    ///
    /// Sets the following default values:
    ///
    /// - `Config::max_upload_size`: 10MiB
    /// - `Config::ownership_invitations_expiration_days`: 30
    ///
    /// Pulls values from the following environment variables:
    ///
    ///
    /// - `SESSION_KEY`: The key used to sign and encrypt session cookies.
    /// This function panics if the Config configuration is invalid.
    pub fn from_environment() -> Self {
        Config {
            env: EnvIs::Dev,
            db: DatabasePools::full_from_environment(false),
            ip: [127, 0, 0, 1].into(),
            port: get_int_env("PORT").try_into().unwrap(),
            max_blocking_threads: Some(get_env("SERVER_THREADS").parse::<usize>().unwrap()),
            session_key: cookie::Key::derive_from(get_env("SESSION_KEY").as_bytes()),
            max_upload_size: 10 * 1024 * 1024, // 10 MB default file upload size limit
            domain_name: get_env("DOMAIN_NAME"),
            allowed_origins: AllowedOrigins::from_default_env(),
            ips_database: get_env("IPS_DATABASE"),
            serve_dist: true,
            serve_html: true,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AllowedOrigins(Vec<String>);

impl AllowedOrigins {
    pub fn from_default_env() -> Self {
        let allowed_origins = get_env("WEB_ALLOWED_ORIGINS")
            .split(',')
            .map(ToString::to_string)
            .collect();

        Self(allowed_origins)
    }

    pub fn contains(&self, value: &HeaderValue) -> bool {
        self.0.iter().any(|it| it == value)
    }
}
