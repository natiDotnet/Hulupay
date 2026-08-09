use regex::Regex;

pub mod create_checkout;
pub mod gateway_response;
pub mod hulu_error;
pub mod hulu_response;
pub mod payment_gateway;
pub mod payment_gateway_error;
pub mod payment_method;
pub mod payment_request;
pub mod request_context;
pub mod claims;

// A simple, robust slugify function
pub fn create_slug(name: &str) -> String {
    // 1. Lowercase
    let lower = name.to_lowercase();

    // 2. Replace spaces with hyphens
    let replaced = lower.replace(" ", "-");

    // 3. Remove all non-alphanumeric characters except hyphens
    let re = Regex::new(r"[^a-z0-9\-]").unwrap();
    let slug = re.replace_all(&replaced, "");

    // 4. Clean up multiple hyphens (e.g., "---" -> "-")
    let re_multi = Regex::new(r"-+").unwrap();
    re_multi.replace_all(&slug, "-").to_string()
}
