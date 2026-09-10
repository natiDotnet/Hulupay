use hulu_core::payment_request::{
    Beneficiary, CallbackUrls, CustomerInfo, Item, PaymentOptions, PaymentRequest,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use utoipa::ToSchema;

use crate::application::helper::normalize;

// ---------------------------------------------------------------------------
// Validation error type — mirrors the error behavior observed from the
// upstream LakiPay v2 checkout API (lakipay-validation/report.md):
//
//   Schema errors (multi!): ALL failing fields in one semicolon-joined
//   message, in struct field order (XB02, XB03):
//     {"success":false,"message":"amount: cannot be blank; currency: cannot be blank."}
//
//   Go unmarshal (type mismatch) errors fail the whole request and are
//   returned alone, leaking the Go struct (AM18-AM26):
//     {"success":false,"message":"json: cannot unmarshal string into Go
//      struct field HostedCheckoutRequest.amount of type float64"}
//
//   Post-schema business checks run only when schema validation passed
//   (XB05: a reference error suppressed the mediums error) and carry no
//   field prefix (SM07/SM08/SM12):
//     {"success":false,"message":"unsupported medium: mpesa"}
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum LakiRequestError {
    /// Schema validation failures — one (field, message) pair per failing
    /// field, rendered joined with "; " in field order.
    Fields(Vec<(&'static str, String)>),
    /// Post-schema business-logic messages without a field prefix
    /// ("unsupported medium: X").
    Plain(String),
    /// Go JSON unmarshal error text, verbatim — a type mismatch aborts the
    /// entire request like Go's json.Unmarshal failing before validation.
    Unmarshal(String),
}

const AMOUNT_MIN: &str = "amount: must be no less than 0.01.";
const BLANK_SUFFIX: &str = "cannot be blank.";
const CURRENCY_LEN: &str = "currency: the length must be exactly 3.";
const PHONE_LEN: &str = "phone_number: the length must be exactly 12.";
const REFERENCE_LEN: &str = "reference: the length must be between 1 and 100.";
const CALLBACK_URL_INVALID: &str = "callback_url: must be a valid URL.";

/// Media accepted upstream — case-sensitive UPPERCASE (SM01-SM03 accepted;
/// SM08 lowercase rejected; SM07 unknown rejected).
pub(crate) const SUPPORTED_MEDIUMS: [&str; 3] = ["MPESA", "TELEBIRR", "CBE"];

fn blank(field: &'static str) -> (&'static str, String) {
    (field, format!("{field}: {BLANK_SUFFIX}"))
}

fn unmarshal_error(actual: &str, field: &str, expected: &str) -> LakiRequestError {
    LakiRequestError::Unmarshal(format!(
        "json: cannot unmarshal {actual} into Go struct field HostedCheckoutRequest.{field} of type {expected}"
    ))
}

fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "bool",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
        Value::Null => "null",
    }
}

// ---------------------------------------------------------------------------
// Wire-format request.
//
// Every field is an `Option<Value>` (not a typed scalar) so JSON type
// mismatches reach `validate()` and produce the upstream Go unmarshal error
// shape instead of an axum plain-text rejection — LakiPay is STRICTLY typed
// upstream: numeric strings for `amount` fail (AM18), numbers for
// `phone_number` fail (PH13), etc.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LakiInitializeRequest {
    pub amount: Option<Value>,
    pub currency: Option<Value>,
    pub phone_number: Option<Value>,
    pub reference: Option<Value>,
    pub description: Option<Value>,
    pub callback_url: Option<Value>,
    pub redirects: Option<Value>,
    pub supported_mediums: Option<Value>,
}

// ---------------------------------------------------------------------------
// Validated request — produced by `LakiInitializeRequest::validate()`.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ValidatedLakiRequest {
    pub amount: Decimal,
    pub currency: String,
    pub phone_number: Option<String>,
    pub reference: String,
    pub description: Option<String>,
    pub callback_url: String,
    pub redirect_success: Option<String>,
    pub redirect_failed: Option<String>,
    pub supported_mediums: Vec<String>,
}

impl LakiInitializeRequest {
    /// Validate exactly like the upstream LakiPay v2 checkout endpoint
    /// (every rule observed in lakipay-validation/report.md).
    ///
    /// Unlike Chapa (single error per response), upstream reports ALL schema
    /// errors in one "; "-joined message in struct field order. Field order
    /// follows the observed priority (XB02: amount before currency; XB03:
    /// callback_url before phone_number):
    /// amount, currency, callback_url, phone_number, reference, description,
    /// redirects — then, only when schema validation passed, the mediums
    /// business check (XB05).
    pub fn validate(self) -> Result<ValidatedLakiRequest, LakiRequestError> {
        let mut errors: Vec<(&'static str, String)> = Vec::new();

        let amount = validate_amount(&self.amount, &mut errors)?;
        let currency = validate_currency(&self.currency, &mut errors)?;
        let callback_url = validate_callback_url(&self.callback_url, &mut errors)?;
        let phone_number = validate_phone(&self.phone_number, &mut errors)?;
        let reference = validate_reference(&self.reference, &mut errors)?;
        let description = validate_description(&self.description, &mut errors)?;
        let (redirect_success, redirect_failed) =
            validate_redirects(&self.redirects, &mut errors)?;
        let mediums = validate_mediums_shape(&self.supported_mediums, &mut errors)?;

        if !errors.is_empty() {
            return Err(LakiRequestError::Fields(errors));
        }

        let mediums = mediums.expect("supported_mediums valid when no schema errors");

        // Post-schema business check: medium membership (case-sensitive).
        for medium in &mediums {
            let name = medium.as_deref().unwrap_or("");
            if !SUPPORTED_MEDIUMS.contains(&name) {
                return Err(LakiRequestError::Plain(format!(
                    "unsupported medium: {name}"
                )));
            }
        }

        // Unwrap guards: every Option below is set when `errors` is empty.
        Ok(ValidatedLakiRequest {
            amount: amount.expect("amount valid when no schema errors"),
            currency: currency.expect("currency valid when no schema errors"),
            phone_number,
            reference: reference.expect("reference valid when no schema errors"),
            description,
            callback_url: callback_url.expect("callback_url valid when no schema errors"),
            redirect_success,
            redirect_failed,
            supported_mediums: mediums.into_iter().flatten().collect(),
        })
    }
}

// ---------------------------------------------------------------------------
// Field validators — rule failures (blank / length / URL) accumulate into
// `errors` in field order; type mismatches return Err immediately (Go fails
// the whole unmarshal before validation ever runs).
// ---------------------------------------------------------------------------

/// amount: required strict float64; 0 is BLANK ("cannot be blank", AM05 —
/// Go's required tag treats the zero value as empty), < 0.01 → min error
/// (AM02-AM04, AM06/AM07), >= 0.01 accepted (AM08+). Wrong JSON type →
/// unmarshal error (AM18-AM26: "abc", true, {}, [] all fail; strings are
/// NOT coerced upstream).
fn validate_amount(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<Decimal>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => {
            errors.push(blank("amount"));
            Ok(None)
        }
        Some(Value::Number(n)) => {
            let f = n.as_f64().unwrap_or(f64::NAN);
            if f == 0.0 {
                errors.push(blank("amount"));
                return Ok(None);
            }
            let dec = Decimal::from_f64(f)
                .unwrap_or_else(|| Decimal::from_str(&n.to_string()).unwrap_or(Decimal::ZERO));
            if dec < Decimal::new(1, 2) {
                errors.push(("amount", AMOUNT_MIN.to_string()));
                return Ok(None);
            }
            Ok(Some(dec))
        }
        Some(other) => Err(unmarshal_error(json_type_name(other), "amount", "float64")),
    }
}

/// currency: required; length-ONLY validation — any 3-character string is
/// accepted ("XXX", "etb", "Etb" all 200 upstream, CU06-CU08); 2/4 chars
/// rejected (CU09/CU10); empty/null/missing → blank (CU11-CU13).
fn validate_currency(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<String>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => {
            errors.push(blank("currency"));
            Ok(None)
        }
        Some(Value::String(s)) => {
            if s.is_empty() {
                errors.push(blank("currency"));
                return Ok(None);
            }
            if s.len() != 3 {
                errors.push(("currency", CURRENCY_LEN.to_string()));
                return Ok(None);
            }
            Ok(Some(s.clone()))
        }
        Some(other) => Err(unmarshal_error(json_type_name(other), "currency", "string")),
    }
}

/// callback_url: required; loose Go URL parsing — scheme-less domains like
/// "www.example.com" pass (CB05), ftp:// passes (CB09), http:// passes
/// (CB01); only non-URLs like "invalid-url" fail (CB04). Empty/null/missing
/// → blank (CB06-CB08).
fn validate_callback_url(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<String>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => {
            errors.push(blank("callback_url"));
            Ok(None)
        }
        Some(Value::String(s)) => {
            if s.is_empty() {
                errors.push(blank("callback_url"));
                return Ok(None);
            }
            if is_laki_url(s) {
                Ok(Some(s.clone()))
            } else {
                errors.push(("callback_url", CALLBACK_URL_INVALID.to_string()));
                Ok(None)
            }
        }
        Some(other) => Err(unmarshal_error(json_type_name(other), "callback_url", "string")),
    }
}

/// phone_number: optional; length-ONLY validation when non-empty — exactly
/// 12 bytes ("abc123!@#" fails only on length, PH06). ""/null/missing all
/// succeed (PH10-PH12). Number type → unmarshal error (PH13).
fn validate_phone(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<String>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            if s.is_empty() || s.len() == 12 {
                Ok(Some(s.clone()))
            } else {
                errors.push(("phone_number", PHONE_LEN.to_string()));
                Ok(None)
            }
        }
        Some(other) => Err(unmarshal_error(json_type_name(other), "phone_number", "string")),
    }
}

/// reference: required; length 1-100, any characters (spaces/dots/Unicode
/// all accepted, RF01-RF07); 255 chars → length error (RF08);
/// ""/null/missing → blank (RF09-RF11); number type → unmarshal error (RF12).
fn validate_reference(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<String>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => {
            errors.push(blank("reference"));
            Ok(None)
        }
        Some(Value::String(s)) => {
            if s.is_empty() {
                errors.push(blank("reference"));
                return Ok(None);
            }
            if s.len() > 100 {
                errors.push(("reference", REFERENCE_LEN.to_string()));
                return Ok(None);
            }
            Ok(Some(s.clone()))
        }
        Some(other) => Err(unmarshal_error(json_type_name(other), "reference", "string")),
    }
}

/// description: optional; any string accepted (empty DS01, 500 chars DS02,
/// Unicode DS03); null/missing succeed (DS06/DS07); number → unmarshal
/// error (DS08).
fn validate_description(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<String>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(other) => Err(unmarshal_error(json_type_name(other), "description", "string")),
    }
}

/// redirects: optional object (entity.TransactionRedirects); null/missing/{}
/// all succeed (RD06-RD08); child values are NEVER validated — "not-a-url"
/// and null children pass (RD01/RD05); partial objects fine (RD10/RD11);
/// string/array → unmarshal error naming the entity type (RD09/RD13).
fn validate_redirects(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<(Option<String>, Option<String>), LakiRequestError> {
    match v {
        None | Some(Value::Null) => Ok((None, None)),
        Some(Value::Object(map)) => Ok((
            map.get("success").and_then(Value::as_str).map(String::from),
            map.get("failed").and_then(Value::as_str).map(String::from),
        )),
        Some(other) => Err(unmarshal_error(
            json_type_name(other),
            "redirects",
            "entity.TransactionRedirects",
        )),
    }
}

/// supported_mediums (schema stage): required non-empty ARRAY of strings
/// (duplicates tolerated, SM11). Empty/null/missing → blank (SM04-SM06).
/// Non-array → unmarshal error naming []entity.TransactionMedium (SM09);
/// non-string ELEMENTS (numbers) → unmarshal error naming the element type
/// (SM10). Null elements pass the shape stage and fail the business stage
/// with an empty name (SM12).
fn validate_mediums_shape(
    v: &Option<Value>,
    errors: &mut Vec<(&'static str, String)>,
) -> Result<Option<Vec<Option<String>>>, LakiRequestError> {
    match v {
        None | Some(Value::Null) => {
            errors.push(blank("supported_mediums"));
            Ok(None)
        }
        Some(Value::Array(items)) => {
            if items.is_empty() {
                errors.push(blank("supported_mediums"));
                return Ok(None);
            }
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                match item {
                    Value::String(s) => out.push(Some(s.clone())),
                    Value::Null => out.push(None),
                    other => {
                        return Err(unmarshal_error(
                            json_type_name(other),
                            "supported_mediums",
                            "entity.TransactionMedium",
                        ));
                    }
                }
            }
            Ok(Some(out))
        }
        Some(other) => Err(unmarshal_error(
            json_type_name(other),
            "supported_mediums",
            "[]entity.TransactionMedium",
        )),
    }
}

/// Approximation of the upstream URL check (CB01-CB09): a `scheme://rest`
/// shape with a valid scheme, OR a scheme-less domain-like host
/// ("www.example.com" passes, "invalid-url" does not).
fn is_laki_url(s: &str) -> bool {
    if let Some((scheme, rest)) = s.split_once("://") {
        let scheme_ok = scheme
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
        return scheme_ok && !rest.is_empty();
    }
    // No scheme: must look like a dotted domain (optionally with port/path).
    let host = s.split(['/', '?', '#']).next().unwrap_or("");
    let host = host.split(':').next().unwrap_or(host);
    !host.is_empty()
        && host.contains('.')
        && host.split('.').all(|label| {
            !label.is_empty() && label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        })
}

// ---------------------------------------------------------------------------
// Conversion into HuluPay's domain model
// ---------------------------------------------------------------------------

impl From<ValidatedLakiRequest> for PaymentRequest {
    fn from(value: ValidatedLakiRequest) -> Self {
        let amount = value.amount;
        Self {
            customer: CustomerInfo {
                // LakiPay has no email/name fields — left empty.
                email: String::new(),
                phone: normalized_phone(value.phone_number.as_deref().unwrap_or("")),
                name: String::new(),
            },
            payment: PaymentOptions {
                amount,
                currency: value.currency,
                reference: value.reference,
                payment_methods: value.supported_mediums,
                lang: None,
                expire_date: None,
            },
            items: vec![Item {
                image: None,
                quantity: 1,
                price: amount,
                name: value.description.clone().unwrap_or_default(),
                description: value.description.unwrap_or_default(),
            }],
            callbacks: CallbackUrls {
                notify_url: value.callback_url,
                // Only one "failed" redirect exists upstream; error and
                // cancel share it.
                success_url: value.redirect_success.unwrap_or_default(),
                error_url: value.redirect_failed.clone().unwrap_or_default(),
                cancel_url: value.redirect_failed.unwrap_or_default(),
            },
            beneficiaries: vec![Beneficiary {
                account_number: "01320811436100".to_string(),
                bank: "AWINETAA".to_string(),
                amount,
            }],
            metadata: Default::default(),
        }
    }
}

/// `normalize()` rejects bare `2519...` numbers (no `+`), which upstream
/// accepts (PH02/PH05) — retry with the plus prefix, then fall back to the
/// raw value instead of panicking (12-char junk passes upstream's
/// length-only check).
fn normalized_phone(raw: &str) -> String {
    if raw.is_empty() {
        return String::new();
    }
    if let Ok(n) = normalize(raw) {
        return n;
    }
    if let Some(rest) = raw.strip_prefix("251") {
        if let Ok(n) = normalize(&format!("+251{rest}")) {
            return n;
        }
    }
    raw.to_string()
}

// ---------------------------------------------------------------------------
// Outbound wire format (LakiPayService → upstream). The service serializes
// THIS struct; phone must be 12 digits WITHOUT the leading `+` — upstream
// rejects "+251994000000" (PH05).
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Redirects {
    pub success: String,
    pub failed: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LakiCheckoutRequest {
    pub amount: Decimal,
    pub currency: String,
    pub phone_number: String,
    pub reference: String,
    pub description: String,
    pub callback_url: String,
    pub redirects: Redirects,
    pub supported_mediums: Vec<String>,
}

impl From<&PaymentRequest> for LakiCheckoutRequest {
    fn from(value: &PaymentRequest) -> Self {
        Self {
            amount: value.payment.amount,
            currency: value.payment.currency.clone(),
            phone_number: value
                .customer
                .phone
                .strip_prefix('+')
                .unwrap_or(&value.customer.phone)
                .to_string(),
            reference: value.payment.reference.clone(),
            description: value
                .items
                .first()
                .map(|item| item.description.clone())
                .unwrap_or_default(),
            callback_url: value.callbacks.notify_url.clone(),
            redirects: Redirects {
                failed: value.callbacks.error_url.clone(),
                success: value.callbacks.success_url.clone(),
            },
            supported_mediums: value.payment.payment_methods.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Helper: build a request from a JSON patch over the standard control
    /// payload (mirrors how the lakipay-validation matrix varied one field).
    fn request(patch: Value) -> LakiInitializeRequest {
        let mut base = json!({
            "amount": 10,
            "currency": "ETB",
            "phone_number": "251994000000",
            "reference": "hulupay-test-001",
            "description": "Payment for order #123",
            "callback_url": "https://example.com/callback",
            "redirects": {
                "success": "https://example.com/success",
                "failed": "https://example.com/failed"
            },
            "supported_mediums": ["MPESA", "TELEBIRR", "CBE"]
        });
        if let (Some(target), Some(changes)) = (base.as_object_mut(), patch.as_object()) {
            for (key, value) in changes {
                target.insert(key.clone(), value.clone());
            }
        }
        serde_json::from_value(base).expect("deserialization must accept any JSON shape")
    }

    fn request_without(fields: &[&str]) -> LakiInitializeRequest {
        let mut base = json!({
            "amount": 10,
            "currency": "ETB",
            "phone_number": "251994000000",
            "reference": "hulupay-test-001",
            "description": "Payment for order #123",
            "callback_url": "https://example.com/callback",
            "redirects": {
                "success": "https://example.com/success",
                "failed": "https://example.com/failed"
            },
            "supported_mediums": ["MPESA", "TELEBIRR", "CBE"]
        });
        for field in fields {
            base.as_object_mut().unwrap().remove(*field);
        }
        serde_json::from_value(base).expect("deserialization must accept any JSON shape")
    }

    fn err_of(payload: Value) -> LakiRequestError {
        request(payload)
            .validate()
            .expect_err("expected validation failure")
    }

    fn joined(err: LakiRequestError) -> String {
        match err {
            LakiRequestError::Fields(fields) => fields
                .into_iter()
                .map(|(_, msg)| msg)
                .collect::<Vec<_>>()
                .join("; "),
            other => panic!("expected Fields error, got {other:?}"),
        }
    }

    fn plain(err: LakiRequestError) -> String {
        match err {
            LakiRequestError::Plain(m) => m,
            other => panic!("expected Plain error, got {other:?}"),
        }
    }

    fn unmarshal(err: LakiRequestError) -> String {
        match err {
            LakiRequestError::Unmarshal(m) => m,
            other => panic!("expected Unmarshal error, got {other:?}"),
        }
    }

    // ---- amount (AM01-AM27) ---------------------------------------------------

    #[test]
    fn amount_valid_range_accepted() {
        for amount in [
            json!(0.01),
            json!(0.5),
            json!(0.99),
            json!(1),
            json!(9.99),
            json!(10),
            json!(9.555),
        ] {
            request(json!({ "amount": amount }))
                .validate()
                .unwrap_or_else(|e| panic!("amount {amount} must be accepted: {e:?}"));
        }
    }

    #[test]
    fn amount_below_min_rejected() {
        for amount in [json!(-100), json!(-1), json!(-0.01), json!(0.001), json!(0.009)] {
            assert_eq!(
                joined(err_of(json!({ "amount": amount }))),
                "amount: must be no less than 0.01."
            );
        }
    }

    #[test]
    fn amount_zero_is_blank_not_min() {
        // AM05: 0 → "cannot be blank", NOT "must be no less than 0.01".
        assert_eq!(joined(err_of(json!({ "amount": 0 }))), "amount: cannot be blank.");
        assert_eq!(
            joined(err_of(json!({ "amount": null }))),
            "amount: cannot be blank."
        );
        assert_eq!(
            joined(request_without(&["amount"]).validate().unwrap_err()),
            "amount: cannot be blank."
        );
    }

    #[test]
    fn amount_type_mismatches_are_go_unmarshal_errors() {
        for (value, ty) in [
            (json!("10"), "string"),
            (json!("10.50"), "string"),
            (json!("abc"), "string"),
            (json!(""), "string"),
            (json!(true), "bool"),
            (json!({}), "object"),
            (json!([]), "array"),
        ] {
            assert_eq!(
                unmarshal(err_of(json!({ "amount": value }))),
                format!("json: cannot unmarshal {ty} into Go struct field HostedCheckoutRequest.amount of type float64")
            );
        }
    }

    // ---- currency (CU01-CU13) ---------------------------------------------------

    #[test]
    fn currency_any_three_chars_accepted() {
        // Length-ONLY validation: even "XXX" succeeds (CU06).
        for currency in ["ETB", "USD", "XXX", "etb", "Etb"] {
            request(json!({ "currency": currency }))
                .validate()
                .unwrap_or_else(|e| panic!("currency {currency} must be accepted: {e:?}"));
        }
    }

    #[test]
    fn currency_wrong_length_rejected() {
        for currency in ["ET", "ETBB"] {
            assert_eq!(
                joined(err_of(json!({ "currency": currency }))),
                "currency: the length must be exactly 3."
            );
        }
    }

    #[test]
    fn currency_blank_cases() {
        assert_eq!(
            joined(err_of(json!({ "currency": "" }))),
            "currency: cannot be blank."
        );
        assert_eq!(
            joined(err_of(json!({ "currency": null }))),
            "currency: cannot be blank."
        );
        assert_eq!(
            joined(request_without(&["currency"]).validate().unwrap_err()),
            "currency: cannot be blank."
        );
    }

    // ---- phone (PH01-PH14) --------------------------------------------------------

    #[test]
    fn phone_length_only_twelve() {
        // Valid: 12 bytes. "abc123!@#" is 9 bytes → length error, not content.
        request(json!({ "phone_number": "251991234567" })).validate().unwrap();
        assert_eq!(
            joined(err_of(json!({ "phone_number": "abc123!@#" }))),
            "phone_number: the length must be exactly 12."
        );
        assert_eq!(
            joined(err_of(json!({ "phone_number": "0994000000" }))),
            "phone_number: the length must be exactly 12."
        );
        assert_eq!(
            joined(err_of(json!({ "phone_number": "+251994000000" }))),
            "phone_number: the length must be exactly 12."
        );
        assert_eq!(
            joined(err_of(json!({ "phone_number": "2519940000001234567" }))),
            "phone_number: the length must be exactly 12."
        );
    }

    #[test]
    fn phone_optional_all_absent_forms_accepted() {
        for phone in [json!(""), json!(null)] {
            request(json!({ "phone_number": phone })).validate().unwrap();
        }
        request_without(&["phone_number"]).validate().unwrap();
    }

    #[test]
    fn phone_number_type_is_unmarshal_error() {
        assert_eq!(
            unmarshal(err_of(json!({ "phone_number": 994000000 }))),
            "json: cannot unmarshal number into Go struct field HostedCheckoutRequest.phone_number of type string"
        );
    }

    // ---- reference (RF01-RF14) -------------------------------------------------------

    #[test]
    fn reference_any_chars_up_to_100() {
        for reference in ["ABC123", "abc_123", "abc.123", "ref with spaces 007", "r".repeat(100).as_str()] {
            request(json!({ "reference": reference }))
                .validate()
                .unwrap_or_else(|e| panic!("reference {reference} must be accepted: {e:?}"));
        }
    }

    #[test]
    fn reference_over_100_rejected() {
        assert_eq!(
            joined(err_of(json!({ "reference": "r".repeat(255) }))),
            "reference: the length must be between 1 and 100."
        );
    }

    #[test]
    fn reference_blank_cases() {
        assert_eq!(
            joined(err_of(json!({ "reference": "" }))),
            "reference: cannot be blank."
        );
        assert_eq!(
            joined(err_of(json!({ "reference": null }))),
            "reference: cannot be blank."
        );
        assert_eq!(
            joined(request_without(&["reference"]).validate().unwrap_err()),
            "reference: cannot be blank."
        );
    }

    #[test]
    fn reference_number_type_is_unmarshal_error() {
        assert_eq!(
            unmarshal(err_of(json!({ "reference": 12345 }))),
            "json: cannot unmarshal number into Go struct field HostedCheckoutRequest.reference of type string"
        );
    }

    // ---- description (DS01-DS08) ---------------------------------------------------

    #[test]
    fn description_fully_optional() {
        for description in ["", "የክፍያ ለትእዛዝ ★", "D".repeat(500).as_str(), json!(null).as_str().unwrap()] {
            request(json!({ "description": description })).validate().unwrap();
        }
        request_without(&["description"]).validate().unwrap();
    }

    #[test]
    fn description_number_type_is_unmarshal_error() {
        assert_eq!(
            unmarshal(err_of(json!({ "description": 123 }))),
            "json: cannot unmarshal number into Go struct field HostedCheckoutRequest.description of type string"
        );
    }

    // ---- callback_url (CB01-CB09) ---------------------------------------------------

    #[test]
    fn callback_url_loose_parsing() {
        // http, no-path, query strings, scheme-less domains, ftp — all pass.
        for callback in [
            "http://example.com/callback",
            "https://example.com",
            "https://example.com/cb?payment=123",
            "www.example.com",
            "ftp://files.example.com/hook",
        ] {
            request(json!({ "callback_url": callback }))
                .validate()
                .unwrap_or_else(|e| panic!("callback {callback} must be accepted: {e:?}"));
        }
    }

    #[test]
    fn callback_url_invalid_rejected() {
        assert_eq!(
            joined(err_of(json!({ "callback_url": "invalid-url" }))),
            "callback_url: must be a valid URL."
        );
    }

    #[test]
    fn callback_url_required() {
        for callback in ["", json!(null).as_str().unwrap()] {
            assert_eq!(
                joined(err_of(json!({ "callback_url": callback }))),
                "callback_url: cannot be blank."
            );
        }
        assert_eq!(
            joined(request_without(&["callback_url"]).validate().unwrap_err()),
            "callback_url: cannot be blank."
        );
    }

    // ---- redirects (RD01-RD13) --------------------------------------------------------

    #[test]
    fn redirects_children_never_validated() {
        for redirects in [
            json!({ "success": "not-a-url", "failed": "https://example.com/failed" }),
            json!({ "success": "", "failed": "" }),
            json!({ "success": null, "failed": "https://example.com/failed" }),
            json!({}),
            json!({ "success": "https://example.com/success", "failed": "https://example.com/failed", "cancel": "https://example.com/cancel" }),
            json!(null),
        ] {
            request(json!({ "redirects": redirects }))
                .validate()
                .unwrap_or_else(|e| panic!("redirects {redirects} must be accepted: {e:?}"));
        }
        request_without(&["redirects"]).validate().unwrap();
    }

    #[test]
    fn redirects_wrong_container_types_are_unmarshal_errors() {
        for (value, ty) in [
            (json!("https://example.com/r"), "string"),
            (json!([]), "array"),
        ] {
            assert_eq!(
                unmarshal(err_of(json!({ "redirects": value }))),
                format!("json: cannot unmarshal {ty} into Go struct field HostedCheckoutRequest.redirects of type entity.TransactionRedirects")
            );
        }
    }

    // ---- supported_mediums (SM01-SM13) ---------------------------------------------------

    #[test]
    fn mediums_single_and_duplicates_accepted() {
        request(json!({ "supported_mediums": ["MPESA"] })).validate().unwrap();
        request(json!({ "supported_mediums": ["TELEBIRR"] })).validate().unwrap();
        request(json!({ "supported_mediums": ["CBE"] })).validate().unwrap();
        request(json!({ "supported_mediums": ["MPESA", "MPESA"] })).validate().unwrap();
    }

    #[test]
    fn mediums_required_non_empty() {
        for mediums in [json!([]), json!(null)] {
            assert_eq!(
                joined(err_of(json!({ "supported_mediums": mediums }))),
                "supported_mediums: cannot be blank."
            );
        }
        assert_eq!(
            joined(request_without(&["supported_mediums"]).validate().unwrap_err()),
            "supported_mediums: cannot be blank."
        );
    }

    #[test]
    fn mediums_unknown_and_lowercase_rejected_as_plain() {
        // Business-stage errors: no field prefix, single message.
        assert_eq!(
            plain(err_of(json!({ "supported_mediums": ["UNKNOWN_MEDIUM"] }))),
            "unsupported medium: UNKNOWN_MEDIUM"
        );
        // SM08: first bad element reported.
        assert_eq!(
            plain(err_of(json!({ "supported_mediums": ["mpesa", "telebirr", "cbe"] }))),
            "unsupported medium: mpesa"
        );
        // SM12: null element → empty name.
        assert_eq!(
            plain(err_of(json!({ "supported_mediums": ["MPESA", null] }))),
            "unsupported medium: "
        );
    }

    #[test]
    fn mediums_wrong_types_are_unmarshal_errors() {
        assert_eq!(
            unmarshal(err_of(json!({ "supported_mediums": "MPESA" }))),
            "json: cannot unmarshal string into Go struct field HostedCheckoutRequest.supported_mediums of type []entity.TransactionMedium"
        );
        assert_eq!(
            unmarshal(err_of(json!({ "supported_mediums": [123] }))),
            "json: cannot unmarshal number into Go struct field HostedCheckoutRequest.supported_mediums of type entity.TransactionMedium"
        );
        assert_eq!(
            unmarshal(err_of(json!({ "supported_mediums": {} }))),
            "json: cannot unmarshal object into Go struct field HostedCheckoutRequest.supported_mediums of type []entity.TransactionMedium"
        );
    }

    // ---- combinations (XB01-XB05) --------------------------------------------------------

    #[test]
    fn combo_multi_error_joined_in_field_order() {
        // XB02: amount missing + currency missing → BOTH, amount first.
        assert_eq!(
            joined(request_without(&["amount", "currency"]).validate().unwrap_err()),
            "amount: cannot be blank; currency: cannot be blank."
        );

        // XB03: invalid phone + invalid callback → BOTH, callback first.
        assert_eq!(
            joined(err_of(json!({
                "phone_number": "abcdef",
                "callback_url": "nope"
            }))),
            "callback_url: must be a valid URL; phone_number: the length must be exactly 12."
        );
    }

    #[test]
    fn combo_mediums_check_runs_only_after_schema_passes() {
        // XB05: 255-char reference + unknown medium → reference error only;
        // the mediums business check never runs when schema validation failed.
        let payload = json!({
            "amount": 10,
            "currency": "ETB",
            "phone_number": "251994000000",
            "reference": "r".repeat(255),
            "callback_url": "https://example.com/callback",
            "supported_mediums": ["NOPE"]
        });
        let req: LakiInitializeRequest = serde_json::from_value(payload).unwrap();
        assert_eq!(
            joined(req.validate().unwrap_err()),
            "reference: the length must be between 1 and 100."
        );
    }

    // ---- conversion -----------------------------------------------------------------------

    #[test]
    fn conversion_maps_all_fields() {
        let validated = request(json!({})).validate().unwrap();
        let pr = PaymentRequest::from(validated);
        assert_eq!(pr.payment.currency, "ETB");
        assert_eq!(pr.payment.reference, "hulupay-test-001");
        assert_eq!(pr.payment.payment_methods, vec!["MPESA", "TELEBIRR", "CBE"]);
        assert_eq!(pr.callbacks.notify_url, "https://example.com/callback");
        assert_eq!(pr.callbacks.success_url, "https://example.com/success");
        assert_eq!(pr.callbacks.error_url, "https://example.com/failed");
        assert_eq!(pr.items[0].description, "Payment for order #123");
    }

    #[test]
    fn outbound_phone_has_no_plus_prefix() {
        // Upstream rejects "+251994000000" (PH05); the service's outbound
        // struct must send 12 digits without the plus.
        let validated = request(json!({})).validate().unwrap();
        let pr = PaymentRequest::from(validated);
        let outbound = LakiCheckoutRequest::from(&pr);
        assert_eq!(outbound.phone_number, "251994000000");
        assert!(!outbound.phone_number.starts_with('+'));
    }
}
