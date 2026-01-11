use once_cell::sync::Lazy;
use regex::Regex;

pub fn is_datetime_valid(datetime_str: &str) -> bool {
    static DATETIME_REGEX: Lazy<Regex> = Lazy::new(|| {
        #[allow(clippy::unwrap_used)]
        Regex::new(r"^\d{2}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap()
    });
    DATETIME_REGEX.is_match(datetime_str)
}
