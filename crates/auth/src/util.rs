/// Conversion utilities between `jiff::Timestamp` and `chrono::jiff::Timestamp`.
/// Used at domain/DTO boundaries where the DB layer stores jiff timestamps
/// but the application layer (JWT, API responses) uses chrono.

/// Convert a `jiff::Timestamp` to `chrono::DateTime<chrono::Utc>`.
pub fn to_chrono(ts: jiff::Timestamp) -> chrono::DateTime<chrono::Utc> {
    let millis = ts.as_millisecond();
    chrono::DateTime::from_timestamp_millis(millis).expect("valid chrono timestamp")
}

/// Convert a `chrono::DateTime<chrono::Utc>` to `jiff::Timestamp`.
pub fn to_jiff(dt: chrono::DateTime<chrono::Utc>) -> jiff::Timestamp {
    jiff::Timestamp::from_millisecond(dt.timestamp_millis()).expect("valid jiff timestamp")
}

/// Get the current time as a `jiff::Timestamp`.
pub fn now_jiff() -> jiff::Timestamp {
    jiff::Timestamp::now()
}
