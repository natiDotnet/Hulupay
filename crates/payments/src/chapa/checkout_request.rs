use hulu_core::payment_request::{
    Beneficiary, CallbackUrls, CustomerInfo, Item, PaymentOptions, PaymentRequest,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::application::helper::normalize;

// ---------------------------------------------------------------------------
// Validation error type — mirrors the two error shapes observed from the
// upstream Chapa API (chapa-validation/report.md, section 12):
//
//   Field (Laravel-style, field-keyed):
//     {"message":{"amount":["validation.min.numeric"]},"status":"failed","data":null}
//
//   Plain (phone number only):
//     {"message":"Invalid Phone number, please use a proper phone number or
//      use business shortcode.","status":"failed","data":null}
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum ChapaRequestError {
    Field(&'static str, Vec<String>),
    Plain(&'static str),
}

pub(crate) const PHONE_INVALID_MSG: &str = "Invalid Phone number, please use a proper phone number or use business shortcode.";
const AMOUNT_MAX_MSG: &str = "The amount must not exceed 1000000.";
const FIRST_NAME_MAX_MSG: &str = "The first name must not exceed 35 characters.";
const LAST_NAME_MAX_MSG: &str = "The last name must not exceed 35 characters.";
const TX_REF_MAX_MSG: &str = "The tx ref must not exceed 50 characters.";
const CALLBACK_URL_MSG: &str = "The callback url must be a valid URL.";
const RETURN_URL_MSG: &str = "The return url must be a valid URL.";
const CURRENCY_FORMAT_MSG: &str =
    "The currency may only contain uppercase letters and numbers.";
const CURRENCY_UNSUPPORTED_MSG: &str = "Invalid currency or currency is not supported";

/// Currencies observed as supported upstream (ETB and USD both returned 200;
/// every other tested code — EUR, GBP, KES, AED, XXX — was rejected).
const SUPPORTED_CURRENCIES: [&str; 2] = ["ETB", "USD"];

// ---------------------------------------------------------------------------
// Wire-format request.
//
// Every scalar field is an `Option<Value>` (not a typed scalar) so that JSON
// type mismatches reach `validate()` instead of failing in serde with an
// axum plain-text rejection. Upstream Chapa coerces what it can and answers
// the rest with its own error codes — e.g. `amount: true` becomes
// {"amount":["validation.numeric"]}, `tx_ref: 12345` becomes
// {"tx_ref":["validation.string"]} — and so do we.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ChapaInitializeRequest {
    pub amount: Option<Value>,
    pub currency: Option<Value>,
    pub email: Option<Value>,
    pub first_name: Option<Value>,
    pub last_name: Option<Value>,
    pub phone_number: Option<Value>,
    pub tx_ref: Option<Value>,
    pub callback_url: Option<Value>,
    pub return_url: Option<Value>,
    pub customization: HashMap<String, serde_json::Value>,
    pub meta: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct ChapaInitializeRequestHelper {
    amount: Option<Value>,
    currency: Option<Value>,
    email: Option<Value>,
    first_name: Option<Value>,
    last_name: Option<Value>,
    phone_number: Option<Value>,
    tx_ref: Option<Value>,
    callback_url: Option<Value>,
    return_url: Option<Value>,

    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

impl<'de> Deserialize<'de> for ChapaInitializeRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = ChapaInitializeRequestHelper::deserialize(deserializer)?;

        let mut customization = HashMap::new();
        let mut meta = HashMap::new();

        for (key, value) in helper.extra {
            if let Some(field) = key
                .strip_prefix("customization[")
                .and_then(|s| s.strip_suffix(']'))
            {
                customization.insert(field.to_owned(), value);
            } else if let Some(field) = key.strip_prefix("meta[").and_then(|s| s.strip_suffix(']'))
            {
                meta.insert(field.to_owned(), value);
            } else if key == "customization" {
                // upstream also accepts a nested JSON object (CS01 -> 200)
                if let Value::Object(map) = value {
                    customization.extend(map);
                }
            } else if key == "meta" {
                if let Value::Object(map) = value {
                    meta.extend(map);
                }
            }
        }

        Ok(Self {
            amount: helper.amount,
            currency: helper.currency,
            email: helper.email,
            first_name: helper.first_name,
            last_name: helper.last_name,
            phone_number: helper.phone_number,
            tx_ref: helper.tx_ref,
            callback_url: helper.callback_url,
            return_url: helper.return_url,
            customization,
            meta,
        })
    }
}

// ---------------------------------------------------------------------------
// Validated request — produced by `ChapaInitializeRequest::validate()`.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ValidatedChapaRequest {
    pub amount: Decimal,
    pub currency: Option<String>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: Option<String>,
    pub tx_ref: String,
    pub callback_url: Option<String>,
    pub return_url: Option<String>,
    pub customization: HashMap<String, serde_json::Value>,
    pub meta: HashMap<String, serde_json::Value>,
}

impl ChapaInitializeRequest {
    /// Validate exactly like the upstream Chapa initialize endpoint
    /// (every rule observed in chapa-validation/report.md, sections 6 and 13).
    ///
    /// Like upstream, validation is early-exit: only the first failing field
    /// is reported per response. Field order follows the observed upstream
    /// priority (XB03: phone beats email; XB02/XB04: amount beats currency):
    /// phone, amount, currency, email, first_name, last_name, tx_ref,
    /// callback_url, return_url.
    pub fn validate(self) -> Result<ValidatedChapaRequest, ChapaRequestError> {
        let phone_number = validate_phone(self.phone_number.as_ref())?;
        let amount = validate_amount(self.amount.as_ref())?;
        let currency = validate_currency(self.currency.as_ref())?;
        let email = validate_email(self.email.as_ref())?;
        let first_name = validate_name(
            self.first_name.as_ref(),
            "first_name",
            FIRST_NAME_MAX_MSG,
        )?;
        let last_name = validate_name(self.last_name.as_ref(), "last_name", LAST_NAME_MAX_MSG)?;
        let tx_ref = validate_tx_ref(self.tx_ref.as_ref())?;
        let callback_url =
            validate_url(self.callback_url.as_ref(), "callback_url", CALLBACK_URL_MSG)?;
        let return_url = validate_url(self.return_url.as_ref(), "return_url", RETURN_URL_MSG)?;

        Ok(ValidatedChapaRequest {
            amount,
            currency,
            email,
            first_name,
            last_name,
            phone_number,
            tx_ref,
            callback_url,
            return_url,
            customization: self.customization,
            meta: self.meta,
        })
    }
}

// ---------------------------------------------------------------------------
// Field validators
// ---------------------------------------------------------------------------

/// amount: required; numeric (numeric strings coerced, bools rejected with
/// `validation.numeric`, objects/arrays with `validation.required`);
/// 1 <= amount <= 1,000,000.
fn validate_amount(v: Option<&Value>) -> Result<Decimal, ChapaRequestError> {
    let required = || ChapaRequestError::Field("amount", vec!["validation.required".into()]);
    let numeric = || ChapaRequestError::Field("amount", vec!["validation.numeric".into()]);

    let Some(v) = v else {
        return Err(required());
    };

    let dec = match v {
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Decimal::from(i)
            } else if let Some(u) = n.as_u64() {
                Decimal::from(u)
            } else {
                Decimal::from_f64(n.as_f64().unwrap_or(f64::NAN)).ok_or_else(numeric)?
            }
        }
        Value::String(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                return Err(required());
            }
            Decimal::from_str(trimmed).map_err(|_| numeric())?
        }
        Value::Bool(_) => return Err(numeric()),
        _ => return Err(required()),
    };

    if dec < Decimal::ONE {
        return Err(ChapaRequestError::Field(
            "amount",
            vec!["validation.min.numeric".into()],
        ));
    }
    if dec > Decimal::from(1_000_000u32) {
        return Err(ChapaRequestError::Field("amount", vec![AMOUNT_MAX_MSG.into()]));
    }

    Ok(dec)
}

/// currency: optional (empty/null/missing accepted, defaults to ETB
/// downstream); when present must be an uppercase alphanumeric 3-char code
/// and one of the supported currencies. Format-invalid values get both
/// messages; valid-format-but-unsupported get only the second
/// (CU07/CU08/CU09 vs CU02/CU04-CU06).
fn validate_currency(v: Option<&Value>) -> Result<Option<String>, ChapaRequestError> {
    let invalid_format = || {
        ChapaRequestError::Field(
            "currency",
            vec![CURRENCY_FORMAT_MSG.into(), CURRENCY_UNSUPPORTED_MSG.into()],
        )
    };

    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(None);
            }
            if SUPPORTED_CURRENCIES.contains(&s.as_str()) {
                return Ok(Some(s.clone()));
            }
            if s.len() == 3 && s.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
                Err(ChapaRequestError::Field(
                    "currency",
                    vec![CURRENCY_UNSUPPORTED_MSG.into()],
                ))
            } else {
                Err(invalid_format())
            }
        }
        _ => Err(invalid_format()),
    }
}

/// email: optional; RFC-syntax validated. NOTE: upstream Chapa additionally
/// performs DNS/MX validation on the domain (user@example.com is rejected
/// there because example.com publishes no MX records); we deliberately do
/// not replicate the network lookup. Number types get both rules (EM11).
fn validate_email(v: Option<&Value>) -> Result<Option<String>, ChapaRequestError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(None);
            }
            if is_valid_email(s) {
                Ok(Some(s.clone()))
            } else {
                Err(ChapaRequestError::Field(
                    "email",
                    vec!["validation.email".into()],
                ))
            }
        }
        _ => Err(ChapaRequestError::Field(
            "email",
            vec!["validation.string".into(), "validation.email".into()],
        )),
    }
}

/// first_name / last_name: optional; max 35 characters; no other
/// constraints (digits, special characters, Unicode all accepted upstream).
fn validate_name(
    v: Option<&Value>,
    field: &'static str,
    max_msg: &'static str,
) -> Result<Option<String>, ChapaRequestError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(None);
            }
            if s.chars().count() > 35 {
                return Err(ChapaRequestError::Field(field, vec![max_msg.into()]));
            }
            Ok(Some(s.clone()))
        }
        // numbers are coerced to strings, mirroring phone_number (PH14)
        Some(Value::Number(n)) => {
            let s = n.to_string();
            if s.chars().count() > 35 {
                return Err(ChapaRequestError::Field(field, vec![max_msg.into()]));
            }
            Ok(Some(s))
        }
        _ => Ok(None),
    }
}

/// phone_number: optional; Ethiopian mobile formats accepted
/// (09xxxxxxxx, 9xxxxxxxx, 2519xxxxxxxx, +2519xxxxxxxx — PH01-PH06);
/// everything else rejected with a plain-string message that is NOT
/// field-keyed (PH10-PH13). Numeric values are coerced to strings (PH14).
fn validate_phone(v: Option<&Value>) -> Result<Option<String>, ChapaRequestError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(None);
            }
            if is_ethiopian_phone(s) {
                Ok(Some(s.clone()))
            } else {
                Err(ChapaRequestError::Plain(PHONE_INVALID_MSG))
            }
        }
        Some(Value::Number(n)) => {
            let s = n.to_string();
            if is_ethiopian_phone(&s) {
                Ok(Some(s))
            } else {
                Err(ChapaRequestError::Plain(PHONE_INVALID_MSG))
            }
        }
        _ => Err(ChapaRequestError::Plain(PHONE_INVALID_MSG)),
    }
}

/// Ethiopian mobile formats observed as accepted upstream:
/// `09xxxxxxxx` | `9xxxxxxxx` | `251(9|7)xxxxxxxx` | `+251(9|7)xxxxxxxx`.
fn is_ethiopian_phone(s: &str) -> bool {
    let digits = s.strip_prefix('+').unwrap_or(s);
    let rest = digits
        .strip_prefix("251")
        .or_else(|| digits.strip_prefix('0'))
        .unwrap_or(digits);
    rest.len() == 9
        && matches!(rest.as_bytes()[0], b'9' | b'7')
        && rest.bytes().all(|b| b.is_ascii_digit())
}

/// tx_ref: optional (a reference is generated when absent, since upstream
/// generates one server-side — TR07/TR08/TR09); must be a string
/// (`validation.string` for numbers — TR10); max 50 characters (TR13/TR14).
fn validate_tx_ref(v: Option<&Value>) -> Result<String, ChapaRequestError> {
    match v {
        None | Some(Value::Null) => Ok(generate_tx_ref()),
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(generate_tx_ref());
            }
            if s.chars().count() > 50 {
                return Err(ChapaRequestError::Field(
                    "tx_ref",
                    vec![TX_REF_MAX_MSG.into()],
                ));
            }
            Ok(s.clone())
        }
        _ => Err(ChapaRequestError::Field(
            "tx_ref",
            vec!["validation.string".into()],
        )),
    }
}

fn generate_tx_ref() -> String {
    format!("hulupay-{}", Uuid::new_v4().simple())
}

/// callback_url / return_url: optional; validated only when present and
/// non-empty (CB04/CB05, RT04/RT05); any scheme is accepted — ftp included
/// (CB09) — but a scheme is required.
fn validate_url(
    v: Option<&Value>,
    field: &'static str,
    msg: &'static str,
) -> Result<Option<String>, ChapaRequestError> {
    match v {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => {
            if s.is_empty() {
                return Ok(None);
            }
            if is_valid_url(s) {
                Ok(Some(s.clone()))
            } else {
                Err(ChapaRequestError::Field(field, vec![msg.into()]))
            }
        }
        _ => Err(ChapaRequestError::Field(field, vec![msg.into()])),
    }
}

/// Approximation of Laravel's `url` rule: a `scheme://rest` shape where the
/// scheme starts with a letter and continues with alphanumeric/+/-.  chars,
/// and the remainder is a non-empty host-ish string.
fn is_valid_url(s: &str) -> bool {
    let Some((scheme, rest)) = s.split_once("://") else {
        return false;
    };
    let scheme_ok = scheme
        .chars()
        .next()
        .map_or(false, |c| c.is_ascii_alphabetic())
        && scheme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
    let rest = rest.trim();
    scheme_ok && !rest.is_empty() && !rest.starts_with('/') && !rest.starts_with('.')
}

/// Pragmatic RFC-syntax email check (EM01-EM07): `local@domain.tld` with a
/// dotted domain and an alphabetic TLD of at least two characters.
fn is_valid_email(s: &str) -> bool {
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    if s.chars().any(char::is_whitespace) {
        return false;
    }

    let local_ok = local
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '+' | '%' | '&'))
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..");
    if !local_ok {
        return false;
    }

    let labels: Vec<&str> = domain.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    let tld = labels[labels.len() - 1];
    if tld.len() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    labels
        .iter()
        .all(|l| !l.is_empty() && l.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'))
}

// ---------------------------------------------------------------------------
// Conversion into HuluPay's domain model
// ---------------------------------------------------------------------------

impl From<ValidatedChapaRequest> for PaymentRequest {
    fn from(value: ValidatedChapaRequest) -> Self {
        let name = match (&value.first_name, &value.last_name) {
            (Some(first), Some(last)) => format!("{first} {last}"),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => String::new(),
        };
        let return_url = value.return_url.clone().unwrap_or_default();

        Self {
            customer: CustomerInfo {
                email: value.email.unwrap_or_default(),
                phone: normalized_phone(value.phone_number.as_deref().unwrap_or("")),
                name,
            },
            payment: PaymentOptions {
                amount: value.amount,
                // upstream silently defaults a missing currency to ETB (CU11-CU13)
                currency: value.currency.unwrap_or_else(|| "ETB".to_string()),
                reference: value.tx_ref,
                payment_methods: vec![],
                lang: None,
                expire_date: None,
            },
            items: vec![Item {
                image: None,
                quantity: 1,
                price: value.amount,
                name: value
                    .customization
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                description: value
                    .customization
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
            }],
            callbacks: CallbackUrls {
                notify_url: value.callback_url.unwrap_or_default(),
                success_url: return_url.clone(),
                cancel_url: return_url.clone(),
                error_url: return_url,
            },
            beneficiaries: vec![Beneficiary {
                account_number: "01320811436100".to_string(),
                bank: "AWINETAA".to_string(),
                amount: value.amount,
            }],
            metadata: value.meta,
        }
    }
}

/// `normalize()` rejects bare `2519...` numbers (no `+`), which upstream
/// accepts (PH02/PH05) — retry with the plus prefix, then fall back to the
/// raw value instead of panicking like the previous `.unwrap()` did.
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Customization {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub invoices: Vec<Invoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub key: String,
    pub value: String,
}

pub struct Data {
    pub checkout_url: String,
}

pub struct ChapaInitializeResponse {
    pub message: String,
    pub status: String,
    pub data: Option<Data>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Helper: build a request from a JSON patch over the standard control
    /// payload (mirrors how the chapa-validation matrix varied one field).
    fn request(patch: Value) -> ChapaInitializeRequest {
        let mut base = json!({
            "amount": 100,
            "currency": "ETB",
            "email": "abebech_bekele@gmail.com",
            "first_name": "Bilen",
            "last_name": "Gizachew",
            "phone_number": "0905410217",
            "tx_ref": "hulupay-test-001",
            "callback_url": "https://example.com/callback",
            "return_url": "https://example.com/return",
        });
        if let (Some(target), Some(changes)) = (base.as_object_mut(), patch.as_object()) {
            for (key, value) in changes {
                target.insert(key.clone(), value.clone());
            }
        }
        serde_json::from_value(base).expect("deserialization must accept any JSON shape")
    }

    /// Helper: control payload with the given fields entirely absent.
    fn request_without(fields: &[&str]) -> ChapaInitializeRequest {
        let mut base = json!({
            "amount": 100,
            "currency": "ETB",
            "email": "abebech_bekele@gmail.com",
            "first_name": "Bilen",
            "last_name": "Gizachew",
            "phone_number": "0905410217",
            "tx_ref": "hulupay-test-001",
            "callback_url": "https://example.com/callback",
            "return_url": "https://example.com/return",
        });
        for field in fields {
            base.as_object_mut().unwrap().remove(*field);
        }
        serde_json::from_value(base).expect("deserialization must accept any JSON shape")
    }

    fn err_of(payload: Value) -> ChapaRequestError {
        request(payload).validate().expect_err("expected validation failure")
    }

    fn assert_field(err: ChapaRequestError, field: &str, messages: &[&str]) {
        match err {
            ChapaRequestError::Field(f, msgs) => {
                assert_eq!(f, field, "wrong field reported");
                assert_eq!(msgs, messages, "wrong messages for {field}");
            }
            other => panic!("expected Field error, got {other:?}"),
        }
    }

    fn assert_plain(err: ChapaRequestError) {
        assert!(
            matches!(err, ChapaRequestError::Plain(m) if m == PHONE_INVALID_MSG),
            "expected phone plain error, got {err:?}"
        );
    }

    // ---- amount (T001, AM02-AM25) -----------------------------------------

    #[test]
    fn amount_negative_rejected() {
        for amount in [json!(-10), json!(-100), json!(-1), json!(-0.01)] {
            assert_field(
                err_of(json!({ "amount": amount })),
                "amount",
                &["validation.min.numeric"],
            );
        }
    }

    #[test]
    fn amount_below_one_rejected() {
        for amount in [json!(0), json!(0.01), json!(0.001), json!(0.5), json!(0.99)] {
            assert_field(
                err_of(json!({ "amount": amount })),
                "amount",
                &["validation.min.numeric"],
            );
        }
    }

    #[test]
    fn amount_valid_range_accepted() {
        for amount in [json!(1), json!(10), json!(999.99), json!(1000), json!(1000000)] {
            request(json!({ "amount": amount }))
                .validate()
                .unwrap_or_else(|e| panic!("amount {amount} must be accepted: {e:?}"));
        }
    }

    #[test]
    fn amount_above_max_rejected() {
        assert_field(
            err_of(json!({ "amount": 999999999 })),
            "amount",
            &["The amount must not exceed 1000000."],
        );
    }

    #[test]
    fn amount_numeric_string_coerced() {
        for amount in [json!("100"), json!("100.50")] {
            request(json!({ "amount": amount })).validate().unwrap();
        }
    }

    #[test]
    fn amount_type_mismatches() {
        assert_field(
            err_of(json!({ "amount": "abc" })),
            "amount",
            &["validation.numeric"],
        );
        assert_field(
            err_of(json!({ "amount": true })),
            "amount",
            &["validation.numeric"],
        );
        for bad in [json!({}), json!([]), json!(""), json!(null)] {
            assert_field(err_of(json!({ "amount": bad })), "amount", &["validation.required"]);
        }
        // entirely missing behaves identically to null (AM17 == AM18)
        assert_field(
            request_without(&["amount"]).validate().unwrap_err(),
            "amount",
            &["validation.required"],
        );
    }

    // ---- currency (CU01-CU13) ----------------------------------------------

    #[test]
    fn currency_supported_accepted() {
        for currency in [json!("ETB"), json!("USD")] {
            request(json!({ "currency": currency })).validate().unwrap();
        }
    }

    #[test]
    fn currency_valid_format_unsupported_single_message() {
        for currency in [json!("EUR"), json!("GBP"), json!("KES"), json!("AED"), json!("XXX")] {
            assert_field(
                err_of(json!({ "currency": currency })),
                "currency",
                &["Invalid currency or currency is not supported"],
            );
        }
    }

    #[test]
    fn currency_bad_format_double_message() {
        let expected: &[&str] = &[
            "The currency may only contain uppercase letters and numbers.",
            "Invalid currency or currency is not supported",
        ];
        for currency in [json!("etb"), json!("Etb"), json!("ET"), json!("ETBB")] {
            assert_field(err_of(json!({ "currency": currency })), "currency", expected);
        }
    }

    #[test]
    fn currency_absent_accepted() {
        for currency in [json!(""), json!(null)] {
            request(json!({ "currency": currency })).validate().unwrap();
        }
        request_without(&["currency"]).validate().unwrap();
    }

    // ---- email (EM01-EM14) ---------------------------------------------------

    #[test]
    fn email_valid_accepted() {
        for email in [
            json!("abebech_bekele@gmail.com"),
            json!("user@gmail.com"),
            json!("user@microsoft.com"),
            json!("user+tag@gmail.com"),
            json!("user.name@gmail.com"),
            json!("user@sub.example.com"),
        ] {
            request(json!({ "email": email })).validate().unwrap();
        }
    }

    #[test]
    fn email_invalid_syntax_rejected() {
        for email in [
            json!("user@"),
            json!("@example.com"),
            json!("invalid"),
        ] {
            assert_field(
                err_of(json!({ "email": email })),
                "email",
                &["validation.email"],
            );
        }
    }

    #[test]
    fn email_number_type_double_rule() {
        assert_field(
            err_of(json!({ "email": 123 })),
            "email",
            &["validation.string", "validation.email"],
        );
    }

    #[test]
    fn email_absent_accepted() {
        for email in [json!(""), json!(null)] {
            request(json!({ "email": email })).validate().unwrap();
        }
        request_without(&["email"]).validate().unwrap();
    }

    #[test]
    fn email_dns_validated_domains_not_checked_offline() {
        // upstream rejects user@example.com on DNS/MX grounds (EM01);
        // user@nonexistent-domain-qwz123x.com (EM14) similarly.
        // We only validate RFC syntax, so both pass — documented divergence.
        request(json!({ "email": "user@example.com" })).validate().unwrap();
        request(json!({ "email": "user@nonexistent-domain-qwz123x.com" }))
            .validate()
            .unwrap();
    }

    // ---- names (FN/LN01-10, boundaries) ------------------------------------

    #[test]
    fn names_lenient_but_capped_at_35() {
        for field in ["first_name", "last_name"] {
            let msg = if field == "first_name" {
                "The first name must not exceed 35 characters."
            } else {
                "The last name must not exceed 35 characters."
            };
            let ok: String = "N".repeat(35);
            let too_long: String = "N".repeat(36);
            let patch = json!({ field: ok });
            request(patch).validate().unwrap();
            let patch2 = json!({ field: too_long });
            assert_field(err_of(patch2), field, &[msg]);
        }
    }

    #[test]
    fn names_accept_digits_specials_unicode_empty() {
        for value in [json!("A"), json!("John Doe"), json!("123"), json!("!@#"), json!("ብለን ግዛቸው"), json!("")] {
            for field in ["first_name", "last_name"] {
                request(json!({ field: value })).validate().unwrap();
            }
        }
    }

    // ---- phone (PH01-PH14) ---------------------------------------------------

    #[test]
    fn phone_ethiopian_formats_accepted() {
        for phone in [
            json!("0905410217"),
            json!("0912345678"),
            json!("251905410217"),
            json!("+251905410217"),
            json!("+251912345678"),
            json!("251912345678"),
            json!("905410217"),
            json!(905410217), // number type coerced (PH14)
            json!(""),
            json!(null),
        ] {
            request(json!({ "phone_number": phone })).validate().unwrap();
        }
        request_without(&["phone_number"]).validate().unwrap();
    }

    #[test]
    fn phone_invalid_plain_string_error() {
        for phone in [
            json!("abc123!@#"),
            json!("123"),
            json!("2519111111111111111"), // 20 digits
            json!("+1 555 123 4567"),
        ] {
            assert_plain(err_of(json!({ "phone_number": phone })));
        }
    }

    // ---- tx_ref (TR01-TR14) ---------------------------------------------------

    #[test]
    fn tx_ref_length_capped_at_50() {
        let ok = "t".repeat(50);
        let too_long = "t".repeat(51);
        request(json!({ "tx_ref": ok })).validate().unwrap();
        assert_field(
            err_of(json!({ "tx_ref": too_long })),
            "tx_ref",
            &["The tx ref must not exceed 50 characters."],
        );
    }

    #[test]
    fn tx_ref_number_type_rejected() {
        assert_field(
            err_of(json!({ "tx_ref": 12345 })),
            "tx_ref",
            &["validation.string"],
        );
    }

    #[test]
    fn tx_ref_absent_generated() {
        for value in [json!(""), json!(null)] {
            let validated = request(json!({ "tx_ref": value })).validate().unwrap();
            assert!(validated.tx_ref.starts_with("hulupay-"));
            assert!(validated.tx_ref.len() <= 50);
        }
        let validated = request_without(&["tx_ref"]).validate().unwrap();
        assert!(validated.tx_ref.starts_with("hulupay-"));
    }

    // ---- urls (CB01-CB09, RT01-RT08) ----------------------------------------

    #[test]
    fn urls_valid_accepted() {
        for patch in [
            json!({ "callback_url": "http://example.com/callback" }),
            json!({ "callback_url": "https://example.com" }),
            json!({ "callback_url": "https://example.com/cb?payment=123&sig=abc" }),
            json!({ "callback_url": "ftp://files.example.com/hook" }),
            json!({ "return_url": "http://example.com/return" }),
            json!({ "return_url": "https://example.com/return?order=42" }),
        ] {
            request(patch).validate().unwrap();
        }
    }

    #[test]
    fn urls_schemeless_rejected() {
        assert_field(
            err_of(json!({ "callback_url": "invalid-url" })),
            "callback_url",
            &["The callback url must be a valid URL."],
        );
        assert_field(
            err_of(json!({ "callback_url": "www.example.com" })),
            "callback_url",
            &["The callback url must be a valid URL."],
        );
        assert_field(
            err_of(json!({ "return_url": "not-a-url" })),
            "return_url",
            &["The return url must be a valid URL."],
        );
    }

    #[test]
    fn urls_absent_accepted() {
        for field in ["callback_url", "return_url"] {
            for value in [json!(""), json!(null)] {
                request(json!({ field: value })).validate().unwrap();
            }
            request_without(&[field]).validate().unwrap();
        }
    }

    // ---- combinations (XB02-XB05) ---------------------------------------------

    #[test]
    fn combo_single_error_priority() {
        // XB02: amount missing + currency missing -> only amount reported
        assert_field(
            request_without(&["amount", "currency"])
                .validate()
                .unwrap_err(),
            "amount",
            &["validation.required"],
        );

        // XB04: amount 0 + currency "" -> amount error wins
        assert_field(
            err_of(json!({ "amount": 0, "currency": "" })),
            "amount",
            &["validation.min.numeric"],
        );

        // XB03: invalid email + invalid phone -> plain phone error wins
        assert_plain(err_of(json!({ "email": "invalid", "phone_number": "abcdef" })));
    }

    #[test]
    fn combo_all_optional_fields_missing_accepted() {
        // XB05: email + first_name + last_name missing -> 200 upstream
        request_without(&["email", "first_name", "last_name"])
            .validate()
            .unwrap();
    }

    // ---- deserialization shapes ------------------------------------------------

    #[test]
    fn bracket_and_nested_customization_meta_accepted() {
        let payload = json!({
            "amount": 100,
            "currency": "ETB",
            "email": "abebech_bekele@gmail.com",
            "first_name": "Bilen",
            "last_name": "Gizachew",
            "phone_number": "0905410217",
            "tx_ref": "bracket-keys-001",
            "customization[title]": "Payment for my favourite merchant",
            "customization[description]": "I love online payments.",
            "meta[hide_receipt]": "false"
        });
        let req: ChapaInitializeRequest = serde_json::from_value(payload).unwrap();
        assert_eq!(
            req.customization.get("title").and_then(Value::as_str),
            Some("Payment for my favourite merchant")
        );
        assert_eq!(req.meta.get("hide_receipt").and_then(Value::as_str), Some("false"));

        let payload = json!({
            "amount": 100,
            "currency": "ETB",
            "email": "abebech_bekele@gmail.com",
            "first_name": "Bilen",
            "last_name": "Gizachew",
            "phone_number": "0905410217",
            "tx_ref": "nested-keys-001",
            "customization": { "title": "Nested Title", "description": "Nested description" },
            "meta": { "hide_receipt": true }
        });
        let req: ChapaInitializeRequest = serde_json::from_value(payload).unwrap();
        assert_eq!(
            req.customization.get("title").and_then(Value::as_str),
            Some("Nested Title")
        );
        assert_eq!(req.meta.get("hide_receipt"), Some(&json!(true)));
    }

    #[test]
    fn currency_defaults_to_etb_in_conversion() {
        let validated = request(json!({ "currency": null })).validate().unwrap();
        let pr = PaymentRequest::from(validated);
        assert_eq!(pr.payment.currency, "ETB");
    }
}
