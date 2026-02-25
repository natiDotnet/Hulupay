pub enum MerchantError {
    NotFound(String),
    Internal(anyhow::Error),
}