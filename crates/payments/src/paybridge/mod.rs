//! PayBridge client library — wire types for the PayBridge microservice's
//! merchant API (`/api/v1/checkouts`) and its webhook events.

pub mod checkout_request;
pub mod checkout_response;
pub mod paybridge_service;
pub mod webhook;

pub use checkout_request::PayBridgeCreateCheckout;
pub use checkout_response::PayBridgeCheckoutDto;
pub use paybridge_service::PayBridgeService;
pub use webhook::PayBridgeWebhookEvent;
