use crate::application::cache_service::CacheService;
use anyhow::Result;
use phonenumber::{Mode, country, parse};
use serde::{Serialize, de::DeserializeOwned};

pub fn normalize(phone: &str) -> Result<String, phonenumber::ParseError> {
    let number = parse(Some(country::ET), phone)?;

    if !number.is_valid() {
        return Err(phonenumber::ParseError::InvalidCountryCode);
    }

    Ok(number.format().mode(Mode::E164).to_string())
}

pub const DEFAULT_CACHE_TTL: u64 = 300;
pub async fn set_cache<T>(
    cache: &dyn CacheService,
    key: &str,
    value: &T,
    ttl_seconds: Option<u64>,
) -> Result<()>
where
    T: Serialize,
{
    let json = serde_json::to_string(value)?;
    let ttl_seconds = ttl_seconds.unwrap_or(DEFAULT_CACHE_TTL);
    cache.set(key, json, ttl_seconds).await
}

pub async fn get_cache<T>(cache: &dyn CacheService, key: &str) -> Result<Option<T>>
where
    T: DeserializeOwned,
{
    let value = cache.get(key).await?;

    match value {
        Some(json) => Ok(Some(serde_json::from_str(&json)?)),
        None => Ok(None),
    }
}

#[macro_export]
macro_rules! cache_get {
    ($cache:expr, $type:ty, $key:expr) => {
        $crate::application::helper::get_cache::<$type>($cache, $key)
    };
}

#[macro_export]
macro_rules! cache_set {
    ($cache:expr, $key:expr, $value:expr) => {
        $crate::application::helper::set_cache($cache, $key, &$value, None)
    };

    ($cache:expr, $key:expr, $value:expr, $ttl:expr) => {
        $crate::application::helper::set_cache($cache, $key, &$value, Some($ttl))
    };
}
