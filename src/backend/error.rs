#![allow(dead_code)]
use crate::mysql::errors::MySQLError;

pub type BackendResult<T> = std::result::Result<T, BackendError>;

#[derive(Debug)]
pub enum BackendError {
    InnerErrPipeEmpty,
    InnerErrOfflineOrQuit,
    InnerErrGreaterThenMaxConnCount,
    PoolErrClusterIdNotFound(String),
    PoolErrNodeNotFound(String),
    PoolErrConnGrowFailed(String),
    PoolErrConnGrowGiveup(String),
    IO(std::io::Error),
    Mysql(MySQLError),
}

impl From<std::io::Error> for BackendError {
    fn from(e: std::io::Error) -> Self {
        BackendError::IO(e)
    }
}
impl From<MySQLError> for BackendError {
    fn from(e: MySQLError) -> Self {
        BackendError::Mysql(e)
    }
}
impl std::error::Error for BackendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BackendError::InnerErrPipeEmpty => None,
            BackendError::PoolErrClusterIdNotFound(..) => None,
            BackendError::PoolErrNodeNotFound(..) => None,
            BackendError::InnerErrOfflineOrQuit => None,
            BackendError::PoolErrConnGrowFailed(..) => None,
            BackendError::PoolErrConnGrowGiveup(..) => None,
            BackendError::InnerErrGreaterThenMaxConnCount => None,
            BackendError::IO(e) => e.source(),
            BackendError::Mysql(e) => e.source(),
        }
    }
}
impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendError::InnerErrPipeEmpty => write!(f, "node conn pipe is empty!"),
            BackendError::PoolErrClusterIdNotFound(id) => {
                write!(f, "cluster_id: {:?} not exist!", id)
            }
            BackendError::PoolErrNodeNotFound(id) => write!(f, "node: {:?}  not exist!", id),
            BackendError::InnerErrOfflineOrQuit => write!(f, "node offline or quit!"),
            BackendError::PoolErrConnGrowFailed(id) => {
                write!(f, "node conn grow failed! node_id: {:?}", id)
            }
            BackendError::PoolErrConnGrowGiveup(id) => write!(f, "node: {:?} give up grow!", id),
            BackendError::InnerErrGreaterThenMaxConnCount => {
                write!(f, "total conn count >= max conn limit!")
            }
            BackendError::IO(e) => e.fmt(f),
            BackendError::Mysql(e) => e.fmt(f),
        }
    }
}
