// This file contains intentional errors for CI failure testing
// DO NOT MERGE

// Format error: bad formatting
pub fn badly_formatted(){let x=1;let y=2;return x+y;}

// Clippy error: needless return
pub fn needless_return_example() -> i32 {
    return 42;
}

// Clippy error: unused variable
pub fn unused_var() {
    let _unused = 123;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_will_fail() {
        // This test will intentionally fail
        assert_eq!(1 + 1, 3, "Math is broken!");
    }
}

