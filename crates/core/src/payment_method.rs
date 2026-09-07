use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Display, EnumString, Default, Deserialize, Serialize, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum PaymentMethod {
    #[default]
    None,
    Telebirr,
    Mpesa,
    CbeBirr,
    AwashBirr,
    Awash,
    Yaya,
    CoopayEbirr,
    ZamZam,
    Binget,
    Kacha,
    Boa,
    Amole,
    Unknown(String),
}

#[derive(Debug, Copy, Clone, Display, EnumString, Deserialize, Serialize, Eq, PartialEq)]
#[strum(serialize_all = "UPPERCASE")]
pub enum GatewayProvider {
    Hulu,
    ArifPay,
    Chapa,
    Simulator,
    LakiPay,
    StarPay,
}
