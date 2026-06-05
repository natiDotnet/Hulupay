use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GatewayResponse<T> {
    pub status: u16,
    pub message: String,
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<serde_json::Value>,
    pub row_response: Option<serde_json::Value>,
}
