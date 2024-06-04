use crate::db::DatabasePools;
use axum::http::HeaderValue;
use menva::{get_env, get_int_env, read_env_file};
use std::{net::IpAddr, path::PathBuf, str::FromStr};

const DEFAULT_VERSION_ID_CACHE_SIZE: u64 = 10_000;
const DEFAULT_VERSION_ID_CACHE_TTL: u64 = 5 * 60; // 5 minutes

/// Maximum number of features a crate can have or that a feature itself can
/// enable. This value can be overridden in the database on a per-crate basis.
const DEFAULT_MAX_FEATURES: usize = 300;

/// Maximum number of dependencies a crate can have.
const DEFAULT_MAX_DEPENDENCIES: usize = 500;

#[derive(Debug)]
pub struct StorageConfig {
    dir: PathBuf,
    remote: Option<String>,
}

impl StorageConfig {
    pub fn new() -> Self {
        Self {
            dir: PathBuf::from_str(".").unwrap(),
            remote: None,
        }
    }
}

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
    pub storage: StorageConfig,

    pub session_key: cookie::Key,
    pub max_upload_size: u64,
    pub domain_name: String,
    pub allowed_origins: AllowedOrigins,

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
        let ip = [127, 0, 0, 1].into();

        let port = get_int_env("PORT");

        let max_blocking_threads = Some(get_env("SERVER_THREADS").parse::<usize>().unwrap());

        let storage = StorageConfig::new();

        let allowed_origins = AllowedOrigins::from_default_env();

        Config {
            env: EnvIs::Dev,
            db: DatabasePools::full_from_environment(false),
            storage,
            ip,
            port: port.try_into().unwrap(),
            max_blocking_threads,
            session_key: cookie::Key::derive_from(get_env("SESSION_KEY").as_bytes()),
            max_upload_size: 10 * 1024 * 1024, // 10 MB default file upload size limit
            domain_name: std::env::var("DOMAIN_NAME").unwrap_or_else(|_| "crates.io".into()),
            allowed_origins,
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
