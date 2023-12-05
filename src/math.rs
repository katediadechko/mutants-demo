pub fn is_positive(number: i32) -> bool {
    number > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_positive() {
        assert_eq!(is_positive(5), true);
    }
}
