use storage::StorageError;

pub type ProjectionResult<T> = Result<T, ProjectionError>;

#[derive(Debug, thiserror::Error)]
pub enum ProjectionError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error("invalid built-in ENS constant {0}")]
    InvalidConstant(String),
    #[error("block number {0} does not fit GraphQL Int")]
    BlockNumberOutOfRange(i64),
    #[error("projection handler is not implemented yet: {0}")]
    Unimplemented(&'static str),
}
