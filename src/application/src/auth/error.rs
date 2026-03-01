use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Internal error")]
    Internal,
}