fn main() {
    println!("Running test_continue");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_that_fails() {
        assert_eq!(1, 2, "This test is designed to fail");
    }

    #[test]
    fn test_that_passes() {
        assert_eq!(1, 1, "This test should pass");
    }
}