// Shared Context - 全コンテキスト共通

pub mod application;
pub mod domain;

/// Helper function for CI verification
/// Returns a formatted greeting message
pub fn format_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_greeting() {
        assert_eq!(format_greeting("World"), "Hello, World!");
        assert_eq!(format_greeting("Rust"), "Hello, Rust!");
    }
}
