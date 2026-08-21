use regex::Regex;

pub mod claims;
pub mod create_checkout;
pub mod gateway_response;
pub mod hulu_error;
pub mod hulu_response;
pub mod payment_gateway;
pub mod payment_gateway_error;
pub mod payment_method;
pub mod payment_request;
pub mod request_context;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slug_lowercases_input() {
        assert_eq!(create_slug("HuluPay"), "hulupay");
        assert_eq!(create_slug("ABC Supermarket"), "abc-supermarket");
    }

    #[test]
    fn slug_replaces_spaces_with_hyphens() {
        assert_eq!(create_slug("master merchant"), "master-merchant");
        assert_eq!(create_slug("a b c"), "a-b-c");
    }

    #[test]
    fn slug_strips_special_characters() {
        assert_eq!(create_slug("Hello, World!"), "hello-world");
        assert_eq!(create_slug("foo@bar.com"), "foobarcom");
    }

    #[test]
    fn slug_collapses_multiple_hyphens() {
        assert_eq!(create_slug("a---b"), "a-b");
        assert_eq!(create_slug("a - b - c"), "a-b-c");
    }

    #[test]
    fn slug_empty_input_stays_empty() {
        assert_eq!(create_slug(""), "");
        assert_eq!(create_slug("!!!"), "");
    }
}
