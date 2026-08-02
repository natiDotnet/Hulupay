/// Build the Toasty `Db` with all domain models registered.
/// Shared between the main server binary and the `migrate` binary.
pub async fn build_toasty_db(db_url: &str) -> anyhow::Result<toasty::Db> {
    let db = toasty::Db::builder()
        .models(toasty::models!(
            auth::domain::user::User,
            auth::domain::apikey::ApiKey,
            auth::domain::refresh_token::RefreshToken,
            auth::domain::revoked_token::RevokedToken,
            auth::domain::role_permission::RolePermission,
            auth::domain::email_verification::EmailVerification,
            auth::domain::password_reset::PasswordReset,
            merchant::domain::merchant::Merchant,
            payments::domain::payment_order::PaymentOrder,
            payments::domain::payment_transaction::PaymentTransaction,
            payments::domain::payment_provider::PaymentProvider,
            payments::domain::merchant_config::MerchantConfig,
            payments::domain::merchant_routing_rule::MerchantRoutingRule,
            payments::domain::merchant_routing_strategy::MerchantRoutingStrategy,
            payments::domain::provider_metric::ProviderMetric,
            payments::domain::provider_payment_method::ProviderPaymentMethod,
            payments::domain::payments::payment_callback::PaymentCallback,
            payments::domain::payments::payment_customer::PaymentCustomer,
            payments::domain::payments::payment_item::PaymentItem,
            payments::domain::payments::merchant_webhook::MerchantWebhook,
        ))
        .connect(db_url)
        .await
        .map_err(|e| anyhow::anyhow!("toasty connect error: {:?}", e))?;
    Ok(db)
}
