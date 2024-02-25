use std::collections::HashMap;

/// Calculates the statistics of consecutive pairs of IDs in the given slice.
/// Returns a HashMap where the keys are pairs of IDs and the values are the number of times the pair appears.
///
/// # Arguments
///
/// * `ids` - A slice of `u32` IDs.
///
/// # Examples
///
/// ```
/// let ids = vec![1, 2, 1, 2, 3, 1, 2];
/// let stats = get_stats(&ids);
/// assert_eq!(stats.get(&(1, 2)), Some(&3)); // Appears 3 times
/// assert_eq!(stats.get(&(2, 1)), Some(&1)); // Appears 1 time
/// assert_eq!(stats.get(&(2, 3)), Some(&1)); // Appears 1 time
/// assert_eq!(stats.get(&(3, 1)), Some(&1)); // Appears 1 time
/// assert_eq!(stats.len(), 4); // Only 4 unique pairs
/// ```
pub fn get_stats(ids: &[u32]) -> HashMap<(u32, u32), u32> {
    let mut counts = HashMap::new();
    for window in ids.windows(2) {
        *counts.entry((window[0], window[1])).or_insert(0) += 1;
    }
    counts
}

/// Merges consecutive pairs of IDs in the given vector with a specified pair and index.
/// Returns a new vector with the merged IDs.
///
/// # Arguments
///
/// * `ids` - A vector of `u32` IDs.
/// * `pair` - A tuple representing the pair of IDs to be merged.
/// * `idx` - The ID to replace the merged pair with.
///
/// # Examples
///
/// ```
/// let ids = vec![1, 2, 1, 2, 3, 1, 2];
/// let pair = (1, 2);
/// let new_id = 256;
/// let merged_ids = merge(ids, pair, new_id);
/// assert_eq!(merged_ids, vec![256, 256, 3, 256]); // Pair (1, 2) replaced by 256
/// ```
pub fn merge(ids: Vec<u32>, pair: (u32, u32), idx: u32) -> Vec<u32> {
    let mut new_ids = Vec::new();
    let mut i = 0;
    while i < ids.len() {
        if i + 1 < ids.len() && ids[i] == pair.0 && ids[i + 1] == pair.1 {
            new_ids.push(idx);
            i += 2;
        } else {
            new_ids.push(ids[i]);
            i += 1;
        }
    }
    new_ids
}

/// Replaces control characters in the given string with their Unicode escape sequences.
/// Returns a new string with the replaced control characters.
///
/// # Arguments
///
/// * `s` - The input string.
///
/// # Examples
///
/// ```
/// let input = "\u{0007}Hello, \u{0009}world!\u{000A}";
/// let expected = "\\u0007Hello, \\u0009world!\\u000a";
/// assert_eq!(replace_control_characters(input), expected);
/// ```
pub fn replace_control_characters(s: &str) -> String {
    s.chars()
        .map(|ch| if ch.is_control() { format!("\\u{:04x}", ch as u32) } else { ch.to_string() })
        .collect()
}

/// Renders a token represented by a byte slice as a string.
/// Control characters are rendered as Unicode escape sequences, while other bytes are rendered as characters.
///
/// # Arguments
///
/// * `token` - A byte slice representing the token.
///
/// # Examples
///
/// ```
/// let token = &[0x00, 0x1F, 0x20, 0x7F];
/// assert_eq!(render_token(token), "\\x00\\x1f \\x7f");
/// ```
pub fn render_token(token: &[u8]) -> String {
    let mut result = String::new();
    for &byte in token {
        match byte {
            0x00..=0x1F | 0x7F => result.push_str(&format!("\\x{:02x}", byte)),
            _ => result.push(char::from(byte)),
        }
    }
    result
}
use std::collections::HashMap;

#[cfg(test)]
mod tests {

    use super::{get_stats, merge, render_token, replace_control_characters};

    #[test]
    fn test_get_stats() {
        let ids = vec![1, 2, 1, 2, 3, 1, 2];
        let stats = get_stats(&ids);
        assert_eq!(stats.get(&(1, 2)), Some(&3)); // Appears 3 times
        assert_eq!(stats.get(&(2, 1)), Some(&1)); // Appears 1 time
        assert_eq!(stats.get(&(2, 3)), Some(&1)); // Appears 1 time
        assert_eq!(stats.get(&(3, 1)), Some(&1)); // Appears 1 time
        assert_eq!(stats.len(), 4); // Only 4 unique pairs
    }

    #[test]
    fn test_merge() {
        let ids = vec![1, 2, 1, 2, 3, 1, 2];
        let pair = (1, 2);
        let new_id = 256;
        let merged_ids = merge(ids, pair, new_id);
        assert_eq!(merged_ids, vec![256, 256, 3, 256]); // Pair (1, 2) replaced by 256
    }

    #[test]
    fn test_replace_control_characters() {
        // Test with a string containing control characters
        let input = "\u{0007}Hello, \u{0009}world!\u{000A}";
        let expected = "\\u0007Hello, \\u0009world!\\u000a";
        assert_eq!(replace_control_characters(input), expected);

        // Test with a string without control characters
        let input = "Hello, world!";
        let expected = "Hello, world!";
        assert_eq!(replace_control_characters(input), expected);

        // Test with an empty string
        let input = "";
        let expected = "";
        assert_eq!(replace_control_characters(input), expected);

        // Test with a string containing only control characters
        let input = "\u{0000}\u{001F}";
        let expected = "\\u0000\\u001f";
        assert_eq!(replace_control_characters(input), expected);
    }

    #[test]
    fn test_render_token() {
        // Test with control characters
        let token = &[0x00, 0x1F, 0x20, 0x7F];
        assert_eq!(render_token(token), "\\x00\\x1f \\x7f");

        // Test with ASCII characters
        let token = b"Hello, world!";
        assert_eq!(render_token(token), "Hello, world!");

        // Test with a mix of control and ASCII characters
        let token = &[0x00, b'H', b'e', b'l', b'l', b'o', 0x7F];
        assert_eq!(render_token(token), "\\x00Hello\\x7f");

        // Test with an empty slice
        let token: &[u8] = &[];
        assert_eq!(render_token(token), "");
    }
}
