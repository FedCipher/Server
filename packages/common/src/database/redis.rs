use std::{time::Duration, fmt};

use mobc::{Pool, Connection};
use mobc_redis::{redis::{Client, RedisError, IntoConnectionInfo}, RedisConnectionManager};

pub type MobcPool = Pool<RedisConnectionManager>;
pub type MobcConnection = Connection<RedisConnectionManager>;

#[derive(Debug)]
pub enum RedisDatabaseError {
    OpenClient(RedisError),
    CreateConnection(mobc::Error<RedisError>)
}

impl fmt::Display for RedisDatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisDatabaseError::OpenClient(error) => write!(formatter, "{}", error),
            RedisDatabaseError::CreateConnection(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for RedisDatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RedisDatabaseError::OpenClient(ref error) => Some(error),
            RedisDatabaseError::CreateConnection(ref error) => Some(error)
        }
    }
}

const MAX_POOL_SIZE: u64 = 30;
const CONNECTION_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

pub fn create_pool<T : IntoConnectionInfo>(host_addr: T) -> Result<MobcPool, RedisDatabaseError> {
    let client = Client::open(host_addr).map_err(RedisDatabaseError::OpenClient)?;

    let manager = RedisConnectionManager::new(client);

    let pool = Pool::builder()
        .max_open(MAX_POOL_SIZE)
        .get_timeout(CONNECTION_TIMEOUT)
        .build(manager);

    Ok(pool)
}

pub async fn get_connection(pool: &MobcPool) -> Result<MobcConnection, RedisDatabaseError> {
    pool
        .get()
        .await
        .map_err(RedisDatabaseError::CreateConnection)
}
