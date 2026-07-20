use crate::payment_method::GatewayProvider;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub merchant: String,
    pub provider: GatewayProvider,
    pub use_provider: GatewayProvider,
    pub headers: HashMap<String, String>,
}
impl RequestContext {
    pub fn set_provider(&mut self, provider: GatewayProvider) {
        self.provider = provider;
    }
}
