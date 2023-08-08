pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut low: usize = 0;
    let mut high = chars.len() - 1;

    while high > low {
        if chars[low] != chars[high] {
            return false;
        }

        low += 1;
        high -= 1;
    }

    true
}
