use hulu_core::payment_method::PaymentMethod;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ArifPaymentMethod {
    #[serde(rename = "TELEBIRR")]
    TeleBirr,
    #[serde(rename = "AWASH")]
    Awash,
    #[serde(rename = "AWASH_BIRR")]
    AwashBirr,
    #[serde(rename = "BINGET")]
    Binget,
    #[serde(rename = "YAYA")]
    Yaya,
    #[serde(rename = "KACHA")]
    Kacha,
    // Pss,
    #[serde(rename = "CBE")]
    Cbe,
    Amole,
    Boa,
    ZamZam,
    #[serde(rename = "TELEBIRR_USSD")]
    TeleBirrUssd,
    Mpesa,
    Unknown(String),
}

impl From<ArifPaymentMethod> for PaymentMethod {
    fn from(value: ArifPaymentMethod) -> Self {
        match value {
            ArifPaymentMethod::TeleBirr => PaymentMethod::Telebirr,
            ArifPaymentMethod::Awash => PaymentMethod::Awash,
            ArifPaymentMethod::AwashBirr => PaymentMethod::AwashBirr,
            ArifPaymentMethod::Binget => PaymentMethod::Binget,
            ArifPaymentMethod::Yaya => PaymentMethod::Yaya,
            ArifPaymentMethod::Kacha => PaymentMethod::Kacha,
            ArifPaymentMethod::Cbe => PaymentMethod::CbeBirr,
            ArifPaymentMethod::Amole => PaymentMethod::Amole,
            ArifPaymentMethod::Boa => PaymentMethod::Boa,
            ArifPaymentMethod::ZamZam => PaymentMethod::ZamZam,
            ArifPaymentMethod::TeleBirrUssd => PaymentMethod::Telebirr,
            ArifPaymentMethod::Mpesa => PaymentMethod::Mpesa,
            ArifPaymentMethod::Unknown(s) => PaymentMethod::Unknown(s),
        }
    }
}

impl From<PaymentMethod> for ArifPaymentMethod {
    fn from(value: PaymentMethod) -> Self {
        match value {
            PaymentMethod::Telebirr => ArifPaymentMethod::TeleBirr,
            PaymentMethod::Awash => ArifPaymentMethod::Awash,
            PaymentMethod::AwashBirr => ArifPaymentMethod::AwashBirr,
            PaymentMethod::Binget => ArifPaymentMethod::Binget,
            PaymentMethod::Yaya => ArifPaymentMethod::Yaya,
            PaymentMethod::Kacha => ArifPaymentMethod::Kacha,
            PaymentMethod::CbeBirr => ArifPaymentMethod::Cbe,
            PaymentMethod::Amole => ArifPaymentMethod::Amole,
            PaymentMethod::Boa => ArifPaymentMethod::Boa,
            PaymentMethod::ZamZam => ArifPaymentMethod::ZamZam,
            PaymentMethod::Mpesa => ArifPaymentMethod::Mpesa,
            _ => ArifPaymentMethod::Unknown(value.to_string()),
        }
    }
}
