use crate::application::WebhookHandler;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct HandleProviderWebhook {
    handlers: HashMap<String, Arc<dyn WebhookHandler>>,
}

impl HandleProviderWebhook {
    pub fn new(handlers: HashMap<String, Arc<dyn WebhookHandler>>) -> Self {
        Self { handlers }
    }

    pub async fn execute(
        &self,
        payload: serde_json::Value,
        provider: String,
    ) -> anyhow::Result<()> {
        let handler = self.handlers.get(&provider).ok_or_else(|| {
            anyhow::anyhow!("No webhook handler found for provider: {}", provider)
        })?;

        handler.handle_webhook(payload).await?;
        Ok(())
    }
}
