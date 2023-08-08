use std::collections::HashMap;

pub fn count(string: &str) -> (u32, HashMap<char, u32>) {
    let mut total = 0;
    let mut vowels_counting: HashMap<char, u32> =
        HashMap::from([('a', 0), ('e', 0), ('i', 0), ('o', 0), ('u', 0)]);

    for ch in string.to_ascii_lowercase().chars() {
        if is_vowel(ch) {
            total += 1;
            let old_char_counting = vowels_counting.get(&ch).unwrap();
            vowels_counting.insert(ch, old_char_counting + 1);
        }
    }

    (total, vowels_counting)
}

fn is_vowel(ch: char) -> bool {
    match ch.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}
