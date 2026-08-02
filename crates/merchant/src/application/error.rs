use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Merchant not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("Database error: {0}")]
    Database(#[from] toasty::Error),
}
