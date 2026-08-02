use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(
    Debug, Clone, Display, EnumString, Default, Deserialize, Serialize, Eq, PartialEq, toasty::Embed,
)]
#[column(rename_all = "UPPERCASE")]
pub enum PaymentMethod {
    #[default]
    None,
    Telebirr,
    Mpesa,
    CbeBirr,
    AwashBirr,
    Yaya,
    CoopayEbirr,
    ZamZam,
    Binget,
    Kacha,
    Boa,
    Amole,
    Unknown,
}

impl From<hulu_core::payment_method::PaymentMethod> for PaymentMethod {
    fn from(value: hulu_core::payment_method::PaymentMethod) -> Self {
        match value {
            hulu_core::payment_method::PaymentMethod::None => Self::None,
            hulu_core::payment_method::PaymentMethod::Telebirr => Self::Telebirr,
            hulu_core::payment_method::PaymentMethod::Mpesa => Self::Mpesa,
            hulu_core::payment_method::PaymentMethod::CbeBirr => Self::CbeBirr,
            hulu_core::payment_method::PaymentMethod::AwashBirr => Self::AwashBirr,
            hulu_core::payment_method::PaymentMethod::Awash => Self::AwashBirr,
            hulu_core::payment_method::PaymentMethod::Yaya => Self::Yaya,
            hulu_core::payment_method::PaymentMethod::CoopayEbirr => Self::CoopayEbirr,
            hulu_core::payment_method::PaymentMethod::ZamZam => Self::ZamZam,
            hulu_core::payment_method::PaymentMethod::Binget => Self::Binget,
            hulu_core::payment_method::PaymentMethod::Kacha => Self::Kacha,
            hulu_core::payment_method::PaymentMethod::Boa => Self::Boa,
            hulu_core::payment_method::PaymentMethod::Amole => Self::Amole,
            hulu_core::payment_method::PaymentMethod::Unknown(_) => Self::Unknown,
        }
    }
}
