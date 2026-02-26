#[derive(Clone)]
pub struct ArifPayConfig {
    pub api_key: String,
    pub base_url: String,

    pub cancel_url: String,
    pub success_url: String,
    pub error_url: String,
    pub notify_url: String,

    pub account_number: String,
    pub bank: String,
}