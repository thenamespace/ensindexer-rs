pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Migrate(#[from] sqlx::migrate::MigrateError),
    #[error("invalid decimal value {0}")]
    InvalidDecimal(String),
    #[error("storage change buffer lock is poisoned")]
    ChangeBufferPoisoned,
    #[error("storage change buffer is already active")]
    ChangeBufferAlreadyActive,
    #[error("storage change buffer is not active")]
    ChangeBufferNotActive,
    #[error("storage event buffer lock is poisoned")]
    EventBufferPoisoned,
    #[error("storage event buffer is already active")]
    EventBufferAlreadyActive,
    #[error("storage event buffer is not active")]
    EventBufferNotActive,
}
