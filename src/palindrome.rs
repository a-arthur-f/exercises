pub fn is_palindrome(s: &str) -> bool {
    let half = s.len() / 2;

    s.to_lowercase()[..=half]
        .chars()
        .eq(s.to_lowercase()[half..].chars().rev())
}
