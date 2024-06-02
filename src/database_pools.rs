//! Configuration for setting up database pools
//!
//! - `DATABASE_URL`: The URL of the postgres database to use.
//! - `READ_ONLY_REPLICA_URL`: The URL of an optional postgres read-only replica database.
//! - `DB_PRIMARY_ASYNC_POOL_SIZE`: The number of connections of the primary database.
//! - `DB_REPLICA_ASYNC_POOL_SIZE`: The number of connections of the read-only / replica database.
//! - `DB_PRIMARY_MIN_IDLE`: The primary pool will maintain at least this number of connections.
//! - `DB_REPLICA_MIN_IDLE`: The replica pool will maintain at least this number of connections.
//! - `DB_OFFLINE`: If set to `leader` then use the read-only follower as if it was the leader.
//!   If set to `follower` then act as if `READ_ONLY_REPLICA_URL` was unset.
//! - `READ_ONLY_MODE`: If defined (even as empty) then force all connections to be read-only.
//! - `DB_TCP_TIMEOUT_MS`: TCP timeout in milliseconds. See the doc comment for more details.
use deadpool_diesel::postgres::{Hook, HookError};
use diesel::prelude::*;
use menva::{get_bool_env, get_env};
use std::time::Duration;

//TODO: refactor this and make sense of it
#[derive(Debug)]
pub struct DatabasePools {
    /// Settings for the primary database. This is usually writeable, but will be read-only in
    /// some configurations.
    pub primary: DbPoolConfig,
    /// An optional follower database. Always read-only.
    pub replica: Option<DbPoolConfig>,
    /// Number of seconds to wait for unacknowledged TCP packets before treating the connection as
    /// broken. This value will determine how long crates.io stays unavailable in case of full
    /// packet loss between the application and the database: setting it too high will result in an
    /// unnecessarily long outage (before the unhealthy database logic kicks in), while setting it
    /// too low might result in healthy connections being dropped.
    pub tcp_timeout_ms: u64,
    /// Time to wait for a connection to become available from the connection
    /// pool before returning an error.
    pub connection_timeout: Duration,
    /// Time to wait for a query response before canceling the query and
    /// returning an error.
    pub statement_timeout: Duration,
    /// Number of threads to use for asynchronous operations such as connection
    /// creation.
    pub helper_threads: usize,
    /// Whether to enforce that all the database connections are encrypted with TLS.
    pub enforce_tls: bool,
}

#[derive(Debug)]
pub struct DbPoolConfig {
    pub url: String,
    pub read_only_mode: bool,
    pub pool_size: usize,
    pub min_idle: Option<u32>,
}

impl DatabasePools {
    pub fn are_all_read_only(&self) -> bool {
        self.primary.read_only_mode
    }
}

impl DatabasePools {
    const DEFAULT_POOL_SIZE: usize = 3;

    /// Load settings for one or more database pools from the environment
    ///
    /// # Panics
    ///
    /// This function panics if `DB_OFFLINE=leader` but `READ_ONLY_REPLICA_URL` is unset.
    pub fn full_from_environment(enforce_tls: bool) -> Self {
        let leader_url = format!(
            "postgres://{}:{}@0.0.0.0:5432/{}",
            get_env("DATABASE_USERNAME"),
            get_env("DATABASE_PASSWORD"),
            get_env("DATABASE_NAME"),
        );
        let follower_url = None;
        let read_only_mode = get_bool_env("READ_ONLY_MODE");

        println!("leader_url {:?}", &leader_url);
        println!("*******************************************************************");

        let primary_async_pool_size = get_env("DB_PRIMARY_ASYNC_POOL_SIZE")
            .parse::<usize>()
            .unwrap();
        let replica_async_pool_size: usize = get_env("DB_REPLICA_ASYNC_POOL_SIZE")
            .parse::<usize>()
            .unwrap();

        let primary_min_idle = None;
        let replica_min_idle = None;

        let tcp_timeout_ms = get_env("DB_TCP_TIMEOUT_MS").parse::<u64>().unwrap();

        let connection_timeout = get_env("DB_TIMEOUT").parse::<u64>().unwrap();
        let connection_timeout = Duration::from_secs(connection_timeout);

        // `DB_TIMEOUT` currently configures both the connection timeout and
        // the statement timeout, so we can copy the parsed connection timeout.
        let statement_timeout = connection_timeout;

        let helper_threads = get_env("DB_HELPER_THREADS").parse::<usize>().unwrap();

        match get_env("DB_OFFLINE").as_str() {
            // The actual leader is down, use the follower in read-only mode as the primary and
            // don't configure a replica.
            "leader" => Self {
                primary: DbPoolConfig {
                    url: follower_url
                        .ok_or_else(|| {
                            info!(
                                "Must set `READ_ONLY_REPLICA_URL` when using `DB_OFFLINE=leader`."
                            )
                        })
                        .unwrap(),
                    read_only_mode: true,
                    pool_size: primary_async_pool_size,
                    min_idle: primary_min_idle,
                },
                replica: None,
                tcp_timeout_ms,
                connection_timeout,
                statement_timeout,
                helper_threads,
                enforce_tls,
            },
            // The follower is down, don't configure the replica.
            "follower" => Self {
                primary: DbPoolConfig {
                    url: leader_url,
                    read_only_mode,
                    pool_size: primary_async_pool_size,
                    min_idle: primary_min_idle,
                },
                replica: None,
                tcp_timeout_ms,
                connection_timeout,
                statement_timeout,
                helper_threads,
                enforce_tls,
            },
            _ => Self {
                primary: DbPoolConfig {
                    url: leader_url,
                    read_only_mode,
                    pool_size: primary_async_pool_size,
                    min_idle: primary_min_idle,
                },
                replica: follower_url.map(|url| DbPoolConfig {
                    url,
                    // Always enable read-only mode for the follower. In staging, we attach the
                    // same leader database to both environment variables and this ensures the
                    // connection is opened read-only even when attached to a writeable database.
                    read_only_mode: true,
                    pool_size: replica_async_pool_size,
                    min_idle: replica_min_idle,
                }),
                tcp_timeout_ms,
                connection_timeout,
                statement_timeout,
                helper_threads,
                enforce_tls,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConnectionConfig {
    pub statement_timeout: Duration,
    pub read_only: bool,
}

impl ConnectionConfig {
    fn apply(&self, conn: &mut PgConnection) -> QueryResult<()> {
        let statement_timeout = self.statement_timeout.as_millis();
        diesel::sql_query(format!("SET statement_timeout = {statement_timeout}")).execute(conn)?;

        if self.read_only {
            diesel::sql_query("SET default_transaction_read_only = 't'").execute(conn)?;
        }

        Ok(())
    }
}

impl From<ConnectionConfig> for Hook {
    fn from(config: ConnectionConfig) -> Self {
        Hook::async_fn(move |conn, _| {
            Box::pin(async move {
                conn.interact(move |conn| config.apply(conn))
                    .await
                    .map_err(|err| HookError::message(err.to_string()))?
                    .map_err(|err| HookError::message(err.to_string()))
            })
        })
    }
}
