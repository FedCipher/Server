use std::{time::Duration, fmt};

use r2d2::{Pool, PooledConnection};
use redis::{Client, RedisError};

pub type R2D2Pool = Pool<Client>;
pub type R2D2PooledConnection = PooledConnection<Client>;

#[derive(Debug)]
pub enum RedisDatabaseError {
    OpenClient(RedisError),
    BuildPool(r2d2::Error),
    CreateConnection(r2d2::Error)
}

impl fmt::Display for RedisDatabaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisDatabaseError::OpenClient(error) => write!(formatter, "{}", error),
            RedisDatabaseError::BuildPool(error) => write!(formatter, "{}", error),
            RedisDatabaseError::CreateConnection(error) => write!(formatter, "{}", error)
        }
    }
}

impl std::error::Error for RedisDatabaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            RedisDatabaseError::OpenClient(ref error) => Some(error),
            RedisDatabaseError::BuildPool(ref error) => Some(error),
            RedisDatabaseError::CreateConnection(ref error) => Some(error)
        }
    }
}

const MAX_POOL_SIZE: u32 = 30;
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);

pub fn create_pool(host_addr: &str) -> Result<R2D2Pool, RedisDatabaseError> {
    let client = Client::open(host_addr).map_err(RedisDatabaseError::OpenClient)?;

    Pool::builder()
        .max_size(MAX_POOL_SIZE)
        .connection_timeout(CONNECTION_TIMEOUT)
        .build(client)
        .map_err(RedisDatabaseError::BuildPool)
}

pub fn get_connection(pool: &R2D2Pool) -> Result<R2D2PooledConnection, RedisDatabaseError> {
    pool.get().map_err(RedisDatabaseError::CreateConnection)
}
