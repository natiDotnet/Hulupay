use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ArifPayConfig {
    pub api_key: String,
    pub base_url: String,

    pub cancel_url: Option<String>,
    pub success_url: Option<String>,
    pub error_url: Option<String>,
    pub notify_url: String,

    pub account_number: String,
    pub bank: String,
    pub is_test_key: bool,
}

impl ArifPayConfig {
    pub fn new(api_key: String,
        is_test_key: bool,
    ) -> Self {

        let base_url = if is_test_key {
            "https://gateway.sandbox.arifpay.org".to_string()
        } else {
            "https://gateway.arifpay.org".to_string()
        };

        Self {
            api_key,
            is_test_key,
            base_url,
            notify_url: "".to_string(),
            cancel_url: None,
            success_url: None,
            error_url: None,
            account_number: "01320811436100".to_string(),
            bank: "AWINETAA".to_string(),
        }
    }
}