pub struct CheckoutResponse {
    pub reference: String,
    pub checkout_url: String,
    pub amount: rust_decimal::Decimal,
}
